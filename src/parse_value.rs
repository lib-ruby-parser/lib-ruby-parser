use crate::builder::{ArgsType, PKwLabel};
use crate::str_term::StrTerm;
use crate::Node;
use crate::Token;

impl From<ParseValue> for Node {
    fn from(value: ParseValue) -> Node {
        match value {
            ParseValue::Node(value) => value,
            other => unimplemented!("expected Node, got {:?}", other),
        }
    }
}

impl From<ParseValue> for Token {
    fn from(value: ParseValue) -> Token {
        match value {
            ParseValue::Token(value) => value,
            other => unimplemented!("expected Token, got {:?}", other),
        }
    }
}

pub(crate) type TokenList = Vec<Token>;
impl From<ParseValue> for TokenList {
    fn from(value: ParseValue) -> TokenList {
        match value {
            ParseValue::TokenList(value) => value,
            other => unimplemented!("expected TokenList, got {:?}", other),
        }
    }
}

pub(crate) type NodeList = Vec<Node>;
impl From<ParseValue> for NodeList {
    fn from(value: ParseValue) -> NodeList {
        match value {
            ParseValue::NodeList(value) => value,
            other => unimplemented!("expected NodeList, got {:?}", other),
        }
    }
}

pub(crate) type Bool = bool;
impl From<ParseValue> for Bool {
    fn from(value: ParseValue) -> Bool {
        match value {
            ParseValue::Bool(value) => value,
            other => unimplemented!("expected Bool, got {:?}", other),
        }
    }
}

pub(crate) type MaybeStrTerm = Option<StrTerm>;
impl From<ParseValue> for MaybeStrTerm {
    fn from(value: ParseValue) -> MaybeStrTerm {
        match value {
            ParseValue::MaybeStrTerm(value) => value,
            other => unimplemented!("expected MaybeStrTerm, got {:?}", other),
        }
    }
}

pub(crate) type Num = i32;
impl From<ParseValue> for Num {
    fn from(value: ParseValue) -> Num {
        match value {
            ParseValue::Num(value) => value,
            other => unimplemented!("expected Num, got {:?}", other),
        }
    }
}

pub(crate) type Superclass = Option<(Token, Node)>;
impl From<ParseValue> for Superclass {
    fn from(value: ParseValue) -> Superclass {
        match value {
            ParseValue::Superclass(value) => value,
            other => unimplemented!("expected Superclass, got {:?}", other),
        }
    }
}

pub(crate) type OptEnsure = Option<(Token, Option<Node>)>;
impl From<ParseValue> for OptEnsure {
    fn from(value: ParseValue) -> OptEnsure {
        match value {
            ParseValue::OptEnsure(value) => value,
            other => unimplemented!("expected OptEnsure, got {:?}", other),
        }
    }
}

// they are equivalent
pub(crate) type OptElse = OptEnsure;

// they are equivalent
pub(crate) type ExcVar = Superclass;

// they are equivalent
pub(crate) type IfTail = OptElse;

pub(crate) type ExprValueDo = (Node, Token);
impl From<ParseValue> for ExprValueDo {
    fn from(value: ParseValue) -> ExprValueDo {
        match value {
            ParseValue::ExprValueDo(value) => value,
            other => unimplemented!("expected ExprValueDo, got {:?}", other),
        }
    }
}

impl From<ParseValue> for PKwLabel {
    fn from(value: ParseValue) -> PKwLabel {
        match value {
            ParseValue::PKwLabel(value) => value,
            other => unimplemented!("expected PKwLabel, got {:?}", other),
        }
    }
}

pub(crate) type BraceBody = (ArgsType, Option<Node>);
impl From<ParseValue> for BraceBody {
    fn from(value: ParseValue) -> BraceBody {
        match value {
            ParseValue::BraceBody(value) => value,
            other => unimplemented!("expected BraceBody, got {:?}", other),
        }
    }
}

pub(crate) type CmdBraceBlock = (Token, ArgsType, Option<Node>, Token);
impl From<ParseValue> for CmdBraceBlock {
    fn from(value: ParseValue) -> CmdBraceBlock {
        match value {
            ParseValue::CmdBraceBlock(value) => value,
            other => unimplemented!("expected CmdBraceBlock, got {:?}", other),
        }
    }
}

pub(crate) type ParenArgs = (Token, Vec<Node>, Token);
impl From<ParseValue> for ParenArgs {
    fn from(value: ParseValue) -> ParenArgs {
        match value {
            ParseValue::ParenArgs(value) => value,
            other => unimplemented!("expected ParenArgs, got {:?}", other),
        }
    }
}

pub(crate) type OptParenArgs = (Option<Token>, Vec<Node>, Option<Token>);
impl From<ParseValue> for OptParenArgs {
    fn from(value: ParseValue) -> OptParenArgs {
        match value {
            ParseValue::OptParenArgs(value) => value,
            other => unimplemented!("expected OptParenArgs, got {:?}", other),
        }
    }
}

pub(crate) type BeginBlock = (Token, Option<Node>, Token);
impl From<ParseValue> for BeginBlock {
    fn from(value: ParseValue) -> BeginBlock {
        match value {
            ParseValue::BeginBlock(value) => value,
            other => unimplemented!("expected BeginBlock, got {:?}", other),
        }
    }
}

// they are equivalent
pub(crate) type LambdaBody = BeginBlock;

// they are equivalent
pub(crate) type DoBlock = CmdBraceBlock;

// they are equivalent
pub(crate) type BraceBlock = CmdBraceBlock;

pub(crate) type DefsHead = (Token, Node, Token, Token);
impl From<ParseValue> for DefsHead {
    fn from(value: ParseValue) -> DefsHead {
        match value {
            ParseValue::DefsHead(value) => value,
            other => unimplemented!("expected DefsHead, got {:?}", other),
        }
    }
}

pub(crate) type DefnHead = (Token, Token);
impl From<ParseValue> for DefnHead {
    fn from(value: ParseValue) -> DefnHead {
        match value {
            ParseValue::DefnHead(value) => value,
            other => unimplemented!("expected DefnHead, got {:?}", other),
        }
    }
}

pub(crate) type Cases = (Vec<Node>, Option<(Token, Option<Node>)>);
impl From<ParseValue> for Cases {
    fn from(value: ParseValue) -> Cases {
        match value {
            ParseValue::Cases(value) => value,
            other => unimplemented!("expected Cases, got {:?}", other),
        }
    }
}

// they are equivalent
pub(crate) type CaseBody = Cases;

// they are equivalent
pub(crate) type PCases = Cases;

// they are equivalent
pub(crate) type PCaseBody = Cases;

pub(crate) type MaybeNode = Option<Node>;
impl From<ParseValue> for MaybeNode {
    fn from(value: ParseValue) -> MaybeNode {
        match value {
            ParseValue::MaybeNode(value) => value,
            other => unimplemented!("expected MaybeNode, got {:?}", other),
        }
    }
}

// they are equivalent
pub(crate) type DoBody = BraceBody;

pub(crate) type PTopExpr = (Node, Option<Node>);
impl From<ParseValue> for PTopExpr {
    fn from(value: ParseValue) -> PTopExpr {
        match value {
            ParseValue::PTopExpr(value) => value,
            other => unimplemented!("expected PTopExpr, got {:?}", other),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum ParseValue {
    Stolen,
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
