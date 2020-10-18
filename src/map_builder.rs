// use crate::source::map::*;
use crate::source::Range;
use crate::Node;
use crate::{Loc, Token};
use std::convert::TryInto;

pub fn loc(token: &Token) -> Range {
    let (_, _, loc) = token;
    Range::new(loc.begin, loc.end)
}

pub fn maybe_loc(token: &Option<Token>) -> Option<Range> {
    match token {
        Some(token) => Some(loc(token)),
        None => None,
    }
}

pub fn maybe_node_expr(node: &Option<Node>) -> Option<Range> {
    match node {
        Some(node) => Some(node.expression().clone()),
        None => None,
    }
}

pub fn collection_expr(nodes: &Vec<Node>) -> Option<Range> {
    join_maybe_exprs(&nodes.first().cloned(), &nodes.last().cloned())
}

pub fn merge_maybe_locs(locs: Vec<Option<Range>>) -> Option<Range> {
    let mut result: Option<Range> = None;
    for loc in locs {
        result = join_maybe_locs(&result, &loc)
    }
    result
}

pub fn value(token: &Token) -> String {
    let (_, token_value, _) = token;
    token_value.to_string_lossy()
}

pub fn maybe_value(token: &Option<Token>) -> Option<String> {
    match token {
        Some(token) => Some(value(token)),
        None => None,
    }
}

pub fn join_exprs(lhs: &Node, rhs: &Node) -> Range {
    lhs.expression().join(rhs.expression())
}

pub fn join_maybe_exprs(lhs: &Option<Node>, rhs: &Option<Node>) -> Option<Range> {
    join_maybe_locs(&maybe_node_expr(lhs), &maybe_node_expr(rhs))
}

pub fn join_maybe_locs(lhs: &Option<Range>, rhs: &Option<Range>) -> Option<Range> {
    match (lhs, rhs) {
        (None, None) => None,
        (None, Some(rhs)) => Some(rhs.clone()),
        (Some(lhs), None) => Some(lhs.clone()),
        (Some(lhs), Some(rhs)) => Some(lhs.join(&rhs)),
    }
}

pub fn token_map(token: &Token) -> Range {
    loc(token)
}

pub fn delimited_string_map() {}

pub fn prefix_string_map(symbol: &Token) -> (Range, Range) {
    let str_range = loc(symbol);
    let begin_l = str_range.with(str_range.begin_pos, str_range.begin_pos + 1);
    (begin_l, str_range)
}

pub fn unquoted_map(token: &Token) -> Range {
    loc(token)
}

// (begin_l, end_l, expr_l), (operator_l, expr_l)
pub fn pair_keyword_map(key_t: &Token, value: &Node) -> (Range, Range, Range) {
    let key_range = loc(key_t);
    let key_l = key_range.adjust(0, -1);
    let colon_l = key_range.adjust((key_range.end_pos - 1).try_into().unwrap(), 0);
    let expr_l = key_range.join(&value.expression());

    (key_l, colon_l, expr_l)
}

pub fn pair_quoted_map(begin_t: &Token, end_t: &Token, node: &Node) -> (Token, Range, Range) {
    let end_l = loc(end_t);

    let quote_loc = Loc {
        begin: end_l.end_pos - 2,
        end: end_l.end_pos - 1,
    };

    let colon_l = end_l.with(end_l.end_pos - 1, end_l.end_pos);

    let end_t: Token = (end_t.0, end_t.1.clone(), quote_loc);
    let expr_l = loc(begin_t).join(&node.expression());
    (end_t, colon_l, expr_l)
}

pub fn expr_map(loc: &Range) -> Range {
    loc.clone()
}

pub fn collection_map(
    begin_t: &Option<Token>,
    parts: &Vec<Node>,
    end_t: &Option<Token>,
) -> (Option<Range>, Option<Range>, Range) {
    let begin_l = maybe_loc(begin_t);
    let end_l = maybe_loc(end_t);

    let expr_l = merge_maybe_locs(vec![
        begin_l.clone(),
        collection_expr(&parts),
        end_l.clone(),
    ])
    .unwrap_or_else(|| panic!("empty collection without begin_t/end_t, can't build source map"));

    (begin_l, end_l, expr_l)
}

