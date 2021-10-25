#[cfg(feature = "onig")]
use onig::{Regex, RegexOptions};

use std::collections::HashMap;
use std::convert::TryInto;

crate::use_native_or_external!(Ptr);
crate::use_native_or_external!(StringPtr);
crate::use_native_or_external!(List);
crate::use_native_or_external!(Maybe);

use crate::error::Diagnostics;
use crate::nodes::internal;
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
    PlainLabel(Ptr<Token>),
    QuotedLabel((Ptr<Token>, Box<List<Node>>, Ptr<Token>)),
}

#[derive(Debug, Clone)]
pub(crate) enum ArgsType {
    Args(Maybe<Ptr<Node>>),
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

    pub(crate) fn nil(&self, nil_t: Ptr<Token>) -> Ptr<Node> {
        Ptr::new(Node::new_nil(self.loc(&nil_t)))
    }

    pub(crate) fn true_(&self, true_t: Ptr<Token>) -> Ptr<Node> {
        Ptr::new(Node::new_true(self.loc(&true_t)))
    }

    pub(crate) fn false_(&self, false_t: Ptr<Token>) -> Ptr<Node> {
        Ptr::new(Node::new_false(self.loc(&false_t)))
    }

    // Numerics

    pub(crate) fn integer(&self, integer_t: Ptr<Token>) -> Ptr<Node> {
        let expression_l = self.loc(&integer_t);
        Ptr::new(Node::new_int(value(integer_t), Maybe::none(), expression_l))
    }

    pub(crate) fn float(&self, float_t: Ptr<Token>) -> Ptr<Node> {
        let expression_l = self.loc(&float_t);
        Ptr::new(Node::new_float(value(float_t), Maybe::none(), expression_l))
    }

    pub(crate) fn rational(&self, rational_t: Ptr<Token>) -> Ptr<Node> {
        let expression_l = self.loc(&rational_t);
        Ptr::new(Node::new_rational(
            value(rational_t),
            Maybe::none(),
            expression_l,
        ))
    }

    pub(crate) fn complex(&self, complex_t: Ptr<Token>) -> Ptr<Node> {
        let expression_l = self.loc(&complex_t);
        Ptr::new(Node::new_complex(
            value(complex_t),
            Maybe::none(),
            expression_l,
        ))
    }

    pub(crate) fn unary_num(&self, unary_t: Ptr<Token>, numeric: Ptr<Node>) -> Ptr<Node> {
        let new_operator_l = self.loc(&unary_t);
        let sign = String::from(value(unary_t));
        let mut numeric = numeric.unptr();

        if let Some(int) = numeric.as_int_mut() {
            let new_value = StringPtr::from(sign + int.get_value().as_str());
            int.set_value(new_value);

            let new_expression_l = new_operator_l.join(int.get_expression_l());
            int.set_expression_l(new_expression_l);

            int.set_operator_l(Maybe::some(new_operator_l));
        } else if let Some(float) = numeric.as_float_mut() {
            let new_value = StringPtr::from(sign + float.get_value().as_str());
            float.set_value(new_value);

            let new_expression_l = new_operator_l.join(float.get_expression_l());
            float.set_expression_l(new_expression_l);

            float.set_operator_l(Maybe::some(new_operator_l));
        } else if let Some(rational) = numeric.as_rational_mut() {
            let new_value = StringPtr::from(sign + rational.get_value().as_str());
            rational.set_value(new_value);

            let new_expression_l = new_operator_l.join(rational.get_expression_l());
            rational.set_expression_l(new_expression_l);

            rational.set_operator_l(Maybe::some(new_operator_l));
        } else if let Some(complex) = numeric.as_complex_mut() {
            let new_value = StringPtr::from(sign + complex.get_value().as_str());
            complex.set_value(new_value);

            let new_expression_l = new_operator_l.join(complex.get_expression_l());
            complex.set_expression_l(new_expression_l);

            complex.set_operator_l(Maybe::some(new_operator_l));
        } else {
            unreachable!()
        }

        Ptr::new(numeric)
    }

    pub(crate) fn __line__(&self, line_t: Ptr<Token>) -> Ptr<Node> {
        Ptr::new(Node::new_line(self.loc(&line_t)))
    }

    // Strings

    pub(crate) fn str_node(
        &self,
        begin_t: Maybe<Ptr<Token>>,
        value: Bytes,
        parts: Box<List<Node>>,
        end_t: Maybe<Ptr<Token>>,
    ) -> Ptr<Node> {
        if self.is_heredoc(&begin_t) {
            let HeredocMap {
                heredoc_body_l,
                heredoc_end_l,
                expression_l,
            } = self.heredoc_map(&begin_t, &parts, &end_t);

            Ptr::new(Node::new_heredoc(
                *parts,
                heredoc_body_l,
                heredoc_end_l,
                expression_l,
            ))
        } else {
            let CollectionMap {
                begin_l,
                end_l,
                expression_l,
            } = self.collection_map(&begin_t, &parts, &end_t);

            Ptr::new(Node::new_str(value, begin_l, end_l, expression_l))
        }
    }

    pub(crate) fn string_internal(&self, string_t: Ptr<Token>) -> Ptr<Node> {
        let expression_l = self.loc(&string_t);
        let value = string_t.unptr().into_token_value();
        Ptr::new(Node::new_str(
            value,
            Maybe::none(),
            Maybe::none(),
            expression_l,
        ))
    }

    pub(crate) fn string_compose(
        &self,
        begin_t: Maybe<Ptr<Token>>,
        parts: Box<List<Node>>,
        end_t: Maybe<Ptr<Token>>,
    ) -> Ptr<Node> {
        if parts.is_empty() {
            return self.str_node(begin_t, Bytes::empty(), parts, end_t);
        } else if parts.len() == 1 {
            let part = parts.first().unwrap();

            if (part.is_str() || part.is_dstr() || part.is_heredoc())
                && begin_t.is_none()
                && end_t.is_none()
            {
                return Ptr::new(
                    parts
                        .into_iter()
                        .next()
                        .expect("expected at least 1 element"),
                );
            }

            if let Some(part) = part.as_str() {
                let value = part.get_value().clone();
                return self.str_node(begin_t, value, parts, end_t);
            }

            if part.is_dstr() || part.is_heredoc() {
                unreachable!()
            }
        }

        if self.is_heredoc(&begin_t) {
            let HeredocMap {
                heredoc_body_l,
                heredoc_end_l,
                expression_l,
            } = self.heredoc_map(&begin_t, &parts, &end_t);

            Ptr::new(Node::new_heredoc(
                *parts,
                heredoc_body_l,
                heredoc_end_l,
                expression_l,
            ))
        } else {
            let CollectionMap {
                begin_l,
                end_l,
                expression_l,
            } = self.collection_map(&begin_t, &parts, &end_t);

            Ptr::new(Node::new_dstr(*parts, begin_l, end_l, expression_l))
        }
    }

    pub(crate) fn character(&self, char_t: Ptr<Token>) -> Ptr<Node> {
        let str_loc = self.loc(&char_t);

        let begin_l = Maybe::some(str_loc.with_end(str_loc.begin() + 1));
        let end_l = Maybe::none();
        let expression_l = str_loc;

        let value = char_t.unptr().into_token_value();
        Ptr::new(Node::new_str(value, begin_l, end_l, expression_l))
    }

    pub(crate) fn __file__(&self, file_t: Ptr<Token>) -> Ptr<Node> {
        Ptr::new(Node::new_file(self.loc(&file_t)))
    }

    // Symbols

    fn validate_sym_value(&self, value: &Bytes, loc: &Loc) {
        if !value.is_valid_utf8() {
            self.error(
                DiagnosticMessage::new_invalid_symbol(StringPtr::from("UTF-8")),
                loc,
            )
        }
    }

    pub(crate) fn symbol(&self, start_t: Ptr<Token>, value_t: Ptr<Token>) -> Ptr<Node> {
        let expression_l = self.loc(&start_t).join(&self.loc(&value_t));
        let begin_l = Maybe::some(self.loc(&start_t));
        let value = value_t.unptr().into_token_value();
        self.validate_sym_value(&value, &expression_l);
        Ptr::new(Node::new_sym(value, begin_l, Maybe::none(), expression_l))
    }

    pub(crate) fn symbol_internal(&self, symbol_t: Ptr<Token>) -> Ptr<Node> {
        let expression_l = self.loc(&symbol_t);
        let value = symbol_t.unptr().into_token_value();
        self.validate_sym_value(&value, &expression_l);
        Ptr::new(Node::new_sym(
            value,
            Maybe::none(),
            Maybe::none(),
            expression_l,
        ))
    }

    pub(crate) fn symbol_compose(
        &self,
        begin_t: Ptr<Token>,
        parts: Box<List<Node>>,
        end_t: Ptr<Token>,
    ) -> Ptr<Node> {
        if parts.len() == 1 && parts.first().unwrap().is_str() {
            let value = parts.first().unwrap().as_str().unwrap().get_value();

            let CollectionMap {
                begin_l,
                end_l,
                expression_l,
            } = self.collection_map(&Maybe::some(begin_t), &[], &Maybe::some(end_t));

            self.validate_sym_value(value, &expression_l);

            return Ptr::new(Node::new_sym(value.clone(), begin_l, end_l, expression_l));
        }

        let CollectionMap {
            begin_l,
            end_l,
            expression_l,
        } = self.collection_map(&Maybe::some(begin_t), &parts, &Maybe::some(end_t));
        Ptr::new(Node::new_dsym(*parts, begin_l, end_l, expression_l))
    }

    // Executable strings

