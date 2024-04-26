use lib_ruby_parser_ast::{Blob, NodeList};

use crate::builder::{ArgsType, PKwLabel};
use crate::context::Context;
use crate::str_term::StrTerm;
use crate::Node;
use crate::Token;

pub(crate) trait FromParseValue<'b> {
    type Output;

    fn from_value(value: ParseValue<'b>) -> Self::Output;
}

impl<'b> FromParseValue<'b> for Node<'b> {
    type Output = &'b Node<'b>;

    fn from_value(value: ParseValue<'b>) -> Self::Output {
        match value {
            ParseValue::Node(value) => value,
            other => unreachable!("expected Node, got {:?}", other),
        }
    }
}

impl<'b> FromParseValue<'b> for Token<'b> {
    type Output = &'b Self;

    fn from_value(value: ParseValue<'b>) -> Self::Output {
        match value {
            ParseValue::Token(value) => value,
            other => unreachable!("expected Token, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct TokenWithContext<'b> {
    pub(crate) token: &'b Token<'b>,
    pub(crate) ctx: Context,
}

impl<'b> FromParseValue<'b> for TokenWithContext<'b> {
    type Output = TokenWithContext<'b>;

    fn from_value(value: ParseValue<'b>) -> Self::Output {
        match value {
            ParseValue::TokenWithContext(value) => value,
            other => unreachable!("expected TokenWithContext, got {:?}", other),
        }
    }
}

impl<'b> FromParseValue<'b> for Context {
    type Output = Context;

    fn from_value(value: ParseValue<'b>) -> Self::Output {
        match value {
            ParseValue::Context(ctx) => ctx,
            other => unreachable!("expected Context, got {:?}", other),
        }
    }
}

impl<'b> FromParseValue<'b> for NodeList<'b> {
    type Output = &'b NodeList<'b>;

    fn from_value(value: ParseValue<'b>) -> Self::Output {
        match value {
            ParseValue::NodeList(value) => value,
            other => unreachable!("expected NodeList, got {:?}", other),
        }
    }
}

pub(crate) struct Bool;

impl<'b> FromParseValue<'b> for Bool {
    type Output = bool;

    fn from_value(value: ParseValue<'b>) -> Self::Output {
        match value {
            ParseValue::Bool(value) => value,
            other => unreachable!("expected Bool, got {:?}", other),
        }
    }
}

pub(crate) struct MaybeStrTerm;
impl<'b> FromParseValue<'b> for MaybeStrTerm {
    type Output = Option<StrTerm<'b>>;

    fn from_value(value: ParseValue<'b>) -> Self::Output {
        match value {
            ParseValue::MaybeStrTerm(value) => value,
            other => unreachable!("expected MaybeStrTerm, got {:?}", other),
        }
    }
}

pub(crate) struct Num;
impl<'b> FromParseValue<'b> for Num {
    type Output = i32;

