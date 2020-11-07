use crate::builder::{ArgsType, PKwLabel};
use crate::str_term::StrTerm;
use crate::Node;
use crate::Token;

impl From<ParseValue> for Node {
    fn from(value: ParseValue) -> Node {
        match value {
            ParseValue::Node(value) => value,
            other => unreachable!("expected Node, got {:?}", other),
        }
    }
}

impl From<ParseValue> for Token {
    fn from(value: ParseValue) -> Token {
        match value {
            ParseValue::Token(value) => value,
            other => unreachable!("expected Token, got {:?}", other),
        }
    }
}

pub(crate) type TokenList = Vec<Token>;
impl From<ParseValue> for TokenList {
    fn from(value: ParseValue) -> TokenList {
        match value {
            ParseValue::TokenList(value) => value,
            other => unreachable!("expected TokenList, got {:?}", other),
        }
    }
}

pub(crate) type NodeList = Vec<Node>;
impl From<ParseValue> for NodeList {
    fn from(value: ParseValue) -> NodeList {
        match value {
            ParseValue::NodeList(value) => value,
            other => unreachable!("expected NodeList, got {:?}", other),
        }
    }
}

pub(crate) type Bool = bool;
impl From<ParseValue> for Bool {
    fn from(value: ParseValue) -> Bool {
        match value {
            ParseValue::Bool(value) => value,
            other => unreachable!("expected Bool, got {:?}", other),
        }
    }
}

pub(crate) type MaybeStrTerm = Option<StrTerm>;
impl From<ParseValue> for MaybeStrTerm {
    fn from(value: ParseValue) -> MaybeStrTerm {
        match value {
            ParseValue::MaybeStrTerm(value) => value,
            other => unreachable!("expected MaybeStrTerm, got {:?}", other),
        }
    }
}