    pub(crate) fn xstring_compose(
        &self,
        begin_t: Ptr<Token>,
        parts: Box<List<Node>>,
        end_t: Ptr<Token>,
    ) -> Ptr<Node> {
        let begin_l = self.loc(&begin_t);
        if lossy_value(begin_t).as_str().starts_with("<<") {
            let heredoc_body_l = collection_expr(&parts).unwrap_or_else(|| self.loc(&end_t));
            let heredoc_end_l = self.loc(&end_t);
            let expression_l = begin_l;

            Ptr::new(Node::new_x_heredoc(
                *parts,
                heredoc_body_l,
                heredoc_end_l,
                expression_l,
            ))
        } else {
            let end_l = self.loc(&end_t);
            let expression_l = begin_l.join(&end_l);

            Ptr::new(Node::new_xstr(*parts, begin_l, end_l, expression_l))
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

        let dedent_heredoc_parts = |parts: List<Node>| -> List<Node> {
            let parts = parts
                .into_iter()
                .filter_map(|part| {
                    if part.is_str() {
                        let internal::Str {
                            value,
                            begin_l,
                            end_l,
                            expression_l,
                        } = part.into_str().into_internal();
                        let value = Self::dedent_string(value, dedent_level);
                        if value.is_empty() {
                            None
                        } else {
                            Some(Node::new_str(value, begin_l, end_l, expression_l))
                        }
                    } else if part.is_begin()
                        || part.is_gvar()
                        || part.is_back_ref()
                        || part.is_nth_ref()
                        || part.is_ivar()
                        || part.is_cvar()
                    {
                        Some(part)
                    } else {
                        unreachable!("unsupported heredoc child {}", part.str_type())
                    }
                })
                .collect::<Vec<_>>();
            List::from(parts)
        };

        if node.is_heredoc() {
            let internal::Heredoc {
                parts,
                heredoc_body_l,
                heredoc_end_l,
                expression_l,
            } = node.into_heredoc().into_internal();
            let parts = dedent_heredoc_parts(parts);
            Node::new_heredoc(parts, heredoc_body_l, heredoc_end_l, expression_l)
        } else if node.is_x_heredoc() {
            let internal::XHeredoc {
                parts,
                heredoc_body_l,
                heredoc_end_l,
                expression_l,
            } = node.into_x_heredoc().into_internal();
            let parts = dedent_heredoc_parts(parts);
            Node::new_x_heredoc(parts, heredoc_body_l, heredoc_end_l, expression_l)
        } else {
            unreachable!("unsupported heredoc_dedent argument {}", node.str_type())
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

        Bytes::new(List::from(&s.as_raw()[i..]))
    }

    // Regular expressions

    pub(crate) fn regexp_options(&self, regexp_end_t: Ptr<Token>) -> Maybe<Ptr<Node>> {
        if regexp_end_t.loc().end() - regexp_end_t.loc().begin() == 1 {
            // no regexp options, only trailing "/"
            return Maybe::none();
        }
        let expression_l = self.loc(&regexp_end_t).adjust_begin(1);
        let options = value(regexp_end_t);
        let mut options = options.as_str().chars().skip(1).collect::<Vec<_>>();
        options.sort_unstable();
        options.dedup();
        let options = if options.is_empty() {
            Maybe::none()
        } else {
            Maybe::some(StringPtr::from(options.into_iter().collect::<String>()))
        };

        Maybe::some(Ptr::new(Node::new_reg_opt(options, expression_l)))
    }

    pub(crate) fn regexp_compose(
        &self,
        begin_t: Ptr<Token>,
        parts: Box<List<Node>>,
        end_t: Ptr<Token>,
        options: Maybe<Ptr<Node>>,
    ) -> Ptr<Node> {
        let begin_l = self.loc(&begin_t);
        let end_l = self.loc(&end_t).resize(1);
        let expression_l =
            begin_l.join(&maybe_boxed_node_expr(&options).unwrap_or_else(|| self.loc(&end_t)));

        if options.is_some() && options.as_ref().unwrap().is_reg_opt() {
            let options = options
                .as_ref()
                .unwrap()
                .as_reg_opt()
                .unwrap()
                .get_options();
            self.validate_static_regexp(&parts, options, &expression_l)
        } else if options.is_none() {
            self.validate_static_regexp(&parts, &Maybe::none(), &expression_l)
        } else {
            unreachable!("must be Option<RegOpt>")
        }

        Ptr::new(Node::new_regexp(
            *parts,
            options,
            begin_l,
            end_l,
            expression_l,
        ))
    }

    // Arrays

    pub(crate) fn array(
        &self,
        begin_t: Maybe<Ptr<Token>>,
        elements: Box<List<Node>>,
        end_t: Maybe<Ptr<Token>>,
    ) -> Ptr<Node> {
        let CollectionMap {
            begin_l,
            end_l,
            expression_l,
        } = self.collection_map(&begin_t, &elements, &end_t);

        Ptr::new(Node::new_array(*elements, begin_l, end_l, expression_l))
    }

    pub(crate) fn splat(&self, star_t: Ptr<Token>, value: Maybe<Ptr<Node>>) -> Ptr<Node> {
        let operator_l = self.loc(&star_t);
        let expression_l = operator_l.maybe_join(&maybe_boxed_node_expr(&value));

        Ptr::new(Node::new_splat(value, operator_l, expression_l))
    }

    pub(crate) fn word(&self, parts: Box<List<Node>>) -> Ptr<Node> {
        if parts.len() == 1 && (parts[0].is_str() || parts[0].is_dstr()) {
            let part = parts
                .into_iter()
                .next()
                .expect("parts is supposed to have exactly 1 element");
            return Ptr::new(part);
        }

        let CollectionMap {
            begin_l,
            end_l,
            expression_l,
        } = self.collection_map(&Maybe::none(), &parts, &Maybe::none());

        Ptr::new(Node::new_dstr(*parts, begin_l, end_l, expression_l))
    }

    pub(crate) fn words_compose(
        &self,
        begin_t: Ptr<Token>,
        elements: Box<List<Node>>,
        end_t: Ptr<Token>,
    ) -> Ptr<Node> {
        let begin_l = self.loc(&begin_t);
        let end_l = self.loc(&end_t);
        let expression_l = begin_l.join(&end_l);
        Ptr::new(Node::new_array(
            *elements,
            Maybe::some(begin_l),
            Maybe::some(end_l),
            expression_l,
        ))
    }

    pub(crate) fn symbols_compose(
        &self,
        begin_t: Ptr<Token>,
        parts: Box<List<Node>>,
        end_t: Ptr<Token>,
    ) -> Ptr<Node> {
        let parts = parts
            .into_iter()
            .map(|part| {
                if part.is_str() {
                    let internal::Str {
                        value,
                        begin_l,
                        end_l,
                        expression_l,
                    } = part.into_str().into_internal();
                    self.validate_sym_value(&value, &expression_l);
                    Node::new_sym(value, begin_l, end_l, expression_l)
                } else if part.is_dstr() {
                    let internal::Dstr {
                        parts,
                        begin_l,
                        end_l,
                        expression_l,
                    } = part.into_dstr().into_internal();
                    Node::new_dsym(parts, begin_l, end_l, expression_l)
                } else {
                    part
                }
            })
            .collect::<Vec<_>>();

        let begin_l = self.loc(&begin_t);
        let end_l = self.loc(&end_t);
        let expression_l = begin_l.join(&end_l);
        Ptr::new(Node::new_array(
            List::from(parts),
            Maybe::some(begin_l),
            Maybe::some(end_l),
            expression_l,
        ))
    }

    // Hashes

    pub(crate) fn pair(&self, key: Ptr<Node>, assoc_t: Ptr<Token>, value: Ptr<Node>) -> Ptr<Node> {
        let operator_l = self.loc(&assoc_t);
        let expression_l = join_exprs(&key, &value);

        Ptr::new(Node::new_pair(key, value, operator_l, expression_l))
    }

    pub(crate) fn pair_keyword(&self, key_t: Ptr<Token>, value: Ptr<Node>) -> Ptr<Node> {
        let key_loc = self.loc(&key_t);
        let key_l = key_loc.adjust_end(-1);
        let colon_l = key_loc.with_begin(key_loc.end() - 1);
        let expression_l = key_loc.join(value.expression());

        let key = key_t.unptr().into_token_value();
        self.validate_sym_value(&key, &key_l);

        Ptr::new(Node::new_pair(
            Ptr::new(Node::new_sym(key, Maybe::none(), Maybe::none(), key_l)),
            value,
            colon_l,
            expression_l,
        ))
    }

    pub(crate) fn pair_quoted(
        &self,
        begin_t: Ptr<Token>,
        parts: Box<List<Node>>,
        end_t: Ptr<Token>,
        value: Ptr<Node>,
    ) -> Ptr<Node> {
        let end_l = self.loc(&end_t);

        let quote_loc = Loc::new(end_l.end() - 2, end_l.end() - 1);

        let colon_l = end_l.with_begin(end_l.end() - 1);

        let end_t = end_t.unptr();
        let end_t: Ptr<Token> = Ptr::new(Token::new(
            end_t.token_type(),
            end_t.into_token_value(),
            quote_loc,
            LexState::default(),
            LexState::default(),
        ));
        let expression_l = self.loc(&begin_t).join(value.expression());

        Ptr::new(Node::new_pair(
            self.symbol_compose(begin_t, parts, end_t),
            value,
            colon_l,
            expression_l,
        ))
    }

    pub(crate) fn kwsplat(&self, dstar_t: Ptr<Token>, value: Ptr<Node>) -> Ptr<Node> {
        let operator_l = self.loc(&dstar_t);
        let expression_l = value.expression().join(&operator_l);

        Ptr::new(Node::new_kwsplat(value, operator_l, expression_l))
    }

    pub(crate) fn associate(
        &self,
        begin_t: Maybe<Ptr<Token>>,
        pairs: Box<List<Node>>,
        end_t: Maybe<Ptr<Token>>,
    ) -> Ptr<Node> {
        let CollectionMap {
            begin_l,
            end_l,
            expression_l,
        } = self.collection_map(&begin_t, &pairs, &end_t);

        Ptr::new(Node::new_hash(*pairs, begin_l, end_l, expression_l))
    }

    // Ranges

    pub(crate) fn range_inclusive(
        &self,
        left: Maybe<Ptr<Node>>,
        dot2_t: Ptr<Token>,
        right: Maybe<Ptr<Node>>,
    ) -> Ptr<Node> {
        let operator_l = self.loc(&dot2_t);
        let expression_l = operator_l
            .maybe_join(&maybe_boxed_node_expr(&left))
            .maybe_join(&maybe_boxed_node_expr(&right));

        Ptr::new(Node::new_irange(left, right, operator_l, expression_l))
    }

    pub(crate) fn range_exclusive(
        &self,
        left: Maybe<Ptr<Node>>,
        dot3_t: Ptr<Token>,
        right: Maybe<Ptr<Node>>,
    ) -> Ptr<Node> {
        let operator_l = self.loc(&dot3_t);
        let expression_l = operator_l
            .maybe_join(&maybe_boxed_node_expr(&left))
            .maybe_join(&maybe_boxed_node_expr(&right));

        Ptr::new(Node::new_erange(left, right, operator_l, expression_l))
    }

    //
    // Access
    //

    pub(crate) fn self_(&self, token: Ptr<Token>) -> Ptr<Node> {
        Ptr::new(Node::new_self_(self.loc(&token)))
    }

    pub(crate) fn lvar(&self, token: Ptr<Token>) -> Ptr<Node> {
        let expression_l = self.loc(&token);
        Ptr::new(Node::new_lvar(value(token), expression_l))
    }

    pub(crate) fn ivar(&self, token: Ptr<Token>) -> Ptr<Node> {
        let expression_l = self.loc(&token);
        Ptr::new(Node::new_ivar(value(token), expression_l))
    }

    pub(crate) fn gvar(&self, token: Ptr<Token>) -> Ptr<Node> {
        let expression_l = self.loc(&token);
        Ptr::new(Node::new_gvar(value(token), expression_l))
    }

    pub(crate) fn cvar(&self, token: Ptr<Token>) -> Ptr<Node> {
        let expression_l = self.loc(&token);
        Ptr::new(Node::new_cvar(value(token), expression_l))
    }

    pub(crate) fn back_ref(&self, token: Ptr<Token>) -> Ptr<Node> {
        let expression_l = self.loc(&token);
        Ptr::new(Node::new_back_ref(value(token), expression_l))
    }

    const MAX_NTH_REF: usize = 0b111111111111111111111111111111;

    pub(crate) fn nth_ref(&self, token: Ptr<Token>) -> Ptr<Node> {
        let expression_l = self.loc(&token);
        let name = value(token);
        let name = &name.as_str()[1..];
        let parsed = name.parse::<usize>();
        let name = StringPtr::from(name);

        if parsed.is_err() || parsed.map(|n| n > Self::MAX_NTH_REF) == Ok(true) {
            self.warn(
                DiagnosticMessage::new_nth_ref_is_too_big(name.clone()),
                &expression_l,
            )
        }

        Ptr::new(Node::new_nth_ref(name, expression_l))
    }
    pub(crate) fn accessible(&self, node: Ptr<Node>) -> Ptr<Node> {
        if node.is_lvar() {
            let internal::Lvar { name, expression_l } = node.unptr().into_lvar().into_internal();
            let name_s = name.as_str();
            if self.static_env.is_declared(name_s) {
                if let Some(current_arg) = self.current_arg_stack.top() {
                    if current_arg == name_s {
                        self.error(
                            DiagnosticMessage::new_circular_argument_reference(name.clone()),
                            &expression_l,
                        );
                    }
                }

                Ptr::new(Node::new_lvar(name, expression_l))
            } else {
                Ptr::new(Node::new_send(
                    Maybe::none(),
                    name,
                    list![],
                    Maybe::none(),
                    Maybe::some(expression_l.clone()),
                    Maybe::none(),
                    Maybe::none(),
                    Maybe::none(),
                    expression_l,
                ))
            }
        } else {
            node
        }
    }

    pub(crate) fn const_(&self, name_t: Ptr<Token>) -> Ptr<Node> {
        let name_l = self.loc(&name_t);
        let expression_l = name_l.clone();

        Ptr::new(Node::new_const(
            Maybe::none(),
            value(name_t),
            Maybe::none(),
            name_l,
            expression_l,
        ))
    }

    pub(crate) fn const_global(&self, t_colon3: Ptr<Token>, name_t: Ptr<Token>) -> Ptr<Node> {
        let scope = Ptr::new(Node::new_cbase(self.loc(&t_colon3)));

        let name_l = self.loc(&name_t);
        let expression_l = scope.expression().join(&name_l);
        let double_colon_l = self.loc(&t_colon3);

        Ptr::new(Node::new_const(
            Maybe::some(scope),
            value(name_t),
            Maybe::some(double_colon_l),
            name_l,
            expression_l,
        ))
    }

    pub(crate) fn const_fetch(
        &self,
        scope: Ptr<Node>,
        t_colon2: Ptr<Token>,
        name_t: Ptr<Token>,
    ) -> Ptr<Node> {
        let scope: Ptr<Node> = scope;
        let name_l = self.loc(&name_t);
        let expression_l = scope.expression().join(&name_l);
        let double_colon_l = self.loc(&t_colon2);

        Ptr::new(Node::new_const(
            Maybe::some(scope),
            value(name_t),
            Maybe::some(double_colon_l),
            name_l,
            expression_l,
        ))
    }

    pub(crate) fn __encoding__(&self, encoding_t: Ptr<Token>) -> Ptr<Node> {
        Ptr::new(Node::new_encoding(self.loc(&encoding_t)))
    }

    //
    // Assignments
    //

    pub(crate) fn assignable(&self, node: Ptr<Node>) -> Result<Ptr<Node>, ()> {
        let node = if node.is_cvar() {
            let internal::Cvar { name, expression_l } = node.unptr().into_cvar().into_internal();
            Node::new_cvasgn(
                name,
                Maybe::none(),
                expression_l.clone(),
                Maybe::none(),
                expression_l,
            )
        } else if node.is_ivar() {
            let internal::Ivar { name, expression_l } = node.unptr().into_ivar().into_internal();
            Node::new_ivasgn(
                name,
                Maybe::none(),
                expression_l.clone(),
                Maybe::none(),
                expression_l,
            )
        } else if node.is_gvar() {
            let internal::Gvar { name, expression_l } = node.unptr().into_gvar().into_internal();
            Node::new_gvasgn(
                name,
                Maybe::none(),
                expression_l.clone(),
                Maybe::none(),
                expression_l,
            )
        } else if node.is_const() {
            let internal::Const {
                scope,
                name,
                double_colon_l,
                name_l,
                expression_l,
            } = node.unptr().into_const().into_internal();
            if !self.context.is_dynamic_const_definition_allowed() {
                self.error(
                    DiagnosticMessage::new_dynamic_constant_assignment(),
                    &expression_l,
                );
                return Err(());
            }
            Node::new_casgn(
                scope,
                name,
                Maybe::none(),
                double_colon_l,
                name_l,
                Maybe::none(),
                expression_l,
            )
        } else if node.is_lvar() {
            let internal::Lvar { name, expression_l } = node.unptr().into_lvar().into_internal();
            let name_s = name.as_str();
            self.check_assignment_to_numparam(name_s, &expression_l)?;
            self.check_reserved_for_numparam(name_s, &expression_l)?;

            self.static_env.declare(name_s);

            Node::new_lvasgn(
                name,
                Maybe::none(),
                expression_l.clone(),
                Maybe::none(),
                expression_l,
            )
        } else if let Some(self_) = node.as_self_() {
            let expression_l = self_.get_expression_l();
            self.error(DiagnosticMessage::new_cant_assign_to_self(), expression_l);
            return Err(());
        } else if let Some(nil) = node.as_nil() {
            let expression_l = nil.get_expression_l();
            self.error(DiagnosticMessage::new_cant_assign_to_nil(), expression_l);
            return Err(());
        } else if let Some(true_) = node.as_true() {
            let expression_l = true_.get_expression_l();
            self.error(DiagnosticMessage::new_cant_assign_to_true(), expression_l);
            return Err(());
        } else if let Some(false_) = node.as_false() {
            let expression_l = false_.get_expression_l();
            self.error(DiagnosticMessage::new_cant_assign_to_false(), expression_l);
            return Err(());
        } else if let Some(file) = node.as_file() {
            let expression_l = file.get_expression_l();
            self.error(DiagnosticMessage::new_cant_assign_to_file(), expression_l);
            return Err(());
        } else if let Some(line) = node.as_line() {
            let expression_l = line.get_expression_l();
            self.error(DiagnosticMessage::new_cant_assign_to_line(), expression_l);
            return Err(());
        } else if let Some(encoding) = node.as_encoding() {
            let expression_l = encoding.get_expression_l();
            self.error(
                DiagnosticMessage::new_cant_assign_to_encoding(),
                expression_l,
            );
            return Err(());
        } else if let Some(back_ref) = node.as_back_ref() {
            let expression_l = back_ref.get_expression_l();
            let name = back_ref.get_name().to_owned();
            self.error(DiagnosticMessage::new_cant_set_variable(name), expression_l);
            return Err(());
        } else if let Some(nth_ref) = node.as_nth_ref() {
            let name = nth_ref.get_name().as_str();
            let expression_l = nth_ref.get_expression_l();
            self.error(
                DiagnosticMessage::new_cant_set_variable(StringPtr::from(format!("${}", name))),
                expression_l,
            );
            return Err(());
        } else {
            unreachable!("{:?} can't be used in assignment", node)
        };

        Ok(Ptr::new(node))
    }

    pub(crate) fn const_op_assignable(&self, node: Ptr<Node>) -> Ptr<Node> {
        if node.is_const() {
            let internal::Const {
                scope,
                name,
                double_colon_l,
                name_l,
                expression_l,
            } = node.unptr().into_const().into_internal();
            Ptr::new(Node::new_casgn(
                scope,
                name,
                Maybe::none(),
                double_colon_l,
                name_l,
                Maybe::none(),
                expression_l,
            ))
        } else {
            unreachable!("unsupported const_op_assignable arument: {:?}", node)
        }
    }

    pub(crate) fn assign(
        &self,
        lhs: Ptr<Node>,
        eql_t: Ptr<Token>,
        new_rhs: Ptr<Node>,
    ) -> Ptr<Node> {
        let op_l = Maybe::some(self.loc(&eql_t));
        let expr_l = join_exprs(&lhs, &new_rhs);
        let mut lhs = lhs.unptr();

        if let Some(cvasgn) = lhs.as_cvasgn_mut() {
            cvasgn.set_expression_l(expr_l);
            cvasgn.set_operator_l(op_l);
            cvasgn.set_value(Maybe::some(new_rhs));
        } else if let Some(ivasgn) = lhs.as_ivasgn_mut() {
            ivasgn.set_expression_l(expr_l);
            ivasgn.set_operator_l(op_l);
            ivasgn.set_value(Maybe::some(new_rhs));
        } else if let Some(gvasgn) = lhs.as_gvasgn_mut() {
            gvasgn.set_expression_l(expr_l);
            gvasgn.set_operator_l(op_l);
            gvasgn.set_value(Maybe::some(new_rhs));
        } else if let Some(lvasgn) = lhs.as_lvasgn_mut() {
            lvasgn.set_expression_l(expr_l);
            lvasgn.set_operator_l(op_l);
            lvasgn.set_value(Maybe::some(new_rhs));
        } else if let Some(casgn) = lhs.as_casgn_mut() {
            casgn.set_expression_l(expr_l);
            casgn.set_operator_l(op_l);
            casgn.set_value(Maybe::some(new_rhs));
        } else if let Some(index_asgn) = lhs.as_index_asgn_mut() {
            index_asgn.set_expression_l(expr_l);
            index_asgn.set_operator_l(op_l);
            index_asgn.set_value(Maybe::some(new_rhs));
        } else if let Some(send) = lhs.as_send_mut() {
            send.set_expression_l(expr_l);
            send.set_operator_l(op_l);
            if send.get_args().is_empty() {
                send.set_args(list![new_rhs.unptr()]);
            } else {
                unreachable!("can't assign to method call with args")
            }
        } else if let Some(c_send) = lhs.as_c_send_mut() {
            c_send.set_expression_l(expr_l);
            c_send.set_operator_l(op_l);
            if c_send.get_args().is_empty() {
                c_send.set_args(list![new_rhs.unptr()]);
            } else {
                unreachable!("can't assign to method call with args")
            }
        } else {
            unreachable!("{:?} can't be used in assignment", lhs)
        }

        Ptr::new(lhs)
    }

    pub(crate) fn op_assign(
        &self,
        mut lhs: Ptr<Node>,
        op_t: Ptr<Token>,
        rhs: Ptr<Node>,
    ) -> Result<Ptr<Node>, ()> {
        let operator_l = self.loc(&op_t);
        let mut operator = String::from(value(op_t));
        operator.pop();
        let expression_l = join_exprs(&lhs, &rhs);

        if lhs.is_gvasgn()
            || lhs.is_ivasgn()
            || lhs.is_lvasgn()
            || lhs.is_cvasgn()
            || lhs.is_casgn()
            || lhs.is_send()
            || lhs.is_c_send()
        {
            // ignore
        } else if lhs.is_index() {
            let internal::Index {
                recv,
                indexes,
                begin_l,
                end_l,
                expression_l,
            } = lhs.unptr().into_index().into_internal();
            lhs = Ptr::new(Node::new_index_asgn(
                recv,
                indexes,
                Maybe::none(),
                begin_l,
                end_l,
                Maybe::none(),
                expression_l,
            ));
        } else if lhs.is_back_ref() {
            let internal::BackRef { name, expression_l } =
                lhs.unptr().into_back_ref().into_internal();
            self.error(
                DiagnosticMessage::new_cant_set_variable(name),
                &expression_l,
            );
            return Err(());
        } else if lhs.is_nth_ref() {
            let nth_ref = lhs.as_nth_ref().unwrap();
            let name = nth_ref.get_name().as_str();
            let expression_l = nth_ref.get_expression_l();
            self.error(
                DiagnosticMessage::new_cant_set_variable(StringPtr::from(format!("${}", name))),
                expression_l,
            );
            return Err(());
        } else {
            unreachable!("unsupported op_assign lhs {:?}", lhs)
        }

        let recv: Ptr<Node> = lhs;
        let value: Ptr<Node> = rhs;

        let result = match &operator[..] {
            "&&" => Node::new_and_asgn(recv, value, operator_l, expression_l),
            "||" => Node::new_or_asgn(recv, value, operator_l, expression_l),
            _ => Node::new_op_asgn(
                recv,
                StringPtr::from(operator),
                value,
                operator_l,
                expression_l,
            ),
        };

        Ok(Ptr::new(result))
    }

    pub(crate) fn multi_lhs(
        &self,
        begin_t: Maybe<Ptr<Token>>,
        items: Box<List<Node>>,
        end_t: Maybe<Ptr<Token>>,
    ) -> Ptr<Node> {
        let CollectionMap {
            begin_l,
            end_l,
            expression_l,
        } = self.collection_map(&begin_t, &items, &end_t);

        Ptr::new(Node::new_mlhs(*items, begin_l, end_l, expression_l))
    }

    pub(crate) fn multi_assign(
        &self,
        lhs: Ptr<Node>,
        eql_t: Ptr<Token>,
        rhs: Ptr<Node>,
    ) -> Ptr<Node> {
        let operator_l = self.loc(&eql_t);
        let expression_l = join_exprs(&lhs, &rhs);

        Ptr::new(Node::new_masgn(lhs, rhs, operator_l, expression_l))
    }

    //
    // Class and module definition
    //

    pub(crate) fn def_class(
        &self,
        class_t: Ptr<Token>,
        name: Ptr<Node>,
        lt_t: Maybe<Ptr<Token>>,
        superclass: Maybe<Ptr<Node>>,
        body: Maybe<Ptr<Node>>,
        end_t: Ptr<Token>,
    ) -> Ptr<Node> {
        let keyword_l = self.loc(&class_t);
        let end_l = self.loc(&end_t);
        let operator_l = self.maybe_loc(&lt_t);
        let expression_l = keyword_l.join(&end_l);

        Ptr::new(Node::new_class(
            name,
            superclass,
            body,
            keyword_l,
            operator_l,
            end_l,
            expression_l,
        ))
    }

    pub(crate) fn def_sclass(
        &self,
        class_t: Ptr<Token>,
        lshift_t: Ptr<Token>,
        expr: Ptr<Node>,
        body: Maybe<Ptr<Node>>,
        end_t: Ptr<Token>,
    ) -> Ptr<Node> {
        let keyword_l = self.loc(&class_t);
        let end_l = self.loc(&end_t);
        let operator_l = self.loc(&lshift_t);
        let expression_l = keyword_l.join(&end_l);

        Ptr::new(Node::new_s_class(
            expr,
            body,
            keyword_l,
            operator_l,
            end_l,
            expression_l,
        ))
    }

    pub(crate) fn def_module(
        &self,
        module_t: Ptr<Token>,
        name: Ptr<Node>,
        body: Maybe<Ptr<Node>>,
        end_t: Ptr<Token>,
    ) -> Ptr<Node> {
        let keyword_l = self.loc(&module_t);
        let end_l = self.loc(&end_t);
        let expression_l = keyword_l.join(&end_l);

        Ptr::new(Node::new_module(name, body, keyword_l, end_l, expression_l))
    }

    //
    // Method (un)definition
    //

    pub(crate) fn def_method(
        &self,
        def_t: Ptr<Token>,
        name_t: Ptr<Token>,
        args: Maybe<Ptr<Node>>,
        body: Maybe<Ptr<Node>>,
        end_t: Ptr<Token>,
    ) -> Result<Ptr<Node>, ()> {
        let name_l = self.loc(&name_t);
        let keyword_l = self.loc(&def_t);
        let end_l = self.loc(&end_t);
        let expression_l = keyword_l.join(&end_l);

        let name = value(name_t);
        self.check_reserved_for_numparam(name.as_str(), &name_l)?;

        Ok(Ptr::new(Node::new_def(
            name,
            args,
            body,
            keyword_l,
            name_l,
            Maybe::some(end_l),
            Maybe::none(),
            expression_l,
        )))
    }

    pub(crate) fn def_endless_method(
        &self,
        def_t: Ptr<Token>,
        name_t: Ptr<Token>,
        args: Maybe<Ptr<Node>>,
        assignment_t: Ptr<Token>,
        body: Maybe<Ptr<Node>>,
    ) -> Result<Ptr<Node>, ()> {
        let body_l = maybe_boxed_node_expr(&body)
            .unwrap_or_else(|| unreachable!("endless method always has a body"));

        let keyword_l = self.loc(&def_t);
        let expression_l = keyword_l.join(&body_l);
        let name_l = self.loc(&name_t);
        let assignment_l = self.loc(&assignment_t);

        let name = value(name_t);
        self.check_reserved_for_numparam(name.as_str(), &name_l)?;

        Ok(Ptr::new(Node::new_def(
            name,
            args,
            body,
            keyword_l,
            name_l,
            Maybe::none(),
            Maybe::some(assignment_l),
            expression_l,
        )))
    }

    pub(crate) fn def_singleton(
        &self,
        def_t: Ptr<Token>,
        definee: Ptr<Node>,
        dot_t: Ptr<Token>,
        name_t: Ptr<Token>,
        args: Maybe<Ptr<Node>>,
        body: Maybe<Ptr<Node>>,
        end_t: Ptr<Token>,
    ) -> Result<Ptr<Node>, ()> {
        let keyword_l = self.loc(&def_t);
        let operator_l = self.loc(&dot_t);
        let name_l = self.loc(&name_t);
        let end_l = self.loc(&end_t);
        let expression_l = keyword_l.join(&end_l);

        let name = value(name_t);
        self.check_reserved_for_numparam(name.as_str(), &name_l)?;

        Ok(Ptr::new(Node::new_defs(
            definee,
            name,
            args,
            body,
            keyword_l,
            operator_l,
            name_l,
            Maybe::none(),
            Maybe::some(end_l),
            expression_l,
        )))
    }

    pub(crate) fn def_endless_singleton(
        &self,
        def_t: Ptr<Token>,
        definee: Ptr<Node>,
        dot_t: Ptr<Token>,
        name_t: Ptr<Token>,
        args: Maybe<Ptr<Node>>,
        assignment_t: Ptr<Token>,
        body: Maybe<Ptr<Node>>,
    ) -> Result<Ptr<Node>, ()> {
        let body_l = maybe_boxed_node_expr(&body)
            .unwrap_or_else(|| unreachable!("endless method always has body"));

        let keyword_l = self.loc(&def_t);
        let operator_l = self.loc(&dot_t);
        let name_l = self.loc(&name_t);
        let assignment_l = self.loc(&assignment_t);
        let expression_l = keyword_l.join(&body_l);

        let name = value(name_t);
        self.check_reserved_for_numparam(name.as_str(), &name_l)?;

        Ok(Ptr::new(Node::new_defs(
            definee,
            name,
            args,
            body,
            keyword_l,
            operator_l,
            name_l,
            Maybe::some(assignment_l),
            Maybe::none(),
            expression_l,
        )))
    }

    pub(crate) fn undef_method(&self, undef_t: Ptr<Token>, names: Box<List<Node>>) -> Ptr<Node> {
        let keyword_l = self.loc(&undef_t);
        let expression_l = keyword_l.maybe_join(&collection_expr(&names));
        Ptr::new(Node::new_undef(*names, keyword_l, expression_l))
    }

    pub(crate) fn alias(&self, alias_t: Ptr<Token>, to: Ptr<Node>, from: Ptr<Node>) -> Ptr<Node> {
        let keyword_l = self.loc(&alias_t);
        let expression_l = keyword_l.join(from.expression());
        Ptr::new(Node::new_alias(to, from, keyword_l, expression_l))
    }

    //
    // Formal arguments
    //

    pub(crate) fn args(
        &self,
        begin_t: Maybe<Ptr<Token>>,
        args: Box<List<Node>>,
        end_t: Maybe<Ptr<Token>>,
    ) -> Maybe<Ptr<Node>> {
        self.check_duplicate_args(&args, &mut HashMap::new());

        if begin_t.is_none() && args.is_empty() && end_t.is_none() {
            return Maybe::none();
        }

        let CollectionMap {
            begin_l,
            end_l,
            expression_l,
        } = self.collection_map(&begin_t, &args, &end_t);

        Maybe::some(Ptr::new(Node::new_args(
            *args,
            expression_l,
            begin_l,
            end_l,
        )))
    }

    pub(crate) fn forward_only_args(
        &self,
        begin_t: Ptr<Token>,
        dots_t: Ptr<Token>,
        end_t: Ptr<Token>,
    ) -> Ptr<Node> {
        let args = list![self.forward_arg(dots_t).unptr()];
        let begin_l = self.loc(&begin_t);
        let end_l = self.loc(&end_t);
        let expression_l = begin_l.join(&end_l);
        Ptr::new(Node::new_args(
            args,
            expression_l,
            Maybe::some(begin_l),
            Maybe::some(end_l),
        ))
    }

    pub(crate) fn forward_arg(&self, dots_t: Ptr<Token>) -> Ptr<Node> {
        Ptr::new(Node::new_forward_arg(self.loc(&dots_t)))
    }

    pub(crate) fn arg(&self, name_t: Ptr<Token>) -> Result<Ptr<Node>, ()> {
        let name_l = self.loc(&name_t);
        let name = value(name_t);

        self.check_reserved_for_numparam(name.as_str(), &name_l)?;

        Ok(Ptr::new(Node::new_arg(name, name_l)))
    }

    pub(crate) fn optarg(
        &self,
        name_t: Ptr<Token>,
        eql_t: Ptr<Token>,
        default: Ptr<Node>,
    ) -> Result<Ptr<Node>, ()> {
        let operator_l = self.loc(&eql_t);
        let name_l = self.loc(&name_t);
        let expression_l = self.loc(&name_t).join(default.expression());

        let name = value(name_t);
        self.check_reserved_for_numparam(name.as_str(), &name_l)?;

        Ok(Ptr::new(Node::new_optarg(
            name,
            default,
            name_l,
            operator_l,
            expression_l,
        )))
    }

    pub(crate) fn restarg(
        &self,
        star_t: Ptr<Token>,
        name_t: Maybe<Ptr<Token>>,
    ) -> Result<Ptr<Node>, ()> {
        let (name, name_l) = if name_t.is_some() {
            let name_t = name_t.unwrap();
            let name_l = self.loc(&name_t);
            let name = value(name_t);
            self.check_reserved_for_numparam(name.as_str(), &name_l)?;
            (Maybe::some(name), Maybe::some(name_l))
        } else {
            (Maybe::none(), Maybe::none())
        };

        let operator_l = self.loc(&star_t);
        let expression_l = operator_l.maybe_join(&name_l);

        Ok(Ptr::new(Node::new_restarg(
            name,
            operator_l,
            name_l,
            expression_l,
        )))
    }

    pub(crate) fn kwarg(&self, name_t: Ptr<Token>) -> Result<Ptr<Node>, ()> {
        let name_l = self.loc(&name_t);
        let name = value(name_t);
        self.check_reserved_for_numparam(name.as_str(), &name_l)?;

        let expression_l = name_l;
        let name_l = expression_l.adjust_end(-1);

        Ok(Ptr::new(Node::new_kwarg(name, name_l, expression_l)))
    }

    pub(crate) fn kwoptarg(&self, name_t: Ptr<Token>, default: Ptr<Node>) -> Result<Ptr<Node>, ()> {
        let name_l = self.loc(&name_t);
        let name = value(name_t);
        self.check_reserved_for_numparam(name.as_str(), &name_l)?;

        let label_l = name_l;
        let name_l = label_l.adjust_end(-1);
        let expression_l = default.expression().join(&label_l);

        Ok(Ptr::new(Node::new_kwoptarg(
            name,
            default,
            name_l,
            expression_l,
        )))
    }

    pub(crate) fn kwrestarg(
        &self,
        dstar_t: Ptr<Token>,
        name_t: Maybe<Ptr<Token>>,
    ) -> Result<Ptr<Node>, ()> {
        let (name, name_l) = if name_t.is_some() {
            let name_t = name_t.unwrap();
            let name_l = self.loc(&name_t);
            let name = value(name_t);
            self.check_reserved_for_numparam(name.as_str(), &name_l)?;
            (Maybe::some(name), Maybe::some(name_l))
        } else {
            (Maybe::none(), Maybe::none())
        };

        let operator_l = self.loc(&dstar_t);
        let expression_l = operator_l.maybe_join(&name_l);

        Ok(Ptr::new(Node::new_kwrestarg(
            name,
            operator_l,
            name_l,
            expression_l,
        )))
    }

    pub(crate) fn kwnilarg(&self, dstar_t: Ptr<Token>, nil_t: Ptr<Token>) -> Ptr<Node> {
        let dstar_l = self.loc(&dstar_t);
        let nil_l = self.loc(&nil_t);
        let expression_l = dstar_l.join(&nil_l);
        Ptr::new(Node::new_kwnilarg(nil_l, expression_l))
    }

    pub(crate) fn shadowarg(&self, name_t: Ptr<Token>) -> Result<Ptr<Node>, ()> {
        let name_l = self.loc(&name_t);
        let name = value(name_t);
        self.check_reserved_for_numparam(name.as_str(), &name_l)?;

        Ok(Ptr::new(Node::new_shadowarg(name, name_l)))
    }

    pub(crate) fn blockarg(
        &self,
        amper_t: Ptr<Token>,
        name_t: Ptr<Token>,
    ) -> Result<Ptr<Node>, ()> {
        let name_l = self.loc(&name_t);
        let name = value(name_t);
        self.check_reserved_for_numparam(name.as_str(), &name_l)?;

        let operator_l = self.loc(&amper_t);
        let expression_l = operator_l.join(&name_l);

        Ok(Ptr::new(Node::new_blockarg(
            name,
            operator_l,
            name_l,
            expression_l,
        )))
    }

    pub(crate) fn procarg0(&self, arg: Ptr<Node>) -> Ptr<Node> {
        if arg.is_mlhs() {
            let internal::Mlhs {
                items,
                begin_l,
                end_l,
                expression_l,
            } = arg.unptr().into_mlhs().into_internal();
            Ptr::new(Node::new_procarg0(items, begin_l, end_l, expression_l))
        } else if arg.is_arg() {
            let expression_l = arg.expression().clone();
            Ptr::new(Node::new_procarg0(
                list![arg.unptr()],
                Maybe::none(),
                Maybe::none(),
                expression_l,
            ))
        } else {
            unreachable!("unsupported procarg0 child {:?}", arg)
        }
    }

    //
    // Method calls
    //

    fn call_type_for_dot(&self, dot_t: &Maybe<Ptr<Token>>) -> MethodCallType {
        match dot_t.as_ref() {
            Some(token) if token.token_type() == Lexer::tANDDOT => MethodCallType::CSend,
            _ => MethodCallType::Send,
        }
    }

    pub(crate) fn forwarded_args(&self, dots_t: Ptr<Token>) -> Ptr<Node> {
        Ptr::new(Node::new_forwarded_args(self.loc(&dots_t)))
    }

    pub(crate) fn call_method(
        &self,
        receiver: Maybe<Ptr<Node>>,
        dot_t: Maybe<Ptr<Token>>,
        selector_t: Maybe<Ptr<Token>>,
        lparen_t: Maybe<Ptr<Token>>,
        mut args: Box<List<Node>>,
        rparen_t: Maybe<Ptr<Token>>,
    ) -> Ptr<Node> {
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
            StringPtr::from("call")
        };

        self.rewrite_hash_args_to_kwargs(&mut args);

        match self.call_type_for_dot(&dot_t) {
            MethodCallType::Send => Ptr::new(Node::new_send(
                receiver,
                method_name,
                *args,
                dot_l,
                selector_l,
                begin_l,
                end_l,
                Maybe::none(),
                expression_l,
            )),

            MethodCallType::CSend => Ptr::new(Node::new_c_send(
                receiver.expect("csend node must have a receiver"),
                method_name,
                *args,
                dot_l.expect("csend node must have &."),
                selector_l,
                begin_l,
                end_l,
                Maybe::none(),
                expression_l,
            )),
        }
    }

    pub(crate) fn call_lambda(&self, lambda_t: Ptr<Token>) -> Ptr<Node> {
        Ptr::new(Node::new_lambda(self.loc(&lambda_t)))
    }

    pub(crate) fn block(
        &self,
        method_call: Ptr<Node>,
        begin_t: Ptr<Token>,
        block_args: ArgsType,
        body: Maybe<Ptr<Node>>,
        end_t: Ptr<Token>,
    ) -> Result<Ptr<Node>, ()> {
        let block_body = body;

        let validate_block_and_block_arg = |args: &List<Node>| {
            if let Some(last_arg) = args.last() {
                if last_arg.is_block_pass() || last_arg.is_forwarded_args() {
                    self.error(
                        DiagnosticMessage::new_block_and_block_arg_given(),
                        last_arg.expression(),
                    );
                    Err(())
                } else {
                    Ok(())
                }
            } else {
                Ok(())
            }
        };

        if let Some(yield_) = method_call.as_yield() {
            let keyword_l = yield_.get_keyword_l();
            self.error(DiagnosticMessage::new_block_given_to_yield(), keyword_l);
            return Err(());
        } else if let Some(send) = method_call.as_send() {
            validate_block_and_block_arg(send.get_args())?;
        } else if let Some(c_send) = method_call.as_c_send() {
            validate_block_and_block_arg(c_send.get_args())?;
        }

        let rewrite_args_and_loc =
            |method_args: &List<Node>,
             keyword_expression_l: &Loc,
             block_args: ArgsType,
             block_body: Maybe<Ptr<Node>>| {
                // Code like "return foo 1 do end" is reduced in a weird sequence.
                // Here, method_call is actually (return).
                let actual_send = method_args[0].clone();

                let begin_l = self.loc(&begin_t);
                let end_l = self.loc(&end_t);
                let expression_l = actual_send.expression().join(&end_l);

                let block = match block_args {
                    ArgsType::Args(args) => Node::new_block(
                        Ptr::new(actual_send),
                        args,
                        block_body,
                        begin_l,
                        end_l,
                        expression_l,
                    ),
                    ArgsType::Numargs(numargs) => Node::new_numblock(
                        Ptr::new(actual_send),
                        numargs,
                        block_body.expect("numblock always has body"),
                        begin_l,
                        end_l,
                        expression_l,
                    ),
                };

                let expr_l = keyword_expression_l.join(block.expression());

                (list![block], expr_l)
            };

        if method_call.is_send()
            || method_call.is_c_send()
            || method_call.is_index()
            || method_call.is_super()
            || method_call.is_z_super()
            || method_call.is_lambda()
        {
            let begin_l = self.loc(&begin_t);
            let end_l = self.loc(&end_t);
            let expression_l = method_call.expression().join(&end_l);

            let result = match block_args {
                ArgsType::Args(args) => {
                    Node::new_block(method_call, args, block_body, begin_l, end_l, expression_l)
                }
                ArgsType::Numargs(numargs) => Node::new_numblock(
                    method_call,
                    numargs,
                    {
                        let block_body: Maybe<Ptr<Node>> = block_body;
                        block_body.expect("numblock always has body")
                    },
                    begin_l,
                    end_l,
                    expression_l,
                ),
            };
            return Ok(Ptr::new(result));
        };

        let method_call = method_call.unptr();
        let result = if method_call.is_return() {
            let return_ = method_call.into_return();
            let args = return_.get_args();
            let keyword_l = return_.get_keyword_l().to_owned();
            let expression_l = return_.get_expression_l();

            let (args, expression_l) =
                rewrite_args_and_loc(args, expression_l, block_args, block_body);
            Node::new_return(args, keyword_l, expression_l)
        } else if method_call.is_next() {
            let next = method_call.into_next();
            let args = next.get_args();
            let keyword_l = next.get_keyword_l().to_owned();
            let expression_l = next.get_expression_l();

            let (args, expression_l) =
                rewrite_args_and_loc(args, expression_l, block_args, block_body);
            Node::new_next(args, keyword_l, expression_l)
        } else if method_call.is_break() {
            let break_ = method_call.into_break();
            let args = break_.get_args();
            let keyword_l = break_.get_keyword_l().to_owned();
            let expression_l = break_.get_expression_l();

            let (args, expression_l) =
                rewrite_args_and_loc(args, expression_l, block_args, block_body);
            Node::new_break(args, keyword_l, expression_l)
        } else {
            unreachable!("unsupported method call {:?}", method_call)
        };

        Ok(Ptr::new(result))
    }
    pub(crate) fn block_pass(&self, amper_t: Ptr<Token>, value: Ptr<Node>) -> Ptr<Node> {
        let amper_l = self.loc(&amper_t);
        let expression_l = value.expression().join(&amper_l);

        Ptr::new(Node::new_block_pass(value, amper_l, expression_l))
    }

    pub(crate) fn attr_asgn(
        &self,
        receiver: Ptr<Node>,
        dot_t: Ptr<Token>,
        selector_t: Ptr<Token>,
    ) -> Ptr<Node> {
        let dot_l = self.loc(&dot_t);
        let selector_l = self.loc(&selector_t);
        let expression_l = receiver.expression().join(&selector_l);
        let receiver: Ptr<Node> = receiver;

        let method_name = StringPtr::from(String::from(value(selector_t)) + "=");

        match self.call_type_for_dot(&Maybe::some(dot_t)) {
            MethodCallType::Send => Ptr::new(Node::new_send(
                Maybe::some(receiver),
                method_name,
                list![],
                Maybe::some(dot_l),
                Maybe::some(selector_l),
                Maybe::none(),
                Maybe::none(),
                Maybe::none(),
                expression_l,
            )),

            MethodCallType::CSend => Ptr::new(Node::new_c_send(
                receiver,
                method_name,
                list![],
                dot_l,
                Maybe::some(selector_l),
                Maybe::none(),
                Maybe::none(),
                Maybe::none(),
                expression_l,
            )),
        }
    }

    pub(crate) fn index(
        &self,
        recv: Ptr<Node>,
        lbrack_t: Ptr<Token>,
        mut indexes: Box<List<Node>>,
        rbrack_t: Ptr<Token>,
    ) -> Ptr<Node> {
        let begin_l = self.loc(&lbrack_t);
        let end_l = self.loc(&rbrack_t);
        let expression_l = recv.expression().join(&end_l);

        self.rewrite_hash_args_to_kwargs(&mut indexes);

        Ptr::new(Node::new_index(
            recv,
            *indexes,
            begin_l,
            end_l,
            expression_l,
        ))
    }

    pub(crate) fn index_asgn(
        &self,
        recv: Ptr<Node>,
        lbrack_t: Ptr<Token>,
        indexes: Box<List<Node>>,
        rbrack_t: Ptr<Token>,
    ) -> Ptr<Node> {
        let begin_l = self.loc(&lbrack_t);
        let end_l = self.loc(&rbrack_t);
        let expression_l = recv.expression().join(&end_l);

        Ptr::new(Node::new_index_asgn(
            recv,
            *indexes,
            Maybe::none(),
            begin_l,
            end_l,
            Maybe::none(),
            expression_l,
        ))
    }

    pub(crate) fn binary_op(
        &self,
        receiver: Ptr<Node>,
        operator_t: Ptr<Token>,
        arg: Ptr<Node>,
    ) -> Result<Ptr<Node>, ()> {
        self.value_expr(&receiver)?;
        self.value_expr(&arg)?;

        let selector_l = Maybe::some(self.loc(&operator_t));
        let expression_l = join_exprs(&receiver, &arg);

        Ok(Ptr::new(Node::new_send(
            Maybe::some(receiver),
            value(operator_t),
            list![arg.unptr()],
            Maybe::none(),
            selector_l,
            Maybe::none(),
            Maybe::none(),
            Maybe::none(),
            expression_l,
        )))
    }

    pub(crate) fn match_op(
        &self,
        receiver: Ptr<Node>,
        match_t: Ptr<Token>,
        arg: Ptr<Node>,
    ) -> Result<Ptr<Node>, ()> {
        self.value_expr(&receiver)?;
        self.value_expr(&arg)?;

        let selector_l = self.loc(&match_t);
        let expression_l = join_exprs(&receiver, &arg);

        let result = match self.static_regexp_captures(&receiver) {
            Some(captures) => {
                for capture in captures {
                    self.static_env.declare(&capture);
                }

                Node::new_match_with_lvasgn(receiver, arg, selector_l, expression_l)
            }
            None => Node::new_send(
                Maybe::some(receiver),
                StringPtr::from("=~"),
                list![arg.unptr()],
                Maybe::none(),
                Maybe::some(selector_l),
                Maybe::none(),
                Maybe::none(),
                Maybe::none(),
                expression_l,
            ),
        };

        Ok(Ptr::new(result))
    }

    pub(crate) fn unary_op(&self, op_t: Ptr<Token>, receiver: Ptr<Node>) -> Result<Ptr<Node>, ()> {
        self.value_expr(&receiver)?;

        let selector_l = self.loc(&op_t);
        let expression_l = receiver.expression().join(&selector_l);

        let op = String::from(value(op_t));
        let method_name = if op == "+" || op == "-" { op + "@" } else { op };
        Ok(Ptr::new(Node::new_send(
            Maybe::some(receiver),
            StringPtr::from(method_name),
            list![],
            Maybe::none(),
            Maybe::some(selector_l),
            Maybe::none(),
            Maybe::none(),
            Maybe::none(),
            expression_l,
        )))
    }

    pub(crate) fn not_op(
        &self,
        not_t: Ptr<Token>,
        begin_t: Maybe<Ptr<Token>>,
        receiver: Maybe<Ptr<Node>>,
        end_t: Maybe<Ptr<Token>>,
    ) -> Result<Ptr<Node>, ()> {
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

            Ok(Ptr::new(Node::new_send(
                Maybe::some(self.check_condition(receiver)),
                StringPtr::from("!"),
                list![],
                Maybe::none(),
                Maybe::some(selector_l),
                begin_l,
                end_l,
                Maybe::none(),
                expression_l,
            )))
        } else {
            let CollectionMap {
                begin_l,
                end_l,
                expression_l,
            } = self.collection_map(&begin_t, &[], &end_t);

            let nil_node = Ptr::new(Node::new_begin(list![], begin_l, end_l, expression_l));

            let selector_l = self.loc(&not_t);
            let expression_l = nil_node.expression().join(&selector_l);
            Ok(Ptr::new(Node::new_send(
                Maybe::some(nil_node),
                StringPtr::from("!"),
                list![],
                Maybe::none(),
                Maybe::some(selector_l),
                Maybe::none(),
                Maybe::none(),
                Maybe::none(),
                expression_l,
            )))
        }
    }

