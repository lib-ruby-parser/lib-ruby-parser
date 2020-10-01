use onig::{Regex, RegexOptions};
use crate::source::Range;
use crate::{Lexer, Node, Token, StaticEnvironment, Context, CurrentArgStack};
use crate::source::map::*;
use crate::map_builder::*;

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

    pub fn string(&self) {}

    pub fn string_internal(&self, string_t: Token) -> Node {
        let loc = unquoted_map(&string_t);
        let (_, bytes, _) = string_t;
        Node::Str { value: bytes, loc }
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
                                    loc: string_map(&begin_t.as_ref(), &parts.iter().collect(), &end_t.as_ref())
                                }
                            },
                            _ => unreachable!()
                        }
                    }
                },
                _ => {}
            }
        }

        Node::Dstr {
            loc: string_map(&begin_t.as_ref(), &parts.iter().collect(), &end_t.as_ref()),
            children: parts,
        }
    }

    pub fn character(&self, char_t: Token) -> Node {
        let loc = prefix_string_map(&char_t);
        let (_, bytes, _) = char_t;
        Node::Str { value: bytes, loc }
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
        let (_, bytes, _) = value_t;
        Node::Sym { name: bytes, loc }
    }

    pub fn symbol_internal(&self, symbol_t: Token) -> Node {
        let loc = unquoted_map(&symbol_t);
        let (_, bytes, _) = symbol_t;
        Node::Sym { name: bytes, loc }
    }

    pub fn symbol_compose(&self, begin_t: Token, parts: Vec<Node>, end_t: Token) -> Node {
        if parts.len() == 1 {
            let part = &parts[0];
            let value = match part {
                // collapse_string_parts? == true
                Node::Str { value, .. } => {
                    value.clone()
                }
                _ => unreachable!()
            };
            return Node::Sym {
                loc: collection_map(&Some(&begin_t), &vec![], &Some(&end_t)),
                name: value,
            }
        }

        Node::Dsym {
            loc: collection_map(&Some(&begin_t), &parts.iter().collect(), &Some(&end_t)),
            children: parts
        }
    }

    // Executable strings

    pub fn xstring_compose(&self, begin_t: Token, parts: Vec<Node>, end_t: Token) -> Node {
        Node::Xstr {
            loc: string_map(&Some(&begin_t), &parts.iter().collect(), &Some(&end_t)),
            children: parts
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
        let (_, bytes, _) = key_t;
        let key = Node::Sym {
            name: bytes,
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
                if self.static_env.is_declared(&name.as_bytes().to_vec()) {
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
                // if !context.is_dynamic_const_definition_allowed() {
                //     diagnostic :error, :dynamic_const, nil, node.loc.expression
                // }
                Node::Casgn { name, scope, loc, rhs: None }
            },
            Node::Lvar { name, loc: VariableMap { expression, .. } } => {
                let loc = expression.clone();

                self.check_assignment_to_numparam(&name, &loc);
                self.check_reserved_for_numparam(&name, &loc);

                self.static_env.declare(&name.as_bytes().to_vec());

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
            _ => panic!("{:#?} can't be used in assignment", lhs)
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

    pub fn numargs(&self) {}

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

    pub fn block(&self, method_call: Node, begin_t: Token, args: Option<Node>, body: Option<Node>, end_t: Token) -> Node {
        let block_args = args.map(|node| Box::new(node));
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

        let rewrite_args_and_loc = |args: &Vec<Node>, loc: &KeywordMap, block_args: Option<Box<Node>>, block_body: Option<Box<Node>>| {
            // Code like "return foo 1 do end" is reduced in a weird sequence.
            // Here, method_call is actually (return).
            let actual_send = args[0].clone();

            let block = Node::Block {
                loc: block_map(&actual_send.expression(), &begin_t, &end_t),
                call: Box::new(actual_send),
                args: block_args,
                body: block_body,
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
            // | Node::Lambda { .. }
            => {
                let loc = block_map(&method_call.expression(), &begin_t, &end_t);
                return Node::Block {
                    call: Box::new(method_call),
                    args: block_args,
                    body: block_body,
                    loc
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
            _ => unreachable!()
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
                self.static_env.declare(&capture.as_bytes().to_vec());
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

    pub fn loop_(&self, _loop_type: LoopType, _keyword_t: Token, _cond: Node, _do_t: Token, _body: Option<Node>, _end_t: Token) -> Node {
        unimplemented!("loop_");
    }

    pub fn loop_mod(&self, loop_type: LoopType, body: Node, keyword_t: Token, cond: Node) -> Node {
        let loc = keyword_mod_map(&body, &keyword_t, &cond);
        let cond = Box::new(self.check_condition(cond));

        match (loop_type, &body) {
            (LoopType::While, Node::KwBegin { .. }) => Node::WhilePost { cond, body: Box::new(body), loc },
            (LoopType::While, _)                    => Node::While     { cond, body: Box::new(body), loc },
            (LoopType::Until, Node::KwBegin { .. }) => Node::UntilPost { cond, body: Box::new(body), loc },
            (LoopType::Until, _)                    => Node::Until     { cond, body: Box::new(body), loc },
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
            // match args.last() {
            //     Some(Node::BlockPass { .. }) => {
            //         diagnostic :error, :block_given_to_yield, nil, loc(keyword_t), [last_arg.loc.expression]
            //     }
            // }
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

    pub fn begin_body(&self, compound_stmt: Option<Node>, rescue_bodies: Vec<Node>, else_: Option<(Token, Node)>, ensure: Option<(Token, Node)>) -> Option<Node> {
        let mut result: Option<Node>;

        if !rescue_bodies.is_empty() {
            if let Some((else_t, else_)) = else_ {
                let loc = eh_keyword_map(
                    &compound_stmt.as_ref(),
                    &None,
                    &rescue_bodies.iter().collect(),
                    &Some(&else_t),
                    &Some(&else_)
                );
                result = Some(
                        Node::Rescue {
                        body: compound_stmt.map(|node| Box::new(node)),
                        rescue_bodies,
                        else_: Some(Box::new(else_)),
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
            let loc = collection_map(&Some(&else_t), &vec![&else_], &None);
            statements.push(
                Node::Begin {
                    statements: vec![else_],
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
            let loc = eh_keyword_map(&result.as_ref(), &Some(&ensure_t), &vec![&ensure], &None, &None);
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

    pub fn case_match(&self, _case_t: Token, _expr: Node, _in_bodies: Vec<Node>, _else_t: Option<Token>, _else_body: Option<Node>, _end_t: Token) -> Node {
        unimplemented!("case_match")
    }

    pub fn in_match(&self, _lhs: Node, _in_t: Token, _rhs: Node) -> Node { unimplemented!("in_match") }
    pub fn in_pattern(&self, _in_t: Token, _pattern: Node, _guard: Option<Node>, _then_t: Token, _body: Option<Node>) -> Node { unimplemented!("in_pattern") }

    pub fn if_guard(&self, _if_t: Token, _if_body: Node) -> Node { unimplemented!("if_guard") }
    pub fn unless_guard(&self, _unless_t: Token, _unless_body: Node) -> Node { unimplemented!("unless_guard") }

    pub fn match_var(&self, _name_t: Token) -> Node { unimplemented!("match_var") }
    pub fn match_hash_var(&self, _name_t: Token) -> Node { unimplemented!("match_hash_var") }
    pub fn match_hash_var_from_str(&self, _begin_t: Token, _strings: Vec<Node>, _end_t: Token) -> Node { unimplemented!("match_hash_var_from_str") }
    pub fn match_rest(&self, _star_t: Token, __name_t: Option<Token>) -> Node { unimplemented!("match_rest") }
    pub fn hash_pattern(&self, _lbrace_t: Option<Token>, _kwargs: Vec<Node>, _rbrace_t: Option<Token>) -> Node { unimplemented!("hash_pattern") }
    pub fn array_pattern(&self, _lbrack_t: Option<Token>, _elements: Vec<Node>, _rbrack_t: Option<Token>) -> Node { unimplemented!("array_pattern") }
    pub fn find_pattern(&self, _lbrack_t: Option<Token>, _elements: Vec<Node>, _rbrack_t: Option<Token>) -> Node { unimplemented!("find_pattern") }
    pub fn match_with_trailing_comma(&self, _match_: Node, _comma_t: Token) -> Node { unimplemented!("match_with_trailing_comma") }
    pub fn const_pattern(&self, _const_: Node, _ldelim_t: Token, _pattern: Node, _rdelim_t: Token) -> Node { unimplemented!("const_pattern") }
    pub fn pin(&self, _pin_t: Token, _var: Node) -> Node { unimplemented!("pin") }
    pub fn match_alt(&self, _left: Node, _pipe_t: Token, _right: Node) -> Node { unimplemented!("match_alt") }
    pub fn match_as(&self, _value: Node, _assoc_t: Token, _as_: Node) -> Node { unimplemented!("match_as") }
    pub fn match_nil_pattern(&self, _dstar_t: Token, _nil_t: Token) -> Node { unimplemented!("match_nil_pattern") }
    pub fn match_pair(&self, _p_kw_label: PKwLabel, _value: Node) -> Node { unimplemented!("match_pair") }
    pub fn match_label(&self, _p_kw_label: PKwLabel) -> Node { unimplemented!("match_label") }

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
    // Helpers
    //

    pub fn static_string(&self, nodes: &Vec<Node>) -> Option<String> {
        let mut result = String::from("");

        for node in nodes {
            match node {
                Node::Str { value, .. } => {
                    let value = String::from_utf8_lossy(&value).into_owned();
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