pub enum StringMap {
    CollectionMap((Option<Range>, Option<Range>, Range)),
    HeredocMap((Range, Range, Range)),
}

pub fn string_map(begin_t: &Option<Token>, parts: &Vec<Node>, end_t: &Option<Token>) -> StringMap {
    if let Some(begin_t) = begin_t {
        if value(&begin_t).starts_with("<<") {
            let end_t = end_t
                .as_ref()
                .unwrap_or_else(|| panic!("heredoc must have end_t"));
            let expr_l = collection_expr(&parts).unwrap_or_else(|| loc(end_t));

            return StringMap::HeredocMap((loc(begin_t), loc(end_t), expr_l));
        }
    }

    StringMap::CollectionMap(collection_map(begin_t, parts, end_t))
}

pub fn regexp_map(begin_t: &Token, end_t: &Token, options: &Node) -> (Range, Range, Range) {
    let begin_l = loc(begin_t);
    let end_l = loc(end_t);
    let expr_l = begin_l.join(options.expression());
    (begin_l, end_l, expr_l)
}

// double_colon, name, operator, expression
pub fn constant_map(
    scope: &Option<Node>,
    colon2_t: &Option<Token>,
    name_t: &Token,
) -> (Option<Range>, Range, Range) {
    let name_l = loc(name_t);
    let expr_l = maybe_node_expr(scope)
        .map(|l| l.join(&name_l))
        .unwrap_or_else(|| name_l.clone());
    let colon2_l = maybe_loc(colon2_t);

    (colon2_l, name_l, expr_l)
}

pub fn variable_map(name_t: &Token) -> Range {
    loc(&name_t)
}

pub fn binary_op_map(left_e: &Node, op_t: &Token, right_e: &Node) -> (Range, Range) {
    let op_l = loc(op_t);
    let expr_l = join_exprs(&left_e, &right_e);
    (op_l, expr_l)
}

pub fn unary_op_map(op_t: &Token, arg: &Option<Node>) -> (Range, Range) {
    let op_l = loc(op_t);
    let expr_l = maybe_node_expr(arg)
        .map(|l| l.join(&op_l))
        .unwrap_or_else(|| op_l.clone());

    (op_l, expr_l)
}

pub fn range_map(start: &Option<Node>, op_t: &Token, end: &Option<Node>) -> (Range, Range) {
    let expr_l = merge_maybe_locs(vec![
        maybe_node_expr(start),
        Some(loc(op_t)),
        maybe_node_expr(end),
    ])
    .unwrap_or_else(|| unreachable!("at least one of start/end is required"));

    (loc(op_t), expr_l)
}

pub fn arg_prefix_map(op_t: &Token, name_t: &Option<Token>) -> (Range, Option<Range>, Range) {
    let op_l = loc(op_t);
    let name_l = maybe_loc(name_t);
    let expr_l = op_l.maybe_join(&name_l);

    (op_l, name_l, expr_l)
}

pub fn kwarg_map(name_t: &Token, default: &Option<Node>) -> (Range, Range) {
    let label_l = loc(name_t);
    let name_l = label_l.adjust(0, -1);

    let expr_l = maybe_node_expr(default)
        .map(|l| l.join(&label_l))
        .unwrap_or_else(|| label_l);

    (name_l, expr_l)
}

pub fn definition_map(
    keyword_t: &Token,
    operator_t: &Option<Token>,
    name_t: &Token,
    end_t: &Token,
) -> (Range, Option<Range>, Range, Range, Range) {
    let keyword_l = loc(keyword_t);
    let operator_l = maybe_loc(operator_t);
    let name_l = loc(name_t);
    let end_l = loc(end_t);
    let expr_l = keyword_l.join(&loc(end_t));

    (keyword_l, operator_l, name_l, end_l, expr_l)
}

