#[cfg(feature = "onig")]
use onig::{Regex, RegexOptions};

use std::collections::HashMap;
use std::convert::TryInto;

use crate::containers::{
    maybe_ptr::{MaybePtrNone, MaybePtrSome},
    ptr::ToMaybePtr,
    MaybePtr, Ptr,
};
use crate::error::Diagnostics;
use crate::nodes::*;
use crate::LexState;
use crate::Loc;
use crate::StringValue;
use crate::{
    Bytes, Context, CurrentArgStack, Lexer, MaxNumparamStack, Node, StaticEnvironment, Token,
    VariablesStack,
};
use crate::{Diagnostic, DiagnosticMessage, ErrorLevel};

#[derive(Debug, PartialEq)]
pub(crate) enum LoopType {
    While,
    Until,
}

#[derive(Debug, PartialEq)]
pub(crate) enum KeywordCmd {
    Break,
    Defined,
    Next,
    Redo,
    Retry,
    Return,
    Super,
    Yield,
    Zsuper,
}

enum MethodCallType {
    Send,
    CSend,
}

#[derive(Debug, PartialEq)]
pub(crate) enum LogicalOp {
    And,
    Or,
}

#[derive(Debug, Clone)]
pub(crate) enum PKwLabel {
    PlainLabel(Box<Token>),
    QuotedLabel((Box<Token>, Vec<Node>, Box<Token>)),
}

#[derive(Debug, Clone)]
pub(crate) enum ArgsType {
    Args(Option<Box<Node>>),
    Numargs(u8),
}

#[derive(Debug)]
pub(crate) struct Builder {
    static_env: StaticEnvironment,
    context: Context,
    current_arg_stack: CurrentArgStack,
    max_numparam_stack: MaxNumparamStack,
    pattern_variables: VariablesStack,
    pattern_hash_keys: VariablesStack,
    diagnostics: Diagnostics,
}

impl Builder {
    pub(crate) fn new(
        static_env: StaticEnvironment,
        context: Context,
        current_arg_stack: CurrentArgStack,
        max_numparam_stack: MaxNumparamStack,
        pattern_variables: VariablesStack,
        pattern_hash_keys: VariablesStack,
        diagnostics: Diagnostics,
    ) -> Self {
        Self {
            static_env,
            context,
            current_arg_stack,
            max_numparam_stack,
            pattern_variables,
            pattern_hash_keys,
            diagnostics,
        }
    }

    //
    // Literals
    //

    // Singletons

    pub(crate) fn nil(&self, nil_t: Box<Token>) -> Box<Node> {
        Box::new(Node::Nil(Nil {
            expression_l: self.loc(&nil_t),
        }))
    }

    pub(crate) fn true_(&self, true_t: Box<Token>) -> Box<Node> {
        Box::new(Node::True(True {
            expression_l: self.loc(&true_t),
        }))
    }

    pub(crate) fn false_(&self, false_t: Box<Token>) -> Box<Node> {
        Box::new(Node::False(False {
            expression_l: self.loc(&false_t),
        }))
    }

    // Numerics

    pub(crate) fn integer(&self, integer_t: Box<Token>) -> Box<Node> {
        let expression_l = self.loc(&integer_t);
        Box::new(Node::Int(Int {
            value: value(integer_t),
            expression_l,
            operator_l: None,
        }))
    }

    pub(crate) fn float(&self, float_t: Box<Token>) -> Box<Node> {
        let expression_l = self.loc(&float_t);
        Box::new(Node::Float(Float {
            value: value(float_t),
            expression_l,
            operator_l: None,
        }))
    }

    pub(crate) fn rational(&self, rational_t: Box<Token>) -> Box<Node> {
        let expression_l = self.loc(&rational_t);
        Box::new(Node::Rational(Rational {
            value: value(rational_t),
            expression_l,
            operator_l: None,
        }))
    }

    pub(crate) fn complex(&self, complex_t: Box<Token>) -> Box<Node> {
        let expression_l = self.loc(&complex_t);
        Box::new(Node::Complex(Complex {
            value: value(complex_t),
            expression_l,
            operator_l: None,
        }))
    }

    pub(crate) fn unary_num(&self, unary_t: Box<Token>, mut numeric: Box<Node>) -> Box<Node> {
        let new_operator_l = self.loc(&unary_t);
        let sign = value(unary_t);

        match &mut *numeric {
            Node::Int(Int {
                value,
                expression_l,
                operator_l,
            })
            | Node::Float(Float {
                value,
                expression_l,
                operator_l,
            })
            | Node::Rational(Rational {
                value,
                expression_l,
                operator_l,
            })
            | Node::Complex(Complex {
                value,
                expression_l,
                operator_l,
            }) => {
                *value = sign + value;
                *expression_l = new_operator_l.join(expression_l);
                *operator_l = Some(new_operator_l);
            }
            _ => unreachable!(),
        }

        numeric
    }

    pub(crate) fn __line__(&self, line_t: Box<Token>) -> Box<Node> {
        Box::new(Node::Line(Line {
            expression_l: self.loc(&line_t),
        }))
    }

    // Strings

    pub(crate) fn str_node(
        &self,
        begin_t: Option<Box<Token>>,
        value: StringValue,
        parts: Vec<Node>,
        end_t: Option<Box<Token>>,
    ) -> Box<Node> {
        if self.is_heredoc(&begin_t) {
            let HeredocMap {
                heredoc_body_l,
                heredoc_end_l,
                expression_l,
            } = self.heredoc_map(&begin_t, &parts, &end_t);

            Box::new(Node::Heredoc(Heredoc {
                parts,
                heredoc_body_l,
                heredoc_end_l,
                expression_l,
            }))
        } else {
            let CollectionMap {
                begin_l,
                end_l,
                expression_l,
            } = self.collection_map(&begin_t, &parts, &end_t);

            Box::new(Node::Str(Str {
                value,
                begin_l,
                end_l,
                expression_l,
            }))
        }
    }

    pub(crate) fn string_internal(&self, string_t: Box<Token>) -> Box<Node> {
        let expression_l = self.loc(&string_t);
        let value = StringValue::new(string_t);
        Box::new(Node::Str(Str {
            value,
            begin_l: None,
            end_l: None,
            expression_l,
        }))
    }

    pub(crate) fn string_compose(
        &self,
        begin_t: Option<Box<Token>>,
        parts: Vec<Node>,
        end_t: Option<Box<Token>>,
    ) -> Box<Node> {
        match &parts[..] {
            [] => return self.str_node(begin_t, StringValue::empty(), parts, end_t),
            [Node::Str(_)] | [Node::Dstr(_)] | [Node::Heredoc(_)]
                if begin_t.is_none() && end_t.is_none() =>
            {
                return Box::new(first(parts));
            }
            [Node::Str(Str { value, .. })] => {
                let value = value.clone();
                return self.str_node(begin_t, value, parts, end_t);
            }
            [Node::Dstr(_)] | [Node::Heredoc(_)] => unreachable!(),
            _ => {}
        };

        if self.is_heredoc(&begin_t) {
            let HeredocMap {
                heredoc_body_l,
                heredoc_end_l,
                expression_l,
            } = self.heredoc_map(&begin_t, &parts, &end_t);

            Box::new(Node::Heredoc(Heredoc {
                parts,
                heredoc_body_l,
                heredoc_end_l,
                expression_l,
            }))
        } else {
            let CollectionMap {
                begin_l,
                end_l,
                expression_l,
            } = self.collection_map(&begin_t, &parts, &end_t);

            Box::new(Node::Dstr(Dstr {
                parts,
                begin_l,
                end_l,
                expression_l,
            }))
        }
    }

    pub(crate) fn character(&self, char_t: Box<Token>) -> Box<Node> {
        let str_loc = self.loc(&char_t);

        let begin_l = Some(str_loc.with_end(str_loc.begin + 1));
        let end_l = None;
        let expression_l = str_loc;

        let value = StringValue::new(char_t);
        Box::new(Node::Str(Str {
            value,
            begin_l,
            end_l,
            expression_l,
        }))
    }

    pub(crate) fn __file__(&self, file_t: Box<Token>) -> Box<Node> {
        Box::new(Node::File(File {
            expression_l: self.loc(&file_t),
        }))
    }

    // Symbols

    fn validate_sym_value(&self, value: &StringValue, loc: &Loc) {
        if !value.bytes.is_valid_utf8() {
            self.error(
                DiagnosticMessage::InvalidSymbol {
                    symbol: "UTF-8".to_string(),
                },
                loc,
            )
        }
    }

    pub(crate) fn symbol(&self, start_t: Box<Token>, value_t: Box<Token>) -> Box<Node> {
        let expression_l = self.loc(&start_t).join(&self.loc(&value_t));
        let begin_l = Some(self.loc(&start_t));
        let value = StringValue::new(value_t);
        self.validate_sym_value(&value, &expression_l);
        Box::new(Node::Sym(Sym {
            name: value,
            begin_l,
            end_l: None,
            expression_l,
        }))
    }

    pub(crate) fn symbol_internal(&self, symbol_t: Box<Token>) -> Box<Node> {
        let expression_l = self.loc(&symbol_t);
        let value = StringValue::new(symbol_t);
        self.validate_sym_value(&value, &expression_l);
        Box::new(Node::Sym(Sym {
            name: value,
            begin_l: None,
            end_l: None,
            expression_l,
        }))
    }

    pub(crate) fn symbol_compose(
        &self,
        begin_t: Box<Token>,
        parts: Vec<Node>,
        end_t: Box<Token>,
    ) -> Box<Node> {
        if let [Node::Str(Str { value, .. })] = &parts[..] {
            let CollectionMap {
                begin_l,
                end_l,
                expression_l,
            } = self.collection_map(&Some(begin_t), &[], &Some(end_t));

            self.validate_sym_value(value, &expression_l);

            return Box::new(Node::Sym(Sym {
                name: value.clone(),
                begin_l,
                end_l,
                expression_l,
            }));
        }

        let CollectionMap {
            begin_l,
            end_l,
            expression_l,
        } = self.collection_map(&Some(begin_t), &parts, &Some(end_t));
        Box::new(Node::Dsym(Dsym {
            parts,
            begin_l,
            end_l,
            expression_l,
        }))
    }

    // Executable strings

    pub(crate) fn xstring_compose(
        &self,
        begin_t: Box<Token>,
        parts: Vec<Node>,
        end_t: Box<Token>,
    ) -> Box<Node> {
        let begin_l = self.loc(&begin_t);
        if lossy_value(begin_t).starts_with("<<") {
            let heredoc_body_l = collection_expr(&parts).unwrap_or_else(|| self.loc(&end_t));
            let heredoc_end_l = self.loc(&end_t);
            let expression_l = begin_l;

            Box::new(Node::XHeredoc(XHeredoc {
                parts,
                heredoc_body_l,
                heredoc_end_l,
                expression_l,
            }))
        } else {
            let end_l = self.loc(&end_t);
            let expression_l = begin_l.join(&end_l);

            Box::new(Node::Xstr(Xstr {
                parts,
                begin_l,
                end_l,
                expression_l,
            }))
        }
    }

    // Indented (interpolated, noninterpolated, executable) strings

    pub(crate) fn heredoc_dedent(&self, node: &mut Node, dedent_level: i32) {
        if dedent_level == 0 {
            return;
        }

        let dedent_level: usize = dedent_level
            .try_into()
            .expect("dedent_level must be positive");

        let dedent_heredoc_parts = |parts: &mut Vec<Node>| {
            let mut idx_to_drop = vec![];
            for (idx, part) in parts.iter_mut().enumerate() {
                match part {
                    Node::Str(Str { value, .. }) => {
                        Self::dedent_string(value, dedent_level);
                        if value.bytes.is_empty() {
                            idx_to_drop.push(idx);
                        }
                    }
                    Node::Begin(_)
                    | Node::Gvar(_)
                    | Node::BackRef(_)
                    | Node::NthRef(_)
                    | Node::Ivar(_)
                    | Node::Cvar(_) => {}
                    _ => unreachable!("unsupported heredoc child {}", part.str_type()),
                }
            }
            for idx in idx_to_drop.iter().rev() {
                parts.remove(*idx);
            }
        };

        match node {
            Node::Heredoc(heredoc) => {
                dedent_heredoc_parts(&mut heredoc.parts);
            }
            Node::XHeredoc(heredoc) => {
                dedent_heredoc_parts(&mut heredoc.parts);
            }
            other => unreachable!("unsupported heredoc_dedent argument {}", other.str_type()),
        }
    }

    const TAB_WIDTH: usize = 8;

    pub(crate) fn dedent_string(s: &mut StringValue, width: usize) {
        let mut col: usize = 0;
        let mut i: usize = 0;

        loop {
            if !(i < s.bytes.len() && col < width) {
                break;
            }

            if s.bytes[i] == b' ' {
                col += 1;
            } else if s.bytes[i] == b'\t' {
                let n = Self::TAB_WIDTH * (col / Self::TAB_WIDTH + 1);
                if n > Self::TAB_WIDTH {
                    break;
                }
                col = n;
            } else {
                break;
            }

            i += 1;
        }

        s.bytes = Bytes::new(s.bytes.raw[i..].to_vec());
    }

    // Regular expressions

    pub(crate) fn regexp_options(&self, regexp_end_t: Box<Token>) -> Option<Box<Node>> {
        if regexp_end_t.loc.end - regexp_end_t.loc.begin == 1 {
            // no regexp options, only trailing "/"
            return None;
        }
        let expression_l = self.loc(&regexp_end_t).adjust_begin(1);
        let options = value(regexp_end_t);
        let mut options = options.chars().skip(1).collect::<Vec<_>>();
        options.sort_unstable();
        options.dedup();

        Some(Box::new(Node::RegOpt(RegOpt {
            options,
            expression_l,
        })))
    }

