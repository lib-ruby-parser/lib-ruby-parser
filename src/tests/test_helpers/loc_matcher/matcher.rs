use lib_ruby_parser_ast::{write_to, Writer};

use super::LocName;
use crate::traverse::finder::Finder;
use crate::Node;

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
pub struct LocMatcher<'s> {
    begin: usize,
    end: usize,
    loc_name: LocName,
    pattern: &'s str,
}

impl<'s> LocMatcher<'s> {
    pub fn new(loc: &'s str) -> Self {
        let mut state = ParseLocState::SkipWs;
        let mut begin: Option<usize> = None;
        let mut end: Option<usize> = None;

        let mut name_start_at = None;
        let mut name_ends_at = None;

        let mut pattern_start_at = None;
        let mut pattern_ends_at = None;

        for (idx, c) in loc.bytes().enumerate() {
            match (&state, c) {
                (ParseLocState::SkipWs, b' ') => { /* skip */ }
                (ParseLocState::SkipWs, b'~') => {
                    state = ParseLocState::Cursor;
                    begin = Some(idx);
                }
                (ParseLocState::Cursor, b'~') => { /* keep reading */ }
                (ParseLocState::Cursor, b' ') => {
                    state = ParseLocState::Name;
                    name_start_at = Some(idx + 1);
                    end = Some(idx);
                }
                (ParseLocState::Name, b' ') => {
                    state = ParseLocState::Lparen;
                }
                (ParseLocState::Name, _) => {
                    name_ends_at = Some(idx + 1);
                }
                (ParseLocState::Lparen, b'(') => {
                    state = ParseLocState::Pattern;
                    pattern_start_at = Some(idx + 1);
                }
                (ParseLocState::Pattern, b')') => {
                    state = ParseLocState::Done;
                }
                (ParseLocState::Pattern, _) => {
                    pattern_ends_at = Some(idx + 1);
                }
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

        let name = LocName::new(&loc[name_start_at.unwrap()..name_ends_at.unwrap()]);

        let pattern = &loc[pattern_start_at.unwrap()..pattern_ends_at.unwrap()];

        LocMatcher {
            begin,
            end,
            loc_name: name,
            pattern,
        }
    }

    pub fn test<'b>(&self, root: &'b Node<'b>) -> Result<(), [u8; 200]> {
        let mut err_buf = [0; 200];

        match Finder::run(&self.pattern, root).unwrap() {
            Some(node) => match self.loc_name.get(&node).as_ref() {
                Some(loc) => {
                    if loc.begin() != self.begin {
                        write_to(
                            &mut err_buf,
                            format_args!(
                                "begin of {:?} - {:?} doesn't match, expected {}, got {}",
                                self.pattern,
                                self.loc_name,
                                self.begin,
                                loc.begin()
                            ),
                        )
                        .unwrap();
                        return Err(err_buf);
                    }

                    if loc.end() != self.end {
                        write_to(
                            &mut err_buf,
                            format_args!(
                                "end of {:?} - {:?} doesn't match, expected {}, got {}",
                                self.pattern,
                                self.loc_name,
                                self.end,
                                loc.end()
                            ),
                        )
                        .unwrap();
                        return Err(err_buf);
                    }

                    Ok(())
                }
                None => {
                    write_to(
                        &mut err_buf,
                        format_args!(
                            "failed to get {:?} of {:?} on {:?}",
                            self.loc_name, self.pattern, root
                        ),
                    )
                    .unwrap();
                    Err(err_buf)
                }
            },
            None => {
                let mut buf = [0; 1000];
                let mut writer = Writer::new(&mut buf);
                root.inspect(0, &mut writer).unwrap();
                let inspected = writer.as_str().unwrap_or("<nothing>");
                panic!("failed to find {:?} in {}", self.pattern, inspected);
            }
        }
    }
}
