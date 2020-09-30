use std::{convert::TryInto};
use crate::source::Range;
use crate::{Lexer, Node, Token, StaticEnvironment, Context, CurrentArgStack};
use crate::source::map::*;

#[derive(Debug, PartialEq)]
pub enum LoopType {
    While,
    Until
}

#[derive(Debug, PartialEq)]
pub enum KeywordCmd {
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
    CSend
}

#[derive(Debug, PartialEq)]
pub enum LogicalOp {
    And,
    Or
}

#[derive(Debug, Clone, PartialEq)]
pub enum PKwLabel {
    PlainLabel(Token),
    QuotedLabel(( Token, Vec<Node>, Token ))
}

#[derive(Debug, Default)]
pub struct Builder {
    static_env: StaticEnvironment,
    context: Context,
    current_arg_stack: CurrentArgStack
}

impl Builder {
    pub fn new(static_env: StaticEnvironment, context: Context, current_arg_stack: CurrentArgStack) -> Self {
        Self { static_env, context, current_arg_stack }
    }

    //
    // Literals
    //

    // Singletons

    pub fn nil(&self, nil_t: Token) -> Node {
        Node::Nil {
            loc: self.token_map(&nil_t)
        }
    }

    pub fn true_(&self, true_t: Token) -> Node {
        Node::True {
            loc: self.token_map(&true_t)
        }
    }

    pub fn false_(&self, false_t: Token) -> Node {
        Node::False {
            loc: self.token_map(&false_t)
        }
    }

    // Numerics

    pub fn integer(&self, integer_t: Token) -> Node {
        Node::Int {
            value: self.value(&integer_t),
            loc: OperatorMap {
                expression: self.loc(&integer_t),
                operator: None,
            }
        }
    }

    pub fn float(&self, float_t: Token) -> Node {
        Node::Float {
            value: self.value(&float_t),
            loc: OperatorMap {
                expression: self.loc(&float_t),
                operator: None,
            }
        }
    }

    pub fn rational(&self, rational_t: Token) -> Node {
        Node::Rational {
            value: self.value(&rational_t),
            loc: OperatorMap {
                expression: self.loc(&rational_t),
                operator: None,
            }
        }
    }

    pub fn complex(&self, complex: Token) -> Node {
        Node::Complex {
            value: self.value(&complex),
            loc: OperatorMap {
                expression: self.loc(&complex),
                operator: None,
            }
        }
    }

    pub fn unary_num(&self, unary_t: Token, mut numeric: Node) -> Node {
        let sign = self.value(&unary_t);
        let operator_l = self.loc(&unary_t);

        match &mut numeric {
            Node::Int { value, loc }
            | Node::Float { value, loc }
            | Node::Rational { value, loc }
            | Node::Complex { value, loc } => {
                *value = sign + value;
                loc.expression = operator_l.join(&loc.expression);
                loc.operator = Some(operator_l);
                numeric
            },
            _ => panic!("unreachable")
        }
    }

    pub fn __line__(&self, line_t: Token) -> Node {
        Node::__LINE__ {
            loc: self.token_map(&line_t)
        }
    }

    // Strings

    pub fn string(&self) {}

    pub fn string_internal(&self, string_t: Token) -> Node {
        Node::Str {
            value: self.value(&string_t),
            loc: self.unquoted_map(&string_t)
        }
    }

    pub fn string_compose(&self, begin_t: Option<Token>, parts: Vec<Node>, end_t: Option<Token>) -> Node {
        if parts.len() == 1 {
            let part = &parts[0];
            match part {
                Node::Str { .. } | Node::Dstr { .. } => {
                    // collapse_string_parts? == true
                    if begin_t.is_none() && end_t.is_none() {
                        return part.clone();
                    } else {
                        match part {
                            Node::Str { value, .. } => {
                                return Node::Str {
                                    value: value.clone(),
                                    loc: self.string_map(&begin_t, &parts, &end_t)
                                }
                            },
                            _ => panic!("unreachable")
                        }
                    }
                },
                _ => {}
            }
        }

        Node::Dstr {
            loc: self.string_map(&begin_t, &parts, &end_t),
            children: parts,
        }
    }

    pub fn character(&self, char_t: Token) -> Node { unimplemented!("character") }

    pub fn __file__(&self, file_t: Token) -> Node {
        Node::__FILE__ {
            loc: self.token_map(&file_t)
        }
    }

    // Symbols

    pub fn symbol(&self, start_t: Token, value_t: Token) -> Node {
        Node::Sym {
            name: self.value(&value_t),
            loc: CollectionMap {
                expression: self.loc(&start_t).join(&self.loc(&value_t)),
                begin: Some(self.loc(&start_t)),
                end: None
            }
        }
    }

    pub fn symbol_internal(&self, symbol_t: Token) -> Node {
        Node::Sym {
            name: self.value(&symbol_t),
            loc: self.unquoted_map(&symbol_t)
        }
    }

    pub fn symbol_compose(&self, begin_t: Token, parts: Vec<Node>, end_t: Token) -> Node { unimplemented!("symbol_compose") }

    // Executable strings

    pub fn xstring_compose(&self, begin_t: Token, parts: Vec<Node>, end_t: Token) -> Node { unimplemented!("xstring_compose") }

    // Indented (interpolated, noninterpolated, executable) strings

    pub fn dedent_string(&self, node: Node, dedent_level: i32) -> Node { unimplemented!("dedent_string") }

    // Regular expressions

    pub fn regexp_options(&self, regexp_end_t: &Token) -> Node {
        let mut options = self.value(&regexp_end_t).chars().collect::<Vec<_>>();
        options.sort();
        options.dedup();

        Node::RegOpt {
            options,
            loc: self.token_map(&regexp_end_t)
        }
    }

    pub fn regexp_compose(&self, begin_t: Token, parts: Vec<Node>, end_t: Token, options: Node) -> Node {
        Node::Regexp {
            loc: self.regexp_map(&begin_t, &end_t, &options),
            parts,
            options: Box::new(options),
        }
    }

    // Arrays

    pub fn array(&self, begin_t: Option<Token>, elements: Vec<Node>, end_t: Option<Token>) -> Node {
        let loc = self.collection_map(&begin_t, &elements, &end_t);
        Node::Array { elements, loc }
    }

    pub fn splat(&self, star_t: Token, arg: Option<Node>) -> Node {
        Node::Splat {
            loc: self.unary_op_map(&star_t, &arg),
            arg: arg.map(|node| Box::new(node))
        }
    }

