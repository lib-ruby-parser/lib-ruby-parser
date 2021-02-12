use super::LocName;
use lib_ruby_parser::traverse::finder::Finder;
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
pub struct LocMatcher {
    begin: usize,
    end: usize,
    loc_name: LocName,
    pattern: String,
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

        let name = LocName::new(&name);

        LocMatcher {
            begin,
            end,
            loc_name: name,
            pattern,
        }
    }

    pub fn test(&self, root: &Node) -> Result<(), String> {
        match Finder::run(&self.pattern, root).unwrap() {
            Some(node) => match self.loc_name.get(&node) {
                Some(loc) => {
                    if loc.begin != self.begin {
                        return Err(format!(
                            "begin of {:?} - {:?} doesn't match, expected {}, got {}",
                            self.pattern, self.loc_name, self.begin, loc.begin
                        ));
                    }

                    if loc.end != self.end {
                        return Err(format!(
                            "end of {:?} - {:?} doesn't match, expected {}, got {}",
                            self.pattern, self.loc_name, self.end, loc.end
                        ));
                    }

                    Ok(())
                }
                None => Err(format!(
                    "failed to get {:?} of {:?} on {:?}",
                    self.loc_name, self.pattern, root
                )),
            },
            None => panic!("failed to find {:?} in {:?}", self.pattern, root.inspect(0)),
        }
    }
}
