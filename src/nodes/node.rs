use crate::nodes::*;
use crate::source::Range;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Encoding(Encoding),
    File(File),
    Line(Line),
    Alias(Alias),
    And(And),
    AndAsgn(AndAsgn),
    Arg(Arg),
    Args(Args),
    Array(Array),
    ArrayPattern(ArrayPattern),
    ArrayPatternWithTail(ArrayPatternWithTail),
    BackRef(BackRef),
    Begin(Begin),
    Block(Block),
    Blockarg(Blockarg),
    BlockPass(BlockPass),
    Break(Break),
    Case(Case),
    CaseMatch(CaseMatch),
    Casgn(Casgn),
    Cbase(Cbase),
    Class(Class),
    Complex(Complex),
    Const(Const),
    ConstPattern(ConstPattern),
    CSend(CSend),
    Cvar(Cvar),
    Cvasgn(Cvasgn),
    Def(Def),
    Defined(Defined),
    Defs(Defs),
    Dstr(Dstr),
    Dsym(Dsym),
    EFlipFlop(EFlipFlop),
    EmptyElse(EmptyElse),
    Ensure(Ensure),
    Erange(Erange),
    False(False),
    FindPattern(FindPattern),
    Float(Float),
    For(For),
    ForwardArg(ForwardArg),
    ForwardedArgs(ForwardedArgs),
    Gvar(Gvar),
    Gvasgn(Gvasgn),
    Hash(Hash),
    HashPattern(HashPattern),
    Heredoc(Heredoc),
    If(If),
    IfGuard(IfGuard),
    IFlipFlop(IFlipFlop),
    IfMod(IfMod),
    IfTernary(IfTernary),
    Index(Index),
    IndexAsgn(IndexAsgn),
    InMatch(InMatch),
    InPattern(InPattern),
    Int(Int),
    Irange(Irange),
    Ivar(Ivar),
    Ivasgn(Ivasgn),
    Kwarg(Kwarg),
    KwBegin(KwBegin),
    Kwnilarg(Kwnilarg),
    Kwoptarg(Kwoptarg),
    Kwrestarg(Kwrestarg),
    Kwsplat(Kwsplat),
    Lambda(Lambda),
    Lvar(Lvar),
    Lvasgn(Lvasgn),
    Masgn(Masgn),
    MatchAlt(MatchAlt),
    MatchAs(MatchAs),
    MatchCurrentLine(MatchCurrentLine),
    MatchNilPattern(MatchNilPattern),
    MatchRest(MatchRest),
    MatchVar(MatchVar),
    MatchWithLvasgn(MatchWithLvasgn),
    MatchWithTrailingComma(MatchWithTrailingComma),
    Mlhs(Mlhs),
    Module(Module),
    Next(Next),
    Nil(Nil),
    NthRef(NthRef),
    Numblock(Numblock),
    OpAsgn(OpAsgn),
    Optarg(Optarg),
    Or(Or),
    OrAsgn(OrAsgn),
    Pair(Pair),
    Pin(Pin),
    Postexe(Postexe),
    Preexe(Preexe),
    Procarg0(Procarg0),
    Rational(Rational),
    Redo(Redo),
    Regexp(Regexp),
    RegOpt(RegOpt),
    Rescue(Rescue),
    RescueBody(RescueBody),
    Restarg(Restarg),
    Retry(Retry),
    Return(Return),
    SClass(SClass),
    Self_(Self_),
    Send(Send),
    Shadowarg(Shadowarg),
    Splat(Splat),
    Str(Str),
    Super(Super),
    Sym(Sym),
    True(True),
    Undef(Undef),
    UnlessGuard(UnlessGuard),
    Until(Until),
    UntilPost(UntilPost),
    When(When),
    While(While),
    WhilePost(WhilePost),
    XHeredoc(XHeredoc),
    Xstr(Xstr),
    Yield(Yield),
    ZSuper(ZSuper),
}