    pub(crate) fn regexp_compose(
        &self,
        begin_t: Box<Token>,
        parts: Vec<Node>,
        end_t: Box<Token>,
        options: Option<Box<Node>>,
    ) -> Box<Node> {
        let begin_l = self.loc(&begin_t);
        let end_l = self.loc(&end_t).resize(1);
        let expression_l =
            begin_l.join(&maybe_boxed_node_expr(&options).unwrap_or_else(|| self.loc(&end_t)));
        match &options.as_deref() {
            Some(Node::RegOpt(RegOpt { options, .. })) => {
                self.validate_static_regexp(&parts, options, &expression_l)
            }
            None => self.validate_static_regexp(&parts, &[], &expression_l),
            _ => unreachable!("must be Option<RegOpt>"),
        };
        Box::new(Node::Regexp(Regexp {
            parts,
            options,
            begin_l,
            end_l,
            expression_l,
        }))
    }

    // Arrays

    pub(crate) fn array(
        &self,
        begin_t: Option<Box<Token>>,
        elements: Vec<Node>,
        end_t: Option<Box<Token>>,
    ) -> Box<Node> {
        let CollectionMap {
            begin_l,
            end_l,
            expression_l,
        } = self.collection_map(&begin_t, &elements, &end_t);

        Box::new(Node::Array(Array {
            elements,
            begin_l,
            end_l,
            expression_l,
        }))
    }

    pub(crate) fn splat(&self, star_t: Box<Token>, value: Option<Box<Node>>) -> Box<Node> {
        let operator_l = self.loc(&star_t);
        let expression_l = operator_l.maybe_join(&maybe_boxed_node_expr(&value));

        Box::new(Node::Splat(Splat {
            operator_l,
            expression_l,
            value,
        }))
    }

    pub(crate) fn word(&self, parts: Vec<Node>) -> Box<Node> {
        match &parts[..] {
            [Node::Str(_)] | [Node::Dstr(_)] => {
                // collapse_string_parts? == true
                return Box::new(
                    parts
                        .into_iter()
                        .next()
                        .expect("parts is supposed to have exactly 1 element"),
                );
            }
            _ => {}
        }

        let CollectionMap {
            begin_l,
            end_l,
            expression_l,
        } = self.collection_map(&None, &parts, &None);

        Box::new(Node::Dstr(Dstr {
            parts,
            begin_l,
            end_l,
            expression_l,
        }))
    }

    pub(crate) fn words_compose(
        &self,
        begin_t: Box<Token>,
        elements: Vec<Node>,
        end_t: Box<Token>,
    ) -> Box<Node> {
        let begin_l = self.loc(&begin_t);
        let end_l = self.loc(&end_t);
        let expression_l = begin_l.join(&end_l);
        Box::new(Node::Array(Array {
            elements,
            begin_l: Some(begin_l),
            end_l: Some(end_l),
            expression_l,
        }))
    }

    pub(crate) fn symbols_compose(
        &self,
        begin_t: Box<Token>,
        parts: Vec<Node>,
        end_t: Box<Token>,
    ) -> Box<Node> {
        let parts = parts
            .into_iter()
            .map(|part| match part {
                Node::Str(Str {
                    value,
                    begin_l,
                    end_l,
                    expression_l,
                }) => {
                    self.validate_sym_value(&value, &expression_l);
                    Node::Sym(Sym {
                        name: value,
                        begin_l,
                        end_l,
                        expression_l,
                    })
                }
                Node::Dstr(Dstr {
                    parts,
                    begin_l,
                    end_l,
                    expression_l,
                }) => Node::Dsym(Dsym {
                    parts,
                    begin_l,
                    end_l,
                    expression_l,
                }),
                _ => part,
            })
            .collect::<Vec<_>>();

        let begin_l = self.loc(&begin_t);
        let end_l = self.loc(&end_t);
        let expression_l = begin_l.join(&end_l);
        Box::new(Node::Array(Array {
            elements: parts,
            begin_l: Some(begin_l),
            end_l: Some(end_l),
            expression_l,
        }))
    }

    // Hashes

    pub(crate) fn pair(&self, key: Box<Node>, assoc_t: Box<Token>, value: Box<Node>) -> Box<Node> {
        let operator_l = self.loc(&assoc_t);
        let expression_l = join_exprs(&key, &value);

        Box::new(Node::Pair(Pair {
            key,
            value,
            operator_l,
            expression_l,
        }))
    }

    pub(crate) fn pair_keyword(&self, key_t: Box<Token>, value: Box<Node>) -> Box<Node> {
        let key_loc = self.loc(&key_t);
        let key_l = key_loc.adjust_end(-1);
        let colon_l = key_loc.with_begin(key_loc.end - 1);
        let expression_l = key_loc.join(&value.expression());

        let key = StringValue::new(key_t);
        self.validate_sym_value(&key, &key_l);

        Box::new(Node::Pair(Pair {
            key: Box::new(Node::Sym(Sym {
                name: key,
                begin_l: None,
                end_l: None,
                expression_l: key_l,
            })),
            value,
            operator_l: colon_l,
            expression_l,
        }))
    }

    pub(crate) fn pair_quoted(
        &self,
        begin_t: Box<Token>,
        parts: Vec<Node>,
        end_t: Box<Token>,
        value: Box<Node>,
    ) -> Box<Node> {
        let end_l = self.loc(&end_t);

        let quote_loc = Loc::new(end_l.end - 2, end_l.end - 1);

        let colon_l = end_l.with_begin(end_l.end - 1);

        let end_t: Box<Token> = Box::new(Token {
            token_type: end_t.token_type,
            token_value: end_t.token_value,
            loc: quote_loc,
            lex_state_before: LexState::default(),
            lex_state_after: LexState::default(),
        });
        let expression_l = self.loc(&begin_t).join(&value.expression());

        Box::new(Node::Pair(Pair {
            key: self.symbol_compose(begin_t, parts, end_t),
            value,
            operator_l: colon_l,
            expression_l,
        }))
    }

    pub(crate) fn kwsplat(&self, dstar_t: Box<Token>, value: Box<Node>) -> Box<Node> {
        let operator_l = self.loc(&dstar_t);
        let expression_l = value.expression().join(&operator_l);

        Box::new(Node::Kwsplat(Kwsplat {
            value,
            operator_l,
            expression_l,
        }))
    }

    pub(crate) fn associate(
        &self,
        begin_t: Option<Box<Token>>,
        pairs: Vec<Node>,
        end_t: Option<Box<Token>>,
    ) -> Box<Node> {
        let CollectionMap {
            begin_l,
            end_l,
            expression_l,
        } = self.collection_map(&begin_t, &pairs, &end_t);

        Box::new(Node::Hash(Hash {
            pairs,
            begin_l,
            end_l,
            expression_l,
        }))
    }

    // Ranges

    pub(crate) fn range_inclusive(
        &self,
        left: Option<Box<Node>>,
        dot2_t: Box<Token>,
        right: Option<Box<Node>>,
    ) -> Box<Node> {
        let operator_l = self.loc(&dot2_t);
        let expression_l = operator_l
            .maybe_join(&maybe_boxed_node_expr(&left))
            .maybe_join(&maybe_boxed_node_expr(&right));

        Box::new(Node::Irange(Irange {
            left,
            right,
            operator_l,
            expression_l,
        }))
    }

    pub(crate) fn range_exclusive(
        &self,
        left: Option<Box<Node>>,
        dot3_t: Box<Token>,
        right: Option<Box<Node>>,
    ) -> Box<Node> {
        let operator_l = self.loc(&dot3_t);
        let expression_l = operator_l
            .maybe_join(&maybe_boxed_node_expr(&left))
            .maybe_join(&maybe_boxed_node_expr(&right));

        Box::new(Node::Erange(Erange {
            left,
            right,
            operator_l,
            expression_l,
        }))
    }

    //
    // Access
    //

    pub(crate) fn self_(&self, token: Box<Token>) -> Box<Node> {
        Box::new(Node::Self_(Self_ {
            expression_l: self.loc(&token),
        }))
    }

    pub(crate) fn lvar(&self, token: Box<Token>) -> Box<Node> {
        let expression_l = self.loc(&token);
        Box::new(Node::Lvar(Lvar {
            name: value(token),
            expression_l,
        }))
    }

    pub(crate) fn ivar(&self, token: Box<Token>) -> Box<Node> {
        let expression_l = self.loc(&token);
        Box::new(Node::Ivar(Ivar {
            name: value(token),
            expression_l,
        }))
    }

    pub(crate) fn gvar(&self, token: Box<Token>) -> Box<Node> {
        let expression_l = self.loc(&token);
        Box::new(Node::Gvar(Gvar {
            name: value(token),
            expression_l,
        }))
    }

    pub(crate) fn cvar(&self, token: Box<Token>) -> Box<Node> {
        let expression_l = self.loc(&token);
        Box::new(Node::Cvar(Cvar {
            name: value(token),
            expression_l,
        }))
    }

    pub(crate) fn back_ref(&self, token: Box<Token>) -> Box<Node> {
        let expression_l = self.loc(&token);
        Box::new(Node::BackRef(BackRef {
            name: value(token),
            expression_l,
        }))
    }

    const MAX_NTH_REF: usize = 0b111111111111111111111111111111;

    pub(crate) fn nth_ref(&self, token: Box<Token>) -> Box<Node> {
        let expression_l = self.loc(&token);
        let name = value(token)[1..].to_string();
        let parsed = name.parse::<usize>();

        if parsed.is_err() || parsed.map(|n| n > Self::MAX_NTH_REF) == Ok(true) {
            self.warn(
                DiagnosticMessage::NthRefIsTooBig {
                    nth_ref: name.clone(),
                },
                &expression_l,
            )
        }

        Box::new(Node::NthRef(NthRef { name, expression_l }))
    }
    pub(crate) fn accessible(&self, node: Box<Node>) -> Box<Node> {
        match *node {
            Node::Lvar(Lvar { name, expression_l }) => {
                if self.static_env.is_declared(&name) {
                    if let Some(current_arg) = self.current_arg_stack.top() {
                        if current_arg == name {
                            self.error(
                                DiagnosticMessage::CircularArgumentReference {
                                    arg_name: name.clone(),
                                },
                                &expression_l,
                            );
                        }
                    }

                    Box::new(Node::Lvar(Lvar { name, expression_l }))
                } else {
                    Box::new(Node::Send(Send {
                        recv: None,
                        method_name: name,
                        args: vec![],
                        dot_l: None,
                        selector_l: Some(expression_l.clone()),
                        begin_l: None,
                        end_l: None,
                        operator_l: None,
                        expression_l,
                    }))
                }
            }
            _ => node,
        }
    }

    pub(crate) fn const_(&self, name_t: Box<Token>) -> Box<Node> {
        let name_l = self.loc(&name_t);
        let expression_l = name_l.clone();

        Box::new(Node::Const(Const {
            scope: None,
            name: value(name_t),
            double_colon_l: None,
            name_l,
            expression_l,
        }))
    }

    pub(crate) fn const_global(&self, t_colon3: Box<Token>, name_t: Box<Token>) -> Box<Node> {
        let scope = Node::Cbase(Cbase {
            expression_l: self.loc(&t_colon3),
        });

        let name_l = self.loc(&name_t);
        let expression_l = scope.expression().join(&name_l);
        let double_colon_l = self.loc(&t_colon3);

        Box::new(Node::Const(Const {
            scope: Some(Box::new(scope)),
            name: value(name_t),
            double_colon_l: Some(double_colon_l),
            name_l,
            expression_l,
        }))
    }

    pub(crate) fn const_fetch(
        &self,
        scope: Box<Node>,
        t_colon2: Box<Token>,
        name_t: Box<Token>,
    ) -> Box<Node> {
        let name_l = self.loc(&name_t);
        let expression_l = scope.expression().join(&name_l);
        let double_colon_l = self.loc(&t_colon2);

        Box::new(Node::Const(Const {
            scope: Some(scope),
            name: value(name_t),
            double_colon_l: Some(double_colon_l),
            name_l,
            expression_l,
        }))
    }

    pub(crate) fn __encoding__(&self, encoding_t: Box<Token>) -> Box<Node> {
        Box::new(Node::Encoding(Encoding {
            expression_l: self.loc(&encoding_t),
        }))
    }

    //
    // Assignments
    //

