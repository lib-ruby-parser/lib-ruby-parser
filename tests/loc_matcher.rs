use lib_ruby_parser::source::Range;
use lib_ruby_parser::traverse::Find;
use lib_ruby_parser::Node;

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
                Node::Args(inner) => inner.begin_l.clone(),
                Node::Array(inner) => inner.begin_l.clone(),
                Node::ArrayPattern(inner) => inner.begin_l.clone(),
                Node::ArrayPatternWithTail(inner) => inner.begin_l.clone(),
                Node::Begin(inner) => inner.begin_l.clone(),
                Node::CSend(inner) => inner.begin_l.clone(),
                Node::Defined(inner) => inner.begin_l.clone(),
                Node::Dstr(inner) => inner.begin_l.clone(),
                Node::Dsym(inner) => inner.begin_l.clone(),
                Node::FindPattern(inner) => inner.begin_l.clone(),
                Node::Hash(inner) => inner.begin_l.clone(),
                Node::HashPattern(inner) => inner.begin_l.clone(),
                Node::KwBegin(inner) => inner.begin_l.clone(),
                Node::Mlhs(inner) => inner.begin_l.clone(),
                Node::Procarg0(inner) => inner.begin_l.clone(),
                Node::RescueBody(inner) => inner.begin_l.clone(),
                Node::Send(inner) => inner.begin_l.clone(),
                Node::Str(inner) => inner.begin_l.clone(),
                Node::Super(inner) => inner.begin_l.clone(),
                Node::Sym(inner) => inner.begin_l.clone(),
                Node::Until(inner) => inner.begin_l.clone(),
                Node::While(inner) => inner.begin_l.clone(),
                Node::Yield(inner) => inner.begin_l.clone(),

                Node::Block(inner) => Some(inner.begin_l.clone()),
                Node::ConstPattern(inner) => Some(inner.begin_l.clone()),
                Node::For(inner) => Some(inner.begin_l.clone()),
                Node::If(inner) => Some(inner.begin_l.clone()),
                Node::InPattern(inner) => Some(inner.begin_l.clone()),
                Node::Index(inner) => Some(inner.begin_l.clone()),
                Node::IndexAsgn(inner) => Some(inner.begin_l.clone()),
                Node::Numblock(inner) => Some(inner.begin_l.clone()),
                Node::Postexe(inner) => Some(inner.begin_l.clone()),
                Node::Preexe(inner) => Some(inner.begin_l.clone()),
                Node::Regexp(inner) => Some(inner.begin_l.clone()),
                Node::When(inner) => Some(inner.begin_l.clone()),
                Node::Xstr(inner) => Some(inner.begin_l.clone()),

                other => panic!("node {} doesn't support begin loc", other.str_type()),
            },
            LocName::End => match node {
                Node::Args(inner) => inner.end_l.clone(),
                Node::Array(inner) => inner.end_l.clone(),
                Node::ArrayPattern(inner) => inner.end_l.clone(),
                Node::ArrayPatternWithTail(inner) => inner.end_l.clone(),
                Node::Begin(inner) => inner.end_l.clone(),
                Node::CSend(inner) => inner.end_l.clone(),
                Node::Def(inner) => inner.end_l.clone(),
                Node::Defined(inner) => inner.end_l.clone(),
                Node::Defs(inner) => inner.end_l.clone(),
                Node::Dstr(inner) => inner.end_l.clone(),
                Node::Dsym(inner) => inner.end_l.clone(),
                Node::FindPattern(inner) => inner.end_l.clone(),
                Node::Hash(inner) => inner.end_l.clone(),
                Node::HashPattern(inner) => inner.end_l.clone(),
                Node::If(inner) => inner.end_l.clone(),
                Node::KwBegin(inner) => inner.end_l.clone(),
                Node::Mlhs(inner) => inner.end_l.clone(),
                Node::Procarg0(inner) => inner.end_l.clone(),
                Node::Send(inner) => inner.end_l.clone(),
                Node::Str(inner) => inner.end_l.clone(),
                Node::Super(inner) => inner.end_l.clone(),
                Node::Sym(inner) => inner.end_l.clone(),
                Node::Until(inner) => inner.end_l.clone(),
                Node::While(inner) => inner.end_l.clone(),
                Node::Yield(inner) => inner.end_l.clone(),

                Node::Block(inner) => Some(inner.end_l.clone()),
                Node::Case(inner) => Some(inner.end_l.clone()),
                Node::CaseMatch(inner) => Some(inner.end_l.clone()),
                Node::Class(inner) => Some(inner.end_l.clone()),
                Node::ConstPattern(inner) => Some(inner.end_l.clone()),
                Node::For(inner) => Some(inner.end_l.clone()),
                Node::Index(inner) => Some(inner.end_l.clone()),
                Node::IndexAsgn(inner) => Some(inner.end_l.clone()),
                Node::Module(inner) => Some(inner.end_l.clone()),
                Node::Numblock(inner) => Some(inner.end_l.clone()),
                Node::Postexe(inner) => Some(inner.end_l.clone()),
                Node::Preexe(inner) => Some(inner.end_l.clone()),
                Node::Regexp(inner) => Some(inner.end_l.clone()),
                Node::SClass(inner) => Some(inner.end_l.clone()),
                Node::Xstr(inner) => Some(inner.end_l.clone()),

                other => panic!("node {} doesn't support end loc", other.str_type()),
            },
            LocName::Expression => Some(node.expression().clone()),
            LocName::Keyword => match node {
                Node::Alias(inner) => Some(inner.keyword_l.clone()),
                Node::Break(inner) => Some(inner.keyword_l.clone()),
                Node::Case(inner) => Some(inner.keyword_l.clone()),
                Node::CaseMatch(inner) => Some(inner.keyword_l.clone()),
                Node::Class(inner) => Some(inner.keyword_l.clone()),
                Node::Def(inner) => Some(inner.keyword_l.clone()),
                Node::Defined(inner) => Some(inner.keyword_l.clone()),
                Node::Defs(inner) => Some(inner.keyword_l.clone()),
                Node::Ensure(inner) => Some(inner.keyword_l.clone()),
                Node::For(inner) => Some(inner.keyword_l.clone()),
                Node::If(inner) => Some(inner.keyword_l.clone()),
                Node::IfGuard(inner) => Some(inner.keyword_l.clone()),
                Node::IfMod(inner) => Some(inner.keyword_l.clone()),
                Node::InPattern(inner) => Some(inner.keyword_l.clone()),
                Node::Module(inner) => Some(inner.keyword_l.clone()),
                Node::Next(inner) => Some(inner.keyword_l.clone()),
                Node::Postexe(inner) => Some(inner.keyword_l.clone()),
                Node::Preexe(inner) => Some(inner.keyword_l.clone()),
                Node::RescueBody(inner) => Some(inner.keyword_l.clone()),
                Node::Return(inner) => Some(inner.keyword_l.clone()),
                Node::SClass(inner) => Some(inner.keyword_l.clone()),
                Node::Super(inner) => Some(inner.keyword_l.clone()),
                Node::Undef(inner) => Some(inner.keyword_l.clone()),
                Node::UnlessGuard(inner) => Some(inner.keyword_l.clone()),
                Node::Until(inner) => Some(inner.keyword_l.clone()),
                Node::UntilPost(inner) => Some(inner.keyword_l.clone()),
                Node::When(inner) => Some(inner.keyword_l.clone()),
                Node::While(inner) => Some(inner.keyword_l.clone()),
                Node::WhilePost(inner) => Some(inner.keyword_l.clone()),
                Node::Yield(inner) => Some(inner.keyword_l.clone()),

                other => panic!("node {} doesn't support keyword loc", other.str_type()),
            },
            LocName::Name => match node {
                Node::Blockarg(inner) => Some(inner.name_l.clone()),
                Node::Casgn(inner) => Some(inner.name_l.clone()),
                Node::Const(inner) => Some(inner.name_l.clone()),
                Node::Cvasgn(inner) => Some(inner.name_l.clone()),
                Node::Def(inner) => Some(inner.name_l.clone()),
                Node::Defs(inner) => Some(inner.name_l.clone()),
                Node::Gvasgn(inner) => Some(inner.name_l.clone()),
                Node::Ivasgn(inner) => Some(inner.name_l.clone()),
                Node::Kwarg(inner) => Some(inner.name_l.clone()),
                Node::Kwoptarg(inner) => Some(inner.name_l.clone()),
                Node::Lvasgn(inner) => Some(inner.name_l.clone()),
                Node::MatchNilPattern(inner) => Some(inner.name_l.clone()),
                Node::Kwnilarg(inner) => Some(inner.name_l.clone()),
                Node::MatchVar(inner) => Some(inner.name_l.clone()),
                Node::Optarg(inner) => Some(inner.name_l.clone()),

                Node::Kwrestarg(inner) => inner.name_l.clone(),
                Node::Restarg(inner) => inner.name_l.clone(),

                other => panic!("node {} doesn't support name loc", other.str_type()),
            },
            LocName::Assignment => match node {
                Node::Def(inner) => inner.assignment_l.clone(),
                Node::Defs(inner) => inner.assignment_l.clone(),
                other => panic!("node {} doesn't support assignment loc", other.str_type()),
            },
            LocName::Colon => match node {
                Node::IfTernary(inner) => Some(inner.colon_l.clone()),
                other => panic!("node {} doesn't support colon loc", other.str_type()),
            },
            LocName::DoubleColon => match node {
                Node::Casgn(inner) => inner.double_colon_l.clone(),
                Node::Const(inner) => inner.double_colon_l.clone(),
                other => panic!("node {} doesn't support double_colon loc", other.str_type()),
            },
            LocName::Else => match node {
                Node::Case(inner) => inner.else_l.clone(),
                Node::CaseMatch(inner) => inner.else_l.clone(),
                Node::If(inner) => inner.else_l.clone(),
                Node::Rescue(inner) => inner.else_l.clone(),
                other => panic!("node {} doesn't support else loc", other.str_type()),
            },
            LocName::HeredocBody => match node {
                Node::Heredoc(inner) => Some(inner.heredoc_body_l.clone()),
                Node::XHeredoc(inner) => Some(inner.heredoc_body_l.clone()),
                other => panic!("node {} doesn't support heredoc_body loc", other.str_type()),
            },
            LocName::Operator => match node {
                Node::And(inner) => Some(inner.operator_l.clone()),
                Node::AndAsgn(inner) => Some(inner.operator_l.clone()),
                Node::BlockPass(inner) => Some(inner.operator_l.clone()),
                Node::Defs(inner) => Some(inner.operator_l.clone()),
                Node::EFlipFlop(inner) => Some(inner.operator_l.clone()),
                Node::Erange(inner) => Some(inner.operator_l.clone()),
                Node::IFlipFlop(inner) => Some(inner.operator_l.clone()),
                Node::InMatch(inner) => Some(inner.operator_l.clone()),
                Node::Irange(inner) => Some(inner.operator_l.clone()),
                Node::Kwsplat(inner) => Some(inner.operator_l.clone()),
                Node::Masgn(inner) => Some(inner.operator_l.clone()),
                Node::MatchAlt(inner) => Some(inner.operator_l.clone()),
                Node::MatchAs(inner) => Some(inner.operator_l.clone()),
                Node::MatchNilPattern(inner) => Some(inner.operator_l.clone()),
                Node::MatchRest(inner) => Some(inner.operator_l.clone()),
                Node::MatchWithLvasgn(inner) => Some(inner.operator_l.clone()),
                Node::OpAsgn(inner) => Some(inner.operator_l.clone()),
                Node::Optarg(inner) => Some(inner.operator_l.clone()),
                Node::Or(inner) => Some(inner.operator_l.clone()),
                Node::OrAsgn(inner) => Some(inner.operator_l.clone()),
                Node::Pair(inner) => Some(inner.operator_l.clone()),
                Node::SClass(inner) => Some(inner.operator_l.clone()),
                Node::Splat(inner) => Some(inner.operator_l.clone()),

                Node::Casgn(inner) => inner.operator_l.clone(),
                Node::Class(inner) => inner.operator_l.clone(),
                Node::Complex(inner) => inner.operator_l.clone(),
                Node::CSend(inner) => inner.operator_l.clone(),
                Node::Cvasgn(inner) => inner.operator_l.clone(),
                Node::Float(inner) => inner.operator_l.clone(),
                Node::Gvasgn(inner) => inner.operator_l.clone(),
                Node::IndexAsgn(inner) => inner.operator_l.clone(),
                Node::Int(inner) => inner.operator_l.clone(),
                Node::Ivasgn(inner) => inner.operator_l.clone(),
                Node::Lvasgn(inner) => inner.operator_l.clone(),
                Node::Rational(inner) => inner.operator_l.clone(),
                Node::Send(inner) => inner.operator_l.clone(),

                other => panic!("node {} doesn't support operator loc", other.str_type()),
            },
            LocName::Selector => match node {
                Node::Send(inner) => inner.selector_l.clone(),
                Node::CSend(inner) => Some(inner.selector_l.clone()),
                Node::Pin(inner) => Some(inner.selector_l.clone()),
                other => panic!("node {} doesn't support selector loc", other.str_type()),
            },
            LocName::Assoc => match node {
                Node::RescueBody(inner) => inner.assoc_l.clone(),
                other => panic!("node {} doesn't support assoc loc", other.str_type()),
            },
            LocName::Question => match node {
                Node::IfTernary(inner) => Some(inner.question_l.clone()),
                other => panic!("node {} doesn't support question loc", other.str_type()),
            },
            LocName::HeredocEnd => match node {
                Node::Heredoc(inner) => Some(inner.heredoc_end_l.clone()),
                Node::XHeredoc(inner) => Some(inner.heredoc_end_l.clone()),
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
