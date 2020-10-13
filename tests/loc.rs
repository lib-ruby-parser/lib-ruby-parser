use ruby_parser::Node;

#[derive(Debug, PartialEq)]
enum ParseLocState {
    SkipWs,
    Cursor,
    Name,
    Lparen,
    Path,
    Done,
}

#[derive(Debug)]
enum LocName {
    Begin,
    End,
    Expression,
    Keyword,
    Name,
}

#[derive(Debug)]
pub struct Loc {
    begin: usize,
    end: usize,
    name: LocName,
    path: Path,
}

impl Loc {
    pub fn new(loc: &str) -> Self {
        let mut state = ParseLocState::SkipWs;
        let mut begin: Option<usize> = None;
        let mut end: Option<usize> = None;
        let mut name = String::from("");
        let mut path = String::from("");

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
                    state = ParseLocState::Path;
                }
                (ParseLocState::Path, ')') => {
                    state = ParseLocState::Done;
                }
                (ParseLocState::Path, c) => path.push(c),
                _ => {
                    panic!("Got state = {:?} and c = {}", state, c);
                }
            }
        }

        if state != ParseLocState::Done {
            panic!("Failed to parse loc {}, state = {:?}", loc, state);
        }
        let begin = begin.unwrap_or_else(|| panic!("no begin captured"));
        let end = end.unwrap_or_else(|| panic!("no begin captured"));

        let path = path.split("/").map(|e| e.to_owned()).collect::<Vec<_>>();
        let path = path
            .into_iter()
            .filter(|e| !e.is_empty())
            .collect::<Vec<_>>();
        let path = Path::new(path);

        let name = match &name[..] {
            "begin" => LocName::Begin,
            "end" => LocName::End,
            "expression" => LocName::Expression,
            "keyword" => LocName::Keyword,
            "name" => LocName::Name,
            _ => panic!("unsupported loc name {}", name),
        };

        Loc {
            begin,
            end,
            name,
            path,
        }
    }

    pub fn test(&self, node: &Node) -> Result<(), String> {
        let node = match self.path.go(node) {
            Some(node) => node,
            None => return Err(format!("Failed to get node {:?}", self.path)),
        };

        let loc = match (&self.name, node) {
            (LocName::Keyword, Node::CaseMatch { loc, .. }) => loc.keyword.clone(),
            (LocName::Keyword, Node::InPattern { loc, .. }) => Some(loc.keyword.clone()),

            (LocName::End, Node::CaseMatch { loc, .. }) => loc.end.clone(),

            (LocName::Name, Node::Lvar { loc, .. }) => loc.name.clone(),
            (LocName::Name, Node::MatchVar { loc, .. }) => loc.name.clone(),

            (LocName::Begin, Node::InPattern { loc, .. }) => loc.begin.clone(),
            (LocName::Begin, Node::Str { loc, .. }) => loc.begin.clone(),
            (LocName::Begin, Node::HashPattern { loc, .. }) => loc.begin.clone(),
            (LocName::Begin, Node::Super { loc, .. }) => loc.begin.clone(),
            (LocName::Begin, Node::KwBegin { loc, .. }) => loc.begin.clone(),
            (LocName::Begin, Node::Regexp { loc, .. }) => loc.begin.clone(),
            (LocName::Begin, Node::Block { loc, .. }) => loc.begin.clone(),
            (LocName::Begin, Node::Array { loc, .. }) => loc.begin.clone(),
            (LocName::Begin, Node::Hash { loc, .. }) => loc.begin.clone(),
            (LocName::Begin, Node::Send { loc, .. }) => loc.begin.clone(),
            (LocName::Begin, Node::While { loc, .. }) => loc.begin.clone(),
            (LocName::Begin, Node::Postexe { loc, .. }) => loc.begin.clone(),
            (LocName::Begin, Node::Preexe { loc, .. }) => loc.begin.clone(),
            (LocName::Begin, Node::Until { loc, .. }) => loc.begin.clone(),
            (LocName::Begin, Node::ConstPattern { loc, .. }) => loc.begin.clone(),
            (LocName::Begin, Node::Xstr { loc, .. }) => loc.begin.clone(),
            (LocName::Begin, Node::Dstr { loc, .. }) => loc.begin.clone(),
            (LocName::Begin, Node::Index { loc, .. }) => Some(loc.begin.clone()),
            (LocName::Begin, Node::Defined { loc, .. }) => loc.begin.clone(),
            (LocName::Begin, Node::ArrayPattern { loc, .. }) => loc.begin.clone(),
            (LocName::Begin, Node::ArrayPatternWithTail { loc, .. }) => loc.begin.clone(),
            (LocName::Begin, Node::Numblock { loc, .. }) => loc.begin.clone(),
            (LocName::Begin, Node::Sym { loc, .. }) => loc.begin.clone(),
            (LocName::Begin, Node::Begin { loc, .. }) => loc.begin.clone(),
            (LocName::Begin, Node::Yield { loc, .. }) => loc.begin.clone(),
            (LocName::Begin, Node::FindPattern { loc, .. }) => loc.begin.clone(),
            (LocName::Begin, Node::Dsym { loc, .. }) => loc.begin.clone(),

            (LocName::Expression, node) => Some(node.expression().clone()),
            (name, node) => {
                panic!("node {} has no {:?} loc", node.str_type(), name);
            }
        };

        match loc {
            Some(loc) => {
                if self.begin != loc.begin_pos {
                    return Err(format!(
                        "{:?} has incorrect {:?} begin pos: expected {}, got {}",
                        self.path, self.name, self.begin, loc.begin_pos
                    ));
                }

                if self.end != loc.end_pos {
                    return Err(format!(
                        "{:?} has incorrect {:?} end pos: expected {}, got {}",
                        self.path, self.name, self.begin, loc.begin_pos
                    ));
                }
            }
            None => return Err(format!("{:?} has no {:?} loc", self.path, self.name)),
        }

        Ok(())
    }
}

#[derive(Debug)]
struct Path {
    components: Vec<String>,
}

impl Path {
    pub fn new(components: Vec<String>) -> Self {
        Self { components }
    }

    pub fn go(&self, node: &Node) -> Option<Node> {
        if self.components.is_empty() {
            return Some(node.clone());
        }

        let first = self.components.first()?;
        let rest = self.components[1..].to_owned();

        let node: &Node = match (&first[..], node) {
            ("expr", Node::CaseMatch { expr, .. }) => expr,
            ("in_body[0]", Node::CaseMatch { in_bodies, .. }) => &in_bodies[0],

            ("pattern", Node::InPattern { pattern, .. }) => pattern,
            ("body", Node::InPattern { body, .. }) => body.as_ref()?,
            ("item[0]", Node::HashPattern { args, .. }) => &args[0],
            _ => panic!("can't go into {} on {:?}", first, node),
        };

        Path::new(rest).go(&node)
    }
}