    //
    // Control flow
    //

    // Logical operations: and, or

    pub(crate) fn logical_op(
        &self,
        type_: LogicalOp,
        lhs: Ptr<Node>,
        op_t: Ptr<Token>,
        rhs: Ptr<Node>,
    ) -> Result<Ptr<Node>, ()> {
        self.value_expr(&lhs)?;

        let operator_l = self.loc(&op_t);
        let expression_l = join_exprs(&lhs, &rhs);
        let lhs: Ptr<Node> = lhs;
        let rhs: Ptr<Node> = rhs;

        let result = match type_ {
            LogicalOp::And => Node::new_and(lhs, rhs, operator_l, expression_l),
            LogicalOp::Or => Node::new_or(lhs, rhs, operator_l, expression_l),
        };
        Ok(Ptr::new(result))
    }

    // Conditionals

    pub(crate) fn condition(
        &self,
        cond_t: Ptr<Token>,
        cond: Ptr<Node>,
        then_t: Ptr<Token>,
        if_true: Maybe<Ptr<Node>>,
        else_t: Maybe<Ptr<Token>>,
        if_false: Maybe<Ptr<Node>>,
        end_t: Maybe<Ptr<Token>>,
    ) -> Ptr<Node> {
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

        Ptr::new(Node::new_if(
            self.check_condition(cond),
            if_true,
            if_false,
            keyword_l,
            begin_l,
            else_l,
            end_l,
            expression_l,
        ))
    }