    pub fn word(&self, parts: Vec<Node>) -> Node {
        if parts.len() == 1 {
            let part = &parts[0];
            match part {
                Node::Str { .. } | Node::Dstr { .. } => {
                    // collapse_string_parts? == true
                    return part.clone()
                }
                _ => {}
            }
        }

        Node::Dstr {
            loc: self.collection_map(&None, &parts, &None),
            children: parts,
        }
    }

    pub fn words_compose(&self, begin_t: Token, parts: Vec<Node>, end_t: Token) -> Node {
        Node::Array {
            loc: self.collection_map(&Some(begin_t), &parts, &Some(end_t)),
            elements: parts
        }
    }

    pub fn symbols_compose(&self, begin_t: Token, parts: Vec<Node>, end_t: Token) -> Node {
        let parts = parts.into_iter().map(|part| {
            match part {
                Node::Str { value, loc } => {
                    Node::Sym {
                        name: value,
                        loc: loc
                    }
                },
                Node::Dstr { children, loc } => {
                    Node::Dsym { children, loc }
                },
                _ => part
            }
        }).collect::<Vec<_>>();

        Node::Array {
            loc: self.collection_map(&Some(begin_t), &parts, &Some(end_t)),
            elements: parts
        }
    }

    // Hashes

    pub fn pair(&self, key: Node, assoc_t: Token, value: Node) -> Node {
        let loc = self.binary_op_map(&key, &assoc_t, &value);
        Node::Pair {
            key: Box::new(key),
            value: Box::new(value),
            loc
        }
    }
    pub fn pair_list_18(&self) {}

    pub fn pair_keyword(&self, key_t: Token, value: Node) -> Node {
        let (key_map, pair_map) = self.pair_keyword_map(&key_t, &value);
        let key = Node::Sym {
            name: self.value(&key_t),
            loc: key_map
        };
        Node::Pair {
            key: Box::new(key),
            value: Box::new(value),
            loc: pair_map
        }
    }

    pub fn pair_quoted(&self, begin_t: Token, parts: Vec<Node>, end_t: Token, value: Node) -> Node { unimplemented!("pair_quoted") }
    pub fn kwsplat(&self, dstar_t: Token, arg: Node) -> Node { unimplemented!("kwsplat") }

    pub fn associate(&self, begin_t: Option<Token>, pairs: Vec<Node>, end_t: Option<Token>) -> Node {
        let loc = self.collection_map(&begin_t, &pairs, &end_t);
        Node::Hash {
            pairs,
            loc
        }
    }

    // Ranges

    pub fn range_inclusive(&self, _lhs: Option<Node>, _dot2_t: Token, _rhs: Option<Node>) -> Node {
        unimplemented!("range_inclusive")
    }
    pub fn range_exclusive(&self, _lhs: Option<Node>, _dot3_t: Token, _rhs: Option<Node>) -> Node {
        unimplemented!("range_exclusive")
    }

    //
    // Access
    //

    pub fn self_(&self, token: Token) -> Node {
        Node::Self_ {
            loc: self.token_map(&token)
        }
    }

    pub fn lvar(&self, token: Token) -> Node {
        Node::Lvar {
            name: self.value(&token),
            loc: self.variable_map(&token)
        }
    }

    pub fn ivar(&self, token: Token) -> Node {
        Node::Ivar {
            name: self.value(&token),
            loc: self.variable_map(&token)
        }
    }

    pub fn gvar(&self, token: Token) -> Node {
        Node::Gvar {
            name: self.value(&token),
            loc: self.variable_map(&token)
        }
    }

    pub fn cvar(&self, token: Token) -> Node {
        Node::Cvar {
            name: self.value(&token),
            loc: self.variable_map(&token)
        }
    }

    pub fn back_ref(&self, token: Token) -> Node {
        Node::BackRef {
            name: self.value(&token),
            loc: self.variable_map(&token)
        }
    }

    pub fn nth_ref(&self, token: Token) -> Node {
        Node::NthRef {
            name: self.value(&token),
            loc: self.variable_map(&token)
        }
    }
    pub fn accessible(&self, node: Node) -> Node {
        match node {
            Node::Lvar { name, loc } => {
                if self.static_env.is_declared(&name) {
                    if let Some(current_arg) = self.current_arg_stack.top() {
                        if current_arg == name {
                            // diagnostic :error, :circular_argument_reference,
                            //        { :var_name => name.to_s }, node.loc.expression
                        }
                    }

                    Node::Lvar { name, loc }
                } else {
                    Node::Send {
                        receiver: None,
                        operator: name,
                        args: vec![],
                        loc: SendMap {
                            expression: loc.expression.clone(),
                            dot: None,
                            selector: Some(loc.expression.clone()),
                            operator: None,
                            begin: None,
                            end: None
                        }
                    }
                }
            },
            _ => node
        }
    }

    pub fn const_(&self, name_t: Token) -> Node {
        Node::Const {
            scope: None,
            name: self.value(&name_t),
            loc: self.constant_map(&None, &None, &name_t)
        }
    }

    pub fn const_global(&self, t_colon3: Token, name_t: Token) -> Node {
        let cbase = Node::Cbase { loc: self.token_map(&t_colon3) };
        Node::Const {
            loc: self.constant_map(&Some(&cbase), &Some(t_colon3), &name_t),
            scope: Some(Box::new(cbase)),
            name: self.value(&name_t)
        }
    }

    pub fn const_fetch(&self, scope: Node, t_colon2: Token, name_t: Token) -> Node {
        Node::Const {
            loc: self.constant_map(&Some(&scope), &Some(t_colon2), &name_t),
            scope: Some(Box::new(scope)),
            name: self.value(&name_t)
        }
    }

    pub fn __encoding__(&self, _encoding_t: Token) -> Node {
        Node::__ENCODING__ {
            loc: self.token_map(&_encoding_t)
        }
    }

    //
    // Assignments
    //

    pub fn assignable(&self, node: Node) -> Node {
        match node {
            Node::Cvar { name, loc } => {
                Node::Cvasgn { name, loc, rhs: None }
            },
            Node::Ivar { name, loc } => {
                Node::Ivasgn { name, loc, rhs: None }
            },
            Node::Gvar { name, loc } => {
                Node::Gvasgn { name, loc, rhs: None }
            },
            Node::Const { name, scope, loc } => {
                // if !self.context.is_dynamic_const_definition_allowed() {
                //     diagnostic :error, :dynamic_const, nil, node.loc.expression
                // }
                Node::Casgn { name, scope, loc, rhs: None }
            },
            Node::Lvar { name, loc: VariableMap { expression, .. } } => {
                let loc = expression.clone();

                self.check_assignment_to_numparam(&name, &loc);
                self.check_reserved_for_numparam(&name, &loc);

                self.static_env.declare(&name);

                Node::Lvasgn {
                    name,
                    loc: VariableMap { expression: loc, operator: None },
                    rhs: None
                }
            },

            Node::Nil { .. }
            | Node::Self_ { .. }
            | Node::True { .. }
            | Node::False { .. }
            | Node::__FILE__ { .. }
            | Node::__LINE__ { .. }
            | Node::__ENCODING__  { .. } => {
                // diagnostic :error, :invalid_assignment, nil, node.loc.expression
                node
            },
            Node::BackRef { .. }
            | Node::NthRef { .. } => {
                // diagnostic :error, :backref_assignment, nil, node.loc.expression
                node
            },
            _ => {
                panic!("{:#?} can't be used in assignment", node)
            }
        }
    }

