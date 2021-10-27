use crate::lex_states::*;
use crate::Lexer;
use std::fs;
use std::panic;

enum TestSection {
    None,
    Vars,
    State,
    Input,
    Tokens,
}

#[derive(Debug)]
struct Fixture {
    cond: bool,
    cmdarg: bool,
    vars: Vec<String>,
    state: Option<String>,
    input: String,
    tokens: String,
}
impl Fixture {
    fn new(path: &str) -> Self {
        let content =
            fs::read_to_string(path).unwrap_or_else(|_| panic!("failed to read fixture {}", path));

        let mut vars: Vec<String> = vec![];
        let mut input: Vec<String> = vec![];
        let mut tokens: Vec<String> = vec![];
        let mut state: Option<String> = None;
        let mut current_section = TestSection::None;
        let mut cond = false;
        let mut cmdarg = false;

        for line in content.lines() {
            match (line, &current_section) {
                ("--COND", _) => cond = true,
                ("--CMDARG", _) => cmdarg = true,
                ("--VARS", _) => current_section = TestSection::Vars,
                ("--STATE", _) => current_section = TestSection::State,
                ("--INPUT", _) => current_section = TestSection::Input,
                ("--TOKENS", _) => current_section = TestSection::Tokens,
                (_, &TestSection::Vars) => vars = line.split(' ').map(|s| s.to_string()).collect(),
                (_, &TestSection::State) => state = Some(line.to_string()),
                (_, &TestSection::Input) => input.push(line.to_string()),
                (_, &TestSection::Tokens) => tokens.push(line.to_string()),
                (_, &TestSection::None) => {
                    panic!("empty state while parsing fixture on line {:#?}", line)
                }
            }
        }

        let input = input.join("\n");
        let tokens = tokens.join("\n");

        Self {
            cond,
            cmdarg,
            vars,
            state,
            input,
            tokens,
        }
    }
}

fn lex_state(state: &str) -> Result<i32, &'static str> {
    let result = match state {
        "expr_arg" => EXPR_ARG,
        "expr_beg" => EXPR_BEG,
        "expr_mid" => EXPR_MID,
        "expr_end" => EXPR_END,
        "expr_fname" => EXPR_FNAME,
        "expr_value" => EXPR_VALUE,
        "expr_dot" => EXPR_DOT,
        "expr_endfn" => EXPR_ENDFN,
        "expr_endarg" => EXPR_ENDARG,
        "expr_cmdarg" => EXPR_CMDARG,
        _ => return Err("Unknown lex state {}"),
    };
    Ok(result)
}

pub(crate) fn test_file(fixture_path: &str) {
    let fixture = Fixture::new(fixture_path);

    let mut lexer = Lexer::new(
        fixture.input.as_str(),
        format!("(test {})", fixture_path),
        None,
    );
    for var in fixture.vars {
        lexer.static_env.declare(&var);
    }
    if let Some(state) = fixture.state {
        lexer.lex_state.set(lex_state(&state).unwrap());
    }
    if fixture.cond {
        lexer.cond.push(true)
    }
    if fixture.cmdarg {
        lexer.cmdarg.push(true)
    }
    let tokens = lexer.tokenize_until_eof();
    let tokens = tokens
        .iter()
        .map(|token| {
            format!(
                "{} {:?} [{}, {}]",
                token.token_name(),
                token.to_string_lossy(),
                token.loc.begin,
                token.loc.end
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    assert_eq!(
        tokens, fixture.tokens,
        "actual:\n{}\nexpected:\n{}\n",
        tokens, fixture.tokens
    );
}
