use std::convert::TryInto;

use onig::{Regex, RegexOptions};
use crate::source::Range;
use crate::{Lexer, Node, Token, StaticEnvironment, Context, CurrentArgStack, MaxNumparamStack};
use crate::source::map::*;
use crate::map_builder::*;
use crate::parser::TokenValue;
use crate::node::StringValue;

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

#[derive(Debug, Clone)]
pub enum PKwLabel {
    PlainLabel(Token),
    QuotedLabel(( Token, Vec<Node>, Token ))
}

#[derive(Debug, Clone)]
pub enum ArgsType {
    Args(Option<Node>),
    Numargs(u8)
}

#[derive(Debug, Default)]
pub struct Builder {
    static_env: StaticEnvironment,
    context: Context,
    current_arg_stack: CurrentArgStack,
    max_numparam_stack: MaxNumparamStack,
}

impl Builder {
    pub fn new(static_env: StaticEnvironment, context: Context, current_arg_stack: CurrentArgStack, max_numparam_stack: MaxNumparamStack) -> Self {
        Self { static_env, context, current_arg_stack, max_numparam_stack }
    }

    //
    // Literals
    //

    // Singletons

    pub fn nil(&self, nil_t: Token) -> Node {
        Node::Nil {
            loc: token_map(&nil_t)
        }
    }

    pub fn true_(&self, true_t: Token) -> Node {
        Node::True {
            loc: token_map(&true_t)
        }
    }

    pub fn false_(&self, false_t: Token) -> Node {
        Node::False {
            loc: token_map(&false_t)
        }
    }

    // Numerics

    pub fn integer(&self, integer_t: Token) -> Node {
        Node::Int {
            value: value(&integer_t),
            loc: OperatorMap {
                expression: loc(&integer_t),
                operator: None,
            }
        }
    }

    pub fn float(&self, float_t: Token) -> Node {
        Node::Float {
            value: value(&float_t),
            loc: OperatorMap {
                expression: loc(&float_t),
                operator: None,
            }
        }
    }

    pub fn rational(&self, rational_t: Token) -> Node {
        Node::Rational {
            value: value(&rational_t),
            loc: OperatorMap {
                expression: loc(&rational_t),
                operator: None,
            }
        }
    }

    pub fn complex(&self, complex: Token) -> Node {
        Node::Complex {
            value: value(&complex),
            loc: OperatorMap {
                expression: loc(&complex),
                operator: None,
            }
        }
    }