impl Node {
    pub fn inner(&self) -> &dyn InnerNode {
        match self {
            Node::Encoding(inner) => inner,
            Node::File(inner) => inner,
            Node::Line(inner) => inner,
            Node::Alias(inner) => inner,
            Node::And(inner) => inner,
            Node::AndAsgn(inner) => inner,
            Node::Arg(inner) => inner,
            Node::Args(inner) => inner,
            Node::Array(inner) => inner,
            Node::ArrayPattern(inner) => inner,
            Node::ArrayPatternWithTail(inner) => inner,
            Node::BackRef(inner) => inner,
            Node::Begin(inner) => inner,
            Node::Block(inner) => inner,
            Node::Blockarg(inner) => inner,
            Node::BlockPass(inner) => inner,
            Node::Break(inner) => inner,
            Node::Case(inner) => inner,
            Node::CaseMatch(inner) => inner,
            Node::Casgn(inner) => inner,
            Node::Cbase(inner) => inner,
            Node::Class(inner) => inner,
            Node::Complex(inner) => inner,
            Node::Const(inner) => inner,
            Node::ConstPattern(inner) => inner,
            Node::CSend(inner) => inner,
            Node::Cvar(inner) => inner,
            Node::Cvasgn(inner) => inner,
            Node::Def(inner) => inner,
            Node::Defined(inner) => inner,
            Node::Defs(inner) => inner,
            Node::Dstr(inner) => inner,
            Node::Dsym(inner) => inner,
            Node::EFlipFlop(inner) => inner,
            Node::EmptyElse(inner) => inner,
            Node::Ensure(inner) => inner,
            Node::Erange(inner) => inner,
            Node::False(inner) => inner,
            Node::FindPattern(inner) => inner,
            Node::Float(inner) => inner,
            Node::For(inner) => inner,
            Node::ForwardArg(inner) => inner,
            Node::ForwardedArgs(inner) => inner,
            Node::Gvar(inner) => inner,
            Node::Gvasgn(inner) => inner,
            Node::Hash(inner) => inner,
            Node::HashPattern(inner) => inner,
            Node::Heredoc(inner) => inner,
            Node::If(inner) => inner,
            Node::IfGuard(inner) => inner,
            Node::IFlipFlop(inner) => inner,
            Node::IfMod(inner) => inner,
            Node::IfTernary(inner) => inner,
            Node::Index(inner) => inner,
            Node::IndexAsgn(inner) => inner,
            Node::InMatch(inner) => inner,
            Node::InPattern(inner) => inner,
            Node::Int(inner) => inner,
            Node::Irange(inner) => inner,
            Node::Ivar(inner) => inner,
            Node::Ivasgn(inner) => inner,
            Node::Kwarg(inner) => inner,
            Node::KwBegin(inner) => inner,
            Node::Kwnilarg(inner) => inner,
            Node::Kwoptarg(inner) => inner,
            Node::Kwrestarg(inner) => inner,
            Node::Kwsplat(inner) => inner,
            Node::Lambda(inner) => inner,
            Node::Lvar(inner) => inner,
            Node::Lvasgn(inner) => inner,
            Node::Masgn(inner) => inner,
            Node::MatchAlt(inner) => inner,
            Node::MatchAs(inner) => inner,
            Node::MatchCurrentLine(inner) => inner,
            Node::MatchNilPattern(inner) => inner,
            Node::MatchRest(inner) => inner,
            Node::MatchVar(inner) => inner,
            Node::MatchWithLvasgn(inner) => inner,
            Node::MatchWithTrailingComma(inner) => inner,
            Node::Mlhs(inner) => inner,
            Node::Module(inner) => inner,
            Node::Next(inner) => inner,
            Node::Nil(inner) => inner,
            Node::NthRef(inner) => inner,
            Node::Numblock(inner) => inner,
            Node::OpAsgn(inner) => inner,
            Node::Optarg(inner) => inner,
            Node::Or(inner) => inner,
            Node::OrAsgn(inner) => inner,
            Node::Pair(inner) => inner,
            Node::Pin(inner) => inner,
            Node::Postexe(inner) => inner,
            Node::Preexe(inner) => inner,
            Node::Procarg0(inner) => inner,
            Node::Rational(inner) => inner,
            Node::Redo(inner) => inner,
            Node::Regexp(inner) => inner,
            Node::RegOpt(inner) => inner,
            Node::Rescue(inner) => inner,
            Node::RescueBody(inner) => inner,
            Node::Restarg(inner) => inner,
            Node::Retry(inner) => inner,
            Node::Return(inner) => inner,
            Node::SClass(inner) => inner,
            Node::Self_(inner) => inner,
            Node::Send(inner) => inner,
            Node::Shadowarg(inner) => inner,
            Node::Splat(inner) => inner,
            Node::Str(inner) => inner,
            Node::Super(inner) => inner,
            Node::Sym(inner) => inner,
            Node::True(inner) => inner,
            Node::Undef(inner) => inner,
            Node::UnlessGuard(inner) => inner,
            Node::Until(inner) => inner,
            Node::UntilPost(inner) => inner,
            Node::When(inner) => inner,
            Node::While(inner) => inner,
            Node::WhilePost(inner) => inner,
            Node::XHeredoc(inner) => inner,
            Node::Xstr(inner) => inner,
            Node::Yield(inner) => inner,
            Node::ZSuper(inner) => inner,
        }
    }

    pub fn inspect(&self, indent: usize) -> String {
        self.inner().inspect(indent)
    }

    pub fn expression(&self) -> &Range {
        self.inner().expression()
    }
}

pub trait InnerNode {
    fn expression(&self) -> &Range;
    fn str_type(&self) -> &'static str;
    fn inspected_children(&self, indent: usize) -> Vec<String>;

    fn inspect(&self, indent: usize) -> String {
        let indented = "  ".repeat(indent);
        let mut sexp = format!("{}s(:{}", indented, self.str_type());

        for child in self.inspected_children(indent) {
            sexp.push_str(&child);
        }

        sexp.push_str(")");

        sexp
    }
}

pub struct InspectVec {
    indent: usize,
    strings: Vec<String>,
}

impl InspectVec {
    pub fn new(indent: usize) -> Self {
        Self {
            indent,
            strings: vec![],
        }
    }

    pub fn push_str(&mut self, string: &str) {
        self.strings.push(format!(", {:?}", string));
    }

    pub fn push_nil(&mut self) {
        self.strings.push(", nil".to_owned());
    }

    pub fn push_u8(&mut self, n: u8) {
        self.strings.push(format!(", {}", n))
    }

    pub fn push_usize(&mut self, n: usize) {
        self.strings.push(format!(", {}", n))
    }

    pub fn push_node(&mut self, node: &Node) {
        self.strings
            .push(format!(",\n{}", node.inspect(self.indent + 1)))
    }

    pub fn push_maybe_node(&mut self, node: &Option<Box<Node>>) {
        if let Some(node) = node {
            self.push_node(node)
        }
    }

    pub fn push_maybe_node_or_nil(&mut self, node: &Option<Box<Node>>) {
        if let Some(node) = node {
            self.push_node(node)
        } else {
            self.push_nil()
        }
    }

    pub fn push_nodes(&mut self, nodes: &Vec<Node>) {
        for node in nodes {
            self.push_node(node)
        }
    }

    pub fn strings(self) -> Vec<String> {
        self.strings
    }
}
