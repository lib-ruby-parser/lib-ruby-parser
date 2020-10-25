use onig::{Regex, RegexOptions};
use std::collections::HashMap;
use std::convert::TryInto;
use std::rc::Rc;

use crate::nodes::StringValue;
use crate::nodes::*;
use crate::parser::TokenValue;
// use crate::source::map::*;
use crate::source::Range;
use crate::{
    source::buffer::Input, Context, CurrentArgStack, Lexer, Loc, MaxNumparamStack, Node,
    StaticEnvironment, Token, VariablesStack,
};

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
    PlainLabel(Token),
    QuotedLabel((Token, Vec<Node>, Token)),
}

#[derive(Debug, Clone)]
pub(crate) enum ArgsType {
    Args(Option<Node>),
    Numargs(u8),
}

#[derive(Debug)]
pub struct Builder {
    static_env: StaticEnvironment,
    context: Context,
    current_arg_stack: CurrentArgStack,
    max_numparam_stack: MaxNumparamStack,
    pattern_variables: VariablesStack,
    pattern_hash_keys: VariablesStack,
    source: Rc<Input>,
}

impl Builder {
    pub(crate) fn new(
        static_env: StaticEnvironment,
        context: Context,
        current_arg_stack: CurrentArgStack,
        max_numparam_stack: MaxNumparamStack,
        pattern_variables: VariablesStack,
        pattern_hash_keys: VariablesStack,
        source: Rc<Input>,
    ) -> Self {
        Self {
            static_env,
            context,
            current_arg_stack,
            max_numparam_stack,
            pattern_variables,
            pattern_hash_keys,
            source,
        }
    }

    //
    // Literals
    //

    // Singletons

    pub(crate) fn nil(&self, nil_t: Token) -> Node {
        Node::Nil(Nil {
            expression_l: self.loc(&nil_t),
        })
    }

    pub(crate) fn true_(&self, true_t: Token) -> Node {
        Node::True(True {
            expression_l: self.loc(&true_t),
        })
    }

    pub(crate) fn false_(&self, false_t: Token) -> Node {
        Node::False(False {
            expression_l: self.loc(&false_t),
        })
    }

    // Numerics

    pub(crate) fn integer(&self, integer_t: Token) -> Node {
        Node::Int(Int {
            value: value(&integer_t),
            expression_l: self.loc(&integer_t),
            operator_l: None,
        })
    }

    pub(crate) fn float(&self, float_t: Token) -> Node {
        Node::Float(Float {
            value: value(&float_t),
            expression_l: self.loc(&float_t),
            operator_l: None,
        })
    }

    pub(crate) fn rational(&self, rational_t: Token) -> Node {
        Node::Rational(Rational {
            value: value(&rational_t),
            expression_l: self.loc(&rational_t),
            operator_l: None,
        })
    }

    pub(crate) fn complex(&self, complex_t: Token) -> Node {
        Node::Complex(Complex {
            value: value(&complex_t),
            expression_l: self.loc(&complex_t),
            operator_l: None,
        })
    }