    pub fn unary_num(&self, unary_t: Token, mut numeric: Node) -> Node {
        let sign = value(&unary_t);
        let operator_l = loc(&unary_t);

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
            _ => unreachable!()
        }
    }

    pub fn __line__(&self, line_t: Token) -> Node {
        Node::__LINE__ {
            loc: token_map(&line_t)
        }
    }

    // Strings

    pub fn str_node(&self, begin_t: Option<Token>, value: StringValue, parts: Vec<Node>, end_t: Option<Token>) -> Node {
        match string_map(&begin_t.as_ref(), &parts.iter().collect(), &end_t.as_ref()) {
            StringMap::CollectionMap(loc) => {
                Node::Str { value, loc }
            }
            StringMap::HeredocMap(loc) => {
                Node::Heredoc { children: parts, loc }
            }
        }
    }

    pub fn string(&self) {}

    pub fn string_internal(&self, string_t: Token) -> Node {
        let loc = unquoted_map(&string_t);
        let (_, value, _) = string_t;
        let value = match value {
            TokenValue::String(s) => StringValue::String(s),
            TokenValue::InvalidString(bytes) => StringValue::Bytes(bytes)
        };
        Node::Str { value, loc }
    }

    pub fn string_compose(&self, begin_t: Option<Token>, parts: Vec<Node>, end_t: Option<Token>) -> Node {
        match &parts[..] {
            [] => {
                return self.str_node(begin_t, StringValue::String("".to_owned()), parts, end_t)
            },
            [part] => {
                match part {
                    Node::Str { .. } | Node::Dstr { .. } => {
                        // collapse_string_parts? == true
                        if begin_t.is_none() && end_t.is_none() {
                            return part.clone();
                        } else {
                            match part {
                                Node::Str { value, .. } => {
                                    return self.str_node(begin_t, value.clone(), parts, end_t)
                                },
                                _ => unreachable!()
                            }
                        }
                    },
                    _ => {}
                }
            },
            _ => {}
        };

        match string_map(&begin_t.as_ref(), &parts.iter().collect(), &end_t.as_ref()) {
            StringMap::CollectionMap(loc) => {
                Node::Dstr { loc, children: parts, }
            }
            StringMap::HeredocMap(loc) => {
                Node::Heredoc { loc, children: parts }
            }
        }
    }

    pub fn character(&self, char_t: Token) -> Node {
        let loc = prefix_string_map(&char_t);
        let (_, value, _) = char_t;
        let value = match value {
            TokenValue::String(s) => StringValue::String(s),
            TokenValue::InvalidString(bytes) => StringValue::Bytes(bytes)
        };
        Node::Str { value, loc }
    }

    pub fn __file__(&self, file_t: Token) -> Node {
        Node::__FILE__ {
            loc: token_map(&file_t)
        }
    }

    // Symbols

    pub fn symbol(&self, start_t: Token, value_t: Token) -> Node {
        let loc = CollectionMap {
            expression: loc(&start_t).join(&loc(&value_t)),
            begin: Some(loc(&start_t)),
            end: None
        };
        Node::Sym { name: value(&value_t), loc }
    }

    pub fn symbol_internal(&self, symbol_t: Token) -> Node {
        let loc = unquoted_map(&symbol_t);
        Node::Sym { name: value(&symbol_t), loc }
    }

    pub fn symbol_compose(&self, begin_t: Token, parts: Vec<Node>, end_t: Token) -> Node {
        if parts.len() == 1 {
            let part = &parts[0];
            match part {
                // collapse_string_parts? == true
                Node::Str { value, .. } => {
                    let value = value.clone();
                    return Node::Sym {
                        loc: collection_map(&Some(&begin_t), &vec![], &Some(&end_t)),
                        name: value.to_string_lossy(),
                    }
                }
                _ => {}
            };
        }

        Node::Dsym {
            loc: collection_map(&Some(&begin_t), &parts.iter().collect(), &Some(&end_t)),
            children: parts
        }
    }

    // Executable strings

    pub fn xstring_compose(&self, begin_t: Token, parts: Vec<Node>, end_t: Token) -> Node {
        match string_map(&Some(&begin_t), &parts.iter().collect(), &Some(&end_t)) {
            StringMap::CollectionMap(loc) => {
                Node::Xstr { loc, children: parts, }
            }
            StringMap::HeredocMap(loc) => {
                Node::XHeredoc { loc, children: parts }
            }
        }
    }

    // Indented (interpolated, noninterpolated, executable) strings

    pub fn dedent_string(&self, node: Node, _dedent_level: i32) -> Node {
        // FIXME
        node
    }

    // Regular expressions

    pub fn regexp_options(&self, regexp_end_t: &Token) -> Node {
        let mut options = value(&regexp_end_t).chars().collect::<Vec<_>>();
        options.sort();
        options.dedup();

        Node::RegOpt {
            options,
            loc: token_map(&regexp_end_t)
        }
    }

    pub fn regexp_compose(&self, begin_t: Token, parts: Vec<Node>, end_t: Token, options: Node) -> Node {
        Node::Regexp {
            loc: regexp_map(&begin_t, &end_t, &options),
            parts,
            options: Box::new(options),
        }
    }

    // Arrays

    pub fn array(&self, begin_t: Option<Token>, elements: Vec<Node>, end_t: Option<Token>) -> Node {
        let loc = collection_map(&begin_t.as_ref(), &elements.iter().collect(), &end_t.as_ref());
        Node::Array { elements, loc }
    }

    pub fn splat(&self, star_t: Token, arg: Option<Node>) -> Node {
        Node::Splat {
            loc: unary_op_map(&star_t, &arg.as_ref()),
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
            loc: collection_map(&None, &parts.iter().collect(), &None),
            children: parts,
        }
    }

    pub fn words_compose(&self, begin_t: Token, parts: Vec<Node>, end_t: Token) -> Node {
        Node::Array {
            loc: collection_map(&Some(&begin_t), &parts.iter().collect(), &Some(&end_t)),
            elements: parts
        }
    }

    pub fn symbols_compose(&self, begin_t: Token, parts: Vec<Node>, end_t: Token) -> Node {
        let parts = parts.into_iter().map(|part| {
            match part {
                Node::Str { value, loc } => {
                    Node::Sym {
                        name: value.to_string_lossy(),
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
            loc: collection_map(&Some(&begin_t), &parts.iter().collect(), &Some(&end_t)),
            elements: parts
        }
    }

    // Hashes

    pub fn pair(&self, key: Node, assoc_t: Token, value: Node) -> Node {
        let loc = binary_op_map(&key, &assoc_t, &value);
        Node::Pair {
            key: Box::new(key),
            value: Box::new(value),
            loc
        }
    }
    pub fn pair_list_18(&self) {}

    pub fn pair_keyword(&self, key_t: Token, value_node: Node) -> Node {
        let (key_map, pair_map) = pair_keyword_map(&key_t, &value_node);
        let key = Node::Sym {
            name: value(&key_t),
            loc: key_map
        };
        Node::Pair {
            key: Box::new(key),
            value: Box::new(value_node),
            loc: pair_map
        }
    }

    pub fn pair_quoted(&self, begin_t: Token, parts: Vec<Node>, end_t: Token, value: Node) -> Node {
        let (end_t, pair_map) = pair_quoted_map(&begin_t, &end_t, &value);

        let key = self.symbol_compose(begin_t, parts, end_t);

        Node::Pair {
            key: Box::new(key),
            value: Box::new(value),
            loc: pair_map
        }
    }

    pub fn kwsplat(&self, dstar_t: Token, arg: Node) -> Node {
        Node::Kwsplat {
            loc: unary_op_map(&dstar_t, &Some(&arg)),
            value: Box::new(arg)
        }
    }

    pub fn associate(&self, begin_t: Option<Token>, pairs: Vec<Node>, end_t: Option<Token>) -> Node {
        let loc = collection_map(&begin_t.as_ref(), &pairs.iter().collect(), &end_t.as_ref());
        Node::Hash {
            pairs,
            loc
        }
    }

    // Ranges

    pub fn range_inclusive(&self, lhs: Option<Node>, dot2_t: Token, rhs: Option<Node>) -> Node {
        Node::Irange {
            loc: range_map(&lhs.as_ref(), &dot2_t, &rhs.as_ref()),
            left: lhs.map(|node| Box::new(node)),
            right: rhs.map(|node| Box::new(node))
        }
    }

    pub fn range_exclusive(&self, lhs: Option<Node>, dot3_t: Token, rhs: Option<Node>) -> Node {
        Node::Erange {
            loc: range_map(&lhs.as_ref(), &dot3_t, &rhs.as_ref()),
            left: lhs.map(|node| Box::new(node)),
            right: rhs.map(|node| Box::new(node))
        }
    }

    //
    // Access
    //

    pub fn self_(&self, token: Token) -> Node {
        Node::Self_ {
            loc: token_map(&token)
        }
    }

    pub fn lvar(&self, token: Token) -> Node {
        Node::Lvar {
            name: value(&token),
            loc: variable_map(&token)
        }
    }

    pub fn ivar(&self, token: Token) -> Node {
        Node::Ivar {
            name: value(&token),
            loc: variable_map(&token)
        }
    }

    pub fn gvar(&self, token: Token) -> Node {
        Node::Gvar {
            name: value(&token),
            loc: variable_map(&token)
        }
    }

    pub fn cvar(&self, token: Token) -> Node {
        Node::Cvar {
            name: value(&token),
            loc: variable_map(&token)
        }
    }

    pub fn back_ref(&self, token: Token) -> Node {
        Node::BackRef {
            name: value(&token),
            loc: variable_map(&token)
        }
    }

    pub fn nth_ref(&self, token: Token) -> Node {
        Node::NthRef {
            name: value(&token),
            loc: variable_map(&token)
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
            name: value(&name_t),
            loc: constant_map(&None, &None, &name_t)
        }
    }

    pub fn const_global(&self, t_colon3: Token, name_t: Token) -> Node {
        let cbase = Node::Cbase { loc: token_map(&t_colon3) };
        Node::Const {
            loc: constant_map(&Some(&cbase), &Some(&t_colon3), &name_t),
            scope: Some(Box::new(cbase)),
            name: value(&name_t)
        }
    }

    pub fn const_fetch(&self, scope: Node, t_colon2: Token, name_t: Token) -> Node {
        Node::Const {
            loc: constant_map(&Some(&scope), &Some(&t_colon2), &name_t),
            scope: Some(Box::new(scope)),
            name: value(&name_t)
        }
    }

    pub fn __encoding__(&self, _encoding_t: Token) -> Node {
        Node::__ENCODING__ {
            loc: token_map(&_encoding_t)
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
                if !self.context.is_dynamic_const_definition_allowed() {
                    // diagnostic :error, :dynamic_const, nil, node.loc.expression
                }
                Node::Casgn { name, scope, loc, rhs: None }
            },
            Node::Lvar { name, loc: VariableMap { expression: loc, name: name_l, .. } } => {
                self.check_assignment_to_numparam(&name, &loc);
                self.check_reserved_for_numparam(&name, &loc);

                self.static_env.declare(&name);

                Node::Lvasgn {
                    name,
                    loc: VariableMap { expression: loc, name: name_l, operator: None },
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
                panic!("{:?} can't be used in assignment", node)
            }
        }
    }

    pub fn const_op_assignable(&self, node: Node) -> Node {
        match node {
            Node::Const { scope, name, loc } => {
                Node::Casgn { scope, name, loc, rhs: None }
            },
            _ => panic!("unsupported const_op_assignable arument: {:?}", node)
        }
    }

    pub fn assign(&self, mut lhs: Node, eql_t: Token, new_rhs: Node) -> Node {
        let operator_l = Some(loc(&eql_t));
        let expr_l = join_exprs(&lhs, &new_rhs);

        match lhs {
            Node::Cvasgn { ref mut loc, ref mut rhs, .. }
            | Node::Ivasgn { ref mut loc, ref mut rhs, .. }
            | Node::Gvasgn { ref mut loc, ref mut rhs, .. }
            | Node::Lvasgn { ref mut loc, ref mut rhs, .. } => {
                loc.expression = expr_l;
                loc.operator = operator_l;
                *rhs = Some(Box::new(new_rhs));
                lhs
            },
            Node::Casgn { ref mut loc, ref mut rhs, .. } => {
                loc.expression = expr_l;
                loc.operator = operator_l;
                *rhs = Some(Box::new(new_rhs));
                lhs
            },
            Node::IndexAsgn { ref mut loc, ref mut rhs, .. } => {
                loc.expression = expr_l;
                loc.operator = operator_l;
                *rhs = Some(Box::new(new_rhs));
                lhs
            }
            Node::Send { ref mut args, ref mut loc, .. }
            | Node::CSend { ref mut args, ref mut loc, .. } => {
                loc.expression = expr_l;
                loc.operator = operator_l;
                if args.is_empty() {
                    *args = vec![new_rhs];
                } else {
                    unreachable!("can't assign to method call with args")
                }
                lhs
            }
            _ => panic!("{:?} can't be used in assignment", lhs)
        }
    }

    pub fn op_assign(&self, mut lhs: Node, op_t: Token, rhs: Node) -> Node {
        let operator = value(&op_t);
        let operator_l = loc(&op_t);
        let expression_l = join_exprs(&lhs, &rhs);

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
            _ => panic!("unsupported op_assign lhs {:?}", lhs)
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
            loc: collection_map(&begin_t.as_ref(), &items.iter().collect(), &end_t.as_ref()),
            items,
        }
    }

    pub fn multi_assign(&self, lhs: Node, eql_t: Token, rhs: Node) -> Node {
        Node::Masgn {
            loc: binary_op_map(&lhs, &eql_t, &rhs),
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

    pub fn def_class(&self, class_t: Token, name: Node, lt_t: Option<Token>, superclass: Option<Node>, body: Option<Node>, end_t: Token) -> Node {
        Node::Class {
            loc: module_definition_map(&class_t, &Some(&name), &lt_t.as_ref(), &end_t),
            name: Box::new(name),
            superclass: superclass.map(|node| Box::new(node)),
            body: body.map(|node| Box::new(node))
        }
    }

    pub fn def_sclass(&self, class_t: Token, lshift_t: Token, expr: Node, body: Option<Node>, end_t: Token) -> Node {
        Node::Sclass {
            loc: module_definition_map(&class_t, &None, &Some(&lshift_t), &end_t),
            expr: Box::new(expr),
            body: body.map(|node| Box::new(node))
        }
    }

    pub fn def_module(&self, module_t: Token, name: Node, body: Option<Node>, end_t: Token) -> Node {
        Node::Module {
            loc: module_definition_map(&module_t, &Some(&name), &None, &end_t),
            name: Box::new(name),
            body: body.map(|node| Box::new(node))
        }
    }

    //
    // Method (un)definition
    //

    pub fn def_method(&self, def_t: Token, name_t: Token, args: Option<Node>, body: Option<Node>, end_t: Token) -> Node {
        self.check_reserved_for_numparam(&value(&name_t), &loc(&name_t));

        Node::Def {
            loc: definition_map(&def_t, &None, &name_t, &end_t),
            name: value(&name_t),
            args: args.map(|node| Box::new(node)),
            body: body.map(|node| Box::new(node)),
        }
    }

    pub fn def_endless_method(&self, def_t: Token, name_t: Token, args: Option<Node>, assignment_t: Token, body: Option<Node>) -> Node {
        self.check_reserved_for_numparam(&value(&name_t), &loc(&name_t));

        Node::Def {
            loc: endless_definition_map(&def_t, &None, &name_t, &assignment_t, &body.as_ref()),
            name: value(&name_t),
            args: args.map(|node| Box::new(node)),
            body: body.map(|node| Box::new(node)),
        }
    }

    pub fn def_singleton(&self, def_t: Token, definee: Node, dot_t: Token, name_t: Token, args: Option<Node>, body: Option<Node>, end_t: Token) -> Node {
        self.validate_definee(&definee);
        self.check_reserved_for_numparam(&value(&name_t), &loc(&name_t));

        Node::Defs {
            loc: definition_map(&def_t, &Some(&dot_t), &name_t, &end_t),
            definee: Box::new(definee),
            name: value(&name_t),
            args: args.map(|node| Box::new(node)),
            body: body.map(|node| Box::new(node)),
        }
    }

    pub fn def_endless_singleton(&self, def_t: Token, definee: Node, dot_t: Token, name_t: Token, args: Option<Node>, assignment_t: Token, body: Option<Node>) -> Node {
        self.validate_definee(&definee);
        self.check_reserved_for_numparam(&value(&name_t), &loc(&name_t));

        Node::Defs {
            loc: endless_definition_map(&def_t, &Some(&dot_t), &name_t, &assignment_t, &body.as_ref()),
            definee: Box::new(definee),
            name: value(&name_t),
            args: args.map(|node| Box::new(node)),
            body: body.map(|node| Box::new(node)),
        }
    }

    pub fn undef_method(&self, undef_t: Token, names: Vec<Node>) -> Node {
        let loc = keyword_map(&undef_t, &None, &names.iter().collect(), &None);
        Node::Undef {
            names,
            loc
        }
    }

    pub fn alias(&self, alias_t: Token, to: Node, from: Node) -> Node {
        let loc = keyword_map(&alias_t, &None, &vec![&from, &to], &None);
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
                let loc = collection_map(&begin_t.as_ref(), &args.iter().collect(), &end_t.as_ref());
                Some(
                    Node::Args {
                        args,
                        loc
                    }
                )
            }
        }
    }

    pub fn forward_only_args(&self, begin_t: Token, dots_t: Token, end_t: Token) -> Node {
        let args = vec![ self.forward_arg(dots_t) ];
        Node::Args {
            loc: collection_map(&Some(&begin_t), &args.iter().collect(), &Some(&end_t)),
            args,
        }
    }

    pub fn forward_arg(&self, dots_t: Token) -> Node {
        Node::ForwardArg {
            loc: token_map(&dots_t)
        }
    }

    pub fn arg(&self, name_t: Token) -> Node {
        self.check_reserved_for_numparam(&value(&name_t), &loc(&name_t));
        Node::Arg {
            name: value(&name_t),
            loc: variable_map(&name_t)
        }
    }

    pub fn optarg(&self, name_t: Token, eql_t: Token, value_node: Node) -> Node {
        self.check_reserved_for_numparam(&value(&name_t), &loc(&name_t));
        Node::Optarg {
            loc: VariableMap {
                operator: Some(loc(&eql_t)),
                name: Some(loc(&name_t)),
                expression: loc(&name_t).join(&value_node.expression())
            },
            name: value(&name_t),
            value: Box::new(value_node),
        }
    }

    pub fn restarg(&self, star_t: Token, name_t: Option<Token>) -> Node {
        let map = arg_prefix_map(&star_t, &name_t.as_ref());
        let name = match name_t {
            Some(name_t) => {
                self.check_reserved_for_numparam(&value(&name_t), &loc(&name_t));
                Some(value(&name_t))
            },
            _ => None
        };
        Node::Restarg { loc: map, name }
    }

    pub fn kwarg(&self, name_t: Token) -> Node {
        self.check_reserved_for_numparam(&value(&name_t), &loc(&name_t));
        Node::Kwarg {
            loc: kwarg_map(&name_t, &None),
            name: value(&name_t)
        }
    }

    pub fn kwoptarg(&self, name_t: Token, value_node: Node) -> Node {
        self.check_reserved_for_numparam(&value(&name_t), &loc(&name_t));

        Node::Kwoptarg {
            loc: kwarg_map(&name_t, &Some(&value_node)),
            name: value(&name_t),
            value: Box::new(value_node)
        }
    }

    pub fn kwrestarg(&self, dstar_t: Token, name_t: Option<Token>) -> Node {
        let map = arg_prefix_map(&dstar_t, &name_t.as_ref());
        let name = match name_t {
            Some(name_t) => {
                self.check_reserved_for_numparam(&value(&name_t), &loc(&name_t));
                Some(value(&name_t))
            },
            _ => {
                None
            }
        };

        Node::Kwrestarg { name, loc: map }
    }

    pub fn kwnilarg(&self, dstar_t: Token, nil_t: Token) -> Node {
        Node::Kwnilarg { loc: arg_prefix_map(&dstar_t, &Some(&nil_t)) }
    }

    pub fn shadowarg(&self, name_t: Token) -> Node {
        self.check_reserved_for_numparam(&value(&name_t), &loc(&name_t));
        Node::Shadowarg {
            loc: variable_map(&name_t),
            name: value(&name_t)
        }
    }

    pub fn blockarg(&self, amper_t: Token, name_t: Token) -> Node {
        self.check_reserved_for_numparam(&value(&name_t), &loc(&name_t));
        Node::Blockarg {
            name: value(&name_t),
            loc: arg_prefix_map(&amper_t, &Some(&name_t)),
        }
    }

    pub fn procarg0(&self, arg: Node) -> Node {
        Node::Procarg0 {
            loc: CollectionMap {
                begin: None,
                end: None,
                expression: arg.expression().clone()
            },
            arg: Box::new(arg)
        }
    }

    //
    // Method calls
    //

    fn call_type_for_dot(&self, dot_t: &Option<Token>) -> MethodCallType {
        match dot_t {
            Some((Lexer::tANDDOT, _, _)) => MethodCallType::CSend,
            _ => MethodCallType::Send,
        }
    }

    pub fn forwarded_args(&self, dots_t: Token) -> Node {
        Node::ForwardedArgs { loc: token_map(&dots_t) }
    }

    pub fn call_method(&self, receiver: Option<Node>, dot_t: Option<Token>, selector_t: Option<Token>, lparen_t: Option<Token>, args: Vec<Node>, rparen_t: Option<Token>) -> Node {
        let loc = send_map(
            &receiver.as_ref(),
            &dot_t.as_ref(),
            &selector_t.as_ref(),
            &lparen_t.as_ref(),
            &args.iter().collect(),
            &rparen_t.as_ref()
        );
        let method_name = selector_t.map(|t| value(&t)).unwrap_or("call".to_owned());

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

    pub fn call_lambda(&self, lambda_t: Token) -> Node {
        Node::Lambda { loc: expr_map(&loc(&lambda_t)) }
    }

    pub fn block(&self, method_call: Node, begin_t: Token, block_args: ArgsType, body: Option<Node>, end_t: Token) -> Node {

        // let block_args = args.map(|node| Box::new(node));
        let block_body = body.map(|node| Box::new(node));

        match &method_call {
            Node::Yield { .. } => {
                // diagnostic :error, :block_given_to_yield, nil, method_call.loc.keyword, [loc(begin_t)]
            },
            Node::Send { args, .. }
            | Node::CSend { args, .. } => {
                match args.last() {
                    Some(Node::Blockarg { loc: VariableMap { expression: _, .. }, .. })
                    | Some(Node::ForwardedArgs { loc: Map { expression: _ }, .. }) => {
                        // diagnostic :error, :block_and_blockarg, nil, expression, [loc(begin_t)]
                    },
                    _ => {}
                }
            },
            _ => {}
        }

        let rewrite_args_and_loc = |method_args: &Vec<Node>, loc: &KeywordMap, block_args: ArgsType, block_body: Option<Box<Node>>| {
            // Code like "return foo 1 do end" is reduced in a weird sequence.
            // Here, method_call is actually (return).
            let actual_send = method_args[0].clone();

            let block = match block_args {
                ArgsType::Args(args) => {
                    Node::Block {
                        loc: block_map(&actual_send.expression(), &begin_t, &end_t),
                        call: Box::new(actual_send),
                        args: args.map(Box::new),
                        body: block_body,
                    }
                }
                ArgsType::Numargs(numargs) => {
                    Node::Numblock {
                        loc: block_map(&actual_send.expression(), &begin_t, &end_t),
                        call: Box::new(actual_send),
                        numargs,
                        body: block_body.unwrap(),
                    }
                }
            };

            let loc = KeywordMap {
                keyword: loc.keyword.clone(),
                begin: loc.begin.clone(),
                end: loc.end.clone(),
                expression: loc.expression.join(&block.expression())
            };
            let args = vec![block];

            (args, loc)
        };

        match &method_call {
            Node::Send { .. }
            | Node::CSend { .. }
            | Node::Index { .. }
            | Node::Super { .. }
            | Node::Zsuper { .. }
            | Node::Lambda { .. }
            => {
                let loc = block_map(&method_call.expression(), &begin_t, &end_t);
                match block_args {
                    ArgsType::Args(args) => {
                        Node::Block {
                            call: Box::new(method_call),
                            args: args.map(Box::new),
                            body: block_body,
                            loc
                        }
                    }
                    ArgsType::Numargs(numargs) => {
                        Node::Numblock {
                            numargs,
                            call: Box::new(method_call),
                            body: block_body.unwrap(),
                            loc
                        }
                    }
                }
            },
            Node::Return { args, loc } => {
                let (args, loc) = rewrite_args_and_loc(args, loc, block_args, block_body);
                Node::Return { args, loc }
            },
            Node::Next { args, loc } => {
                let (args, loc) = rewrite_args_and_loc(args, loc, block_args, block_body);
                Node::Next { args, loc }
            },
            Node::Break { args, loc } => {
                let (args, loc) = rewrite_args_and_loc(args, loc, block_args, block_body);
                Node::Break { args, loc }
            },
            _ => unreachable!("unsupported method call {:?}", method_call)
        }
    }
    pub fn block_pass(&self, amper_t: Token, arg: Node) -> Node {
        Node::BlockPass {
            loc: unary_op_map(&amper_t, &Some(&arg)),
            arg: Box::new(arg)
        }
    }

    pub fn attr_asgn(&self, receiver: Node, dot_t: Token, selector_t: Token) -> Node {
        let method_name = value(&selector_t) + "=";
        let loc = send_map(&Some(&receiver), &Some(&dot_t), &Some(&selector_t), &None, &vec![], &None);

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
        let loc = index_map(&receiver, &lbrack_t, &rbrack_t);
        Node::Index {
            receiver: Box::new(receiver),
            indexes,
            loc
        }
    }

    pub fn index_asgn(&self, receiver: Node, lbrack_t: Token, indexes: Vec<Node>, rbrack_t: Token) -> Node {
        let loc = index_map(&receiver, &lbrack_t, &rbrack_t);
        Node::IndexAsgn {
            receiver: Box::new(receiver),
            indexes,
            rhs: None,
            loc
        }
    }

    pub fn binary_op(&self, receiver: Node, operator_t: Token, arg: Node) -> Node {
        let source_map = send_binary_op_map(&receiver, &operator_t, &arg);
        Node::Send { receiver: Some(Box::new(receiver)), operator: value(&operator_t), args: vec![arg], loc: source_map }
    }

    pub fn match_op(&self, receiver: Node, match_t: Token, arg: Node) -> Node {
        let source_map = send_binary_op_map(&receiver, &match_t, &arg);
        let captures = self.static_regexp_captures(&receiver);
        if captures.is_empty() {
            Node::Send {
                receiver: Some(Box::new(receiver)),
                operator: String::from("=~"),
                args: vec![arg],
                loc: source_map
            }
        } else {
            for capture in captures {
                self.static_env.declare(&capture);
            }

            Node::MatchWithLvasgn {
                receiver: Box::new(receiver),
                arg: Box::new(arg),
                loc: source_map
            }
        }
    }

    pub fn unary_op(&self, op_t: Token, receiver: Node) -> Node {
        let loc = send_unary_op_map(&op_t, &Some(&receiver));
        let op = value(&op_t);
        let method = if op == "+" || op == "-" {
            op + "@"
        } else {
            op
        };
        Node::Send {
            receiver: Some(Box::new(receiver)),
            operator: method,
            args: vec![],
            loc
        }
    }

    pub fn not_op(&self, not_t: Token, begin_t: Option<Token>, receiver: Option<Node>, end_t: Option<Token>) -> Node {
        if let Some(receiver) = receiver {
            Node::Send {
                loc: send_map(&None, &None, &Some(&not_t), &begin_t.as_ref(), &vec![&receiver], &end_t.as_ref()),
                receiver: Some(Box::new(self.check_condition(receiver))),
                operator: "!".to_owned(),
                args: vec![],
            }
        } else {
            let nil_node = Node::Begin {
                statements: vec![],
                loc: collection_map(&begin_t.as_ref(), &vec![], &end_t.as_ref())
            };
            Node::Send {
                loc: send_unary_op_map(&not_t, &Some(&nil_node)),
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
        let loc = binary_op_map(&lhs, &op_t, &rhs);
        let lhs = Box::new(lhs);
        let rhs = Box::new(rhs);
        match type_ {
            LogicalOp::And => Node::And { lhs, rhs, loc },
            LogicalOp::Or => Node::Or { lhs, rhs, loc },
        }
    }

    // Conditionals

    pub fn condition(&self, cond_t: Token, cond: Node, then_t: Token, if_true: Option<Node>, else_t: Option<Token>, if_false: Option<Node>, end_t: Option<Token>) -> Node {
        Node::If {
            loc: condition_map(&cond_t, &Some(&cond), &Some(&then_t), &if_true.as_ref(), &else_t.as_ref(), &if_false.as_ref(), &end_t.as_ref()),
            cond: Box::new(self.check_condition(cond)),
            if_true: if_true.map(|node| Box::new(node)),
            if_false: if_false.map(|node| Box::new(node)),
        }
    }

    pub fn condition_mod(&self, if_true: Option<Node>, if_false: Option<Node>, cond_t: Token, cond: Node) -> Node {
        let pre = match (&if_true, &if_false) {
            (None, None) => panic!("at least one of if_true/if_false is required"),
            (None, Some(if_false)) => if_false.clone(),
            (Some(if_true), None) => if_true.clone(),
            (Some(_), Some(_)) => panic!("only one of if_true/if_false is required")
        };

        let loc = keyword_mod_map(&pre, &cond_t, &cond);
        Node::IfMod {
            cond: Box::new(self.check_condition(cond)),
            if_true: if_true.map(|node| Box::new(node)),
            if_false: if_false.map(|node| Box::new(node)),
            loc
        }
    }

    pub fn ternary(&self, cond: Node, question_t: Token, if_true: Node, colon_t: Token, if_false: Node) -> Node {
        Node::IfTernary {
            loc: ternary_map(&cond, &question_t, &if_true, &colon_t, &if_false),
            cond: Box::new(cond),
            if_true: Box::new(if_true),
            if_false: Box::new(if_false)
        }
    }

    // Case matching

    pub fn when(&self, when_t: Token, patterns: Vec<Node>, then_t: Token, body: Option<Node>) -> Node {
        let loc = if let Some(body) = &body {
            let args = [ patterns.iter().collect(), vec![body] ].concat();
            keyword_map(&when_t, &Some(&then_t), &args, &None)
        } else {
            keyword_map(&when_t, &Some(&then_t), &patterns.iter().collect(), &None)
        };

        Node::When {
            patterns,
            body: body.map(|node| Box::new(node)),
            loc
        }
    }

    pub fn case(&self, case_t: Token, expr: Option<Node>, when_bodies: Vec<Node>, else_t: Option<Token>, else_body: Option<Node>, end_t: Token) -> Node {
        let loc = condition_map(
            &case_t,
            &expr.as_ref(),
            &None,
            &None,
            &else_t.as_ref(),
            &else_body.as_ref(),
            &Some(&end_t)
        );
        Node::Case {
            loc,
            expr: expr.map(Box::new),
            when_bodies,
            else_body: else_body.map(Box::new)
        }
    }

    // Loops

    pub fn loop_(&self, loop_type: LoopType, keyword_t: Token, cond: Node, do_t: Token, body: Option<Node>, end_t: Token) -> Node {
        let loc = keyword_map(&keyword_t, &Some(&do_t), &vec![], &Some(&end_t));
        let cond = Box::new(self.check_condition(cond));
        let body = body.map(Box::new);

        match loop_type {
            LoopType::While => Node::While { cond, body, loc },
            LoopType::Until => Node::Until { cond, body, loc }
        }
    }

    pub fn loop_mod(&self, loop_type: LoopType, body: Node, keyword_t: Token, cond: Node) -> Node {
        let loc = keyword_mod_map(&body, &keyword_t, &cond);
        let cond = Box::new(self.check_condition(cond));

        match (loop_type, &body) {
            (LoopType::While, Node::KwBegin { .. }) => Node::WhilePost { cond, body: Box::new(body), loc },
            (LoopType::While, _)                    => Node::While     { cond, body: Some(Box::new(body)), loc },
            (LoopType::Until, Node::KwBegin { .. }) => Node::UntilPost { cond, body: Box::new(body), loc },
            (LoopType::Until, _)                    => Node::Until     { cond, body: Some(Box::new(body)), loc },
        }
    }

    pub fn for_(&self, for_t: Token, iterator: Node, in_t: Token, iteratee: Node, do_t: Token, body: Option<Node>, end_t: Token) -> Node {
        Node::For {
            loc: for_map(&for_t, &in_t, &do_t, &end_t),
            iterator: Box::new(iterator),
            iteratee: Box::new(iteratee),
            body: body.map(|node| Box::new(node))
        }
    }

    // Keywords

    pub fn keyword_cmd(&self, type_: KeywordCmd, keyword_t: Token, lparen_t: Option<Token>, args: Vec<Node>, rparen_t: Option<Token>) -> Node {
        if type_ == KeywordCmd::Yield && !args.is_empty() {
            match args.last() {
                Some(Node::BlockPass { .. }) => {
                    // diagnostic :error, :block_given_to_yield, nil, loc(keyword_t), [last_arg.loc.expression]
                },
                _ => {}
            }
        }

        let loc = keyword_map(&keyword_t, &lparen_t.as_ref(), &args.iter().collect(), &rparen_t.as_ref());

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
            loc: keyword_map(&preexe_t, &Some(&lbrace_t), &vec![], &Some(&rbrace_t))
        }
    }
    pub fn postexe(&self, postexe_t: Token, lbrace_t: Token, compstmt: Option<Node>, rbrace_t: Token) -> Node {
        Node::Postexe {
            body: compstmt.map(|node| Box::new(node)),
            loc: keyword_map(&postexe_t, &Some(&lbrace_t), &vec![], &Some(&rbrace_t))
        }
    }

    // Exception handling

    pub fn rescue_body(&self, rescue_t: Token, exc_list: Option<Node>, assoc_t: Option<Token>, exc_var: Option<Node>, then_t: Option<Token>, compound_stmt: Option<Node>) -> Node {
        let loc = rescue_body_map(&rescue_t, &exc_list.as_ref(), &assoc_t.as_ref(), &exc_var.as_ref(), &then_t.as_ref(), &compound_stmt.as_ref());

        Node::RescueBody {
            exc_list: exc_list.map(|node| Box::new(node)),
            exc_var: exc_var.map(|node| Box::new(node)),
            stmt: compound_stmt.map(|node| Box::new(node)),
            loc
        }
    }

    pub fn begin_body(&self, compound_stmt: Option<Node>, rescue_bodies: Vec<Node>, else_: Option<(Token, Option<Node>)>, ensure: Option<(Token, Option<Node>)>) -> Option<Node> {
        let mut result: Option<Node>;

        if !rescue_bodies.is_empty() {
            if let Some((else_t, else_)) = else_ {
                let loc = eh_keyword_map(
                    &compound_stmt.as_ref(),
                    &None,
                    &rescue_bodies.iter().collect(),
                    &Some(&else_t),
                    &else_.as_ref()
                );
                result = Some(
                        Node::Rescue {
                        body: compound_stmt.map(|node| Box::new(node)),
                        rescue_bodies,
                        else_: else_.map(Box::new),
                        loc
                    }
                )
            } else {
                let loc = eh_keyword_map(&compound_stmt.as_ref(), &None, &rescue_bodies.iter().collect(), &None, &None);
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
            let parts = if let Some(else_) = else_ { vec![else_] } else { vec![] };
            let loc = collection_map(&Some(&else_t), &parts.iter().collect(), &None);
            statements.push(
                Node::Begin {
                    statements: parts,
                    loc
                }
            );

            let loc = collection_map(&None, &statements.iter().collect(), &None);
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
            let body_es = if let Some(ensure) = &ensure { vec![ensure] } else { vec![] };
            let loc = eh_keyword_map(&result.as_ref(), &Some(&ensure_t), &body_es, &None, &None);

            result = Some(
                Node::Ensure {
                    body: result.map(|node| Box::new(node)),
                    ensure: ensure.map(Box::new),
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
            let source_map = collection_map(&None, &statements.iter().collect(), &None);
            Some(Node::Begin { statements, loc: source_map })
        }
    }

    pub fn begin(&self, begin_t: Token, body: Option<Node>, end_t: Token) -> Node {
        if let Some(body) = body {
            match body {
                // Synthesized (begin) from compstmt "a; b" or (mlhs)
                // from multi_lhs "(a, b) = *foo".
                Node::Mlhs { loc: CollectionMap { expression, .. }, items } => {
                    let loc = CollectionMap {
                        begin: Some(loc(&begin_t)),
                        end: Some(loc(&end_t)),
                        expression
                    };
                    Node::Mlhs { items, loc }
                }
                Node::Begin { loc: CollectionMap { begin: None, end: None, expression }, statements } => {
                    let loc = CollectionMap {
                        begin: Some(loc(&begin_t)),
                        end: Some(loc(&end_t)),
                        expression
                    };
                    Node::Begin { statements, loc }
                }
                body => {
                    let statements = vec![body];
                    Node::Begin {
                        loc: collection_map(&Some(&begin_t), &statements.iter().collect(), &Some(&end_t)),
                        statements
                    }
                }
            }
        } else {
            // A nil expression: `()'.
            Node::Begin {
                statements: vec![],
                loc: collection_map(&Some(&begin_t), &vec![], &Some(&end_t))
            }
        }
    }

    pub fn begin_keyword(&self, begin_t: Token, body: Option<Node>, end_t: Token) -> Node {
        match body {
            None => {
                // A nil expression: `begin end'.
                Node::KwBegin  {
                    statements: vec![],
                    loc: collection_map(&Some(&begin_t), &vec![], &Some(&end_t))
                }
            },
            Some(Node::Begin { statements, loc: CollectionMap { begin: None, end: None, .. }, .. }) => {
                // Synthesized (begin) from compstmt "a; b".
                let loc = collection_map(&Some(&begin_t), &statements.iter().collect(), &Some(&end_t));
                Node::KwBegin {
                    statements,
                    loc
                }
            },
            Some(node) => {
                let statements = vec![node];
                let loc = collection_map(&Some(&begin_t), &statements.iter().collect(), &Some(&end_t));
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

    pub fn case_match(&self, case_t: Token, expr: Node, in_bodies: Vec<Node>, else_t: Option<Token>, else_body: Option<Node>, end_t: Token) -> Node {
        let else_body = match (&else_t, &else_body) {
            (Some(else_t), None) => {
                Some( Node::EmptyElse { loc: token_map(else_t) } )
            }
            _ => else_body,
        };

        Node::CaseMatch {
            loc: condition_map(&case_t, &Some(&expr), &None, &None, &else_t.as_ref(), &else_body.as_ref(), &Some(&end_t)),
            expr: Box::new(expr),
            in_bodies,
            else_body: else_body.map(Box::new)
        }
    }

    pub fn in_match(&self, lhs: Node, in_t: Token, rhs: Node) -> Node {
        Node::InMatch {
            loc: binary_op_map(&lhs, &in_t, &rhs),
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub fn in_pattern(&self, in_t: Token, pattern: Node, guard: Option<Node>, then_t: Token, body: Option<Node>) -> Node {
        let mut children = vec![&pattern];
        if let Some(guard) = &guard { children.push(&guard) }
        if let Some(body) = &body { children.push(&body) }

        Node::InPattern {
            loc: keyword_map(&in_t, &Some(&then_t), &children, &None),
            pattern: Box::new(pattern),
            guard: guard.map(Box::new),
            body: body.map(Box::new)
        }
    }

    pub fn if_guard(&self, if_t: Token, if_body: Node) -> Node {
        Node::IfGuard {
            loc: guard_map(&if_t, &if_body),
            body: Box::new(if_body)
        }
    }
    pub fn unless_guard(&self, unless_t: Token, unless_body: Node) -> Node {
        Node::UnlessGuard {
            loc: guard_map(&unless_t, &unless_body),
            body: Box::new(unless_body)
        }
    }

    pub fn match_var(&self, name_t: Token) -> Node {
        let name = value(&name_t);
        let name_l = loc(&name_t);

        self.check_lvar_name(&name, &name_l);
        self.check_duplicate_pattern_variable(&name, &name_l);
        self.static_env.declare(&name);

        Node::MatchVar {
            name,
            loc: variable_map(&name_t)
        }
    }

    pub fn match_hash_var(&self, name_t: Token) -> Node {
        let name = value(&name_t);

        let expr_l = loc(&name_t);
        let name_l = expr_l.adjust(0, -1);

        self.check_lvar_name(&name, &name_l);
        self.check_duplicate_pattern_variable(&name, &name_l);
        self.static_env.declare(&name);

        Node::MatchVar {
            name,
            loc: VariableMap { name: Some(name_l), operator: None, expression: expr_l }
        }
    }
    pub fn match_hash_var_from_str(&self, begin_t: Token, mut strings: Vec<Node>, end_t: Token) -> Node {
        if strings.len() != 1 {
            // diagnostic :error, :pm_interp_in_var_name, nil, loc(begin_t).join(loc(end_t))
        }

        match strings.remove(0) {
            Node::Str { value, loc: CollectionMap { begin, end, expression } } => {
                let name = value.to_string_lossy();
                let mut name_l = expression.clone();

                self.check_lvar_name(&name, &name_l);
                self.check_duplicate_pattern_variable(&name, &name_l);

                self.static_env.declare(&name);

                match &begin {
                    Some(begin_l) => {
                        let begin_pos_d: i32 = begin_l.size().try_into().unwrap();
                        name_l = name_l.adjust(begin_pos_d, 0)
                    },
                    _ => {}
                }

                match &end {
                    Some(end_l) => {
                        let end_pos_d: i32 = end_l.size().try_into().unwrap();
                        name_l = name_l.adjust(0, -end_pos_d)
                    },
                    _ => {}
                }

                let expr_l = loc(&begin_t).join(&expression).join(&loc(&end_t));
                Node::MatchVar {
                    name,
                    loc: VariableMap {
                        name: Some(name_l),
                        operator: None,
                        expression: expr_l
                    }
                }

            },
            Node::Begin { statements, .. } => {
                self.match_hash_var_from_str(begin_t, statements, end_t)
            },
            _ => {
                // diagnostic :error, :pm_interp_in_var_name, nil, loc(begin_t).join(loc(end_t))
                panic!("missing diagnostic")
            }
        }
    }

    pub fn match_rest(&self, star_t: Token, name_t: Option<Token>) -> Node {
        let name = name_t.map(|t| self.match_var(t));
        Node::MatchRest {
            loc: unary_op_map(&star_t, &name.as_ref()),
            name: name.map(Box::new)
        }
    }

    pub fn hash_pattern(&self, lbrace_t: Option<Token>, kwargs: Vec<Node>, rbrace_t: Option<Token>) -> Node {
        self.check_duplicate_args(&kwargs);
        Node::HashPattern {
            loc: collection_map(&lbrace_t.as_ref(), &kwargs.iter().collect(), &rbrace_t.as_ref()),
            args: kwargs,
        }
    }

    pub fn array_pattern(&self, lbrack_t: Option<Token>, elements: Vec<Node>, rbrack_t: Option<Token>) -> Node {
        let loc = collection_map(&lbrack_t.as_ref(), &elements.iter().collect(), &rbrack_t.as_ref());

        if elements.is_empty() {
            return Node::ArrayPattern { elements: vec![], loc }
        }

        let mut trailing_comma = false;
        let nodes_elements = elements.into_iter().map(|element| {
            match element {
                Node::MatchWithTrailingComma { match_, .. } => {
                    trailing_comma = true;
                    *match_
                }
                e => e
            }
        }).collect::<Vec<_>>();

        if trailing_comma {
            Node::ArrayPatternWithTail { elements: nodes_elements, loc }
        } else {
            Node::ArrayPattern { elements: nodes_elements, loc }
        }
    }

    pub fn find_pattern(&self, lbrack_t: Option<Token>, elements: Vec<Node>, rbrack_t: Option<Token>) -> Node {
        Node::FindPattern {
            loc: collection_map(&lbrack_t.as_ref(), &elements.iter().collect(), &rbrack_t.as_ref()),
            elements
        }
    }

    pub fn match_with_trailing_comma(&self, match_: Node, comma_t: Token) -> Node {
        Node::MatchWithTrailingComma {
            loc: expr_map(&match_.expression().join(&loc(&comma_t))),
            match_: Box::new(match_)
        }
    }

    pub fn const_pattern(&self, const_: Node, ldelim_t: Token, pattern: Node, rdelim_t: Token) -> Node {
        Node::ConstPattern {
            loc: CollectionMap {
                begin: Some(loc(&ldelim_t)),
                end: Some(loc(&rdelim_t)),
                expression: const_.expression().join(&loc(&rdelim_t))
            },
            const_: Box::new(const_),
            pattern: Box::new(pattern)
        }
    }

    pub fn pin(&self, pin_t: Token, var: Node) -> Node {
        Node::Pin {
            loc: send_unary_op_map(&pin_t, &Some(&var)),
            var: Box::new(var)
        }
    }

    pub fn match_alt(&self, left: Node, pipe_t: Token, right: Node) -> Node {
        let loc = binary_op_map(&left, &pipe_t, &right);
        Node::MatchAlt {
            left: Box::new(left),
            right: Box::new(right),
            loc
        }
    }

    pub fn match_as(&self, value: Node, assoc_t: Token, as_: Node) -> Node {
        let loc = binary_op_map(&value, &assoc_t, &as_);
        Node::MatchAs {
            value: Box::new(value),
            as_: Box::new(as_),
            loc
        }
    }

    pub fn match_nil_pattern(&self, dstar_t: Token, nil_t: Token) -> Node {
        Node::MatchNilPattern {
            loc: arg_prefix_map(&dstar_t, &Some(&nil_t))
        }
    }

    pub fn match_pair(&self, p_kw_label: PKwLabel, value_node: Node) -> Node {
        match p_kw_label {
            PKwLabel::PlainLabel(label_t) => {
                self.check_duplicate_pattern_key(&value(&label_t), &loc(&label_t));
                self.pair_keyword(label_t, value_node)
            }
            PKwLabel::QuotedLabel((begin_t, parts, end_t)) => {
                let label_loc = loc(&begin_t).join(&loc(&end_t));

                match self.static_string(&parts) {
                    Some(var_name) => self.check_duplicate_pattern_key(&var_name, &label_loc),
                    _ => {} /* diagnostic :error, :pm_interp_in_var_name, nil, label_loc */
                }

                self.pair_quoted(begin_t, parts, end_t, value_node)
            }
        }
    }

    pub fn match_label(&self, p_kw_label: PKwLabel) -> Node {
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

    pub fn check_condition(&self, cond: Node) -> Node {
        match cond {
            Node::Begin { statements, loc } if statements.len() == 1 => {
                let stmt = statements[statements.len() - 1].clone();
                let stmt = self.check_condition(stmt);
                Node::Begin { statements: vec![stmt], loc: loc.clone() }
            },
            Node::And { lhs, rhs, loc } => {
                Node::And {
                    lhs: Box::new(self.check_condition(*lhs)),
                    rhs: Box::new(self.check_condition(*rhs)),
                    loc
                }
            },
            Node::Or { lhs, rhs, loc } => {
                Node::Or {
                    lhs: Box::new(self.check_condition(*lhs)),
                    rhs: Box::new(self.check_condition(*rhs)),
                    loc
                }
            },
            Node::Irange { left, right, loc } => {
                Node::IFlipFlop {
                    left: left.map(|node| Box::new(self.check_condition(*node))),
                    right: right.map(|node| Box::new(self.check_condition(*node))),
                    loc
                }
            },
            Node::Erange { left, right, loc } => {
                Node::EFlipFlop {
                    left: left.map(|node| Box::new(self.check_condition(*node))),
                    right: right.map(|node| Box::new(self.check_condition(*node))),
                    loc
                }
            },
            _ => cond
        }
    }

    pub fn check_duplicate_args(&self, _args: &Vec<Node>) {}
    pub fn check_duplicate_arg(&self) {}
    pub fn check_assignment_to_numparam(&self, _name: &str, _loc: &Range){
    }

    pub fn check_reserved_for_numparam(&self, name: &str, _loc: &Range) {
        if name.len() != 2 { return }

        let c1 = name.chars().nth(0).unwrap();
        let c2 = name.chars().nth(1).unwrap();

        if c1 == '0' && (c2 >= '1' && c2 <= '9') {
            // diagnostic :error, "reserved_for_numparam"
        }
    }

    pub fn arg_name_collides(&self, _this_name: &str, _that_name: &str) -> bool { false }
    pub fn check_lvar_name(&self, _name: &str, _loc: &Range) {}
    pub fn check_duplicate_pattern_variable(&self, _name: &str, _loc: &Range) {}
    pub fn check_duplicate_pattern_key(&self, _name: &str, _loc: &Range) {}

    //
    // Helpers
    //

    pub fn static_string(&self, nodes: &Vec<Node>) -> Option<String> {
        let mut result = String::from("");

        for node in nodes {
            match node {
                Node::Str { value, .. } => {
                    let value = value.to_string_lossy();
                    result.push_str(&value)
                },
                Node::Begin { statements, .. } => {
                    if let Some(s) = self.static_string(statements) {
                        result.push_str(&s)
                    } else {
                        return None
                    }
                },
                _ => return None
            }
        }

        Some(result)
    }

    pub fn static_regexp(&self, parts: &Vec<Node>, options: &Vec<char>) -> Option<Regex> {
        if let Some(source) = self.static_string(&parts) {
            let mut reg_options = RegexOptions::REGEX_OPTION_NONE;
            reg_options |= RegexOptions::REGEX_OPTION_CAPTURE_GROUP;
            if options.contains(&'x') {
                reg_options |= RegexOptions::REGEX_OPTION_EXTEND;
            }
            let regex = Regex::with_options(&source, reg_options, onig::Syntax::default());

            match regex {
                Ok(regex) => return Some(regex),
                Err(err) => println!("Failed to process static regex source, got error {:?}", err)
            }
        }

        None
    }

    pub fn static_regexp_captures(&self, node: &Node) -> Vec<String> {
        match node {
            Node::Regexp { parts, options, .. } => {
                match &**options {
                    Node::RegOpt { options, .. } => {
                        if let Some(regex) = self.static_regexp(parts, options) {
                            let mut result: Vec<String> = vec![];

                            regex.foreach_name(|name, _| {
                                result.push(name.to_owned());
                                true
                            });

                            return result;
                        }
                    },
                    _ => {}
                }
            },
            _ => {}
        }

        vec![]
    }

    pub fn collapse_string_parts(&self) {}

    pub fn string_value(&self) {}

    pub fn diagnostic(&self) {}
    pub fn validate_definee(&self, _definee: &Node) {}

}