    pub(crate) fn condition_mod(
        &self,
        if_true: Maybe<Ptr<Node>>,
        if_false: Maybe<Ptr<Node>>,
        cond_t: Ptr<Token>,
        cond: Ptr<Node>,
    ) -> Ptr<Node> {
        let pre = match (if_true.as_ref(), if_false.as_ref()) {
            (None, None) => unreachable!("at least one of if_true/if_false is required"),
            (None, Some(if_false)) => if_false,
            (Some(if_true), None) => if_true,
            (Some(_), Some(_)) => unreachable!("only one of if_true/if_false is required"),
        };

        let expression_l = pre.expression().join(cond.expression());
        let keyword_l = self.loc(&cond_t);

        Ptr::new(Node::new_if_mod(
            self.check_condition(cond),
            if_true,
            if_false,
            keyword_l,
            expression_l,
        ))
    }

    pub(crate) fn ternary(
        &self,
        cond: Ptr<Node>,
        question_t: Ptr<Token>,
        if_true: Ptr<Node>,
        colon_t: Ptr<Token>,
        if_false: Ptr<Node>,
    ) -> Ptr<Node> {
        let expression_l = join_exprs(&cond, &if_false);
        let question_l = self.loc(&question_t);
        let colon_l = self.loc(&colon_t);

        Ptr::new(Node::new_if_ternary(
            cond,
            if_true,
            if_false,
            question_l,
            colon_l,
            expression_l,
        ))
    }