pub fn endless_definition_map(
    keyword_t: &Token,
    operator_t: &Option<Token>,
    name_t: &Token,
    assignment_t: &Token,
    body: &Option<Node>,
) -> (Range, Option<Range>, Range, Range, Range) {
    let body_l =
        maybe_node_expr(body).unwrap_or_else(|| unreachable!("endless method always has body"));

    let keyword_l = loc(keyword_t);
    let expr_l = keyword_l.join(&body_l);
    let operator_l = maybe_loc(operator_t);
    let name_l = loc(name_t);
    let assignment_l = loc(assignment_t);
    (keyword_l, operator_l, name_l, assignment_l, expr_l)
}

pub fn send_map(
    receiver_e: &Option<Node>,
    dot_t: &Option<Token>,
    selector_t: &Option<Token>,
    begin_t: &Option<Token>,
    args: &Vec<Node>,
    end_t: &Option<Token>,
) -> (
    Option<Range>,
    Option<Range>,
    Option<Range>,
    Option<Range>,
    Range,
) {
    let begin_l = maybe_node_expr(receiver_e)
        .or_else(|| maybe_loc(selector_t))
        .unwrap_or_else(|| unreachable!("can't compute begin_l"));
    let end_l = maybe_loc(end_t)
        .or_else(|| maybe_node_expr(&args.last().cloned()))
        .or_else(|| maybe_loc(selector_t))
        .unwrap_or_else(|| unreachable!("can't compute end_l"));

    let expr_l = begin_l.join(&end_l);

    let dot_l = maybe_loc(dot_t);
    let selector_l = maybe_loc(selector_t);
    let begin_l = maybe_loc(begin_t);
    let end_l = maybe_loc(end_t);
    (begin_l, dot_l, selector_l, end_l, expr_l)
}

pub fn var_send_map(variable_t: &Token) -> Range {
    loc(variable_t)
}

pub fn send_binary_op_map(lhs_e: &Node, selector_t: &Token, rhs_e: &Node) -> (Range, Range) {
    let expr_l = join_exprs(&lhs_e, &rhs_e);
    let selector_l = loc(selector_t);
    (selector_l, expr_l)
}

pub fn send_unary_op_map(selector_t: &Token, arg: &Option<Node>) -> (Range, Range) {
    let selector_l = loc(selector_t);
    let expr_l = maybe_node_expr(arg)
        .map(|l| l.join(&selector_l))
        .unwrap_or_else(|| selector_l.clone());

    (selector_l, expr_l)
}

pub fn index_map(receiver_e: &Node, lbrack_t: &Token, rbrack_t: &Token) -> (Range, Range, Range) {
    let begin_l = loc(lbrack_t);
    let end_l = loc(rbrack_t);
    let expr_l = receiver_e.expression().join(&end_l);
    (begin_l, end_l, expr_l)
}
pub fn send_index_map() {}

pub fn block_map(receiver_l: &Range, begin_t: &Token, end_t: &Token) -> (Range, Range, Range) {
    let begin_l = loc(begin_t);
    let end_l = loc(end_t);
    let expr_l = receiver_l.join(&end_l);
    (begin_l, end_l, expr_l)
}

pub fn keyword_map(
    keyword_t: &Token,
    begin_t: &Option<Token>,
    args: &Vec<Node>,
    end_t: &Option<Token>,
) -> (Range, Option<Range>, Option<Range>, Range) {
    let keyword_l = loc(keyword_t);
    let begin_l = maybe_loc(begin_t);
    let end_l = maybe_loc(end_t);

    let expr_end_l = end_l
        .clone()
        .or_else(|| maybe_node_expr(&args.last().cloned()))
        .unwrap_or_else(|| keyword_l.clone());

    let expr_l = keyword_l.join(&expr_end_l);
    (keyword_l, begin_l, end_l, expr_l)
}

pub fn keyword_mod_map(pre_e: &Node, keyword_t: &Token, post_e: &Node) -> (Range, Range) {
    let expr_l = pre_e.expression().join(&post_e.expression());
    let keyword_l = loc(keyword_t);
    (keyword_l, expr_l)
}