    pub(crate) fn assignable(&self, node: Box<Node>) -> Result<Box<Node>, ()> {
        let node = match *node {
            Node::Cvar(Cvar { name, expression_l }) => Node::Cvasgn(Cvasgn {
                name,
                value: None,
                name_l: expression_l.clone(),
                expression_l,
                operator_l: None,
            }),
            Node::Ivar(Ivar { name, expression_l }) => Node::Ivasgn(Ivasgn {
                name,
                value: None,
                name_l: expression_l.clone(),
                expression_l,
                operator_l: None,
            }),
            Node::Gvar(Gvar { name, expression_l }) => Node::Gvasgn(Gvasgn {
                name,
                value: None,
                name_l: expression_l.clone(),
                expression_l,
                operator_l: None,
            }),
            Node::Const(Const {
                name,
                scope,
                expression_l,
                double_colon_l,
                name_l,
            }) => {
                if !self.context.is_dynamic_const_definition_allowed() {
                    self.error(DiagnosticMessage::DynamicConstantAssignment, &expression_l);
                    return Err(());
                }
                Node::Casgn(Casgn {
                    name,
                    scope,
                    value: None,
                    name_l,
                    double_colon_l,
                    expression_l,
                    operator_l: None,
                })
            }
            Node::Lvar(Lvar { name, expression_l }) => {
                self.check_assignment_to_numparam(&name, &expression_l)?;
                self.check_reserved_for_numparam(&name, &expression_l)?;

                self.static_env.declare(&name);

                Node::Lvasgn(Lvasgn {
                    name,
                    value: None,
                    name_l: expression_l.clone(),
                    expression_l,
                    operator_l: None,
                })
            }

            Node::Self_(Self_ { expression_l }) => {
                self.error(DiagnosticMessage::CantAssignToSelf, &expression_l);
                return Err(());
            }
            Node::Nil(Nil { expression_l }) => {
                self.error(DiagnosticMessage::CantAssignToNil, &expression_l);
                return Err(());
            }
            Node::True(True { expression_l }) => {
                self.error(DiagnosticMessage::CantAssignToTrue, &expression_l);
                return Err(());
            }
            Node::False(False { expression_l }) => {
                self.error(DiagnosticMessage::CantAssignToFalse, &expression_l);
                return Err(());
            }
            Node::File(File { expression_l }) => {
                self.error(DiagnosticMessage::CantAssignToFile, &expression_l);
                return Err(());
            }
            Node::Line(Line { expression_l }) => {
                self.error(DiagnosticMessage::CantAssignToLine, &expression_l);
                return Err(());
            }
            Node::Encoding(Encoding { expression_l }) => {
                self.error(DiagnosticMessage::CantAssignToEncoding, &expression_l);
                return Err(());
            }
            Node::BackRef(BackRef { expression_l, name }) => {
                self.error(
                    DiagnosticMessage::CantSetVariable { var_name: name },
                    &expression_l,
                );
                return Err(());
            }
            Node::NthRef(NthRef { expression_l, name }) => {
                self.error(
                    DiagnosticMessage::CantSetVariable {
                        var_name: format!("${}", name),
                    },
                    &expression_l,
                );
                return Err(());
            }
            _ => unreachable!("{:?} can't be used in assignment", node),
        };

        Ok(Box::new(node))
    }

    pub(crate) fn const_op_assignable(&self, node: Box<Node>) -> Box<Node> {
        match *node {
            Node::Const(Const {
                scope,
                name,
                name_l,
                double_colon_l,
                expression_l,
            }) => Box::new(Node::Casgn(Casgn {
                scope,
                name,
                name_l,
                double_colon_l,
                expression_l,
                value: None,
                operator_l: None,
            })),
            _ => unreachable!("unsupported const_op_assignable arument: {:?}", node),
        }
    }

    pub(crate) fn assign(
        &self,
        mut lhs: Box<Node>,
        eql_t: Box<Token>,
        new_rhs: Box<Node>,
    ) -> Box<Node> {
        let op_l = Some(self.loc(&eql_t));
        let expr_l = join_exprs(&lhs, &new_rhs);

        match &mut *lhs {
            Node::Cvasgn(Cvasgn {
                value,
                operator_l,
                expression_l,
                ..
            })
            | Node::Ivasgn(Ivasgn {
                value,
                operator_l,
                expression_l,
                ..
            })
            | Node::Gvasgn(Gvasgn {
                value,
                operator_l,
                expression_l,
                ..
            })
            | Node::Lvasgn(Lvasgn {
                value,
                operator_l,
                expression_l,
                ..
            })
            | Node::Casgn(Casgn {
                value,
                operator_l,
                expression_l,
                ..
            })
            | Node::IndexAsgn(IndexAsgn {
                value,
                operator_l,
                expression_l,
                ..
            }) => {
                *expression_l = expr_l;
                *operator_l = op_l;
                *value = Some(new_rhs);
            }
            Node::Send(Send {
                args,
                operator_l,
                expression_l,
                ..
            })
            | Node::CSend(CSend {
                args,
                operator_l,
                expression_l,
                ..
            }) => {
                *expression_l = expr_l;
                *operator_l = op_l;
                if args.is_empty() {
                    *args = vec![*new_rhs];
                } else {
                    unreachable!("can't assign to method call with args")
                }
            }
            _ => unreachable!("{:?} can't be used in assignment", lhs),
        }

        lhs
    }

    pub(crate) fn op_assign(
        &self,
        mut lhs: Box<Node>,
        op_t: Box<Token>,
        rhs: Box<Node>,
    ) -> Result<Box<Node>, ()> {
        let operator_l = self.loc(&op_t);
        let mut operator = value(op_t);
        operator.pop();
        let expression_l = join_exprs(&lhs, &rhs);

        match *lhs {
            Node::Gvasgn { .. }
            | Node::Ivasgn { .. }
            | Node::Lvasgn { .. }
            | Node::Cvasgn { .. }
            | Node::Casgn { .. }
            | Node::Send { .. }
            | Node::CSend { .. } => {}
            Node::Index(Index {
                recv,
                indexes,
                begin_l,
                end_l,
                expression_l,
            }) => {
                lhs = Box::new(Node::IndexAsgn(IndexAsgn {
                    recv,
                    indexes,
                    value: None,
                    begin_l,
                    end_l,
                    expression_l,
                    operator_l: None,
                }));
            }
            Node::BackRef(BackRef { expression_l, name }) => {
                self.error(
                    DiagnosticMessage::CantSetVariable { var_name: name },
                    &expression_l,
                );
                return Err(());
            }
            Node::NthRef(NthRef { expression_l, name }) => {
                self.error(
                    DiagnosticMessage::CantSetVariable {
                        var_name: format!("${}", name),
                    },
                    &expression_l,
                );
                return Err(());
            }
            _ => unreachable!("unsupported op_assign lhs {:?}", lhs),
        };

        let recv = lhs;
        let value = rhs;

        let result = match &operator[..] {
            "&&" => Node::AndAsgn(AndAsgn {
                recv,
                value,
                operator_l,
                expression_l,
            }),
            "||" => Node::OrAsgn(OrAsgn {
                recv,
                value,
                operator_l,
                expression_l,
            }),
            _ => Node::OpAsgn(OpAsgn {
                recv,
                value,
                operator,
                operator_l,
                expression_l,
            }),
        };

        Ok(Box::new(result))
    }

    pub(crate) fn multi_lhs(
        &self,
        begin_t: Option<Box<Token>>,
        items: Vec<Node>,
        end_t: Option<Box<Token>>,
    ) -> Box<Node> {
        let CollectionMap {
            begin_l,
            end_l,
            expression_l,
        } = self.collection_map(&begin_t, &items, &end_t);

        Box::new(Node::Mlhs(Mlhs {
            items,
            begin_l,
            end_l,
            expression_l,
        }))
    }

    pub(crate) fn multi_assign(
        &self,
        lhs: Box<Node>,
        eql_t: Box<Token>,
        rhs: Box<Node>,
    ) -> Box<Node> {
        let operator_l = self.loc(&eql_t);
        let expression_l = join_exprs(&lhs, &rhs);

        Box::new(Node::Masgn(Masgn {
            lhs,
            rhs,
            operator_l,
            expression_l,
        }))
    }

    //
    // Class and module definition
    //

    pub(crate) fn def_class(
        &self,
        class_t: Box<Token>,
        name: Box<Node>,
        lt_t: Option<Box<Token>>,
        superclass: Option<Box<Node>>,
        body: Option<Box<Node>>,
        end_t: Box<Token>,
    ) -> Box<Node> {
        let keyword_l = self.loc(&class_t);
        let end_l = self.loc(&end_t);
        let operator_l = self.maybe_loc(&lt_t);
        let expression_l = keyword_l.join(&end_l);

        Box::new(Node::Class(Class {
            name,
            superclass,
            body,
            keyword_l,
            operator_l,
            end_l,
            expression_l,
        }))
    }

    pub(crate) fn def_sclass(
        &self,
        class_t: Box<Token>,
        lshift_t: Box<Token>,
        expr: Box<Node>,
        body: Option<Box<Node>>,
        end_t: Box<Token>,
    ) -> Box<Node> {
        let keyword_l = self.loc(&class_t);
        let end_l = self.loc(&end_t);
        let operator_l = self.loc(&lshift_t);
        let expression_l = keyword_l.join(&end_l);

        Box::new(Node::SClass(SClass {
            expr,
            body,
            keyword_l,
            operator_l,
            end_l,
            expression_l,
        }))
    }

    pub(crate) fn def_module(
        &self,
        module_t: Box<Token>,
        name: Box<Node>,
        body: Option<Box<Node>>,
        end_t: Box<Token>,
    ) -> Box<Node> {
        let keyword_l = self.loc(&module_t);
        let end_l = self.loc(&end_t);
        let expression_l = keyword_l.join(&end_l);

        Box::new(Node::Module(Module {
            name,
            body,
            keyword_l,
            end_l,
            expression_l,
        }))
    }

    //
    // Method (un)definition
    //

    pub(crate) fn def_method(
        &self,
        def_t: Box<Token>,
        name_t: Box<Token>,
        args: Option<Box<Node>>,
        body: Option<Box<Node>>,
        end_t: Box<Token>,
    ) -> Result<Box<Node>, ()> {
        let name_l = self.loc(&name_t);
        let keyword_l = self.loc(&def_t);
        let end_l = self.loc(&end_t);
        let expression_l = keyword_l.join(&end_l);

        let name = value(name_t);
        self.check_reserved_for_numparam(&name, &name_l)?;

        Ok(Box::new(Node::Def(Def {
            name,
            args,
            body,
            keyword_l,
            name_l,
            assignment_l: None,
            end_l: Some(end_l),
            expression_l,
        })))
    }

    pub(crate) fn def_endless_method(
        &self,
        def_t: Box<Token>,
        name_t: Box<Token>,
        args: Option<Box<Node>>,
        assignment_t: Box<Token>,
        body: Option<Box<Node>>,
    ) -> Result<Box<Node>, ()> {
        let body_l = maybe_boxed_node_expr(&body)
            .unwrap_or_else(|| unreachable!("endless method always has a body"));

        let keyword_l = self.loc(&def_t);
        let expression_l = keyword_l.join(&body_l);
        let name_l = self.loc(&name_t);
        let assignment_l = self.loc(&assignment_t);

        let name = value(name_t);
        self.check_reserved_for_numparam(&name, &name_l)?;

        Ok(Box::new(Node::Def(Def {
            name,
            args,
            body,
            keyword_l,
            name_l,
            assignment_l: Some(assignment_l),
            end_l: None,
            expression_l,
        })))
    }

    pub(crate) fn def_singleton(
        &self,
        def_t: Box<Token>,
        definee: Box<Node>,
        dot_t: Box<Token>,
        name_t: Box<Token>,
        args: Option<Box<Node>>,
        body: Option<Box<Node>>,
        end_t: Box<Token>,
    ) -> Result<Box<Node>, ()> {
        let keyword_l = self.loc(&def_t);
        let operator_l = self.loc(&dot_t);
        let name_l = self.loc(&name_t);
        let end_l = self.loc(&end_t);
        let expression_l = keyword_l.join(&end_l);

        let name = value(name_t);
        self.check_reserved_for_numparam(&name, &name_l)?;

        Ok(Box::new(Node::Defs(Defs {
            definee,
            name,
            args,
            body,
            keyword_l,
            operator_l,
            name_l,
            assignment_l: None,
            end_l: Some(end_l),
            expression_l,
        })))
    }

    pub(crate) fn def_endless_singleton(
        &self,
        def_t: Box<Token>,
        definee: Box<Node>,
        dot_t: Box<Token>,
        name_t: Box<Token>,
        args: Option<Box<Node>>,
        assignment_t: Box<Token>,
        body: Option<Box<Node>>,
    ) -> Result<Box<Node>, ()> {
        let body_l = maybe_boxed_node_expr(&body)
            .unwrap_or_else(|| unreachable!("endless method always has body"));

        let keyword_l = self.loc(&def_t);
        let operator_l = self.loc(&dot_t);
        let name_l = self.loc(&name_t);
        let assignment_l = self.loc(&assignment_t);
        let expression_l = keyword_l.join(&body_l);

        let name = value(name_t);
        self.check_reserved_for_numparam(&name, &name_l)?;

        Ok(Box::new(Node::Defs(Defs {
            definee,
            name,
            args,
            body,
            keyword_l,
            operator_l,
            name_l,
            assignment_l: Some(assignment_l),
            end_l: None,
            expression_l,
        })))
    }

    pub(crate) fn undef_method(&self, undef_t: Box<Token>, names: Vec<Node>) -> Box<Node> {
        let keyword_l = self.loc(&undef_t);
        let expression_l = keyword_l.maybe_join(&collection_expr(&names));
        Box::new(Node::Undef(Undef {
            names,
            keyword_l,
            expression_l,
        }))
    }

    pub(crate) fn alias(&self, alias_t: Box<Token>, to: Box<Node>, from: Box<Node>) -> Box<Node> {
        let keyword_l = self.loc(&alias_t);
        let expression_l = keyword_l.join(from.expression());
        Box::new(Node::Alias(Alias {
            to,
            from,
            keyword_l,
            expression_l,
        }))
    }

    //
    // Formal arguments
    //

    pub(crate) fn args(
        &self,
        begin_t: Option<Box<Token>>,
        args: Vec<Node>,
        end_t: Option<Box<Token>>,
    ) -> Option<Box<Node>> {
        self.check_duplicate_args(&args, &mut HashMap::new());

        if begin_t.is_none() && args.is_empty() && end_t.is_none() {
            return None;
        }

        let CollectionMap {
            begin_l,
            end_l,
            expression_l,
        } = self.collection_map(&begin_t, &args, &end_t);

        Some(Box::new(Node::Args(Args {
            args,
            begin_l,
            end_l,
            expression_l,
        })))
    }