    pub fn const_op_assignable(&self, node: Node) -> Node {
        match node {
            Node::Const { scope, name, loc } => {
                Node::Casgn { scope, name, loc, rhs: None }
            },
            _ => panic!("unsupported const_op_assignable arument: {:#?}", node)
        }
    }

    pub fn assign(&self, mut lhs: Node, eql_t: Token, rhs_value: Node) -> Node {
        let operator_l = Some(self.loc(&eql_t));

        let var_loc = VariableMap {
            expression: self.join_expr(&lhs, &rhs_value),
            operator: operator_l
        };

        match lhs {
            Node::Cvasgn { ref mut loc, ref mut rhs, .. }
            | Node::Ivasgn { ref mut loc, ref mut rhs, .. }
            | Node::Gvasgn { ref mut loc, ref mut rhs, .. }
            | Node::Lvasgn { ref mut loc, ref mut rhs, .. } => {
                *loc = var_loc;
                *rhs = Some(Box::new(rhs_value));
                lhs
            },
            _ => panic!("{:#?} can't be used in assignment", lhs)
        }
    }

    pub fn op_assign(&self, mut lhs: Node, op_t: Token, rhs: Node) -> Node {
        let operator = self.value(&op_t);
        let operator_l = self.loc(&op_t);
        let expression_l = self.join_expr(&lhs, &rhs);

        let loc = match lhs {
            Node::Gvasgn { .. }
            | Node::Ivasgn { .. }
            | Node::Lvasgn { .. }
            | Node::Cvasgn { .. }
            | Node::Casgn { .. }
            | Node::Send { .. }
            | Node::CSend { .. } => {
                OpAssignMap {
                    expression: expression_l,
                    operator: operator_l,
                }
            },
            Node::Index { receiver, indexes, loc } => {
                lhs = Node::IndexAsgn {
                    receiver, indexes, rhs: None, loc
                };
                OpAssignMap {
                    expression: expression_l,
                    operator: operator_l,
                }
            },
            Node::BackRef { .. }
            | Node::NthRef { .. } => {
                // diagnostic :error, :backref_assignment, nil, lhs.loc.expression
                return rhs;
            }
            _ => panic!("unsupported op_assign lhs {:#?}", lhs)
        };

        let lhs = Box::new(lhs);
        let rhs = Box::new(rhs);

        match &operator[..] {
            "&&" => Node::AndAsgn { lhs, rhs, loc },
            "||" => Node::OrAsgn { lhs, rhs, loc },
            _ => Node::OpAsgn { lhs, rhs, operator, loc }
        }
    }

    pub fn multi_lhs(&self, begin_t: Option<Token>, items: Vec<Node>, end_t: Option<Token>) -> Node {
        Node::Mlhs {
            loc: self.collection_map(&begin_t, &items, &end_t),
            items,
        }
    }