    // Case matching

    pub(crate) fn when(
        &self,
        when_t: Ptr<Token>,
        patterns: Box<List<Node>>,
        then_t: Ptr<Token>,
        body: Maybe<Ptr<Node>>,
    ) -> Ptr<Node> {
        let begin_l = self.loc(&then_t);

        let expr_end_l = maybe_boxed_node_expr(&body)
            .or_else(|| maybe_node_expr(&patterns.last()))
            .unwrap_or_else(|| self.loc(&when_t));
        let when_l = self.loc(&when_t);
        let expression_l = when_l.join(&expr_end_l);

        Ptr::new(Node::new_when(
            *patterns,
            body,
            when_l,
            begin_l,
            expression_l,
        ))
    }

    pub(crate) fn case(
        &self,
        case_t: Ptr<Token>,
        expr: Maybe<Ptr<Node>>,
        when_bodies: Box<List<Node>>,
        else_t: Maybe<Ptr<Token>>,
        else_body: Maybe<Ptr<Node>>,
        end_t: Ptr<Token>,
    ) -> Ptr<Node> {
        let keyword_l = self.loc(&case_t);
        let else_l = self.maybe_loc(&else_t);
        let end_l = self.loc(&end_t);
        let expression_l = keyword_l.join(&end_l);

        Ptr::new(Node::new_case(
            expr,
            *when_bodies,
            else_body,
            keyword_l,
            else_l,
            end_l,
            expression_l,
        ))
    }