    pub(crate) fn unary_num(&self, unary_t: Token, mut numeric: Node) -> Node {
        let sign = value(&unary_t);
        let new_operator_l = self.loc(&unary_t);

        match &mut numeric {
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
                *expression_l = new_operator_l.join(&expression_l);
                *operator_l = Some(new_operator_l);
                numeric
            }
            _ => unreachable!(),
        }
    }

    pub(crate) fn __line__(&self, line_t: Token) -> Node {
        Node::Line(Line {
            expression_l: self.loc(&line_t),
        })
    }

    // Strings

    pub(crate) fn str_node(
        &self,
        begin_t: Option<Token>,
        value: StringValue,
        parts: Vec<Node>,
        end_t: Option<Token>,
    ) -> Node {
        match self.string_map(&begin_t, &parts, &end_t) {
            StringMap::CollectionMap((begin_l, end_l, expression_l)) => Node::Str(Str {
                value,
                begin_l,
                end_l,
                expression_l,
            }),
            StringMap::HeredocMap((heredoc_body_l, heredoc_end_l, expression_l)) => {
                Node::Heredoc(Heredoc {
                    parts,
                    heredoc_body_l,
                    heredoc_end_l,
                    expression_l,
                })
            }
        }
    }

    pub(crate) fn string_internal(&self, string_t: Token) -> Node {
        let expression_l = self.loc(&string_t);
        let value = match string_t.token_value {
            TokenValue::String(s) => StringValue {
                bytes: s.as_bytes().to_owned(),
            },
            TokenValue::InvalidString(bytes) => StringValue { bytes },
        };
        Node::Str(Str {
            value,
            begin_l: None,
            end_l: None,
            expression_l,
        })
    }

    pub(crate) fn string_compose(
        &self,
        begin_t: Option<Token>,
        parts: Vec<Node>,
        end_t: Option<Token>,
    ) -> Node {
        match &parts[..] {
            [] => return self.str_node(begin_t, StringValue { bytes: vec![] }, parts, end_t),
            [Node::Str(_)] | [Node::Dstr(_)] | [Node::Heredoc(_)]
                if begin_t.is_none() && end_t.is_none() =>
            {
                return parts[0].clone();
            }
            [Node::Str(Str { value, .. })] => {
                return self.str_node(begin_t, value.clone(), parts, end_t)
            }
            [Node::Dstr(_)] | [Node::Heredoc(_)] => unreachable!(),
            _ => {}
        };

        match self.string_map(&begin_t, &parts, &end_t) {
            StringMap::CollectionMap((begin_l, end_l, expression_l)) => Node::Dstr(Dstr {
                parts,
                begin_l,
                end_l,
                expression_l,
            }),
            StringMap::HeredocMap((heredoc_body_l, heredoc_end_l, expression_l)) => {
                Node::Heredoc(Heredoc {
                    parts,
                    heredoc_body_l,
                    heredoc_end_l,
                    expression_l,
                })
            }
        }
    }

    pub(crate) fn character(&self, char_t: Token) -> Node {
        let str_range = self.loc(&char_t);

        let begin_l = Some(str_range.with_end(str_range.begin_pos + 1));
        let end_l = None;
        let expression_l = str_range;

        let value = match char_t.token_value {
            TokenValue::String(s) => StringValue {
                bytes: s.as_bytes().to_owned(),
            },
            TokenValue::InvalidString(bytes) => StringValue { bytes },
        };
        Node::Str(Str {
            value,
            begin_l,
            end_l,
            expression_l,
        })
    }

    pub(crate) fn __file__(&self, file_t: Token) -> Node {
        Node::File(File {
            expression_l: self.loc(&file_t),
        })
    }

    // Symbols

    pub(crate) fn symbol(&self, start_t: Token, value_t: Token) -> Node {
        let expression_l = self.loc(&start_t).join(&self.loc(&value_t));
        let begin_l = Some(self.loc(&start_t));
        Node::Sym(Sym {
            name: value(&value_t),
            begin_l,
            end_l: None,
            expression_l,
        })
    }

    pub(crate) fn symbol_internal(&self, symbol_t: Token) -> Node {
        Node::Sym(Sym {
            name: value(&symbol_t),
            begin_l: None,
            end_l: None,
            expression_l: self.loc(&symbol_t),
        })
    }

    pub(crate) fn symbol_compose(&self, begin_t: Token, parts: Vec<Node>, end_t: Token) -> Node {
        match &parts[..] {
            [Node::Str(Str { value, .. })] => {
                let (begin_l, end_l, expression_l) =
                    self.collection_map(&Some(begin_t), &vec![], &Some(end_t));

                return Node::Sym(Sym {
                    name: value.to_string_lossy(),
                    begin_l,
                    end_l,
                    expression_l,
                });
            }
            _ => {}
        };

        let (begin_l, end_l, expression_l) =
            self.collection_map(&Some(begin_t), &parts, &Some(end_t));
        Node::Dsym(Dsym {
            parts,
            begin_l,
            end_l,
            expression_l,
        })
    }

    // Executable strings

    pub(crate) fn xstring_compose(&self, begin_t: Token, parts: Vec<Node>, end_t: Token) -> Node {
        match self.string_map(&Some(begin_t), &parts, &Some(end_t)) {
            StringMap::CollectionMap((begin_l, end_l, expression_l)) => Node::Xstr(Xstr {
                parts,
                begin_l: begin_l.unwrap(),
                end_l: end_l.unwrap(),
                expression_l,
            }),
            StringMap::HeredocMap((heredoc_body_l, heredoc_end_l, expression_l)) => {
                Node::XHeredoc(XHeredoc {
                    parts,
                    heredoc_body_l,
                    heredoc_end_l,
                    expression_l,
                })
            }
        }
    }

    // Indented (interpolated, noninterpolated, executable) strings

    pub(crate) fn heredoc_dedent(&self, node: &mut Node, dedent_level: i32) {
        if dedent_level == 0 {
            return;
        }

        let dedent_level: usize = dedent_level.try_into().unwrap();

        match node {
            Node::Heredoc(Heredoc { parts, .. }) | Node::XHeredoc(XHeredoc { parts, .. }) => {
                for part in parts.iter_mut() {
                    match part {
                        Node::Str(Str { value, .. }) => Self::dedent_string(value, dedent_level),
                        Node::Begin(_) => {}
                        _ => unimplemented!("unsupported heredoc child {}", part.str_type()),
                    }
                }
            }
            other => unimplemented!("unsupported heredoc_dedent argument {}", other.str_type()),
        }
    }

    const TAB_WIDTH: usize = 8;

    pub fn dedent_string(s: &mut StringValue, width: usize) {
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

        s.bytes = s.bytes[i..].to_owned()
    }

    // Regular expressions

    pub(crate) fn regexp_options(&self, regexp_end_t: &Token) -> Node {
        let mut options = value(&regexp_end_t)[1..].chars().collect::<Vec<_>>();
        options.sort();
        options.dedup();

        Node::RegOpt(RegOpt {
            options,
            expression_l: self.loc(&regexp_end_t).adjust_begin(1),
        })
    }

    pub(crate) fn regexp_compose(
        &self,
        begin_t: Token,
        parts: Vec<Node>,
        end_t: Token,
        options: Node,
    ) -> Node {
        let begin_l = self.loc(&begin_t);
        let end_l = self.loc(&end_t).resize(1);
        let expression_l = begin_l.join(options.expression());
        Node::Regexp(Regexp {
            parts,
            options: Box::new(options),
            begin_l,
            end_l,
            expression_l,
        })
    }

    // Arrays

    pub(crate) fn array(
        &self,
        begin_t: Option<Token>,
        elements: Vec<Node>,
        end_t: Option<Token>,
    ) -> Node {
        let (begin_l, end_l, expression_l) = self.collection_map(&begin_t, &elements, &end_t);
        Node::Array(Array {
            elements,
            begin_l,
            end_l,
            expression_l,
        })
    }

    pub(crate) fn splat(&self, star_t: Token, value: Option<Node>) -> Node {
        let operator_l = self.loc(&star_t);
        let expression_l = operator_l.maybe_join(&maybe_node_expr(&value));

        Node::Splat(Splat {
            operator_l,
            expression_l,
            value: value.map(Box::new),
        })
    }

    pub(crate) fn word(&self, parts: Vec<Node>) -> Node {
        match &parts[..] {
            [Node::Str(_)] | [Node::Dstr(_)] => {
                // collapse_string_parts? == true
                return parts[0].clone();
            }
            _ => {}
        }

        let (begin_l, end_l, expression_l) = self.collection_map(&None, &parts, &None);
        Node::Dstr(Dstr {
            parts,
            begin_l,
            end_l,
            expression_l,
        })
    }

    pub(crate) fn words_compose(&self, begin_t: Token, elements: Vec<Node>, end_t: Token) -> Node {
        let begin_l = self.loc(&begin_t);
        let end_l = self.loc(&end_t);
        let expression_l = begin_l.join(&end_l);
        Node::Array(Array {
            elements,
            begin_l: Some(begin_l),
            end_l: Some(end_l),
            expression_l,
        })
    }

    pub(crate) fn symbols_compose(&self, begin_t: Token, parts: Vec<Node>, end_t: Token) -> Node {
        let parts = parts
            .into_iter()
            .map(|part| match part {
                Node::Str(Str {
                    value,
                    begin_l,
                    end_l,
                    expression_l,
                }) => Node::Sym(Sym {
                    name: value.to_string_lossy(),
                    begin_l,
                    end_l,
                    expression_l,
                }),
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
        Node::Array(Array {
            elements: parts,
            begin_l: Some(begin_l),
            end_l: Some(end_l),
            expression_l,
        })
    }

    // Hashes

    pub(crate) fn pair(&self, key: Node, assoc_t: Token, value: Node) -> Node {
        let operator_l = self.loc(&assoc_t);
        let expression_l = join_exprs(&key, &value);

        Node::Pair(Pair {
            key: Box::new(key),
            value: Box::new(value),
            operator_l,
            expression_l,
        })
    }

    pub(crate) fn pair_keyword(&self, key_t: Token, value_node: Node) -> Node {
        let key_range = self.loc(&key_t);
        let key_l = key_range.adjust_end(-1);
        let colon_l = key_range.with_begin((key_range.end_pos - 1).try_into().unwrap());
        let expression_l = key_range.join(&value_node.expression());

        let key = Node::Sym(Sym {
            name: value(&key_t),
            begin_l: None,
            end_l: None,
            expression_l: key_l,
        });
        Node::Pair(Pair {
            key: Box::new(key),
            value: Box::new(value_node),
            operator_l: colon_l,
            expression_l,
        })
    }

    pub(crate) fn pair_quoted(
        &self,
        begin_t: Token,
        parts: Vec<Node>,
        end_t: Token,
        value: Node,
    ) -> Node {
        let end_l = self.loc(&end_t);

        let quote_loc = Loc {
            begin: end_l.end_pos - 2,
            end: end_l.end_pos - 1,
        };

        let colon_l = end_l.with_begin(end_l.end_pos - 1);

        let end_t: Token = Token {
            token_type: end_t.token_type,
            token_value: end_t.token_value.clone(),
            loc: quote_loc,
        };
        let expression_l = self.loc(&begin_t).join(&value.expression());

        let key = self.symbol_compose(begin_t, parts, end_t);

        Node::Pair(Pair {
            key: Box::new(key),
            value: Box::new(value),
            operator_l: colon_l,
            expression_l,
        })
    }

    pub(crate) fn kwsplat(&self, dstar_t: Token, arg: Node) -> Node {
        let operator_l = self.loc(&dstar_t);
        let expression_l = arg.expression().join(&operator_l);

        Node::Kwsplat(Kwsplat {
            value: Box::new(arg),
            operator_l,
            expression_l,
        })
    }

    pub(crate) fn associate(
        &self,
        begin_t: Option<Token>,
        pairs: Vec<Node>,
        end_t: Option<Token>,
    ) -> Node {
        let (begin_l, end_l, expression_l) = self.collection_map(&begin_t, &pairs, &end_t);
        Node::Hash(Hash {
            pairs,
            begin_l,
            end_l,
            expression_l,
        })
    }

    // Ranges

    pub(crate) fn range_inclusive(
        &self,
        lhs: Option<Node>,
        dot2_t: Token,
        rhs: Option<Node>,
    ) -> Node {
        let operator_l = self.loc(&dot2_t);
        let expression_l = operator_l
            .maybe_join(&maybe_node_expr(&lhs))
            .maybe_join(&maybe_node_expr(&rhs));

        Node::Irange(Irange {
            left: lhs.map(Box::new),
            right: rhs.map(Box::new),
            operator_l,
            expression_l,
        })
    }

    pub(crate) fn range_exclusive(
        &self,
        lhs: Option<Node>,
        dot3_t: Token,
        rhs: Option<Node>,
    ) -> Node {
        let operator_l = self.loc(&dot3_t);
        let expression_l = operator_l
            .maybe_join(&maybe_node_expr(&lhs))
            .maybe_join(&maybe_node_expr(&rhs));

        Node::Erange(Erange {
            left: lhs.map(Box::new),
            right: rhs.map(Box::new),
            operator_l,
            expression_l,
        })
    }

    //
    // Access
    //

    pub(crate) fn self_(&self, token: Token) -> Node {
        Node::Self_(Self_ {
            expression_l: self.loc(&token),
        })
    }

    pub(crate) fn lvar(&self, token: Token) -> Node {
        Node::Lvar(Lvar {
            name: value(&token),
            expression_l: self.loc(&token),
        })
    }

    pub(crate) fn ivar(&self, token: Token) -> Node {
        Node::Ivar(Ivar {
            name: value(&token),
            expression_l: self.loc(&token),
        })
    }

    pub(crate) fn gvar(&self, token: Token) -> Node {
        Node::Gvar(Gvar {
            name: value(&token),
            expression_l: self.loc(&token),
        })
    }

    pub(crate) fn cvar(&self, token: Token) -> Node {
        Node::Cvar(Cvar {
            name: value(&token),
            expression_l: self.loc(&token),
        })
    }

    pub(crate) fn back_ref(&self, token: Token) -> Node {
        Node::BackRef(BackRef {
            name: value(&token),
            expression_l: self.loc(&token),
        })
    }

    pub(crate) fn nth_ref(&self, token: Token) -> Node {
        let name = value(&token)[1..].parse::<usize>().unwrap();
        Node::NthRef(NthRef {
            name,
            expression_l: self.loc(&token),
        })
    }
    pub(crate) fn accessible(&self, node: Node) -> Node {
        match node {
            Node::Lvar(Lvar { name, expression_l }) => {
                if self.static_env.is_declared(&name) {
                    if let Some(current_arg) = self.current_arg_stack.top() {
                        if current_arg == name {
                            // diagnostic :error, :circular_argument_reference,
                            //        { :var_name => name.to_s }, node.loc.expression
                        }
                    }

                    Node::Lvar(Lvar { name, expression_l })
                } else {
                    Node::Send(Send {
                        recv: None,
                        method_name: name,
                        args: vec![],
                        dot_l: None,
                        selector_l: Some(expression_l.clone()),
                        begin_l: None,
                        end_l: None,
                        operator_l: None,
                        expression_l,
                    })
                }
            }
            _ => node,
        }
    }

    pub(crate) fn const_(&self, name_t: Token) -> Node {
        let name_l = self.loc(&name_t);
        let expression_l = name_l.clone();

        Node::Const(Const {
            scope: None,
            name: value(&name_t),
            double_colon_l: None,
            name_l,
            expression_l,
        })
    }

    pub(crate) fn const_global(&self, t_colon3: Token, name_t: Token) -> Node {
        let scope = Node::Cbase(Cbase {
            expression_l: self.loc(&t_colon3),
        });

        let name_l = self.loc(&name_t);
        let expression_l = scope.expression().join(&name_l);
        let double_colon_l = self.loc(&t_colon3);

        Node::Const(Const {
            scope: Some(Box::new(scope)),
            name: value(&name_t),
            double_colon_l: Some(double_colon_l),
            name_l,
            expression_l,
        })
    }

    pub(crate) fn const_fetch(&self, scope: Node, t_colon2: Token, name_t: Token) -> Node {
        let name_l = self.loc(&name_t);
        let expression_l = scope.expression().join(&name_l);
        let double_colon_l = self.loc(&t_colon2);

        Node::Const(Const {
            scope: Some(Box::new(scope)),
            name: value(&name_t),
            double_colon_l: Some(double_colon_l),
            name_l,
            expression_l,
        })
    }

    pub(crate) fn __encoding__(&self, encoding_t: Token) -> Node {
        Node::Encoding(Encoding {
            expression_l: self.loc(&encoding_t),
        })
    }

    //
    // Assignments
    //

    pub(crate) fn assignable(&self, node: Node) -> Node {
        match node {
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
                    // diagnostic :error, :dynamic_const, nil, node.loc.expression
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
                self.check_assignment_to_numparam(&name, &expression_l);
                self.check_reserved_for_numparam(&name, &expression_l);

                self.static_env.declare(&name);

                Node::Lvasgn(Lvasgn {
                    name,
                    value: None,
                    name_l: expression_l.clone(),
                    expression_l,
                    operator_l: None,
                })
            }

            Node::Nil(Nil { .. })
            | Node::Self_(Self_ { .. })
            | Node::True(True { .. })
            | Node::False(False { .. })
            | Node::File(File { .. })
            | Node::Line(Line { .. })
            | Node::Encoding(Encoding { .. }) => {
                // diagnostic :error, :invalid_assignment, nil, node.loc.expression
                node
            }
            Node::BackRef(BackRef { .. }) | Node::NthRef(NthRef { .. }) => {
                // diagnostic :error, :backref_assignment, nil, node.loc.expression
                node
            }
            _ => panic!("{:?} can't be used in assignment", node),
        }
    }

    pub(crate) fn const_op_assignable(&self, node: Node) -> Node {
        match node {
            Node::Const(Const {
                scope,
                name,
                name_l,
                double_colon_l,
                expression_l,
            }) => Node::Casgn(Casgn {
                scope,
                name,
                name_l,
                double_colon_l,
                expression_l,
                value: None,
                operator_l: None,
            }),
            _ => panic!("unsupported const_op_assignable arument: {:?}", node),
        }
    }

    pub(crate) fn assign(&self, mut lhs: Node, eql_t: Token, new_rhs: Node) -> Node {
        let op_l = Some(self.loc(&eql_t));
        let expr_l = join_exprs(&lhs, &new_rhs);

        match lhs {
            Node::Cvasgn(Cvasgn {
                ref mut expression_l,
                ref mut operator_l,
                ref mut value,
                ..
            })
            | Node::Ivasgn(Ivasgn {
                ref mut expression_l,
                ref mut operator_l,
                ref mut value,
                ..
            })
            | Node::Gvasgn(Gvasgn {
                ref mut expression_l,
                ref mut operator_l,
                ref mut value,
                ..
            })
            | Node::Lvasgn(Lvasgn {
                ref mut expression_l,
                ref mut operator_l,
                ref mut value,
                ..
            })
            | Node::Casgn(Casgn {
                ref mut expression_l,
                ref mut operator_l,
                ref mut value,
                ..
            })
            | Node::IndexAsgn(IndexAsgn {
                ref mut expression_l,
                ref mut operator_l,
                ref mut value,
                ..
            }) => {
                *expression_l = expr_l;
                *operator_l = op_l;
                *value = Some(Box::new(new_rhs));
                lhs
            }
            Node::Send(Send {
                ref mut args,
                ref mut expression_l,
                ref mut operator_l,
                ..
            })
            | Node::CSend(CSend {
                ref mut args,
                ref mut expression_l,
                ref mut operator_l,
                ..
            }) => {
                *expression_l = expr_l;
                *operator_l = op_l;
                if args.is_empty() {
                    *args = vec![new_rhs];
                } else {
                    unreachable!("can't assign to method call with args")
                }
                lhs
            }
            _ => panic!("{:?} can't be used in assignment", lhs),
        }
    }

    pub(crate) fn op_assign(&self, mut lhs: Node, op_t: Token, rhs: Node) -> Node {
        let mut operator = value(&op_t);
        operator.pop();
        let operator_l = self.loc(&op_t);
        let expression_l = join_exprs(&lhs, &rhs);

        match lhs {
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
                lhs = Node::IndexAsgn(IndexAsgn {
                    recv,
                    indexes,
                    value: None,
                    begin_l,
                    end_l,
                    expression_l,
                    operator_l: None,
                });
            }
            Node::BackRef { .. } | Node::NthRef { .. } => {
                // diagnostic :error, :backref_assignment, nil, lhs.loc.expression
                return rhs;
            }
            _ => panic!("unsupported op_assign lhs {:?}", lhs),
        };

        let recv = Box::new(lhs);
        let value = Box::new(rhs);

        match &operator[..] {
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
        }
    }

    pub(crate) fn multi_lhs(
        &self,
        begin_t: Option<Token>,
        items: Vec<Node>,
        end_t: Option<Token>,
    ) -> Node {
        let (begin_l, end_l, expression_l) = self.collection_map(&begin_t, &items, &end_t);
        Node::Mlhs(Mlhs {
            items,
            begin_l,
            end_l,
            expression_l,
        })
    }

    pub(crate) fn multi_assign(&self, lhs: Node, eql_t: Token, rhs: Node) -> Node {
        let operator_l = self.loc(&eql_t);
        let expression_l = join_exprs(&lhs, &rhs);

        Node::Masgn(Masgn {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            operator_l,
            expression_l,
        })
    }

    pub(crate) fn rassign(&self, lhs: Node, eql_t: Token, rhs: Node) -> Node {
        self.assign(rhs, eql_t, lhs)
    }

    pub(crate) fn multi_rassign(&self, lhs: Node, eql_t: Token, rhs: Node) -> Node {
        self.multi_assign(rhs, eql_t, lhs)
    }

    //
    // Class and module definition
    //

    pub(crate) fn def_class(
        &self,
        class_t: Token,
        name: Node,
        lt_t: Option<Token>,
        superclass: Option<Node>,
        body: Option<Node>,
        end_t: Token,
    ) -> Node {
        let keyword_l = self.loc(&class_t);
        let end_l = self.loc(&end_t);
        let operator_l = self.maybe_loc(&lt_t);
        let expression_l = keyword_l.join(&end_l);

        Node::Class(Class {
            name: Box::new(name),
            superclass: superclass.map(Box::new),
            body: body.map(Box::new),
            keyword_l,
            operator_l,
            end_l,
            expression_l,
        })
    }

    pub(crate) fn def_sclass(
        &self,
        class_t: Token,
        lshift_t: Token,
        expr: Node,
        body: Option<Node>,
        end_t: Token,
    ) -> Node {
        let keyword_l = self.loc(&class_t);
        let end_l = self.loc(&end_t);
        let operator_l = self.loc(&lshift_t);
        let expression_l = keyword_l.join(&end_l);

        Node::SClass(SClass {
            expr: Box::new(expr),
            body: body.map(|node| Box::new(node)),
            keyword_l,
            operator_l,
            end_l,
            expression_l,
        })
    }

    pub(crate) fn def_module(
        &self,
        module_t: Token,
        name: Node,
        body: Option<Node>,
        end_t: Token,
    ) -> Node {
        let keyword_l = self.loc(&module_t);
        let end_l = self.loc(&end_t);
        let expression_l = keyword_l.join(&end_l);

        Node::Module(Module {
            name: Box::new(name),
            body: body.map(|node| Box::new(node)),
            keyword_l,
            end_l,
            expression_l,
        })
    }

    //
    // Method (un)definition
    //

    pub(crate) fn def_method(
        &self,
        def_t: Token,
        name_t: Token,
        args: Option<Node>,
        body: Option<Node>,
        end_t: Token,
    ) -> Node {
        self.check_reserved_for_numparam(&value(&name_t), &self.loc(&name_t));

        let keyword_l = self.loc(&def_t);
        let name_l = self.loc(&name_t);
        let end_l = self.loc(&end_t);
        let expression_l = keyword_l.join(&end_l);

        Node::Def(Def {
            name: value(&name_t),
            args: args.map(|node| Box::new(node)),
            body: body.map(|node| Box::new(node)),
            keyword_l,
            name_l,
            assignment_l: None,
            end_l: Some(end_l),
            expression_l,
        })
    }

    pub(crate) fn def_endless_method(
        &self,
        def_t: Token,
        name_t: Token,
        args: Option<Node>,
        assignment_t: Token,
        body: Option<Node>,
    ) -> Node {
        self.check_reserved_for_numparam(&value(&name_t), &self.loc(&name_t));

        let body_l = maybe_node_expr(&body)
            .unwrap_or_else(|| unreachable!("endless method always has a body"));

        let keyword_l = self.loc(&def_t);
        let expression_l = keyword_l.join(&body_l);
        let name_l = self.loc(&name_t);
        let assignment_l = self.loc(&assignment_t);

        Node::Def(Def {
            name: value(&name_t),
            args: args.map(|node| Box::new(node)),
            body: body.map(|node| Box::new(node)),
            keyword_l,
            name_l,
            assignment_l: Some(assignment_l),
            end_l: None,
            expression_l,
        })
    }

    pub(crate) fn def_singleton(
        &self,
        def_t: Token,
        definee: Node,
        dot_t: Token,
        name_t: Token,
        args: Option<Node>,
        body: Option<Node>,
        end_t: Token,
    ) -> Node {
        self.validate_definee(&definee);
        self.check_reserved_for_numparam(&value(&name_t), &self.loc(&name_t));

        let keyword_l = self.loc(&def_t);
        let operator_l = self.loc(&dot_t);
        let name_l = self.loc(&name_t);
        let end_l = self.loc(&end_t);
        let expression_l = keyword_l.join(&end_l);

        Node::Defs(Defs {
            definee: Box::new(definee),
            name: value(&name_t),
            args: args.map(|node| Box::new(node)),
            body: body.map(|node| Box::new(node)),
            keyword_l,
            operator_l,
            name_l,
            assignment_l: None,
            end_l: Some(end_l),
            expression_l,
        })
    }

    pub(crate) fn def_endless_singleton(
        &self,
        def_t: Token,
        definee: Node,
        dot_t: Token,
        name_t: Token,
        args: Option<Node>,
        assignment_t: Token,
        body: Option<Node>,
    ) -> Node {
        self.validate_definee(&definee);
        self.check_reserved_for_numparam(&value(&name_t), &self.loc(&name_t));

        let body_l = maybe_node_expr(&body)
            .unwrap_or_else(|| unreachable!("endless method always has body"));

        let keyword_l = self.loc(&def_t);
        let operator_l = self.loc(&dot_t);
        let name_l = self.loc(&name_t);
        let assignment_l = self.loc(&assignment_t);
        let expression_l = keyword_l.join(&body_l);

        Node::Defs(Defs {
            definee: Box::new(definee),
            name: value(&name_t),
            args: args.map(|node| Box::new(node)),
            body: body.map(|node| Box::new(node)),
            keyword_l,
            operator_l,
            name_l,
            assignment_l: Some(assignment_l),
            end_l: None,
            expression_l,
        })
    }

    pub(crate) fn undef_method(&self, undef_t: Token, names: Vec<Node>) -> Node {
        let keyword_l = self.loc(&undef_t);
        let expression_l = keyword_l.maybe_join(&collection_expr(&names));
        Node::Undef(Undef {
            names,
            keyword_l,
            expression_l,
        })
    }

    pub(crate) fn alias(&self, alias_t: Token, to: Node, from: Node) -> Node {
        let keyword_l = self.loc(&alias_t);
        let expression_l = keyword_l.join(from.expression());
        Node::Alias(Alias {
            to: Box::new(to),
            from: Box::new(from),
            keyword_l,
            expression_l,
        })
    }

    //
    // Formal arguments
    //

    pub(crate) fn args(
        &self,
        begin_t: Option<Token>,
        args: Vec<Node>,
        end_t: Option<Token>,
    ) -> Option<Node> {
        self.check_duplicate_args(&args, &mut HashMap::new());

        if begin_t.is_none() && args.is_empty() && end_t.is_none() {
            return None;
        }

        let (begin_l, end_l, expression_l) = self.collection_map(&begin_t, &args, &end_t);
        Some(Node::Args(Args {
            args,
            begin_l,
            end_l,
            expression_l,
        }))
    }

    pub(crate) fn forward_only_args(&self, begin_t: Token, dots_t: Token, end_t: Token) -> Node {
        let args = vec![self.forward_arg(dots_t)];
        let begin_l = self.loc(&begin_t);
        let end_l = self.loc(&end_t);
        let expression_l = begin_l.join(&end_l);
        Node::Args(Args {
            args,
            begin_l: Some(begin_l),
            end_l: Some(end_l),
            expression_l,
        })
    }

    pub(crate) fn forward_arg(&self, dots_t: Token) -> Node {
        Node::ForwardArg(ForwardArg {
            expression_l: self.loc(&dots_t),
        })
    }

    pub(crate) fn arg(&self, name_t: Token) -> Node {
        self.check_reserved_for_numparam(&value(&name_t), &self.loc(&name_t));
        Node::Arg(Arg {
            name: value(&name_t),
            expression_l: self.loc(&name_t),
        })
    }

    pub(crate) fn optarg(&self, name_t: Token, eql_t: Token, default: Node) -> Node {
        self.check_reserved_for_numparam(&value(&name_t), &self.loc(&name_t));

        let operator_l = self.loc(&eql_t);
        let name_l = self.loc(&name_t);
        let expression_l = self.loc(&name_t).join(default.expression());

        Node::Optarg(Optarg {
            name: value(&name_t),
            default: Box::new(default),
            name_l,
            operator_l,
            expression_l,
        })
    }

    pub(crate) fn restarg(&self, star_t: Token, name_t: Option<Token>) -> Node {
        let name = match &name_t {
            Some(name_t) => {
                self.check_reserved_for_numparam(&value(name_t), &self.loc(name_t));
                Some(value(name_t))
            }
            _ => None,
        };

        let star_l = self.loc(&star_t);
        let name_l = self.maybe_loc(&name_t);
        let expression_l = star_l.maybe_join(&name_l);

        Node::Restarg(Restarg {
            name,
            star_l,
            name_l,
            expression_l,
        })
    }

    pub(crate) fn kwarg(&self, name_t: Token) -> Node {
        self.check_reserved_for_numparam(&value(&name_t), &self.loc(&name_t));

        let expression_l = self.loc(&name_t);
        let name_l = expression_l.adjust_end(-1);

        Node::Kwarg(Kwarg {
            name: value(&name_t),
            name_l,
            expression_l,
        })
    }

    pub(crate) fn kwoptarg(&self, name_t: Token, default: Node) -> Node {
        self.check_reserved_for_numparam(&value(&name_t), &self.loc(&name_t));

        let label_l = self.loc(&name_t);
        let name_l = label_l.adjust_end(-1);
        let expression_l = default.expression().join(&label_l);

        Node::Kwoptarg(Kwoptarg {
            name: value(&name_t),
            default: Box::new(default),
            name_l,
            expression_l,
        })
    }

    pub(crate) fn kwrestarg(&self, dstar_t: Token, name_t: Option<Token>) -> Node {
        let name = match &name_t {
            Some(name_t) => {
                self.check_reserved_for_numparam(&value(name_t), &self.loc(name_t));
                Some(value(name_t))
            }
            _ => None,
        };

        let dstar_l = self.loc(&dstar_t);
        let name_l = self.maybe_loc(&name_t);
        let expression_l = dstar_l.maybe_join(&name_l);

        Node::Kwrestarg(Kwrestarg {
            name,
            dstar_l,
            name_l,
            expression_l,
        })
    }

    pub(crate) fn kwnilarg(&self, dstar_t: Token, nil_t: Token) -> Node {
        let dstar_l = self.loc(&dstar_t);
        let nil_l = self.loc(&nil_t);
        let expression_l = dstar_l.join(&nil_l);
        Node::Kwnilarg(Kwnilarg {
            name_l: nil_l,
            expression_l,
        })
    }

    pub(crate) fn shadowarg(&self, name_t: Token) -> Node {
        self.check_reserved_for_numparam(&value(&name_t), &self.loc(&name_t));
        Node::Shadowarg(Shadowarg {
            name: value(&name_t),
            expression_l: self.loc(&name_t),
        })
    }

    pub(crate) fn blockarg(&self, amper_t: Token, name_t: Token) -> Node {
        self.check_reserved_for_numparam(&value(&name_t), &self.loc(&name_t));

        let amper_l = self.loc(&amper_t);
        let name_l = self.loc(&name_t);
        let expression_l = amper_l.join(&name_l);

        Node::Blockarg(Blockarg {
            name: value(&name_t),
            amper_l,
            name_l,
            expression_l,
        })
    }

    pub(crate) fn procarg0(&self, arg: Node) -> Node {
        match arg {
            Node::Mlhs(Mlhs {
                items,
                begin_l,
                end_l,
                expression_l,
            }) => Node::Procarg0(Procarg0 {
                args: items,
                begin_l,
                end_l,
                expression_l,
            }),
            Node::Arg(arg) => Node::Procarg0(Procarg0 {
                expression_l: arg.expression_l.clone(),
                args: vec![Node::Arg(arg)],
                begin_l: None,
                end_l: None,
            }),
            other => unreachable!("unsupported procarg0 child {:?}", other),
        }
    }

    //
    // Method calls
    //

    fn call_type_for_dot(&self, dot_t: &Option<Token>) -> MethodCallType {
        match dot_t {
            Some(Token {
                token_type: Lexer::tANDDOT,
                ..
            }) => MethodCallType::CSend,
            _ => MethodCallType::Send,
        }
    }

    pub(crate) fn forwarded_args(&self, dots_t: Token) -> Node {
        Node::ForwardedArgs(ForwardedArgs {
            expression_l: self.loc(&dots_t),
        })
    }

    pub(crate) fn call_method(
        &self,
        receiver: Option<Node>,
        dot_t: Option<Token>,
        selector_t: Option<Token>,
        lparen_t: Option<Token>,
        args: Vec<Node>,
        rparen_t: Option<Token>,
    ) -> Node {
        let begin_l = maybe_node_expr(&receiver)
            .or_else(|| self.maybe_loc(&selector_t))
            .unwrap_or_else(|| unreachable!("can't compute begin_l"));
        let end_l = self
            .maybe_loc(&rparen_t)
            .or_else(|| maybe_node_expr(&args.last().cloned()))
            .or_else(|| self.maybe_loc(&selector_t))
            .unwrap_or_else(|| unreachable!("can't compute end_l"));

        let expression_l = begin_l.join(&end_l);

        let dot_l = self.maybe_loc(&dot_t);
        let selector_l = self.maybe_loc(&selector_t);
        let begin_l = self.maybe_loc(&lparen_t);
        let end_l = self.maybe_loc(&rparen_t);

        let method_name = maybe_value(&selector_t).unwrap_or_else(|| "call".to_owned());

        match self.call_type_for_dot(&dot_t) {
            MethodCallType::Send => Node::Send(Send {
                method_name,
                recv: receiver.map(Box::new),
                args,
                dot_l,
                selector_l,
                begin_l,
                end_l,
                operator_l: None,
                expression_l,
            }),

            MethodCallType::CSend => Node::CSend(CSend {
                method_name,
                recv: Box::new(receiver.unwrap()),
                args,
                dot_l,
                selector_l,
                begin_l,
                end_l,
                operator_l: None,
                expression_l,
            }),
        }
    }

    pub(crate) fn call_lambda(&self, lambda_t: Token) -> Node {
        Node::Lambda(Lambda {
            expression_l: self.loc(&lambda_t),
        })
    }

    pub(crate) fn block(
        &self,
        method_call: Node,
        begin_t: Token,
        block_args: ArgsType,
        body: Option<Node>,
        end_t: Token,
    ) -> Node {
        // let block_args = args.map(|node| Box::new(node));
        let block_body = body.map(|node| Box::new(node));

        match &method_call {
            Node::Yield { .. } => {
                // diagnostic :error, :block_given_to_yield, nil, method_call.loc.keyword, [loc(begin_t)]
            }
            Node::Send(Send { args, .. }) | Node::CSend(CSend { args, .. }) => {
                match args.last() {
                    Some(Node::Blockarg(Blockarg { .. }))
                    | Some(Node::ForwardedArgs(ForwardedArgs { .. })) => {
                        // diagnostic :error, :block_and_blockarg, nil, expression, [loc(begin_t)]
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        let rewrite_args_and_loc =
            |method_args: &Vec<Node>,
             keyword_expression_l: &Range,
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
                        args: args.map(Box::new),
                        body: block_body,
                        begin_l,
                        end_l,
                        expression_l,
                    }),
                    ArgsType::Numargs(numargs) => Node::Numblock(Numblock {
                        call: Box::new(actual_send),
                        numargs,
                        body: block_body.unwrap(),
                        begin_l,
                        end_l,
                        expression_l,
                    }),
                };

                let expr_l = keyword_expression_l.join(block.expression());
                let args = vec![block];

                (args, expr_l)
            };

        match &method_call {
            Node::Send(Send { .. })
            | Node::CSend(CSend { .. })
            | Node::Index(Index { .. })
            | Node::Super(Super { .. })
            | Node::ZSuper(ZSuper { .. })
            | Node::Lambda(Lambda { .. }) => {
                let begin_l = self.loc(&begin_t);
                let end_l = self.loc(&end_t);
                let expression_l = method_call.expression().join(&end_l);

                match block_args {
                    ArgsType::Args(args) => Node::Block(Block {
                        call: Box::new(method_call),
                        args: args.map(Box::new),
                        body: block_body,
                        begin_l,
                        end_l,
                        expression_l,
                    }),
                    ArgsType::Numargs(numargs) => Node::Numblock(Numblock {
                        numargs,
                        call: Box::new(method_call),
                        body: block_body.unwrap(),
                        begin_l,
                        end_l,
                        expression_l,
                    }),
                }
            }
            Node::Return(Return {
                args,
                keyword_l,
                expression_l,
            }) => {
                let (args, expression_l) =
                    rewrite_args_and_loc(args, expression_l, block_args, block_body);
                Node::Return(Return {
                    args,
                    keyword_l: keyword_l.clone(),
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
                    keyword_l: keyword_l.clone(),
                    expression_l,
                })
            }
            Node::Break(Break {
                args,
                keyword_l,
                begin_l,
                end_l,
                expression_l,
            }) => {
                let (args, expression_l) =
                    rewrite_args_and_loc(args, expression_l, block_args, block_body);
                Node::Break(Break {
                    args,
                    keyword_l: keyword_l.clone(),
                    begin_l: begin_l.clone(),
                    end_l: end_l.clone(),
                    expression_l,
                })
            }
            _ => unreachable!("unsupported method call {:?}", method_call),
        }
    }
    pub(crate) fn block_pass(&self, amper_t: Token, value: Node) -> Node {
        let amper_l = self.loc(&amper_t);
        let expression_l = value.expression().join(&amper_l);

        Node::BlockPass(BlockPass {
            value: Box::new(value),
            operator_l: amper_l,
            expression_l,
        })
    }

    pub(crate) fn attr_asgn(&self, receiver: Node, dot_t: Token, selector_t: Token) -> Node {
        let method_name = value(&selector_t) + "=";

        let dot_l = self.loc(&dot_t);
        let selector_l = self.loc(&selector_t);
        let expression_l = receiver.expression().join(&selector_l);

        match self.call_type_for_dot(&Some(dot_t)) {
            MethodCallType::Send => Node::Send(Send {
                method_name,
                recv: Some(Box::new(receiver)),
                args: vec![],
                dot_l: Some(dot_l),
                selector_l: Some(selector_l),
                begin_l: None,
                end_l: None,
                operator_l: None,
                expression_l,
            }),

            MethodCallType::CSend => Node::CSend(CSend {
                method_name,
                recv: Box::new(receiver),
                args: vec![],
                dot_l: Some(dot_l),
                selector_l: Some(selector_l),
                begin_l: None,
                end_l: None,
                operator_l: None,
                expression_l,
            }),
        }
    }

    pub(crate) fn index(
        &self,
        recv: Node,
        lbrack_t: Token,
        indexes: Vec<Node>,
        rbrack_t: Token,
    ) -> Node {
        let begin_l = self.loc(&lbrack_t);
        let end_l = self.loc(&rbrack_t);
        let expression_l = recv.expression().join(&end_l);

        Node::Index(Index {
            recv: Box::new(recv),
            indexes,
            begin_l,
            end_l,
            expression_l,
        })
    }

    pub(crate) fn index_asgn(
        &self,
        recv: Node,
        lbrack_t: Token,
        indexes: Vec<Node>,
        rbrack_t: Token,
    ) -> Node {
        let begin_l = self.loc(&lbrack_t);
        let end_l = self.loc(&rbrack_t);
        let expression_l = recv.expression().join(&end_l);

        Node::IndexAsgn(IndexAsgn {
            recv: Box::new(recv),
            indexes,
            value: None,
            begin_l,
            end_l,
            operator_l: None,
            expression_l,
        })
    }

    pub(crate) fn binary_op(&self, receiver: Node, operator_t: Token, arg: Node) -> Node {
        let selector_l = self.loc(&operator_t);
        let expression_l = join_exprs(&receiver, &arg);

        Node::Send(Send {
            recv: Some(Box::new(receiver)),
            method_name: value(&operator_t),
            args: vec![arg],
            dot_l: None,
            selector_l: Some(selector_l),
            begin_l: None,
            end_l: None,
            operator_l: None,
            expression_l,
        })
    }

    pub(crate) fn match_op(&self, receiver: Node, match_t: Token, arg: Node) -> Node {
        let selector_l = self.loc(&match_t);
        let expression_l = join_exprs(&receiver, &arg);

        match self.static_regexp(&receiver) {
            Some(regex) => {
                let captures = self.static_regexp_captures(&regex);
                for capture in captures {
                    self.static_env.declare(&capture);
                }

                Node::MatchWithLvasgn(MatchWithLvasgn {
                    re: Box::new(receiver),
                    value: Box::new(arg),
                    operator_l: selector_l,
                    expression_l,
                })
            }
            None => Node::Send(Send {
                recv: Some(Box::new(receiver)),
                method_name: String::from("=~"),
                args: vec![arg],
                dot_l: None,
                selector_l: Some(selector_l),
                begin_l: None,
                end_l: None,
                operator_l: None,
                expression_l,
            }),
        }
    }

    pub(crate) fn unary_op(&self, op_t: Token, receiver: Node) -> Node {
        let selector_l = self.loc(&op_t);
        let expression_l = receiver.expression().join(&selector_l);

        let op = value(&op_t);
        let method = if op == "+" || op == "-" { op + "@" } else { op };
        Node::Send(Send {
            recv: Some(Box::new(receiver)),
            method_name: method,
            args: vec![],
            dot_l: None,
            selector_l: Some(selector_l),
            begin_l: None,
            end_l: None,
            operator_l: None,
            expression_l,
        })
    }

    pub(crate) fn not_op(
        &self,
        not_t: Token,
        begin_t: Option<Token>,
        receiver: Option<Node>,
        end_t: Option<Token>,
    ) -> Node {
        if let Some(receiver) = receiver {
            let begin_l = self.loc(&not_t);
            let end_l = self
                .maybe_loc(&end_t)
                .unwrap_or_else(|| receiver.expression().clone());

            let expression_l = begin_l.join(&end_l);

            let selector_l = self.loc(&not_t);
            let begin_l = self.maybe_loc(&begin_t);
            let end_l = self.maybe_loc(&end_t);

            Node::Send(Send {
                recv: Some(Box::new(self.check_condition(receiver))),
                method_name: "!".to_owned(),
                args: vec![],
                selector_l: Some(selector_l),
                dot_l: None,
                begin_l,
                end_l,
                operator_l: None,
                expression_l,
            })
        } else {
            let (begin_l, end_l, expression_l) = self.collection_map(&begin_t, &vec![], &end_t);
            let nil_node = Node::Begin(Begin {
                statements: vec![],
                begin_l,
                end_l,
                expression_l,
            });

            let selector_l = self.loc(&not_t);
            let expression_l = nil_node.expression().join(&selector_l);
            Node::Send(Send {
                recv: Some(Box::new(nil_node)),
                method_name: "!".to_owned(),
                args: vec![],
                selector_l: Some(selector_l),
                dot_l: None,
                begin_l: None,
                end_l: None,
                operator_l: None,
                expression_l,
            })
        }
    }

    //
    // Control flow
    //

    // Logical operations: and, or

    pub(crate) fn logical_op(&self, type_: LogicalOp, lhs: Node, op_t: Token, rhs: Node) -> Node {
        let operator_l = self.loc(&op_t);
        let expression_l = join_exprs(&lhs, &rhs);

        let lhs = Box::new(lhs);
        let rhs = Box::new(rhs);
        match type_ {
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
        }
    }

    // Conditionals

    pub(crate) fn condition(
        &self,
        cond_t: Token,
        cond: Node,
        then_t: Token,
        if_true: Option<Node>,
        else_t: Option<Token>,
        if_false: Option<Node>,
        end_t: Option<Token>,
    ) -> Node {
        let end_l = self
            .maybe_loc(&end_t)
            .or_else(|| maybe_node_expr(&if_false))
            .or_else(|| self.maybe_loc(&else_t))
            .or_else(|| maybe_node_expr(&if_true))
            .unwrap_or_else(|| self.loc(&then_t));

        let expression_l = self.loc(&cond_t).join(&end_l);
        let keyword_l = self.loc(&cond_t);
        let begin_l = self.loc(&then_t);
        let else_l = self.maybe_loc(&else_t);
        let end_l = self.maybe_loc(&end_t);

        Node::If(If {
            cond: Box::new(self.check_condition(cond)),
            if_true: if_true.map(|node| Box::new(node)),
            if_false: if_false.map(|node| Box::new(node)),
            keyword_l,
            begin_l,
            else_l,
            end_l,
            expression_l,
        })
    }

    pub(crate) fn condition_mod(
        &self,
        if_true: Option<Node>,
        if_false: Option<Node>,
        cond_t: Token,
        cond: Node,
    ) -> Node {
        let pre = match (&if_true, &if_false) {
            (None, None) => panic!("at least one of if_true/if_false is required"),
            (None, Some(if_false)) => if_false.clone(),
            (Some(if_true), None) => if_true.clone(),
            (Some(_), Some(_)) => panic!("only one of if_true/if_false is required"),
        };

        let expression_l = pre.expression().join(&cond.expression());
        let keyword_l = self.loc(&cond_t);

        Node::IfMod(IfMod {
            cond: Box::new(self.check_condition(cond)),
            if_true: if_true.map(|node| Box::new(node)),
            if_false: if_false.map(|node| Box::new(node)),
            keyword_l,
            expression_l,
        })
    }

    pub(crate) fn ternary(
        &self,
        cond: Node,
        question_t: Token,
        if_true: Node,
        colon_t: Token,
        if_false: Node,
    ) -> Node {
        let expression_l = join_exprs(&cond, &if_false);
        let question_l = self.loc(&question_t);
        let colon_l = self.loc(&colon_t);

        Node::IfTernary(IfTernary {
            cond: Box::new(cond),
            if_true: Box::new(if_true),
            if_false: Box::new(if_false),
            question_l,
            colon_l,
            expression_l,
        })
    }

    // Case matching

    pub(crate) fn when(
        &self,
        when_t: Token,
        patterns: Vec<Node>,
        then_t: Token,
        body: Option<Node>,
    ) -> Node {
        let begin_l = self.loc(&then_t);

        let expr_end_l = maybe_node_expr(&body)
            .or_else(|| maybe_node_expr(&patterns.last().cloned()))
            .unwrap_or_else(|| self.loc(&when_t));
        let when_l = self.loc(&when_t);
        let expression_l = when_l.join(&expr_end_l);

        Node::When(When {
            patterns,
            body: body.map(Box::new),
            keyword_l: when_l,
            begin_l,
            expression_l,
        })
    }

    pub(crate) fn case(
        &self,
        case_t: Token,
        expr: Option<Node>,
        when_bodies: Vec<Node>,
        else_t: Option<Token>,
        else_body: Option<Node>,
        end_t: Token,
    ) -> Node {
        let keyword_l = self.loc(&case_t);
        let else_l = self.maybe_loc(&else_t);
        let end_l = self.loc(&end_t);
        let expression_l = keyword_l.join(&end_l);

        Node::Case(Case {
            expr: expr.map(Box::new),
            when_bodies,
            else_body: else_body.map(Box::new),
            keyword_l,
            else_l,
            end_l,
            expression_l,
        })
    }

    // Loops

    pub(crate) fn loop_(
        &self,
        loop_type: LoopType,
        keyword_t: Token,
        cond: Node,
        do_t: Token,
        body: Option<Node>,
        end_t: Token,
    ) -> Node {
        let keyword_l = self.loc(&keyword_t);
        let begin_l = self.loc(&do_t);
        let end_l = self.loc(&end_t);
        let expression_l = self.loc(&keyword_t).join(&end_l);

        let cond = Box::new(self.check_condition(cond));
        let body = body.map(Box::new);

        match loop_type {
            LoopType::While => Node::While(While {
                cond,
                body,
                keyword_l,
                begin_l: Some(begin_l),
                end_l: Some(end_l),
                expression_l,
            }),
            LoopType::Until => Node::Until(Until {
                cond,
                body,
                keyword_l,
                begin_l: Some(begin_l),
                end_l: Some(end_l),
                expression_l,
            }),
        }
    }

    pub(crate) fn loop_mod(
        &self,
        loop_type: LoopType,
        body: Node,
        keyword_t: Token,
        cond: Node,
    ) -> Node {
        let expression_l = body.expression().join(&cond.expression());
        let keyword_l = self.loc(&keyword_t);

        let cond = Box::new(self.check_condition(cond));

        match (loop_type, &body) {
            (LoopType::While, Node::KwBegin(KwBegin { .. })) => Node::WhilePost(WhilePost {
                cond,
                body: Box::new(body),
                keyword_l,
                expression_l,
            }),
            (LoopType::While, _) => Node::While(While {
                cond,
                body: Some(Box::new(body)),
                keyword_l,
                expression_l,
                begin_l: None,
                end_l: None,
            }),
            (LoopType::Until, Node::KwBegin(KwBegin { .. })) => Node::UntilPost(UntilPost {
                cond,
                body: Box::new(body),
                keyword_l,
                expression_l,
            }),
            (LoopType::Until, _) => Node::Until(Until {
                cond,
                body: Some(Box::new(body)),
                keyword_l,
                expression_l,
                begin_l: None,
                end_l: None,
            }),
        }
    }

    pub(crate) fn for_(
        &self,
        for_t: Token,
        iterator: Node,
        in_t: Token,
        iteratee: Node,
        do_t: Token,
        body: Option<Node>,
        end_t: Token,
    ) -> Node {
        let keyword_l = self.loc(&for_t);
        let in_l = self.loc(&in_t);
        let begin_l = self.loc(&do_t);
        let end_l = self.loc(&end_t);
        let expression_l = keyword_l.join(&end_l);

        Node::For(For {
            iterator: Box::new(iterator),
            iteratee: Box::new(iteratee),
            body: body.map(Box::new),
            keyword_l,
            in_l,
            begin_l,
            end_l,
            expression_l,
        })
    }

    // Keywords

    pub(crate) fn keyword_cmd(
        &self,
        type_: KeywordCmd,
        keyword_t: Token,
        lparen_t: Option<Token>,
        mut args: Vec<Node>,
        rparen_t: Option<Token>,
    ) -> Node {
        if type_ == KeywordCmd::Yield && !args.is_empty() {
            match args.last() {
                Some(Node::BlockPass(_)) => {
                    // diagnostic :error, :block_given_to_yield, nil, self.loc(keyword_t), [last_arg.loc.expression]
                }
                _ => {}
            }
        }

        let keyword_l = self.loc(&keyword_t);
        let begin_l = self.maybe_loc(&lparen_t);
        let end_l = self.maybe_loc(&rparen_t);

        let expr_end_l = end_l
            .clone()
            .or_else(|| maybe_node_expr(&args.last().cloned()))
            .unwrap_or_else(|| keyword_l.clone());

        let expression_l = keyword_l.join(&expr_end_l);

        match type_ {
            KeywordCmd::Break => Node::Break(Break {
                args,
                keyword_l,
                begin_l,
                end_l,
                expression_l,
            }),
            KeywordCmd::Defined => Node::Defined(Defined {
                value: Box::new(args.pop().unwrap()),
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
        }
    }

    // BEGIN, END

    pub(crate) fn preexe(
        &self,
        preexe_t: Token,
        lbrace_t: Token,
        compstmt: Option<Node>,
        rbrace_t: Token,
    ) -> Node {
        let keyword_l = self.loc(&preexe_t);
        let begin_l = self.loc(&lbrace_t);
        let end_l = self.loc(&rbrace_t);
        let expression_l = keyword_l.join(&end_l);

        Node::Preexe(Preexe {
            body: compstmt.map(Box::new),
            keyword_l,
            begin_l,
            end_l,
            expression_l,
        })
    }
    pub(crate) fn postexe(
        &self,
        postexe_t: Token,
        lbrace_t: Token,
        compstmt: Option<Node>,
        rbrace_t: Token,
    ) -> Node {
        let keyword_l = self.loc(&postexe_t);
        let begin_l = self.loc(&lbrace_t);
        let end_l = self.loc(&rbrace_t);
        let expression_l = keyword_l.join(&end_l);

        Node::Postexe(Postexe {
            body: compstmt.map(Box::new),
            keyword_l,
            begin_l,
            end_l,
            expression_l,
        })
    }

    // Exception handling

    pub(crate) fn rescue_body(
        &self,
        rescue_t: Token,
        exc_list: Option<Node>,
        assoc_t: Option<Token>,
        exc_var: Option<Node>,
        then_t: Option<Token>,
        body: Option<Node>,
    ) -> Node {
        let end_l = maybe_node_expr(&body)
            .or_else(|| self.maybe_loc(&then_t))
            .or_else(|| maybe_node_expr(&exc_var))
            .or_else(|| maybe_node_expr(&exc_list))
            .unwrap_or_else(|| self.loc(&rescue_t));

        let expression_l = self.loc(&rescue_t).join(&end_l);
        let keyword_l = self.loc(&rescue_t);
        let assoc_l = self.maybe_loc(&assoc_t);
        let begin_l = self.maybe_loc(&then_t);

        Node::RescueBody(RescueBody {
            exc_list: exc_list.map(Box::new),
            exc_var: exc_var.map(Box::new),
            body: body.map(Box::new),
            keyword_l,
            begin_l,
            assoc_l,
            expression_l,
        })
    }

    pub(crate) fn begin_body(
        &self,
        compound_stmt: Option<Node>,
        rescue_bodies: Vec<Node>,
        else_: Option<(Token, Option<Node>)>,
        ensure: Option<(Token, Option<Node>)>,
    ) -> Option<Node> {
        let mut result: Option<Node>;

        if !rescue_bodies.is_empty() {
            if let Some((else_t, else_)) = else_ {
                let begin_l = maybe_node_expr(&compound_stmt)
                    .or_else(|| maybe_node_expr(&rescue_bodies.first().cloned()))
                    .unwrap_or_else(|| unreachable!("can't compute begin_l"));

                let end_l = maybe_node_expr(&else_).unwrap_or_else(|| self.loc(&else_t));

                let expression_l = begin_l.join(&end_l);
                let else_l = self.loc(&else_t);

                result = Some(Node::Rescue(Rescue {
                    body: compound_stmt.map(|node| Box::new(node)),
                    rescue_bodies,
                    else_: else_.map(Box::new),
                    else_l: Some(else_l),
                    expression_l,
                }))
            } else {
                let begin_l = maybe_node_expr(&compound_stmt)
                    .or_else(|| maybe_node_expr(&rescue_bodies.first().cloned()))
                    .unwrap_or_else(|| unreachable!("can't compute begin_l"));

                let end_l = maybe_node_expr(&rescue_bodies.last().cloned())
                    .unwrap_or_else(|| unreachable!("can't compute end_l"));

                let expression_l = begin_l.join(&end_l);
                let else_l = self.maybe_loc(&None);

                result = Some(Node::Rescue(Rescue {
                    body: compound_stmt.map(|node| Box::new(node)),
                    rescue_bodies,
                    else_: None,
                    else_l,
                    expression_l,
                }))
            }
        } else if let Some((else_t, else_)) = else_ {
            let mut statements: Vec<Node> = vec![];

            match compound_stmt {
                Some(Node::Begin(Begin {
                    statements: begin_statements,
                    ..
                })) => statements = begin_statements,
                Some(compound_stmt) => statements.push(compound_stmt),
                _ => {}
            }
            let parts = if let Some(else_) = else_ {
                vec![else_]
            } else {
                vec![]
            };
            let (begin_l, end_l, expression_l) = self.collection_map(&Some(else_t), &parts, &None);
            statements.push(Node::Begin(Begin {
                statements: parts,
                begin_l,
                end_l,
                expression_l,
            }));

            let (begin_l, end_l, expression_l) = self.collection_map(&None, &statements, &None);
            result = Some(Node::Begin(Begin {
                statements,
                begin_l,
                end_l,
                expression_l,
            }))
        } else {
            result = compound_stmt;
        }

        if let Some((ensure_t, ensure)) = ensure {
            let mut ensure_body = if let Some(ensure) = ensure {
                vec![ensure]
            } else {
                vec![]
            };
            let keyword_l = self.loc(&ensure_t);

            let begin_l = maybe_node_expr(&result).unwrap_or_else(|| self.loc(&ensure_t));

            let end_l = maybe_node_expr(&ensure_body.last().cloned())
                .unwrap_or_else(|| self.loc(&ensure_t));

            let expression_l = begin_l.join(&end_l);

            result = Some(Node::Ensure(Ensure {
                body: result.map(|node| Box::new(node)),
                ensure: ensure_body.pop().map(Box::new),
                keyword_l,
                expression_l,
            }))
        }

        result
    }

    //
    // Expression grouping
    //

    pub(crate) fn compstmt(&self, mut statements: Vec<Node>) -> Option<Node> {
        match &statements[..] {
            [] => None,
            [_] => statements.pop(),
            _ => {
                let (begin_l, end_l, expression_l) = self.collection_map(&None, &statements, &None);
                Some(Node::Begin(Begin {
                    statements,
                    begin_l,
                    end_l,
                    expression_l,
                }))
            }
        }
    }

    pub(crate) fn begin(&self, begin_t: Token, body: Option<Node>, end_t: Token) -> Node {
        let begin_l = self.loc(&begin_t);
        let end_l = self.loc(&end_t);
        let expression_l = begin_l.join(&end_l);

        if let Some(body) = body {
            match body {
                // Synthesized (begin) from compstmt "a; b" or (mlhs)
                // from multi_lhs "(a, b) = *foo".
                Node::Mlhs(Mlhs { items, .. }) => Node::Mlhs(Mlhs {
                    items,
                    begin_l: Some(begin_l),
                    end_l: Some(end_l),
                    expression_l,
                }),
                Node::Begin(Begin {
                    statements,
                    begin_l: None,
                    end_l: None,
                    ..
                }) => Node::Begin(Begin {
                    statements,
                    begin_l: Some(begin_l),
                    end_l: Some(end_l),
                    expression_l,
                }),
                body => {
                    let statements = vec![body];
                    Node::Begin(Begin {
                        statements,
                        begin_l: Some(begin_l),
                        end_l: Some(end_l),
                        expression_l,
                    })
                }
            }
        } else {
            // A nil expression: `()'.
            Node::Begin(Begin {
                statements: vec![],
                begin_l: Some(begin_l),
                end_l: Some(end_l),
                expression_l,
            })
        }
    }

    pub(crate) fn begin_keyword(&self, begin_t: Token, body: Option<Node>, end_t: Token) -> Node {
        let begin_l = self.loc(&begin_t);
        let end_l = self.loc(&end_t);
        let expression_l = begin_l.join(&end_l);

        match body {
            None => {
                // A nil expression: `begin end'.
                Node::KwBegin(KwBegin {
                    statements: vec![],
                    begin_l: Some(begin_l),
                    end_l: Some(end_l),
                    expression_l,
                })
            }
            Some(Node::Begin(Begin {
                statements,
                begin_l: None,
                end_l: None,
                ..
            })) => {
                // Synthesized (begin) from compstmt "a; b".
                Node::KwBegin(KwBegin {
                    statements,
                    begin_l: Some(begin_l),
                    end_l: Some(end_l),
                    expression_l,
                })
            }
            Some(node) => {
                let statements = vec![node];
                Node::KwBegin(KwBegin {
                    statements,
                    begin_l: Some(begin_l),
                    end_l: Some(end_l),
                    expression_l,
                })
            }
        }
    }

    //
    // Pattern matching
    //

    pub(crate) fn case_match(
        &self,
        case_t: Token,
        expr: Node,
        in_bodies: Vec<Node>,
        else_t: Option<Token>,
        else_body: Option<Node>,
        end_t: Token,
    ) -> Node {
        let else_body = match (&else_t, &else_body) {
            (Some(else_t), None) => Some(Node::EmptyElse(EmptyElse {
                expression_l: self.loc(else_t),
            })),
            _ => else_body,
        };

        let keyword_l = self.loc(&case_t);
        let else_l = self.maybe_loc(&else_t);
        let end_l = self.loc(&end_t);
        let expression_l = self.loc(&case_t).join(&end_l);

        Node::CaseMatch(CaseMatch {
            expr: Box::new(expr),
            in_bodies,
            else_body: else_body.map(Box::new),
            keyword_l,
            else_l,
            end_l,
            expression_l,
        })
    }

    pub(crate) fn in_match(&self, value: Node, in_t: Token, pattern: Node) -> Node {
        let keyword_l = self.loc(&in_t);
        let expression_l = join_exprs(&value, &pattern);

        Node::InMatch(InMatch {
            value: Box::new(value),
            pattern: Box::new(pattern),
            operator_l: keyword_l,
            expression_l,
        })
    }

    pub(crate) fn in_pattern(
        &self,
        in_t: Token,
        pattern: Node,
        guard: Option<Node>,
        then_t: Token,
        body: Option<Node>,
    ) -> Node {
        let keyword_l = self.loc(&in_t);
        let begin_l = self.loc(&then_t);

        let expression_l = maybe_node_expr(&body)
            .or_else(|| maybe_node_expr(&guard))
            .unwrap_or_else(|| pattern.expression().clone())
            .join(&keyword_l);

        Node::InPattern(InPattern {
            pattern: Box::new(pattern),
            guard: guard.map(Box::new),
            body: body.map(Box::new),
            keyword_l,
            begin_l,
            expression_l,
        })
    }

    pub(crate) fn if_guard(&self, if_t: Token, cond: Node) -> Node {
        let keyword_l = self.loc(&if_t);
        let expression_l = keyword_l.join(cond.expression());

        Node::IfGuard(IfGuard {
            cond: Box::new(cond),
            keyword_l,
            expression_l,
        })
    }
    pub(crate) fn unless_guard(&self, unless_t: Token, cond: Node) -> Node {
        let keyword_l = self.loc(&unless_t);
        let expression_l = keyword_l.join(cond.expression());

        Node::UnlessGuard(UnlessGuard {
            cond: Box::new(cond),
            keyword_l,
            expression_l,
        })
    }

    pub(crate) fn match_var(&self, name_t: Token) -> Node {
        let name = value(&name_t);
        let name_l = self.loc(&name_t);
        let expression_l = name_l.clone();

        self.check_lvar_name(&name, &name_l);
        self.check_duplicate_pattern_variable(&name, &name_l);
        self.static_env.declare(&name);

        Node::MatchVar(MatchVar {
            name,
            name_l,
            expression_l,
        })
    }

    pub(crate) fn match_hash_var(&self, name_t: Token) -> Node {
        let name = value(&name_t);

        let expression_l = self.loc(&name_t);
        let name_l = expression_l.adjust_end(-1);

        self.check_lvar_name(&name, &name_l);
        self.check_duplicate_pattern_variable(&name, &name_l);
        self.static_env.declare(&name);

        Node::MatchVar(MatchVar {
            name,
            name_l,
            expression_l,
        })
    }
    pub(crate) fn match_hash_var_from_str(
        &self,
        begin_t: Token,
        mut strings: Vec<Node>,
        end_t: Token,
    ) -> Node {
        if strings.len() != 1 {
            // diagnostic :error, :pm_interp_in_var_name, nil, self.loc(begin_t).join(self.loc(end_t))
        }

        match strings.remove(0) {
            Node::Str(Str {
                value,
                begin_l,
                end_l,
                expression_l,
            }) => {
                let name = value.to_string_lossy();
                let mut name_l = expression_l.clone();

                self.check_lvar_name(&name, &name_l);
                self.check_duplicate_pattern_variable(&name, &name_l);

                self.static_env.declare(&name);

                match &begin_l {
                    Some(begin_l) => {
                        let begin_pos_d: i32 = begin_l.size().try_into().unwrap();
                        name_l = name_l.adjust_begin(begin_pos_d)
                    }
                    _ => {}
                }

                match &end_l {
                    Some(end_l) => {
                        let end_pos_d: i32 = end_l.size().try_into().unwrap();
                        name_l = name_l.adjust_end(-end_pos_d)
                    }
                    _ => {}
                }

                let expression_l = self
                    .loc(&begin_t)
                    .join(&expression_l)
                    .join(&self.loc(&end_t));
                Node::MatchVar(MatchVar {
                    name,
                    name_l,
                    expression_l,
                })
            }
            Node::Begin(Begin { statements, .. }) => {
                self.match_hash_var_from_str(begin_t, statements, end_t)
            }
            _ => {
                // diagnostic :error, :pm_interp_in_var_name, nil, self.loc(begin_t).join(self.loc(end_t))
                panic!("missing diagnostic")
            }
        }
    }

    pub(crate) fn match_rest(&self, star_t: Token, name_t: Option<Token>) -> Node {
        let name = name_t.map(|t| self.match_var(t));

        let operator_l = self.loc(&star_t);
        let expression_l = operator_l.maybe_join(&maybe_node_expr(&name));

        Node::MatchRest(MatchRest {
            name: name.map(Box::new),
            operator_l,
            expression_l,
        })
    }

    pub(crate) fn hash_pattern(
        &self,
        lbrace_t: Option<Token>,
        kwargs: Vec<Node>,
        rbrace_t: Option<Token>,
    ) -> Node {
        self.check_duplicate_args(&kwargs, &mut HashMap::new());
        let (begin_l, end_l, expression_l) = self.collection_map(&lbrace_t, &kwargs, &rbrace_t);
        Node::HashPattern(HashPattern {
            elements: kwargs,
            begin_l,
            end_l,
            expression_l,
        })
    }

    pub(crate) fn array_pattern(
        &self,
        lbrack_t: Option<Token>,
        elements: Vec<Node>,
        rbrack_t: Option<Token>,
    ) -> Node {
        let (begin_l, end_l, expression_l) = self.collection_map(&lbrack_t, &elements, &rbrack_t);

        if elements.is_empty() {
            return Node::ArrayPattern(ArrayPattern {
                elements: vec![],
                begin_l,
                end_l,
                expression_l,
            });
        }

        let mut trailing_comma = false;
        let nodes_elements = elements
            .into_iter()
            .map(|element| match element {
                Node::MatchWithTrailingComma(MatchWithTrailingComma { match_, .. }) => {
                    trailing_comma = true;
                    *match_
                }
                e => {
                    trailing_comma = false;
                    e
                }
            })
            .collect::<Vec<_>>();

        if trailing_comma {
            Node::ArrayPatternWithTail(ArrayPatternWithTail {
                elements: nodes_elements,
                begin_l,
                end_l,
                expression_l,
            })
        } else {
            Node::ArrayPattern(ArrayPattern {
                elements: nodes_elements,
                begin_l,
                end_l,
                expression_l,
            })
        }
    }

    pub(crate) fn find_pattern(
        &self,
        lbrack_t: Option<Token>,
        elements: Vec<Node>,
        rbrack_t: Option<Token>,
    ) -> Node {
        let (begin_l, end_l, expression_l) = self.collection_map(&lbrack_t, &elements, &rbrack_t);
        Node::FindPattern(FindPattern {
            elements,
            begin_l,
            end_l,
            expression_l,
        })
    }

    pub(crate) fn match_with_trailing_comma(&self, match_: Node, comma_t: Token) -> Node {
        Node::MatchWithTrailingComma(MatchWithTrailingComma {
            expression_l: match_.expression().join(&self.loc(&comma_t)),
            match_: Box::new(match_),
        })
    }

    pub(crate) fn const_pattern(
        &self,
        const_: Node,
        ldelim_t: Token,
        pattern: Node,
        rdelim_t: Token,
    ) -> Node {
        let begin_l = self.loc(&ldelim_t);
        let end_l = self.loc(&rdelim_t);
        let expression_l = const_.expression().join(&self.loc(&rdelim_t));

        Node::ConstPattern(ConstPattern {
            const_: Box::new(const_),
            pattern: Box::new(pattern),
            begin_l,
            end_l,
            expression_l,
        })
    }

    pub(crate) fn pin(&self, pin_t: Token, var: Node) -> Node {
        let operator_l = self.loc(&pin_t);
        let expression_l = var.expression().join(&operator_l);

        Node::Pin(Pin {
            var: Box::new(var),
            selector_l: operator_l,
            expression_l,
        })
    }

    pub(crate) fn match_alt(&self, left: Node, pipe_t: Token, right: Node) -> Node {
        let operator_l = self.loc(&pipe_t);
        let expression_l = join_exprs(&left, &right);

        Node::MatchAlt(MatchAlt {
            lhs: Box::new(left),
            rhs: Box::new(right),
            operator_l,
            expression_l,
        })
    }

    pub(crate) fn match_as(&self, value: Node, assoc_t: Token, as_: Node) -> Node {
        let operator_l = self.loc(&assoc_t);
        let expression_l = join_exprs(&value, &as_);

        Node::MatchAs(MatchAs {
            value: Box::new(value),
            as_: Box::new(as_),
            operator_l,
            expression_l,
        })
    }

    pub(crate) fn match_nil_pattern(&self, dstar_t: Token, nil_t: Token) -> Node {
        let operator_l = self.loc(&dstar_t);
        let name_l = self.loc(&nil_t);
        let expression_l = operator_l.join(&name_l);

        Node::MatchNilPattern(MatchNilPattern {
            operator_l,
            name_l,
            expression_l,
        })
    }

    pub(crate) fn match_pair(&self, p_kw_label: PKwLabel, value_node: Node) -> Node {
        match p_kw_label {
            PKwLabel::PlainLabel(label_t) => {
                self.check_duplicate_pattern_key(&value(&label_t), &self.loc(&label_t));
                self.pair_keyword(label_t, value_node)
            }
            PKwLabel::QuotedLabel((begin_t, parts, end_t)) => {
                let label_loc = self.loc(&begin_t).join(&self.loc(&end_t));

                match self.static_string(&parts) {
                    Some(var_name) => self.check_duplicate_pattern_key(&var_name, &label_loc),
                    _ => {} /* diagnostic :error, :pm_interp_in_var_name, nil, label_loc */
                }

                self.pair_quoted(begin_t, parts, end_t, value_node)
            }
        }
    }

    pub(crate) fn match_label(&self, p_kw_label: PKwLabel) -> Node {
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

    pub(crate) fn check_condition(&self, cond: Node) -> Node {
        match cond {
            Node::Begin(Begin {
                statements,
                begin_l,
                end_l,
                expression_l,
            }) if statements.len() == 1 => {
                let stmt = statements[statements.len() - 1].clone();
                let stmt = self.check_condition(stmt);
                Node::Begin(Begin {
                    statements: vec![stmt],
                    begin_l,
                    end_l,
                    expression_l,
                })
            }
            Node::And(And {
                lhs,
                rhs,
                operator_l,
                expression_l,
            }) => Node::And(And {
                lhs: Box::new(self.check_condition(*lhs)),
                rhs: Box::new(self.check_condition(*rhs)),
                operator_l,
                expression_l,
            }),
            Node::Or(Or {
                lhs,
                rhs,
                operator_l,
                expression_l,
            }) => Node::Or(Or {
                lhs: Box::new(self.check_condition(*lhs)),
                rhs: Box::new(self.check_condition(*rhs)),
                operator_l,
                expression_l,
            }),
            Node::Irange(Irange {
                left,
                right,
                operator_l,
                expression_l,
            }) => Node::IFlipFlop(IFlipFlop {
                left: left.map(|node| Box::new(self.check_condition(*node))),
                right: right.map(|node| Box::new(self.check_condition(*node))),
                operator_l,
                expression_l,
            }),
            Node::Erange(Erange {
                left,
                right,
                operator_l,
                expression_l,
            }) => Node::EFlipFlop(EFlipFlop {
                left: left.map(|node| Box::new(self.check_condition(*node))),
                right: right.map(|node| Box::new(self.check_condition(*node))),
                operator_l,
                expression_l,
            }),
            Node::Regexp(Regexp {
                parts,
                options,
                begin_l,
                end_l,
                expression_l,
            }) => Node::MatchCurrentLine(MatchCurrentLine {
                re: Box::new(Node::Regexp(Regexp {
                    parts,
                    options,
                    begin_l,
                    end_l,
                    expression_l: expression_l.clone(),
                })),
                expression_l,
            }),
            _ => cond,
        }
    }

    pub(crate) fn check_duplicate_args<'a>(
        &self,
        args: &'a Vec<Node>,
        map: &mut HashMap<String, &'a Node>,
    ) {
        for arg in args {
            match arg {
                Node::Arg(_)
                | Node::Optarg(_)
                | Node::Blockarg(_)
                | Node::Kwarg(_)
                | Node::Kwoptarg(_)
                | Node::Restarg(_)
                | Node::Kwrestarg(_) => {
                    self.check_duplicate_arg(arg, map);
                }
                Node::Mlhs(Mlhs { items, .. }) | Node::Procarg0(Procarg0 { args: items, .. }) => {
                    self.check_duplicate_args(items, map);
                }
                _ => {}
            }
        }
    }

    fn arg_name<'a>(&self, node: &'a Node) -> Option<&'a String> {
        match node {
            Node::Arg(Arg { name, .. })
            | Node::Optarg(Optarg { name, .. })
            | Node::Restarg(Restarg {
                name: Some(name), ..
            })
            | Node::Kwarg(Kwarg { name, .. })
            | Node::Kwoptarg(Kwoptarg { name, .. })
            | Node::Kwrestarg(Kwrestarg {
                name: Some(name), ..
            })
            | Node::Blockarg(Blockarg { name, .. }) => Some(name),

            Node::Restarg(_) | Node::Kwrestarg(_) => None,
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
                map.insert(this_name.to_owned(), this_arg);
            }
            Some(that_arg) => {
                let that_name = match self.arg_name(*that_arg) {
                    Some(name) => name,
                    None => return,
                };
                if self.arg_name_collides(this_name, that_name) {
                    // duplicate_argument
                }
            }
        }
    }

    pub(crate) fn check_assignment_to_numparam(&self, name: &str, loc: &Range) {
        let assigning_to_numparam = self.context.is_in_dynamic_block()
            && match name {
                "_1" | "_2" | "_3" | "_4" | "_5" | "_6" | "_7" | "_8" | "_9" => true,
                _ => false,
            }
            && self.max_numparam_stack.has_numparams();

        if assigning_to_numparam {
            loc.begin_pos;
            // diagnostic :error, :cant_assign_to_numparam, { :name => name }, loc
        }
    }

    pub(crate) fn check_reserved_for_numparam(&self, name: &str, _loc: &Range) {
        match name {
            "_1" | "_2" | "_3" | "_4" | "_5" | "_6" | "_7" | "_8" | "_9" => {
                // diagnostic :error, "reserved_for_numparam"
            }
            _ => {}
        }
    }

    pub(crate) fn arg_name_collides(&self, this_name: &str, that_name: &str) -> bool {
        &this_name[0..1] != "_" && this_name == that_name
    }

    pub(crate) fn check_lvar_name(&self, name: &str, loc: &Range) {
        let first = name.chars().nth(0).unwrap();
        let rest = &name[1..];

        if (first.is_lowercase() || first == '_')
            && rest.chars().all(|c| c.is_alphanumeric() || c == '_')
        {
            // OK
        } else {
            loc.begin_pos;
            // diagnostic :error, :lvar_name, { name: name }, loc
        }
    }

    pub(crate) fn check_duplicate_pattern_variable(&self, name: &str, loc: &Range) {
        if name.starts_with("_") {
            return;
        }

        if self.pattern_variables.is_declared(name) {
            loc.begin_pos;
            // diagnostic :error, :duplicate_variable_name, { name: name.to_s }, loc
        }

        self.pattern_variables.declare(name)
    }

    pub(crate) fn check_duplicate_pattern_key(&self, name: &str, loc: &Range) {
        if self.pattern_hash_keys.is_declared(name) {
            loc.begin_pos;
            // diagnostic :error, :duplicate_pattern_key, { name: name.to_s }, loc
        }

        self.pattern_hash_keys.declare(name)
    }

    //
    // Helpers
    //

    pub(crate) fn static_string(&self, nodes: &Vec<Node>) -> Option<String> {
        let mut result = String::from("");

        for node in nodes {
            match node {
                Node::Str(Str { value, .. }) => {
                    let value = value.to_string_lossy();
                    result.push_str(&value)
                }
                Node::Begin(Begin { statements, .. }) => {
                    if let Some(s) = self.static_string(statements) {
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

    pub(crate) fn static_regexp(&self, node: &Node) -> Option<Regex> {
        match node {
            Node::Regexp(Regexp { parts, options, .. }) => match &**options {
                Node::RegOpt(RegOpt { options, .. }) => {
                    if let Some(source) = self.static_string(&parts) {
                        let mut reg_options = RegexOptions::REGEX_OPTION_NONE;
                        reg_options |= RegexOptions::REGEX_OPTION_CAPTURE_GROUP;
                        if options.contains(&'x') {
                            reg_options |= RegexOptions::REGEX_OPTION_EXTEND;
                        }

                        let bytes = onig::EncodedBytes::ascii(source.as_bytes());
                        let regex = Regex::with_options_and_encoding(
                            bytes,
                            reg_options,
                            onig::Syntax::ruby(),
                        );

                        match regex {
                            Ok(regex) => return Some(regex),
                            Err(err) => println!(
                                "Failed to process static regex source, got error {:?}",
                                err,
                            ),
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        };

        None
    }

    pub(crate) fn static_regexp_captures(&self, regex: &Regex) -> Vec<String> {
        let mut result: Vec<String> = vec![];

        regex.foreach_name(|name, _| {
            result.push(name.to_owned());
            true
        });

        result
    }

    pub(crate) fn validate_definee(&self, definee: &Node) {
        match definee {
            Node::Int(_)
            | Node::Str(_)
            | Node::Dstr(_)
            | Node::Sym(_)
            | Node::Dsym(_)
            | Node::Heredoc(_)
            | Node::Regexp(_)
            | Node::Array(_)
            | Node::Hash(_) => {
                // diagnostic :error, :singleton_literal, nil, definee.loc.expression
            }
            _ => {}
        }
    }

    pub(crate) fn loc(&self, token: &Token) -> Range {
        Range::new(token.loc.begin, token.loc.end, Rc::clone(&self.source))
    }

    pub(crate) fn maybe_loc(&self, token: &Option<Token>) -> Option<Range> {
        token.as_ref().map(|t| self.loc(t))
    }

    pub(crate) fn collection_map(
        &self,
        begin_t: &Option<Token>,
        parts: &Vec<Node>,
        end_t: &Option<Token>,
    ) -> (Option<Range>, Option<Range>, Range) {
        let begin_l = self.maybe_loc(begin_t);
        let end_l = self.maybe_loc(end_t);

        let expr_l = merge_maybe_locs(vec![
            begin_l.clone(),
            collection_expr(&parts),
            end_l.clone(),
        ])
        .unwrap_or_else(|| {
            panic!("empty collection without begin_t/end_t, can't build source map")
        });

        (begin_l, end_l, expr_l)
    }

    pub(crate) fn string_map(
        &self,
        begin_t: &Option<Token>,
        parts: &Vec<Node>,
        end_t: &Option<Token>,
    ) -> StringMap {
        if let Some(begin_t) = begin_t {
            if value(&begin_t).starts_with("<<") {
                let end_t = end_t
                    .as_ref()
                    .unwrap_or_else(|| panic!("heredoc must have end_t"));
                let heredoc_body_l = collection_expr(&parts).unwrap_or_else(|| self.loc(end_t));
                let expression_l = self.loc(begin_t);
                let heredoc_end_l = self.loc(end_t);

                return StringMap::HeredocMap((heredoc_body_l, heredoc_end_l, expression_l));
            }
        }

        StringMap::CollectionMap(self.collection_map(begin_t, parts, end_t))
    }
}

pub(crate) fn maybe_node_expr(node: &Option<Node>) -> Option<Range> {
    node.as_ref().map(|node| node.expression().clone())
}

pub(crate) fn collection_expr(nodes: &Vec<Node>) -> Option<Range> {
    join_maybe_exprs(&nodes.first().cloned(), &nodes.last().cloned())
}

pub(crate) fn merge_maybe_locs(locs: Vec<Option<Range>>) -> Option<Range> {
    let mut result: Option<Range> = None;
    for loc in locs {
        result = join_maybe_locs(&result, &loc)
    }
    result
}

pub(crate) fn value(token: &Token) -> String {
    token.token_value.to_string_lossy()
}

pub(crate) fn maybe_value(token: &Option<Token>) -> Option<String> {
    token.as_ref().map(value)
}

pub(crate) fn join_exprs(lhs: &Node, rhs: &Node) -> Range {
    lhs.expression().join(rhs.expression())
}

pub(crate) fn join_maybe_exprs(lhs: &Option<Node>, rhs: &Option<Node>) -> Option<Range> {
    join_maybe_locs(&maybe_node_expr(lhs), &maybe_node_expr(rhs))
}

pub(crate) fn join_maybe_locs(lhs: &Option<Range>, rhs: &Option<Range>) -> Option<Range> {
    match (lhs, rhs) {
        (None, None) => None,
        (None, Some(rhs)) => Some(rhs.clone()),
        (Some(lhs), None) => Some(lhs.clone()),
        (Some(lhs), Some(rhs)) => Some(lhs.join(&rhs)),
    }
}

pub(crate) enum StringMap {
    CollectionMap((Option<Range>, Option<Range>, Range)),
    HeredocMap((Range, Range, Range)),
}
