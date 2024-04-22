use lib_ruby_parser_ast_arena::Blob;

use crate::builder::{ArgsType, PKwLabel};
use crate::context::Context;
use crate::str_term::StrTerm;
use crate::Node;
use crate::Token;

impl From<ParseValue<'_>> for Node {
    fn from(value: ParseValue) -> Node {
        match value {
            ParseValue::Node(value) => *value,
            other => unreachable!("expected Node, got {:?}", other),
        }
    }
}

#[allow(non_snake_case)]
pub(crate) mod BoxedNode {
    use super::{Node, ParseValue};

    pub(crate) fn from(value: ParseValue) -> Box<Node> {
        match value {
            ParseValue::Node(value) => value,
            other => unreachable!("expected BoxedNode, got {:?}", other),
        }
    }
}

impl<'b> Token<'b> {
    pub(crate) fn from(value: ParseValue<'b>) -> &'b mut Self {
        match value {
            ParseValue::Token(value) => value,
            other => unreachable!("expected Token, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct TokenWithContext<'b> {
    pub(crate) token: &'b mut Token<'b>,
    pub(crate) ctx: Context,
}

impl<'b> TokenWithContext<'b> {
    pub(crate) fn from(value: ParseValue<'b>) -> Self {
        match value {
            ParseValue::TokenWithContext(value) => value,
            other => unreachable!("expected TokenWithContext, got {:?}", other),
        }
    }
}

impl Context {
    pub(crate) fn from(value: ParseValue) -> Self {
        match value {
            ParseValue::Context(ctx) => ctx,
            other => unreachable!("expected Context, got {:?}", other),
        }
    }
}

#[allow(non_snake_case)]
pub(crate) mod NodeList {
    use super::{Node, ParseValue};

    pub(crate) fn from(value: ParseValue) -> Vec<Node> {
        match value {
            ParseValue::NodeList(value) => *value,
            other => unreachable!("expected NodeList, got {:?}", other),
        }
    }
}

#[allow(non_snake_case)]
#[allow(clippy::box_collection)]
pub(crate) mod BoxedNodeList {
    use super::{Node, ParseValue};

    pub(crate) fn from(value: ParseValue) -> Box<Vec<Node>> {
        match value {
            ParseValue::NodeList(value) => value,
            other => unreachable!("expected NodeList, got {:?}", other),
        }
    }
}

#[allow(non_snake_case)]
pub(crate) mod Bool {
    use super::ParseValue;

    pub(crate) fn from(value: ParseValue) -> bool {
        match value {
            ParseValue::Bool(value) => value,
            other => unreachable!("expected Bool, got {:?}", other),
        }
    }
}

#[allow(non_snake_case)]
pub(crate) mod MaybeStrTerm {
    use super::{ParseValue, StrTerm};

    pub(crate) fn from<'b>(value: ParseValue<'b>) -> Option<StrTerm<'b>> {
        match value {
            ParseValue::MaybeStrTerm(value) => value,
            other => unreachable!("expected MaybeStrTerm, got {:?}", other),
        }
    }
}

#[allow(non_snake_case)]
pub(crate) mod Num {
    use super::ParseValue;

    pub(crate) fn from(value: ParseValue) -> i32 {
        match value {
            ParseValue::Num(value) => value,
            other => unreachable!("expected Num, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Superclass<'b> {
    pub(crate) lt_t: Option<&'b mut Token<'b>>,
    pub(crate) value: Option<Box<Node>>,
}
impl<'b> Superclass<'b> {
    pub(crate) fn from(value: ParseValue<'b>) -> Superclass<'b> {
        match value {
            ParseValue::Superclass(value) => *value,
            other => unreachable!("expected Superclass, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Ensure<'b> {
    pub(crate) ensure_t: &'b mut Token<'b>,
    pub(crate) body: Option<Box<Node>>,
}
#[allow(non_snake_case)]
pub(crate) mod OptEnsure {
    use super::{Ensure, ParseValue};

    pub(crate) fn from<'b>(value: ParseValue<'b>) -> Option<Ensure<'b>> {
        match value {
            ParseValue::OptEnsure(value) => value.map(|v| *v),
            other => unreachable!("expected OptEnsure, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Else<'b> {
    pub(crate) else_t: &'b mut Token<'b>,
    pub(crate) body: Option<Box<Node>>,
}
#[allow(non_snake_case)]
pub(crate) mod OptElse {
    use super::{Else, ParseValue};

    pub(crate) fn from<'b>(value: ParseValue<'b>) -> Option<Else<'b>> {
        match value {
            ParseValue::OptElse(value) => value.map(|v| *v),
            other => unreachable!("expected OptElse, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct ExcVar<'b> {
    pub(crate) assoc_t: Option<&'b mut Token<'b>>,
    pub(crate) exc_var: Option<Box<Node>>,
}
impl<'b> ExcVar<'b> {
    pub(crate) fn from(value: ParseValue<'b>) -> ExcVar<'b> {
        match value {
            ParseValue::ExcVar(value) => *value,
            other => unreachable!("expected ExcVar, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct IfTail<'b> {
    pub(crate) keyword_t: Option<&'b mut Token<'b>>,
    pub(crate) body: Option<Box<Node>>,
}
impl<'b> IfTail<'b> {
    pub(crate) fn from(value: ParseValue) -> IfTail {
        match value {
            ParseValue::IfTail(value) => *value,
            other => unreachable!("expected IfTail, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct ExprValueDo<'b> {
    pub(crate) value: Box<Node>,
    pub(crate) do_t: &'b mut Token<'b>,
}
impl<'b> ExprValueDo<'b> {
    pub(crate) fn from(value: ParseValue<'b>) -> ExprValueDo<'b> {
        match value {
            ParseValue::ExprValueDo(value) => *value,
            other => unreachable!("expected ExprValueDo, got {:?}", other),
        }
    }
}

impl<'b> PKwLabel<'b> {
    pub(crate) fn from(value: ParseValue<'b>) -> PKwLabel<'b> {
        match value {
            ParseValue::PKwLabel(value) => *value,
            other => unreachable!("expected PKwLabel, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct BraceBody {
    pub(crate) args_type: ArgsType,
    pub(crate) body: Option<Box<Node>>,
}
impl BraceBody {
    pub(crate) fn from(value: ParseValue) -> BraceBody {
        match value {
            ParseValue::BraceBody(value) => *value,
            other => unreachable!("expected BraceBody, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct CmdBraceBlock<'b> {
    pub(crate) begin_t: &'b mut Token<'b>,
    pub(crate) args_type: ArgsType,
    pub(crate) body: Option<Box<Node>>,
    pub(crate) end_t: &'b mut Token<'b>,
}
impl<'b> CmdBraceBlock<'b> {
    pub(crate) fn from(value: ParseValue<'b>) -> CmdBraceBlock<'b> {
        match value {
            ParseValue::CmdBraceBlock(value) => *value,
            other => unreachable!("expected CmdBraceBlock, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct ParenArgs<'b> {
    pub(crate) begin_t: &'b mut Token<'b>,
    pub(crate) args: Vec<Node>,
    pub(crate) end_t: &'b mut Token<'b>,
}
impl<'b> ParenArgs<'b> {
    pub(crate) fn from(value: ParseValue<'b>) -> ParenArgs<'b> {
        match value {
            ParseValue::ParenArgs(value) => *value,
            other => unreachable!("expected ParenArgs, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct OptParenArgs<'b> {
    pub(crate) begin_t: Option<&'b mut Token<'b>>,
    pub(crate) args: Vec<Node>,
    pub(crate) end_t: Option<&'b mut Token<'b>>,
}
impl<'b> OptParenArgs<'b> {
    pub(crate) fn from(value: ParseValue<'b>) -> OptParenArgs<'b> {
        match value {
            ParseValue::OptParenArgs(value) => *value,
            other => unreachable!("expected OptParenArgs, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct BeginBlock<'b> {
    pub(crate) begin_t: &'b mut Token<'b>,
    pub(crate) body: Option<Box<Node>>,
    pub(crate) end_t: &'b mut Token<'b>,
}
impl<'b> BeginBlock<'b> {
    pub(crate) fn from(value: ParseValue<'b>) -> BeginBlock<'b> {
        match value {
            ParseValue::BeginBlock(value) => *value,
            other => unreachable!("expected BeginBlock, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct LambdaBody<'b> {
    pub(crate) begin_t: &'b mut Token<'b>,
    pub(crate) body: Option<Box<Node>>,
    pub(crate) end_t: &'b mut Token<'b>,
}
impl<'b> LambdaBody<'b> {
    pub(crate) fn from(value: ParseValue<'b>) -> LambdaBody<'b> {
        match value {
            ParseValue::LambdaBody(value) => *value,
            other => unreachable!("expected LambdaBody, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct DoBlock<'b> {
    pub(crate) begin_t: &'b mut Token<'b>,
    pub(crate) args_type: ArgsType,
    pub(crate) body: Option<Box<Node>>,
    pub(crate) end_t: &'b mut Token<'b>,
}
impl<'b> DoBlock<'b> {
    pub(crate) fn from(value: ParseValue<'b>) -> DoBlock<'b> {
        match value {
            ParseValue::DoBlock(value) => *value,
            other => unreachable!("expected DoBlock, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct BraceBlock<'b> {
    pub(crate) begin_t: &'b mut Token<'b>,
    pub(crate) args_type: ArgsType,
    pub(crate) body: Option<Box<Node>>,
    pub(crate) end_t: &'b mut Token<'b>,
}
impl<'b> BraceBlock<'b> {
    pub(crate) fn from(value: ParseValue<'b>) -> BraceBlock<'b> {
        match value {
            ParseValue::BraceBlock(value) => *value,
            other => unreachable!("expected BraceBlock, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct DefsHead<'b> {
    pub(crate) def_t: &'b mut Token<'b>,
    pub(crate) definee: Box<Node>,
    pub(crate) dot_t: &'b mut Token<'b>,
    pub(crate) name_t: TokenWithContext<'b>,
}
impl<'b> DefsHead<'b> {
    pub(crate) fn from(value: ParseValue<'b>) -> DefsHead<'b> {
        match value {
            ParseValue::DefsHead(value) => *value,
            other => unreachable!("expected DefsHead, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct DefnHead<'b> {
    pub(crate) def_t: &'b mut Token<'b>,
    pub(crate) name_t: TokenWithContext<'b>,
}
impl<'b> DefnHead<'b> {
    pub(crate) fn from(value: ParseValue<'b>) -> DefnHead<'b> {
        match value {
            ParseValue::DefnHead(value) => *value,
            other => unreachable!("expected DefnHead, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Cases<'b> {
    pub(crate) when_bodies: Vec<Node>,
    pub(crate) opt_else: Option<Else<'b>>,
}
impl<'b> Cases<'b> {
    pub(crate) fn from(value: ParseValue<'b>) -> Cases<'b> {
        match value {
            ParseValue::Cases(value) => *value,
            other => unreachable!("expected Cases, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct CaseBody<'b> {
    pub(crate) when_bodies: Vec<Node>,
    pub(crate) opt_else: Option<Else<'b>>,
}
impl<'b> CaseBody<'b> {
    pub(crate) fn from(value: ParseValue<'b>) -> CaseBody<'b> {
        match value {
            ParseValue::CaseBody(value) => *value,
            other => unreachable!("expected CaseBody, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct PCases<'b> {
    pub(crate) in_bodies: Vec<Node>,
    pub(crate) opt_else: Option<Else<'b>>,
}
impl<'b> PCases<'b> {
    pub(crate) fn from(value: ParseValue<'b>) -> PCases<'b> {
        match value {
            ParseValue::PCases(value) => *value,
            other => unreachable!("expected PCases, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct PCaseBody<'b> {
    pub(crate) in_bodies: Vec<Node>,
    pub(crate) opt_else: Option<Else<'b>>,
}
impl<'b> PCaseBody<'b> {
    pub(crate) fn from(value: ParseValue<'b>) -> PCaseBody<'b> {
        match value {
            ParseValue::PCaseBody(value) => *value,
            other => unreachable!("expected PCaseBody, got {:?}", other),
        }
    }
}

#[allow(non_snake_case)]
pub(crate) mod MaybeNode {
    use super::{Node, ParseValue};

    pub(crate) fn from(value: ParseValue) -> Option<Node> {
        match value {
            ParseValue::MaybeNode(maybe_node) => maybe_node.map(|node| *node),
            other => unreachable!("expected MaybeNode, got {:?}", other),
        }
    }
}

#[allow(non_snake_case)]
pub(crate) mod MaybeBoxedNode {
    use super::{Node, ParseValue};

    pub(crate) fn from(value: ParseValue) -> Option<Box<Node>> {
        match value {
            ParseValue::MaybeNode(value) => value,
            other => unreachable!("expected MaybeNode, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct DoBody {
    pub(crate) args_type: ArgsType,
    pub(crate) body: Option<Box<Node>>,
}
impl DoBody {
    pub(crate) fn from(value: ParseValue) -> DoBody {
        match value {
            ParseValue::DoBody(value) => *value,
            other => unreachable!("expected DoBody, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct PTopExpr {
    pub(crate) pattern: Box<Node>,
    pub(crate) guard: Option<Box<Node>>,
}
impl PTopExpr {
    pub(crate) fn from(value: ParseValue) -> PTopExpr {
        match value {
            ParseValue::PTopExpr(value) => *value,
            other => unreachable!("expected PTopExpr, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct MatchPatternWithTrailingComma<'b> {
    pub(crate) elements: Vec<Node>,
    pub(crate) trailing_comma: Option<&'b mut Token<'b>>,
}
impl<'b> MatchPatternWithTrailingComma<'b> {
    pub(crate) fn from(value: ParseValue<'b>) -> Self {
        match value {
            ParseValue::MatchPatternWithTrailingComma(value) => *value,
            other => unreachable!("expected MatchPatternWithTrailingComma, got {:?}", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct NoKwRest<'b> {
    pub(crate) kwrest_mark: &'b mut Token<'b>,
    pub(crate) k_nil: &'b mut Token<'b>,
}
impl<'b> NoKwRest<'b> {
    pub(crate) fn from(value: ParseValue<'b>) -> Self {
        match value {
            ParseValue::NoKwRest(value) => *value,
            other => unreachable!("expected NoKwRest, got {:?}", other),
        }
    }
}

#[allow(clippy::box_collection)]
#[derive(Debug)]
pub(crate) enum ParseValue<'b> {
    Stolen,
    Uninitialized,
    None,
    Token(&'b mut Token<'b>),
    TokenWithContext(TokenWithContext<'b>),
    Context(Context),
    Node(Box<Node>),
    NodeList(Box<Vec<Node>>),
    Bool(bool),
    MaybeStrTerm(Option<StrTerm<'b>>),
    Num(i32),

    /* For custom superclass rule */
    Superclass(Box<Superclass<'b>>),

    /* For custom opt_ensure rule */
    OptEnsure(Option<Box<Ensure<'b>>>),

    /* For custom opt_else rule */
    OptElse(Option<Box<Else<'b>>>),

    /* For custom exc_var rule */
    ExcVar(Box<ExcVar<'b>>),

    /* For custom if_tail rule */
    IfTail(Box<IfTail<'b>>),

    /* For custom expr_value_do rule */
    ExprValueDo(Box<ExprValueDo<'b>>),

    /* For custom p_kw_label rule */
    PKwLabel(Box<PKwLabel<'b>>),

    /* For custom brace_body rule */
    BraceBody(Box<BraceBody>),

    /* For custom cmd_brace_block rule */
    CmdBraceBlock(Box<CmdBraceBlock<'b>>),

    /* For custom paren_args rule  */
    ParenArgs(Box<ParenArgs<'b>>),

    /* For custom opt_paren_args rule  */
    OptParenArgs(Box<OptParenArgs<'b>>),

    /* For custom lambda_body rule  */
    LambdaBody(Box<LambdaBody<'b>>),

    /* For custom do_block rule  */
    DoBlock(Box<DoBlock<'b>>),

    /* For custom brace_block rule  */
    BraceBlock(Box<BraceBlock<'b>>),

    /* For custom defs_head rule */
    DefsHead(Box<DefsHead<'b>>),

    /* For custom defn_head rule */
    DefnHead(Box<DefnHead<'b>>),

    /* For custom begin_block rule  */
    BeginBlock(Box<BeginBlock<'b>>),

    /* For custom cases rule */
    Cases(Box<Cases<'b>>),

    /* For custom case_body rule */
    CaseBody(Box<CaseBody<'b>>),

    /* For custom p_cases rule */
    PCases(Box<PCases<'b>>),

    /* For custom p_case_body rule */
    PCaseBody(Box<PCaseBody<'b>>),

    /* For custom compstmt rule */
    MaybeNode(Option<Box<Node>>),

    /* For custom do_body rule */
    DoBody(Box<DoBody>),

    /* For custom p_top_expr rule */
    PTopExpr(Box<PTopExpr>),

    /* For pattern matching patterns with trailing comma */
    MatchPatternWithTrailingComma(Box<MatchPatternWithTrailingComma<'b>>),

    /* For p_kwnorest and f_no_kwarg rules */
    NoKwRest(Box<NoKwRest<'b>>),
}

impl<'b> ParseValue<'b> {
    // rust-bison-skeleton contract
    pub(crate) fn from_token(token: &'b mut Token<'b>) -> Self {
        Self::Token(token)
    }
    pub(crate) fn new_uninitialized() -> Self {
        Self::Uninitialized
    }
    pub(crate) fn is_uninitialized(&self) -> bool {
        matches!(self, Self::Uninitialized)
    }

    pub(crate) fn new_superclass(value: Superclass<'b>) -> Self {
        Self::Superclass(Box::new(value))
    }
    pub(crate) fn new_opt_ensure(value: Option<Ensure<'b>>) -> Self {
        Self::OptEnsure(value.map(Box::new))
    }
    pub(crate) fn new_opt_else(value: Option<Else<'b>>) -> Self {
        Self::OptElse(value.map(Box::new))
    }
    pub(crate) fn new_exc_var(value: ExcVar<'b>) -> Self {
        Self::ExcVar(Box::new(value))
    }
    pub(crate) fn new_if_tail(value: IfTail<'b>) -> Self {
        Self::IfTail(Box::new(value))
    }
    pub(crate) fn new_expr_value_do(value: ExprValueDo<'b>) -> Self {
        Self::ExprValueDo(Box::new(value))
    }
    pub(crate) fn new_p_kw_label(value: PKwLabel<'b>) -> Self {
        Self::PKwLabel(Box::new(value))
    }
    pub(crate) fn new_brace_body(value: BraceBody) -> Self {
        Self::BraceBody(Box::new(value))
    }
    pub(crate) fn new_cmd_brace_block(value: CmdBraceBlock<'b>) -> Self {
        Self::CmdBraceBlock(Box::new(value))
    }
    pub(crate) fn new_paren_args(value: ParenArgs<'b>) -> Self {
        Self::ParenArgs(Box::new(value))
    }
    pub(crate) fn new_opt_paren_args(value: OptParenArgs<'b>) -> Self {
        Self::OptParenArgs(Box::new(value))
    }
    pub(crate) fn new_lambda_body(value: LambdaBody<'b>) -> Self {
        Self::LambdaBody(Box::new(value))
    }
    pub(crate) fn new_do_block(value: DoBlock<'b>) -> Self {
        Self::DoBlock(Box::new(value))
    }
    pub(crate) fn new_brace_block(value: BraceBlock<'b>) -> Self {
        Self::BraceBlock(Box::new(value))
    }
    pub(crate) fn new_defs_head(value: DefsHead<'b>) -> Self {
        Self::DefsHead(Box::new(value))
    }
    pub(crate) fn new_defn_head(value: DefnHead<'b>) -> Self {
        Self::DefnHead(Box::new(value))
    }
    pub(crate) fn new_begin_block(value: BeginBlock<'b>) -> Self {
        Self::BeginBlock(Box::new(value))
    }
    pub(crate) fn new_cases(value: Cases<'b>) -> Self {
        Self::Cases(Box::new(value))
    }
    pub(crate) fn new_case_body(value: CaseBody<'b>) -> Self {
        Self::CaseBody(Box::new(value))
    }
    pub(crate) fn new_p_cases(value: PCases<'b>) -> Self {
        Self::PCases(Box::new(value))
    }
    pub(crate) fn new_p_case_body(value: PCaseBody<'b>) -> Self {
        Self::PCaseBody(Box::new(value))
    }
    pub(crate) fn new_do_body(value: DoBody) -> Self {
        Self::DoBody(Box::new(value))
    }
    pub(crate) fn new_p_top_expr(value: PTopExpr) -> Self {
        Self::PTopExpr(Box::new(value))
    }
    pub(crate) fn new_match_pattern_with_trailing_comma(
        value: MatchPatternWithTrailingComma<'b>,
    ) -> Self {
        Self::MatchPatternWithTrailingComma(Box::new(value))
    }
    pub(crate) fn new_no_kw_rest(value: NoKwRest<'b>) -> Self {
        Self::NoKwRest(Box::new(value))
    }

    pub(crate) fn make_copy(&self, blob: &'b Blob<'b>) -> Self {
        match self {
            ParseValue::Stolen => ParseValue::Stolen,
            ParseValue::Uninitialized => ParseValue::Uninitialized,
            ParseValue::None => ParseValue::None,
            ParseValue::Token(token) => {
                let out = blob.alloc_mut::<Token>();
                out.token_type = token.token_type;
                out.token_value = token.token_value.clone();
                out.loc = token.loc;
                ParseValue::Token(out)
            }
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
