use ruby_parser::nodes::*;
use ruby_parser::source::Range;
use ruby_parser::traverse::Find;
use ruby_parser::Node;

#[derive(Debug, PartialEq)]
enum ParseLocState {
    SkipWs,
    Cursor,
    Name,
    Lparen,
    Pattern,
    Done,
}

#[derive(Debug)]
enum LocName {
    Begin,
    End,
    Expression,
    Keyword,
    Name,
    Assignment,
    Colon,
    DoubleColon,
    Else,
    HeredocBody,
    Operator,
    Selector,
    Assoc,
    Question,
    HeredocEnd,
}

impl LocName {
    pub fn new(name: &str) -> Self {
        match &name[..] {
            "begin" => LocName::Begin,
            "end" => LocName::End,
            "expression" => LocName::Expression,
            "keyword" => LocName::Keyword,
            "name" => LocName::Name,
            "assignment" => LocName::Assignment,
            "colon" => LocName::Colon,
            "double_colon" => LocName::DoubleColon,
            "else" => LocName::Else,
            "heredoc_body" => LocName::HeredocBody,
            "operator" => LocName::Operator,
            "selector" => LocName::Selector,
            "assoc" => LocName::Assoc,
            "question" => LocName::Question,
            "heredoc_end" => LocName::HeredocEnd,
            _ => panic!("unsupported loc name {}", name),
        }
    }

