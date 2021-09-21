crate::use_native_or_external!(Ptr);
crate::use_native_or_external!(List);
crate::use_native_or_external!(Maybe);

use crate::builder::{ArgsType, PKwLabel};
use crate::str_term::StrTerm;
use crate::Node;
use crate::Token;

impl Node {
    pub(crate) fn from(value: ParseValue) -> Node {
        match value {
            ParseValue::Node(value) => value.unptr(),
            other => unreachable!("expected Node, got {:?}", other),
        }
    }
}

#[allow(non_snake_case)]
pub(crate) mod BoxedNode {
    use super::{Node, ParseValue, Ptr};

    pub(crate) fn from(value: ParseValue) -> Ptr<Node> {
        match value {
            ParseValue::Node(value) => value,
            other => unreachable!("expected BoxedNode, got {:?}", other),
        }
    }
}

impl Token {
    pub(crate) fn from(value: ParseValue) -> Ptr<Token> {
        match value {
            ParseValue::Token(value) => value,
            other => unreachable!("expected Token, got {:?}", other),
        }
    }
}

#[allow(non_snake_case)]
pub(crate) mod NodeList {
    use super::{List, Node, ParseValue};

    pub(crate) fn from(value: ParseValue) -> List<Node> {
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

    pub(crate) fn from(value: ParseValue) -> Option<Box<StrTerm>> {
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

#[derive(Debug, Clone)]
pub(crate) struct Superclass {
    pub(crate) lt_t: Maybe<Ptr<Token>>,
    pub(crate) value: Maybe<Ptr<Node>>,
}
impl Superclass {
    pub(crate) fn from(value: ParseValue) -> Superclass {
        match value {
            ParseValue::Superclass(value) => *value,
            other => unreachable!("expected Superclass, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Ensure {
    pub(crate) ensure_t: Ptr<Token>,
    pub(crate) body: Maybe<Ptr<Node>>,
}
#[allow(non_snake_case)]
pub(crate) mod OptEnsure {
    use super::{Ensure, ParseValue};

    pub(crate) fn from(value: ParseValue) -> Option<Ensure> {
        match value {
            ParseValue::OptEnsure(value) => *value,
            other => unreachable!("expected OptEnsure, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Else {
    pub(crate) else_t: Ptr<Token>,
    pub(crate) body: Maybe<Ptr<Node>>,
}
#[allow(non_snake_case)]
pub(crate) mod OptElse {
    use super::{Else, ParseValue};

    pub(crate) fn from(value: ParseValue) -> Option<Else> {
        match value {
            ParseValue::OptElse(value) => *value,
            other => unreachable!("expected OptElse, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ExcVar {
    pub(crate) assoc_t: Maybe<Ptr<Token>>,
    pub(crate) exc_var: Maybe<Ptr<Node>>,
}
impl ExcVar {
    pub(crate) fn from(value: ParseValue) -> ExcVar {
        match value {
            ParseValue::ExcVar(value) => *value,
            other => unreachable!("expected ExcVar, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct IfTail {
    pub(crate) keyword_t: Maybe<Ptr<Token>>,
    pub(crate) body: Maybe<Ptr<Node>>,
}
impl IfTail {
    pub(crate) fn from(value: ParseValue) -> IfTail {
        match value {
            ParseValue::IfTail(value) => *value,
            other => unreachable!("expected IfTail, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ExprValueDo {
    pub(crate) value: Ptr<Node>,
    pub(crate) do_t: Ptr<Token>,
}
impl ExprValueDo {
    pub(crate) fn from(value: ParseValue) -> ExprValueDo {
        match value {
            ParseValue::ExprValueDo(value) => *value,
            other => unreachable!("expected ExprValueDo, got {:?}", other),
        }
    }
}

impl PKwLabel {
    pub(crate) fn from(value: ParseValue) -> PKwLabel {
        match value {
            ParseValue::PKwLabel(value) => *value,
            other => unreachable!("expected PKwLabel, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct BraceBody {
    pub(crate) args_type: ArgsType,
    pub(crate) body: Maybe<Ptr<Node>>,
}
impl BraceBody {
    pub(crate) fn from(value: ParseValue) -> BraceBody {
        match value {
            ParseValue::BraceBody(value) => *value,
            other => unreachable!("expected BraceBody, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct CmdBraceBlock {
    pub(crate) begin_t: Ptr<Token>,
    pub(crate) args_type: ArgsType,
    pub(crate) body: Maybe<Ptr<Node>>,
    pub(crate) end_t: Ptr<Token>,
}
impl CmdBraceBlock {
    pub(crate) fn from(value: ParseValue) -> CmdBraceBlock {
        match value {
            ParseValue::CmdBraceBlock(value) => *value,
            other => unreachable!("expected CmdBraceBlock, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ParenArgs {
    pub(crate) begin_t: Ptr<Token>,
    pub(crate) args: List<Node>,
    pub(crate) end_t: Ptr<Token>,
}
impl ParenArgs {
    pub(crate) fn from(value: ParseValue) -> ParenArgs {
        match value {
            ParseValue::ParenArgs(value) => *value,
            other => unreachable!("expected ParenArgs, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct OptParenArgs {
    pub(crate) begin_t: Maybe<Ptr<Token>>,
    pub(crate) args: List<Node>,
    pub(crate) end_t: Maybe<Ptr<Token>>,
}
impl OptParenArgs {
    pub(crate) fn from(value: ParseValue) -> OptParenArgs {
        match value {
            ParseValue::OptParenArgs(value) => *value,
            other => unreachable!("expected OptParenArgs, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct BeginBlock {
    pub(crate) begin_t: Ptr<Token>,
    pub(crate) body: Maybe<Ptr<Node>>,
    pub(crate) end_t: Ptr<Token>,
}
impl BeginBlock {
    pub(crate) fn from(value: ParseValue) -> BeginBlock {
        match value {
            ParseValue::BeginBlock(value) => *value,
            other => unreachable!("expected BeginBlock, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct LambdaBody {
    pub(crate) begin_t: Ptr<Token>,
    pub(crate) body: Maybe<Ptr<Node>>,
    pub(crate) end_t: Ptr<Token>,
}
impl LambdaBody {
    pub(crate) fn from(value: ParseValue) -> LambdaBody {
        match value {
            ParseValue::LambdaBody(value) => *value,
            other => unreachable!("expected LambdaBody, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DoBlock {
    pub(crate) begin_t: Ptr<Token>,
    pub(crate) args_type: ArgsType,
    pub(crate) body: Maybe<Ptr<Node>>,
    pub(crate) end_t: Ptr<Token>,
}
impl DoBlock {
    pub(crate) fn from(value: ParseValue) -> DoBlock {
        match value {
            ParseValue::DoBlock(value) => *value,
            other => unreachable!("expected DoBlock, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct BraceBlock {
    pub(crate) begin_t: Ptr<Token>,
    pub(crate) args_type: ArgsType,
    pub(crate) body: Maybe<Ptr<Node>>,
    pub(crate) end_t: Ptr<Token>,
}
impl BraceBlock {
    pub(crate) fn from(value: ParseValue) -> BraceBlock {
        match value {
            ParseValue::BraceBlock(value) => *value,
            other => unreachable!("expected BraceBlock, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DefsHead {
    pub(crate) def_t: Ptr<Token>,
    pub(crate) definee: Ptr<Node>,
    pub(crate) dot_t: Ptr<Token>,
    pub(crate) name_t: Ptr<Token>,
}
impl DefsHead {
    pub(crate) fn from(value: ParseValue) -> DefsHead {
        match value {
            ParseValue::DefsHead(value) => *value,
            other => unreachable!("expected DefsHead, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DefnHead {
    pub(crate) def_t: Ptr<Token>,
    pub(crate) name_t: Ptr<Token>,
}
impl DefnHead {
    pub(crate) fn from(value: ParseValue) -> DefnHead {
        match value {
            ParseValue::DefnHead(value) => *value,
            other => unreachable!("expected DefnHead, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Cases {
    pub(crate) when_bodies: List<Node>,
    pub(crate) opt_else: Option<Else>,
}
impl Cases {
    pub(crate) fn from(value: ParseValue) -> Cases {
        match value {
            ParseValue::Cases(value) => *value,
            other => unreachable!("expected Cases, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct CaseBody {
    pub(crate) when_bodies: List<Node>,
    pub(crate) opt_else: Option<Else>,
}
impl CaseBody {
    pub(crate) fn from(value: ParseValue) -> CaseBody {
        match value {
            ParseValue::CaseBody(value) => *value,
            other => unreachable!("expected CaseBody, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PCases {
    pub(crate) in_bodies: List<Node>,
    pub(crate) opt_else: Option<Else>,
}
impl PCases {
    pub(crate) fn from(value: ParseValue) -> PCases {
        match value {
            ParseValue::PCases(value) => *value,
            other => unreachable!("expected PCases, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PCaseBody {
    pub(crate) in_bodies: List<Node>,
    pub(crate) opt_else: Option<Else>,
}
impl PCaseBody {
    pub(crate) fn from(value: ParseValue) -> PCaseBody {
        match value {
            ParseValue::PCaseBody(value) => *value,
            other => unreachable!("expected PCaseBody, got {:?}", other),
        }
    }
}

#[allow(non_snake_case)]
pub(crate) mod MaybeNode {
    use super::{Node, ParseValue, PtrAPI};

    pub(crate) fn from(value: ParseValue) -> Option<Node> {
        match value {
            ParseValue::MaybeNode(value) => {
                if value.is_some() {
                    Some(value.unwrap().unptr())
                } else {
                    None
                }
            }
            other => unreachable!("expected MaybeNode, got {:?}", other),
        }
    }
}

#[allow(non_snake_case)]
pub(crate) mod MaybeBoxedNode {
    use super::{Maybe, Node, ParseValue, Ptr};

    pub(crate) fn from(value: ParseValue) -> Maybe<Ptr<Node>> {
        match value {
            ParseValue::MaybeNode(value) => value,
            other => unreachable!("expected MaybeNode, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DoBody {
    pub(crate) args_type: ArgsType,
    pub(crate) body: Maybe<Ptr<Node>>,
}
impl DoBody {
    pub(crate) fn from(value: ParseValue) -> DoBody {
        match value {
            ParseValue::DoBody(value) => *value,
            other => unreachable!("expected DoBody, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PTopExpr {
    pub(crate) pattern: Ptr<Node>,
    pub(crate) guard: Maybe<Ptr<Node>>,
}
impl PTopExpr {
    pub(crate) fn from(value: ParseValue) -> PTopExpr {
        match value {
            ParseValue::PTopExpr(value) => *value,
            other => unreachable!("expected PTopExpr, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct MatchPatternWithTrailingComma {
    pub(crate) elements: List<Node>,
    pub(crate) trailing_comma: Maybe<Ptr<Token>>,
}
impl MatchPatternWithTrailingComma {
    pub(crate) fn from(value: ParseValue) -> MatchPatternWithTrailingComma {
        match value {
            ParseValue::MatchPatternWithTrailingComma(value) => *value,
            other => unreachable!("expected MatchPatternWithTrailingComma, got {:?}", other),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum ParseValue {
    Stolen,
    Uninitialized,
    None,
    Token(Ptr<Token>),
    TokenList(List<Token>),
    Node(Ptr<Node>),
    NodeList(List<Node>),
    Bool(bool),
    MaybeStrTerm(Option<Box<StrTerm>>),
    Num(i32),

    /* For custom superclass rule */
    Superclass(Box<Superclass>),

    /* For custom opt_ensure rule */
    OptEnsure(Box<Option<Ensure>>),

    /* For custom opt_else rule */
    OptElse(Box<Option<Else>>),

    /* For custom exc_var rule */
    ExcVar(Box<ExcVar>),

    /* For custom if_tail rule */
    IfTail(Box<IfTail>),

    /* For custom expr_value_do rule */
    ExprValueDo(Box<ExprValueDo>),

    /* For custom p_kw_label rule */
    PKwLabel(Box<PKwLabel>),

    /* For custom brace_body rule */
    BraceBody(Box<BraceBody>),

    /* For custom cmd_brace_block rule */
    CmdBraceBlock(Box<CmdBraceBlock>),

    /* For custom paren_args rule  */
    ParenArgs(Box<ParenArgs>),

    /* For custom opt_paren_args rule  */
    OptParenArgs(Box<OptParenArgs>),

    /* For custom lambda_body rule  */
    LambdaBody(Box<LambdaBody>),

    /* For custom do_block rule  */
    DoBlock(Box<DoBlock>),

    /* For custom brace_block rule  */
    BraceBlock(Box<BraceBlock>),

    /* For custom defs_head rule */
    DefsHead(Box<DefsHead>),

    /* For custom defn_head rule */
    DefnHead(Box<DefnHead>),

    /* For custom begin_block rule  */
    BeginBlock(Box<BeginBlock>),

    /* For custom cases rule */
    Cases(Box<Cases>),

    /* For custom case_body rule */
    CaseBody(Box<CaseBody>),

    /* For custom p_cases rule */
    PCases(Box<PCases>),

    /* For custom p_case_body rule */
    PCaseBody(Box<PCaseBody>),

    /* For custom compstmt rule */
    MaybeNode(Maybe<Ptr<Node>>),

    /* For custom do_body rule */
    DoBody(Box<DoBody>),

    /* For custom p_top_expr rule */
    PTopExpr(Box<PTopExpr>),

    /* For pattern matching patterns with trailing comma */
    MatchPatternWithTrailingComma(Box<MatchPatternWithTrailingComma>),
}

impl ParseValue {
    pub(crate) fn from_token(token: Ptr<Token>) -> Self {
        Self::Token(token)
    }

    pub(crate) fn new_superclass(value: Superclass) -> Self {
        Self::Superclass(Box::new(value))
    }
    pub(crate) fn new_opt_ensure(value: Option<Ensure>) -> Self {
        Self::OptEnsure(Box::new(value))
    }
    pub(crate) fn new_opt_else(value: Option<Else>) -> Self {
        Self::OptElse(Box::new(value))
    }
    pub(crate) fn new_exc_var(value: ExcVar) -> Self {
        Self::ExcVar(Box::new(value))
    }
    pub(crate) fn new_if_tail(value: IfTail) -> Self {
        Self::IfTail(Box::new(value))
    }
    pub(crate) fn new_expr_value_do(value: ExprValueDo) -> Self {
        Self::ExprValueDo(Box::new(value))
    }
    pub(crate) fn new_p_kw_label(value: PKwLabel) -> Self {
        Self::PKwLabel(Box::new(value))
    }
    pub(crate) fn new_brace_body(value: BraceBody) -> Self {
        Self::BraceBody(Box::new(value))
    }
    pub(crate) fn new_cmd_brace_block(value: CmdBraceBlock) -> Self {
        Self::CmdBraceBlock(Box::new(value))
    }
    pub(crate) fn new_paren_args(value: ParenArgs) -> Self {
        Self::ParenArgs(Box::new(value))
    }
    pub(crate) fn new_opt_paren_args(value: OptParenArgs) -> Self {
        Self::OptParenArgs(Box::new(value))
    }
    pub(crate) fn new_lambda_body(value: LambdaBody) -> Self {
        Self::LambdaBody(Box::new(value))
    }
    pub(crate) fn new_do_block(value: DoBlock) -> Self {
        Self::DoBlock(Box::new(value))
    }
    pub(crate) fn new_brace_block(value: BraceBlock) -> Self {
        Self::BraceBlock(Box::new(value))
    }
    pub(crate) fn new_defs_head(value: DefsHead) -> Self {
        Self::DefsHead(Box::new(value))
    }
    pub(crate) fn new_defn_head(value: DefnHead) -> Self {
        Self::DefnHead(Box::new(value))
    }
    pub(crate) fn new_begin_block(value: BeginBlock) -> Self {
        Self::BeginBlock(Box::new(value))
    }
    pub(crate) fn new_cases(value: Cases) -> Self {
        Self::Cases(Box::new(value))
    }
    pub(crate) fn new_case_body(value: CaseBody) -> Self {
        Self::CaseBody(Box::new(value))
    }
    pub(crate) fn new_p_cases(value: PCases) -> Self {
        Self::PCases(Box::new(value))
    }
    pub(crate) fn new_p_case_body(value: PCaseBody) -> Self {
        Self::PCaseBody(Box::new(value))
    }
    pub(crate) fn new_do_body(value: DoBody) -> Self {
        Self::DoBody(Box::new(value))
    }
    pub(crate) fn new_p_top_expr(value: PTopExpr) -> Self {
        Self::PTopExpr(Box::new(value))
    }
    pub(crate) fn new_match_pattern_with_trailing_comma(
        value: MatchPatternWithTrailingComma,
    ) -> Self {
        Self::MatchPatternWithTrailingComma(Box::new(value))
    }
}

impl Default for ParseValue {
    fn default() -> Self {
        Self::Stolen
    }
}
