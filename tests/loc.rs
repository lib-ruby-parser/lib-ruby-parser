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
}

#[derive(Debug)]
pub struct Loc {
    begin: usize,
    end: usize,
    name: LocName,
    pattern: Vec<String>,
}

impl Loc {
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
            .split("/")
            .filter(|e| !e.is_empty())
            .map(|e| e.to_owned())
            .collect::<Vec<_>>();
        let name = LocName::new(&name);

        Loc {
            begin,
            end,
            name,
            pattern,
        }
    }

    pub fn test(&self, node: &Node) -> Result<(), String> {
        match Find::run(&self.pattern, node) {
            Some(node) => Ok(()),
            None => panic!("failed to find {:?} in {:?}", self.pattern, node.inspect(0)),
        }
    }
}
