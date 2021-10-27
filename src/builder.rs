#[cfg(feature = "onig")]
use onig::{Regex, RegexOptions};

use std::collections::HashMap;
use std::convert::TryInto;

use crate::error::Diagnostics;
#[allow(unused_imports)]
use crate::nodes::*;
use crate::LexState;
use crate::Loc;
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
    QuotedLabel((Box<Token>, Box<Vec<Node>>, Box<Token>)),
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
            operator_l: None,
            expression_l,
        }))
    }

    pub(crate) fn float(&self, float_t: Box<Token>) -> Box<Node> {
        let expression_l = self.loc(&float_t);
        Box::new(Node::Float(Float {
            value: value(float_t),
            operator_l: None,
            expression_l,
        }))
    }

    pub(crate) fn rational(&self, rational_t: Box<Token>) -> Box<Node> {
        let expression_l = self.loc(&rational_t);
        Box::new(Node::Rational(Rational {
            value: value(rational_t),
            operator_l: None,
            expression_l,
        }))
    }

    pub(crate) fn complex(&self, complex_t: Box<Token>) -> Box<Node> {
        let expression_l = self.loc(&complex_t);
        Box::new(Node::Complex(Complex {
            value: value(complex_t),
            operator_l: None,
            expression_l,
        }))
    }

    pub(crate) fn unary_num(&self, unary_t: Box<Token>, mut numeric: Box<Node>) -> Box<Node> {
        let new_operator_l = self.loc(&unary_t);
        let sign = String::from(value(unary_t));

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
                operator_l,
                expression_l,
            })
            | Node::Complex(Complex {
                value,
                operator_l,
                expression_l,
            }) => {
                *value = format!("{}{}", sign, value);
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
        value: Bytes,
        parts: Box<Vec<Node>>,
        end_t: Option<Box<Token>>,
    ) -> Box<Node> {
        if self.is_heredoc(&begin_t) {
            let HeredocMap {
                heredoc_body_l,
                heredoc_end_l,
                expression_l,
            } = self.heredoc_map(&begin_t, &parts, &end_t);

            Box::new(Node::Heredoc(Heredoc {
                parts: *parts,
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
        let value = string_t.token_value;
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
        parts: Box<Vec<Node>>,
        end_t: Option<Box<Token>>,
    ) -> Box<Node> {
        match &**parts {
            [] => {
                return self.str_node(begin_t, Bytes::empty(), parts, end_t);
            }
            [Node::Str(_) | Node::Dstr(_) | Node::Heredoc(_)]
                if begin_t.is_none() && end_t.is_none() =>
            {
                return Box::new(
                    parts
                        .into_iter()
                        .next()
                        .expect("expected at least 1 element"),
                );
            }

            [Node::Str(Str { value, .. })] => {
                return self.str_node(begin_t, value.clone(), parts, end_t);
            }

            [Node::Dstr(_) | Node::Heredoc(_)] => {
                unreachable!()
            }
            _ => {}
        }

        if self.is_heredoc(&begin_t) {
            let HeredocMap {
                heredoc_body_l,
                heredoc_end_l,
                expression_l,
            } = self.heredoc_map(&begin_t, &parts, &end_t);

            Box::new(Node::Heredoc(Heredoc {
                parts: *parts,
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
                parts: *parts,
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

        let value = char_t.token_value;
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

    fn validate_sym_value(&self, value: &Bytes, loc: &Loc) {
        if !value.is_valid_utf8() {
            self.error(
                DiagnosticMessage::InvalidSymbol {
                    symbol: String::from("UTF-8"),
                },
                loc,
            )
        }
    }

    pub(crate) fn symbol(&self, start_t: Box<Token>, value_t: Box<Token>) -> Box<Node> {
        let expression_l = self.loc(&start_t).join(&self.loc(&value_t));
        let begin_l = Some(self.loc(&start_t));
        let value = value_t.token_value;
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
        let value = symbol_t.token_value;
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
        parts: Box<Vec<Node>>,
        end_t: Box<Token>,
    ) -> Box<Node> {
        if parts.len() == 1 && matches!(&parts[0], Node::Str(_)) {
            match parts.into_iter().next().unwrap() {
                Node::Str(Str { value, .. }) => {
                    let CollectionMap {
                        begin_l,
                        end_l,
                        expression_l,
                    } = self.collection_map(&Some(begin_t), &[], &Some(end_t));

                    self.validate_sym_value(&value, &expression_l);

                    return Box::new(Node::Sym(Sym {
                        name: value,
                        begin_l,
                        end_l,
                        expression_l,
                    }));
                }
                _ => unreachable!(),
            }
        }

        let CollectionMap {
            begin_l,
            end_l,
            expression_l,
        } = self.collection_map(&Some(begin_t), &parts, &Some(end_t));
        Box::new(Node::Dsym(Dsym {
            parts: *parts,
            begin_l,
            end_l,
            expression_l,
        }))
    }

    // Executable strings

    pub(crate) fn xstring_compose(
        &self,
        begin_t: Box<Token>,
        parts: Box<Vec<Node>>,
        end_t: Box<Token>,
    ) -> Box<Node> {
        let begin_l = self.loc(&begin_t);
        if lossy_value(begin_t).as_str().starts_with("<<") {
            let heredoc_body_l = collection_expr(&parts).unwrap_or_else(|| self.loc(&end_t));
            let heredoc_end_l = self.loc(&end_t);
            let expression_l = begin_l;

            Box::new(Node::XHeredoc(XHeredoc {
                parts: *parts,
                heredoc_body_l,
                heredoc_end_l,
                expression_l,
            }))
        } else {
            let end_l = self.loc(&end_t);
            let expression_l = begin_l.join(&end_l);

            Box::new(Node::Xstr(Xstr {
                parts: *parts,
                begin_l,
                end_l,
                expression_l,
            }))
        }
    }

    // Indented (interpolated, noninterpolated, executable) strings

    pub(crate) fn heredoc_dedent(&self, node: Node, dedent_level: i32) -> Node {
        if dedent_level == 0 {
            return node;
        }

        let dedent_level: usize = dedent_level
            .try_into()
            .expect("dedent_level must be positive");

        let dedent_heredoc_parts = |parts: Vec<Node>| -> Vec<Node> {
            let parts = parts
                .into_iter()
                .filter_map(|part| match part {
                    Node::Str(Str {
                        value,
                        begin_l,
                        end_l,
                        expression_l,
                    }) => {
                        let value = Self::dedent_string(value, dedent_level);
                        if value.is_empty() {
                            None
                        } else {
                            Some(Node::Str(Str {
                                value,
                                begin_l,
                                end_l,
                                expression_l,
                            }))
                        }
                    }
                    Node::Begin(_)
                    | Node::Gvar(_)
                    | Node::BackRef(_)
                    | Node::NthRef(_)
                    | Node::Ivar(_)
                    | Node::Cvar(_) => Some(part),
                    other => {
                        unreachable!("unsupported heredoc child {}", other.str_type())
                    }
                })
                .collect::<Vec<_>>();
            Vec::from(parts)
        };

        match node {
            Node::Heredoc(Heredoc {
                parts,
                heredoc_body_l,
                heredoc_end_l,
                expression_l,
            }) => {
                let parts = dedent_heredoc_parts(parts);
                Node::Heredoc(Heredoc {
                    parts,
                    heredoc_body_l,
                    heredoc_end_l,
                    expression_l,
                })
            }
            Node::XHeredoc(XHeredoc {
                parts,
                heredoc_body_l,
                heredoc_end_l,
                expression_l,
            }) => {
                let parts = dedent_heredoc_parts(parts);
                Node::XHeredoc(XHeredoc {
                    parts,
                    heredoc_body_l,
                    heredoc_end_l,
                    expression_l,
                })
            }
            _ => {
                unreachable!("unsupported heredoc_dedent argument {}", node.str_type())
            }
        }
    }

    const TAB_WIDTH: usize = 8;

    pub(crate) fn dedent_string(s: Bytes, width: usize) -> Bytes {
        let mut col: usize = 0;
        let mut i: usize = 0;

        loop {
            if !(i < s.len() && col < width) {
                break;
            }

            if s[i] == b' ' {
                col += 1;
            } else if s[i] == b'\t' {
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

        Bytes::new(Vec::from(&s.as_raw()[i..]))
    }

    // Regular expressions

    pub(crate) fn regexp_options(&self, regexp_end_t: Box<Token>) -> Option<Box<Node>> {
        if regexp_end_t.loc.end - regexp_end_t.loc.begin == 1 {
            // no regexp options, only trailing "/"
            return None;
        }
        let expression_l = self.loc(&regexp_end_t).adjust_begin(1);
        let options = value(regexp_end_t);
        let mut options = options.as_str().chars().skip(1).collect::<Vec<_>>();
        options.sort_unstable();
        options.dedup();
        let options = if options.is_empty() {
            None
        } else {
            Some(String::from(options.into_iter().collect::<String>()))
        };

        Some(Box::new(Node::RegOpt(RegOpt {
            options,
            expression_l,
        })))
    }

    pub(crate) fn regexp_compose(
        &self,
        begin_t: Box<Token>,
        parts: Box<Vec<Node>>,
        end_t: Box<Token>,
        options: Option<Box<Node>>,
    ) -> Box<Node> {
        let begin_l = self.loc(&begin_t);
        let end_l = self.loc(&end_t).resize(1);
        let expression_l =
            begin_l.join(&maybe_boxed_node_expr(&options).unwrap_or_else(|| self.loc(&end_t)));

        match options.as_ref().map(|boxed| &**boxed) {
            Some(Node::RegOpt(RegOpt {
                ref options,
                ref expression_l,
            })) => self.validate_static_regexp(&parts, options, &expression_l),
            None => self.validate_static_regexp(&parts, &None, &expression_l),
            _ => unreachable!("must be Option<RegOpt>"),
        }

        Box::new(Node::Regexp(Regexp {
            parts: *parts,
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
        elements: Box<Vec<Node>>,
        end_t: Option<Box<Token>>,
    ) -> Box<Node> {
        let CollectionMap {
            begin_l,
            end_l,
            expression_l,
        } = self.collection_map(&begin_t, &elements, &end_t);

        Box::new(Node::Array(Array {
            elements: *elements,
            begin_l,
            end_l,
            expression_l,
        }))
    }

    pub(crate) fn splat(&self, star_t: Box<Token>, value: Option<Box<Node>>) -> Box<Node> {
        let operator_l = self.loc(&star_t);
        let expression_l = operator_l.maybe_join(&maybe_boxed_node_expr(&value));

        Box::new(Node::Splat(Splat {
            value,
            operator_l,
            expression_l,
        }))
    }

    pub(crate) fn word(&self, parts: Box<Vec<Node>>) -> Box<Node> {
        if parts.len() == 1 && matches!(&parts[0], Node::Str(_) | Node::Dstr(_)) {
            let part = parts
                .into_iter()
                .next()
                .expect("parts is supposed to have exactly 1 element");
            return Box::new(part);
        }

        let CollectionMap {
            begin_l,
            end_l,
            expression_l,
        } = self.collection_map(&None, &parts, &None);

        Box::new(Node::Dstr(Dstr {
            parts: *parts,
            begin_l,
            end_l,
            expression_l,
        }))
    }

    pub(crate) fn words_compose(
        &self,
        begin_t: Box<Token>,
        elements: Box<Vec<Node>>,
        end_t: Box<Token>,
    ) -> Box<Node> {
        let begin_l = self.loc(&begin_t);
        let end_l = self.loc(&end_t);
        let expression_l = begin_l.join(&end_l);
        Box::new(Node::Array(Array {
            elements: *elements,
            begin_l: Some(begin_l),
            end_l: Some(end_l),
            expression_l,
        }))
    }

    pub(crate) fn symbols_compose(
        &self,
        begin_t: Box<Token>,
        parts: Box<Vec<Node>>,
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
                other => other,
            })
            .collect::<Vec<_>>();

        let begin_l = self.loc(&begin_t);
        let end_l = self.loc(&end_t);
        let expression_l = begin_l.join(&end_l);
        Box::new(Node::Array(Array {
            elements: Vec::from(parts),
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
        let expression_l = key_loc.join(value.expression());

        let key = key_t.token_value;
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
        parts: Box<Vec<Node>>,
        end_t: Box<Token>,
        value: Box<Node>,
    ) -> Box<Node> {
        let end_l = self.loc(&end_t);

        let quote_loc = Loc {
            begin: end_l.end - 2,
            end: end_l.end - 1,
        };

        let colon_l = end_l.with_begin(end_l.end - 1);

        let end_t = end_t;
        let end_t: Box<Token> = Box::new(Token {
            token_type: end_t.token_type,
            token_value: end_t.token_value,
            loc: quote_loc,
            lex_state_before: LexState::default(),
            lex_state_after: LexState::default(),
        });
        let expression_l = self.loc(&begin_t).join(value.expression());

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
        pairs: Box<Vec<Node>>,
        end_t: Option<Box<Token>>,
    ) -> Box<Node> {
        let CollectionMap {
            begin_l,
            end_l,
            expression_l,
        } = self.collection_map(&begin_t, &pairs, &end_t);

        Box::new(Node::Hash(Hash {
            pairs: *pairs,
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
        let name = value(token);
        let name = &name.as_str()[1..];
        let parsed = name.parse::<usize>();
        let name = String::from(name);

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
        if matches!(&*node, Node::Lvar(_)) {
            match *node {
                Node::Lvar(Lvar { name, expression_l }) => {
                    let name_s = name.as_str();
                    if self.static_env.is_declared(name_s) {
                        if let Some(current_arg) = self.current_arg_stack.top() {
                            if current_arg == name_s {
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
                _ => unreachable!(),
            }
        } else {
            node
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
        let scope = Box::new(Node::Cbase(Cbase {
            expression_l: self.loc(&t_colon3),
        }));

        let name_l = self.loc(&name_t);
        let expression_l = scope.expression().join(&name_l);
        let double_colon_l = self.loc(&t_colon3);

        Box::new(Node::Const(Const {
            scope: Some(scope),
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
        let scope: Box<Node> = scope;
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
                name_l: expression_l,
                operator_l: None,
                expression_l,
            }),
            Node::Ivar(Ivar { name, expression_l }) => Node::Ivasgn(Ivasgn {
                name,
                value: None,
                name_l: expression_l,
                operator_l: None,
                expression_l,
            }),
            Node::Gvar(Gvar { name, expression_l }) => Node::Gvasgn(Gvasgn {
                name,
                value: None,
                name_l: expression_l,
                operator_l: None,
                expression_l,
            }),
            Node::Const(Const {
                scope,
                name,
                double_colon_l,
                name_l,
                expression_l,
            }) => {
                if !self.context.is_dynamic_const_definition_allowed() {
                    self.error(
                        DiagnosticMessage::DynamicConstantAssignment {},
                        &expression_l,
                    );
                    return Err(());
                }
                Node::Casgn(Casgn {
                    scope,
                    name,
                    value: None,
                    double_colon_l,
                    name_l,
                    operator_l: None,
                    expression_l,
                })
            }
            Node::Lvar(Lvar { name, expression_l }) => {
                let name_s = name.as_str();
                self.check_assignment_to_numparam(name_s, &expression_l)?;
                self.check_reserved_for_numparam(name_s, &expression_l)?;

                self.static_env.declare(name_s);

                Node::Lvasgn(Lvasgn {
                    name,
                    value: None,
                    name_l: expression_l,
                    operator_l: None,
                    expression_l,
                })
            }
            Node::Self_(Self_ { expression_l }) => {
                self.error(DiagnosticMessage::CantAssignToSelf {}, &expression_l);
                return Err(());
            }
            Node::Nil(Nil { expression_l }) => {
                self.error(DiagnosticMessage::CantAssignToNil {}, &expression_l);
                return Err(());
            }
            Node::True(True { expression_l }) => {
                self.error(DiagnosticMessage::CantAssignToTrue {}, &expression_l);
                return Err(());
            }
            Node::False(False { expression_l }) => {
                self.error(DiagnosticMessage::CantAssignToFalse {}, &expression_l);
                return Err(());
            }
            Node::File(File { expression_l }) => {
                self.error(DiagnosticMessage::CantAssignToFile {}, &expression_l);
                return Err(());
            }
            Node::Line(Line { expression_l }) => {
                self.error(DiagnosticMessage::CantAssignToLine {}, &expression_l);
                return Err(());
            }
            Node::Encoding(Encoding { expression_l }) => {
                self.error(DiagnosticMessage::CantAssignToEncoding {}, &expression_l);
                return Err(());
            }
            Node::BackRef(BackRef { name, expression_l }) => {
                self.error(
                    DiagnosticMessage::CantSetVariable { var_name: name },
                    &expression_l,
                );
                return Err(());
            }
            Node::NthRef(NthRef { name, expression_l }) => {
                self.error(
                    DiagnosticMessage::CantSetVariable {
                        var_name: String::from(format!("${}", name)),
                    },
                    &expression_l,
                );
                return Err(());
            }
            other => unreachable!("{:?} can't be used in assignment", other),
        };

        Ok(Box::new(node))
    }

    pub(crate) fn const_op_assignable(&self, node: Box<Node>) -> Box<Node> {
        match *node {
            Node::Const(Const {
                scope,
                name,
                double_colon_l,
                name_l,
                expression_l,
            }) => Box::new(Node::Casgn(Casgn {
                scope,
                name,
                value: None,
                double_colon_l,
                name_l,
                operator_l: None,
                expression_l,
            })),
            other => {
                unreachable!("unsupported const_op_assignable arument: {:?}", other)
            }
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
                expression_l,
                operator_l,
                value,
                ..
            })
            | Node::Ivasgn(Ivasgn {
                expression_l,
                operator_l,
                value,
                ..
            })
            | Node::Gvasgn(Gvasgn {
                expression_l,
                operator_l,
                value,
                ..
            })
            | Node::Lvasgn(Lvasgn {
                expression_l,
                operator_l,
                value,
                ..
            })
            | Node::Casgn(Casgn {
                expression_l,
                operator_l,
                value,
                ..
            })
            | Node::IndexAsgn(IndexAsgn {
                expression_l,
                operator_l,
                value,
                ..
            }) => {
                *expression_l = expr_l;
                *operator_l = op_l;
                *value = Some(new_rhs);
            }
            Node::Send(Send {
                expression_l,
                operator_l,
                args,
                ..
            })
            | Node::CSend(CSend {
                expression_l,
                operator_l,
                args,
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
            other => unreachable!("{:?} can't be used in assignment", other),
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
        let mut operator = String::from(value(op_t));
        operator.pop();
        let expression_l = join_exprs(&lhs, &rhs);

        match &*lhs {
            Node::Gvasgn(_)
            | Node::Ivasgn(_)
            | Node::Lvasgn(_)
            | Node::Cvasgn(_)
            | Node::Casgn(_)
            | Node::Send(_)
            | Node::CSend(_) => {
                // ignore
            }
            Node::Index(_) => match *lhs {
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
                        operator_l: None,
                        expression_l,
                    }));
                }
                _ => unreachable!(),
            },
            Node::BackRef(BackRef { name, expression_l }) => {
                self.error(
                    DiagnosticMessage::CantSetVariable {
                        var_name: name.clone(),
                    },
                    expression_l,
                );
                return Err(());
            }
            Node::NthRef(NthRef { name, expression_l }) => {
                self.error(
                    DiagnosticMessage::CantSetVariable {
                        var_name: String::from(format!("${}", name)),
                    },
                    expression_l,
                );
                return Err(());
            }
            _ => unreachable!("unsupported op_assign lhs {:?}", lhs),
        }

        let recv: Box<Node> = lhs;
        let value: Box<Node> = rhs;

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
                operator: String::from(operator),
                value,
                operator_l,
                expression_l,
            }),
        };

        Ok(Box::new(result))
    }

    pub(crate) fn multi_lhs(
        &self,
        begin_t: Option<Box<Token>>,
        items: Box<Vec<Node>>,
        end_t: Option<Box<Token>>,
    ) -> Box<Node> {
        let CollectionMap {
            begin_l,
            end_l,
            expression_l,
        } = self.collection_map(&begin_t, &items, &end_t);

        Box::new(Node::Mlhs(Mlhs {
            items: *items,
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
        self.check_reserved_for_numparam(name.as_str(), &name_l)?;

        Ok(Box::new(Node::Def(Def {
            name,
            args,
            body,
            keyword_l,
            name_l,
            end_l: Some(end_l),
            assignment_l: None,
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
        self.check_reserved_for_numparam(name.as_str(), &name_l)?;

        Ok(Box::new(Node::Def(Def {
            name,
            args,
            body,
            keyword_l,
            name_l,
            end_l: None,
            assignment_l: Some(assignment_l),
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
        self.check_reserved_for_numparam(name.as_str(), &name_l)?;

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
        self.check_reserved_for_numparam(name.as_str(), &name_l)?;

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

    pub(crate) fn undef_method(&self, undef_t: Box<Token>, names: Box<Vec<Node>>) -> Box<Node> {
        let keyword_l = self.loc(&undef_t);
        let expression_l = keyword_l.maybe_join(&collection_expr(&names));
        Box::new(Node::Undef(Undef {
            names: *names,
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
        args: Box<Vec<Node>>,
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
            args: *args,
            expression_l,
            begin_l,
            end_l,
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

        self.check_reserved_for_numparam(name.as_str(), &name_l)?;

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
        self.check_reserved_for_numparam(name.as_str(), &name_l)?;

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
        let (name, name_l) = if name_t.is_some() {
            let name_t = name_t.unwrap();
            let name_l = self.loc(&name_t);
            let name = value(name_t);
            self.check_reserved_for_numparam(name.as_str(), &name_l)?;
            (Some(name), Some(name_l))
        } else {
            (None, None)
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
        self.check_reserved_for_numparam(name.as_str(), &name_l)?;

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
        self.check_reserved_for_numparam(name.as_str(), &name_l)?;

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
        let (name, name_l) = if name_t.is_some() {
            let name_t = name_t.unwrap();
            let name_l = self.loc(&name_t);
            let name = value(name_t);
            self.check_reserved_for_numparam(name.as_str(), &name_l)?;
            (Some(name), Some(name_l))
        } else {
            (None, None)
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
        self.check_reserved_for_numparam(name.as_str(), &name_l)?;

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
        self.check_reserved_for_numparam(name.as_str(), &name_l)?;

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
            Node::Arg(Arg { name, expression_l }) => Box::new(Node::Procarg0(Procarg0 {
                args: vec![Node::Arg(Arg {
                    name,
                    expression_l: expression_l.clone(),
                })],
                begin_l: None,
                end_l: None,
                expression_l,
            })),
            other => {
                unreachable!("unsupported procarg0 child {:?}", other)
            }
        }
    }

    //
    // Method calls
    //

    fn call_type_for_dot(&self, dot_t: &Option<Box<Token>>) -> MethodCallType {
        match dot_t.as_ref() {
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
        mut args: Box<Vec<Node>>,
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

        let method_name = maybe_value(selector_t);
        let method_name = if method_name.is_some() {
            method_name.unwrap()
        } else {
            String::from("call")
        };

        self.rewrite_hash_args_to_kwargs(&mut args);

        match self.call_type_for_dot(&dot_t) {
            MethodCallType::Send => Box::new(Node::Send(Send {
                recv: receiver,
                method_name,
                args: *args,
                dot_l,
                selector_l,
                begin_l,
                end_l,
                operator_l: None,
                expression_l,
            })),

            MethodCallType::CSend => Box::new(Node::CSend(CSend {
                recv: receiver.expect("csend node must have a receiver"),
                method_name,
                args: *args,
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
                    Node::BlockPass(_) | Node::ForwardedArgs(_) => {
                        self.error(
                            DiagnosticMessage::BlockAndBlockArgGiven {},
                            last_arg.expression(),
                        );
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
                self.error(DiagnosticMessage::BlockGivenToYield {}, keyword_l);
                return Err(());
            }
            Node::Send(Send { args, .. }) => {
                validate_block_and_block_arg(args)?;
            }
            Node::CSend(CSend { args, .. }) => {
                validate_block_and_block_arg(args)?;
            }
            _ => {}
        }

        let rewrite_args_and_loc =
            |method_args: Vec<Node>,
             keyword_expression_l: Loc,
             block_args: ArgsType,
             block_body: Option<Box<Node>>| {
                // Code like "return foo 1 do end" is reduced in a weird sequence.
                // Here, method_call is actually (return).
                let actual_send = method_args.into_iter().next().unwrap();

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

                (vec![block], expr_l)
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
                        call: method_call,
                        numargs,
                        body: {
                            let block_body: Option<Box<Node>> = block_body;
                            block_body.expect("numblock always has body")
                        },
                        begin_l,
                        end_l,
                        expression_l,
                    }),
                };
                return Ok(Box::new(result));
            }
            _ => {}
        }

        let method_call = method_call;
        let result = match *method_call {
            Node::Return(Return {
                args,
                keyword_l,
                expression_l,
            }) => {
                let (args, expression_l) =
                    rewrite_args_and_loc(args, expression_l, block_args, block_body);
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
                    rewrite_args_and_loc(args, expression_l, block_args, block_body);
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
                    rewrite_args_and_loc(args, expression_l, block_args, block_body);
                Node::Break(Break {
                    args,
                    keyword_l,
                    expression_l,
                })
            }
            other => {
                unreachable!("unsupported method call {:?}", other)
            }
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
        let receiver: Box<Node> = receiver;

        let method_name = String::from(String::from(value(selector_t)) + "=");

        match self.call_type_for_dot(&Some(dot_t)) {
            MethodCallType::Send => Box::new(Node::Send(Send {
                recv: Some(receiver),
                method_name,
                args: vec![],
                dot_l: Some(dot_l),
                selector_l: Some(selector_l),
                begin_l: None,
                end_l: None,
                operator_l: None,
                expression_l,
            })),

            MethodCallType::CSend => Box::new(Node::CSend(CSend {
                recv: receiver,
                method_name,
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
        mut indexes: Box<Vec<Node>>,
        rbrack_t: Box<Token>,
    ) -> Box<Node> {
        let begin_l = self.loc(&lbrack_t);
        let end_l = self.loc(&rbrack_t);
        let expression_l = recv.expression().join(&end_l);

        self.rewrite_hash_args_to_kwargs(&mut indexes);

        Box::new(Node::Index(Index {
            recv,
            indexes: *indexes,
            begin_l,
            end_l,
            expression_l,
        }))
    }

    pub(crate) fn index_asgn(
        &self,
        recv: Box<Node>,
        lbrack_t: Box<Token>,
        indexes: Box<Vec<Node>>,
        rbrack_t: Box<Token>,
    ) -> Box<Node> {
        let begin_l = self.loc(&lbrack_t);
        let end_l = self.loc(&rbrack_t);
        let expression_l = recv.expression().join(&end_l);

        Box::new(Node::IndexAsgn(IndexAsgn {
            recv,
            indexes: *indexes,
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

        let selector_l = Some(self.loc(&operator_t));
        let expression_l = join_exprs(&receiver, &arg);

        Ok(Box::new(Node::Send(Send {
            recv: Some(receiver),
            method_name: value(operator_t),
            args: vec![*arg],
            dot_l: None,
            selector_l,
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

        let op = String::from(value(op_t));
        let method_name = if op == "+" || op == "-" { op + "@" } else { op };
        Ok(Box::new(Node::Send(Send {
            recv: Some(receiver),
            method_name: String::from(method_name),
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
        if receiver.is_some() {
            let receiver = receiver.unwrap();
            self.value_expr(&receiver)?;

            let begin_l = self.loc(&not_t);
            let end_l = self
                .maybe_loc(&end_t)
                .unwrap_or_else(|| receiver.expression().clone());

            let expression_l = begin_l.join(&end_l);

            let selector_l = self.loc(&not_t);
            let begin_l = self.maybe_loc(&begin_t);
            let end_l = self.maybe_loc(&end_t);

            Ok(Box::new(Node::Send(Send {
                recv: Some(self.check_condition(receiver)),
                method_name: String::from("!"),
                args: vec![],
                dot_l: None,
                selector_l: Some(selector_l),
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

            let nil_node = Box::new(Node::Begin(Begin {
                statements: vec![],
                begin_l,
                end_l,
                expression_l,
            }));

            let selector_l = self.loc(&not_t);
            let expression_l = nil_node.expression().join(&selector_l);
            Ok(Box::new(Node::Send(Send {
                recv: Some(nil_node),
                method_name: String::from("!"),
                args: vec![],
                dot_l: None,
                selector_l: Some(selector_l),
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
        let lhs: Box<Node> = lhs;
        let rhs: Box<Node> = rhs;

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
        let pre = match (if_true.as_ref(), if_false.as_ref()) {
            (None, None) => unreachable!("at least one of if_true/if_false is required"),
            (None, Some(if_false)) => if_false,
            (Some(if_true), None) => if_true,
            (Some(_), Some(_)) => unreachable!("only one of if_true/if_false is required"),
        };

        let expression_l = pre.expression().join(cond.expression());
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
        patterns: Box<Vec<Node>>,
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
            patterns: *patterns,
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
        when_bodies: Box<Vec<Node>>,
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
            when_bodies: *when_bodies,
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
        let expression_l = body.expression().join(cond.expression());
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
                begin_l: None,
                end_l: None,
                expression_l,
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
                begin_l: None,
                end_l: None,
                expression_l,
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
        mut args: Box<Vec<Node>>,
        rparen_t: Option<Box<Token>>,
    ) -> Result<Box<Node>, ()> {
        let keyword_l = self.loc(&keyword_t);

        if type_ == KeywordCmd::Yield && !args.is_empty() {
            if let Some(Node::BlockPass(_)) = args.last() {
                self.error(DiagnosticMessage::BlockGivenToYield {}, &keyword_l);
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
                args: *args,
                keyword_l,
                expression_l,
            }),
            KeywordCmd::Defined => Node::Defined(Defined {
                value: Box::new(args.into_iter().next().unwrap()),
                keyword_l,
                begin_l,
                end_l,
                expression_l,
            }),
            KeywordCmd::Next => Node::Next(Next {
                args: *args,
                keyword_l,
                expression_l,
            }),
            KeywordCmd::Redo => Node::Redo(Redo { expression_l }),
            KeywordCmd::Retry => Node::Retry(Retry { expression_l }),
            KeywordCmd::Return => Node::Return(Return {
                args: *args,
                keyword_l,
                expression_l,
            }),
            KeywordCmd::Super => Node::Super(Super {
                args: *args,
                keyword_l,
                begin_l,
                end_l,
                expression_l,
            }),
            KeywordCmd::Yield => Node::Yield(Yield {
                args: *args,
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
            assoc_l,
            begin_l,
            expression_l,
        }))
    }

    pub(crate) fn begin_body(
        &self,
        compound_stmt: Option<Box<Node>>,
        rescue_bodies: Box<Vec<Node>>,
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
                    rescue_bodies: *rescue_bodies,
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
                    rescue_bodies: *rescue_bodies,
                    else_: None,
                    else_l,
                    expression_l,
                })))
            }
        } else if let Some((else_t, else_)) = else_ {
            let mut statements = vec![];

            if compound_stmt.is_some() {
                let compound_stmt = *compound_stmt.unwrap();
                match compound_stmt {
                    Node::Begin(Begin {
                        statements: stmts, ..
                    }) => statements = stmts,
                    other => statements.push(other),
                }
            }

            let parts = if else_.is_some() {
                vec![*else_.unwrap()]
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
            let ensure_body = ensure;
            let keyword_l = self.loc(&ensure_t);

            let begin_l = maybe_boxed_node_expr(&result).unwrap_or_else(|| self.loc(&ensure_t));

            let end_l = maybe_node_expr(&ensure_body.as_ref().map(|x| x.as_ref()))
                .unwrap_or_else(|| self.loc(&ensure_t));

            let expression_l = begin_l.join(&end_l);

            result = Some(Box::new(Node::Ensure(Ensure {
                body: result,
                ensure: ensure_body,
                keyword_l,
                expression_l,
            })))
        }

        result
    }

    //
    // Expression grouping
    //

    pub(crate) fn compstmt(&self, statements: Box<Vec<Node>>) -> Option<Box<Node>> {
        match &statements[..] {
            [] => None,
            [_] => Some(Box::new(statements.into_iter().next().unwrap())),
            _ => {
                let CollectionMap {
                    begin_l,
                    end_l,
                    expression_l,
                } = self.collection_map(&None, &statements, &None);

                Some(Box::new(Node::Begin(Begin {
                    statements: *statements,
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

        let new_begin_l = Some(new_begin_l);
        let new_end_l = Some(new_end_l);

        if body.is_some() {
            let mut body = *body.unwrap();
            match &mut body {
                Node::Mlhs(Mlhs {
                    begin_l,
                    end_l,
                    expression_l,
                    ..
                }) => {
                    // Synthesized (begin) from compstmt "a; b" or (mlhs)
                    // from multi_lhs "(a, b) = *foo".
                    *begin_l = new_begin_l;
                    *end_l = new_end_l;
                    *expression_l = new_expression_l;
                    Box::new(body)
                }
                Node::Begin(Begin {
                    begin_l,
                    end_l,
                    expression_l,
                    ..
                }) if begin_l.is_none() && end_l.is_none() => {
                    *begin_l = new_begin_l;
                    *end_l = new_end_l;
                    *expression_l = new_expression_l;
                    Box::new(body)
                }
                _ => {
                    let mut statements = vec![];
                    statements.push(body);
                    Box::new(Node::Begin(Begin {
                        statements,
                        begin_l: new_begin_l,
                        end_l: new_end_l,
                        expression_l: new_expression_l,
                    }))
                }
            }
        } else {
            // A nil expression: `()'.
            Box::new(Node::Begin(Begin {
                statements: vec![],
                begin_l: new_begin_l,
                end_l: new_end_l,
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

        let begin_l = Some(begin_l);
        let end_l = Some(end_l);

        if body.is_none() {
            // A nil expression: `begin end'.
            Box::new(Node::KwBegin(KwBegin {
                statements: vec![],
                begin_l,
                end_l,
                expression_l,
            }))
        } else {
            let body = *body.unwrap();
            match body {
                Node::Begin(Begin { statements, .. }) => {
                    // Synthesized (begin) from compstmt "a; b".
                    Box::new(Node::KwBegin(KwBegin {
                        statements,
                        begin_l,
                        end_l,
                        expression_l,
                    }))
                }
                other => {
                    let mut statements = vec![];
                    statements.push(other);
                    Box::new(Node::KwBegin(KwBegin {
                        statements,
                        begin_l,
                        end_l,
                        expression_l,
                    }))
                }
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
        in_bodies: Box<Vec<Node>>,
        else_t: Option<Box<Token>>,
        else_body: Option<Box<Node>>,
        end_t: Box<Token>,
    ) -> Box<Node> {
        let else_body = match (else_t.as_ref(), else_body.as_ref()) {
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
            in_bodies: *in_bodies,
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
            .unwrap_or_else(|| pattern.expression().clone())
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

        self.check_lvar_name(name.as_str(), &name_l)?;
        self.check_duplicate_pattern_variable(name.as_str(), &name_l)?;
        self.static_env.declare(name.as_str());

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

        self.check_lvar_name(name.as_str(), &name_l)?;
        self.check_duplicate_pattern_variable(name.as_str(), &name_l)?;
        self.static_env.declare(name.as_str());

        Ok(Box::new(Node::MatchVar(MatchVar {
            name,
            name_l,
            expression_l,
        })))
    }
    pub(crate) fn match_hash_var_from_str(
        &self,
        begin_t: Box<Token>,
        mut strings: Box<Vec<Node>>,
        end_t: Box<Token>,
    ) -> Result<Box<Node>, ()> {
        if strings.len() != 1 {
            self.error(
                DiagnosticMessage::SymbolLiteralWithInterpolation {},
                &self.loc(&begin_t).join(&self.loc(&end_t)),
            );
            return Err(());
        }

        let string = strings.remove(0);
        let result = match string {
            Node::Str(Str {
                value,
                begin_l,
                end_l,
                expression_l,
            }) => {
                let name = value.to_string_lossy();
                let mut name_l = expression_l.clone();

                self.check_lvar_name(name.as_str(), &name_l)?;
                self.check_duplicate_pattern_variable(name.as_str(), &name_l)?;

                self.static_env.declare(name.as_str());

                if let Some(begin_l) = begin_l.as_ref() {
                    let begin_d: i32 = begin_l
                        .size()
                        .try_into()
                        .expect("failed to convert usize loc into i32, is it too big?");
                    name_l = name_l.adjust_begin(begin_d)
                }

                if let Some(end_l) = end_l.as_ref() {
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
                self.match_hash_var_from_str(begin_t, Box::new(statements), end_t)?
            }
            _ => {
                self.error(
                    DiagnosticMessage::SymbolLiteralWithInterpolation {},
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
        let name = if name_t.is_none() {
            None
        } else {
            let t = name_t.unwrap();
            Some(self.match_var(t)?)
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
        kwargs: Box<Vec<Node>>,
        rbrace_t: Option<Box<Token>>,
    ) -> Box<Node> {
        let CollectionMap {
            begin_l,
            end_l,
            expression_l,
        } = self.collection_map(&lbrace_t, &kwargs, &rbrace_t);

        Box::new(Node::HashPattern(HashPattern {
            elements: *kwargs,
            begin_l,
            end_l,
            expression_l,
        }))
    }

    pub(crate) fn array_pattern(
        &self,
        lbrack_t: Option<Box<Token>>,
        elements: Box<Vec<Node>>,
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
                elements: *elements,
                begin_l,
                end_l,
                expression_l,
            }))
        } else {
            Box::new(Node::ArrayPattern(ArrayPattern {
                elements: *elements,
                begin_l,
                end_l,
                expression_l,
            }))
        }
    }

    pub(crate) fn find_pattern(
        &self,
        lbrack_t: Option<Box<Token>>,
        elements: Box<Vec<Node>>,
        rbrack_t: Option<Box<Token>>,
    ) -> Box<Node> {
        let CollectionMap {
            begin_l,
            end_l,
            expression_l,
        } = self.collection_map(&lbrack_t, &elements, &rbrack_t);

        Box::new(Node::FindPattern(FindPattern {
            elements: *elements,
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
                self.check_duplicate_pattern_key(
                    clone_value(&label_t).as_str(),
                    &self.loc(&label_t),
                )?;
                self.pair_keyword(label_t, value)
            }
            PKwLabel::QuotedLabel((begin_t, parts, end_t)) => {
                let label_loc = self.loc(&begin_t).join(&self.loc(&end_t));

                match self.static_string(&parts) {
                    Some(var_name) => self.check_duplicate_pattern_key(&var_name, &label_loc)?,
                    _ => {
                        self.error(
                            DiagnosticMessage::SymbolLiteralWithInterpolation {},
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
        let cond = cond;

        match *cond {
            Node::Begin(Begin {
                statements,
                begin_l,
                end_l,
                expression_l,
            }) => {
                if statements.len() == 1 {
                    let stmt = statements.into_iter().next().unwrap();
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
            regexp if matches!(regexp, Node::Regexp(_)) => {
                let expression_l = regexp.expression().clone();

                Box::new(Node::MatchCurrentLine(MatchCurrentLine {
                    re: Box::new(regexp),
                    expression_l,
                }))
            }
            other => Box::new(other),
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
                Node::ForwardArg(_) | Node::Kwnilarg(_) => {
                    // ignore
                }
                _ => {
                    unreachable!("unsupported arg type {:?}", arg)
                }
            }
        }
    }

    fn arg_name<'a>(&self, node: &'a Node) -> Option<&'a str> {
        match node {
            Node::Arg(Arg { name, .. })
            | Node::Optarg(Optarg { name, .. })
            | Node::Kwarg(Kwarg { name, .. })
            | Node::Kwoptarg(Kwoptarg { name, .. })
            | Node::Shadowarg(Shadowarg { name, .. })
            | Node::Blockarg(Blockarg { name, .. }) => Some(name.as_str()),

            Node::Restarg(Restarg { name, .. }) | Node::Kwrestarg(Kwrestarg { name, .. }) => {
                name.as_ref().map(|s| s.as_str())
            }
            _ => {
                unreachable!("unsupported arg {:?}", node)
            }
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
            }) => name_l.as_ref().unwrap_or(expression_l),
            Node::Kwrestarg(Kwrestarg {
                name_l,
                expression_l,
                ..
            }) => name_l.as_ref().unwrap_or(expression_l),
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
                map.insert(this_name.to_string(), this_arg);
            }
            Some(that_arg) => {
                let that_name = match self.arg_name(*that_arg) {
                    Some(name) => name,
                    None => return,
                };
                if self.arg_name_collides(this_name, that_name) {
                    self.error(
                        DiagnosticMessage::DuplicatedArgumentName {},
                        self.arg_name_loc(this_arg),
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
                    numparam: String::from(name),
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
                        numparam: String::from(name),
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
        let mut rest = all_chars;

        if (first.is_lowercase() || first == '_') && rest.all(|c| c.is_alphanumeric() || c == '_') {
            Ok(())
        } else {
            self.error(DiagnosticMessage::KeyMustBeValidAsLocalVariable {}, loc);
            Err(())
        }
    }

    pub(crate) fn check_duplicate_pattern_variable(&self, name: &str, loc: &Loc) -> Result<(), ()> {
        if name.starts_with('_') {
            return Ok(());
        }

        if self.pattern_variables.is_declared(name) {
            self.error(DiagnosticMessage::DuplicateVariableName {}, loc);
            return Err(());
        }

        self.pattern_variables.declare(name);
        Ok(())
    }

    pub(crate) fn check_duplicate_pattern_key(&self, name: &str, loc: &Loc) -> Result<(), ()> {
        if self.pattern_hash_keys.is_declared(name) {
            self.error(DiagnosticMessage::DuplicateKeyName {}, loc);
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
                    let value = value.to_string_lossy();
                    result.push_str(value.as_str())
                }
                Node::Begin(Begin { statements, .. }) => {
                    if let Some(s) = self.static_string(statements) {
                        result.push_str(&s)
                    } else {
                        return None;
                    }
                }
                _ => {
                    return None;
                }
            }
        }

        Some(result)
    }

    #[cfg(feature = "onig")]
    pub(crate) fn build_static_regexp(
        &self,
        parts: &[Node],
        options: &Option<String>,
        loc: &Loc,
    ) -> Option<Regex> {
        let source = self.static_string(&parts)?;
        let mut reg_options = RegexOptions::REGEX_OPTION_NONE;
        reg_options |= RegexOptions::REGEX_OPTION_CAPTURE_GROUP;
        if let Some(options_s) = options.as_ref().map(|s| s.as_str()) {
            if options_s.as_bytes().contains(&b'x') {
                reg_options |= RegexOptions::REGEX_OPTION_EXTEND;
            }
        }

        let bytes = onig::EncodedBytes::ascii(source.as_bytes());

        match Regex::with_options_and_encoding(bytes, reg_options, onig::Syntax::ruby()) {
            Ok(regex) => Some(regex),
            Err(err) => {
                self.error(
                    DiagnosticMessage::RegexError {
                        error: String::from(err.description()),
                    },
                    loc,
                );
                None
            }
        }
    }

    #[cfg(feature = "onig")]
    pub(crate) fn validate_static_regexp(
        &self,
        parts: &[Node],
        options: &Option<String>,
        loc: &Loc,
    ) {
        self.build_static_regexp(parts, options, loc);
    }

    #[cfg(not(feature = "onig"))]
    pub(crate) fn validate_static_regexp(
        &self,
        _parts: &[Node],
        _options: &Option<String>,
        _loc: &Loc,
    ) {
    }

    #[cfg(feature = "onig")]
    pub(crate) fn static_regexp_captures(&self, node: &Node) -> Option<Vec<String>> {
        match node {
            Node::Regexp(Regexp {
                parts,
                options,
                expression_l,
                ..
            }) => {
                let mut re_options = &None;
                if let Some(Node::RegOpt(RegOpt { options, .. })) = options.as_ref().map(|b| &**b) {
                    re_options = options;
                };
                let regex = self.build_static_regexp(parts, re_options, expression_l)?;

                let mut result: Vec<String> = vec![];

                regex.foreach_name(|name, _| {
                    result.push(name.to_string());
                    true
                });

                return Some(result);
            }
            _ => {}
        }
        None
    }

    #[cfg(not(feature = "onig"))]
    pub(crate) fn static_regexp_captures(&self, _node: &Node) -> Option<Vec<String>> {
        None
    }

    pub(crate) fn loc(&self, token: &Token) -> Loc {
        token.loc.clone()
    }

    pub(crate) fn maybe_loc(&self, token: &Option<Box<Token>>) -> Option<Loc> {
        match token.as_ref() {
            Some(token) => Some(self.loc(token.as_ref())),
            None => None,
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

        let expression_l = collection_expr(parts);
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
        if let Some(begin_t) = begin_t.as_ref() {
            if clone_value(begin_t.as_ref()).as_str().starts_with("<<") {
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
        let begin_t = begin_t.as_ref().expect("bug: begin_t must be Some");
        let end_t = end_t.as_ref().expect("heredoc must have end_t");

        let heredoc_body_l = collection_expr(parts).unwrap_or_else(|| self.loc(end_t));
        let expression_l = self.loc(begin_t);
        let heredoc_end_l = self.loc(end_t);

        HeredocMap {
            heredoc_body_l,
            heredoc_end_l,
            expression_l,
        }
    }

    pub(crate) fn error(&self, message: DiagnosticMessage, loc: &Loc) {
        self.diagnostics.emit(Diagnostic {
            level: ErrorLevel::Error,
            message,
            loc: loc.clone(),
        })
    }

    pub(crate) fn warn(&self, message: DiagnosticMessage, loc: &Loc) {
        self.diagnostics.emit(Diagnostic {
            level: ErrorLevel::Warning,
            message,
            loc: loc.clone(),
        })
    }

    pub(crate) fn value_expr(&self, node: &Node) -> Result<(), ()> {
        if let Some(void_node) = self.void_value(node) {
            self.error(
                DiagnosticMessage::VoidValueExpression {},
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
                if_true.as_ref(),
                if_false.as_ref(),
            ) {
                (None, None) | (None, Some(_)) | (Some(_), None) => None,
                (Some(if_true), Some(if_false)) => check_condition(if_true, if_false),
            };

        match node {
            Node::Return(_) | Node::Break(_) | Node::Next(_) | Node::Redo(_) | Node::Retry(_) => {
                Some(node)
            }

            Node::MatchPattern(MatchPattern { value, .. })
            | Node::MatchPatternP(MatchPatternP { value, .. }) => self.void_value(value),

            Node::Begin(Begin { statements, .. }) | Node::KwBegin(KwBegin { statements, .. }) => {
                check_stmts(statements)
            }

            Node::IfTernary(IfTernary {
                if_true, if_false, ..
            }) => check_condition(if_true, if_false),

            Node::If(If {
                if_true, if_false, ..
            })
            | Node::IfMod(IfMod {
                if_true, if_false, ..
            }) => check_maybe_condition(if_true, if_false),

            Node::And(And { lhs, .. }) | Node::Or(Or { lhs, .. }) => self.void_value(lhs),

            _ => None,
        }
    }

    fn rewrite_hash_args_to_kwargs(&self, args: &mut Vec<Node>) {
        let len = args.len();

        if !args.is_empty() && self.is_kwargs(&args[len - 1]) {
            match args.pop().unwrap() {
                Node::Hash(Hash {
                    pairs,
                    expression_l,
                    ..
                }) => {
                    let kwargs = Node::Kwargs(Kwargs {
                        pairs,
                        expression_l,
                    });
                    args.push(kwargs);
                }
                _ => unreachable!(),
            }
        } else if len > 1
            && matches!(args[len - 1], Node::BlockPass(_))
            && self.is_kwargs(&args[len - 2])
        {
            let block_pass = args.pop().unwrap();
            match args.pop().unwrap() {
                Node::Hash(Hash {
                    pairs,
                    expression_l,
                    ..
                }) => {
                    let kwargs = Node::Kwargs(Kwargs {
                        pairs,
                        expression_l,
                    });
                    args.push(kwargs);
                    args.push(block_pass);
                }
                _ => unreachable!(),
            }
        }
    }

    fn is_kwargs(&self, node: &Node) -> bool {
        matches!(
            node,
            Node::Hash(Hash {
                begin_l: None,
                end_l: None,
                ..
            })
        )
    }
}

pub(crate) fn maybe_node_expr(node: &Option<&Node>) -> Option<Loc> {
    match node {
        Some(node) => Some(node.expression().clone()),
        None => None,
    }
}

pub(crate) fn maybe_boxed_node_expr(node: &Option<Box<Node>>) -> Option<Loc> {
    match node.as_ref() {
        Some(node) => Some(node.expression().clone()),
        None => None,
    }
}

pub(crate) fn collection_expr(nodes: &[Node]) -> Option<Loc> {
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

pub(crate) fn join_exprs(lhs: &Node, rhs: &Node) -> Loc {
    lhs.expression().join(rhs.expression())
}

pub(crate) fn join_maybe_exprs(lhs: &Option<&Node>, rhs: &Option<&Node>) -> Option<Loc> {
    join_maybe_locs(&maybe_node_expr(lhs), &maybe_node_expr(rhs))
}

pub(crate) fn join_maybe_locs(lhs: &Option<Loc>, rhs: &Option<Loc>) -> Option<Loc> {
    match (lhs.as_ref(), rhs.as_ref()) {
        (None, None) => None,
        (None, Some(rhs)) => Some(rhs.clone()),
        (Some(lhs), None) => Some(lhs.clone()),
        (Some(lhs), Some(rhs)) => Some(lhs.join(rhs)),
    }
}

pub(crate) struct CollectionMap {
    begin_l: Option<Loc>,
    end_l: Option<Loc>,
    expression_l: Loc,
}

pub(crate) struct HeredocMap {
    heredoc_body_l: Loc,
    heredoc_end_l: Loc,
    expression_l: Loc,
}