    fn from_value(value: ParseValue<'b>) -> Self::Output {
        match value {
            ParseValue::Num(value) => value,
            other => unreachable!("expected Num, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Superclass<'b> {
    pub(crate) lt_t: Option<&'b Token<'b>>,
    pub(crate) value: Option<&'b Node<'b>>,
}
impl<'b> FromParseValue<'b> for Superclass<'b> {
    type Output = Superclass<'b>;

    fn from_value(value: ParseValue<'b>) -> Self::Output {
        match value {
            ParseValue::Superclass(value) => value,
            other => unreachable!("expected Superclass, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Ensure<'b> {
    pub(crate) ensure_t: &'b Token<'b>,
    pub(crate) body: Option<&'b Node<'b>>,
}
pub(crate) struct OptEnsure;
impl<'b> FromParseValue<'b> for OptEnsure {
    type Output = Option<Ensure<'b>>;

    fn from_value(value: ParseValue<'b>) -> Self::Output {
        match value {
            ParseValue::OptEnsure(value) => value,
            other => unreachable!("expected OptEnsure, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Else<'b> {
    pub(crate) else_t: &'b Token<'b>,
    pub(crate) body: Option<&'b Node<'b>>,
}
pub(crate) struct OptElse;
impl<'b> FromParseValue<'b> for OptElse {
    type Output = Option<Else<'b>>;

    fn from_value(value: ParseValue<'b>) -> Self::Output {
        match value {
            ParseValue::OptElse(value) => value,
            other => unreachable!("expected OptElse, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct ExcVar<'b> {
    pub(crate) assoc_t: Option<&'b Token<'b>>,
    pub(crate) exc_var: Option<&'b Node<'b>>,
}
impl<'b> FromParseValue<'b> for ExcVar<'b> {
    type Output = ExcVar<'b>;

    fn from_value(value: ParseValue<'b>) -> Self::Output {
        match value {
            ParseValue::ExcVar(value) => value,
            other => unreachable!("expected ExcVar, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct IfTail<'b> {
    pub(crate) keyword_t: Option<&'b Token<'b>>,
    pub(crate) body: Option<&'b Node<'b>>,
}
impl<'b> FromParseValue<'b> for IfTail<'b> {
    type Output = IfTail<'b>;

    fn from_value(value: ParseValue<'b>) -> Self::Output {
        match value {
            ParseValue::IfTail(value) => value,
            other => unreachable!("expected IfTail, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct ExprValueDo<'b> {
    pub(crate) value: &'b Node<'b>,
    pub(crate) do_t: &'b Token<'b>,
}
impl<'b> FromParseValue<'b> for ExprValueDo<'b> {
    type Output = ExprValueDo<'b>;

    fn from_value(value: ParseValue<'b>) -> Self::Output {
        match value {
            ParseValue::ExprValueDo(value) => value,
            other => unreachable!("expected ExprValueDo, got {:?}", other),
        }
    }
}

impl<'b> FromParseValue<'b> for PKwLabel<'b> {
    type Output = PKwLabel<'b>;

    fn from_value(value: ParseValue<'b>) -> Self::Output {
        match value {
            ParseValue::PKwLabel(value) => value,
            other => unreachable!("expected PKwLabel, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct BraceBody<'b> {
    pub(crate) args_type: ArgsType<'b>,
    pub(crate) body: Option<&'b Node<'b>>,
}
impl<'b> FromParseValue<'b> for BraceBody<'b> {
    type Output = BraceBody<'b>;

    fn from_value(value: ParseValue<'b>) -> Self::Output {
        match value {
            ParseValue::BraceBody(value) => value,
            other => unreachable!("expected BraceBody, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct CmdBraceBlock<'b> {
    pub(crate) begin_t: &'b Token<'b>,
    pub(crate) args_type: ArgsType<'b>,
    pub(crate) body: Option<&'b Node<'b>>,
    pub(crate) end_t: &'b Token<'b>,
}
impl<'b> FromParseValue<'b> for CmdBraceBlock<'b> {
    type Output = CmdBraceBlock<'b>;

    fn from_value(value: ParseValue<'b>) -> Self::Output {
        match value {
            ParseValue::CmdBraceBlock(value) => value,
            other => unreachable!("expected CmdBraceBlock, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct ParenArgs<'b> {
    pub(crate) begin_t: &'b Token<'b>,
    pub(crate) args: &'b NodeList<'b>,
    pub(crate) end_t: &'b Token<'b>,
}
impl<'b> FromParseValue<'b> for ParenArgs<'b> {
    type Output = ParenArgs<'b>;

    fn from_value(value: ParseValue<'b>) -> Self::Output {
        match value {
            ParseValue::ParenArgs(value) => value,
            other => unreachable!("expected ParenArgs, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct OptParenArgs<'b> {
    pub(crate) begin_t: Option<&'b Token<'b>>,
    pub(crate) args: &'b NodeList<'b>,
    pub(crate) end_t: Option<&'b Token<'b>>,
}
impl<'b> FromParseValue<'b> for OptParenArgs<'b> {
    type Output = OptParenArgs<'b>;

    fn from_value(value: ParseValue<'b>) -> Self::Output {
        match value {
            ParseValue::OptParenArgs(value) => value,
            other => unreachable!("expected OptParenArgs, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct BeginBlock<'b> {
    pub(crate) begin_t: &'b Token<'b>,
    pub(crate) body: Option<&'b Node<'b>>,
    pub(crate) end_t: &'b Token<'b>,
}
impl<'b> FromParseValue<'b> for BeginBlock<'b> {
    type Output = BeginBlock<'b>;

    fn from_value(value: ParseValue<'b>) -> Self::Output {
        match value {
            ParseValue::BeginBlock(value) => value,
            other => unreachable!("expected BeginBlock, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct LambdaBody<'b> {
    pub(crate) begin_t: &'b Token<'b>,
    pub(crate) body: Option<&'b Node<'b>>,
    pub(crate) end_t: &'b Token<'b>,
}
impl<'b> FromParseValue<'b> for LambdaBody<'b> {
    type Output = LambdaBody<'b>;

    fn from_value(value: ParseValue<'b>) -> Self::Output {
        match value {
            ParseValue::LambdaBody(value) => value,
            other => unreachable!("expected LambdaBody, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct DoBlock<'b> {
    pub(crate) begin_t: &'b Token<'b>,
    pub(crate) args_type: ArgsType<'b>,
    pub(crate) body: Option<&'b Node<'b>>,
    pub(crate) end_t: &'b Token<'b>,
}
impl<'b> FromParseValue<'b> for DoBlock<'b> {
    type Output = DoBlock<'b>;

    fn from_value(value: ParseValue<'b>) -> Self::Output {
        match value {
            ParseValue::DoBlock(value) => value,
            other => unreachable!("expected DoBlock, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct BraceBlock<'b> {
    pub(crate) begin_t: &'b Token<'b>,
    pub(crate) args_type: ArgsType<'b>,
    pub(crate) body: Option<&'b Node<'b>>,
    pub(crate) end_t: &'b Token<'b>,
}
impl<'b> FromParseValue<'b> for BraceBlock<'b> {
    type Output = BraceBlock<'b>;

    fn from_value(value: ParseValue<'b>) -> Self::Output {
        match value {
            ParseValue::BraceBlock(value) => value,
            other => unreachable!("expected BraceBlock, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct DefsHead<'b> {
    pub(crate) def_t: &'b Token<'b>,
    pub(crate) definee: &'b Node<'b>,
    pub(crate) dot_t: &'b Token<'b>,
    pub(crate) name_t: TokenWithContext<'b>,
}
impl<'b> FromParseValue<'b> for DefsHead<'b> {
    type Output = DefsHead<'b>;

    fn from_value(value: ParseValue<'b>) -> Self::Output {
        match value {
            ParseValue::DefsHead(value) => value,
            other => unreachable!("expected DefsHead, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct DefnHead<'b> {
    pub(crate) def_t: &'b Token<'b>,
    pub(crate) name_t: TokenWithContext<'b>,
}
impl<'b> FromParseValue<'b> for DefnHead<'b> {
    type Output = DefnHead<'b>;

    fn from_value(value: ParseValue<'b>) -> Self::Output {
        match value {
            ParseValue::DefnHead(value) => value,
            other => unreachable!("expected DefnHead, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Cases<'b> {
    pub(crate) when_bodies: &'b NodeList<'b>,
    pub(crate) opt_else: Option<Else<'b>>,
}
impl<'b> FromParseValue<'b> for Cases<'b> {
    type Output = Cases<'b>;

    fn from_value(value: ParseValue<'b>) -> Self::Output {
        match value {
            ParseValue::Cases(value) => value,
            other => unreachable!("expected Cases, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct CaseBody<'b> {
    pub(crate) when_bodies: &'b NodeList<'b>,
    pub(crate) opt_else: Option<Else<'b>>,
}
impl<'b> FromParseValue<'b> for CaseBody<'b> {
    type Output = CaseBody<'b>;

    fn from_value(value: ParseValue<'b>) -> Self::Output {
        match value {
            ParseValue::CaseBody(value) => value,
            other => unreachable!("expected CaseBody, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct PCases<'b> {
    pub(crate) in_bodies: &'b NodeList<'b>,
    pub(crate) opt_else: Option<Else<'b>>,
}
impl<'b> FromParseValue<'b> for PCases<'b> {
    type Output = PCases<'b>;

    fn from_value(value: ParseValue<'b>) -> Self::Output {
        match value {
            ParseValue::PCases(value) => value,
            other => unreachable!("expected PCases, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct PCaseBody<'b> {
    pub(crate) in_bodies: &'b NodeList<'b>,
    pub(crate) opt_else: Option<Else<'b>>,
}
impl<'b> FromParseValue<'b> for PCaseBody<'b> {
    type Output = PCaseBody<'b>;

    fn from_value(value: ParseValue<'b>) -> Self::Output {
        match value {
            ParseValue::PCaseBody(value) => value,
            other => unreachable!("expected PCaseBody, got {:?}", other),
        }
    }
}

pub(crate) struct MaybeNode;
impl<'b> FromParseValue<'b> for MaybeNode {
    type Output = Option<&'b Node<'b>>;

    fn from_value(value: ParseValue<'b>) -> Self::Output {
        match value {
            ParseValue::MaybeNode(maybe_node) => maybe_node,
            other => unreachable!("expected MaybeNode, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct DoBody<'b> {
    pub(crate) args_type: ArgsType<'b>,
    pub(crate) body: Option<&'b Node<'b>>,
}
impl<'b> FromParseValue<'b> for DoBody<'b> {
    type Output = DoBody<'b>;

    fn from_value(value: ParseValue<'b>) -> Self::Output {
        match value {
            ParseValue::DoBody(value) => value,
            other => unreachable!("expected DoBody, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct PTopExpr<'b> {
    pub(crate) pattern: &'b Node<'b>,
    pub(crate) guard: Option<&'b Node<'b>>,
}
impl<'b> FromParseValue<'b> for PTopExpr<'b> {
    type Output = PTopExpr<'b>;

    fn from_value(value: ParseValue<'b>) -> Self::Output {
        match value {
            ParseValue::PTopExpr(value) => value,
            other => unreachable!("expected PTopExpr, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct MatchPatternWithTrailingComma<'b> {
    pub(crate) elements: &'b NodeList<'b>,
    pub(crate) trailing_comma: Option<&'b Token<'b>>,
}
impl<'b> FromParseValue<'b> for MatchPatternWithTrailingComma<'b> {
    type Output = MatchPatternWithTrailingComma<'b>;

    fn from_value(value: ParseValue<'b>) -> Self::Output {
        match value {
            ParseValue::MatchPatternWithTrailingComma(value) => value,
            other => unreachable!("expected MatchPatternWithTrailingComma, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct NoKwRest<'b> {
    pub(crate) kwrest_mark: &'b Token<'b>,
    pub(crate) k_nil: &'b Token<'b>,
}
impl<'b> FromParseValue<'b> for NoKwRest<'b> {
    type Output = NoKwRest<'b>;

    fn from_value(value: ParseValue<'b>) -> Self::Output {
        match value {
            ParseValue::NoKwRest(value) => value,
            other => unreachable!("expected NoKwRest, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) enum ParseValue<'b> {
    Stolen,
    Uninitialized,
    None,
    Token(&'b Token<'b>),
    TokenWithContext(TokenWithContext<'b>),
    Context(Context),
    Node(&'b Node<'b>),
    NodeList(&'b NodeList<'b>),
    Bool(bool),
    MaybeStrTerm(Option<StrTerm<'b>>),
    Num(i32),

    /* For custom superclass rule */
    Superclass(Superclass<'b>),

    /* For custom opt_ensure rule */
    OptEnsure(Option<Ensure<'b>>),

    /* For custom opt_else rule */
    OptElse(Option<Else<'b>>),

    /* For custom exc_var rule */
    ExcVar(ExcVar<'b>),

    /* For custom if_tail rule */
    IfTail(IfTail<'b>),

    /* For custom expr_value_do rule */
    ExprValueDo(ExprValueDo<'b>),

    /* For custom p_kw_label rule */
    PKwLabel(PKwLabel<'b>),

    /* For custom brace_body rule */
    BraceBody(BraceBody<'b>),

    /* For custom cmd_brace_block rule */
    CmdBraceBlock(CmdBraceBlock<'b>),

    /* For custom paren_args rule  */
    ParenArgs(ParenArgs<'b>),

    /* For custom opt_paren_args rule  */
    OptParenArgs(OptParenArgs<'b>),

    /* For custom lambda_body rule  */
    LambdaBody(LambdaBody<'b>),

    /* For custom do_block rule  */
    DoBlock(DoBlock<'b>),

    /* For custom brace_block rule  */
    BraceBlock(BraceBlock<'b>),

    /* For custom defs_head rule */
    DefsHead(DefsHead<'b>),

    /* For custom defn_head rule */
    DefnHead(DefnHead<'b>),

    /* For custom begin_block rule  */
    BeginBlock(BeginBlock<'b>),

    /* For custom cases rule */
    Cases(Cases<'b>),

    /* For custom case_body rule */
    CaseBody(CaseBody<'b>),

    /* For custom p_cases rule */
    PCases(PCases<'b>),

    /* For custom p_case_body rule */
    PCaseBody(PCaseBody<'b>),

    /* For custom compstmt rule */
    MaybeNode(Option<&'b Node<'b>>),

    /* For custom do_body rule */
    DoBody(DoBody<'b>),

    /* For custom p_top_expr rule */
    PTopExpr(PTopExpr<'b>),

    /* For pattern matching patterns with trailing comma */
    MatchPatternWithTrailingComma(MatchPatternWithTrailingComma<'b>),

    /* For p_kwnorest and f_no_kwarg rules */
    NoKwRest(NoKwRest<'b>),
}

impl<'b> ParseValue<'b> {
    // rust-bison-skeleton contract
    pub(crate) fn from_token(token: &'b Token<'b>) -> Self {
        Self::Token(token)
    }
    pub(crate) fn new_uninitialized() -> Self {
        Self::Uninitialized
    }
    pub(crate) fn is_uninitialized(&self) -> bool {
        matches!(self, Self::Uninitialized)
    }

    pub(crate) fn new_superclass(value: Superclass<'b>) -> Self {
        Self::Superclass(value)
    }
    pub(crate) fn new_opt_ensure(value: Option<Ensure<'b>>) -> Self {
        Self::OptEnsure(value)
    }
    pub(crate) fn new_opt_else(value: Option<Else<'b>>) -> Self {
        Self::OptElse(value)
    }
    pub(crate) fn new_exc_var(value: ExcVar<'b>) -> Self {
        Self::ExcVar(value)
    }
    pub(crate) fn new_if_tail(value: IfTail<'b>) -> Self {
        Self::IfTail(value)
    }
    pub(crate) fn new_expr_value_do(value: ExprValueDo<'b>) -> Self {
        Self::ExprValueDo(value)
    }
    pub(crate) fn new_p_kw_label(value: PKwLabel<'b>) -> Self {
        Self::PKwLabel(value)
    }
    pub(crate) fn new_brace_body(value: BraceBody<'b>) -> Self {
        Self::BraceBody(value)
    }
    pub(crate) fn new_cmd_brace_block(value: CmdBraceBlock<'b>) -> Self {
        Self::CmdBraceBlock(value)
    }
    pub(crate) fn new_paren_args(value: ParenArgs<'b>) -> Self {
        Self::ParenArgs(value)
    }
    pub(crate) fn new_opt_paren_args(value: OptParenArgs<'b>) -> Self {
        Self::OptParenArgs(value)
    }
    pub(crate) fn new_lambda_body(value: LambdaBody<'b>) -> Self {
        Self::LambdaBody(value)
    }
    pub(crate) fn new_do_block(value: DoBlock<'b>) -> Self {
        Self::DoBlock(value)
    }
    pub(crate) fn new_brace_block(value: BraceBlock<'b>) -> Self {
        Self::BraceBlock(value)
    }
    pub(crate) fn new_defs_head(value: DefsHead<'b>) -> Self {
        Self::DefsHead(value)
    }
    pub(crate) fn new_defn_head(value: DefnHead<'b>) -> Self {
        Self::DefnHead(value)
    }
    pub(crate) fn new_begin_block(value: BeginBlock<'b>) -> Self {
        Self::BeginBlock(value)
    }
    pub(crate) fn new_cases(value: Cases<'b>) -> Self {
        Self::Cases(value)
    }
    pub(crate) fn new_case_body(value: CaseBody<'b>) -> Self {
        Self::CaseBody(value)
    }
    pub(crate) fn new_p_cases(value: PCases<'b>) -> Self {
        Self::PCases(value)
    }
    pub(crate) fn new_p_case_body(value: PCaseBody<'b>) -> Self {
        Self::PCaseBody(value)
    }
    pub(crate) fn new_do_body(value: DoBody<'b>) -> Self {
        Self::DoBody(value)
    }
    pub(crate) fn new_p_top_expr(value: PTopExpr<'b>) -> Self {
        Self::PTopExpr(value)
    }
    pub(crate) fn new_match_pattern_with_trailing_comma(
        value: MatchPatternWithTrailingComma<'b>,
    ) -> Self {
        Self::MatchPatternWithTrailingComma(value)
    }
    pub(crate) fn new_no_kw_rest(value: NoKwRest<'b>) -> Self {
        Self::NoKwRest(value)
    }

    pub(crate) fn make_copy(&self, blob: &'b Blob<'b>) -> Self {
        match self {
            ParseValue::Stolen => ParseValue::Stolen,
            ParseValue::Uninitialized => ParseValue::Uninitialized,
            ParseValue::None => ParseValue::None,
            ParseValue::Token(token) => ParseValue::Token(Token::new(
                token.token_type,
                token.token_value,
                token.loc,
                blob,
            )),
            ParseValue::TokenWithContext(_) => todo!(),
            ParseValue::Context(_) => todo!(),
            ParseValue::Node(_) => todo!(),
            ParseValue::NodeList(_) => todo!(),
            ParseValue::Bool(_) => todo!(),
            ParseValue::MaybeStrTerm(_) => todo!(),
            ParseValue::Num(_) => todo!(),
            ParseValue::Superclass(_) => todo!(),
            ParseValue::OptEnsure(_) => todo!(),
            ParseValue::OptElse(_) => todo!(),
            ParseValue::ExcVar(_) => todo!(),
            ParseValue::IfTail(_) => todo!(),
            ParseValue::ExprValueDo(_) => todo!(),
            ParseValue::PKwLabel(_) => todo!(),
            ParseValue::BraceBody(_) => todo!(),
            ParseValue::CmdBraceBlock(_) => todo!(),
            ParseValue::ParenArgs(_) => todo!(),
            ParseValue::OptParenArgs(_) => todo!(),
            ParseValue::LambdaBody(_) => todo!(),
            ParseValue::DoBlock(_) => todo!(),
            ParseValue::BraceBlock(_) => todo!(),
            ParseValue::DefsHead(_) => todo!(),
            ParseValue::DefnHead(_) => todo!(),
            ParseValue::BeginBlock(_) => todo!(),
            ParseValue::Cases(_) => todo!(),
            ParseValue::CaseBody(_) => todo!(),
            ParseValue::PCases(_) => todo!(),
            ParseValue::PCaseBody(_) => todo!(),
            ParseValue::MaybeNode(_) => todo!(),
            ParseValue::DoBody(_) => todo!(),
            ParseValue::PTopExpr(_) => todo!(),
            ParseValue::MatchPatternWithTrailingComma(_) => todo!(),
            ParseValue::NoKwRest(_) => todo!(),
        }
    }
}

impl Default for ParseValue<'_> {
    fn default() -> Self {
        Self::Stolen
    }
}