    pub(crate) fn forward_only_args(
        &self,
        begin_t: Box<Token>,
        dots_t: Box<Token>,
        end_t: Box<Token>,
    ) -> Box<Node> {
        let args = vec![*self.forward_arg(dots_t)];
        let begin_l = self.loc(&begin_t);
        let end_l = self.loc(&end_t);
        let expression_l = begin_l.join(&end_l);
        Box::new(Node::Args(Args {
            args,
            begin_l: Some(begin_l),
            end_l: Some(end_l),
            expression_l,
        }))
    }

    pub(crate) fn forward_arg(&self, dots_t: Box<Token>) -> Box<Node> {
        Box::new(Node::ForwardArg(ForwardArg {
            expression_l: self.loc(&dots_t),
        }))
    }

    pub(crate) fn arg(&self, name_t: Box<Token>) -> Result<Box<Node>, ()> {
        let name_l = self.loc(&name_t);
        let name = value(name_t);

        self.check_reserved_for_numparam(&name, &name_l)?;

        Ok(Box::new(Node::Arg(Arg {
            name,
            expression_l: name_l,
        })))
    }

    pub(crate) fn optarg(
        &self,
        name_t: Box<Token>,
        eql_t: Box<Token>,
        default: Box<Node>,
    ) -> Result<Box<Node>, ()> {
        let operator_l = self.loc(&eql_t);
        let name_l = self.loc(&name_t);
        let expression_l = self.loc(&name_t).join(default.expression());

        let name = value(name_t);
        self.check_reserved_for_numparam(&name, &name_l)?;

        Ok(Box::new(Node::Optarg(Optarg {
            name,
            default,
            name_l,
            operator_l,
            expression_l,
        })))
    }

    pub(crate) fn restarg(
        &self,
        star_t: Box<Token>,
        name_t: Option<Box<Token>>,
    ) -> Result<Box<Node>, ()> {
        let (name, name_l) = match name_t {
            Some(name_t) => {
                let name_l = self.loc(&name_t);
                let name = value(name_t);
                self.check_reserved_for_numparam(&name, &name_l)?;
                (Some(name), Some(name_l))
            }
            _ => (None, None),
        };

        let operator_l = self.loc(&star_t);
        let expression_l = operator_l.maybe_join(&name_l);

        Ok(Box::new(Node::Restarg(Restarg {
            name,
            operator_l,
            name_l,
            expression_l,
        })))
    }

    pub(crate) fn kwarg(&self, name_t: Box<Token>) -> Result<Box<Node>, ()> {
        let name_l = self.loc(&name_t);
        let name = value(name_t);
        self.check_reserved_for_numparam(&name, &name_l)?;

        let expression_l = name_l;
        let name_l = expression_l.adjust_end(-1);

        Ok(Box::new(Node::Kwarg(Kwarg {
            name,
            name_l,
            expression_l,
        })))
    }

    pub(crate) fn kwoptarg(&self, name_t: Box<Token>, default: Box<Node>) -> Result<Box<Node>, ()> {
        let name_l = self.loc(&name_t);
        let name = value(name_t);
        self.check_reserved_for_numparam(&name, &name_l)?;

        let label_l = name_l;
        let name_l = label_l.adjust_end(-1);
        let expression_l = default.expression().join(&label_l);

        Ok(Box::new(Node::Kwoptarg(Kwoptarg {
            name,
            default,
            name_l,
            expression_l,
        })))
    }

    pub(crate) fn kwrestarg(
        &self,
        dstar_t: Box<Token>,
        name_t: Option<Box<Token>>,
    ) -> Result<Box<Node>, ()> {
        let (name, name_l) = match name_t {
            Some(name_t) => {
                let name_l = self.loc(&name_t);
                let name = value(name_t);
                self.check_reserved_for_numparam(&name, &name_l)?;
                (Some(name), Some(name_l))
            }
            _ => (None, None),
        };

        let operator_l = self.loc(&dstar_t);
        let expression_l = operator_l.maybe_join(&name_l);

        Ok(Box::new(Node::Kwrestarg(Kwrestarg {
            name,
            operator_l,
            name_l,
            expression_l,
        })))
    }

    pub(crate) fn kwnilarg(&self, dstar_t: Box<Token>, nil_t: Box<Token>) -> Box<Node> {
        let dstar_l = self.loc(&dstar_t);
        let nil_l = self.loc(&nil_t);
        let expression_l = dstar_l.join(&nil_l);
        Box::new(Node::Kwnilarg(Kwnilarg {
            name_l: nil_l,
            expression_l,
        }))
    }

    pub(crate) fn shadowarg(&self, name_t: Box<Token>) -> Result<Box<Node>, ()> {
        let name_l = self.loc(&name_t);
        let name = value(name_t);
        self.check_reserved_for_numparam(&name, &name_l)?;

        Ok(Box::new(Node::Shadowarg(Shadowarg {
            name,
            expression_l: name_l,
        })))
    }

    pub(crate) fn blockarg(
        &self,
        amper_t: Box<Token>,
        name_t: Box<Token>,
    ) -> Result<Box<Node>, ()> {
        let name_l = self.loc(&name_t);
        let name = value(name_t);
        self.check_reserved_for_numparam(&name, &name_l)?;

        let operator_l = self.loc(&amper_t);
        let expression_l = operator_l.join(&name_l);

        Ok(Box::new(Node::Blockarg(Blockarg {
            name,
            operator_l,
            name_l,
            expression_l,
        })))
    }

    pub(crate) fn procarg0(&self, arg: Box<Node>) -> Box<Node> {
        match *arg {
            Node::Mlhs(Mlhs {
                items,
                begin_l,
                end_l,
                expression_l,
            }) => Box::new(Node::Procarg0(Procarg0 {
                args: items,
                begin_l,
                end_l,
                expression_l,
            })),
            Node::Arg(arg) => Box::new(Node::Procarg0(Procarg0 {
                expression_l: arg.expression_l.clone(),
                args: vec![Node::Arg(arg)],
                begin_l: None,
                end_l: None,
            })),
            other => unreachable!("unsupported procarg0 child {:?}", other),
        }
    }

    //
    // Method calls
    //

    fn call_type_for_dot(&self, dot_t: &Option<Box<Token>>) -> MethodCallType {
        match dot_t {
            Some(token) if token.token_type == Lexer::tANDDOT => MethodCallType::CSend,
            _ => MethodCallType::Send,
        }
    }

    pub(crate) fn forwarded_args(&self, dots_t: Box<Token>) -> Box<Node> {
        Box::new(Node::ForwardedArgs(ForwardedArgs {
            expression_l: self.loc(&dots_t),
        }))
    }

    pub(crate) fn call_method(
        &self,
        receiver: Option<Box<Node>>,
        dot_t: Option<Box<Token>>,
        selector_t: Option<Box<Token>>,
        lparen_t: Option<Box<Token>>,
        mut args: Vec<Node>,
        rparen_t: Option<Box<Token>>,
    ) -> Box<Node> {
        let begin_l = maybe_boxed_node_expr(&receiver)
            .or_else(|| self.maybe_loc(&selector_t))
            .unwrap_or_else(|| unreachable!("can't compute begin_l"));
        let end_l = self
            .maybe_loc(&rparen_t)
            .or_else(|| maybe_node_expr(&args.last()))
            .or_else(|| self.maybe_loc(&selector_t))
            .unwrap_or_else(|| unreachable!("can't compute end_l"));

        let expression_l = begin_l.join(&end_l);

        let dot_l = self.maybe_loc(&dot_t);
        let selector_l = self.maybe_loc(&selector_t);
        let begin_l = self.maybe_loc(&lparen_t);
        let end_l = self.maybe_loc(&rparen_t);

        let method_name = maybe_value(selector_t).unwrap_or_else(|| "call".to_string());

        self.rewrite_hash_args_to_kwargs(&mut args);

        match self.call_type_for_dot(&dot_t) {
            MethodCallType::Send => Box::new(Node::Send(Send {
                method_name,
                recv: receiver,
                args,
                dot_l,
                selector_l,
                begin_l,
                end_l,
                operator_l: None,
                expression_l,
            })),

            MethodCallType::CSend => Box::new(Node::CSend(CSend {
                method_name,
                recv: receiver.expect("csend node must have a receiver"),
                args,
                dot_l: dot_l.expect("csend node must have &."),
                selector_l,
                begin_l,
                end_l,
                operator_l: None,
                expression_l,
            })),
        }
    }

    pub(crate) fn call_lambda(&self, lambda_t: Box<Token>) -> Box<Node> {
        Box::new(Node::Lambda(Lambda {
            expression_l: self.loc(&lambda_t),
        }))
    }

    pub(crate) fn block(
        &self,
        method_call: Box<Node>,
        begin_t: Box<Token>,
        block_args: ArgsType,
        body: Option<Box<Node>>,
        end_t: Box<Token>,
    ) -> Result<Box<Node>, ()> {
        let block_body = body;

        let validate_block_and_block_arg = |args: &Vec<Node>| {
            if let Some(last_arg) = args.last() {
                match last_arg {
                    Node::BlockPass(BlockPass { expression_l, .. })
                    | Node::ForwardedArgs(ForwardedArgs { expression_l, .. }) => {
                        self.error(DiagnosticMessage::BlockAndBlockArgGiven, &expression_l);
                        Err(())
                    }
                    _ => Ok(()),
                }
            } else {
                Ok(())
            }
        };

        match &*method_call {
            Node::Yield(Yield { keyword_l, .. }) => {
                self.error(DiagnosticMessage::BlockGivenToYield, &keyword_l);
                return Err(());
            }
            Node::Send(Send { args, .. }) | Node::CSend(CSend { args, .. }) => {
                validate_block_and_block_arg(args)?;
            }
            _ => {}
        }

        let rewrite_args_and_loc =
            |method_args: &[Node],
             keyword_expression_l: &Loc,
             block_args: ArgsType,
             block_body: Option<Box<Node>>| {
                // Code like "return foo 1 do end" is reduced in a weird sequence.
                // Here, method_call is actually (return).
                let actual_send = method_args[0].clone();

                let begin_l = self.loc(&begin_t);
                let end_l = self.loc(&end_t);
                let expression_l = actual_send.expression().join(&end_l);

                let block = match block_args {
                    ArgsType::Args(args) => Node::Block(Block {
                        call: Box::new(actual_send),
                        args,
                        body: block_body,
                        begin_l,
                        end_l,
                        expression_l,
                    }),
                    ArgsType::Numargs(numargs) => Node::Numblock(Numblock {
                        call: Box::new(actual_send),
                        numargs,
                        body: block_body.expect("numblock always has body"),
                        begin_l,
                        end_l,
                        expression_l,
                    }),
                };

                let expr_l = keyword_expression_l.join(block.expression());
                let args = vec![block];

                (args, expr_l)
            };

        match &*method_call {
            Node::Send(_)
            | Node::CSend(_)
            | Node::Index(_)
            | Node::Super(_)
            | Node::ZSuper(_)
            | Node::Lambda(_) => {
                let begin_l = self.loc(&begin_t);
                let end_l = self.loc(&end_t);
                let expression_l = method_call.expression().join(&end_l);

                let result = match block_args {
                    ArgsType::Args(args) => Node::Block(Block {
                        call: method_call,
                        args,
                        body: block_body,
                        begin_l,
                        end_l,
                        expression_l,
                    }),
                    ArgsType::Numargs(numargs) => Node::Numblock(Numblock {
                        numargs,
                        call: method_call,
                        body: block_body.expect("numblock always has body"),
                        begin_l,
                        end_l,
                        expression_l,
                    }),
                };
                return Ok(Box::new(result));
            }
            _ => {}
        };

        let result = match *method_call {
            Node::Return(Return {
                args,
                keyword_l,
                expression_l,
            }) => {
                let (args, expression_l) =
                    rewrite_args_and_loc(&args, &expression_l, block_args, block_body);
                Node::Return(Return {
                    args,
                    keyword_l,
                    expression_l,
                })
            }
            Node::Next(Next {
                args,
                keyword_l,
                expression_l,
            }) => {
                let (args, expression_l) =
                    rewrite_args_and_loc(&args, &expression_l, block_args, block_body);
                Node::Next(Next {
                    args,
                    keyword_l,
                    expression_l,
                })
            }
            Node::Break(Break {
                args,
                keyword_l,
                expression_l,
            }) => {
                let (args, expression_l) =
                    rewrite_args_and_loc(&args, &expression_l, block_args, block_body);
                Node::Break(Break {
                    args,
                    keyword_l,
                    expression_l,
                })
            }
            _ => unreachable!("unsupported method call {:?}", method_call),
        };

        Ok(Box::new(result))
    }
    pub(crate) fn block_pass(&self, amper_t: Box<Token>, value: Box<Node>) -> Box<Node> {
        let amper_l = self.loc(&amper_t);
        let expression_l = value.expression().join(&amper_l);

        Box::new(Node::BlockPass(BlockPass {
            value,
            operator_l: amper_l,
            expression_l,
        }))
    }

    pub(crate) fn attr_asgn(
        &self,
        receiver: Box<Node>,
        dot_t: Box<Token>,
        selector_t: Box<Token>,
    ) -> Box<Node> {
        let dot_l = self.loc(&dot_t);
        let selector_l = self.loc(&selector_t);
        let expression_l = receiver.expression().join(&selector_l);

        let method_name = value(selector_t) + "=";

        match self.call_type_for_dot(&Some(dot_t)) {
            MethodCallType::Send => Box::new(Node::Send(Send {
                method_name,
                recv: Some(receiver),
                args: vec![],
                dot_l: Some(dot_l),
                selector_l: Some(selector_l),
                begin_l: None,
                end_l: None,
                operator_l: None,
                expression_l,
            })),

            MethodCallType::CSend => Box::new(Node::CSend(CSend {
                method_name,
                recv: receiver,
                args: vec![],
                dot_l,
                selector_l: Some(selector_l),
                begin_l: None,
                end_l: None,
                operator_l: None,
                expression_l,
            })),
        }
    }