    // Loops

    pub(crate) fn loop_(
        &self,
        loop_type: LoopType,
        keyword_t: Ptr<Token>,
        cond: Ptr<Node>,
        do_t: Ptr<Token>,
        body: Maybe<Ptr<Node>>,
        end_t: Ptr<Token>,
    ) -> Ptr<Node> {
        let keyword_l = self.loc(&keyword_t);
        let begin_l = self.loc(&do_t);
        let end_l = self.loc(&end_t);
        let expression_l = self.loc(&keyword_t).join(&end_l);

        let cond = self.check_condition(cond);

        match loop_type {
            LoopType::While => Ptr::new(Node::new_while(
                cond,
                body,
                keyword_l,
                Maybe::some(begin_l),
                Maybe::some(end_l),
                expression_l,
            )),
            LoopType::Until => Ptr::new(Node::new_until(
                cond,
                body,
                keyword_l,
                Maybe::some(begin_l),
                Maybe::some(end_l),
                expression_l,
            )),
        }
    }

    pub(crate) fn loop_mod(
        &self,
        loop_type: LoopType,
        body: Ptr<Node>,
        keyword_t: Ptr<Token>,
        cond: Ptr<Node>,
    ) -> Ptr<Node> {
        let expression_l = body.expression().join(cond.expression());
        let keyword_l = self.loc(&keyword_t);

        let cond = self.check_condition(cond);

        match (loop_type, &*body) {
            (LoopType::While, node) if node.is_kw_begin() => {
                Ptr::new(Node::new_while_post(cond, body, keyword_l, expression_l))
            }
            (LoopType::While, _) => Ptr::new(Node::new_while(
                cond,
                Maybe::some(body),
                keyword_l,
                Maybe::none(),
                Maybe::none(),
                expression_l,
            )),
            (LoopType::Until, node) if node.is_kw_begin() => {
                Ptr::new(Node::new_until_post(cond, body, keyword_l, expression_l))
            }
            (LoopType::Until, _) => Ptr::new(Node::new_until(
                cond,
                Maybe::some(body),
                keyword_l,
                Maybe::none(),
                Maybe::none(),
                expression_l,
            )),
        }
    }

    pub(crate) fn for_(
        &self,
        for_t: Ptr<Token>,
        iterator: Ptr<Node>,
        in_t: Ptr<Token>,
        iteratee: Ptr<Node>,
        do_t: Ptr<Token>,
        body: Maybe<Ptr<Node>>,
        end_t: Ptr<Token>,
    ) -> Ptr<Node> {
        let keyword_l = self.loc(&for_t);
        let operator_l = self.loc(&in_t);
        let begin_l = self.loc(&do_t);
        let end_l = self.loc(&end_t);
        let expression_l = keyword_l.join(&end_l);

        Ptr::new(Node::new_for(
            iterator,
            iteratee,
            body,
            keyword_l,
            operator_l,
            begin_l,
            end_l,
            expression_l,
        ))
    }

    // Keywords