    pub fn get(&self, node: &Node) -> Option<Range> {
        match self {
            LocName::Begin => match node {
                Node::Args(Args { begin_l, .. })
                | Node::Array(Array { begin_l, .. })
                | Node::ArrayPattern(ArrayPattern { begin_l, .. })
                | Node::ArrayPatternWithTail(ArrayPatternWithTail { begin_l, .. })
                | Node::Begin(Begin { begin_l, .. })
                | Node::CSend(CSend { begin_l, .. })
                | Node::Defined(Defined { begin_l, .. })
                | Node::Dstr(Dstr { begin_l, .. })
                | Node::Dsym(Dsym { begin_l, .. })
                | Node::FindPattern(FindPattern { begin_l, .. })
                | Node::Hash(Hash { begin_l, .. })
                | Node::HashPattern(HashPattern { begin_l, .. })
                | Node::KwBegin(KwBegin { begin_l, .. })
                | Node::Mlhs(Mlhs { begin_l, .. })
                | Node::Procarg0(Procarg0 { begin_l, .. })
                | Node::RescueBody(RescueBody { begin_l, .. })
                | Node::Send(Send { begin_l, .. })
                | Node::Str(Str { begin_l, .. })
                | Node::Super(Super { begin_l, .. })
                | Node::Sym(Sym { begin_l, .. })
                | Node::Until(Until { begin_l, .. })
                | Node::While(While { begin_l, .. })
                | Node::Yield(Yield { begin_l, .. }) => begin_l.clone(),

                Node::Block(Block { begin_l, .. })
                | Node::ConstPattern(ConstPattern { begin_l, .. })
                | Node::For(For { begin_l, .. })
                | Node::If(If { begin_l, .. })
                | Node::InPattern(InPattern { begin_l, .. })
                | Node::Index(Index { begin_l, .. })
                | Node::IndexAsgn(IndexAsgn { begin_l, .. })
                | Node::Numblock(Numblock { begin_l, .. })
                | Node::Postexe(Postexe { begin_l, .. })
                | Node::Preexe(Preexe { begin_l, .. })
                | Node::Regexp(Regexp { begin_l, .. })
                | Node::When(When { begin_l, .. })
                | Node::Xstr(Xstr { begin_l, .. }) => Some(begin_l.clone()),

                other => panic!("node {} doesn't support begin loc", other.str_type()),
            },
            LocName::End => match node {
                Node::Args(Args { end_l, .. })
                | Node::Array(Array { end_l, .. })
                | Node::ArrayPattern(ArrayPattern { end_l, .. })
                | Node::ArrayPatternWithTail(ArrayPatternWithTail { end_l, .. })
                | Node::Begin(Begin { end_l, .. })
                | Node::CSend(CSend { end_l, .. })
                | Node::Def(Def { end_l, .. })
                | Node::Defined(Defined { end_l, .. })
                | Node::Defs(Defs { end_l, .. })
                | Node::Dstr(Dstr { end_l, .. })
                | Node::Dsym(Dsym { end_l, .. })
                | Node::FindPattern(FindPattern { end_l, .. })
                | Node::Hash(Hash { end_l, .. })
                | Node::HashPattern(HashPattern { end_l, .. })
                | Node::If(If { end_l, .. })
                | Node::KwBegin(KwBegin { end_l, .. })
                | Node::Mlhs(Mlhs { end_l, .. })
                | Node::Procarg0(Procarg0 { end_l, .. })
                | Node::Send(Send { end_l, .. })
                | Node::Str(Str { end_l, .. })
                | Node::Super(Super { end_l, .. })
                | Node::Sym(Sym { end_l, .. })
                | Node::Until(Until { end_l, .. })
                | Node::While(While { end_l, .. })
                | Node::Yield(Yield { end_l, .. }) => end_l.clone(),

                Node::Block(Block { end_l, .. })
                | Node::Case(Case { end_l, .. })
                | Node::CaseMatch(CaseMatch { end_l, .. })
                | Node::Class(Class { end_l, .. })
                | Node::ConstPattern(ConstPattern { end_l, .. })
                | Node::For(For { end_l, .. })
                | Node::Index(Index { end_l, .. })
                | Node::IndexAsgn(IndexAsgn { end_l, .. })
                | Node::Module(Module { end_l, .. })
                | Node::Numblock(Numblock { end_l, .. })
                | Node::Postexe(Postexe { end_l, .. })
                | Node::Preexe(Preexe { end_l, .. })
                | Node::Regexp(Regexp { end_l, .. })
                | Node::SClass(SClass { end_l, .. })
                | Node::Xstr(Xstr { end_l, .. }) => Some(end_l.clone()),

                other => panic!("node {} doesn't support end loc", other.str_type()),
            },
            LocName::Expression => Some(node.expression().clone()),
            LocName::Keyword => match node {
                Node::Alias(Alias { keyword_l, .. })
                | Node::Break(Break { keyword_l, .. })
                | Node::Case(Case { keyword_l, .. })
                | Node::CaseMatch(CaseMatch { keyword_l, .. })
                | Node::Class(Class { keyword_l, .. })
                | Node::Def(Def { keyword_l, .. })
                | Node::Defined(Defined { keyword_l, .. })
                | Node::Defs(Defs { keyword_l, .. })
                | Node::Ensure(Ensure { keyword_l, .. })
                | Node::For(For { keyword_l, .. })
                | Node::If(If { keyword_l, .. })
                | Node::IfGuard(IfGuard { keyword_l, .. })
                | Node::IfMod(IfMod { keyword_l, .. })
                | Node::InPattern(InPattern { keyword_l, .. })
                | Node::Module(Module { keyword_l, .. })
                | Node::Next(Next { keyword_l, .. })
                | Node::Postexe(Postexe { keyword_l, .. })
                | Node::Preexe(Preexe { keyword_l, .. })
                | Node::RescueBody(RescueBody { keyword_l, .. })
                | Node::Return(Return { keyword_l, .. })
                | Node::SClass(SClass { keyword_l, .. })
                | Node::Super(Super { keyword_l, .. })
                | Node::Undef(Undef { keyword_l, .. })
                | Node::UnlessGuard(UnlessGuard { keyword_l, .. })
                | Node::Until(Until { keyword_l, .. })
                | Node::UntilPost(UntilPost { keyword_l, .. })
                | Node::When(When { keyword_l, .. })
                | Node::While(While { keyword_l, .. })
                | Node::WhilePost(WhilePost { keyword_l, .. })
                | Node::Yield(Yield { keyword_l, .. }) => Some(keyword_l.clone()),

                other => panic!("node {} doesn't support keyword loc", other.str_type()),
            },
            LocName::Name => match node {
                Node::Blockarg(Blockarg { name_l, .. })
                | Node::Casgn(Casgn { name_l, .. })
                | Node::Const(Const { name_l, .. })
                | Node::Cvasgn(Cvasgn { name_l, .. })
                | Node::Def(Def { name_l, .. })
                | Node::Defs(Defs { name_l, .. })
                | Node::Gvasgn(Gvasgn { name_l, .. })
                | Node::Ivasgn(Ivasgn { name_l, .. })
                | Node::Kwarg(Kwarg { name_l, .. })
                | Node::Kwoptarg(Kwoptarg { name_l, .. })
                | Node::Lvasgn(Lvasgn { name_l, .. })
                | Node::MatchNilPattern(MatchNilPattern { name_l, .. })
                | Node::Kwnilarg(Kwnilarg { name_l, .. })
                | Node::MatchVar(MatchVar { name_l, .. })
                | Node::Optarg(Optarg { name_l, .. }) => Some(name_l.clone()),

                Node::Kwrestarg(Kwrestarg { name_l, .. })
                | Node::Restarg(Restarg { name_l, .. }) => name_l.clone(),

                other => panic!("node {} doesn't support name loc", other.str_type()),
            },
            LocName::Assignment => match node {
                Node::Def(Def { assignment_l, .. }) | Node::Defs(Defs { assignment_l, .. }) => {
                    assignment_l.clone()
                }
                other => panic!("node {} doesn't support assignment loc", other.str_type()),
            },
            LocName::Colon => match node {
                Node::IfTernary(IfTernary { colon_l, .. }) => Some(colon_l.clone()),
                other => panic!("node {} doesn't support colon loc", other.str_type()),
            },
            LocName::DoubleColon => match node {
                Node::Casgn(Casgn { double_colon_l, .. })
                | Node::Const(Const { double_colon_l, .. }) => double_colon_l.clone(),
                other => panic!("node {} doesn't support double_colon loc", other.str_type()),
            },
            LocName::Else => match node {
                Node::Case(Case { else_l, .. })
                | Node::CaseMatch(CaseMatch { else_l, .. })
                | Node::If(If { else_l, .. })
                | Node::Rescue(Rescue { else_l, .. }) => else_l.clone(),
                other => panic!("node {} doesn't support else loc", other.str_type()),
            },
            LocName::HeredocBody => match node {
                Node::Heredoc(Heredoc { heredoc_body_l, .. })
                | Node::XHeredoc(XHeredoc { heredoc_body_l, .. }) => Some(heredoc_body_l.clone()),
                other => panic!("node {} doesn't support heredoc_body loc", other.str_type()),
            },
            LocName::Operator => match node {
                Node::And(And { operator_l, .. })
                | Node::AndAsgn(AndAsgn { operator_l, .. })
                | Node::BlockPass(BlockPass { operator_l, .. })
                | Node::Defs(Defs { operator_l, .. })
                | Node::EFlipFlop(EFlipFlop { operator_l, .. })
                | Node::Erange(Erange { operator_l, .. })
                | Node::IFlipFlop(IFlipFlop { operator_l, .. })
                | Node::InMatch(InMatch { operator_l, .. })
                | Node::Irange(Irange { operator_l, .. })
                | Node::Kwsplat(Kwsplat { operator_l, .. })
                | Node::Masgn(Masgn { operator_l, .. })
                | Node::MatchAlt(MatchAlt { operator_l, .. })
                | Node::MatchAs(MatchAs { operator_l, .. })
                | Node::MatchNilPattern(MatchNilPattern { operator_l, .. })
                | Node::MatchRest(MatchRest { operator_l, .. })
                | Node::MatchWithLvasgn(MatchWithLvasgn { operator_l, .. })
                | Node::OpAsgn(OpAsgn { operator_l, .. })
                | Node::Optarg(Optarg { operator_l, .. })
                | Node::Or(Or { operator_l, .. })
                | Node::OrAsgn(OrAsgn { operator_l, .. })
                | Node::Pair(Pair { operator_l, .. })
                | Node::SClass(SClass { operator_l, .. })
                | Node::Splat(Splat { operator_l, .. }) => Some(operator_l.clone()),

                Node::Casgn(Casgn { operator_l, .. })
                | Node::Class(Class { operator_l, .. })
                | Node::Complex(Complex { operator_l, .. })
                | Node::CSend(CSend { operator_l, .. })
                | Node::Cvasgn(Cvasgn { operator_l, .. })
                | Node::Float(Float { operator_l, .. })
                | Node::Gvasgn(Gvasgn { operator_l, .. })
                | Node::IndexAsgn(IndexAsgn { operator_l, .. })
                | Node::Int(Int { operator_l, .. })
                | Node::Ivasgn(Ivasgn { operator_l, .. })
                | Node::Lvasgn(Lvasgn { operator_l, .. })
                | Node::Rational(Rational { operator_l, .. })
                | Node::Send(Send { operator_l, .. }) => operator_l.clone(),

                other => panic!("node {} doesn't support operator loc", other.str_type()),
            },
            LocName::Selector => match node {
                Node::Send(Send { selector_l, .. }) => selector_l.clone(),
                Node::CSend(CSend { selector_l, .. }) | Node::Pin(Pin { selector_l, .. }) => {
                    Some(selector_l.clone())
                }
                other => panic!("node {} doesn't support selector loc", other.str_type()),
            },
            LocName::Assoc => match node {
                Node::RescueBody(RescueBody { assoc_l, .. }) => assoc_l.clone(),
                other => panic!("node {} doesn't support assoc loc", other.str_type()),
            },
            LocName::Question => match node {
                Node::IfTernary(IfTernary { question_l, .. }) => Some(question_l.clone()),
                other => panic!("node {} doesn't support question loc", other.str_type()),
            },
            LocName::HeredocEnd => match node {
                Node::Heredoc(Heredoc { heredoc_end_l, .. })
                | Node::XHeredoc(XHeredoc { heredoc_end_l, .. }) => Some(heredoc_end_l.clone()),
                other => panic!("node {} doesn't support heredoc_end loc", other.str_type()),
            },
        }
    }
}

