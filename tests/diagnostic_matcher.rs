use ruby_parser::{Diagnostic, ErrorLevel};

#[derive(Debug)]
enum State {
    None,
    MaybeTildes,
    Lparen,
    ErrorLevel,
    SpaceBeforeMessage,
    Message,
}

#[derive(Debug)]
pub struct DiagnosticMatcher {
    begin: usize,
    end: usize,
    level: ErrorLevel,
    message: String,
}

impl DiagnosticMatcher {
    pub fn new(s: &str) -> Result<Self, String> {
        let mut state = State::None;

        let mut begin: usize = std::usize::MAX;
        let mut end: usize = std::usize::MAX;
        let mut level = String::from("");
        let mut message = String::from("");

        for (idx, c) in s.chars().enumerate() {
            match (&state, c) {
                (&State::None, ' ') => {}
                (&State::None, '~') => {
                    begin = idx;
                    state = State::MaybeTildes;
                }
                (&State::MaybeTildes, '~') => {}
                (&State::MaybeTildes, ' ') => {
                    end = idx;
                    state = State::Lparen
                }
                (&State::Lparen, '(') => state = State::ErrorLevel,
                (&State::ErrorLevel, ')') => state = State::SpaceBeforeMessage,
                (&State::ErrorLevel, c) => level.push(c),
                (&State::SpaceBeforeMessage, ' ') => state = State::Message,
                (&State::Message, c) => message.push(c),
                (state, c) => panic!("can't interpret char {:?} with state {:?}", c, state),
            }
        }

        let level = match &level[..] {
            "error" => ErrorLevel::Error,
            "warning" => ErrorLevel::Warning,
            other => return Err(format!("unknown error level {:?}", other)),
        };

        Ok(Self {
            begin,
            end,
            level,
            message,
        })
    }

    pub fn test(&self, actual: &Diagnostic) -> Result<(), String> {
        compare(&self.message, &actual.render_message(), "message")?;
        compare(&self.level, &actual.level, "level")?;
        compare(&self.begin, &actual.range.begin_pos, "begin")?;
        compare(&self.end, &actual.range.end_pos, "end")?;

        Ok(())
    }
}

fn compare<T: PartialEq + std::fmt::Debug>(
    expected: &T,
    actual: &T,
    name: &str,
) -> Result<(), String> {
    if expected != actual {
        return Err(format!(
            "{} doesn't match: expected {:?}, got {:?}",
            name, expected, actual
        ));
    }
    Ok(())
}