    pub(crate) fn keyword_cmd(
        &self,
        type_: KeywordCmd,
        keyword_t: Ptr<Token>,
        lparen_t: Maybe<Ptr<Token>>,
        mut args: Box<List<Node>>,
        rparen_t: Maybe<Ptr<Token>>,
    ) -> Result<Ptr<Node>, ()> {
        let keyword_l = self.loc(&keyword_t);

        if type_ == KeywordCmd::Yield && !args.is_empty() {
            if let Some(last_arg) = args.last() {
                if last_arg.is_block_pass() {
                    self.error(DiagnosticMessage::new_block_given_to_yield(), &keyword_l);
                    return Err(());
                }
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
            KeywordCmd::Break => Node::new_break(*args, keyword_l, expression_l),
            KeywordCmd::Defined => Node::new_defined(
                Ptr::new(args.take_first()),
                keyword_l,
                begin_l,
                end_l,
                expression_l,
            ),
            KeywordCmd::Next => Node::new_next(*args, keyword_l, expression_l),
            KeywordCmd::Redo => Node::new_redo(expression_l),
            KeywordCmd::Retry => Node::new_retry(expression_l),
            KeywordCmd::Return => Node::new_return(*args, keyword_l, expression_l),
            KeywordCmd::Super => Node::new_super(*args, keyword_l, begin_l, end_l, expression_l),
            KeywordCmd::Yield => Node::new_yield(*args, keyword_l, begin_l, end_l, expression_l),
            KeywordCmd::Zsuper => Node::new_z_super(expression_l),
        };

        Ok(Ptr::new(result))
    }

    // BEGIN, END

    pub(crate) fn preexe(
        &self,
        preexe_t: Ptr<Token>,
        lbrace_t: Ptr<Token>,
        body: Maybe<Ptr<Node>>,
        rbrace_t: Ptr<Token>,
    ) -> Ptr<Node> {
        let keyword_l = self.loc(&preexe_t);
        let begin_l = self.loc(&lbrace_t);
        let end_l = self.loc(&rbrace_t);
        let expression_l = keyword_l.join(&end_l);

        Ptr::new(Node::new_preexe(
            body,
            keyword_l,
            begin_l,
            end_l,
            expression_l,
        ))
    }
    pub(crate) fn postexe(
        &self,
        postexe_t: Ptr<Token>,
        lbrace_t: Ptr<Token>,
        body: Maybe<Ptr<Node>>,
        rbrace_t: Ptr<Token>,
    ) -> Ptr<Node> {
        let keyword_l = self.loc(&postexe_t);
        let begin_l = self.loc(&lbrace_t);
        let end_l = self.loc(&rbrace_t);
        let expression_l = keyword_l.join(&end_l);

        Ptr::new(Node::new_postexe(
            body,
            keyword_l,
            begin_l,
            end_l,
            expression_l,
        ))
    }

    // Exception handling

    pub(crate) fn rescue_body(
        &self,
        rescue_t: Ptr<Token>,
        exc_list: Maybe<Ptr<Node>>,
        assoc_t: Maybe<Ptr<Token>>,
        exc_var: Maybe<Ptr<Node>>,
        then_t: Maybe<Ptr<Token>>,
        body: Maybe<Ptr<Node>>,
    ) -> Ptr<Node> {
        let end_l = maybe_boxed_node_expr(&body)
            .or_else(|| self.maybe_loc(&then_t))
            .or_else(|| maybe_boxed_node_expr(&exc_var))
            .or_else(|| maybe_boxed_node_expr(&exc_list))
            .unwrap_or_else(|| self.loc(&rescue_t));

        let expression_l = self.loc(&rescue_t).join(&end_l);
        let keyword_l = self.loc(&rescue_t);
        let assoc_l = self.maybe_loc(&assoc_t);
        let begin_l = self.maybe_loc(&then_t);

        Ptr::new(Node::new_rescue_body(
            exc_list,
            exc_var,
            body,
            keyword_l,
            assoc_l,
            begin_l,
            expression_l,
        ))
    }

    pub(crate) fn begin_body(
        &self,
        compound_stmt: Maybe<Ptr<Node>>,
        rescue_bodies: Box<List<Node>>,
        else_: Option<(Ptr<Token>, Maybe<Ptr<Node>>)>,
        ensure: Option<(Ptr<Token>, Maybe<Ptr<Node>>)>,
    ) -> Maybe<Ptr<Node>> {
        let mut result: Maybe<Ptr<Node>>;

        if !rescue_bodies.is_empty() {
            if let Some((else_t, else_)) = else_ {
                let begin_l = maybe_boxed_node_expr(&compound_stmt)
                    .or_else(|| maybe_node_expr(&rescue_bodies.first()))
                    .unwrap_or_else(|| unreachable!("can't compute begin_l"));

                let end_l = maybe_boxed_node_expr(&else_).unwrap_or_else(|| self.loc(&else_t));

                let expression_l = begin_l.join(&end_l);
                let else_l = self.loc(&else_t);

                result = Maybe::some(Ptr::new(Node::new_rescue(
                    compound_stmt,
                    *rescue_bodies,
                    else_,
                    Maybe::some(else_l),
                    expression_l,
                )))
            } else {
                let begin_l = maybe_boxed_node_expr(&compound_stmt)
                    .or_else(|| maybe_node_expr(&rescue_bodies.first()))
                    .unwrap_or_else(|| unreachable!("can't compute begin_l"));

                let end_l = maybe_node_expr(&rescue_bodies.last())
                    .unwrap_or_else(|| unreachable!("can't compute end_l"));

                let expression_l = begin_l.join(&end_l);
                let else_l = self.maybe_loc(&Maybe::none());

                result = Maybe::some(Ptr::new(Node::new_rescue(
                    compound_stmt,
                    *rescue_bodies,
                    Maybe::none(),
                    else_l,
                    expression_l,
                )))
            }
        } else if let Some((else_t, else_)) = else_ {
            let mut statements = list![];

            // let compound_stmt = compound_stmt.map(|boxed| *boxed);
            if compound_stmt.is_some() {
                let compound_stmt = compound_stmt.unwrap().unptr();
                if compound_stmt.is_begin() {
                    let internal::Begin {
                        statements: stmts, ..
                    } = compound_stmt.into_begin().into_internal();
                    statements = stmts;
                } else {
                    statements.push(compound_stmt)
                }
            }

            let parts = if else_.is_some() {
                list![else_.unwrap().unptr()]
            } else {
                list![]
            };
            let CollectionMap {
                begin_l,
                end_l,
                expression_l,
            } = self.collection_map(&Maybe::some(else_t), &parts, &Maybe::none());

            statements.push(Node::new_begin(parts, begin_l, end_l, expression_l));

            let CollectionMap {
                begin_l,
                end_l,
                expression_l,
            } = self.collection_map(&Maybe::none(), &statements, &Maybe::none());

            result = Maybe::some(Ptr::new(Node::new_begin(
                statements,
                begin_l,
                end_l,
                expression_l,
            )))
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

            result = Maybe::some(Ptr::new(Node::new_ensure(
                result,
                ensure_body,
                keyword_l,
                expression_l,
            )))
        }

        result
    }

    //
    // Expression grouping
    //

    pub(crate) fn compstmt(&self, statements: Box<List<Node>>) -> Maybe<Ptr<Node>> {
        match &statements[..] {
            [] => Maybe::none(),
            [_] => Maybe::some(Ptr::new(statements.take_first())),
            _ => {
                let CollectionMap {
                    begin_l,
                    end_l,
                    expression_l,
                } = self.collection_map(&Maybe::none(), &statements, &Maybe::none());

                Maybe::some(Ptr::new(Node::new_begin(
                    *statements,
                    begin_l,
                    end_l,
                    expression_l,
                )))
            }
        }
    }

    pub(crate) fn begin(
        &self,
        begin_t: Ptr<Token>,
        body: Maybe<Ptr<Node>>,
        end_t: Ptr<Token>,
    ) -> Ptr<Node> {
        let new_begin_l = self.loc(&begin_t);
        let new_end_l = self.loc(&end_t);
        let new_expression_l = new_begin_l.join(&new_end_l);

        let new_begin_l = Maybe::some(new_begin_l);
        let new_end_l = Maybe::some(new_end_l);

        if body.is_some() {
            let mut body = body.unwrap().unptr();
            if let Some(mlhs) = body.as_mlhs_mut() {
                // Synthesized (begin) from compstmt "a; b" or (mlhs)
                // from multi_lhs "(a, b) = *foo".
                mlhs.set_begin_l(new_begin_l);
                mlhs.set_end_l(new_end_l);
                mlhs.set_expression_l(new_expression_l);
                Ptr::new(body)
            } else if body.is_begin()
                && body.as_begin().unwrap().get_begin_l().is_none()
                && body.as_begin().unwrap().get_end_l().is_none()
            {
                let begin = body.as_begin_mut().unwrap();
                begin.set_begin_l(new_begin_l);
                begin.set_end_l(new_end_l);
                begin.set_expression_l(new_expression_l);
                Ptr::new(body)
            } else {
                let mut statements = list![];
                statements.push(body);
                Ptr::new(Node::new_begin(
                    statements,
                    new_begin_l,
                    new_end_l,
                    new_expression_l,
                ))
            }
        } else {
            // A nil expression: `()'.
            Ptr::new(Node::new_begin(
                list![],
                new_begin_l,
                new_end_l,
                new_expression_l,
            ))
        }
    }

    pub(crate) fn begin_keyword(
        &self,
        begin_t: Ptr<Token>,
        body: Maybe<Ptr<Node>>,
        end_t: Ptr<Token>,
    ) -> Ptr<Node> {
        let begin_l = self.loc(&begin_t);
        let end_l = self.loc(&end_t);
        let expression_l = begin_l.join(&end_l);

        let begin_l = Maybe::some(begin_l);
        let end_l = Maybe::some(end_l);

        if body.is_none() {
            // A nil expression: `begin end'.
            Ptr::new(Node::new_kw_begin(list![], begin_l, end_l, expression_l))
        } else {
            let body = body.unwrap().unptr();
            if body.is_begin() {
                // Synthesized (begin) from compstmt "a; b".
                let internal::Begin { statements, .. } = body.into_begin().into_internal();
                Ptr::new(Node::new_kw_begin(statements, begin_l, end_l, expression_l))
            } else {
                let mut statements = list![];
                statements.push(body);
                Ptr::new(Node::new_kw_begin(statements, begin_l, end_l, expression_l))
            }
        }
    }

    //
    // Pattern matching
    //

    pub(crate) fn case_match(
        &self,
        case_t: Ptr<Token>,
        expr: Ptr<Node>,
        in_bodies: Box<List<Node>>,
        else_t: Maybe<Ptr<Token>>,
        else_body: Maybe<Ptr<Node>>,
        end_t: Ptr<Token>,
    ) -> Ptr<Node> {
        let else_body = match (else_t.as_ref(), else_body.as_ref()) {
            (Some(else_t), None) => Maybe::some(Ptr::new(Node::new_empty_else(self.loc(else_t)))),
            _ => else_body,
        };

        let keyword_l = self.loc(&case_t);
        let else_l = self.maybe_loc(&else_t);
        let end_l = self.loc(&end_t);
        let expression_l = self.loc(&case_t).join(&end_l);

        Ptr::new(Node::new_case_match(
            expr,
            *in_bodies,
            else_body,
            keyword_l,
            else_l,
            end_l,
            expression_l,
        ))
    }

    pub(crate) fn match_pattern(
        &self,
        value: Ptr<Node>,
        assoc_t: Ptr<Token>,
        pattern: Ptr<Node>,
    ) -> Ptr<Node> {
        let operator_l = self.loc(&assoc_t);
        let expression_l = join_exprs(&value, &pattern);

        Ptr::new(Node::new_match_pattern(
            value,
            pattern,
            operator_l,
            expression_l,
        ))
    }

    pub(crate) fn match_pattern_p(
        &self,
        value: Ptr<Node>,
        in_t: Ptr<Token>,
        pattern: Ptr<Node>,
    ) -> Ptr<Node> {
        let operator_l = self.loc(&in_t);
        let expression_l = join_exprs(&value, &pattern);

        Ptr::new(Node::new_match_pattern_p(
            value,
            pattern,
            operator_l,
            expression_l,
        ))
    }

    pub(crate) fn in_pattern(
        &self,
        in_t: Ptr<Token>,
        pattern: Ptr<Node>,
        guard: Maybe<Ptr<Node>>,
        then_t: Ptr<Token>,
        body: Maybe<Ptr<Node>>,
    ) -> Ptr<Node> {
        let keyword_l = self.loc(&in_t);
        let begin_l = self.loc(&then_t);

        let expression_l = maybe_boxed_node_expr(&body)
            .or_else(|| maybe_boxed_node_expr(&guard))
            .unwrap_or_else(|| pattern.expression().clone())
            .join(&keyword_l);

        Ptr::new(Node::new_in_pattern(
            pattern,
            guard,
            body,
            keyword_l,
            begin_l,
            expression_l,
        ))
    }

    pub(crate) fn if_guard(&self, if_t: Ptr<Token>, cond: Ptr<Node>) -> Ptr<Node> {
        let keyword_l = self.loc(&if_t);
        let expression_l = keyword_l.join(cond.expression());

        Ptr::new(Node::new_if_guard(cond, keyword_l, expression_l))
    }
    pub(crate) fn unless_guard(&self, unless_t: Ptr<Token>, cond: Ptr<Node>) -> Ptr<Node> {
        let keyword_l = self.loc(&unless_t);
        let expression_l = keyword_l.join(cond.expression());

        Ptr::new(Node::new_unless_guard(cond, keyword_l, expression_l))
    }

    pub(crate) fn match_var(&self, name_t: Ptr<Token>) -> Result<Ptr<Node>, ()> {
        let name_l = self.loc(&name_t);
        let expression_l = name_l.clone();
        let name = value(name_t);

        self.check_lvar_name(name.as_str(), &name_l)?;
        self.check_duplicate_pattern_variable(name.as_str(), &name_l)?;
        self.static_env.declare(name.as_str());

        Ok(Ptr::new(Node::new_match_var(name, name_l, expression_l)))
    }

    pub(crate) fn match_hash_var(&self, name_t: Ptr<Token>) -> Result<Ptr<Node>, ()> {
        let expression_l = self.loc(&name_t);
        let name_l = expression_l.adjust_end(-1);

        let name = value(name_t);

        self.check_lvar_name(name.as_str(), &name_l)?;
        self.check_duplicate_pattern_variable(name.as_str(), &name_l)?;
        self.static_env.declare(name.as_str());

        Ok(Ptr::new(Node::new_match_var(name, name_l, expression_l)))
    }
    pub(crate) fn match_hash_var_from_str(
        &self,
        begin_t: Ptr<Token>,
        mut strings: Box<List<Node>>,
        end_t: Ptr<Token>,
    ) -> Result<Ptr<Node>, ()> {
        if strings.len() != 1 {
            self.error(
                DiagnosticMessage::new_symbol_literal_with_interpolation(),
                &self.loc(&begin_t).join(&self.loc(&end_t)),
            );
            return Err(());
        }

        let string = strings.remove(0);
        let result = if string.is_str() {
            let internal::Str {
                value,
                begin_l,
                end_l,
                expression_l,
            } = string.into_str().into_internal();

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
            Ptr::new(Node::new_match_var(name, name_l, expression_l))
        } else if string.is_begin() {
            let internal::Begin { statements, .. } = string.into_begin().into_internal();

            self.match_hash_var_from_str(begin_t, Box::new(statements), end_t)?
        } else {
            self.error(
                DiagnosticMessage::new_symbol_literal_with_interpolation(),
                &self.loc(&begin_t).join(&self.loc(&end_t)),
            );
            return Err(());
        };

        Ok(result)
    }

    pub(crate) fn match_rest(
        &self,
        star_t: Ptr<Token>,
        name_t: Maybe<Ptr<Token>>,
    ) -> Result<Ptr<Node>, ()> {
        let name = if name_t.is_none() {
            Maybe::none()
        } else {
            let t = name_t.unwrap();
            Maybe::some(self.match_var(t)?)
        };

        let operator_l = self.loc(&star_t);
        let expression_l = operator_l.maybe_join(&maybe_boxed_node_expr(&name));

        Ok(Ptr::new(Node::new_match_rest(
            name,
            operator_l,
            expression_l,
        )))
    }

    pub(crate) fn hash_pattern(
        &self,
        lbrace_t: Maybe<Ptr<Token>>,
        kwargs: Box<List<Node>>,
        rbrace_t: Maybe<Ptr<Token>>,
    ) -> Ptr<Node> {
        let CollectionMap {
            begin_l,
            end_l,
            expression_l,
        } = self.collection_map(&lbrace_t, &kwargs, &rbrace_t);

        Ptr::new(Node::new_hash_pattern(
            *kwargs,
            begin_l,
            end_l,
            expression_l,
        ))
    }

    pub(crate) fn array_pattern(
        &self,
        lbrack_t: Maybe<Ptr<Token>>,
        elements: Box<List<Node>>,
        trailing_comma: Maybe<Ptr<Token>>,
        rbrack_t: Maybe<Ptr<Token>>,
    ) -> Ptr<Node> {
        let CollectionMap {
            begin_l,
            end_l,
            expression_l,
        } = self.collection_map(&lbrack_t, &elements, &rbrack_t);

        let expression_l = expression_l.maybe_join(&self.maybe_loc(&trailing_comma));

        if elements.is_empty() {
            return Ptr::new(Node::new_array_pattern(
                list![],
                begin_l,
                end_l,
                expression_l,
            ));
        }

        if trailing_comma.is_some() {
            Ptr::new(Node::new_array_pattern_with_tail(
                *elements,
                begin_l,
                end_l,
                expression_l,
            ))
        } else {
            Ptr::new(Node::new_array_pattern(
                *elements,
                begin_l,
                end_l,
                expression_l,
            ))
        }
    }

    pub(crate) fn find_pattern(
        &self,
        lbrack_t: Maybe<Ptr<Token>>,
        elements: Box<List<Node>>,
        rbrack_t: Maybe<Ptr<Token>>,
    ) -> Ptr<Node> {
        let CollectionMap {
            begin_l,
            end_l,
            expression_l,
        } = self.collection_map(&lbrack_t, &elements, &rbrack_t);

        Ptr::new(Node::new_find_pattern(
            *elements,
            begin_l,
            end_l,
            expression_l,
        ))
    }

    pub(crate) fn const_pattern(
        &self,
        const_: Ptr<Node>,
        ldelim_t: Ptr<Token>,
        pattern: Ptr<Node>,
        rdelim_t: Ptr<Token>,
    ) -> Ptr<Node> {
        let begin_l = self.loc(&ldelim_t);
        let end_l = self.loc(&rdelim_t);
        let expression_l = const_.expression().join(&self.loc(&rdelim_t));

        Ptr::new(Node::new_const_pattern(
            const_,
            pattern,
            begin_l,
            end_l,
            expression_l,
        ))
    }

    pub(crate) fn pin(&self, pin_t: Ptr<Token>, var: Ptr<Node>) -> Ptr<Node> {
        let operator_l = self.loc(&pin_t);
        let expression_l = var.expression().join(&operator_l);

        Ptr::new(Node::new_pin(var, operator_l, expression_l))
    }

    pub(crate) fn match_alt(
        &self,
        lhs: Ptr<Node>,
        pipe_t: Ptr<Token>,
        rhs: Ptr<Node>,
    ) -> Ptr<Node> {
        let operator_l = self.loc(&pipe_t);
        let expression_l = join_exprs(&lhs, &rhs);

        Ptr::new(Node::new_match_alt(lhs, rhs, operator_l, expression_l))
    }

    pub(crate) fn match_as(
        &self,
        value: Ptr<Node>,
        assoc_t: Ptr<Token>,
        as_: Ptr<Node>,
    ) -> Ptr<Node> {
        let operator_l = self.loc(&assoc_t);
        let expression_l = join_exprs(&value, &as_);

        Ptr::new(Node::new_match_as(value, as_, operator_l, expression_l))
    }

    pub(crate) fn match_nil_pattern(&self, dstar_t: Ptr<Token>, nil_t: Ptr<Token>) -> Ptr<Node> {
        let operator_l = self.loc(&dstar_t);
        let name_l = self.loc(&nil_t);
        let expression_l = operator_l.join(&name_l);

        Ptr::new(Node::new_match_nil_pattern(
            operator_l,
            name_l,
            expression_l,
        ))
    }

    pub(crate) fn match_pair(
        &self,
        p_kw_label: PKwLabel,
        value: Ptr<Node>,
    ) -> Result<Ptr<Node>, ()> {
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
                            DiagnosticMessage::new_symbol_literal_with_interpolation(),
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

    pub(crate) fn match_label(&self, p_kw_label: PKwLabel) -> Result<Ptr<Node>, ()> {
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

    pub(crate) fn check_condition(&self, cond: Ptr<Node>) -> Ptr<Node> {
        let cond = cond.unptr();

        if cond.is_begin() {
            let internal::Begin {
                statements,
                begin_l,
                end_l,
                expression_l,
            } = cond.into_begin().into_internal();

            if statements.len() == 1 {
                let stmt = statements.take_first();
                let stmt = self.check_condition(Ptr::new(stmt)).unptr();
                Ptr::new(Node::new_begin(list![stmt], begin_l, end_l, expression_l))
            } else {
                Ptr::new(Node::new_begin(statements, begin_l, end_l, expression_l))
            }
        } else if cond.is_and() {
            let internal::And {
                lhs,
                rhs,
                operator_l,
                expression_l,
            } = cond.into_and().into_internal();

            let lhs = self.check_condition(lhs);
            let rhs = self.check_condition(rhs);
            Ptr::new(Node::new_and(lhs, rhs, operator_l, expression_l))
        } else if cond.is_or() {
            let internal::Or {
                lhs,
                rhs,
                operator_l,
                expression_l,
            } = cond.into_or().into_internal();

            let lhs = self.check_condition(lhs);
            let rhs = self.check_condition(rhs);
            Ptr::new(Node::new_or(lhs, rhs, operator_l, expression_l))
        } else if cond.is_irange() {
            let internal::Irange {
                left,
                right,
                operator_l,
                expression_l,
            } = cond.into_irange().into_internal();

            Ptr::new(Node::new_i_flip_flop(
                left.map(|node| self.check_condition(node)),
                right.map(|node| self.check_condition(node)),
                operator_l,
                expression_l,
            ))
        } else if cond.is_erange() {
            let internal::Erange {
                left,
                right,
                operator_l,
                expression_l,
            } = cond.into_erange().into_internal();

            Ptr::new(Node::new_e_flip_flop(
                left.map(|node| self.check_condition(node)),
                right.map(|node| self.check_condition(node)),
                operator_l,
                expression_l,
            ))
        } else if cond.is_regexp() {
            let expression_l = cond.expression().clone();

            Ptr::new(Node::new_match_current_line(Ptr::new(cond), expression_l))
        } else {
            Ptr::new(cond)
        }
    }

    pub(crate) fn check_duplicate_args<'a>(
        &self,
        args: &'a [Node],
        map: &mut HashMap<String, &'a Node>,
    ) {
        for arg in args {
            if arg.is_arg()
                || arg.is_optarg()
                || arg.is_restarg()
                || arg.is_kwarg()
                || arg.is_kwoptarg()
                || arg.is_kwrestarg()
                || arg.is_shadowarg()
                || arg.is_blockarg()
            {
                self.check_duplicate_arg(arg, map);
            } else if let Some(mlhs) = arg.as_mlhs() {
                self.check_duplicate_args(mlhs.get_items(), map);
            } else if let Some(procarg0) = arg.as_procarg0() {
                self.check_duplicate_args(procarg0.get_args(), map);
            } else if arg.is_forward_arg() || arg.is_kwnilarg() {
                // ignore
            } else {
                unreachable!("unsupported arg type {:?}", arg)
            }
        }
    }

    fn arg_name<'a>(&self, node: &'a Node) -> Option<&'a str> {
        if let Some(arg) = node.as_arg() {
            Some(arg.get_name().as_str())
        } else if let Some(optarg) = node.as_optarg() {
            Some(optarg.get_name().as_str())
        } else if let Some(kwarg) = node.as_kwarg() {
            Some(kwarg.get_name().as_str())
        } else if let Some(kwoptarg) = node.as_kwoptarg() {
            Some(kwoptarg.get_name().as_str())
        } else if let Some(shadowarg) = node.as_shadowarg() {
            Some(shadowarg.get_name().as_str())
        } else if let Some(blockarg) = node.as_blockarg() {
            Some(blockarg.get_name().as_str())
        } else if let Some(restarg) = node.as_restarg() {
            restarg.get_name().as_ref().map(|s| s.as_str())
        } else if let Some(kwrestarg) = node.as_kwrestarg() {
            kwrestarg.get_name().as_ref().map(|s| s.as_str())
        } else {
            unreachable!("unsupported arg {:?}", node)
        }
    }