pub(crate) type Num = i32;
impl From<ParseValue> for Num {
    fn from(value: ParseValue) -> Num {
        match value {
            ParseValue::Num(value) => value,
            other => unreachable!("expected Num, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Superclass {
    pub(crate) lt_t: Option<Token>,
    pub(crate) value: Option<Node>,
}
impl From<ParseValue> for Superclass {
    fn from(value: ParseValue) -> Superclass {
        match value {
            ParseValue::Superclass(value) => value,
            other => unreachable!("expected Superclass, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Ensure {
    pub(crate) ensure_t: Token,
    pub(crate) body: Option<Node>,
}
pub(crate) type OptEnsure = Option<Ensure>;
impl From<ParseValue> for OptEnsure {
    fn from(value: ParseValue) -> OptEnsure {
        match value {
            ParseValue::OptEnsure(value) => value,
            other => unreachable!("expected OptEnsure, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Else {
    pub(crate) else_t: Token,
    pub(crate) body: Option<Node>,
}
pub(crate) type OptElse = Option<Else>;
impl From<ParseValue> for OptElse {
    fn from(value: ParseValue) -> OptElse {
        match value {
            ParseValue::OptElse(value) => value,
            other => unreachable!("expected OptElse, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ExcVar {
    pub(crate) assoc_t: Option<Token>,
    pub(crate) exc_var: Option<Node>,
}
impl From<ParseValue> for ExcVar {
    fn from(value: ParseValue) -> ExcVar {
        match value {
            ParseValue::ExcVar(value) => value,
            other => unreachable!("expected ExcVar, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct IfTail {
    pub(crate) keyword_t: Option<Token>,
    pub(crate) body: Option<Node>,
}
impl From<ParseValue> for IfTail {
    fn from(value: ParseValue) -> IfTail {
        match value {
            ParseValue::IfTail(value) => value,
            other => unreachable!("expected IfTail, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ExprValueDo {
    pub(crate) value: Node,
    pub(crate) do_t: Token,
}
impl From<ParseValue> for ExprValueDo {
    fn from(value: ParseValue) -> ExprValueDo {
        match value {
            ParseValue::ExprValueDo(value) => value,
            other => unreachable!("expected ExprValueDo, got {:?}", other),
        }
    }
}

impl From<ParseValue> for PKwLabel {
    fn from(value: ParseValue) -> PKwLabel {
        match value {
            ParseValue::PKwLabel(value) => value,
            other => unreachable!("expected PKwLabel, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct BraceBody {
    pub(crate) args_type: ArgsType,
    pub(crate) body: Option<Node>,
}
impl From<ParseValue> for BraceBody {
    fn from(value: ParseValue) -> BraceBody {
        match value {
            ParseValue::BraceBody(value) => value,
            other => unreachable!("expected BraceBody, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct CmdBraceBlock {
    pub(crate) begin_t: Token,
    pub(crate) args_type: ArgsType,
    pub(crate) body: Option<Node>,
    pub(crate) end_t: Token,
}
impl From<ParseValue> for CmdBraceBlock {
    fn from(value: ParseValue) -> CmdBraceBlock {
        match value {
            ParseValue::CmdBraceBlock(value) => value,
            other => unreachable!("expected CmdBraceBlock, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ParenArgs {
    pub(crate) begin_t: Token,
    pub(crate) args: Vec<Node>,
    pub(crate) end_t: Token,
}
impl From<ParseValue> for ParenArgs {
    fn from(value: ParseValue) -> ParenArgs {
        match value {
            ParseValue::ParenArgs(value) => value,
            other => unreachable!("expected ParenArgs, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct OptParenArgs {
    pub(crate) begin_t: Option<Token>,
    pub(crate) args: Vec<Node>,
    pub(crate) end_t: Option<Token>,
}
impl From<ParseValue> for OptParenArgs {
    fn from(value: ParseValue) -> OptParenArgs {
        match value {
            ParseValue::OptParenArgs(value) => value,
            other => unreachable!("expected OptParenArgs, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct BeginBlock {
    pub(crate) begin_t: Token,
    pub(crate) body: Option<Node>,
    pub(crate) end_t: Token,
}
impl From<ParseValue> for BeginBlock {
    fn from(value: ParseValue) -> BeginBlock {
        match value {
            ParseValue::BeginBlock(value) => value,
            other => unreachable!("expected BeginBlock, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct LambdaBody {
    pub(crate) begin_t: Token,
    pub(crate) body: Option<Node>,
    pub(crate) end_t: Token,
}
impl From<ParseValue> for LambdaBody {
    fn from(value: ParseValue) -> LambdaBody {
        match value {
            ParseValue::LambdaBody(value) => value,
            other => unreachable!("expected LambdaBody, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DoBlock {
    pub(crate) begin_t: Token,
    pub(crate) args_type: ArgsType,
    pub(crate) body: Option<Node>,
    pub(crate) end_t: Token,
}
impl From<ParseValue> for DoBlock {
    fn from(value: ParseValue) -> DoBlock {
        match value {
            ParseValue::DoBlock(value) => value,
            other => unreachable!("expected DoBlock, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct BraceBlock {
    pub(crate) begin_t: Token,
    pub(crate) args_type: ArgsType,
    pub(crate) body: Option<Node>,
    pub(crate) end_t: Token,
}
impl From<ParseValue> for BraceBlock {
    fn from(value: ParseValue) -> BraceBlock {
        match value {
            ParseValue::BraceBlock(value) => value,
            other => unreachable!("expected BraceBlock, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DefsHead {
    pub(crate) def_t: Token,
    pub(crate) definee: Node,
    pub(crate) dot_t: Token,
    pub(crate) name_t: Token,
}
impl From<ParseValue> for DefsHead {
    fn from(value: ParseValue) -> DefsHead {
        match value {
            ParseValue::DefsHead(value) => value,
            other => unreachable!("expected DefsHead, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DefnHead {
    pub(crate) def_t: Token,
    pub(crate) name_t: Token,
}
impl From<ParseValue> for DefnHead {
    fn from(value: ParseValue) -> DefnHead {
        match value {
            ParseValue::DefnHead(value) => value,
            other => unreachable!("expected DefnHead, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Cases {
    pub(crate) when_bodies: Vec<Node>,
    pub(crate) opt_else: OptElse,
}
impl From<ParseValue> for Cases {
    fn from(value: ParseValue) -> Cases {
        match value {
            ParseValue::Cases(value) => value,
            other => unreachable!("expected Cases, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct CaseBody {
    pub(crate) when_bodies: Vec<Node>,
    pub(crate) opt_else: OptElse,
}
impl From<ParseValue> for CaseBody {
    fn from(value: ParseValue) -> CaseBody {
        match value {
            ParseValue::CaseBody(value) => value,
            other => unreachable!("expected CaseBody, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PCases {
    pub(crate) in_bodies: Vec<Node>,
    pub(crate) opt_else: OptElse,
}
impl From<ParseValue> for PCases {
    fn from(value: ParseValue) -> PCases {
        match value {
            ParseValue::PCases(value) => value,
            other => unreachable!("expected PCases, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PCaseBody {
    pub(crate) in_bodies: Vec<Node>,
    pub(crate) opt_else: OptElse,
}
impl From<ParseValue> for PCaseBody {
    fn from(value: ParseValue) -> PCaseBody {
        match value {
            ParseValue::PCaseBody(value) => value,
            other => unreachable!("expected PCaseBody, got {:?}", other),
        }
    }
}

pub(crate) type MaybeNode = Option<Node>;
impl From<ParseValue> for MaybeNode {
    fn from(value: ParseValue) -> MaybeNode {
        match value {
            ParseValue::MaybeNode(value) => value,
            other => unreachable!("expected MaybeNode, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DoBody {
    pub(crate) args_type: ArgsType,
    pub(crate) body: Option<Node>,
}
impl From<ParseValue> for DoBody {
    fn from(value: ParseValue) -> DoBody {
        match value {
            ParseValue::DoBody(value) => value,
            other => unreachable!("expected DoBody, got {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PTopExpr {
    pub(crate) pattern: Node,
    pub(crate) guard: Option<Node>,
}
impl From<ParseValue> for PTopExpr {
    fn from(value: ParseValue) -> PTopExpr {
        match value {
            ParseValue::PTopExpr(value) => value,
            other => unreachable!("expected PTopExpr, got {:?}", other),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum ParseValue {
    Stolen,
    Uninitialized,
    None,
    Token(Token),
    TokenList(TokenList),
    Node(Node),
    NodeList(NodeList),
    Bool(bool),
    MaybeStrTerm(MaybeStrTerm),
    Num(Num),

    /* For custom superclass rule */
    Superclass(Superclass),

    /* For custom opt_ensure rule */
    OptEnsure(OptEnsure),

    /* For custom opt_else rule */
    OptElse(OptElse),

    /* For custom exc_var rule */
    ExcVar(ExcVar),

    /* For custom if_tail rule */
    IfTail(IfTail),

    /* For custom expr_value_do rule */
    ExprValueDo(ExprValueDo),

    /* For custom p_kw_label rule */
    PKwLabel(PKwLabel),

    /* For custom brace_body rule */
    BraceBody(BraceBody),

    /* For custom cmd_brace_block rule */
    CmdBraceBlock(CmdBraceBlock),

    /* For custom paren_args rule  */
    ParenArgs(ParenArgs),

    /* For custom opt_paren_args rule  */
    OptParenArgs(OptParenArgs),

    /* For custom lambda_body rule  */
    LambdaBody(LambdaBody),

    /* For custom do_block rule  */
    DoBlock(DoBlock),

    /* For custom brace_block rule  */
    BraceBlock(BraceBlock),

    /* For custom defs_head rule */
    DefsHead(DefsHead),

    /* For custom defn_head rule */
    DefnHead(DefnHead),

    /* For custom begin_block rule  */
    BeginBlock(BeginBlock),

    /* For custom cases rule */
    Cases(Cases),

    /* For custom case_body rule */
    CaseBody(CaseBody),

    /* For custom p_cases rule */
    PCases(PCases),

    /* For custom p_case_body rule */
    PCaseBody(PCaseBody),

    /* For custom compstmt rule */
    MaybeNode(MaybeNode),

    /* For custom do_body rule */
    DoBody(DoBody),

    /* For custom p_top_expr rule */
    PTopExpr(PTopExpr),
}

impl ParseValue {
    pub fn from_token(token: Token) -> Self {
        Self::Token(token)
    }
}