    pub(crate) fn index(
        &self,
        recv: Box<Node>,
        lbrack_t: Box<Token>,
        mut indexes: Vec<Node>,
        rbrack_t: Box<Token>,
    ) -> Box<Node> {
        let begin_l = self.loc(&lbrack_t);
        let end_l = self.loc(&rbrack_t);
        let expression_l = recv.expression().join(&end_l);

        self.rewrite_hash_args_to_kwargs(&mut indexes);

        Box::new(Node::Index(Index {
            recv,
            indexes,
            begin_l,
            end_l,
            expression_l,
        }))
    }

    pub(crate) fn index_asgn(
        &self,
        recv: Box<Node>,
        lbrack_t: Box<Token>,
        indexes: Vec<Node>,
        rbrack_t: Box<Token>,
    ) -> Box<Node> {
        let begin_l = self.loc(&lbrack_t);
        let end_l = self.loc(&rbrack_t);
        let expression_l = recv.expression().join(&end_l);

        Box::new(Node::IndexAsgn(IndexAsgn {
            recv,
            indexes,
            value: None,
            begin_l,
            end_l,
            operator_l: None,
            expression_l,
        }))
    }

    pub(crate) fn binary_op(
        &self,
        receiver: Box<Node>,
        operator_t: Box<Token>,
        arg: Box<Node>,
    ) -> Result<Box<Node>, ()> {
        self.value_expr(&receiver)?;
        self.value_expr(&arg)?;

        let selector_l = self.loc(&operator_t);
        let expression_l = join_exprs(&receiver, &arg);

        Ok(Box::new(Node::Send(Send {
            recv: Some(receiver),
            method_name: value(operator_t),
            args: vec![*arg],
            dot_l: None,
            selector_l: Some(selector_l),
            begin_l: None,
            end_l: None,
            operator_l: None,
            expression_l,
        })))
    }

    pub(crate) fn match_op(
        &self,
        receiver: Box<Node>,
        match_t: Box<Token>,
        arg: Box<Node>,
    ) -> Result<Box<Node>, ()> {
        self.value_expr(&receiver)?;
        self.value_expr(&arg)?;

        let selector_l = self.loc(&match_t);
        let expression_l = join_exprs(&receiver, &arg);

        let result = match self.static_regexp_captures(&receiver) {
            Some(captures) => {
                for capture in captures {
                    self.static_env.declare(&capture);
                }

                Node::MatchWithLvasgn(MatchWithLvasgn {
                    re: receiver,
                    value: arg,
                    operator_l: selector_l,
                    expression_l,
                })
            }
            None => Node::Send(Send {
                recv: Some(receiver),
                method_name: String::from("=~"),
                args: vec![*arg],
                dot_l: None,
                selector_l: Some(selector_l),
                begin_l: None,
                end_l: None,
                operator_l: None,
                expression_l,
            }),
        };

        Ok(Box::new(result))
    }

    pub(crate) fn unary_op(&self, op_t: Box<Token>, receiver: Box<Node>) -> Result<Box<Node>, ()> {
        self.value_expr(&receiver)?;

        let selector_l = self.loc(&op_t);
        let expression_l = receiver.expression().join(&selector_l);

        let op = value(op_t);
        let method = if op == "+" || op == "-" { op + "@" } else { op };
        Ok(Box::new(Node::Send(Send {
            recv: Some(receiver),
            method_name: method,
            args: vec![],
            dot_l: None,
            selector_l: Some(selector_l),
            begin_l: None,
            end_l: None,
            operator_l: None,
            expression_l,
        })))
    }

    pub(crate) fn not_op(
        &self,
        not_t: Box<Token>,
        begin_t: Option<Box<Token>>,
        receiver: Option<Box<Node>>,
        end_t: Option<Box<Token>>,
    ) -> Result<Box<Node>, ()> {
        if let Some(receiver) = receiver {
            self.value_expr(&receiver)?;

            let begin_l = self.loc(&not_t);
            let end_l = self
                .maybe_loc(&end_t)
                .unwrap_or_else(|| Ptr::new(receiver.expression().clone()));

            let expression_l = begin_l.join(&end_l);

            let selector_l = self.loc(&not_t);
            let begin_l = self.maybe_loc(&begin_t);
            let end_l = self.maybe_loc(&end_t);

            Ok(Box::new(Node::Send(Send {
                recv: Some(self.check_condition(receiver)),
                method_name: "!".to_string(),
                args: vec![],
                selector_l: Some(selector_l),
                dot_l: None,
                begin_l,
                end_l,
                operator_l: None,
                expression_l,
            })))
        } else {
            let CollectionMap {
                begin_l,
                end_l,
                expression_l,
            } = self.collection_map(&begin_t, &[], &end_t);

            let nil_node = Node::Begin(Begin {
                statements: vec![],
                begin_l,
                end_l,
                expression_l,
            });

            let selector_l = self.loc(&not_t);
            let expression_l = nil_node.expression().join(&selector_l);
            Ok(Box::new(Node::Send(Send {
                recv: Some(Box::new(nil_node)),
                method_name: "!".to_string(),
                args: vec![],
                selector_l: Some(selector_l),
                dot_l: None,
                begin_l: None,
                end_l: None,
                operator_l: None,
                expression_l,
            })))
        }
    }

    //
    // Control flow
    //

    // Logical operations: and, or

    pub(crate) fn logical_op(
        &self,
        type_: LogicalOp,
        lhs: Box<Node>,
        op_t: Box<Token>,
        rhs: Box<Node>,
    ) -> Result<Box<Node>, ()> {
        self.value_expr(&lhs)?;

        let operator_l = self.loc(&op_t);
        let expression_l = join_exprs(&lhs, &rhs);

        let result = match type_ {
            LogicalOp::And => Node::And(And {
                lhs,
                rhs,
                operator_l,
                expression_l,
            }),
            LogicalOp::Or => Node::Or(Or {
                lhs,
                rhs,
                operator_l,
                expression_l,
            }),
        };
        Ok(Box::new(result))
    }

    // Conditionals

    pub(crate) fn condition(
        &self,
        cond_t: Box<Token>,
        cond: Box<Node>,
        then_t: Box<Token>,
        if_true: Option<Box<Node>>,
        else_t: Option<Box<Token>>,
        if_false: Option<Box<Node>>,
        end_t: Option<Box<Token>>,
    ) -> Box<Node> {
        let end_l = self
            .maybe_loc(&end_t)
            .or_else(|| maybe_boxed_node_expr(&if_false))
            .or_else(|| self.maybe_loc(&else_t))
            .or_else(|| maybe_boxed_node_expr(&if_true))
            .unwrap_or_else(|| self.loc(&then_t));

        let expression_l = self.loc(&cond_t).join(&end_l);
        let keyword_l = self.loc(&cond_t);
        let begin_l = self.loc(&then_t);
        let else_l = self.maybe_loc(&else_t);
        let end_l = self.maybe_loc(&end_t);

        Box::new(Node::If(If {
            cond: self.check_condition(cond),
            if_true,
            if_false,
            keyword_l,
            begin_l,
            else_l,
            end_l,
            expression_l,
        }))
    }

    pub(crate) fn condition_mod(
        &self,
        if_true: Option<Box<Node>>,
        if_false: Option<Box<Node>>,
        cond_t: Box<Token>,
        cond: Box<Node>,
    ) -> Box<Node> {
        let pre = match (&if_true, &if_false) {
            (None, None) => unreachable!("at least one of if_true/if_false is required"),
            (None, Some(if_false)) => if_false,
            (Some(if_true), None) => if_true,
            (Some(_), Some(_)) => unreachable!("only one of if_true/if_false is required"),
        };

        let expression_l = pre.expression().join(&cond.expression());
        let keyword_l = self.loc(&cond_t);

        Box::new(Node::IfMod(IfMod {
            cond: self.check_condition(cond),
            if_true,
            if_false,
            keyword_l,
            expression_l,
        }))
    }

    pub(crate) fn ternary(
        &self,
        cond: Box<Node>,
        question_t: Box<Token>,
        if_true: Box<Node>,
        colon_t: Box<Token>,
        if_false: Box<Node>,
    ) -> Box<Node> {
        let expression_l = join_exprs(&cond, &if_false);
        let question_l = self.loc(&question_t);
        let colon_l = self.loc(&colon_t);

        Box::new(Node::IfTernary(IfTernary {
            cond,
            if_true,
            if_false,
            question_l,
            colon_l,
            expression_l,
        }))
    }

    // Case matching

    pub(crate) fn when(
        &self,
        when_t: Box<Token>,
        patterns: Vec<Node>,
        then_t: Box<Token>,
        body: Option<Box<Node>>,
    ) -> Box<Node> {
        let begin_l = self.loc(&then_t);

        let expr_end_l = maybe_boxed_node_expr(&body)
            .or_else(|| maybe_node_expr(&patterns.last()))
            .unwrap_or_else(|| self.loc(&when_t));
        let when_l = self.loc(&when_t);
        let expression_l = when_l.join(&expr_end_l);

        Box::new(Node::When(When {
            patterns,
            body,
            keyword_l: when_l,
            begin_l,
            expression_l,
        }))
    }

    pub(crate) fn case(
        &self,
        case_t: Box<Token>,
        expr: Option<Box<Node>>,
        when_bodies: Vec<Node>,
        else_t: Option<Box<Token>>,
        else_body: Option<Box<Node>>,
        end_t: Box<Token>,
    ) -> Box<Node> {
        let keyword_l = self.loc(&case_t);
        let else_l = self.maybe_loc(&else_t);
        let end_l = self.loc(&end_t);
        let expression_l = keyword_l.join(&end_l);

        Box::new(Node::Case(Case {
            expr,
            when_bodies,
            else_body,
            keyword_l,
            else_l,
            end_l,
            expression_l,
        }))
    }

    // Loops

    pub(crate) fn loop_(
        &self,
        loop_type: LoopType,
        keyword_t: Box<Token>,
        cond: Box<Node>,
        do_t: Box<Token>,
        body: Option<Box<Node>>,
        end_t: Box<Token>,
    ) -> Box<Node> {
        let keyword_l = self.loc(&keyword_t);
        let begin_l = self.loc(&do_t);
        let end_l = self.loc(&end_t);
        let expression_l = self.loc(&keyword_t).join(&end_l);

        let cond = self.check_condition(cond);

        match loop_type {
            LoopType::While => Box::new(Node::While(While {
                cond,
                body,
                keyword_l,
                begin_l: Some(begin_l),
                end_l: Some(end_l),
                expression_l,
            })),
            LoopType::Until => Box::new(Node::Until(Until {
                cond,
                body,
                keyword_l,
                begin_l: Some(begin_l),
                end_l: Some(end_l),
                expression_l,
            })),
        }
    }

    pub(crate) fn loop_mod(
        &self,
        loop_type: LoopType,
        body: Box<Node>,
        keyword_t: Box<Token>,
        cond: Box<Node>,
    ) -> Box<Node> {
        let expression_l = body.expression().join(&cond.expression());
        let keyword_l = self.loc(&keyword_t);

        let cond = self.check_condition(cond);

        match (loop_type, &*body) {
            (LoopType::While, Node::KwBegin(_)) => Box::new(Node::WhilePost(WhilePost {
                cond,
                body,
                keyword_l,
                expression_l,
            })),
            (LoopType::While, _) => Box::new(Node::While(While {
                cond,
                body: Some(body),
                keyword_l,
                expression_l,
                begin_l: None,
                end_l: None,
            })),
            (LoopType::Until, Node::KwBegin(_)) => Box::new(Node::UntilPost(UntilPost {
                cond,
                body,
                keyword_l,
                expression_l,
            })),
            (LoopType::Until, _) => Box::new(Node::Until(Until {
                cond,
                body: Some(body),
                keyword_l,
                expression_l,
                begin_l: None,
                end_l: None,
            })),
        }
    }

    pub(crate) fn for_(
        &self,
        for_t: Box<Token>,
        iterator: Box<Node>,
        in_t: Box<Token>,
        iteratee: Box<Node>,
        do_t: Box<Token>,
        body: Option<Box<Node>>,
        end_t: Box<Token>,
    ) -> Box<Node> {
        let keyword_l = self.loc(&for_t);
        let operator_l = self.loc(&in_t);
        let begin_l = self.loc(&do_t);
        let end_l = self.loc(&end_t);
        let expression_l = keyword_l.join(&end_l);

        Box::new(Node::For(For {
            iterator,
            iteratee,
            body,
            keyword_l,
            operator_l,
            begin_l,
            end_l,
            expression_l,
        }))
    }

    // Keywords