pub fn condition_map(
    keyword_t: &Token,
    cond: &Option<Node>,
    begin_t: &Option<Token>,
    body: &Option<Node>,
    else_t: &Option<Token>,
    else_: &Option<Node>,
    end_t: &Option<Token>,
) -> (Range, Option<Range>, Option<Range>, Option<Range>, Range) {
    let end_l = maybe_loc(end_t)
        .or_else(|| maybe_node_expr(else_))
        .or_else(|| maybe_loc(else_t))
        .or_else(|| maybe_node_expr(body))
        .or_else(|| maybe_loc(begin_t))
        .or_else(|| maybe_node_expr(cond))
        .unwrap_or_else(|| unreachable!("can't compute end_l"));

    let expr_l = loc(keyword_t).join(&end_l);
    let keyword_l = loc(keyword_t);
    let begin_l = maybe_loc(begin_t);
    let else_l = maybe_loc(else_t);
    let end_l = maybe_loc(end_t);

    (keyword_l, begin_l, else_l, end_l, expr_l)
}

pub fn ternary_map(
    begin: &Node,
    question_t: &Token,
    colon_t: &Token,
    end: &Node,
) -> (Range, Range, Range) {
    let expr_l = join_exprs(&begin, &end);
    let question_l = loc(question_t);
    let colon_l = loc(colon_t);
    (question_l, colon_l, expr_l)
}

pub fn for_map(
    keyword_t: &Token,
    in_t: &Token,
    begin_t: &Token,
    end_t: &Token,
) -> (Range, Range, Range, Range, Range) {
    let keyword_l = loc(keyword_t);
    let end_l = loc(end_t);
    let expr_l = keyword_l.join(&end_l);
    let in_l = loc(in_t);
    let begin_l = loc(begin_t);

    (keyword_l, in_l, begin_l, end_l, expr_l)
}

pub fn rescue_body_map(
    keyword_t: &Token,
    exc_list: &Option<Node>,
    assoc_t: &Option<Token>,
    exc_var: &Option<Node>,
    then_t: &Option<Token>,
    compstmt: &Option<Node>,
) -> (Range, Option<Range>, Option<Range>, Range) {
    let end_l = maybe_node_expr(compstmt)
        .or_else(|| maybe_loc(then_t))
        .or_else(|| maybe_node_expr(exc_var))
        .or_else(|| maybe_node_expr(exc_list))
        .unwrap_or_else(|| loc(keyword_t));

    let expr_l = loc(keyword_t).join(&end_l);
    let keyword_l = loc(keyword_t);
    let assoc_l = maybe_loc(assoc_t);
    let then_l = maybe_loc(then_t);

    (keyword_l, then_l, assoc_l, expr_l)
}

pub fn eh_keyword_map(
    compstmt_e: &Option<Node>,
    keyword_t: &Option<Token>,
    body_es: &Vec<Node>,
    else_t: &Option<Token>,
    else_e: &Option<Node>,
) -> (Option<Range>, Option<Range>, Range) {
    let begin_l = maybe_node_expr(compstmt_e)
        .or_else(|| maybe_loc(keyword_t))
        .or_else(|| maybe_node_expr(&body_es.first().cloned()))
        .unwrap_or_else(|| unreachable!("can't compute begin_l"));

    let end_l = maybe_node_expr(else_e)
        .or_else(|| maybe_loc(else_t))
        .or_else(|| maybe_node_expr(&body_es.last().cloned()))
        .or_else(|| maybe_loc(keyword_t))
        .unwrap_or_else(|| unreachable!("can't compute end_l"));

    let expr_l = begin_l.join(&end_l);
    let keyword_l = maybe_loc(keyword_t);
    let else_l = maybe_loc(else_t);

    (keyword_l, else_l, expr_l)
}

pub fn guard_map(keyword_t: &Token, guard_body: &Node) -> (Range, Range) {
    let keyword_l = loc(keyword_t);

    let expr_l = keyword_l.join(guard_body.expression());
    (keyword_l, expr_l)
}