    fn arg_name_loc<'a>(&self, node: &'a Node) -> &'a Loc {
        if let Some(arg) = node.as_arg() {
            arg.get_expression_l()
        } else if let Some(optarg) = node.as_optarg() {
            optarg.get_name_l()
        } else if let Some(kwarg) = node.as_kwarg() {
            kwarg.get_name_l()
        } else if let Some(kwoptarg) = node.as_kwoptarg() {
            kwoptarg.get_name_l()
        } else if let Some(shadowarg) = node.as_shadowarg() {
            shadowarg.get_expression_l()
        } else if let Some(blockarg) = node.as_blockarg() {
            blockarg.get_name_l()
        } else if let Some(restarg) = node.as_restarg() {
            restarg
                .get_name_l()
                .as_ref()
                .unwrap_or_else(|| restarg.get_expression_l())
        } else if let Some(kwrestarg) = node.as_kwrestarg() {
            kwrestarg
                .get_name_l()
                .as_ref()
                .unwrap_or_else(|| kwrestarg.get_expression_l())
        } else {
            unreachable!("unsupported arg {:?}", node)
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
                        DiagnosticMessage::new_duplicated_argument_name(),
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
                DiagnosticMessage::new_cant_assign_to_numparam(StringPtr::from(name)),
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
                    DiagnosticMessage::new_reserved_for_numparam(StringPtr::from(name)),
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
            self.error(
                DiagnosticMessage::new_key_must_be_valid_as_local_variable(),
                loc,
            );
            Err(())
        }
    }

    pub(crate) fn check_duplicate_pattern_variable(&self, name: &str, loc: &Loc) -> Result<(), ()> {
        if name.starts_with('_') {
            return Ok(());
        }

        if self.pattern_variables.is_declared(name) {
            self.error(DiagnosticMessage::new_duplicate_variable_name(), loc);
            return Err(());
        }

        self.pattern_variables.declare(name);
        Ok(())
    }

    pub(crate) fn check_duplicate_pattern_key(&self, name: &str, loc: &Loc) -> Result<(), ()> {
        if self.pattern_hash_keys.is_declared(name) {
            self.error(DiagnosticMessage::new_duplicate_key_name(), loc);
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
            if let Some(str) = node.as_str() {
                let value = str.get_value().to_string_lossy();
                result.push_str(value.as_str())
            } else if let Some(begin) = node.as_begin() {
                let statements = begin.get_statements();
                if let Some(s) = self.static_string(statements) {
                    result.push_str(&s)
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }

        Some(result)
    }

    #[cfg(feature = "onig")]
    pub(crate) fn build_static_regexp(
        &self,
        parts: &[Node],
        options: &Maybe<StringPtr>,
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
                    DiagnosticMessage::new_regex_error(StringPtr::from(err.description())),
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
        options: &Maybe<StringPtr>,
        loc: &Loc,
    ) {
        self.build_static_regexp(parts, options, loc);
    }

    #[cfg(not(feature = "onig"))]
    pub(crate) fn validate_static_regexp(
        &self,
        _parts: &[Node],
        _options: &Maybe<StringPtr>,
        _loc: &Loc,
    ) {
    }

    #[cfg(feature = "onig")]
    pub(crate) fn static_regexp_captures(&self, node: &Node) -> Option<Vec<String>> {
        if node.is_regexp() {
            let node = node.as_regexp().unwrap();
            let parts = node.get_parts();
            let options = node.get_options();
            let expression_l = node.get_expression_l();

            let mut re_options = &Maybe::none();
            if let Some(options) = options.as_ref() {
                if let Some(regopt) = options.as_reg_opt() {
                    re_options = regopt.get_options();
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

    pub(crate) fn loc(&self, token: &Token) -> Loc {
        token.loc().clone()
    }

    pub(crate) fn maybe_loc(&self, token: &Maybe<Ptr<Token>>) -> Maybe<Loc> {
        match token.as_ref() {
            Some(token) => Maybe::some(self.loc(token.as_ref())),
            None => Maybe::none(),
        }
    }

    pub(crate) fn collection_map(
        &self,
        begin_t: &Maybe<Ptr<Token>>,
        parts: &[Node],
        end_t: &Maybe<Ptr<Token>>,
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

    pub(crate) fn is_heredoc(&self, begin_t: &Maybe<Ptr<Token>>) -> bool {
        if let Some(begin_t) = begin_t.as_ref() {
            if clone_value(begin_t.as_ref()).as_str().starts_with("<<") {
                return true;
            }
        }
        false
    }

    pub(crate) fn heredoc_map(
        &self,
        begin_t: &Maybe<Ptr<Token>>,
        parts: &[Node],
        end_t: &Maybe<Ptr<Token>>,
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
        self.diagnostics
            .emit(Diagnostic::new(ErrorLevel::error(), message, loc.clone()))
    }

    pub(crate) fn warn(&self, message: DiagnosticMessage, loc: &Loc) {
        self.diagnostics
            .emit(Diagnostic::new(ErrorLevel::warning(), message, loc.clone()))
    }

    pub(crate) fn value_expr(&self, node: &Node) -> Result<(), ()> {
        if let Some(void_node) = self.void_value(node) {
            self.error(
                DiagnosticMessage::new_void_value_expression(),
                void_node.expression(),
            );
            Err(())
        } else {
            Ok(())
        }
    }

    fn void_value<'a>(&self, node: &'a Node) -> Option<&'a Node> {
        let check_stmts = |statements: &'a List<Node>| {
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
            |if_true: &'a Maybe<Ptr<Node>>, if_false: &'a Maybe<Ptr<Node>>| match (
                if_true.as_ref(),
                if_false.as_ref(),
            ) {
                (None, None) | (None, Some(_)) | (Some(_), None) => None,
                (Some(if_true), Some(if_false)) => check_condition(if_true, if_false),
            };

        if node.is_return()
            || node.is_break()
            || node.is_next()
            || node.is_redo()
            || node.is_retry()
        {
            Some(node)
        } else if let Some(match_pattern) = node.as_match_pattern() {
            self.void_value(match_pattern.get_value())
        } else if let Some(match_pattern_p) = node.as_match_pattern_p() {
            self.void_value(match_pattern_p.get_value())
        } else if let Some(begin) = node.as_begin() {
            check_stmts(begin.get_statements())
        } else if let Some(kw_begin) = node.as_kw_begin() {
            check_stmts(kw_begin.get_statements())
        } else if let Some(if_) = node.as_if() {
            check_maybe_condition(if_.get_if_true(), if_.get_if_false())
        } else if let Some(if_mod) = node.as_if_mod() {
            check_maybe_condition(if_mod.get_if_true(), if_mod.get_if_false())
        } else if let Some(if_ternary) = node.as_if_ternary() {
            check_condition(if_ternary.get_if_true(), if_ternary.get_if_false())
        } else if let Some(and) = node.as_and() {
            self.void_value(and.get_lhs())
        } else if let Some(or) = node.as_or() {
            self.void_value(or.get_lhs())
        } else {
            None
        }
    }

    fn rewrite_hash_args_to_kwargs(&self, args: &mut List<Node>) {
        let len = args.len();

        if !args.is_empty() && self.is_kwargs(&args[len - 1]) {
            let internal::Hash {
                pairs,
                expression_l,
                ..
            } = args.pop().unwrap().into_hash().into_internal();

            let kwargs = Node::new_kwargs(pairs, expression_l);
            args.push(kwargs);
        } else if len > 1 && args[len - 1].is_block_pass() && self.is_kwargs(&args[len - 2]) {
            let block_pass = args.pop().unwrap();
            let internal::Hash {
                pairs,
                expression_l,
                ..
            } = args.pop().unwrap().into_hash().into_internal();
            let kwargs = Node::new_kwargs(pairs, expression_l);
            args.push(kwargs);
            args.push(block_pass);
        }
    }

    fn is_kwargs(&self, node: &Node) -> bool {
        if let Some(hash) = node.as_hash() {
            hash.get_begin_l().is_none() && hash.get_end_l().is_none()
        } else {
            false
        }
    }
}

pub(crate) fn maybe_node_expr(node: &Option<&Node>) -> Maybe<Loc> {
    match node {
        Some(node) => Maybe::some(node.expression().clone()),
        None => Maybe::none(),
    }
}

pub(crate) fn maybe_boxed_node_expr(node: &Maybe<Ptr<Node>>) -> Maybe<Loc> {
    match node.as_ref() {
        Some(node) => Maybe::some(node.expression().clone()),
        None => Maybe::none(),
    }
}

pub(crate) fn collection_expr(nodes: &[Node]) -> Maybe<Loc> {
    join_maybe_exprs(&nodes.first(), &nodes.last())
}

pub(crate) fn value(token: Ptr<Token>) -> StringPtr {
    token.unptr().into_string().unwrap()
}

pub(crate) fn lossy_value(token: Ptr<Token>) -> StringPtr {
    token.to_string_lossy()
}

pub(crate) fn clone_value(token: &Token) -> StringPtr {
    token.to_string_lossy()
}

pub(crate) fn maybe_value(token: Maybe<Ptr<Token>>) -> Maybe<StringPtr> {
    token.map(value)
}

pub(crate) fn join_exprs(lhs: &Node, rhs: &Node) -> Loc {
    lhs.expression().join(rhs.expression())
}

pub(crate) fn join_maybe_exprs(lhs: &Option<&Node>, rhs: &Option<&Node>) -> Maybe<Loc> {
    join_maybe_locs(&maybe_node_expr(lhs), &maybe_node_expr(rhs))
}

pub(crate) fn join_maybe_locs(lhs: &Maybe<Loc>, rhs: &Maybe<Loc>) -> Maybe<Loc> {
    match (lhs.as_ref(), rhs.as_ref()) {
        (None, None) => Maybe::none(),
        (None, Some(rhs)) => Maybe::some(rhs.clone()),
        (Some(lhs), None) => Maybe::some(lhs.clone()),
        (Some(lhs), Some(rhs)) => Maybe::some(lhs.join(rhs)),
    }
}

pub(crate) struct CollectionMap {
    begin_l: Maybe<Loc>,
    end_l: Maybe<Loc>,
    expression_l: Loc,
}

pub(crate) struct HeredocMap {
    heredoc_body_l: Loc,
    heredoc_end_l: Loc,
    expression_l: Loc,
}