    pub(crate) fn keyword_cmd(
        &self,
        type_: KeywordCmd,
        keyword_t: Box<Token>,
        lparen_t: Option<Box<Token>>,
        mut args: Vec<Node>,
        rparen_t: Option<Box<Token>>,
    ) -> Result<Box<Node>, ()> {
        let keyword_l = self.loc(&keyword_t);

        if type_ == KeywordCmd::Yield && !args.is_empty() {
            if let Some(Node::BlockPass(_)) = args.last() {
                self.error(DiagnosticMessage::BlockGivenToYield, &keyword_l);
                return Err(());
            }
        }

        match type_ {
            KeywordCmd::Yield | KeywordCmd::Super => {
                self.rewrite_hash_args_to_kwargs(&mut args);
            }
            _ => {}
        }

        let begin_l = self.maybe_loc(&lparen_t);
        let end_l = self.maybe_loc(&rparen_t);

        let expr_end_l = end_l
            .clone()
            .or_else(|| maybe_node_expr(&args.last()))
            .unwrap_or_else(|| keyword_l.clone());

        let expression_l = keyword_l.join(&expr_end_l);

        let result = match type_ {
            KeywordCmd::Break => Node::Break(Break {
                args,
                keyword_l,
                expression_l,
            }),
            KeywordCmd::Defined => Node::Defined(Defined {
                value: Box::new(args.pop().expect("defined? always has an argument")),
                keyword_l,
                begin_l,
                end_l,
                expression_l,
            }),
            KeywordCmd::Next => Node::Next(Next {
                args,
                keyword_l,
                expression_l,
            }),
            KeywordCmd::Redo => Node::Redo(Redo { expression_l }),
            KeywordCmd::Retry => Node::Retry(Retry { expression_l }),
            KeywordCmd::Return => Node::Return(Return {
                args,
                keyword_l,
                expression_l,
            }),
            KeywordCmd::Super => Node::Super(Super {
                args,
                keyword_l,
                begin_l,
                end_l,
                expression_l,
            }),
            KeywordCmd::Yield => Node::Yield(Yield {
                args,
                keyword_l,
                begin_l,
                end_l,
                expression_l,
            }),
            KeywordCmd::Zsuper => Node::ZSuper(ZSuper { expression_l }),
        };

        Ok(Box::new(result))
    }

    // BEGIN, END

    pub(crate) fn preexe(
        &self,
        preexe_t: Box<Token>,
        lbrace_t: Box<Token>,
        body: Option<Box<Node>>,
        rbrace_t: Box<Token>,
    ) -> Box<Node> {
        let keyword_l = self.loc(&preexe_t);
        let begin_l = self.loc(&lbrace_t);
        let end_l = self.loc(&rbrace_t);
        let expression_l = keyword_l.join(&end_l);

        Box::new(Node::Preexe(Preexe {
            body,
            keyword_l,
            begin_l,
            end_l,
            expression_l,
        }))
    }
    pub(crate) fn postexe(
        &self,
        postexe_t: Box<Token>,
        lbrace_t: Box<Token>,
        body: Option<Box<Node>>,
        rbrace_t: Box<Token>,
    ) -> Box<Node> {
        let keyword_l = self.loc(&postexe_t);
        let begin_l = self.loc(&lbrace_t);
        let end_l = self.loc(&rbrace_t);
        let expression_l = keyword_l.join(&end_l);

        Box::new(Node::Postexe(Postexe {
            body,
            keyword_l,
            begin_l,
            end_l,
            expression_l,
        }))
    }

    // Exception handling

    pub(crate) fn rescue_body(
        &self,
        rescue_t: Box<Token>,
        exc_list: Option<Box<Node>>,
        assoc_t: Option<Box<Token>>,
        exc_var: Option<Box<Node>>,
        then_t: Option<Box<Token>>,
        body: Option<Box<Node>>,
    ) -> Box<Node> {
        let end_l = maybe_boxed_node_expr(&body)
            .or_else(|| self.maybe_loc(&then_t))
            .or_else(|| maybe_boxed_node_expr(&exc_var))
            .or_else(|| maybe_boxed_node_expr(&exc_list))
            .unwrap_or_else(|| self.loc(&rescue_t));

        let expression_l = self.loc(&rescue_t).join(&end_l);
        let keyword_l = self.loc(&rescue_t);
        let assoc_l = self.maybe_loc(&assoc_t);
        let begin_l = self.maybe_loc(&then_t);

        Box::new(Node::RescueBody(RescueBody {
            exc_list,
            exc_var,
            body,
            keyword_l,
            begin_l,
            assoc_l,
            expression_l,
        }))
    }

    pub(crate) fn begin_body(
        &self,
        compound_stmt: Option<Box<Node>>,
        rescue_bodies: Vec<Node>,
        else_: Option<(Box<Token>, Option<Box<Node>>)>,
        ensure: Option<(Box<Token>, Option<Box<Node>>)>,
    ) -> Option<Box<Node>> {
        let mut result: Option<Box<Node>>;

        if !rescue_bodies.is_empty() {
            if let Some((else_t, else_)) = else_ {
                let begin_l = maybe_boxed_node_expr(&compound_stmt)
                    .or_else(|| maybe_node_expr(&rescue_bodies.first()))
                    .unwrap_or_else(|| unreachable!("can't compute begin_l"));

                let end_l = maybe_boxed_node_expr(&else_).unwrap_or_else(|| self.loc(&else_t));

                let expression_l = begin_l.join(&end_l);
                let else_l = self.loc(&else_t);

                result = Some(Box::new(Node::Rescue(Rescue {
                    body: compound_stmt,
                    rescue_bodies,
                    else_,
                    else_l: Some(else_l),
                    expression_l,
                })))
            } else {
                let begin_l = maybe_boxed_node_expr(&compound_stmt)
                    .or_else(|| maybe_node_expr(&rescue_bodies.first()))
                    .unwrap_or_else(|| unreachable!("can't compute begin_l"));

                let end_l = maybe_node_expr(&rescue_bodies.last())
                    .unwrap_or_else(|| unreachable!("can't compute end_l"));

                let expression_l = begin_l.join(&end_l);
                let else_l = self.maybe_loc(&None);

                result = Some(Box::new(Node::Rescue(Rescue {
                    body: compound_stmt,
                    rescue_bodies,
                    else_: None,
                    else_l,
                    expression_l,
                })))
            }
        } else if let Some((else_t, else_)) = else_ {
            let mut statements: Vec<Node> = vec![];

            match compound_stmt.map(|boxed| *boxed) {
                Some(Node::Begin(Begin {
                    statements: stmts, ..
                })) => statements = stmts,
                Some(compound_stmt) => statements.push(compound_stmt),
                _ => {}
            }
            let parts = if let Some(else_) = else_ {
                vec![*else_]
            } else {
                vec![]
            };
            let CollectionMap {
                begin_l,
                end_l,
                expression_l,
            } = self.collection_map(&Some(else_t), &parts, &None);

            statements.push(Node::Begin(Begin {
                statements: parts,
                begin_l,
                end_l,
                expression_l,
            }));

            let CollectionMap {
                begin_l,
                end_l,
                expression_l,
            } = self.collection_map(&None, &statements, &None);

            result = Some(Box::new(Node::Begin(Begin {
                statements,
                begin_l,
                end_l,
                expression_l,
            })))
        } else {
            result = compound_stmt;
        }

        if let Some((ensure_t, ensure)) = ensure {
            let mut ensure_body = if let Some(ensure) = ensure {
                vec![*ensure]
            } else {
                vec![]
            };
            let keyword_l = self.loc(&ensure_t);

            let begin_l = maybe_boxed_node_expr(&result).unwrap_or_else(|| self.loc(&ensure_t));

            let end_l = maybe_node_expr(&ensure_body.last()).unwrap_or_else(|| self.loc(&ensure_t));

            let expression_l = begin_l.join(&end_l);

            result = Some(Box::new(Node::Ensure(Ensure {
                body: result,
                ensure: ensure_body.pop().map(Box::new),
                keyword_l,
                expression_l,
            })))
        }

        result
    }

    //
    // Expression grouping
    //

    pub(crate) fn compstmt(&self, mut statements: Vec<Node>) -> Option<Box<Node>> {
        match &statements[..] {
            [] => None,
            [_] => statements.pop().map(Box::new),
            _ => {
                let CollectionMap {
                    begin_l,
                    end_l,
                    expression_l,
                } = self.collection_map(&None, &statements, &None);

                Some(Box::new(Node::Begin(Begin {
                    statements,
                    begin_l,
                    end_l,
                    expression_l,
                })))
            }
        }
    }

    pub(crate) fn begin(
        &self,
        begin_t: Box<Token>,
        body: Option<Box<Node>>,
        end_t: Box<Token>,
    ) -> Box<Node> {
        let new_begin_l = self.loc(&begin_t);
        let new_end_l = self.loc(&end_t);
        let new_expression_l = new_begin_l.join(&new_end_l);

        if let Some(mut body) = body {
            match &mut *body {
                // Synthesized (begin) from compstmt "a; b" or (mlhs)
                // from multi_lhs "(a, b) = *foo".
                Node::Mlhs(Mlhs {
                    begin_l,
                    end_l,
                    expression_l,
                    ..
                }) => {
                    *begin_l = Some(new_begin_l);
                    *end_l = Some(new_end_l);
                    *expression_l = new_expression_l;
                    body
                }
                Node::Begin(Begin {
                    begin_l,
                    end_l,
                    expression_l,
                    ..
                }) if begin_l.is_none() && end_l.is_none() => {
                    *begin_l = Some(new_begin_l);
                    *end_l = Some(new_end_l);
                    *expression_l = new_expression_l;
                    body
                }
                _ => {
                    let statements = vec![*body];
                    Box::new(Node::Begin(Begin {
                        statements,
                        begin_l: Some(new_begin_l),
                        end_l: Some(new_end_l),
                        expression_l: new_expression_l,
                    }))
                }
            }
        } else {
            // A nil expression: `()'.
            Box::new(Node::Begin(Begin {
                statements: vec![],
                begin_l: Some(new_begin_l),
                end_l: Some(new_end_l),
                expression_l: new_expression_l,
            }))
        }
    }

    pub(crate) fn begin_keyword(
        &self,
        begin_t: Box<Token>,
        body: Option<Box<Node>>,
        end_t: Box<Token>,
    ) -> Box<Node> {
        let begin_l = self.loc(&begin_t);
        let end_l = self.loc(&end_t);
        let expression_l = begin_l.join(&end_l);

        match body.map(|boxed| *boxed) {
            None => {
                // A nil expression: `begin end'.
                Box::new(Node::KwBegin(KwBegin {
                    statements: vec![],
                    begin_l: Some(begin_l),
                    end_l: Some(end_l),
                    expression_l,
                }))
            }
            Some(Node::Begin(Begin { statements, .. })) => {
                // Synthesized (begin) from compstmt "a; b".
                Box::new(Node::KwBegin(KwBegin {
                    statements,
                    begin_l: Some(begin_l),
                    end_l: Some(end_l),
                    expression_l,
                }))
            }
            Some(node) => {
                let statements = vec![node];
                Box::new(Node::KwBegin(KwBegin {
                    statements,
                    begin_l: Some(begin_l),
                    end_l: Some(end_l),
                    expression_l,
                }))
            }
        }
    }

    //
    // Pattern matching
    //

    pub(crate) fn case_match(
        &self,
        case_t: Box<Token>,
        expr: Box<Node>,
        in_bodies: Vec<Node>,
        else_t: Option<Box<Token>>,
        else_body: Option<Box<Node>>,
        end_t: Box<Token>,
    ) -> Box<Node> {
        let else_body = match (&else_t, &else_body) {
            (Some(else_t), None) => Some(Box::new(Node::EmptyElse(EmptyElse {
                expression_l: self.loc(else_t),
            }))),
            _ => else_body,
        };

        let keyword_l = self.loc(&case_t);
        let else_l = self.maybe_loc(&else_t);
        let end_l = self.loc(&end_t);
        let expression_l = self.loc(&case_t).join(&end_l);

        Box::new(Node::CaseMatch(CaseMatch {
            expr,
            in_bodies,
            else_body,
            keyword_l,
            else_l,
            end_l,
            expression_l,
        }))
    }

    pub(crate) fn match_pattern(
        &self,
        value: Box<Node>,
        assoc_t: Box<Token>,
        pattern: Box<Node>,
    ) -> Box<Node> {
        let operator_l = self.loc(&assoc_t);
        let expression_l = join_exprs(&value, &pattern);

        Box::new(Node::MatchPattern(MatchPattern {
            value,
            pattern,
            operator_l,
            expression_l,
        }))
    }

    pub(crate) fn match_pattern_p(
        &self,
        value: Box<Node>,
        in_t: Box<Token>,
        pattern: Box<Node>,
    ) -> Box<Node> {
        let operator_l = self.loc(&in_t);
        let expression_l = join_exprs(&value, &pattern);

        Box::new(Node::MatchPatternP(MatchPatternP {
            value,
            pattern,
            operator_l,
            expression_l,
        }))
    }

    pub(crate) fn in_pattern(
        &self,
        in_t: Box<Token>,
        pattern: Box<Node>,
        guard: Option<Box<Node>>,
        then_t: Box<Token>,
        body: Option<Box<Node>>,
    ) -> Box<Node> {
        let keyword_l = self.loc(&in_t);
        let begin_l = self.loc(&then_t);

        let expression_l = maybe_boxed_node_expr(&body)
            .or_else(|| maybe_boxed_node_expr(&guard))
            .unwrap_or_else(|| Ptr::new(pattern.expression().clone()))
            .join(&keyword_l);

        Box::new(Node::InPattern(InPattern {
            pattern,
            guard,
            body,
            keyword_l,
            begin_l,
            expression_l,
        }))
    }

    pub(crate) fn if_guard(&self, if_t: Box<Token>, cond: Box<Node>) -> Box<Node> {
        let keyword_l = self.loc(&if_t);
        let expression_l = keyword_l.join(cond.expression());

        Box::new(Node::IfGuard(IfGuard {
            cond,
            keyword_l,
            expression_l,
        }))
    }
    pub(crate) fn unless_guard(&self, unless_t: Box<Token>, cond: Box<Node>) -> Box<Node> {
        let keyword_l = self.loc(&unless_t);
        let expression_l = keyword_l.join(cond.expression());

        Box::new(Node::UnlessGuard(UnlessGuard {
            cond,
            keyword_l,
            expression_l,
        }))
    }

