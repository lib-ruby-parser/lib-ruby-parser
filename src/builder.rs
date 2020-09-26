use std::rc::Rc;
use std::cell::RefCell;
use crate::source::Range;
use crate::{Lexer, Node, Token, StaticEnvironment, Context, CurrentArgStack};
use crate::source::map::*;
use crate::parser::UserVariable;

#[derive(Debug, Clone, PartialEq)]
pub enum PartialAssignment {
    Node(Node),
    Ident(Token),
    IndexAsgn((Node, Token, Vec<Node>, Token)), // a[b, c]
    AttrAsgn((Node, Token, Token)), // A::B
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

    pub fn float() {}
    pub fn rational() {}
    pub fn complex() {}
    pub fn unary_num() {}

    pub fn __line__(&self, line_t: Token) -> Node {
        Node::__LINE__ {
            loc: self.token_map(&line_t)
        }
    }

    // Strings

    pub fn string() {}
    pub fn string_internal() {}
    pub fn string_compose() {}
    pub fn character() {}

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

    pub fn symbol_compose() {}

    // Executable strings

    pub fn xstring_compose() {}

    // Indented (interpolated, noninterpolated, executable) strings

    pub fn dedent_string() {}

    // Regular expressions

    pub fn regexp_options() {}
    pub fn regexp_compose() {}

    // Arrays

    pub fn array() {}
    pub fn splat() {}
    pub fn word() {}
    pub fn words_compose() {}
    pub fn symbols_compose() {}

    // Hashes

    pub fn pair() {}
    pub fn pair_list_18() {}
    pub fn pair_keyword() {}
    pub fn pair_quoted() {}
    pub fn kwsplat() {}
    pub fn associate() {}

    // Ranges

    pub fn range_inclusive() {}
    pub fn range_exclusive() {}

    //
    // Access
    //

