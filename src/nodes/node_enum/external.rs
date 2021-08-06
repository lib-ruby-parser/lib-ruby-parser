use crate::nodes::InnerNode;
use crate::nodes::*;

/// Generic combination of all known nodes.
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub enum Node {
Alias(Alias),
    AndAsgn(AndAsgn),
    And(And),
    Arg(Arg),
    Args(Args),
    Array(Array),
    ArrayPattern(ArrayPattern),
    ArrayPatternWithTail(ArrayPatternWithTail),
    BackRef(BackRef),
    Begin(Begin),
    Block(Block),
    BlockPass(BlockPass),
    Blockarg(Blockarg),
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
    Encoding(Encoding),
    Ensure(Ensure),
    Erange(Erange),
    False(False),
    File(File),
    FindPattern(FindPattern),
    Float(Float),
    For(For),
    ForwardArg(ForwardArg),
    ForwardedArgs(ForwardedArgs),
    Gvar(Gvar),
    Gvasgn(Gvasgn),
    Hash(Hash),
    Kwargs(Kwargs),
    HashPattern(HashPattern),
    Heredoc(Heredoc),
    If(If),
    IfGuard(IfGuard),
    IfMod(IfMod),
    IfTernary(IfTernary),
    IFlipFlop(IFlipFlop),
    MatchPattern(MatchPattern),
    MatchPatternP(MatchPatternP),
    InPattern(InPattern),
    Index(Index),
    IndexAsgn(IndexAsgn),
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
    Line(Line),
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
    RegOpt(RegOpt),
    Regexp(Regexp),
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
    ZSuper(ZSuper)
}