    pub(crate) fn match_var(&self, name_t: Box<Token>) -> Result<Box<Node>, ()> {
        let name_l = self.loc(&name_t);
        let expression_l = name_l.clone();
        let name = value(name_t);

        self.check_lvar_name(&name, &name_l)?;
        self.check_duplicate_pattern_variable(&name, &name_l)?;
        self.static_env.declare(&name);

        Ok(Box::new(Node::MatchVar(MatchVar {
            name,
            name_l,
            expression_l,
        })))
    }

    pub(crate) fn match_hash_var(&self, name_t: Box<Token>) -> Result<Box<Node>, ()> {
        let expression_l = self.loc(&name_t);
        let name_l = expression_l.adjust_end(-1);

        let name = value(name_t);

        self.check_lvar_name(&name, &name_l)?;
        self.check_duplicate_pattern_variable(&name, &name_l)?;
        self.static_env.declare(&name);

        Ok(Box::new(Node::MatchVar(MatchVar {
            name,
            name_l,
            expression_l,
        })))
    }
    pub(crate) fn match_hash_var_from_str(
        &self,
        begin_t: Box<Token>,
        mut strings: Vec<Node>,
        end_t: Box<Token>,
    ) -> Result<Box<Node>, ()> {
        if strings.len() != 1 {
            self.error(
                DiagnosticMessage::SymbolLiteralWithInterpolation,
                &self.loc(&begin_t).join(&self.loc(&end_t)),
            );
            return Err(());
        }

        let result = match strings.remove(0) {
            Node::Str(Str {
                value,
                begin_l,
                end_l,
                expression_l,
            }) => {
                let name = value.bytes.to_string_lossy();
                let mut name_l = expression_l.clone();

                self.check_lvar_name(&name, &name_l)?;
                self.check_duplicate_pattern_variable(&name, &name_l)?;

                self.static_env.declare(&name);

                if let Some(begin_l) = &begin_l {
                    let begin_d: i32 = begin_l
                        .size()
                        .try_into()
                        .expect("failed to convert usize loc into i32, is it too big?");
                    name_l = name_l.adjust_begin(begin_d)
                }

                if let Some(end_l) = &end_l {
                    let end_d: i32 = end_l
                        .size()
                        .try_into()
                        .expect("failed to convert usize loc into i32, is it too big?");
                    name_l = name_l.adjust_end(-end_d)
                }

                let expression_l = self
                    .loc(&begin_t)
                    .join(&expression_l)
                    .join(&self.loc(&end_t));
                Box::new(Node::MatchVar(MatchVar {
                    name,
                    name_l,
                    expression_l,
                }))
            }
            Node::Begin(Begin { statements, .. }) => {
                self.match_hash_var_from_str(begin_t, statements, end_t)?
            }
            _ => {
                self.error(
                    DiagnosticMessage::SymbolLiteralWithInterpolation,
                    &self.loc(&begin_t).join(&self.loc(&end_t)),
                );
                return Err(());
            }
        };

        Ok(result)
    }

    pub(crate) fn match_rest(
        &self,
        star_t: Box<Token>,
        name_t: Option<Box<Token>>,
    ) -> Result<Box<Node>, ()> {
        let name = match name_t {
            None => None,
            Some(t) => Some(self.match_var(t)?),
        };

        let operator_l = self.loc(&star_t);
        let expression_l = operator_l.maybe_join(&maybe_boxed_node_expr(&name));

        Ok(Box::new(Node::MatchRest(MatchRest {
            name,
            operator_l,
            expression_l,
        })))
    }

    pub(crate) fn hash_pattern(
        &self,
        lbrace_t: Option<Box<Token>>,
        kwargs: Vec<Node>,
        rbrace_t: Option<Box<Token>>,
    ) -> Box<Node> {
        let CollectionMap {
            begin_l,
            end_l,
            expression_l,
        } = self.collection_map(&lbrace_t, &kwargs, &rbrace_t);

        Box::new(Node::HashPattern(HashPattern {
            elements: kwargs,
            begin_l,
            end_l,
            expression_l,
        }))
    }

    pub(crate) fn array_pattern(
        &self,
        lbrack_t: Option<Box<Token>>,
        elements: Vec<Node>,
        trailing_comma: Option<Box<Token>>,
        rbrack_t: Option<Box<Token>>,
    ) -> Box<Node> {
        let CollectionMap {
            begin_l,
            end_l,
            expression_l,
        } = self.collection_map(&lbrack_t, &elements, &rbrack_t);

        let expression_l = expression_l.maybe_join(&self.maybe_loc(&trailing_comma));

        if elements.is_empty() {
            return Box::new(Node::ArrayPattern(ArrayPattern {
                elements: vec![],
                begin_l,
                end_l,
                expression_l,
            }));
        }

        if trailing_comma.is_some() {
            Box::new(Node::ArrayPatternWithTail(ArrayPatternWithTail {
                elements,
                begin_l,
                end_l,
                expression_l,
            }))
        } else {
            Box::new(Node::ArrayPattern(ArrayPattern {
                elements,
                begin_l,
                end_l,
                expression_l,
            }))
        }
    }

    pub(crate) fn find_pattern(
        &self,
        lbrack_t: Option<Box<Token>>,
        elements: Vec<Node>,
        rbrack_t: Option<Box<Token>>,
    ) -> Box<Node> {
        let CollectionMap {
            begin_l,
            end_l,
            expression_l,
        } = self.collection_map(&lbrack_t, &elements, &rbrack_t);

        Box::new(Node::FindPattern(FindPattern {
            elements,
            begin_l,
            end_l,
            expression_l,
        }))
    }

    pub(crate) fn const_pattern(
        &self,
        const_: Box<Node>,
        ldelim_t: Box<Token>,
        pattern: Box<Node>,
        rdelim_t: Box<Token>,
    ) -> Box<Node> {
        let begin_l = self.loc(&ldelim_t);
        let end_l = self.loc(&rdelim_t);
        let expression_l = const_.expression().join(&self.loc(&rdelim_t));

        Box::new(Node::ConstPattern(ConstPattern {
            const_,
            pattern,
            begin_l,
            end_l,
            expression_l,
        }))
    }

    pub(crate) fn pin(&self, pin_t: Box<Token>, var: Box<Node>) -> Box<Node> {
        let operator_l = self.loc(&pin_t);
        let expression_l = var.expression().join(&operator_l);

        Box::new(Node::Pin(Pin {
            var,
            selector_l: operator_l,
            expression_l,
        }))
    }

    pub(crate) fn match_alt(
        &self,
        lhs: Box<Node>,
        pipe_t: Box<Token>,
        rhs: Box<Node>,
    ) -> Box<Node> {
        let operator_l = self.loc(&pipe_t);
        let expression_l = join_exprs(&lhs, &rhs);

        Box::new(Node::MatchAlt(MatchAlt {
            lhs,
            rhs,
            operator_l,
            expression_l,
        }))
    }

    pub(crate) fn match_as(
        &self,
        value: Box<Node>,
        assoc_t: Box<Token>,
        as_: Box<Node>,
    ) -> Box<Node> {
        let operator_l = self.loc(&assoc_t);
        let expression_l = join_exprs(&value, &as_);

        Box::new(Node::MatchAs(MatchAs {
            value,
            as_,
            operator_l,
            expression_l,
        }))
    }

    pub(crate) fn match_nil_pattern(&self, dstar_t: Box<Token>, nil_t: Box<Token>) -> Box<Node> {
        let operator_l = self.loc(&dstar_t);
        let name_l = self.loc(&nil_t);
        let expression_l = operator_l.join(&name_l);

        Box::new(Node::MatchNilPattern(MatchNilPattern {
            operator_l,
            name_l,
            expression_l,
        }))
    }

    pub(crate) fn match_pair(
        &self,
        p_kw_label: PKwLabel,
        value: Box<Node>,
    ) -> Result<Box<Node>, ()> {
        let result = match p_kw_label {
            PKwLabel::PlainLabel(label_t) => {
                self.check_duplicate_pattern_key(&clone_value(&label_t), &self.loc(&label_t))?;
                self.pair_keyword(label_t, value)
            }
            PKwLabel::QuotedLabel((begin_t, parts, end_t)) => {
                let label_loc = self.loc(&begin_t).join(&self.loc(&end_t));

                match self.static_string(&parts) {
                    Some(var_name) => self.check_duplicate_pattern_key(&var_name, &label_loc)?,
                    _ => {
                        self.error(
                            DiagnosticMessage::SymbolLiteralWithInterpolation,
                            &label_loc,
                        );
                        return Err(());
                    }
                }

                self.pair_quoted(begin_t, parts, end_t, value)
            }
        };
        Ok(result)
    }

    pub(crate) fn match_label(&self, p_kw_label: PKwLabel) -> Result<Box<Node>, ()> {
        match p_kw_label {
            PKwLabel::PlainLabel(label_t) => self.match_hash_var(label_t),
            PKwLabel::QuotedLabel((begin_t, parts, end_t)) => {
                self.match_hash_var_from_str(begin_t, parts, end_t)
            }
        }
    }

    //
    // Verification
    //

    pub(crate) fn check_condition(&self, cond: Box<Node>) -> Box<Node> {
        match *cond {
            Node::Begin(Begin {
                statements,
                begin_l,
                end_l,
                expression_l,
                ..
            }) => {
                if statements.len() == 1 {
                    let stmt = first(statements);
                    let stmt = *self.check_condition(Box::new(stmt));
                    Box::new(Node::Begin(Begin {
                        statements: vec![stmt],
                        begin_l,
                        end_l,
                        expression_l,
                    }))
                } else {
                    Box::new(Node::Begin(Begin {
                        statements,
                        begin_l,
                        end_l,
                        expression_l,
                    }))
                }
            }
            Node::And(And {
                lhs,
                rhs,
                operator_l,
                expression_l,
            }) => {
                let lhs = self.check_condition(lhs);
                let rhs = self.check_condition(rhs);
                Box::new(Node::And(And {
                    lhs,
                    rhs,
                    operator_l,
                    expression_l,
                }))
            }
            Node::Or(Or {
                lhs,
                rhs,
                operator_l,
                expression_l,
            }) => {
                let lhs = self.check_condition(lhs);
                let rhs = self.check_condition(rhs);
                Box::new(Node::Or(Or {
                    lhs,
                    rhs,
                    operator_l,
                    expression_l,
                }))
            }
            Node::Irange(Irange {
                left,
                right,
                operator_l,
                expression_l,
            }) => Box::new(Node::IFlipFlop(IFlipFlop {
                left: left.map(|node| self.check_condition(node)),
                right: right.map(|node| self.check_condition(node)),
                operator_l,
                expression_l,
            })),
            Node::Erange(Erange {
                left,
                right,
                operator_l,
                expression_l,
            }) => Box::new(Node::EFlipFlop(EFlipFlop {
                left: left.map(|node| self.check_condition(node)),
                right: right.map(|node| self.check_condition(node)),
                operator_l,
                expression_l,
            })),
            Node::Regexp(inner) => Box::new(Node::MatchCurrentLine(MatchCurrentLine {
                expression_l: inner.expression_l.clone(),
                re: Box::new(Node::Regexp(inner)),
            })),
            _ => cond,
        }
    }

