use crate::source::Range;
use crate::{Node, Token};
use crate::source::map::*;

//
// Literals
//

// Singletons

pub fn nil(nil_t: Token) -> Node {
    Node::Nil {
        loc: token_map(&nil_t)
    }
}

pub fn true_(true_t: Token) -> Node {
    Node::True {
        loc: token_map(&true_t)
    }
}

pub fn false_(false_t: Token) -> Node {
    Node::False {
        loc: token_map(&false_t)
    }
}

// Numerics

pub fn integer(integer_t: Token) -> Node {
    Node::Int {
        value: value(&integer_t),
        loc: OperatorMap {
            expression: loc(&integer_t),
            operator: None,
        }
    }
}

pub fn float() {}
pub fn rational() {}
pub fn complex() {}
pub fn unary_num() {}

pub fn __line__(line_t: Token) -> Node {
    Node::__LINE__ {
        loc: token_map(&line_t)
    }
}

// Strings

pub fn string() {}
pub fn string_internal() {}
pub fn string_compose() {}
pub fn character() {}

pub fn __file__(file_t: Token) -> Node {
    Node::__FILE__ {
        loc: token_map(&file_t)
    }
}

// Symbols

pub fn symbol() {}
pub fn symbol_internal() {}
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

pub fn self_(token: Token) -> Node {
    Node::Self_ {
        loc: token_map(&token)
    }
}

pub fn ident() {}
pub fn ivar() {}
pub fn gvar() {}
pub fn cvar() {}
pub fn back_ref() {}
pub fn nth_ref() {}
pub fn accessible(_node: Node) -> Node {
    unimplemented!()
}
pub fn const_() {}
pub fn const_global() {}
pub fn const_fetch() {}

pub fn __encoding__(_encoding_t: Token) -> Node {
    Node::__ENCODING__ {
        loc: token_map(&_encoding_t)
    }
}

//
// Assignments
//

pub fn assignable() {}
pub fn const_op_assignable() {}
pub fn assign() {}
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

pub fn def_method() {}
pub fn def_endless_method() {}
pub fn def_singleton() {}
pub fn def_endless_singleton() {}
pub fn undef_method() {}
pub fn alias() {}

//
// Formal arguments
//

pub fn args() {}
pub fn numargs() {}
pub fn forward_only_args() {}
pub fn forward_arg() {}
pub fn arg() {}
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