    pub fn self_(&self, token: Token) -> Node {
        Node::Self_ {
            loc: self.token_map(&token)
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
    pub fn accessible_ident(&self, ident_t: Token) -> Node {
        let name = self.value(&ident_t);

        if self.static_env.is_declared(&name) {
            if let Some(current_arg) = self.current_arg_stack.top() {
                if current_arg == name {
                    // diagnostic :error, :circular_argument_reference,
                    //        { :var_name => name.to_s }, node.loc.expression
                }
            }

            Node::Lvar {
                name,
                loc: self.variable_map(&ident_t)
            }
        } else {
            Node::Send {
                receiver: None,
                operator: name,
                args: vec![],
                loc: self.var_send_map(&ident_t)
            }
        }
    }

    pub fn accessible_node(&self, node: Node) -> Node {
        unimplemented!()
    }

    pub fn const_(&self, name_t: Token) -> Node {
        Node::Const {
            scope: None,
            name: self.value(&name_t),
            loc: self.constant_map(&None, &None, &name_t)
        }
    }

    pub fn const_global(&self, t_colon3: Token, name_t: Token) -> Node {
        unimplemented!()
    }

    pub fn const_fetch(&self, scope: Node, t_colon2: Token, name_t: Token) -> Node {
        unimplemented!()
    }

    pub fn __encoding__(&self, _encoding_t: Token) -> Node {
        Node::__ENCODING__ {
            loc: self.token_map(&_encoding_t)
        }
    }

    //
    // Assignments
    //

    pub fn assignable_ident(&self, ident_t: Token) -> PartialAssignment {
        // wq/parser :ident handling
        let var_name = self.value(&ident_t);
        let name_loc = self.loc(&ident_t);

        self.check_assignment_to_numparam(&var_name, &name_loc);
        self.check_reserved_for_numparam(&var_name, &name_loc);

        self.static_env.declare(&var_name);

        PartialAssignment::Ident(ident_t)
    }

    pub fn assignable_node(&self, node: Node) -> PartialAssignment {
        match &node {
            Node::Cvar {..}
            | Node::Ivar { .. }
            | Node::Gvar { .. } => {
                // ok
            },
            Node::Const { loc, .. } => {
                if !self.context.is_dynamic_const_definition_allowed() {
                    // diagnostic :error, :dynamic_const, nil, node.loc.expression
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
            },
            Node::BackRef { .. }
            | Node::NthRef { .. } => {
                // diagnostic :error, :backref_assignment, nil, node.loc.expression
            },
            _ => {
                panic!("Unsupported assignable node {:#?}", node)
            }
        }

        PartialAssignment::Node(node)
    }

    pub fn const_op_assignable() {}

    pub fn assign(&self, lhs: PartialAssignment, eql_t: Token, rhs: Node) -> Node {
        let operator_l = Some(self.loc(&eql_t));

        let lhs: Node = match lhs {
            PartialAssignment::Ident(ident_t) => {
                let expression_l = self.loc(&ident_t).join(&rhs.expression());
                Node::Lvasgn {
                    name: self.value(&ident_t),
                    rhs: Box::new(rhs),
                    loc: VariableMap {
                        expression: expression_l,
                        operator: Some(self.loc(&eql_t))
                    }
                }
            },
            PartialAssignment::Node(node) => {
                let loc = VariableMap {
                    expression: self.join_expr(&node, &rhs),
                    operator: operator_l
                };

                match node {
                    Node::Cvar { name, .. } => {
                        Node::Cvasgn { name, rhs: Box::new(rhs), loc }
                    },
                    Node::Ivar { name, .. } => {
                        Node::Ivasgn { name, rhs: Box::new(rhs), loc }
                    },
                    Node::Gvar { name, .. } => {
                        Node::Gvasgn { name, rhs: Box::new(rhs), loc }
                    },
                    Node::Const { name, .. } => {
                        Node::Casgn { name, rhs: Box::new(rhs), loc }
                    },
                    _ => panic!("impossible")
                }
            },
            PartialAssignment::IndexAsgn(( receiver, lbrack_t, indexes, rbrack_t )) => {
                let mut loc = self.index_map(&receiver, &lbrack_t, &rbrack_t, &eql_t);
                loc.expression = loc.expression.join(&rhs.expression());
                Node::IndexAsgn {
                    receiver: Box::new(receiver),
                    indexes,
                    rhs: Box::new(rhs),
                    loc
                }
            },
            PartialAssignment::AttrAsgn(( receiver, dot_t, selector_t )) => {
                let method_name = self.value(&selector_t) + "";
                let mut loc = self.send_map(&Some(receiver.clone()), &dot_t, &Some(selector_t.clone()), &None, &vec![], &None);
                loc.operator = Some(self.loc(&eql_t));
                loc.expression = loc.expression.join(&rhs.expression());
                if dot_t.0 == Lexer::tDOT {
                    Node::Send {
                        operator: method_name,
                        receiver: Some(Box::new(receiver)),
                        loc,
                        args: vec![rhs]
                    }
                } else {
                    Node::CSend {
                        operator: method_name,
                        receiver: Some(Box::new(receiver)),
                        loc,
                        args: vec![rhs]
                    }
                }
            }
        };
        lhs
    }

    pub fn op_assign() {}
    pub fn multi_lhs() {}
    pub fn multi_assign() {}
    pub fn rassign() {}
    pub fn multi_rassign() {}

    //
    // Class and module definition
    //

    pub fn def_class() {}
    pub fn def_sclass() {}
    pub fn def_module() {}

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

    pub fn def_endless_method() {}
    pub fn def_singleton() {}
    pub fn def_endless_singleton() {}

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

    pub fn numargs() {}
    pub fn forward_only_args() {}
    pub fn forward_arg() {}

    pub fn arg(&self, name_t: Token) -> Node {
        self.check_reserved_for_numparam(&self.value(&name_t), &self.loc(&name_t));
        Node::Arg {
            name: self.value(&name_t),
            loc: self.variable_map(&name_t)
        }
    }

    pub fn optarg() {}
    pub fn restarg() {}
    pub fn kwarg() {}
    pub fn kwoptarg() {}
    pub fn kwrestarg() {}
    pub fn kwnilarg() {}
    pub fn shadowarg() {}
    pub fn blockarg() {}
    pub fn procarg0() {}

    //
    // Method calls
    //

    pub fn forwarded_args() {}
    pub fn call_method() {}
    pub fn call_lambda() {}
    pub fn block() {}
    pub fn block_pass() {}
    pub fn attr_asgn() {}
    pub fn index() {}
    pub fn index_asgn() {}

    pub fn binary_op(&self, receiver: Node, operator_t: Token, arg: Node) -> Node {
        let source_map = self.send_binary_op_map(&receiver, &operator_t, &arg);
        Node::Send { receiver: Some(Box::new(receiver)), operator: self.value(&operator_t), args: vec![arg], loc: source_map }
    }

    pub fn match_op() {}
    pub fn unary_op() {}
    pub fn not_op() {}

    //
    // Control flow
    //

    // Logical operations: and, or

    pub fn logical_op() {}

    // Conditionals

    pub fn condition() {}
    pub fn condition_mod() {}
    pub fn ternary() {}

    // Case matching

    pub fn when() {}
    pub fn case() {}

    // Loops

    pub fn loop_() {}
    pub fn loop_mod() {}
    pub fn for_() {}

    // Keywords

    pub fn keyword_cmd() {}

    // BEGIN, END

    pub fn preexe(&self, preexe_t: Token, lbrace_t: Token, compstmt: Option<Node>, rbrace_t: Token) -> Node {
        Node::Preexe {
            body: compstmt.map(|node| Box::new(node)),
            loc: self.keyword_map(&preexe_t, &Some(lbrace_t), &vec![], &Some(rbrace_t))
        }
    }
    pub fn postexe() {}

    // Exception handling

    pub fn rescue_body() {}
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

    pub fn begin() {}
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

    pub fn case_match() {}
    pub fn in_match() {}
    pub fn in_pattern() {}
    pub fn if_guard() {}
    pub fn unless_guard() {}
    pub fn match_var() {}
    pub fn match_hash_var() {}
    pub fn match_hash_var_from_str() {}
    pub fn match_rest() {}
    pub fn hash_pattern() {}
    pub fn array_pattern() {}
    pub fn find_pattern() {}
    pub fn match_with_trailing_comma() {}
    pub fn const_pattern() {}
    pub fn pin() {}
    pub fn match_alt() {}
    pub fn match_as() {}
    pub fn match_nil_pattern() {}
    pub fn match_pair() {}
    pub fn match_label() {}

    //
    // Verification
    //

    pub fn check_condition() {}
    pub fn check_duplicate_args() {}
    pub fn check_duplicate_arg() {}
    pub fn check_assignment_to_numparam(&self, name: &str, loc: &Range){
    }

    pub fn check_reserved_for_numparam(&self, name: &str, loc: &Range) {
        if name.len() != 2 { return }

        let c1 = name.chars().nth(1).unwrap();
        let c2 = name.chars().nth(2).unwrap();

        if c1 == '0' && (c2 >= '1' && c2 <= '9') {
            // diagnostic :error, "reserved_for_numparam"
        }
    }

    pub fn arg_name_collides() {}
    pub fn check_lvar_name() {}
    pub fn check_duplicate_pattern_variable() {}
    pub fn check_duplicate_pattern_key() {}

    //
    // Source maps
    //

    pub fn join_expr(&self, left_expr: &Node, right_expr: &Node) -> Range {
        left_expr.expression().join(right_expr.expression())
    }

    pub fn token_map(&self, token: &Token) -> Map {
        Map { expression: self.loc(&token) }
    }

    pub fn delimited_string_map() {}
    pub fn prefix_string_map() {}

    pub fn unquoted_map(&self, token: &Token) -> CollectionMap {
        CollectionMap {
            begin: None,
            end: None,
            expression: self.loc(&token)
        }
    }

    pub fn pair_keyword_map() {}
    pub fn pair_quoted_map() {}
    pub fn expr_map() {}

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

    pub fn string_map() {}
    pub fn regexp_map() {}
    pub fn constant_map(&self, scope: &Option<Node>, colon2_t: &Option<Token>, name_t: &Token) -> ConstantMap {
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

    pub fn binary_op_map() {}
    pub fn unary_op_map() {}
    pub fn range_map() {}
    pub fn arg_prefix_map() {}
    pub fn kwarg_map() {}
    pub fn module_definition_map() {}

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

    pub fn endless_definition_map() {}

    pub fn send_map(&self, receiver_e: &Option<Node>, dot_t: &Token, selector_t: &Option<Token>, begin_t: &Option<Token>, args: &Vec<Node>, end_t: &Option<Token>) -> SendMap {
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
        } else {
            end_l = None
        }

        let expression_l = match (begin_l, end_l) {
            (Some(begin_l), Some(end_l)) => begin_l.join(&end_l),
            _ => panic!("unreachable")
        };

        SendMap {
            expression: expression_l,
            dot: Some(self.loc(dot_t)),
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

    pub fn send_unary_op_map() {}

    pub fn index_map(&self, receiver_e: &Node, lbrack_t: &Token, rbrack_t: &Token, operator_t: &Token) -> IndexMap {
        IndexMap {
            begin: self.loc(lbrack_t),
            end: self.loc(rbrack_t),
            expression: receiver_e.expression().join(&self.loc(rbrack_t)),
            operator: self.loc(operator_t)
        }
    }
    pub fn send_index_map() {}

    pub fn block_map() {}

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

    pub fn keyword_mod_map() {}
    pub fn condition_map() {}
    pub fn ternary_map() {}
    pub fn for_map() {}
    pub fn rescue_body_map() {}

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

    pub fn guard_map() {}

    //
    // Helpers
    //

    pub fn static_string() {}
    pub fn static_regexp() {}
    pub fn static_regexp_node() {}
    pub fn collapse_string_parts() {}

    pub fn value(&self, token: &Token) -> String {
        let (_, token_value, _) = token;
        token_value.clone()
    }

    pub fn string_value() {}

    pub fn loc(&self, token: &Token) -> Range {
        let (_, _, loc) = token;
        Range::new(loc.begin, loc.end)
    }

    pub fn diagnostic() {}
    pub fn validate_definee() {}

}