    pub(crate) fn check_duplicate_args<'a>(
        &self,
        args: &'a [Node],
        map: &mut HashMap<String, &'a Node>,
    ) {
        for arg in args {
            match arg {
                Node::Arg(_)
                | Node::Optarg(_)
                | Node::Restarg(_)
                | Node::Kwarg(_)
                | Node::Kwoptarg(_)
                | Node::Kwrestarg(_)
                | Node::Shadowarg(_)
                | Node::Blockarg(_) => {
                    self.check_duplicate_arg(arg, map);
                }
                Node::Mlhs(Mlhs { items, .. }) => {
                    self.check_duplicate_args(items, map);
                }
                Node::Procarg0(Procarg0 { args, .. }) => {
                    self.check_duplicate_args(args, map);
                }
                Node::ForwardArg(_) | Node::Kwnilarg(_) => {}
                _ => unreachable!("unsupported arg type {:?}", arg),
            }
        }
    }

    fn arg_name<'a>(&self, node: &'a Node) -> Option<&'a String> {
        match node {
            Node::Arg(Arg { name, .. })
            | Node::Optarg(Optarg { name, .. })
            | Node::Kwarg(Kwarg { name, .. })
            | Node::Kwoptarg(Kwoptarg { name, .. })
            | Node::Shadowarg(Shadowarg { name, .. })
            | Node::Blockarg(Blockarg { name, .. }) => Some(name),
            Node::Restarg(Restarg { name, .. }) | Node::Kwrestarg(Kwrestarg { name, .. }) => {
                name.as_ref()
            }
            _ => unreachable!("unsupported arg {:?}", node),
        }
    }

    fn arg_name_loc<'a>(&self, node: &'a Node) -> &'a Loc {
        match node {
            Node::Arg(Arg { expression_l, .. }) => expression_l,
            Node::Optarg(Optarg { name_l, .. }) => name_l,
            Node::Kwarg(Kwarg { name_l, .. }) => name_l,
            Node::Kwoptarg(Kwoptarg { name_l, .. }) => name_l,
            Node::Shadowarg(Shadowarg { expression_l, .. }) => expression_l,
            Node::Blockarg(Blockarg { name_l, .. }) => name_l,
            Node::Restarg(Restarg {
                name_l,
                expression_l,
                ..
            })
            | Node::Kwrestarg(Kwrestarg {
                name_l,
                expression_l,
                ..
            }) => name_l.as_ref().unwrap_or(&expression_l),

            _ => unreachable!("unsupported arg {:?}", node),
        }
    }

    pub(crate) fn check_duplicate_arg<'a>(
        &self,
        this_arg: &'a Node,
        map: &mut HashMap<String, &'a Node>,
    ) {
        let this_name = match self.arg_name(this_arg) {
            Some(name) => name,
            None => return,
        };

        let that_arg = map.get(this_name);

        match that_arg {
            None => {
                map.insert(this_name.clone(), this_arg);
            }
            Some(that_arg) => {
                let that_name = match self.arg_name(*that_arg) {
                    Some(name) => name,
                    None => return,
                };
                if self.arg_name_collides(this_name, that_name) {
                    self.error(
                        DiagnosticMessage::DuplicatedArgumentName,
                        &self.arg_name_loc(this_arg),
                    )
                }
            }
        }
    }

    pub(crate) fn check_assignment_to_numparam(&self, name: &str, loc: &Loc) -> Result<(), ()> {
        let assigning_to_numparam = self.context.is_in_dynamic_block()
            && matches!(
                name,
                "_1" | "_2" | "_3" | "_4" | "_5" | "_6" | "_7" | "_8" | "_9"
            )
            && self.max_numparam_stack.has_numparams();

        if assigning_to_numparam {
            self.error(
                DiagnosticMessage::CantAssignToNumparam {
                    numparam: name.to_string(),
                },
                loc,
            );
            return Err(());
        }
        Ok(())
    }

    pub(crate) fn check_reserved_for_numparam(&self, name: &str, loc: &Loc) -> Result<(), ()> {
        match name {
            "_1" | "_2" | "_3" | "_4" | "_5" | "_6" | "_7" | "_8" | "_9" => {
                self.error(
                    DiagnosticMessage::ReservedForNumparam {
                        numparam: name.to_string(),
                    },
                    loc,
                );
                Err(())
            }
            _ => Ok(()),
        }
    }

    pub(crate) fn arg_name_collides(&self, this_name: &str, that_name: &str) -> bool {
        &this_name[0..1] != "_" && this_name == that_name
    }

    pub(crate) fn check_lvar_name(&self, name: &str, loc: &Loc) -> Result<(), ()> {
        let mut all_chars = name.chars();
        let first = all_chars
            .next()
            .expect("local variable name can't be empty");
        let rest = all_chars.collect::<Vec<_>>();

        if (first.is_lowercase() || first == '_')
            && rest.into_iter().all(|c| c.is_alphanumeric() || c == '_')
        {
            Ok(())
        } else {
            self.error(DiagnosticMessage::KeyMustBeValidAsLocalVariable, loc);
            Err(())
        }
    }

    pub(crate) fn check_duplicate_pattern_variable(&self, name: &str, loc: &Loc) -> Result<(), ()> {
        if name.starts_with('_') {
            return Ok(());
        }

        if self.pattern_variables.is_declared(name) {
            self.error(DiagnosticMessage::DuplicateVariableName, loc);
            return Err(());
        }

        self.pattern_variables.declare(name);
        Ok(())
    }

    pub(crate) fn check_duplicate_pattern_key(&self, name: &str, loc: &Loc) -> Result<(), ()> {
        if self.pattern_hash_keys.is_declared(name) {
            self.error(DiagnosticMessage::DuplicateKeyName, loc);
            return Err(());
        }

        self.pattern_hash_keys.declare(name);
        Ok(())
    }

    //
    // Helpers
    //

    pub(crate) fn static_string(&self, nodes: &[Node]) -> Option<String> {
        let mut result = String::from("");

        for node in nodes {
            match node {
                Node::Str(Str { value, .. }) => {
                    let value = value.bytes.to_string_lossy();
                    result.push_str(&value)
                }
                Node::Begin(Begin { statements, .. }) => {
                    if let Some(s) = self.static_string(&statements) {
                        result.push_str(&s)
                    } else {
                        return None;
                    }
                }
                _ => return None,
            }
        }

        Some(result)
    }

    #[cfg(feature = "onig")]
    pub(crate) fn build_static_regexp(
        &self,
        parts: &[Node],
        options: &[char],
        loc: &Loc,
    ) -> Option<Regex> {
        let source = self.static_string(&parts)?;
        let mut reg_options = RegexOptions::REGEX_OPTION_NONE;
        reg_options |= RegexOptions::REGEX_OPTION_CAPTURE_GROUP;
        if options.contains(&'x') {
            reg_options |= RegexOptions::REGEX_OPTION_EXTEND;
        }

        let bytes = onig::EncodedBytes::ascii(source.as_bytes());

        match Regex::with_options_and_encoding(bytes, reg_options, onig::Syntax::ruby()) {
            Ok(regex) => Some(regex),
            Err(err) => {
                self.error(
                    DiagnosticMessage::RegexError {
                        error: err.description().to_string(),
                    },
                    loc,
                );
                None
            }
        }
    }

    #[cfg(feature = "onig")]
    pub(crate) fn validate_static_regexp(&self, parts: &[Node], options: &[char], loc: &Loc) {
        self.build_static_regexp(parts, options, loc);
    }

    #[cfg(not(feature = "onig"))]
    pub(crate) fn validate_static_regexp(&self, _parts: &[Node], _options: &[char], _loc: &Loc) {}

    #[cfg(feature = "onig")]
    pub(crate) fn static_regexp_captures(&self, node: &Node) -> Option<Vec<String>> {
        if let Node::Regexp(Regexp {
            parts,
            options,
            expression_l,
            ..
        }) = node
        {
            let mut re_options: &[char] = &[];
            if let Some(options) = options {
                if let Node::RegOpt(RegOpt { options, .. }) = &**options {
                    re_options = options;
                }
            };
            let regex = self.build_static_regexp(parts, re_options, expression_l)?;

            let mut result: Vec<String> = vec![];

            regex.foreach_name(|name, _| {
                result.push(name.to_string());
                true
            });

            return Some(result);
        }
        None
    }

    #[cfg(not(feature = "onig"))]
    pub(crate) fn static_regexp_captures(&self, _node: &Node) -> Option<Vec<String>> {
        None
    }

    pub(crate) fn loc(&self, token: &Token) -> Ptr<Loc> {
        token.loc.clone()
    }

    pub(crate) fn maybe_loc(&self, token: &Option<Box<Token>>) -> MaybePtr<Loc> {
        match token {
            Some(token) => self.loc(token).to_maybe_ptr(),
            None => MaybePtr::none(),
        }
    }

    pub(crate) fn collection_map(
        &self,
        begin_t: &Option<Box<Token>>,
        parts: &[Node],
        end_t: &Option<Box<Token>>,
    ) -> CollectionMap {
        let begin_l = self.maybe_loc(begin_t);
        let end_l = self.maybe_loc(end_t);

        let expression_l = collection_expr(&parts);
        let expression_l = join_maybe_locs(&expression_l, &begin_l);
        let expression_l = join_maybe_locs(&expression_l, &end_l);
        let expression_l = expression_l.unwrap_or_else(|| {
            unreachable!("empty collection without begin_t/end_t, can't build source map")
        });

        CollectionMap {
            begin_l,
            end_l,
            expression_l,
        }
    }

    pub(crate) fn is_heredoc(&self, begin_t: &Option<Box<Token>>) -> bool {
        if let Some(begin_t) = begin_t {
            if clone_value(&begin_t).starts_with("<<") {
                return true;
            }
        }
        false
    }

    pub(crate) fn heredoc_map(
        &self,
        begin_t: &Option<Box<Token>>,
        parts: &[Node],
        end_t: &Option<Box<Token>>,
    ) -> HeredocMap {
        let begin_t = begin_t
            .as_ref()
            .unwrap_or_else(|| unreachable!("bug: begin_t must be Some"));
        let end_t = end_t
            .as_ref()
            .unwrap_or_else(|| unreachable!("heredoc must have end_t"));

        let heredoc_body_l = collection_expr(&parts).unwrap_or_else(|| self.loc(end_t));
        let expression_l = self.loc(begin_t);
        let heredoc_end_l = self.loc(end_t);

        HeredocMap {
            heredoc_body_l,
            heredoc_end_l,
            expression_l,
        }
    }

    pub(crate) fn error(&self, message: DiagnosticMessage, loc: &Loc) {
        self.diagnostics.emit(Diagnostic::new(
            ErrorLevel::Error,
            message,
            Box::new(loc.clone()),
        ))
    }

    pub(crate) fn warn(&self, message: DiagnosticMessage, loc: &Loc) {
        self.diagnostics.emit(Diagnostic::new(
            ErrorLevel::Warning,
            message,
            Ptr::new(loc.clone()),
        ))
    }

    pub(crate) fn value_expr(&self, node: &Node) -> Result<(), ()> {
        if let Some(void_node) = self.void_value(node) {
            self.error(
                DiagnosticMessage::VoidValueExpression,
                void_node.expression(),
            );
            Err(())
        } else {
            Ok(())
        }
    }

    fn void_value<'a>(&self, node: &'a Node) -> Option<&'a Node> {
        let check_stmts = |statements: &'a Vec<Node>| {
            if let Some(last_stmt) = statements.last() {
                self.void_value(last_stmt)
            } else {
                None
            }
        };

        let check_condition = |if_true: &'a Node, if_false: &'a Node| {
            if self.void_value(if_true).is_some() && self.void_value(if_false).is_some() {
                Some(if_true)
            } else {
                None
            }
        };

        let check_maybe_condition =
            |if_true: &'a Option<Box<Node>>, if_false: &'a Option<Box<Node>>| match (
                if_true, if_false,
            ) {
                (None, None) | (None, Some(_)) | (Some(_), None) => None,
                (Some(if_true), Some(if_false)) => check_condition(&*if_true, &*if_false),
            };

        match node {
            Node::Return(_) | Node::Break(_) | Node::Next(_) | Node::Redo(_) | Node::Retry(_) => {
                Some(node)
            }

            Node::MatchPattern(MatchPattern { value, .. }) => self.void_value(value),
            Node::MatchPatternP(MatchPatternP { value, .. }) => self.void_value(value),

            Node::Begin(Begin { statements, .. }) | Node::KwBegin(KwBegin { statements, .. }) => {
                check_stmts(statements)
            }

            Node::If(If {
                if_true, if_false, ..
            })
            | Node::IfMod(IfMod {
                if_true, if_false, ..
            }) => check_maybe_condition(if_true, if_false),

            Node::IfTernary(IfTernary {
                if_true, if_false, ..
            }) => check_condition(if_true, if_false),

            Node::And(And { lhs, .. }) | Node::Or(Or { lhs, .. }) => self.void_value(lhs),

            _ => None,
        }
    }

    fn rewrite_hash_args_to_kwargs(&self, args: &mut Vec<Node>) {
        match &mut args[..] {
            [.., last, Node::BlockPass(_)] | [.., last] => {
                match &mut *last {
                    Node::Hash(hash) if hash.begin_l.is_none() && hash.end_l.is_none() => {
                        *last = Node::Kwargs(Kwargs {
                            pairs: std::mem::take(&mut hash.pairs),
                            expression_l: Box::new(hash.expression().clone()),
                        });
                    }
                    _ => {}
                };
            }

            _ => {}
        };
    }
}

pub(crate) fn maybe_node_expr(node: &Option<&Node>) -> MaybePtr<Loc> {
    match node {
        Some(node) => MaybePtr::some(node.expression().clone()),
        None => MaybePtr::none(),
    }
}

pub(crate) fn maybe_boxed_node_expr(node: &Option<Box<Node>>) -> MaybePtr<Loc> {
    match node {
        Some(node) => MaybePtr::some(node.expression().clone()),
        None => MaybePtr::none(),
    }
}

pub(crate) fn collection_expr(nodes: &[Node]) -> MaybePtr<Loc> {
    join_maybe_exprs(&nodes.first(), &nodes.last())
}

pub(crate) fn value(token: Box<Token>) -> String {
    token.into_string().unwrap()
}

pub(crate) fn lossy_value(token: Box<Token>) -> String {
    token.to_string_lossy()
}

pub(crate) fn clone_value(token: &Token) -> String {
    token.to_string_lossy()
}

pub(crate) fn maybe_value(token: Option<Box<Token>>) -> Option<String> {
    token.map(value)
}

pub(crate) fn join_exprs(lhs: &Node, rhs: &Node) -> Ptr<Loc> {
    lhs.expression().join(rhs.expression())
}

pub(crate) fn join_maybe_exprs(lhs: &Option<&Node>, rhs: &Option<&Node>) -> MaybePtr<Loc> {
    join_maybe_locs(&maybe_node_expr(&lhs), &maybe_node_expr(&rhs))
}

pub(crate) fn join_maybe_locs(lhs: &MaybePtr<Loc>, rhs: &MaybePtr<Loc>) -> MaybePtr<Loc> {
    match (lhs, rhs) {
        (None, None) => None,
        (None, Some(rhs)) => Some(rhs.clone()),
        (Some(lhs), None) => Some(lhs.clone()),
        (Some(lhs), Some(rhs)) => Some(lhs.join(&rhs)),
    }
}

pub(crate) struct CollectionMap {
    begin_l: MaybePtr<Loc>,
    end_l: MaybePtr<Loc>,
    expression_l: Ptr<Loc>,
}

pub(crate) struct HeredocMap {
    heredoc_body_l: Ptr<Loc>,
    heredoc_end_l: Ptr<Loc>,
    expression_l: Ptr<Loc>,
}

fn first<T>(vec: Vec<T>) -> T {
    vec.into_iter().next().expect("expected vec to have 1 item")
}
