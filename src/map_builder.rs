use std::{convert::TryInto};
use crate::source::Range;
use crate::{Node, Token, Loc};
use crate::source::map::*;

pub fn loc(token: &Token) -> Range {
    let (_, _, loc) = token;
    Range::new(loc.begin, loc.end)
}

pub fn value(token: &Token) -> String {
    let (_, token_value, _) = token;
    String::from_utf8(token_value.to_owned()).unwrap()
}

pub fn join_exprs(left_expr: &Node, right_expr: &Node) -> Range {
    left_expr.expression().join(right_expr.expression())
}

pub fn token_map(token: &Token) -> Map {
    Map { expression: loc(&token) }
}

pub fn delimited_string_map() {}

pub fn prefix_string_map(symbol: &Token) -> CollectionMap {
    let str_range = loc(symbol);
    let begin_l = str_range.with(str_range.begin_pos, str_range.begin_pos + 1);
    CollectionMap {
        begin: Some(begin_l),
        end: None,
        expression: str_range
    }
}

pub fn unquoted_map(token: &Token) -> CollectionMap {
    CollectionMap {
        begin: None,
        end: None,
        expression: loc(&token)
    }
}

pub fn pair_keyword_map(key_t: &Token, value: &Node) -> (CollectionMap, OperatorMap) {
    let key_range = loc(&key_t);
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

pub fn pair_quoted_map(begin_t: &Token, end_t: &Token, node: &Node) -> ( Token, OperatorMap ) {
    let end_l = loc(end_t);

    let quote_loc = Loc { begin: end_l.end_pos - 2, end: end_l.end_pos - 1 };

    let colon_l = end_l.with(end_l.end_pos - 1, end_l.end_pos);

    let end_t: Token = (end_t.0, end_t.1.clone(), quote_loc);

    (
        end_t,
        OperatorMap {
            operator: Some(colon_l),
            expression: loc(begin_t).join(&node.expression())
        }
    )
}

pub fn expr_map(loc: &Range) -> Map {
    Map { expression: loc.clone() }
}

pub fn collection_map(begin_t: &Option<&Token>, parts: &Vec<&Node>, end_t: &Option<&Token>) -> CollectionMap {
    let expr_l: Range;

    let begin_l = if let Some(begin_t) = begin_t { Some(loc(&begin_t)) } else { None };
    let end_l = if let Some(end_t) = end_t { Some(loc(&end_t)) } else { None };

    match (&begin_l, &end_l, !parts.is_empty()) {
        (Some(begin_l), Some(end_l), _) => {
            expr_l = begin_l.join(&end_l);
        },
        (_, _, true) => {
            expr_l = join_exprs(parts.first().unwrap(), parts.last().unwrap());
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

pub fn string_map(begin_t: &Option<&Token>, parts: &Vec<&Node>, end_t: &Option<&Token>) -> CollectionMap {
    if let Some(begin_t) = begin_t {
        if value(&begin_t).starts_with("<<") {
            unimplemented!("heredoc map")
        }
    }

    collection_map(begin_t, parts, end_t)
}

pub fn regexp_map(begin_t: &Token, end_t: &Token, options: &Node) -> CollectionMap {
    CollectionMap {
        begin: Some(loc(begin_t)),
        end: Some(loc(end_t)),
        expression: loc(begin_t).join(options.expression())
    }
}

pub fn constant_map(scope: &Option<&Node>, colon2_t: &Option<&Token>, name_t: &Token) -> ConstantMap {
    let expr_l: Range;
    if let Some(scope) = scope {
        expr_l = scope.expression().join(&loc(name_t));
    } else {
        expr_l = loc(name_t);
    }

    ConstantMap {
        double_colon: colon2_t.map(loc),
        name: loc(&name_t),
        operator: None,
        expression: expr_l,
    }
}

pub fn variable_map(name_t: &Token) -> VariableMap {
    VariableMap { expression: loc(name_t), operator: None }
}

pub fn binary_op_map(left_e: &Node, op_t: &Token, right_e: &Node) -> OperatorMap {
    OperatorMap {
        operator: Some(loc(op_t)),
        expression: join_exprs(&left_e, &right_e)
    }
}

pub fn unary_op_map(op_t: &Token, arg: &Option<&Node>) -> OperatorMap {
    let expr_l: Range;
    if let Some(arg) = arg {
        expr_l = loc(op_t).join(&arg.expression())
    } else {
        expr_l = loc(op_t);
    }

    OperatorMap {
        operator: Some(loc(op_t)),
        expression: expr_l
    }
}

pub fn range_map(start: &Option<&Node>, op_t: &Token, end: &Option<&Node>) -> OperatorMap {
    let expr_l = match (start, end) {
        (Some(start), Some(end)) => join_exprs(&start, &end),
        (Some(start), None) => start.expression().join(&loc(op_t)),
        (None, Some(end)) => loc(op_t).join(&end.expression()),
        (None, None) => unreachable!("at least one of start/end is required")
    };

    OperatorMap {
        operator: Some(loc(op_t)),
        expression: expr_l
    }
}

pub fn arg_prefix_map(op_t: &Token, name_t: &Option<&Token>) -> VariableMap {
    let op_l = loc(op_t);
    let expr_l = if let Some(name_t) = name_t {
        op_l.join(&loc(&name_t))
    } else {
        op_l.clone()
    };

    VariableMap {
        operator: Some(op_l),
        expression: expr_l
    }
}

pub fn kwarg_map(name_t: &Token, value: &Option<&Node>) -> VariableMap {
    let label_range = loc(name_t);
    let name_range = label_range.adjust(0, -1);

    let name_l = loc(name_t);
    let expr_l = if let Some(value) = value {
        name_l.join(&value.expression())
    } else {
        name_l
    };

    VariableMap {
        operator: Some(name_range),
        expression: expr_l
    }
}

pub fn module_definition_map(keyword_t: &Token, name: &Option<&Node>, operator_t: &Option<&Token>, end_t: &Token) -> DefinitionMap {
    let name_l = if let Some(name) = name { Some(name.expression().clone()) } else { None };

    let keyword_l = loc(keyword_t);
    let end_l = loc(end_t);
    let operator_l = if let Some(operator_t) = operator_t { Some(loc(operator_t)) } else { None };

    DefinitionMap {
        expression: keyword_l.join(&end_l),
        keyword: keyword_l,
        operator: operator_l,
        name: name_l,
        end: end_l,
    }
}

pub fn definition_map(keyword_t: &Token, operator_t: &Option<&Token>, name_t: &Token, end_t: &Token) -> MethodDefinitionMap {
    let keyword_l = loc(keyword_t);
    MethodDefinitionMap {
        expression: keyword_l.join(&loc(end_t)),
        keyword: keyword_l,
        operator: operator_t.map(loc),
        name: loc(name_t),
        end: Some(loc(end_t)),
        assignment: None,
    }
}

pub fn endless_definition_map(keyword_t: &Token, operator_t: &Option<&Token>, name_t: &Token, assignment_t: &Token, body: &Option<&Node>) -> MethodDefinitionMap {
    let body_l = body.map(Node::expression).unwrap_or_else(|| unreachable!("endless method always has body") );
    let keyword_l = loc(keyword_t);

    MethodDefinitionMap {
        expression: keyword_l.join(&body_l),
        keyword: keyword_l,
        operator: operator_t.map(loc),
        name: loc(&name_t),
        end: None,
        assignment: Some(loc(&assignment_t)),
    }
}

pub fn send_map(receiver_e: &Option<&Node>, dot_t: &Option<&Token>, selector_t: &Option<&Token>, begin_t: &Option<&Token>, args: &Vec<&Node>, end_t: &Option<&Token>) -> SendMap {
    let begin_l: Option<Range>;
    let end_l: Option<Range>;

    if let Some(receiver_e) = receiver_e {
        begin_l = Some(receiver_e.expression().clone())
    } else if let Some(selector_t) = selector_t {
        begin_l = Some(loc(selector_t))
    } else {
        begin_l = None
    }

    if let Some(end_t) = end_t {
        end_l = Some(loc(end_t));
    } else if let Some(last_arg) = args.last() {
        end_l = Some(last_arg.expression().clone());
    } else if let Some(selector_t) = selector_t {
        end_l = Some(loc(&selector_t))
    } else {
        end_l = None
    }

    let expression_l = match (&begin_l, &end_l) {
        (Some(begin_l), Some(end_l)) => begin_l.join(end_l),
        _ => unreachable!("begin_l = {:#?}, end_l = {:#?}", begin_l, end_l)
    };

    SendMap {
        expression: expression_l,
        dot: dot_t.map(loc),
        selector: selector_t.map(loc),
        operator: None,
        begin: begin_t.map(loc),
        end: end_t.map(loc),
    }
}

pub fn var_send_map(variable_t: &Token) -> SendMap {
    SendMap {
        expression: loc(&variable_t),
        dot: None,
        selector: Some(loc(&variable_t)),
        operator: None,
        begin: None,
        end: None
    }
}

pub fn send_binary_op_map(lhs_e: &Node, selector_t: &Token, rhs_e: &Node) -> SendMap {
    SendMap {
        expression: join_exprs(&lhs_e, &rhs_e),
        dot: None,
        selector: Some(loc(&selector_t)),
        begin: None,
        end: None,
        operator: None
    }
}

pub fn send_unary_op_map(selector_t: &Token, arg: &Option<&Node>) -> SendMap {
    let expr_l: Range;

    if let Some(arg) = arg {
        expr_l = loc(selector_t).join(arg.expression())
    } else {
        expr_l = loc(selector_t)
    }

    SendMap {
        expression: expr_l,
        selector: Some(loc(&selector_t)),
        dot: None,
        begin: None,
        operator: None,
        end: None
    }
}

pub fn index_map(receiver_e: &Node, lbrack_t: &Token, rbrack_t: &Token) -> IndexMap {
    IndexMap {
        begin: loc(lbrack_t),
        end: loc(rbrack_t),
        expression: receiver_e.expression().join(&loc(rbrack_t)),
        operator: None
    }
}
pub fn send_index_map() {}

pub fn block_map(receiver_l: &Range, begin_t: &Token, end_t: &Token) -> CollectionMap {
    CollectionMap {
        begin: Some(loc(&begin_t)),
        end: Some(loc(&end_t)),
        expression: receiver_l.join(&loc(&end_t))
    }
}

pub fn keyword_map(keyword_t: &Token, begin_t: &Option<&Token>, args: &Vec<&Node>, end_t: &Option<&Token>) -> KeywordMap {
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

pub fn keyword_mod_map(pre_e: &Node, keyword_t: &Token, post_e: &Node) -> KeywordMap {
    KeywordMap {
        expression: pre_e.expression().join(&post_e.expression()),
        keyword: loc(keyword_t),
        begin: None,
        end: None
    }
}

pub fn condition_map(keyword_t: &Token, cond: &Option<&Node>, begin_t: &Option<&Token>, body: &Option<&Node>, else_t: &Option<&Token>, else_: &Option<&Node>, end_t: &Option<&Token>) -> ConditionMap {
    let end_l: Range = if let Some(end_t) = end_t {
        loc(&end_t)
    } else if let Some(else_l) = else_.map(|node| node.expression().clone()) {
        else_l
    } else if let Some(else_t_l) = else_t.clone().map(|t| loc(&t)) {
        else_t_l
    } else if let Some(body_l) = body.map(|node| node.expression().clone()) {
        body_l
    } else if let Some(begin_t_l) = begin_t.clone().map(|t| loc(&t)) {
        begin_t_l
    } else if let Some(cond_l) = cond.map(|node| node.expression().clone()) {
        cond_l
    } else {
        unreachable!()
    };

    ConditionMap {
        keyword: Some(loc(keyword_t)),
        begin: begin_t.map(|t| loc(&t)),
        else_: else_t.map(|t| loc(&t)),
        end: end_t.map(|t| loc(&t)),
        expression: loc(&keyword_t).join(&end_l)
    }
}

pub fn ternary_map(begin: &Node, question_t: &Token, _mid: &Node, colon_t: &Token, end: &Node) -> TernaryMap {
    TernaryMap {
        question: loc(&question_t),
        colon: loc(&colon_t),
        expression: join_exprs(&begin, &end)
    }
}

pub fn for_map(keyword_t: &Token, in_t: &Token, begin_t: &Token, end_t: &Token) -> ForMap {
    let keyword_l = loc(&keyword_t);
    let end_l = loc(&end_t);
    ForMap {
        expression: keyword_l.join(&end_l),
        keyword: keyword_l,
        in_: loc(&in_t),
        begin: loc(&begin_t),
        end: end_l,
    }
}

pub fn rescue_body_map(keyword_t: &Token, exc_list: &Option<&Node>, assoc_t: &Option<&Token>, exc_var: &Option<&Node>, then_t: &Option<&Token>, compstmt: &Option<&Node>) -> RescueBodyMap {
    let end_l = match (compstmt, then_t, exc_var, exc_list) {
        (Some(compstmt), _, _, _) => compstmt.expression().clone(),
        (None, Some(then_t), _, _) => loc(then_t),
        (None, None, Some(exc_var), _) => exc_var.expression().clone(),
        (None, None, None, Some(exc_list)) => exc_list.expression().clone(),
        (None, None, None, None) => loc(&keyword_t)
    };

    RescueBodyMap {
        keyword: loc(keyword_t),
        assoc: assoc_t.map(|t| loc(&t)),
        begin: then_t.map(|t| loc(&t)),
        expression: loc(keyword_t).join(&end_l)
    }
}

pub fn eh_keyword_map(compstmt_e: &Option<&Node>, keyword_t: &Option<&Token>, body_es: &Vec<&Node>, else_t: &Option<&Token>, else_e: &Option<&Node>) -> ConditionMap {
    let begin_l: Range;
    let end_l: Range;

    if let Some(compstmt_e) = compstmt_e {
        begin_l = compstmt_e.expression().clone();
    } else if let Some(keyword_t) = keyword_t {
        begin_l = loc(keyword_t);
    } else {
        begin_l = body_es.first().unwrap().expression().clone();
    }

    if let Some(else_t) = else_t {
        if let Some(else_e) = else_e {
            end_l = else_e.expression().clone();
        } else {
            end_l = loc(else_t);
        }
    } else if let Some(last_body_es) = body_es.last() {
        end_l = last_body_es.expression().clone();
    } else if let Some(keyword_t) = keyword_t {
        end_l = loc(keyword_t);
    } else {
        panic!("bug");
    }

    ConditionMap {
        expression: begin_l.join(&end_l),
        keyword: keyword_t.map(loc),
        begin: None,
        else_: else_t.map(loc),
        end: None
    }
}

pub fn guard_map() {}