    pub fn multi_assign(&self, lhs: Node, eql_t: Token, rhs: Node) -> Node {
        Node::Masgn {
            loc: self.binary_op_map(&lhs, &eql_t, &rhs),
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub fn rassign(&self, lhs: Node, eql_t: Token, rhs: Node) -> Node {
        self.assign(rhs, eql_t, lhs)
    }

    pub fn multi_rassign(&self, lhs: Node, eql_t: Token, rhs: Node) -> Node {
        self.multi_assign(rhs, eql_t, lhs)
    }

    //
    // Class and module definition
    //

    pub fn def_class(&self, _class_t: Token, _name: Node, _lt_t: Option<Token>, _superclass: Option<Node>, _body: Option<Node>, _end_t: Token) -> Node {
        unimplemented!("def_class")
    }
    pub fn def_sclass(&self, _class_t: Token, _lshft_t: Token, _expr: Node, _body: Option<Node>, _end_t: Token) -> Node {
        unimplemented!("def_sclass")
    }
    pub fn def_module(&self, _module_t: Token, _name: Node, _body: Option<Node>, _end_t: Token) -> Node {
        unimplemented!("def_module")
    }

    //
    // Method (un)definition
    //

    pub fn def_method(&self, def_t: Token, name_t: Token, args: Option<Node>, body: Option<Node>, end_t: Token) -> Node {
        self.check_reserved_for_numparam(&self.value(&name_t), &self.loc(&name_t));

        let loc = self.definition_map(&def_t, &None, &name_t, &end_t);
        Node::Def {
            name: self.value(&name_t),
            args: args.map(|node| Box::new(node)),
            body: body.map(|node| Box::new(node)),
            loc
        }
    }

    pub fn def_endless_method(&self, _def_t: Token, _name_t: Token, _args: Option<Node>, _assignment_t: Token, _body: Option<Node>) -> Node {
        unimplemented!("def_endless_method")
    }
    pub fn def_singleton(&self, _def_t: Token, _definee: Node, _dot_t: Token, _name_t: Token, _args: Option<Node>, _body: Option<Node>, _end_t: Token) -> Node {
        unimplemented!("def_singleton")
    }
    pub fn def_endless_singleton(&self, _def_t: Token, _definee: Node, _dot_t: Token, _name_t: Token, _args: Option<Node>, _assignment_t: Token, _body: Option<Node>) -> Node {
        panic!("def_endless_singleton")
    }

    pub fn undef_method(&self, undef_t: Token, names: Vec<Node>) -> Node {
        let loc = self.keyword_map(&undef_t, &None, &names, &None);
        Node::Undef {
            names,
            loc
        }
    }

    pub fn alias(&self, alias_t: Token, to: Node, from: Node) -> Node {
        let loc = self.keyword_map(&alias_t, &None, &vec![from.clone(), to.clone()], &None);
        Node::Alias {
            to: Box::new(to),
            from: Box::new(from),
            loc
        }
    }

    //
    // Formal arguments
    //

    pub fn args(&self, begin_t: Option<Token>, args: Vec<Node>, end_t: Option<Token>) -> Option<Node> {
        match (&begin_t, args.len(), &end_t) {
            (None, 0, None) => None,
            _ => {
                let loc = self.collection_map(&begin_t, &args, &end_t);
                Some(
                    Node::Args {
                        args,
                        loc
                    }
                )
            }
        }
    }

    pub fn numargs(&self) {}
    pub fn forward_only_args(&self, begin_t: Token, dots_t: Token, end_t: Token) -> Node { unimplemented!("forward_only_args") }
    pub fn forward_arg(&self, dots_t: Token) -> Node { unimplemented!("forward_arg") }

    pub fn arg(&self, name_t: Token) -> Node {
        self.check_reserved_for_numparam(&self.value(&name_t), &self.loc(&name_t));
        Node::Arg {
            name: self.value(&name_t),
            loc: self.variable_map(&name_t)
        }
    }

    pub fn optarg(&self, _name_t: Token, _eql_t: Token, _value: Node) -> Node { unimplemented!("optarg") }
    pub fn restarg(&self, _start_t: Token, _name_t: Option<Token>) -> Node { unimplemented!("restarg") }
    pub fn kwarg(&self, _name_t: Token) -> Node { unimplemented!("kwarg") }
    pub fn kwoptarg(&self, _name_t: Token, _value: Node) -> Node { unimplemented!("kwoptarg") }
    pub fn kwrestarg(&self, _dstar_t: Token, _name_t: Option<Token>) -> Node { unimplemented!("kwrestarg") }
    pub fn kwnilarg(&self, _dstar_t: Token, _nil_t: Token) -> Node { unimplemented!("kwnilarg") }
    pub fn shadowarg(&self, _name_t: Token) -> Node { unimplemented!("shadowarg") }
    pub fn blockarg(&self, _amper_t: Token, _name_t: Token) -> Node { unimplemented!("blockarg") }
    pub fn procarg0(&self, _arg: Node) -> Node { unimplemented!("Node") }

    //
    // Method calls
    //

    fn call_type_for_dot(&self, dot_t: &Option<Token>) -> MethodCallType {
        match dot_t {
            Some((Lexer::tANDDOT, _, _)) => MethodCallType::CSend,
            _ => MethodCallType::Send,
        }
    }

    pub fn forwarded_args(&self, _dots_t: Token) -> Node {
        unimplemented!("forwarded_args")
    }

    pub fn call_method(&self, receiver: Option<Node>, dot_t: Option<Token>, selector_t: Option<Token>, lparen_t: Option<Token>, args: Vec<Node>, rparen_t: Option<Token>) -> Node {
        let loc = self.send_map(&receiver, &dot_t, &selector_t, &lparen_t, &args, &rparen_t);
        let method_name = selector_t.map(|t| self.value(&t)).unwrap_or("call".to_owned());

        match self.call_type_for_dot(&dot_t) {
            MethodCallType::Send => {
                Node::Send {
                    operator: method_name,
                    receiver: receiver.map(|node| Box::new(node)),
                    loc,
                    args
                }
            },

            MethodCallType::CSend => {
                Node::CSend {
                    operator: method_name,
                    receiver: receiver.map(|node| Box::new(node)),
                    loc,
                    args
                }
            }
        }
    }

    pub fn call_lambda(&self, _lambda_t: Token) -> Node { unimplemented!("call_lambda") }
    pub fn block(&self, _method_call: Node, _begin_t: Token, _args: Option<Node>, _body: Option<Node>, _end_t: Token) -> Node {
        unimplemented!("block")
    }
    pub fn block_pass(&self, _amper_t: Token, _arg: Node) -> Node {
        unimplemented!("block_pass")
    }

    pub fn attr_asgn(&self, receiver: Node, dot_t: Token, selector_t: Token) -> Node {
        let method_name = self.value(&selector_t) + "";
        let loc = self.send_map(&Some(receiver.clone()), &Some(dot_t.clone()), &Some(selector_t.clone()), &None, &vec![], &None);

        match self.call_type_for_dot(&Some(dot_t)) {
            MethodCallType::Send => {
                Node::Send {
                    operator: method_name,
                    receiver: Some(Box::new(receiver)),
                    loc,
                    args: vec![]
                }
            },

            MethodCallType::CSend => {
                Node::CSend {
                    operator: method_name,
                    receiver: Some(Box::new(receiver)),
                    loc,
                    args: vec![]
                }
            }
        }
    }

    pub fn index(&self, receiver: Node, lbrack_t: Token, indexes: Vec<Node>, rbrack_t: Token) -> Node {
        let loc = self.index_map(&receiver, &lbrack_t, &rbrack_t);
        Node::Index {
            receiver: Box::new(receiver),
            indexes,
            loc
        }
    }

    pub fn index_asgn(&self, receiver: Node, lbrack_t: Token, indexes: Vec<Node>, rbrack_t: Token) -> Node {
        let loc = self.index_map(&receiver, &lbrack_t, &rbrack_t);
        Node::IndexAsgn {
            receiver: Box::new(receiver),
            indexes,
            rhs: None,
            loc
        }
    }

    pub fn binary_op(&self, receiver: Node, operator_t: Token, arg: Node) -> Node {
        let source_map = self.send_binary_op_map(&receiver, &operator_t, &arg);
        Node::Send { receiver: Some(Box::new(receiver)), operator: self.value(&operator_t), args: vec![arg], loc: source_map }
    }

    pub fn match_op(&self, _receiver: Node, _match_t: Token, _arg: Node) -> Node {
        unimplemented!("match_op")
    }

    pub fn unary_op(&self, _op_t: Token, _receiver: Node) -> Node {
        unimplemented!("unary_op")
    }

    pub fn not_op(&self, not_t: Token, begin_t: Option<Token>, receiver: Option<Node>, end_t: Option<Token>) -> Node {
        if let Some(receiver) = receiver {
            Node::Send {
                loc: self.send_map(&None, &None, &Some(not_t), &begin_t, &vec![receiver.clone()], &end_t),
                receiver: Some(Box::new(self.check_condition(receiver))),
                operator: "!".to_owned(),
                args: vec![],
            }
        } else {
            let nil_node = Node::Begin {
                statements: vec![],
                loc: self.collection_map(&begin_t, &vec![], &end_t)
            };
            Node::Send {
                loc: self.send_unary_op_map(&not_t, &Some(&nil_node)),
                receiver: Some(Box::new(nil_node)),
                operator: "!".to_owned(),
                args: vec![]
            }
        }
    }

    //
    // Control flow
    //

    // Logical operations: and, or

    pub fn logical_op(&self, type_: LogicalOp, lhs: Node, op_t: Token, rhs: Node) -> Node {
        let loc = self.binary_op_map(&lhs, &op_t, &rhs);
        let lhs = Box::new(lhs);
        let rhs = Box::new(rhs);
        match type_ {
            LogicalOp::And => Node::And { lhs, rhs, loc },
            LogicalOp::Or => Node::Or { lhs, rhs, loc },
        }
    }

    // Conditionals

    pub fn condition(&self, _cond_t: Token, _cond: Node, _then_t: Token, _if_true: Option<Node>, _else_t: Option<Token>, _else_: Option<Node>, _end_t: Option<Token>) -> Node {
        unimplemented!("condition");
    }

    pub fn condition_mod(&self, if_true: Option<Node>, if_false: Option<Node>, cond_t: Token, cond: Node) -> Node {
        let pre = match (&if_true, &if_false) {
            (None, None) => panic!("at least one of if_true/if_false is required"),
            (None, Some(if_false)) => if_false.clone(),
            (Some(if_true), None) => if_true.clone(),
            (Some(_), Some(_)) => panic!("only one of if_true/if_false is required")
        };

        let loc = self.keyword_mod_map(&pre, &cond_t, &cond);
        Node::If {
            cond: Box::new(self.check_condition(cond)),
            if_true: if_true.map(|node| Box::new(node)),
            if_false: if_false.map(|node| Box::new(node)),
            loc
        }
    }

    pub fn ternary(&self, _cond: Node, _question_t: Token, _if_true: Node, _colon_t: Token, _if_false: Node) -> Node {
        unimplemented!("ternary")
    }

    // Case matching

    pub fn when(&self, when_t: Token, patterns: Vec<Node>, then_t: Token, body: Option<Node>) -> Node { unimplemented!("when") }

    pub fn case(&self, _case_t: Token, _expr: Option<Node>, _when_bodies: Vec<Node>, _else_t: Option<Token>, _else_body: Option<Node>, _end_t: Token) -> Node {
        unimplemented!("case")
    }

    // Loops

    pub fn loop_(&self, _loop_type: LoopType, _keyword_t: Token, _cond: Node, _do_t: Token, _body: Option<Node>, _end_t: Token) -> Node {
        unimplemented!("loop_");
    }

    pub fn loop_mod(&self, loop_type: LoopType, body: Node, keyword_t: Token, cond: Node) -> Node {
        let loc = self.keyword_mod_map(&body, &keyword_t, &cond);
        let cond = Box::new(self.check_condition(cond));

        match (loop_type, &body) {
            (LoopType::While, Node::KwBegin { .. }) => Node::WhilePost { cond, body: Box::new(body), loc },
            (LoopType::While, _)                    => Node::While     { cond, body: Box::new(body), loc },
            (LoopType::Until, Node::KwBegin { .. }) => Node::UntilPost { cond, body: Box::new(body), loc },
            (LoopType::Until, _)                    => Node::Until     { cond, body: Box::new(body), loc },
        }
    }

    pub fn for_(&self, _for_t: Token, _iterator: Node, _in_t: Token, _iteratee: Node, _do_t: Token, _body: Option<Node>, _end_t: Token) -> Node {
        unimplemented!("for_")
    }

    // Keywords

    pub fn keyword_cmd(&self, type_: KeywordCmd, keyword_t: Token, lparen_t: Option<Token>, args: Vec<Node>, rparen_t: Option<Token>) -> Node {
        if type_ == KeywordCmd::Yield && !args.is_empty() {
            // match args.last() {
            //     Some(Node::BlockPass { .. }) => {
            //         diagnostic :error, :block_given_to_yield, nil, loc(keyword_t), [last_arg.loc.expression]
            //     }
            // }
        }

        let loc = self.keyword_map(&keyword_t, &lparen_t, &args, &rparen_t);

        match type_ {
            KeywordCmd::Break => {
                Node::Break { args, loc }
            },
            KeywordCmd::Defined => {
                Node::Defined { args, loc }
            },
            KeywordCmd::Next => {
                Node::Next { args, loc }
            },
            KeywordCmd::Redo => {
                Node::Redo { args, loc }
            },
            KeywordCmd::Retry => {
                Node::Retry { args, loc }
            },
            KeywordCmd::Return => {
                Node::Return { args, loc }
            },
            KeywordCmd::Super => {
                Node::Super { args, loc }
            },
            KeywordCmd::Yield => {
                Node::Yield { args, loc }
            },
            KeywordCmd::Zsuper => {
                Node::Zsuper { args, loc }
            },
        }
    }

    // BEGIN, END

    pub fn preexe(&self, preexe_t: Token, lbrace_t: Token, compstmt: Option<Node>, rbrace_t: Token) -> Node {
        Node::Preexe {
            body: compstmt.map(|node| Box::new(node)),
            loc: self.keyword_map(&preexe_t, &Some(lbrace_t), &vec![], &Some(rbrace_t))
        }
    }
    pub fn postexe(&self, postexe_t: Token, lbrace_t: Token, compstmt: Option<Node>, rbrace_t: Token) -> Node {
        Node::Postexe {
            body: compstmt.map(|node| Box::new(node)),
            loc: self.keyword_map(&postexe_t, &Some(lbrace_t), &vec![], &Some(rbrace_t))
        }
    }

    // Exception handling

    pub fn rescue_body(&self, rescue_t: Token, exc_list: Option<Node>, assoc_t: Option<Token>, exc_var: Option<Node>, then_t: Option<Token>, compound_stmt: Option<Node>) -> Node {
        let loc = self.rescue_body_map(&rescue_t, &exc_list, &assoc_t, &exc_var, &then_t, &compound_stmt);

        Node::RescueBody {
            exc_list: exc_list.map(|node| Box::new(node)),
            exc_var: exc_var.map(|node| Box::new(node)),
            stmt: compound_stmt.map(|node| Box::new(node)),
            loc
        }
    }

    pub fn begin_body(&self, compound_stmt: Option<Node>, rescue_bodies: Vec<Node>, else_: Option<(Token, Node)>, ensure: Option<(Token, Node)>) -> Option<Node> {
        let mut result: Option<Node>;

        if !rescue_bodies.is_empty() {
            if let Some((else_t, else_)) = else_ {
                let loc = self.eh_keyword_map(&compound_stmt, &None, &rescue_bodies, &Some(else_t.clone()), &Some(else_.clone()));
                result = Some(
                        Node::Rescue {
                        body: compound_stmt.map(|node| Box::new(node)),
                        rescue_bodies,
                        else_: Some(Box::new(else_)),
                        loc
                    }
                )
            } else {
                let loc = self.eh_keyword_map(&compound_stmt, &None, &rescue_bodies, &None, &None);
                result = Some(
                        Node::Rescue {
                        body: compound_stmt.map(|node| Box::new(node)),
                        rescue_bodies,
                        else_: None,
                        loc
                    }
                )
            }
        } else if let Some((else_t, else_)) = else_ {
            let mut statements: Vec<Node> = vec![];

            match compound_stmt {
                Some(Node::Begin { statements: begin_statements, .. }) => statements = begin_statements,
                Some(compound_stmt) => statements.push(compound_stmt),
                _ => {}
            }
            let else_ = vec![else_];
            let loc = self.collection_map(&Some(else_t), &else_, &None);
            statements.push(
                Node::Begin {
                    statements: else_,
                    loc
                }
            );

            let loc = self.collection_map(&None, &statements, &None);
            result = Some(
                Node::Begin {
                    statements,
                    loc
                }
            )
        } else {
            result = compound_stmt;
        }

        if let Some((ensure_t, ensure)) = ensure {
            let loc = self.eh_keyword_map(&result, &Some(ensure_t.clone()), &vec![ensure.clone()], &None, &None);
            result = Some(
                    Node::Ensure {
                    body: result.map(|node| Box::new(node)),
                    ensure: Box::new(ensure),
                    loc
                }
            )
        }

        result
    }

    //
    // Expression grouping
    //

    pub fn compstmt(&self, mut statements: Vec<Node>) -> Option<Node> {
        if statements.is_empty() {
            None
        } else if statements.len() == 1 {
            statements.pop()
        } else {
            let source_map = self.collection_map(&None, &statements, &None);
            Some(Node::Begin { statements, loc: source_map })
        }
    }

    pub fn begin(&self, begin_t: Token, body: Option<Node>, end_t: Token) -> Node {
        if let Some(body) = body {
            match body {
                // Synthesized (begin) from compstmt "a; b" or (mlhs)
                // from multi_lhs "(a, b) = *foo".
                Node::Mlhs { loc: CollectionMap { expression, .. }, items: statements }
                | Node::Begin { loc: CollectionMap { begin: None, end: None, expression }, statements } => {
                    let loc = CollectionMap {
                        begin: Some(self.loc(&begin_t)),
                        end: Some(self.loc(&end_t)),
                        expression
                    };
                    Node::Begin { statements, loc }
                }
                body => {
                    let statements = vec![body];
                    Node::Begin {
                        loc: self.collection_map(&Some(begin_t), &statements, &Some(end_t)),
                        statements
                    }
                }
            }
        } else {
            // A nil expression: `()'.
            Node::Begin {
                statements: vec![],
                loc: self.collection_map(&Some(begin_t), &vec![], &Some(end_t))
            }
        }
    }

    pub fn begin_keyword(&self, begin_t: Token, body: Option<Node>, end_t: Token) -> Node {
        match body {
            None => {
                // A nil expression: `begin end'.
                Node::KwBegin  {
                    statements: vec![],
                    loc: self.collection_map(&Some(begin_t), &vec![], &Some(end_t))
                }
            },
            Some(Node::Begin { statements, loc: CollectionMap { begin: None, end: None, .. }, .. }) => {
                // Synthesized (begin) from compstmt "a; b".
                let loc = self.collection_map(&Some(begin_t), &statements, &Some(end_t));
                Node::KwBegin {
                    statements,
                    loc
                }
            },
            Some(node) => {
                let statements = vec![node];
                let loc = self.collection_map(&Some(begin_t), &statements, &Some(end_t));
                Node::KwBegin {
                    statements,
                    loc
                }
            }
        }
    }

    //
    // Pattern matching
    //

    pub fn case_match(&self, _case_t: Token, _expr: Node, _in_bodies: Vec<Node>, _else_t: Option<Token>, _else_body: Option<Node>, _end_t: Token) -> Node {
        unimplemented!("case_match")
    }

    pub fn in_match(&self, _lhs: Node, _in_t: Token, _rhs: Node) -> Node { unimplemented!("in_match") }
    pub fn in_pattern(&self, in_t: Token, pattern: Node, guard: Option<Node>, then_t: Token, body: Option<Node>) -> Node { unimplemented!("in_pattern") }

    pub fn if_guard(&self, if_t: Token, if_body: Node) -> Node { unimplemented!("if_guard") }
    pub fn unless_guard(&self, unless_t: Token, unless_body: Node) -> Node { unimplemented!("unless_guard") }

    pub fn match_var(&self, name_t: Token) -> Node { unimplemented!("match_var") }
    pub fn match_hash_var(&self, name_t: Token) -> Node { unimplemented!("match_hash_var") }
    pub fn match_hash_var_from_str(&self, begin_t: Token, strings: Vec<Node>, end_t: Token) -> Node { unimplemented!("match_hash_var_from_str") }
    pub fn match_rest(&self, star_t: Token, _name_t: Option<Token>) -> Node { unimplemented!("match_rest") }
    pub fn hash_pattern(&self, lbrace_t: Option<Token>, kwargs: Vec<Node>, rbrace_t: Option<Token>) -> Node { unimplemented!("hash_pattern") }
    pub fn array_pattern(&self, lbrack_t: Option<Token>, elements: Vec<Node>, rbrack_t: Option<Token>) -> Node { unimplemented!("array_pattern") }
    pub fn find_pattern(&self, lbrack_t: Option<Token>, elements: Vec<Node>, rbrack_t: Option<Token>) -> Node { unimplemented!("find_pattern") }
    pub fn match_with_trailing_comma(&self, match_: Node, comma_t: Token) -> Node { unimplemented!("match_with_trailing_comma") }
    pub fn const_pattern(&self, const_: Node, ldelim_t: Token, pattern: Node, rdelim_t: Token) -> Node { unimplemented!("const_pattern") }
    pub fn pin(&self, pin_t: Token, var: Node) -> Node { unimplemented!("pin") }
    pub fn match_alt(&self, left: Node, pipe_t: Token, right: Node) -> Node { unimplemented!("match_alt") }
    pub fn match_as(&self, value: Node, assoc_t: Token, as_: Node) -> Node { unimplemented!("match_as") }
    pub fn match_nil_pattern(&self, dstar_t: Token, nil_t: Token) -> Node { unimplemented!("match_nil_pattern") }
    pub fn match_pair(&self, p_kw_label: PKwLabel, value: Node) -> Node { unimplemented!("match_pair") }
    pub fn match_label(&self, p_kw_label: PKwLabel) -> Node { unimplemented!("match_label") }

    //
    // Verification
    //

    pub fn check_condition(&self, cond: Node) -> Node {
        match &cond {
            Node::Begin { statements, loc } => {
                if statements.len() == 1 {
                    let stmt = statements[statements.len() - 1].clone();
                    let stmt = self.check_condition(stmt);
                    return Node::Begin { statements: vec![stmt], loc: loc.clone() }
                }
                cond
            },
            // FIXME:
            // Node::And { lhs, rhs, .. }
            // | Node::Or { lhs, rhs, .. } => {
            // },
            // Node::Irange { begin, end, .. }
            // | Node::Erange { begin, end, .. } => {
            // }
            _ => cond
        }
    }

    pub fn check_duplicate_args(&self) {}
    pub fn check_duplicate_arg(&self) {}
    pub fn check_assignment_to_numparam(&self, _name: &str, _loc: &Range){
    }

    pub fn check_reserved_for_numparam(&self, name: &str, _loc: &Range) {
        if name.len() != 2 { return }

        let c1 = name.chars().nth(1).unwrap();
        let c2 = name.chars().nth(2).unwrap();

        if c1 == '0' && (c2 >= '1' && c2 <= '9') {
            // diagnostic :error, "reserved_for_numparam"
        }
    }

    pub fn arg_name_collides(&self) {}
    pub fn check_lvar_name(&self) {}
    pub fn check_duplicate_pattern_variable(&self) {}
    pub fn check_duplicate_pattern_key(&self) {}

    //
    // Source maps
    //

    pub fn join_expr(&self, left_expr: &Node, right_expr: &Node) -> Range {
        left_expr.expression().join(right_expr.expression())
    }

    pub fn token_map(&self, token: &Token) -> Map {
        Map { expression: self.loc(&token) }
    }

    pub fn delimited_string_map(&self) {}
    pub fn prefix_string_map(&self) {}

    pub fn unquoted_map(&self, token: &Token) -> CollectionMap {
        CollectionMap {
            begin: None,
            end: None,
            expression: self.loc(&token)
        }
    }

    pub fn pair_keyword_map(&self, key_t: &Token, value: &Node) -> (CollectionMap, OperatorMap) {
        let key_range = self.loc(&key_t);
        let key_l = key_range.adjust(0, -1);
        let colon_l = key_range.adjust((key_range.end_pos - 1).try_into().unwrap(), 0);

        (
            CollectionMap {
                begin: None,
                end: None,
                expression: key_l
            },
            OperatorMap {
                operator: Some(colon_l),
                expression: key_range.join(&value.expression())
            }
        )
    }

    pub fn pair_quoted_map(&self) {}
    pub fn expr_map(&self) {}

    pub fn collection_map(&self, begin_t: &Option<Token>, parts: &Vec<Node>, end_t: &Option<Token>) -> CollectionMap {
        let expr_l: Range;

        let begin_l = if let Some(begin_t) = begin_t { Some(self.loc(&begin_t)) } else { None };
        let end_l = if let Some(end_t) = end_t { Some(self.loc(&end_t)) } else { None };

        match (begin_l.clone(), end_l.clone(), !parts.is_empty()) {
            (Some(begin_l), Some(end_l), _) => {
                expr_l = begin_l.join(&end_l);
            },
            (_, _, true) => {
                expr_l = self.join_expr(parts.first().unwrap(), parts.last().unwrap());
            },
            (Some(begin_l), _, false) => {
                expr_l = begin_l.clone();
            },
            (_, Some(end_l), false) => {
                expr_l = end_l.clone();
            },
            (None, None, false) => {
                panic!("empty collection without begin_t/end_t, can't build source map");
            }
        }

        CollectionMap {
            begin: begin_l,
            end: end_l,
            expression: expr_l
        }
    }

    pub fn string_map(&self, begin_t: &Option<Token>, parts: &Vec<Node>, end_t: &Option<Token>) -> CollectionMap {
        if let Some(begin_t) = begin_t {
            if self.value(&begin_t).starts_with("<<") {
                unimplemented!("heredoc map")
            }
        }

        self.collection_map(begin_t, parts, end_t)
    }

    pub fn regexp_map(&self, begin_t: &Token, end_t: &Token, options: &Node) -> CollectionMap {
        CollectionMap {
            begin: Some(self.loc(begin_t)),
            end: Some(self.loc(end_t)),
            expression: self.loc(begin_t).join(options.expression())
        }
    }

    pub fn constant_map(&self, scope: &Option<&Node>, colon2_t: &Option<Token>, name_t: &Token) -> ConstantMap {
        let expr_l: Range;
        if let Some(scope) = scope {
            expr_l = scope.expression().join(&self.loc(name_t));
        } else {
            expr_l = self.loc(name_t);
        }

        ConstantMap {
            double_colon: colon2_t.clone().map(|t| self.loc(&t)),
            name: self.loc(&name_t),
            operator: None,
            expression: expr_l,
        }
    }

    pub fn variable_map(&self, name_t: &Token) -> VariableMap {
        VariableMap { expression: self.loc(name_t), operator: None }
    }

    pub fn binary_op_map(&self, left_e: &Node, op_t: &Token, right_e: &Node) -> OperatorMap {
        OperatorMap {
            operator: Some(self.loc(op_t)),
            expression: self.join_expr(&left_e, &right_e)
        }
    }

    pub fn unary_op_map(&self, op_t: &Token, arg: &Option<Node>) -> OperatorMap {
        let expr_l: Range;
        if let Some(arg) = arg {
            expr_l = self.loc(op_t).join(&arg.expression())
        } else {
            expr_l = self.loc(op_t);
        }

        OperatorMap {
            operator: Some(self.loc(op_t)),
            expression: expr_l
        }
    }
    pub fn range_map(&self) {}

    pub fn arg_prefix_map(&self) {}
    pub fn kwarg_map(&self) {}
    pub fn module_definition_map(&self) {}

    pub fn definition_map(&self, keyword_t: &Token, operator_t: &Option<Token>, name_t: &Token, end_t: &Token) -> MethodDefinitionMap {
        MethodDefinitionMap {
            keyword: self.loc(keyword_t),
            operator: operator_t.clone().map(|op| self.loc(&op)),
            name: self.loc(name_t),
            end: Some(self.loc(end_t)),
            assignment: None,
            expression: self.loc(keyword_t).join(&self.loc(end_t))
        }
    }

    pub fn endless_definition_map(&self) {}

    pub fn send_map(&self, receiver_e: &Option<Node>, dot_t: &Option<Token>, selector_t: &Option<Token>, begin_t: &Option<Token>, args: &Vec<Node>, end_t: &Option<Token>) -> SendMap {
        let begin_l: Option<Range>;
        let end_l: Option<Range>;

        if let Some(receiver_e) = receiver_e {
            begin_l = Some(receiver_e.expression().clone())
        } else if let Some(selector_t) = selector_t {
            begin_l = Some(self.loc(selector_t))
        } else {
            begin_l = None
        }

        if let Some(end_t) = end_t {
            end_l = Some(self.loc(end_t));
        } else if let Some(last_arg) = args.last() {
            end_l = Some(last_arg.expression().clone());
        } else if let Some(selector_t) = selector_t {
            end_l = Some(self.loc(&selector_t))
        } else {
            end_l = None
        }

        let expression_l = match (&begin_l, &end_l) {
            (Some(begin_l), Some(end_l)) => begin_l.join(end_l),
            _ => panic!("unreachable: begin_l = {:#?}, end_l = {:#?}", begin_l, end_l)
        };

        SendMap {
            expression: expression_l,
            dot: dot_t.clone().map(|t| self.loc(&t)),
            selector: selector_t.clone().map(|t| self.loc(&t)),
            operator: None,
            begin: begin_t.clone().map(|t| self.loc(&t)),
            end: end_t.clone().map(|t| self.loc(&t)),
        }
    }

    pub fn var_send_map(&self, variable_t: &Token) -> SendMap {
        SendMap {
            expression: self.loc(&variable_t),
            dot: None,
            selector: Some(self.loc(&variable_t)),
            operator: None,
            begin: None,
            end: None
        }
    }

    pub fn send_binary_op_map(&self, lhs_e: &Node, selector_t: &Token, rhs_e: &Node) -> SendMap {
        SendMap {
            expression: self.join_expr(&lhs_e, &rhs_e),
            dot: None,
            selector: Some(self.loc(&selector_t)),
            begin: None,
            end: None,
            operator: None
        }
    }

    pub fn send_unary_op_map(&self, selector_t: &Token, arg: &Option<&Node>) -> SendMap {
        let expr_l: Range;

        if let Some(arg) = arg {
            expr_l = self.loc(selector_t).join(arg.expression())
        } else {
            expr_l = self.loc(selector_t)
        }

        SendMap {
            expression: expr_l,
            selector: Some(self.loc(&selector_t)),
            dot: None,
            begin: None,
            operator: None,
            end: None
        }
    }

    pub fn index_map(&self, receiver_e: &Node, lbrack_t: &Token, rbrack_t: &Token) -> IndexMap {
        IndexMap {
            begin: self.loc(lbrack_t),
            end: self.loc(rbrack_t),
            expression: receiver_e.expression().join(&self.loc(rbrack_t)),
            operator: None
        }
    }
    pub fn send_index_map(&self) {}

    pub fn block_map(&self) {}

    pub fn keyword_map(&self, keyword_t: &Token, begin_t: &Option<Token>, args: &Vec<Node>, end_t: &Option<Token>) -> KeywordMap {
        let expr_end_l: Range;
        let begin_l = if let Some(begin_t) = begin_t { Some(self.loc(&begin_t)) } else { None };
        let end_l = if let Some(end_t) = end_t { Some(self.loc(&end_t)) } else { None };

        if let Some(end_l) = &end_l {
            expr_end_l = end_l.clone();
        } else if let Some(last_arg) = args.iter().rev().nth(0) {
            expr_end_l = last_arg.expression().clone();
        } else if let Some(second_last_arg) = args.iter().rev().nth(1) {
            expr_end_l = second_last_arg.expression().clone();
        } else {
            expr_end_l = self.loc(&keyword_t);
        }

        KeywordMap {
            expression: self.loc(&keyword_t).join(&expr_end_l),
            keyword: self.loc(&keyword_t),
            begin: begin_l,
            end: end_l
        }
    }

    pub fn keyword_mod_map(&self, pre_e: &Node, keyword_t: &Token, post_e: &Node) -> KeywordMap {
        KeywordMap {
            expression: pre_e.expression().join(&post_e.expression()),
            keyword: self.loc(keyword_t),
            begin: None,
            end: None
        }
    }

    pub fn condition_map(&self) {}
    pub fn ternary_map(&self) {}
    pub fn for_map(&self) {}

    pub fn rescue_body_map(&self, keyword_t: &Token, exc_list: &Option<Node>, assoc_t: &Option<Token>, exc_var: &Option<Node>, then_t: &Option<Token>, compstmt: &Option<Node>) -> RescueBodyMap {
        let end_l = match (compstmt, then_t, exc_var, exc_list) {
            (Some(compstmt), _, _, _) => compstmt.expression().clone(),
            (None, Some(then_t), _, _) => self.loc(then_t),
            (None, None, Some(exc_var), _) => exc_var.expression().clone(),
            (None, None, None, Some(exc_list)) => exc_list.expression().clone(),
            (None, None, None, None) => self.loc(&keyword_t)
        };

        RescueBodyMap {
            keyword: self.loc(keyword_t),
            assoc: assoc_t.clone().map(|t| self.loc(&t)),
            begin: then_t.clone().map(|t| self.loc(&t)),
            expression: self.loc(keyword_t).join(&end_l)
        }
    }

    pub fn eh_keyword_map(&self, compstmt_e: &Option<Node>, keyword_t: &Option<Token>, body_es: &Vec<Node>, else_t: &Option<Token>, else_e: &Option<Node>) -> ConditionMap {
        let begin_l: Range;
        let end_l: Range;

        if let Some(compstmt_e) = &compstmt_e {
            begin_l = compstmt_e.expression().clone();
        } else if let Some(keyword_t) = &keyword_t {
            begin_l = self.loc(&keyword_t);
        } else {
            begin_l = body_es.first().unwrap().expression().clone();
        }

        if let Some(else_t) = &else_t {
            if let Some(else_e) = &else_e {
                end_l = else_e.expression().clone();
            } else {
                end_l = self.loc(&else_t);
            }
        } else if let Some(last_body_es) = body_es.last() {
            end_l = last_body_es.expression().clone();
        } else if let Some(keyword_t) = &keyword_t {
            end_l = self.loc(&keyword_t);
        } else {
            panic!("bug");
        }

        ConditionMap {
            expression: begin_l.join(&end_l),
            keyword: keyword_t.clone().map(|t| self.loc(&t)),
            begin: None,
            else_: else_t.clone().map(|t| self.loc(&t)),
            end: None
        }
    }

    pub fn guard_map(&self) {}

    //
    // Helpers
    //

    pub fn static_string(&self) {}
    pub fn static_regexp(&self) {}
    pub fn static_regexp_node(&self) {}
    pub fn collapse_string_parts(&self) {}

    pub fn value(&self, token: &Token) -> String {
        let (_, token_value, _) = token;
        token_value.clone()
    }

    pub fn string_value(&self) {}

    pub fn loc(&self, token: &Token) -> Range {
        let (_, _, loc) = token;
        Range::new(loc.begin, loc.end)
    }

    pub fn diagnostic(&self) {}
    pub fn validate_definee(&self) {}

}