impl Node {
    pub(crate) fn inner_ref(&self) -> &dyn InnerNode {
        match &self {
            Node::Alias(inner) => inner,
            Node::AndAsgn(inner) => inner,
            Node::And(inner) => inner,
            Node::Arg(inner) => inner,
            Node::Args(inner) => inner,
            Node::Array(inner) => inner,
            Node::ArrayPattern(inner) => inner,
            Node::ArrayPatternWithTail(inner) => inner,
            Node::BackRef(inner) => inner,
            Node::Begin(inner) => inner,
            Node::Block(inner) => inner,
            Node::BlockPass(inner) => inner,
            Node::Blockarg(inner) => inner,
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
            Node::Encoding(inner) => inner,
            Node::Ensure(inner) => inner,
            Node::Erange(inner) => inner,
            Node::False(inner) => inner,
            Node::File(inner) => inner,
            Node::FindPattern(inner) => inner,
            Node::Float(inner) => inner,
            Node::For(inner) => inner,
            Node::ForwardArg(inner) => inner,
            Node::ForwardedArgs(inner) => inner,
            Node::Gvar(inner) => inner,
            Node::Gvasgn(inner) => inner,
            Node::Hash(inner) => inner,
            Node::Kwargs(inner) => inner,
            Node::HashPattern(inner) => inner,
            Node::Heredoc(inner) => inner,
            Node::If(inner) => inner,
            Node::IfGuard(inner) => inner,
            Node::IfMod(inner) => inner,
            Node::IfTernary(inner) => inner,
            Node::IFlipFlop(inner) => inner,
            Node::MatchPattern(inner) => inner,
            Node::MatchPatternP(inner) => inner,
            Node::InPattern(inner) => inner,
            Node::Index(inner) => inner,
            Node::IndexAsgn(inner) => inner,
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
            Node::Line(inner) => inner,
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
            Node::RegOpt(inner) => inner,
            Node::Regexp(inner) => inner,
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

    /// Returns true if `self` is `Node::Alias`
    pub fn is_alias(&self) -> bool { matches!(self, Self::Alias(_)) }
    /// Casts `&Node` to `Option<&nodes::Alias>`
    pub fn as_alias(&self) -> Option<&Alias> {
        if let Self::Alias(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Alias>`
    pub fn as_alias_mut(&mut self) -> Option<&mut Alias> {
        if let Self::Alias(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Alias`, panics if variant doesn't match
    pub fn into_alias(self) -> Alias {
        if let Self::Alias(inner) = self {
            inner
        } else {
            panic!("bug: expected type Alias, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::AndAsgn`
    pub fn is_and_asgn(&self) -> bool { matches!(self, Self::AndAsgn(_)) }
    /// Casts `&Node` to `Option<&nodes::AndAsgn>`
    pub fn as_and_asgn(&self) -> Option<&AndAsgn> {
        if let Self::AndAsgn(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::AndAsgn>`
    pub fn as_and_asgn_mut(&mut self) -> Option<&mut AndAsgn> {
        if let Self::AndAsgn(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::AndAsgn`, panics if variant doesn't match
    pub fn into_and_asgn(self) -> AndAsgn {
        if let Self::AndAsgn(inner) = self {
            inner
        } else {
            panic!("bug: expected type AndAsgn, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::And`
    pub fn is_and(&self) -> bool { matches!(self, Self::And(_)) }
    /// Casts `&Node` to `Option<&nodes::And>`
    pub fn as_and(&self) -> Option<&And> {
        if let Self::And(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::And>`
    pub fn as_and_mut(&mut self) -> Option<&mut And> {
        if let Self::And(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::And`, panics if variant doesn't match
    pub fn into_and(self) -> And {
        if let Self::And(inner) = self {
            inner
        } else {
            panic!("bug: expected type And, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Arg`
    pub fn is_arg(&self) -> bool { matches!(self, Self::Arg(_)) }
    /// Casts `&Node` to `Option<&nodes::Arg>`
    pub fn as_arg(&self) -> Option<&Arg> {
        if let Self::Arg(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Arg>`
    pub fn as_arg_mut(&mut self) -> Option<&mut Arg> {
        if let Self::Arg(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Arg`, panics if variant doesn't match
    pub fn into_arg(self) -> Arg {
        if let Self::Arg(inner) = self {
            inner
        } else {
            panic!("bug: expected type Arg, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Args`
    pub fn is_args(&self) -> bool { matches!(self, Self::Args(_)) }
    /// Casts `&Node` to `Option<&nodes::Args>`
    pub fn as_args(&self) -> Option<&Args> {
        if let Self::Args(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Args>`
    pub fn as_args_mut(&mut self) -> Option<&mut Args> {
        if let Self::Args(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Args`, panics if variant doesn't match
    pub fn into_args(self) -> Args {
        if let Self::Args(inner) = self {
            inner
        } else {
            panic!("bug: expected type Args, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Array`
    pub fn is_array(&self) -> bool { matches!(self, Self::Array(_)) }
    /// Casts `&Node` to `Option<&nodes::Array>`
    pub fn as_array(&self) -> Option<&Array> {
        if let Self::Array(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Array>`
    pub fn as_array_mut(&mut self) -> Option<&mut Array> {
        if let Self::Array(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Array`, panics if variant doesn't match
    pub fn into_array(self) -> Array {
        if let Self::Array(inner) = self {
            inner
        } else {
            panic!("bug: expected type Array, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::ArrayPattern`
    pub fn is_array_pattern(&self) -> bool { matches!(self, Self::ArrayPattern(_)) }
    /// Casts `&Node` to `Option<&nodes::ArrayPattern>`
    pub fn as_array_pattern(&self) -> Option<&ArrayPattern> {
        if let Self::ArrayPattern(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::ArrayPattern>`
    pub fn as_array_pattern_mut(&mut self) -> Option<&mut ArrayPattern> {
        if let Self::ArrayPattern(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::ArrayPattern`, panics if variant doesn't match
    pub fn into_array_pattern(self) -> ArrayPattern {
        if let Self::ArrayPattern(inner) = self {
            inner
        } else {
            panic!("bug: expected type ArrayPattern, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::ArrayPatternWithTail`
    pub fn is_array_pattern_with_tail(&self) -> bool { matches!(self, Self::ArrayPatternWithTail(_)) }
    /// Casts `&Node` to `Option<&nodes::ArrayPatternWithTail>`
    pub fn as_array_pattern_with_tail(&self) -> Option<&ArrayPatternWithTail> {
        if let Self::ArrayPatternWithTail(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::ArrayPatternWithTail>`
    pub fn as_array_pattern_with_tail_mut(&mut self) -> Option<&mut ArrayPatternWithTail> {
        if let Self::ArrayPatternWithTail(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::ArrayPatternWithTail`, panics if variant doesn't match
    pub fn into_array_pattern_with_tail(self) -> ArrayPatternWithTail {
        if let Self::ArrayPatternWithTail(inner) = self {
            inner
        } else {
            panic!("bug: expected type ArrayPatternWithTail, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::BackRef`
    pub fn is_back_ref(&self) -> bool { matches!(self, Self::BackRef(_)) }
    /// Casts `&Node` to `Option<&nodes::BackRef>`
    pub fn as_back_ref(&self) -> Option<&BackRef> {
        if let Self::BackRef(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::BackRef>`
    pub fn as_back_ref_mut(&mut self) -> Option<&mut BackRef> {
        if let Self::BackRef(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::BackRef`, panics if variant doesn't match
    pub fn into_back_ref(self) -> BackRef {
        if let Self::BackRef(inner) = self {
            inner
        } else {
            panic!("bug: expected type BackRef, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Begin`
    pub fn is_begin(&self) -> bool { matches!(self, Self::Begin(_)) }
    /// Casts `&Node` to `Option<&nodes::Begin>`
    pub fn as_begin(&self) -> Option<&Begin> {
        if let Self::Begin(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Begin>`
    pub fn as_begin_mut(&mut self) -> Option<&mut Begin> {
        if let Self::Begin(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Begin`, panics if variant doesn't match
    pub fn into_begin(self) -> Begin {
        if let Self::Begin(inner) = self {
            inner
        } else {
            panic!("bug: expected type Begin, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Block`
    pub fn is_block(&self) -> bool { matches!(self, Self::Block(_)) }
    /// Casts `&Node` to `Option<&nodes::Block>`
    pub fn as_block(&self) -> Option<&Block> {
        if let Self::Block(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Block>`
    pub fn as_block_mut(&mut self) -> Option<&mut Block> {
        if let Self::Block(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Block`, panics if variant doesn't match
    pub fn into_block(self) -> Block {
        if let Self::Block(inner) = self {
            inner
        } else {
            panic!("bug: expected type Block, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::BlockPass`
    pub fn is_block_pass(&self) -> bool { matches!(self, Self::BlockPass(_)) }
    /// Casts `&Node` to `Option<&nodes::BlockPass>`
    pub fn as_block_pass(&self) -> Option<&BlockPass> {
        if let Self::BlockPass(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::BlockPass>`
    pub fn as_block_pass_mut(&mut self) -> Option<&mut BlockPass> {
        if let Self::BlockPass(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::BlockPass`, panics if variant doesn't match
    pub fn into_block_pass(self) -> BlockPass {
        if let Self::BlockPass(inner) = self {
            inner
        } else {
            panic!("bug: expected type BlockPass, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Blockarg`
    pub fn is_blockarg(&self) -> bool { matches!(self, Self::Blockarg(_)) }
    /// Casts `&Node` to `Option<&nodes::Blockarg>`
    pub fn as_blockarg(&self) -> Option<&Blockarg> {
        if let Self::Blockarg(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Blockarg>`
    pub fn as_blockarg_mut(&mut self) -> Option<&mut Blockarg> {
        if let Self::Blockarg(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Blockarg`, panics if variant doesn't match
    pub fn into_blockarg(self) -> Blockarg {
        if let Self::Blockarg(inner) = self {
            inner
        } else {
            panic!("bug: expected type Blockarg, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Break`
    pub fn is_break(&self) -> bool { matches!(self, Self::Break(_)) }
    /// Casts `&Node` to `Option<&nodes::Break>`
    pub fn as_break(&self) -> Option<&Break> {
        if let Self::Break(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Break>`
    pub fn as_break_mut(&mut self) -> Option<&mut Break> {
        if let Self::Break(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Break`, panics if variant doesn't match
    pub fn into_break(self) -> Break {
        if let Self::Break(inner) = self {
            inner
        } else {
            panic!("bug: expected type Break, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Case`
    pub fn is_case(&self) -> bool { matches!(self, Self::Case(_)) }
    /// Casts `&Node` to `Option<&nodes::Case>`
    pub fn as_case(&self) -> Option<&Case> {
        if let Self::Case(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Case>`
    pub fn as_case_mut(&mut self) -> Option<&mut Case> {
        if let Self::Case(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Case`, panics if variant doesn't match
    pub fn into_case(self) -> Case {
        if let Self::Case(inner) = self {
            inner
        } else {
            panic!("bug: expected type Case, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::CaseMatch`
    pub fn is_case_match(&self) -> bool { matches!(self, Self::CaseMatch(_)) }
    /// Casts `&Node` to `Option<&nodes::CaseMatch>`
    pub fn as_case_match(&self) -> Option<&CaseMatch> {
        if let Self::CaseMatch(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::CaseMatch>`
    pub fn as_case_match_mut(&mut self) -> Option<&mut CaseMatch> {
        if let Self::CaseMatch(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::CaseMatch`, panics if variant doesn't match
    pub fn into_case_match(self) -> CaseMatch {
        if let Self::CaseMatch(inner) = self {
            inner
        } else {
            panic!("bug: expected type CaseMatch, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Casgn`
    pub fn is_casgn(&self) -> bool { matches!(self, Self::Casgn(_)) }
    /// Casts `&Node` to `Option<&nodes::Casgn>`
    pub fn as_casgn(&self) -> Option<&Casgn> {
        if let Self::Casgn(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Casgn>`
    pub fn as_casgn_mut(&mut self) -> Option<&mut Casgn> {
        if let Self::Casgn(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Casgn`, panics if variant doesn't match
    pub fn into_casgn(self) -> Casgn {
        if let Self::Casgn(inner) = self {
            inner
        } else {
            panic!("bug: expected type Casgn, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Cbase`
    pub fn is_cbase(&self) -> bool { matches!(self, Self::Cbase(_)) }
    /// Casts `&Node` to `Option<&nodes::Cbase>`
    pub fn as_cbase(&self) -> Option<&Cbase> {
        if let Self::Cbase(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Cbase>`
    pub fn as_cbase_mut(&mut self) -> Option<&mut Cbase> {
        if let Self::Cbase(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Cbase`, panics if variant doesn't match
    pub fn into_cbase(self) -> Cbase {
        if let Self::Cbase(inner) = self {
            inner
        } else {
            panic!("bug: expected type Cbase, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Class`
    pub fn is_class(&self) -> bool { matches!(self, Self::Class(_)) }
    /// Casts `&Node` to `Option<&nodes::Class>`
    pub fn as_class(&self) -> Option<&Class> {
        if let Self::Class(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Class>`
    pub fn as_class_mut(&mut self) -> Option<&mut Class> {
        if let Self::Class(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Class`, panics if variant doesn't match
    pub fn into_class(self) -> Class {
        if let Self::Class(inner) = self {
            inner
        } else {
            panic!("bug: expected type Class, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Complex`
    pub fn is_complex(&self) -> bool { matches!(self, Self::Complex(_)) }
    /// Casts `&Node` to `Option<&nodes::Complex>`
    pub fn as_complex(&self) -> Option<&Complex> {
        if let Self::Complex(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Complex>`
    pub fn as_complex_mut(&mut self) -> Option<&mut Complex> {
        if let Self::Complex(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Complex`, panics if variant doesn't match
    pub fn into_complex(self) -> Complex {
        if let Self::Complex(inner) = self {
            inner
        } else {
            panic!("bug: expected type Complex, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Const`
    pub fn is_const(&self) -> bool { matches!(self, Self::Const(_)) }
    /// Casts `&Node` to `Option<&nodes::Const>`
    pub fn as_const(&self) -> Option<&Const> {
        if let Self::Const(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Const>`
    pub fn as_const_mut(&mut self) -> Option<&mut Const> {
        if let Self::Const(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Const`, panics if variant doesn't match
    pub fn into_const(self) -> Const {
        if let Self::Const(inner) = self {
            inner
        } else {
            panic!("bug: expected type Const, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::ConstPattern`
    pub fn is_const_pattern(&self) -> bool { matches!(self, Self::ConstPattern(_)) }
    /// Casts `&Node` to `Option<&nodes::ConstPattern>`
    pub fn as_const_pattern(&self) -> Option<&ConstPattern> {
        if let Self::ConstPattern(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::ConstPattern>`
    pub fn as_const_pattern_mut(&mut self) -> Option<&mut ConstPattern> {
        if let Self::ConstPattern(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::ConstPattern`, panics if variant doesn't match
    pub fn into_const_pattern(self) -> ConstPattern {
        if let Self::ConstPattern(inner) = self {
            inner
        } else {
            panic!("bug: expected type ConstPattern, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::CSend`
    pub fn is_c_send(&self) -> bool { matches!(self, Self::CSend(_)) }
    /// Casts `&Node` to `Option<&nodes::CSend>`
    pub fn as_c_send(&self) -> Option<&CSend> {
        if let Self::CSend(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::CSend>`
    pub fn as_c_send_mut(&mut self) -> Option<&mut CSend> {
        if let Self::CSend(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::CSend`, panics if variant doesn't match
    pub fn into_c_send(self) -> CSend {
        if let Self::CSend(inner) = self {
            inner
        } else {
            panic!("bug: expected type CSend, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Cvar`
    pub fn is_cvar(&self) -> bool { matches!(self, Self::Cvar(_)) }
    /// Casts `&Node` to `Option<&nodes::Cvar>`
    pub fn as_cvar(&self) -> Option<&Cvar> {
        if let Self::Cvar(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Cvar>`
    pub fn as_cvar_mut(&mut self) -> Option<&mut Cvar> {
        if let Self::Cvar(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Cvar`, panics if variant doesn't match
    pub fn into_cvar(self) -> Cvar {
        if let Self::Cvar(inner) = self {
            inner
        } else {
            panic!("bug: expected type Cvar, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Cvasgn`
    pub fn is_cvasgn(&self) -> bool { matches!(self, Self::Cvasgn(_)) }
    /// Casts `&Node` to `Option<&nodes::Cvasgn>`
    pub fn as_cvasgn(&self) -> Option<&Cvasgn> {
        if let Self::Cvasgn(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Cvasgn>`
    pub fn as_cvasgn_mut(&mut self) -> Option<&mut Cvasgn> {
        if let Self::Cvasgn(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Cvasgn`, panics if variant doesn't match
    pub fn into_cvasgn(self) -> Cvasgn {
        if let Self::Cvasgn(inner) = self {
            inner
        } else {
            panic!("bug: expected type Cvasgn, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Def`
    pub fn is_def(&self) -> bool { matches!(self, Self::Def(_)) }
    /// Casts `&Node` to `Option<&nodes::Def>`
    pub fn as_def(&self) -> Option<&Def> {
        if let Self::Def(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Def>`
    pub fn as_def_mut(&mut self) -> Option<&mut Def> {
        if let Self::Def(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Def`, panics if variant doesn't match
    pub fn into_def(self) -> Def {
        if let Self::Def(inner) = self {
            inner
        } else {
            panic!("bug: expected type Def, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Defined`
    pub fn is_defined(&self) -> bool { matches!(self, Self::Defined(_)) }
    /// Casts `&Node` to `Option<&nodes::Defined>`
    pub fn as_defined(&self) -> Option<&Defined> {
        if let Self::Defined(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Defined>`
    pub fn as_defined_mut(&mut self) -> Option<&mut Defined> {
        if let Self::Defined(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Defined`, panics if variant doesn't match
    pub fn into_defined(self) -> Defined {
        if let Self::Defined(inner) = self {
            inner
        } else {
            panic!("bug: expected type Defined, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Defs`
    pub fn is_defs(&self) -> bool { matches!(self, Self::Defs(_)) }
    /// Casts `&Node` to `Option<&nodes::Defs>`
    pub fn as_defs(&self) -> Option<&Defs> {
        if let Self::Defs(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Defs>`
    pub fn as_defs_mut(&mut self) -> Option<&mut Defs> {
        if let Self::Defs(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Defs`, panics if variant doesn't match
    pub fn into_defs(self) -> Defs {
        if let Self::Defs(inner) = self {
            inner
        } else {
            panic!("bug: expected type Defs, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Dstr`
    pub fn is_dstr(&self) -> bool { matches!(self, Self::Dstr(_)) }
    /// Casts `&Node` to `Option<&nodes::Dstr>`
    pub fn as_dstr(&self) -> Option<&Dstr> {
        if let Self::Dstr(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Dstr>`
    pub fn as_dstr_mut(&mut self) -> Option<&mut Dstr> {
        if let Self::Dstr(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Dstr`, panics if variant doesn't match
    pub fn into_dstr(self) -> Dstr {
        if let Self::Dstr(inner) = self {
            inner
        } else {
            panic!("bug: expected type Dstr, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Dsym`
    pub fn is_dsym(&self) -> bool { matches!(self, Self::Dsym(_)) }
    /// Casts `&Node` to `Option<&nodes::Dsym>`
    pub fn as_dsym(&self) -> Option<&Dsym> {
        if let Self::Dsym(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Dsym>`
    pub fn as_dsym_mut(&mut self) -> Option<&mut Dsym> {
        if let Self::Dsym(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Dsym`, panics if variant doesn't match
    pub fn into_dsym(self) -> Dsym {
        if let Self::Dsym(inner) = self {
            inner
        } else {
            panic!("bug: expected type Dsym, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::EFlipFlop`
    pub fn is_e_flip_flop(&self) -> bool { matches!(self, Self::EFlipFlop(_)) }
    /// Casts `&Node` to `Option<&nodes::EFlipFlop>`
    pub fn as_e_flip_flop(&self) -> Option<&EFlipFlop> {
        if let Self::EFlipFlop(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::EFlipFlop>`
    pub fn as_e_flip_flop_mut(&mut self) -> Option<&mut EFlipFlop> {
        if let Self::EFlipFlop(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::EFlipFlop`, panics if variant doesn't match
    pub fn into_e_flip_flop(self) -> EFlipFlop {
        if let Self::EFlipFlop(inner) = self {
            inner
        } else {
            panic!("bug: expected type EFlipFlop, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::EmptyElse`
    pub fn is_empty_else(&self) -> bool { matches!(self, Self::EmptyElse(_)) }
    /// Casts `&Node` to `Option<&nodes::EmptyElse>`
    pub fn as_empty_else(&self) -> Option<&EmptyElse> {
        if let Self::EmptyElse(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::EmptyElse>`
    pub fn as_empty_else_mut(&mut self) -> Option<&mut EmptyElse> {
        if let Self::EmptyElse(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::EmptyElse`, panics if variant doesn't match
    pub fn into_empty_else(self) -> EmptyElse {
        if let Self::EmptyElse(inner) = self {
            inner
        } else {
            panic!("bug: expected type EmptyElse, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Encoding`
    pub fn is_encoding(&self) -> bool { matches!(self, Self::Encoding(_)) }
    /// Casts `&Node` to `Option<&nodes::Encoding>`
    pub fn as_encoding(&self) -> Option<&Encoding> {
        if let Self::Encoding(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Encoding>`
    pub fn as_encoding_mut(&mut self) -> Option<&mut Encoding> {
        if let Self::Encoding(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Encoding`, panics if variant doesn't match
    pub fn into_encoding(self) -> Encoding {
        if let Self::Encoding(inner) = self {
            inner
        } else {
            panic!("bug: expected type Encoding, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Ensure`
    pub fn is_ensure(&self) -> bool { matches!(self, Self::Ensure(_)) }
    /// Casts `&Node` to `Option<&nodes::Ensure>`
    pub fn as_ensure(&self) -> Option<&Ensure> {
        if let Self::Ensure(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Ensure>`
    pub fn as_ensure_mut(&mut self) -> Option<&mut Ensure> {
        if let Self::Ensure(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Ensure`, panics if variant doesn't match
    pub fn into_ensure(self) -> Ensure {
        if let Self::Ensure(inner) = self {
            inner
        } else {
            panic!("bug: expected type Ensure, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Erange`
    pub fn is_erange(&self) -> bool { matches!(self, Self::Erange(_)) }
    /// Casts `&Node` to `Option<&nodes::Erange>`
    pub fn as_erange(&self) -> Option<&Erange> {
        if let Self::Erange(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Erange>`
    pub fn as_erange_mut(&mut self) -> Option<&mut Erange> {
        if let Self::Erange(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Erange`, panics if variant doesn't match
    pub fn into_erange(self) -> Erange {
        if let Self::Erange(inner) = self {
            inner
        } else {
            panic!("bug: expected type Erange, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::False`
    pub fn is_false(&self) -> bool { matches!(self, Self::False(_)) }
    /// Casts `&Node` to `Option<&nodes::False>`
    pub fn as_false(&self) -> Option<&False> {
        if let Self::False(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::False>`
    pub fn as_false_mut(&mut self) -> Option<&mut False> {
        if let Self::False(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::False`, panics if variant doesn't match
    pub fn into_false(self) -> False {
        if let Self::False(inner) = self {
            inner
        } else {
            panic!("bug: expected type False, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::File`
    pub fn is_file(&self) -> bool { matches!(self, Self::File(_)) }
    /// Casts `&Node` to `Option<&nodes::File>`
    pub fn as_file(&self) -> Option<&File> {
        if let Self::File(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::File>`
    pub fn as_file_mut(&mut self) -> Option<&mut File> {
        if let Self::File(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::File`, panics if variant doesn't match
    pub fn into_file(self) -> File {
        if let Self::File(inner) = self {
            inner
        } else {
            panic!("bug: expected type File, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::FindPattern`
    pub fn is_find_pattern(&self) -> bool { matches!(self, Self::FindPattern(_)) }
    /// Casts `&Node` to `Option<&nodes::FindPattern>`
    pub fn as_find_pattern(&self) -> Option<&FindPattern> {
        if let Self::FindPattern(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::FindPattern>`
    pub fn as_find_pattern_mut(&mut self) -> Option<&mut FindPattern> {
        if let Self::FindPattern(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::FindPattern`, panics if variant doesn't match
    pub fn into_find_pattern(self) -> FindPattern {
        if let Self::FindPattern(inner) = self {
            inner
        } else {
            panic!("bug: expected type FindPattern, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Float`
    pub fn is_float(&self) -> bool { matches!(self, Self::Float(_)) }
    /// Casts `&Node` to `Option<&nodes::Float>`
    pub fn as_float(&self) -> Option<&Float> {
        if let Self::Float(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Float>`
    pub fn as_float_mut(&mut self) -> Option<&mut Float> {
        if let Self::Float(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Float`, panics if variant doesn't match
    pub fn into_float(self) -> Float {
        if let Self::Float(inner) = self {
            inner
        } else {
            panic!("bug: expected type Float, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::For`
    pub fn is_for(&self) -> bool { matches!(self, Self::For(_)) }
    /// Casts `&Node` to `Option<&nodes::For>`
    pub fn as_for(&self) -> Option<&For> {
        if let Self::For(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::For>`
    pub fn as_for_mut(&mut self) -> Option<&mut For> {
        if let Self::For(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::For`, panics if variant doesn't match
    pub fn into_for(self) -> For {
        if let Self::For(inner) = self {
            inner
        } else {
            panic!("bug: expected type For, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::ForwardArg`
    pub fn is_forward_arg(&self) -> bool { matches!(self, Self::ForwardArg(_)) }
    /// Casts `&Node` to `Option<&nodes::ForwardArg>`
    pub fn as_forward_arg(&self) -> Option<&ForwardArg> {
        if let Self::ForwardArg(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::ForwardArg>`
    pub fn as_forward_arg_mut(&mut self) -> Option<&mut ForwardArg> {
        if let Self::ForwardArg(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::ForwardArg`, panics if variant doesn't match
    pub fn into_forward_arg(self) -> ForwardArg {
        if let Self::ForwardArg(inner) = self {
            inner
        } else {
            panic!("bug: expected type ForwardArg, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::ForwardedArgs`
    pub fn is_forwarded_args(&self) -> bool { matches!(self, Self::ForwardedArgs(_)) }
    /// Casts `&Node` to `Option<&nodes::ForwardedArgs>`
    pub fn as_forwarded_args(&self) -> Option<&ForwardedArgs> {
        if let Self::ForwardedArgs(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::ForwardedArgs>`
    pub fn as_forwarded_args_mut(&mut self) -> Option<&mut ForwardedArgs> {
        if let Self::ForwardedArgs(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::ForwardedArgs`, panics if variant doesn't match
    pub fn into_forwarded_args(self) -> ForwardedArgs {
        if let Self::ForwardedArgs(inner) = self {
            inner
        } else {
            panic!("bug: expected type ForwardedArgs, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Gvar`
    pub fn is_gvar(&self) -> bool { matches!(self, Self::Gvar(_)) }
    /// Casts `&Node` to `Option<&nodes::Gvar>`
    pub fn as_gvar(&self) -> Option<&Gvar> {
        if let Self::Gvar(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Gvar>`
    pub fn as_gvar_mut(&mut self) -> Option<&mut Gvar> {
        if let Self::Gvar(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Gvar`, panics if variant doesn't match
    pub fn into_gvar(self) -> Gvar {
        if let Self::Gvar(inner) = self {
            inner
        } else {
            panic!("bug: expected type Gvar, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Gvasgn`
    pub fn is_gvasgn(&self) -> bool { matches!(self, Self::Gvasgn(_)) }
    /// Casts `&Node` to `Option<&nodes::Gvasgn>`
    pub fn as_gvasgn(&self) -> Option<&Gvasgn> {
        if let Self::Gvasgn(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Gvasgn>`
    pub fn as_gvasgn_mut(&mut self) -> Option<&mut Gvasgn> {
        if let Self::Gvasgn(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Gvasgn`, panics if variant doesn't match
    pub fn into_gvasgn(self) -> Gvasgn {
        if let Self::Gvasgn(inner) = self {
            inner
        } else {
            panic!("bug: expected type Gvasgn, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Hash`
    pub fn is_hash(&self) -> bool { matches!(self, Self::Hash(_)) }
    /// Casts `&Node` to `Option<&nodes::Hash>`
    pub fn as_hash(&self) -> Option<&Hash> {
        if let Self::Hash(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Hash>`
    pub fn as_hash_mut(&mut self) -> Option<&mut Hash> {
        if let Self::Hash(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Hash`, panics if variant doesn't match
    pub fn into_hash(self) -> Hash {
        if let Self::Hash(inner) = self {
            inner
        } else {
            panic!("bug: expected type Hash, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Kwargs`
    pub fn is_kwargs(&self) -> bool { matches!(self, Self::Kwargs(_)) }
    /// Casts `&Node` to `Option<&nodes::Kwargs>`
    pub fn as_kwargs(&self) -> Option<&Kwargs> {
        if let Self::Kwargs(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Kwargs>`
    pub fn as_kwargs_mut(&mut self) -> Option<&mut Kwargs> {
        if let Self::Kwargs(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Kwargs`, panics if variant doesn't match
    pub fn into_kwargs(self) -> Kwargs {
        if let Self::Kwargs(inner) = self {
            inner
        } else {
            panic!("bug: expected type Kwargs, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::HashPattern`
    pub fn is_hash_pattern(&self) -> bool { matches!(self, Self::HashPattern(_)) }
    /// Casts `&Node` to `Option<&nodes::HashPattern>`
    pub fn as_hash_pattern(&self) -> Option<&HashPattern> {
        if let Self::HashPattern(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::HashPattern>`
    pub fn as_hash_pattern_mut(&mut self) -> Option<&mut HashPattern> {
        if let Self::HashPattern(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::HashPattern`, panics if variant doesn't match
    pub fn into_hash_pattern(self) -> HashPattern {
        if let Self::HashPattern(inner) = self {
            inner
        } else {
            panic!("bug: expected type HashPattern, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Heredoc`
    pub fn is_heredoc(&self) -> bool { matches!(self, Self::Heredoc(_)) }
    /// Casts `&Node` to `Option<&nodes::Heredoc>`
    pub fn as_heredoc(&self) -> Option<&Heredoc> {
        if let Self::Heredoc(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Heredoc>`
    pub fn as_heredoc_mut(&mut self) -> Option<&mut Heredoc> {
        if let Self::Heredoc(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Heredoc`, panics if variant doesn't match
    pub fn into_heredoc(self) -> Heredoc {
        if let Self::Heredoc(inner) = self {
            inner
        } else {
            panic!("bug: expected type Heredoc, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::If`
    pub fn is_if(&self) -> bool { matches!(self, Self::If(_)) }
    /// Casts `&Node` to `Option<&nodes::If>`
    pub fn as_if(&self) -> Option<&If> {
        if let Self::If(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::If>`
    pub fn as_if_mut(&mut self) -> Option<&mut If> {
        if let Self::If(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::If`, panics if variant doesn't match
    pub fn into_if(self) -> If {
        if let Self::If(inner) = self {
            inner
        } else {
            panic!("bug: expected type If, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::IfGuard`
    pub fn is_if_guard(&self) -> bool { matches!(self, Self::IfGuard(_)) }
    /// Casts `&Node` to `Option<&nodes::IfGuard>`
    pub fn as_if_guard(&self) -> Option<&IfGuard> {
        if let Self::IfGuard(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::IfGuard>`
    pub fn as_if_guard_mut(&mut self) -> Option<&mut IfGuard> {
        if let Self::IfGuard(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::IfGuard`, panics if variant doesn't match
    pub fn into_if_guard(self) -> IfGuard {
        if let Self::IfGuard(inner) = self {
            inner
        } else {
            panic!("bug: expected type IfGuard, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::IfMod`
    pub fn is_if_mod(&self) -> bool { matches!(self, Self::IfMod(_)) }
    /// Casts `&Node` to `Option<&nodes::IfMod>`
    pub fn as_if_mod(&self) -> Option<&IfMod> {
        if let Self::IfMod(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::IfMod>`
    pub fn as_if_mod_mut(&mut self) -> Option<&mut IfMod> {
        if let Self::IfMod(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::IfMod`, panics if variant doesn't match
    pub fn into_if_mod(self) -> IfMod {
        if let Self::IfMod(inner) = self {
            inner
        } else {
            panic!("bug: expected type IfMod, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::IfTernary`
    pub fn is_if_ternary(&self) -> bool { matches!(self, Self::IfTernary(_)) }
    /// Casts `&Node` to `Option<&nodes::IfTernary>`
    pub fn as_if_ternary(&self) -> Option<&IfTernary> {
        if let Self::IfTernary(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::IfTernary>`
    pub fn as_if_ternary_mut(&mut self) -> Option<&mut IfTernary> {
        if let Self::IfTernary(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::IfTernary`, panics if variant doesn't match
    pub fn into_if_ternary(self) -> IfTernary {
        if let Self::IfTernary(inner) = self {
            inner
        } else {
            panic!("bug: expected type IfTernary, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::IFlipFlop`
    pub fn is_i_flip_flop(&self) -> bool { matches!(self, Self::IFlipFlop(_)) }
    /// Casts `&Node` to `Option<&nodes::IFlipFlop>`
    pub fn as_i_flip_flop(&self) -> Option<&IFlipFlop> {
        if let Self::IFlipFlop(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::IFlipFlop>`
    pub fn as_i_flip_flop_mut(&mut self) -> Option<&mut IFlipFlop> {
        if let Self::IFlipFlop(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::IFlipFlop`, panics if variant doesn't match
    pub fn into_i_flip_flop(self) -> IFlipFlop {
        if let Self::IFlipFlop(inner) = self {
            inner
        } else {
            panic!("bug: expected type IFlipFlop, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::MatchPattern`
    pub fn is_match_pattern(&self) -> bool { matches!(self, Self::MatchPattern(_)) }
    /// Casts `&Node` to `Option<&nodes::MatchPattern>`
    pub fn as_match_pattern(&self) -> Option<&MatchPattern> {
        if let Self::MatchPattern(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::MatchPattern>`
    pub fn as_match_pattern_mut(&mut self) -> Option<&mut MatchPattern> {
        if let Self::MatchPattern(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::MatchPattern`, panics if variant doesn't match
    pub fn into_match_pattern(self) -> MatchPattern {
        if let Self::MatchPattern(inner) = self {
            inner
        } else {
            panic!("bug: expected type MatchPattern, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::MatchPatternP`
    pub fn is_match_pattern_p(&self) -> bool { matches!(self, Self::MatchPatternP(_)) }
    /// Casts `&Node` to `Option<&nodes::MatchPatternP>`
    pub fn as_match_pattern_p(&self) -> Option<&MatchPatternP> {
        if let Self::MatchPatternP(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::MatchPatternP>`
    pub fn as_match_pattern_p_mut(&mut self) -> Option<&mut MatchPatternP> {
        if let Self::MatchPatternP(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::MatchPatternP`, panics if variant doesn't match
    pub fn into_match_pattern_p(self) -> MatchPatternP {
        if let Self::MatchPatternP(inner) = self {
            inner
        } else {
            panic!("bug: expected type MatchPatternP, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::InPattern`
    pub fn is_in_pattern(&self) -> bool { matches!(self, Self::InPattern(_)) }
    /// Casts `&Node` to `Option<&nodes::InPattern>`
    pub fn as_in_pattern(&self) -> Option<&InPattern> {
        if let Self::InPattern(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::InPattern>`
    pub fn as_in_pattern_mut(&mut self) -> Option<&mut InPattern> {
        if let Self::InPattern(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::InPattern`, panics if variant doesn't match
    pub fn into_in_pattern(self) -> InPattern {
        if let Self::InPattern(inner) = self {
            inner
        } else {
            panic!("bug: expected type InPattern, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Index`
    pub fn is_index(&self) -> bool { matches!(self, Self::Index(_)) }
    /// Casts `&Node` to `Option<&nodes::Index>`
    pub fn as_index(&self) -> Option<&Index> {
        if let Self::Index(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Index>`
    pub fn as_index_mut(&mut self) -> Option<&mut Index> {
        if let Self::Index(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Index`, panics if variant doesn't match
    pub fn into_index(self) -> Index {
        if let Self::Index(inner) = self {
            inner
        } else {
            panic!("bug: expected type Index, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::IndexAsgn`
    pub fn is_index_asgn(&self) -> bool { matches!(self, Self::IndexAsgn(_)) }
    /// Casts `&Node` to `Option<&nodes::IndexAsgn>`
    pub fn as_index_asgn(&self) -> Option<&IndexAsgn> {
        if let Self::IndexAsgn(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::IndexAsgn>`
    pub fn as_index_asgn_mut(&mut self) -> Option<&mut IndexAsgn> {
        if let Self::IndexAsgn(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::IndexAsgn`, panics if variant doesn't match
    pub fn into_index_asgn(self) -> IndexAsgn {
        if let Self::IndexAsgn(inner) = self {
            inner
        } else {
            panic!("bug: expected type IndexAsgn, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Int`
    pub fn is_int(&self) -> bool { matches!(self, Self::Int(_)) }
    /// Casts `&Node` to `Option<&nodes::Int>`
    pub fn as_int(&self) -> Option<&Int> {
        if let Self::Int(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Int>`
    pub fn as_int_mut(&mut self) -> Option<&mut Int> {
        if let Self::Int(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Int`, panics if variant doesn't match
    pub fn into_int(self) -> Int {
        if let Self::Int(inner) = self {
            inner
        } else {
            panic!("bug: expected type Int, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Irange`
    pub fn is_irange(&self) -> bool { matches!(self, Self::Irange(_)) }
    /// Casts `&Node` to `Option<&nodes::Irange>`
    pub fn as_irange(&self) -> Option<&Irange> {
        if let Self::Irange(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Irange>`
    pub fn as_irange_mut(&mut self) -> Option<&mut Irange> {
        if let Self::Irange(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Irange`, panics if variant doesn't match
    pub fn into_irange(self) -> Irange {
        if let Self::Irange(inner) = self {
            inner
        } else {
            panic!("bug: expected type Irange, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Ivar`
    pub fn is_ivar(&self) -> bool { matches!(self, Self::Ivar(_)) }
    /// Casts `&Node` to `Option<&nodes::Ivar>`
    pub fn as_ivar(&self) -> Option<&Ivar> {
        if let Self::Ivar(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Ivar>`
    pub fn as_ivar_mut(&mut self) -> Option<&mut Ivar> {
        if let Self::Ivar(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Ivar`, panics if variant doesn't match
    pub fn into_ivar(self) -> Ivar {
        if let Self::Ivar(inner) = self {
            inner
        } else {
            panic!("bug: expected type Ivar, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Ivasgn`
    pub fn is_ivasgn(&self) -> bool { matches!(self, Self::Ivasgn(_)) }
    /// Casts `&Node` to `Option<&nodes::Ivasgn>`
    pub fn as_ivasgn(&self) -> Option<&Ivasgn> {
        if let Self::Ivasgn(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Ivasgn>`
    pub fn as_ivasgn_mut(&mut self) -> Option<&mut Ivasgn> {
        if let Self::Ivasgn(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Ivasgn`, panics if variant doesn't match
    pub fn into_ivasgn(self) -> Ivasgn {
        if let Self::Ivasgn(inner) = self {
            inner
        } else {
            panic!("bug: expected type Ivasgn, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Kwarg`
    pub fn is_kwarg(&self) -> bool { matches!(self, Self::Kwarg(_)) }
    /// Casts `&Node` to `Option<&nodes::Kwarg>`
    pub fn as_kwarg(&self) -> Option<&Kwarg> {
        if let Self::Kwarg(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Kwarg>`
    pub fn as_kwarg_mut(&mut self) -> Option<&mut Kwarg> {
        if let Self::Kwarg(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Kwarg`, panics if variant doesn't match
    pub fn into_kwarg(self) -> Kwarg {
        if let Self::Kwarg(inner) = self {
            inner
        } else {
            panic!("bug: expected type Kwarg, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::KwBegin`
    pub fn is_kw_begin(&self) -> bool { matches!(self, Self::KwBegin(_)) }
    /// Casts `&Node` to `Option<&nodes::KwBegin>`
    pub fn as_kw_begin(&self) -> Option<&KwBegin> {
        if let Self::KwBegin(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::KwBegin>`
    pub fn as_kw_begin_mut(&mut self) -> Option<&mut KwBegin> {
        if let Self::KwBegin(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::KwBegin`, panics if variant doesn't match
    pub fn into_kw_begin(self) -> KwBegin {
        if let Self::KwBegin(inner) = self {
            inner
        } else {
            panic!("bug: expected type KwBegin, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Kwnilarg`
    pub fn is_kwnilarg(&self) -> bool { matches!(self, Self::Kwnilarg(_)) }
    /// Casts `&Node` to `Option<&nodes::Kwnilarg>`
    pub fn as_kwnilarg(&self) -> Option<&Kwnilarg> {
        if let Self::Kwnilarg(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Kwnilarg>`
    pub fn as_kwnilarg_mut(&mut self) -> Option<&mut Kwnilarg> {
        if let Self::Kwnilarg(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Kwnilarg`, panics if variant doesn't match
    pub fn into_kwnilarg(self) -> Kwnilarg {
        if let Self::Kwnilarg(inner) = self {
            inner
        } else {
            panic!("bug: expected type Kwnilarg, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Kwoptarg`
    pub fn is_kwoptarg(&self) -> bool { matches!(self, Self::Kwoptarg(_)) }
    /// Casts `&Node` to `Option<&nodes::Kwoptarg>`
    pub fn as_kwoptarg(&self) -> Option<&Kwoptarg> {
        if let Self::Kwoptarg(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Kwoptarg>`
    pub fn as_kwoptarg_mut(&mut self) -> Option<&mut Kwoptarg> {
        if let Self::Kwoptarg(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Kwoptarg`, panics if variant doesn't match
    pub fn into_kwoptarg(self) -> Kwoptarg {
        if let Self::Kwoptarg(inner) = self {
            inner
        } else {
            panic!("bug: expected type Kwoptarg, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Kwrestarg`
    pub fn is_kwrestarg(&self) -> bool { matches!(self, Self::Kwrestarg(_)) }
    /// Casts `&Node` to `Option<&nodes::Kwrestarg>`
    pub fn as_kwrestarg(&self) -> Option<&Kwrestarg> {
        if let Self::Kwrestarg(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Kwrestarg>`
    pub fn as_kwrestarg_mut(&mut self) -> Option<&mut Kwrestarg> {
        if let Self::Kwrestarg(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Kwrestarg`, panics if variant doesn't match
    pub fn into_kwrestarg(self) -> Kwrestarg {
        if let Self::Kwrestarg(inner) = self {
            inner
        } else {
            panic!("bug: expected type Kwrestarg, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Kwsplat`
    pub fn is_kwsplat(&self) -> bool { matches!(self, Self::Kwsplat(_)) }
    /// Casts `&Node` to `Option<&nodes::Kwsplat>`
    pub fn as_kwsplat(&self) -> Option<&Kwsplat> {
        if let Self::Kwsplat(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Kwsplat>`
    pub fn as_kwsplat_mut(&mut self) -> Option<&mut Kwsplat> {
        if let Self::Kwsplat(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Kwsplat`, panics if variant doesn't match
    pub fn into_kwsplat(self) -> Kwsplat {
        if let Self::Kwsplat(inner) = self {
            inner
        } else {
            panic!("bug: expected type Kwsplat, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Lambda`
    pub fn is_lambda(&self) -> bool { matches!(self, Self::Lambda(_)) }
    /// Casts `&Node` to `Option<&nodes::Lambda>`
    pub fn as_lambda(&self) -> Option<&Lambda> {
        if let Self::Lambda(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Lambda>`
    pub fn as_lambda_mut(&mut self) -> Option<&mut Lambda> {
        if let Self::Lambda(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Lambda`, panics if variant doesn't match
    pub fn into_lambda(self) -> Lambda {
        if let Self::Lambda(inner) = self {
            inner
        } else {
            panic!("bug: expected type Lambda, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Line`
    pub fn is_line(&self) -> bool { matches!(self, Self::Line(_)) }
    /// Casts `&Node` to `Option<&nodes::Line>`
    pub fn as_line(&self) -> Option<&Line> {
        if let Self::Line(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Line>`
    pub fn as_line_mut(&mut self) -> Option<&mut Line> {
        if let Self::Line(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Line`, panics if variant doesn't match
    pub fn into_line(self) -> Line {
        if let Self::Line(inner) = self {
            inner
        } else {
            panic!("bug: expected type Line, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Lvar`
    pub fn is_lvar(&self) -> bool { matches!(self, Self::Lvar(_)) }
    /// Casts `&Node` to `Option<&nodes::Lvar>`
    pub fn as_lvar(&self) -> Option<&Lvar> {
        if let Self::Lvar(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Lvar>`
    pub fn as_lvar_mut(&mut self) -> Option<&mut Lvar> {
        if let Self::Lvar(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Lvar`, panics if variant doesn't match
    pub fn into_lvar(self) -> Lvar {
        if let Self::Lvar(inner) = self {
            inner
        } else {
            panic!("bug: expected type Lvar, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Lvasgn`
    pub fn is_lvasgn(&self) -> bool { matches!(self, Self::Lvasgn(_)) }
    /// Casts `&Node` to `Option<&nodes::Lvasgn>`
    pub fn as_lvasgn(&self) -> Option<&Lvasgn> {
        if let Self::Lvasgn(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Lvasgn>`
    pub fn as_lvasgn_mut(&mut self) -> Option<&mut Lvasgn> {
        if let Self::Lvasgn(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Lvasgn`, panics if variant doesn't match
    pub fn into_lvasgn(self) -> Lvasgn {
        if let Self::Lvasgn(inner) = self {
            inner
        } else {
            panic!("bug: expected type Lvasgn, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Masgn`
    pub fn is_masgn(&self) -> bool { matches!(self, Self::Masgn(_)) }
    /// Casts `&Node` to `Option<&nodes::Masgn>`
    pub fn as_masgn(&self) -> Option<&Masgn> {
        if let Self::Masgn(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Masgn>`
    pub fn as_masgn_mut(&mut self) -> Option<&mut Masgn> {
        if let Self::Masgn(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Masgn`, panics if variant doesn't match
    pub fn into_masgn(self) -> Masgn {
        if let Self::Masgn(inner) = self {
            inner
        } else {
            panic!("bug: expected type Masgn, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::MatchAlt`
    pub fn is_match_alt(&self) -> bool { matches!(self, Self::MatchAlt(_)) }
    /// Casts `&Node` to `Option<&nodes::MatchAlt>`
    pub fn as_match_alt(&self) -> Option<&MatchAlt> {
        if let Self::MatchAlt(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::MatchAlt>`
    pub fn as_match_alt_mut(&mut self) -> Option<&mut MatchAlt> {
        if let Self::MatchAlt(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::MatchAlt`, panics if variant doesn't match
    pub fn into_match_alt(self) -> MatchAlt {
        if let Self::MatchAlt(inner) = self {
            inner
        } else {
            panic!("bug: expected type MatchAlt, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::MatchAs`
    pub fn is_match_as(&self) -> bool { matches!(self, Self::MatchAs(_)) }
    /// Casts `&Node` to `Option<&nodes::MatchAs>`
    pub fn as_match_as(&self) -> Option<&MatchAs> {
        if let Self::MatchAs(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::MatchAs>`
    pub fn as_match_as_mut(&mut self) -> Option<&mut MatchAs> {
        if let Self::MatchAs(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::MatchAs`, panics if variant doesn't match
    pub fn into_match_as(self) -> MatchAs {
        if let Self::MatchAs(inner) = self {
            inner
        } else {
            panic!("bug: expected type MatchAs, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::MatchCurrentLine`
    pub fn is_match_current_line(&self) -> bool { matches!(self, Self::MatchCurrentLine(_)) }
    /// Casts `&Node` to `Option<&nodes::MatchCurrentLine>`
    pub fn as_match_current_line(&self) -> Option<&MatchCurrentLine> {
        if let Self::MatchCurrentLine(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::MatchCurrentLine>`
    pub fn as_match_current_line_mut(&mut self) -> Option<&mut MatchCurrentLine> {
        if let Self::MatchCurrentLine(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::MatchCurrentLine`, panics if variant doesn't match
    pub fn into_match_current_line(self) -> MatchCurrentLine {
        if let Self::MatchCurrentLine(inner) = self {
            inner
        } else {
            panic!("bug: expected type MatchCurrentLine, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::MatchNilPattern`
    pub fn is_match_nil_pattern(&self) -> bool { matches!(self, Self::MatchNilPattern(_)) }
    /// Casts `&Node` to `Option<&nodes::MatchNilPattern>`
    pub fn as_match_nil_pattern(&self) -> Option<&MatchNilPattern> {
        if let Self::MatchNilPattern(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::MatchNilPattern>`
    pub fn as_match_nil_pattern_mut(&mut self) -> Option<&mut MatchNilPattern> {
        if let Self::MatchNilPattern(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::MatchNilPattern`, panics if variant doesn't match
    pub fn into_match_nil_pattern(self) -> MatchNilPattern {
        if let Self::MatchNilPattern(inner) = self {
            inner
        } else {
            panic!("bug: expected type MatchNilPattern, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::MatchRest`
    pub fn is_match_rest(&self) -> bool { matches!(self, Self::MatchRest(_)) }
    /// Casts `&Node` to `Option<&nodes::MatchRest>`
    pub fn as_match_rest(&self) -> Option<&MatchRest> {
        if let Self::MatchRest(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::MatchRest>`
    pub fn as_match_rest_mut(&mut self) -> Option<&mut MatchRest> {
        if let Self::MatchRest(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::MatchRest`, panics if variant doesn't match
    pub fn into_match_rest(self) -> MatchRest {
        if let Self::MatchRest(inner) = self {
            inner
        } else {
            panic!("bug: expected type MatchRest, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::MatchVar`
    pub fn is_match_var(&self) -> bool { matches!(self, Self::MatchVar(_)) }
    /// Casts `&Node` to `Option<&nodes::MatchVar>`
    pub fn as_match_var(&self) -> Option<&MatchVar> {
        if let Self::MatchVar(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::MatchVar>`
    pub fn as_match_var_mut(&mut self) -> Option<&mut MatchVar> {
        if let Self::MatchVar(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::MatchVar`, panics if variant doesn't match
    pub fn into_match_var(self) -> MatchVar {
        if let Self::MatchVar(inner) = self {
            inner
        } else {
            panic!("bug: expected type MatchVar, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::MatchWithLvasgn`
    pub fn is_match_with_lvasgn(&self) -> bool { matches!(self, Self::MatchWithLvasgn(_)) }
    /// Casts `&Node` to `Option<&nodes::MatchWithLvasgn>`
    pub fn as_match_with_lvasgn(&self) -> Option<&MatchWithLvasgn> {
        if let Self::MatchWithLvasgn(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::MatchWithLvasgn>`
    pub fn as_match_with_lvasgn_mut(&mut self) -> Option<&mut MatchWithLvasgn> {
        if let Self::MatchWithLvasgn(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::MatchWithLvasgn`, panics if variant doesn't match
    pub fn into_match_with_lvasgn(self) -> MatchWithLvasgn {
        if let Self::MatchWithLvasgn(inner) = self {
            inner
        } else {
            panic!("bug: expected type MatchWithLvasgn, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Mlhs`
    pub fn is_mlhs(&self) -> bool { matches!(self, Self::Mlhs(_)) }
    /// Casts `&Node` to `Option<&nodes::Mlhs>`
    pub fn as_mlhs(&self) -> Option<&Mlhs> {
        if let Self::Mlhs(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Mlhs>`
    pub fn as_mlhs_mut(&mut self) -> Option<&mut Mlhs> {
        if let Self::Mlhs(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Mlhs`, panics if variant doesn't match
    pub fn into_mlhs(self) -> Mlhs {
        if let Self::Mlhs(inner) = self {
            inner
        } else {
            panic!("bug: expected type Mlhs, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Module`
    pub fn is_module(&self) -> bool { matches!(self, Self::Module(_)) }
    /// Casts `&Node` to `Option<&nodes::Module>`
    pub fn as_module(&self) -> Option<&Module> {
        if let Self::Module(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Module>`
    pub fn as_module_mut(&mut self) -> Option<&mut Module> {
        if let Self::Module(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Module`, panics if variant doesn't match
    pub fn into_module(self) -> Module {
        if let Self::Module(inner) = self {
            inner
        } else {
            panic!("bug: expected type Module, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Next`
    pub fn is_next(&self) -> bool { matches!(self, Self::Next(_)) }
    /// Casts `&Node` to `Option<&nodes::Next>`
    pub fn as_next(&self) -> Option<&Next> {
        if let Self::Next(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Next>`
    pub fn as_next_mut(&mut self) -> Option<&mut Next> {
        if let Self::Next(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Next`, panics if variant doesn't match
    pub fn into_next(self) -> Next {
        if let Self::Next(inner) = self {
            inner
        } else {
            panic!("bug: expected type Next, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Nil`
    pub fn is_nil(&self) -> bool { matches!(self, Self::Nil(_)) }
    /// Casts `&Node` to `Option<&nodes::Nil>`
    pub fn as_nil(&self) -> Option<&Nil> {
        if let Self::Nil(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Nil>`
    pub fn as_nil_mut(&mut self) -> Option<&mut Nil> {
        if let Self::Nil(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Nil`, panics if variant doesn't match
    pub fn into_nil(self) -> Nil {
        if let Self::Nil(inner) = self {
            inner
        } else {
            panic!("bug: expected type Nil, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::NthRef`
    pub fn is_nth_ref(&self) -> bool { matches!(self, Self::NthRef(_)) }
    /// Casts `&Node` to `Option<&nodes::NthRef>`
    pub fn as_nth_ref(&self) -> Option<&NthRef> {
        if let Self::NthRef(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::NthRef>`
    pub fn as_nth_ref_mut(&mut self) -> Option<&mut NthRef> {
        if let Self::NthRef(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::NthRef`, panics if variant doesn't match
    pub fn into_nth_ref(self) -> NthRef {
        if let Self::NthRef(inner) = self {
            inner
        } else {
            panic!("bug: expected type NthRef, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Numblock`
    pub fn is_numblock(&self) -> bool { matches!(self, Self::Numblock(_)) }
    /// Casts `&Node` to `Option<&nodes::Numblock>`
    pub fn as_numblock(&self) -> Option<&Numblock> {
        if let Self::Numblock(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Numblock>`
    pub fn as_numblock_mut(&mut self) -> Option<&mut Numblock> {
        if let Self::Numblock(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Numblock`, panics if variant doesn't match
    pub fn into_numblock(self) -> Numblock {
        if let Self::Numblock(inner) = self {
            inner
        } else {
            panic!("bug: expected type Numblock, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::OpAsgn`
    pub fn is_op_asgn(&self) -> bool { matches!(self, Self::OpAsgn(_)) }
    /// Casts `&Node` to `Option<&nodes::OpAsgn>`
    pub fn as_op_asgn(&self) -> Option<&OpAsgn> {
        if let Self::OpAsgn(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::OpAsgn>`
    pub fn as_op_asgn_mut(&mut self) -> Option<&mut OpAsgn> {
        if let Self::OpAsgn(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::OpAsgn`, panics if variant doesn't match
    pub fn into_op_asgn(self) -> OpAsgn {
        if let Self::OpAsgn(inner) = self {
            inner
        } else {
            panic!("bug: expected type OpAsgn, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Optarg`
    pub fn is_optarg(&self) -> bool { matches!(self, Self::Optarg(_)) }
    /// Casts `&Node` to `Option<&nodes::Optarg>`
    pub fn as_optarg(&self) -> Option<&Optarg> {
        if let Self::Optarg(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Optarg>`
    pub fn as_optarg_mut(&mut self) -> Option<&mut Optarg> {
        if let Self::Optarg(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Optarg`, panics if variant doesn't match
    pub fn into_optarg(self) -> Optarg {
        if let Self::Optarg(inner) = self {
            inner
        } else {
            panic!("bug: expected type Optarg, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Or`
    pub fn is_or(&self) -> bool { matches!(self, Self::Or(_)) }
    /// Casts `&Node` to `Option<&nodes::Or>`
    pub fn as_or(&self) -> Option<&Or> {
        if let Self::Or(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Or>`
    pub fn as_or_mut(&mut self) -> Option<&mut Or> {
        if let Self::Or(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Or`, panics if variant doesn't match
    pub fn into_or(self) -> Or {
        if let Self::Or(inner) = self {
            inner
        } else {
            panic!("bug: expected type Or, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::OrAsgn`
    pub fn is_or_asgn(&self) -> bool { matches!(self, Self::OrAsgn(_)) }
    /// Casts `&Node` to `Option<&nodes::OrAsgn>`
    pub fn as_or_asgn(&self) -> Option<&OrAsgn> {
        if let Self::OrAsgn(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::OrAsgn>`
    pub fn as_or_asgn_mut(&mut self) -> Option<&mut OrAsgn> {
        if let Self::OrAsgn(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::OrAsgn`, panics if variant doesn't match
    pub fn into_or_asgn(self) -> OrAsgn {
        if let Self::OrAsgn(inner) = self {
            inner
        } else {
            panic!("bug: expected type OrAsgn, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Pair`
    pub fn is_pair(&self) -> bool { matches!(self, Self::Pair(_)) }
    /// Casts `&Node` to `Option<&nodes::Pair>`
    pub fn as_pair(&self) -> Option<&Pair> {
        if let Self::Pair(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Pair>`
    pub fn as_pair_mut(&mut self) -> Option<&mut Pair> {
        if let Self::Pair(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Pair`, panics if variant doesn't match
    pub fn into_pair(self) -> Pair {
        if let Self::Pair(inner) = self {
            inner
        } else {
            panic!("bug: expected type Pair, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Pin`
    pub fn is_pin(&self) -> bool { matches!(self, Self::Pin(_)) }
    /// Casts `&Node` to `Option<&nodes::Pin>`
    pub fn as_pin(&self) -> Option<&Pin> {
        if let Self::Pin(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Pin>`
    pub fn as_pin_mut(&mut self) -> Option<&mut Pin> {
        if let Self::Pin(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Pin`, panics if variant doesn't match
    pub fn into_pin(self) -> Pin {
        if let Self::Pin(inner) = self {
            inner
        } else {
            panic!("bug: expected type Pin, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Postexe`
    pub fn is_postexe(&self) -> bool { matches!(self, Self::Postexe(_)) }
    /// Casts `&Node` to `Option<&nodes::Postexe>`
    pub fn as_postexe(&self) -> Option<&Postexe> {
        if let Self::Postexe(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Postexe>`
    pub fn as_postexe_mut(&mut self) -> Option<&mut Postexe> {
        if let Self::Postexe(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Postexe`, panics if variant doesn't match
    pub fn into_postexe(self) -> Postexe {
        if let Self::Postexe(inner) = self {
            inner
        } else {
            panic!("bug: expected type Postexe, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Preexe`
    pub fn is_preexe(&self) -> bool { matches!(self, Self::Preexe(_)) }
    /// Casts `&Node` to `Option<&nodes::Preexe>`
    pub fn as_preexe(&self) -> Option<&Preexe> {
        if let Self::Preexe(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Preexe>`
    pub fn as_preexe_mut(&mut self) -> Option<&mut Preexe> {
        if let Self::Preexe(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Preexe`, panics if variant doesn't match
    pub fn into_preexe(self) -> Preexe {
        if let Self::Preexe(inner) = self {
            inner
        } else {
            panic!("bug: expected type Preexe, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Procarg0`
    pub fn is_procarg0(&self) -> bool { matches!(self, Self::Procarg0(_)) }
    /// Casts `&Node` to `Option<&nodes::Procarg0>`
    pub fn as_procarg0(&self) -> Option<&Procarg0> {
        if let Self::Procarg0(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Procarg0>`
    pub fn as_procarg0_mut(&mut self) -> Option<&mut Procarg0> {
        if let Self::Procarg0(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Procarg0`, panics if variant doesn't match
    pub fn into_procarg0(self) -> Procarg0 {
        if let Self::Procarg0(inner) = self {
            inner
        } else {
            panic!("bug: expected type Procarg0, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Rational`
    pub fn is_rational(&self) -> bool { matches!(self, Self::Rational(_)) }
    /// Casts `&Node` to `Option<&nodes::Rational>`
    pub fn as_rational(&self) -> Option<&Rational> {
        if let Self::Rational(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Rational>`
    pub fn as_rational_mut(&mut self) -> Option<&mut Rational> {
        if let Self::Rational(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Rational`, panics if variant doesn't match
    pub fn into_rational(self) -> Rational {
        if let Self::Rational(inner) = self {
            inner
        } else {
            panic!("bug: expected type Rational, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Redo`
    pub fn is_redo(&self) -> bool { matches!(self, Self::Redo(_)) }
    /// Casts `&Node` to `Option<&nodes::Redo>`
    pub fn as_redo(&self) -> Option<&Redo> {
        if let Self::Redo(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Redo>`
    pub fn as_redo_mut(&mut self) -> Option<&mut Redo> {
        if let Self::Redo(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Redo`, panics if variant doesn't match
    pub fn into_redo(self) -> Redo {
        if let Self::Redo(inner) = self {
            inner
        } else {
            panic!("bug: expected type Redo, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::RegOpt`
    pub fn is_reg_opt(&self) -> bool { matches!(self, Self::RegOpt(_)) }
    /// Casts `&Node` to `Option<&nodes::RegOpt>`
    pub fn as_reg_opt(&self) -> Option<&RegOpt> {
        if let Self::RegOpt(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::RegOpt>`
    pub fn as_reg_opt_mut(&mut self) -> Option<&mut RegOpt> {
        if let Self::RegOpt(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::RegOpt`, panics if variant doesn't match
    pub fn into_reg_opt(self) -> RegOpt {
        if let Self::RegOpt(inner) = self {
            inner
        } else {
            panic!("bug: expected type RegOpt, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Regexp`
    pub fn is_regexp(&self) -> bool { matches!(self, Self::Regexp(_)) }
    /// Casts `&Node` to `Option<&nodes::Regexp>`
    pub fn as_regexp(&self) -> Option<&Regexp> {
        if let Self::Regexp(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Regexp>`
    pub fn as_regexp_mut(&mut self) -> Option<&mut Regexp> {
        if let Self::Regexp(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Regexp`, panics if variant doesn't match
    pub fn into_regexp(self) -> Regexp {
        if let Self::Regexp(inner) = self {
            inner
        } else {
            panic!("bug: expected type Regexp, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Rescue`
    pub fn is_rescue(&self) -> bool { matches!(self, Self::Rescue(_)) }
    /// Casts `&Node` to `Option<&nodes::Rescue>`
    pub fn as_rescue(&self) -> Option<&Rescue> {
        if let Self::Rescue(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Rescue>`
    pub fn as_rescue_mut(&mut self) -> Option<&mut Rescue> {
        if let Self::Rescue(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Rescue`, panics if variant doesn't match
    pub fn into_rescue(self) -> Rescue {
        if let Self::Rescue(inner) = self {
            inner
        } else {
            panic!("bug: expected type Rescue, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::RescueBody`
    pub fn is_rescue_body(&self) -> bool { matches!(self, Self::RescueBody(_)) }
    /// Casts `&Node` to `Option<&nodes::RescueBody>`
    pub fn as_rescue_body(&self) -> Option<&RescueBody> {
        if let Self::RescueBody(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::RescueBody>`
    pub fn as_rescue_body_mut(&mut self) -> Option<&mut RescueBody> {
        if let Self::RescueBody(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::RescueBody`, panics if variant doesn't match
    pub fn into_rescue_body(self) -> RescueBody {
        if let Self::RescueBody(inner) = self {
            inner
        } else {
            panic!("bug: expected type RescueBody, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Restarg`
    pub fn is_restarg(&self) -> bool { matches!(self, Self::Restarg(_)) }
    /// Casts `&Node` to `Option<&nodes::Restarg>`
    pub fn as_restarg(&self) -> Option<&Restarg> {
        if let Self::Restarg(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Restarg>`
    pub fn as_restarg_mut(&mut self) -> Option<&mut Restarg> {
        if let Self::Restarg(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Restarg`, panics if variant doesn't match
    pub fn into_restarg(self) -> Restarg {
        if let Self::Restarg(inner) = self {
            inner
        } else {
            panic!("bug: expected type Restarg, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Retry`
    pub fn is_retry(&self) -> bool { matches!(self, Self::Retry(_)) }
    /// Casts `&Node` to `Option<&nodes::Retry>`
    pub fn as_retry(&self) -> Option<&Retry> {
        if let Self::Retry(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Retry>`
    pub fn as_retry_mut(&mut self) -> Option<&mut Retry> {
        if let Self::Retry(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Retry`, panics if variant doesn't match
    pub fn into_retry(self) -> Retry {
        if let Self::Retry(inner) = self {
            inner
        } else {
            panic!("bug: expected type Retry, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Return`
    pub fn is_return(&self) -> bool { matches!(self, Self::Return(_)) }
    /// Casts `&Node` to `Option<&nodes::Return>`
    pub fn as_return(&self) -> Option<&Return> {
        if let Self::Return(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Return>`
    pub fn as_return_mut(&mut self) -> Option<&mut Return> {
        if let Self::Return(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Return`, panics if variant doesn't match
    pub fn into_return(self) -> Return {
        if let Self::Return(inner) = self {
            inner
        } else {
            panic!("bug: expected type Return, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::SClass`
    pub fn is_s_class(&self) -> bool { matches!(self, Self::SClass(_)) }
    /// Casts `&Node` to `Option<&nodes::SClass>`
    pub fn as_s_class(&self) -> Option<&SClass> {
        if let Self::SClass(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::SClass>`
    pub fn as_s_class_mut(&mut self) -> Option<&mut SClass> {
        if let Self::SClass(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::SClass`, panics if variant doesn't match
    pub fn into_s_class(self) -> SClass {
        if let Self::SClass(inner) = self {
            inner
        } else {
            panic!("bug: expected type SClass, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Self_`
    pub fn is_self_(&self) -> bool { matches!(self, Self::Self_(_)) }
    /// Casts `&Node` to `Option<&nodes::Self_>`
    pub fn as_self_(&self) -> Option<&Self_> {
        if let Self::Self_(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Self_>`
    pub fn as_self_mut(&mut self) -> Option<&mut Self_> {
        if let Self::Self_(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Self_`, panics if variant doesn't match
    pub fn into_self_(self) -> Self_ {
        if let Self::Self_(inner) = self {
            inner
        } else {
            panic!("bug: expected type Self_, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Send`
    pub fn is_send(&self) -> bool { matches!(self, Self::Send(_)) }
    /// Casts `&Node` to `Option<&nodes::Send>`
    pub fn as_send(&self) -> Option<&Send> {
        if let Self::Send(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Send>`
    pub fn as_send_mut(&mut self) -> Option<&mut Send> {
        if let Self::Send(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Send`, panics if variant doesn't match
    pub fn into_send(self) -> Send {
        if let Self::Send(inner) = self {
            inner
        } else {
            panic!("bug: expected type Send, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Shadowarg`
    pub fn is_shadowarg(&self) -> bool { matches!(self, Self::Shadowarg(_)) }
    /// Casts `&Node` to `Option<&nodes::Shadowarg>`
    pub fn as_shadowarg(&self) -> Option<&Shadowarg> {
        if let Self::Shadowarg(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Shadowarg>`
    pub fn as_shadowarg_mut(&mut self) -> Option<&mut Shadowarg> {
        if let Self::Shadowarg(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Shadowarg`, panics if variant doesn't match
    pub fn into_shadowarg(self) -> Shadowarg {
        if let Self::Shadowarg(inner) = self {
            inner
        } else {
            panic!("bug: expected type Shadowarg, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Splat`
    pub fn is_splat(&self) -> bool { matches!(self, Self::Splat(_)) }
    /// Casts `&Node` to `Option<&nodes::Splat>`
    pub fn as_splat(&self) -> Option<&Splat> {
        if let Self::Splat(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Splat>`
    pub fn as_splat_mut(&mut self) -> Option<&mut Splat> {
        if let Self::Splat(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Splat`, panics if variant doesn't match
    pub fn into_splat(self) -> Splat {
        if let Self::Splat(inner) = self {
            inner
        } else {
            panic!("bug: expected type Splat, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Str`
    pub fn is_str(&self) -> bool { matches!(self, Self::Str(_)) }
    /// Casts `&Node` to `Option<&nodes::Str>`
    pub fn as_str(&self) -> Option<&Str> {
        if let Self::Str(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Str>`
    pub fn as_str_mut(&mut self) -> Option<&mut Str> {
        if let Self::Str(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Str`, panics if variant doesn't match
    pub fn into_str(self) -> Str {
        if let Self::Str(inner) = self {
            inner
        } else {
            panic!("bug: expected type Str, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Super`
    pub fn is_super(&self) -> bool { matches!(self, Self::Super(_)) }
    /// Casts `&Node` to `Option<&nodes::Super>`
    pub fn as_super(&self) -> Option<&Super> {
        if let Self::Super(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Super>`
    pub fn as_super_mut(&mut self) -> Option<&mut Super> {
        if let Self::Super(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Super`, panics if variant doesn't match
    pub fn into_super(self) -> Super {
        if let Self::Super(inner) = self {
            inner
        } else {
            panic!("bug: expected type Super, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Sym`
    pub fn is_sym(&self) -> bool { matches!(self, Self::Sym(_)) }
    /// Casts `&Node` to `Option<&nodes::Sym>`
    pub fn as_sym(&self) -> Option<&Sym> {
        if let Self::Sym(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Sym>`
    pub fn as_sym_mut(&mut self) -> Option<&mut Sym> {
        if let Self::Sym(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Sym`, panics if variant doesn't match
    pub fn into_sym(self) -> Sym {
        if let Self::Sym(inner) = self {
            inner
        } else {
            panic!("bug: expected type Sym, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::True`
    pub fn is_true(&self) -> bool { matches!(self, Self::True(_)) }
    /// Casts `&Node` to `Option<&nodes::True>`
    pub fn as_true(&self) -> Option<&True> {
        if let Self::True(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::True>`
    pub fn as_true_mut(&mut self) -> Option<&mut True> {
        if let Self::True(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::True`, panics if variant doesn't match
    pub fn into_true(self) -> True {
        if let Self::True(inner) = self {
            inner
        } else {
            panic!("bug: expected type True, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Undef`
    pub fn is_undef(&self) -> bool { matches!(self, Self::Undef(_)) }
    /// Casts `&Node` to `Option<&nodes::Undef>`
    pub fn as_undef(&self) -> Option<&Undef> {
        if let Self::Undef(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Undef>`
    pub fn as_undef_mut(&mut self) -> Option<&mut Undef> {
        if let Self::Undef(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Undef`, panics if variant doesn't match
    pub fn into_undef(self) -> Undef {
        if let Self::Undef(inner) = self {
            inner
        } else {
            panic!("bug: expected type Undef, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::UnlessGuard`
    pub fn is_unless_guard(&self) -> bool { matches!(self, Self::UnlessGuard(_)) }
    /// Casts `&Node` to `Option<&nodes::UnlessGuard>`
    pub fn as_unless_guard(&self) -> Option<&UnlessGuard> {
        if let Self::UnlessGuard(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::UnlessGuard>`
    pub fn as_unless_guard_mut(&mut self) -> Option<&mut UnlessGuard> {
        if let Self::UnlessGuard(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::UnlessGuard`, panics if variant doesn't match
    pub fn into_unless_guard(self) -> UnlessGuard {
        if let Self::UnlessGuard(inner) = self {
            inner
        } else {
            panic!("bug: expected type UnlessGuard, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Until`
    pub fn is_until(&self) -> bool { matches!(self, Self::Until(_)) }
    /// Casts `&Node` to `Option<&nodes::Until>`
    pub fn as_until(&self) -> Option<&Until> {
        if let Self::Until(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Until>`
    pub fn as_until_mut(&mut self) -> Option<&mut Until> {
        if let Self::Until(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Until`, panics if variant doesn't match
    pub fn into_until(self) -> Until {
        if let Self::Until(inner) = self {
            inner
        } else {
            panic!("bug: expected type Until, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::UntilPost`
    pub fn is_until_post(&self) -> bool { matches!(self, Self::UntilPost(_)) }
    /// Casts `&Node` to `Option<&nodes::UntilPost>`
    pub fn as_until_post(&self) -> Option<&UntilPost> {
        if let Self::UntilPost(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::UntilPost>`
    pub fn as_until_post_mut(&mut self) -> Option<&mut UntilPost> {
        if let Self::UntilPost(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::UntilPost`, panics if variant doesn't match
    pub fn into_until_post(self) -> UntilPost {
        if let Self::UntilPost(inner) = self {
            inner
        } else {
            panic!("bug: expected type UntilPost, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::When`
    pub fn is_when(&self) -> bool { matches!(self, Self::When(_)) }
    /// Casts `&Node` to `Option<&nodes::When>`
    pub fn as_when(&self) -> Option<&When> {
        if let Self::When(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::When>`
    pub fn as_when_mut(&mut self) -> Option<&mut When> {
        if let Self::When(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::When`, panics if variant doesn't match
    pub fn into_when(self) -> When {
        if let Self::When(inner) = self {
            inner
        } else {
            panic!("bug: expected type When, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::While`
    pub fn is_while(&self) -> bool { matches!(self, Self::While(_)) }
    /// Casts `&Node` to `Option<&nodes::While>`
    pub fn as_while(&self) -> Option<&While> {
        if let Self::While(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::While>`
    pub fn as_while_mut(&mut self) -> Option<&mut While> {
        if let Self::While(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::While`, panics if variant doesn't match
    pub fn into_while(self) -> While {
        if let Self::While(inner) = self {
            inner
        } else {
            panic!("bug: expected type While, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::WhilePost`
    pub fn is_while_post(&self) -> bool { matches!(self, Self::WhilePost(_)) }
    /// Casts `&Node` to `Option<&nodes::WhilePost>`
    pub fn as_while_post(&self) -> Option<&WhilePost> {
        if let Self::WhilePost(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::WhilePost>`
    pub fn as_while_post_mut(&mut self) -> Option<&mut WhilePost> {
        if let Self::WhilePost(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::WhilePost`, panics if variant doesn't match
    pub fn into_while_post(self) -> WhilePost {
        if let Self::WhilePost(inner) = self {
            inner
        } else {
            panic!("bug: expected type WhilePost, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::XHeredoc`
    pub fn is_x_heredoc(&self) -> bool { matches!(self, Self::XHeredoc(_)) }
    /// Casts `&Node` to `Option<&nodes::XHeredoc>`
    pub fn as_x_heredoc(&self) -> Option<&XHeredoc> {
        if let Self::XHeredoc(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::XHeredoc>`
    pub fn as_x_heredoc_mut(&mut self) -> Option<&mut XHeredoc> {
        if let Self::XHeredoc(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::XHeredoc`, panics if variant doesn't match
    pub fn into_x_heredoc(self) -> XHeredoc {
        if let Self::XHeredoc(inner) = self {
            inner
        } else {
            panic!("bug: expected type XHeredoc, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Xstr`
    pub fn is_xstr(&self) -> bool { matches!(self, Self::Xstr(_)) }
    /// Casts `&Node` to `Option<&nodes::Xstr>`
    pub fn as_xstr(&self) -> Option<&Xstr> {
        if let Self::Xstr(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Xstr>`
    pub fn as_xstr_mut(&mut self) -> Option<&mut Xstr> {
        if let Self::Xstr(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Xstr`, panics if variant doesn't match
    pub fn into_xstr(self) -> Xstr {
        if let Self::Xstr(inner) = self {
            inner
        } else {
            panic!("bug: expected type Xstr, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::Yield`
    pub fn is_yield(&self) -> bool { matches!(self, Self::Yield(_)) }
    /// Casts `&Node` to `Option<&nodes::Yield>`
    pub fn as_yield(&self) -> Option<&Yield> {
        if let Self::Yield(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::Yield>`
    pub fn as_yield_mut(&mut self) -> Option<&mut Yield> {
        if let Self::Yield(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::Yield`, panics if variant doesn't match
    pub fn into_yield(self) -> Yield {
        if let Self::Yield(inner) = self {
            inner
        } else {
            panic!("bug: expected type Yield, got {:?}", self)
        }
    }

    /// Returns true if `self` is `Node::ZSuper`
    pub fn is_z_super(&self) -> bool { matches!(self, Self::ZSuper(_)) }
    /// Casts `&Node` to `Option<&nodes::ZSuper>`
    pub fn as_z_super(&self) -> Option<&ZSuper> {
        if let Self::ZSuper(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `&Node` to `Option<&mut nodes::ZSuper>`
    pub fn as_z_super_mut(&mut self) -> Option<&mut ZSuper> {
        if let Self::ZSuper(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
    /// Casts `self` to `nodes::ZSuper`, panics if variant doesn't match
    pub fn into_z_super(self) -> ZSuper {
        if let Self::ZSuper(inner) = self {
            inner
        } else {
            panic!("bug: expected type ZSuper, got {:?}", self)
        }
    }

}