pub fn binary_op(receiver: Node, operator_t: Token, arg: Node) -> Node {
    let source_map = send_binary_op_map(&receiver, &operator_t, &arg);
    Node::Send { receiver: Some(Box::new(receiver)), operator: value(&operator_t), args: vec![arg], loc: source_map }
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

pub fn preexe(preexe_t: Token, lbrace_t: Token, compstmt: Node, rbrace_t: Token) -> Node {
    Node::Preexe {
        body: Box::new(compstmt),
        loc: keyword_map(&preexe_t, &Some(lbrace_t), &vec![], &Some(rbrace_t))
    }
}
pub fn postexe() {}

// Exception handling

pub fn rescue_body() {}
pub fn begin_body(compound_stmt: Option<Node>, rescue_bodies: Vec<Node>, else_: Option<(Token, Node)>, ensure: Option<(Token, Node)>) -> Option<Node> {
    let mut result: Option<Node>;

    if !rescue_bodies.is_empty() {
        if let Some((else_t, else_)) = else_ {
            let loc = eh_keyword_map(&compound_stmt, None, &rescue_bodies, Some(&else_t), Some(&else_));
            result = Some(
                    Node::Rescue {
                    body: compound_stmt.map(|node| Box::new(node)),
                    rescue_bodies,
                    else_: Some(Box::new(else_)),
                    loc
                }
            )
        } else {
            let loc = eh_keyword_map(&compound_stmt, None, &rescue_bodies, None, None);
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
        let loc = collection_map(&Some(else_t), &else_, &None);
        statements.push(
            Node::Begin {
                statements: else_,
                loc
            }
        );

        let loc = collection_map(&None, &statements, &None);
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
        let loc = eh_keyword_map(&result, Some(&ensure_t), &vec![ensure.clone()], None, None);
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

pub fn compstmt(mut statements: Vec<Node>) -> Option<Node> {
    if statements.is_empty() {
        None
    } else if statements.len() == 1 {
        statements.pop()
    } else {
        let source_map = collection_map(&None, &statements, &None);
        Some(Node::Begin { statements, loc: source_map })
    }
}

pub fn begin() {}
pub fn begin_keyword(begin_t: Token, body: Option<Node>, end_t: Token) -> Node {
    match body {
        None => {
            // A nil expression: `begin end'.
            Node::KwBegin  {
                statements: vec![],
                loc: collection_map(&Some(begin_t), &vec![], &Some(end_t))
            }
        },
        Some(Node::Begin { statements, loc: CollectionMap { begin: None, end: None, .. }, .. }) => {
            // Synthesized (begin) from compstmt "a; b".
            let loc = collection_map(&Some(begin_t), &statements, &Some(end_t));
            Node::KwBegin {
                statements,
                loc
            }
        },
        Some(node) => {
            let statements = vec![node];
            let loc = collection_map(&Some(begin_t), &statements, &Some(end_t));
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
pub fn check_assignment_to_numparam() {}
pub fn check_reserved_for_numparam() {}
pub fn arg_name_collides() {}
pub fn check_lvar_name() {}
pub fn check_duplicate_pattern_variable() {}
pub fn check_duplicate_pattern_key() {}

//
// Source maps
//

pub fn join_expr(left_expr: &Node, right_expr: &Node) -> Range {
    left_expr.expression().join(right_expr.expression())
}

pub fn token_map(token: &Token) -> Map {
    Map { expression: loc(&token) }
}

pub fn delimited_string_map() {}
pub fn prefix_string_map() {}
pub fn unquoted_map() {}
pub fn pair_keyword_map() {}
pub fn pair_quoted_map() {}
pub fn expr_map() {}

pub fn collection_map(begin_t: &Option<Token>, parts: &Vec<Node>, end_t: &Option<Token>) -> CollectionMap {
    let expr_l: Range;

    let begin_l = if let Some(begin_t) = begin_t { Some(loc(&begin_t)) } else { None };
    let end_l = if let Some(end_t) = end_t { Some(loc(&end_t)) } else { None };

    match (begin_l.clone(), end_l.clone(), !parts.is_empty()) {
        (Some(begin_l), Some(end_l), _) => {
            expr_l = begin_l.join(&end_l);
        },
        (_, _, true) => {
            expr_l = join_expr(parts.first().unwrap(), parts.last().unwrap());
        },
        (Some(begin_l), _, false) => {
            expr_l = begin_l.clone();
        },
        (_, Some(end_l), false) => {
            expr_l = end_l.clone();
        },
        (None, None, false) => {
            panic!("empty collection without begin_t/end_t, can't build source map")
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
pub fn constant_map() {}
pub fn variable_map() {}
pub fn binary_op_map() {}
pub fn unary_op_map() {}
pub fn range_map() {}
pub fn arg_prefix_map() {}
pub fn kwarg_map() {}
pub fn module_definition_map() {}
pub fn definition_map() {}
pub fn endless_definition_map() {}
pub fn send_map() {}
pub fn var_send_map() {}

pub fn send_binary_op_map(lhs_e: &Node, selector_t: &Token, rhs_e: &Node) -> SendMap {
    SendMap {
        expression: join_expr(&lhs_e, &rhs_e),
        dot: None,
        selector: Some(loc(&selector_t)),
        begin: None,
        end: None,
        operator: None
    }
}

pub fn send_unary_op_map() {}
pub fn index_map() {}
pub fn send_index_map() {}
pub fn block_map() {}

pub fn keyword_map(keyword_t: &Token, begin_t: &Option<Token>, args: &Vec<Node>, end_t: &Option<Token>) -> KeywordMap {
    let expr_end_l: Range;
    let begin_l = if let Some(begin_t) = begin_t { Some(loc(&begin_t)) } else { None };
    let end_l = if let Some(end_t) = end_t { Some(loc(&end_t)) } else { None };

    if let Some(end_l) = &end_l {
        expr_end_l = end_l.clone();
    } else if let Some(last_arg) = args.iter().rev().nth(0) {
        expr_end_l = last_arg.expression().clone();
    } else if let Some(second_last_arg) = args.iter().rev().nth(1) {
        expr_end_l = second_last_arg.expression().clone();
    } else {
        expr_end_l = loc(&keyword_t);
    }

    KeywordMap {
        expression: loc(&keyword_t).join(&expr_end_l),
        keyword: loc(&keyword_t),
        begin: begin_l,
        end: end_l
    }
}

pub fn keyword_mod_map() {}
pub fn condition_map() {}
pub fn ternary_map() {}
pub fn for_map() {}
pub fn rescue_body_map() {}

pub fn eh_keyword_map(compstmt_e: &Option<Node>, keyword_t: Option<&Token>, body_es: &Vec<Node>, else_t: Option<&Token>, else_e: Option<&Node>) -> ConditionMap {
    let begin_l: Range;
    let end_l: Range;

    if let Some(compstmt_e) = &compstmt_e {
        begin_l = compstmt_e.expression().clone();
    } else if let Some(keyword_t) = &keyword_t {
        begin_l = loc(&keyword_t);
    } else {
        begin_l = body_es.first().unwrap().expression().clone();
    }

    if let Some(else_t) = &else_t {
        if let Some(else_e) = &else_e {
            end_l = else_e.expression().clone();
        } else {
            end_l = loc(&else_t);
        }
    } else if let Some(last_body_es) = body_es.last() {
        end_l = last_body_es.expression().clone();
    } else if let Some(keyword_t) = &keyword_t {
        end_l = loc(&keyword_t);
    } else {
        panic!("bug");
    }

    ConditionMap {
        expression: begin_l.join(&end_l),
        keyword: keyword_t.map(|t| loc(&t)),
        begin: None,
        else_: else_t.map(|t| loc(&t)),
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

pub fn value(token: &Token) -> String {
    let (_, token_value, _) = token;
    token_value.clone()
}

pub fn string_value() {}

pub fn loc(token: &Token) -> Range {
    let (_, _, loc) = token;
    Range::new(loc.begin, loc.end)
}

pub fn diagnostic() {}
pub fn validate_definee() {}