#[derive(Debug)]
pub struct LocMatcher {
    begin: usize,
    end: usize,
    name: LocName,
    pattern: Vec<String>,
}

impl LocMatcher {
    pub fn new(loc: &str) -> Self {
        let mut state = ParseLocState::SkipWs;
        let mut begin: Option<usize> = None;
        let mut end: Option<usize> = None;
        let mut name = String::from("");
        let mut pattern = String::from("");

        for (idx, c) in loc.chars().enumerate() {
            match (&state, c) {
                (ParseLocState::SkipWs, ' ') => { /* skip */ }
                (ParseLocState::SkipWs, '~') => {
                    state = ParseLocState::Cursor;
                    begin = Some(idx);
                }
                (ParseLocState::Cursor, '~') => { /* keep reading */ }
                (ParseLocState::Cursor, ' ') => {
                    state = ParseLocState::Name;
                    end = Some(idx);
                }
                (ParseLocState::Name, ' ') => {
                    state = ParseLocState::Lparen;
                }
                (ParseLocState::Name, c) => {
                    name.push(c);
                }
                (ParseLocState::Lparen, '(') => {
                    state = ParseLocState::Pattern;
                }
                (ParseLocState::Pattern, ')') => {
                    state = ParseLocState::Done;
                }
                (ParseLocState::Pattern, c) => pattern.push(c),
                _ => {
                    panic!("Got state = {:?} and rest = {:?}", state, &loc[idx..]);
                }
            }
        }

        if state != ParseLocState::Done {
            panic!("Failed to parse loc {}, state = {:?}", loc, state);
        }
        let begin = begin.unwrap_or_else(|| panic!("no begin captured"));
        let end = end.unwrap_or_else(|| panic!("no begin captured"));

        let pattern = pattern
            .split('/')
            .filter(|e| !e.is_empty())
            .map(|e| e.to_owned())
            .collect::<Vec<_>>();
        let name = LocName::new(&name);

        LocMatcher {
            begin,
            end,
            name,
            pattern,
        }
    }

    pub fn test(&self, root: &Node) -> Result<(), String> {
        match Find::run(&self.pattern, root).unwrap() {
            Some(node) => match self.name.get(&node) {
                Some(range) => {
                    if range.begin_pos != self.begin {
                        return Err(format!(
                            "begin of {:?} - {:?} doesn't match, expected {}, got {}",
                            self.pattern, self.name, self.begin, range.begin_pos
                        ));
                    }

                    if range.end_pos != self.end {
                        return Err(format!(
                            "end of {:?} - {:?} doesn't match, expected {}, got {}",
                            self.pattern, self.name, self.end, range.end_pos
                        ));
                    }

                    Ok(())
                }
                None => Err(format!(
                    "failed to get {:?} of {:?} on {:?}",
                    self.name, self.pattern, root
                )),
            },
            None => panic!("failed to find {:?} in {:?}", self.pattern, root.inspect(0)),
        }
    }
}
