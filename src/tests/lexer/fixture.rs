use lib_ruby_parser_ast::Blob;

use crate::lex_states::*;
use crate::tests::test_helpers::InlineArray;
use crate::Lexer;

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
    vars: InlineArray<10, &'static str>,
    state: Option<&'static str>,
    input: &'static str,
    tokens: &'static str,
}

impl Fixture {
    fn new(src: &'static str) -> Self {
        let mut vars = InlineArray::new();
        let mut input_starts_at = None;
        let mut input_ends_at = None;
        let mut tokens_start_at = None;
        let mut tokens_end_at = None;
        let mut state = None;
        let mut current_section = TestSection::None;
        let mut cond = false;
        let mut cmdarg = false;
        let mut pos = 0;

        for line in src.lines() {
            pos += line.len() + 1;

            match (line.as_bytes(), &current_section) {
                (&[b'/', b'/', b' ', ..], _) => { /* skip comment */ }

                (b"--COND", _) => cond = true,
                (b"--CMDARG", _) => cmdarg = true,
                (b"--VARS", _) => current_section = TestSection::Vars,
                (b"--STATE", _) => current_section = TestSection::State,
                (b"--INPUT", _) => {
                    current_section = TestSection::Input;
                    input_starts_at = Some(pos);
                }
                (b"--TOKENS", _) => {
                    current_section = TestSection::Tokens;
                    tokens_start_at = Some(pos);
                }
                (_, &TestSection::Vars) => {
                    for var in line.split(' ') {
                        vars.push(var);
                    }
                }
                (_, &TestSection::State) => state = Some(line),
                (_, &TestSection::Input) => {
                    input_ends_at = Some(pos - 1);
                }
                (_, &TestSection::Tokens) => {
                    // tokens.push(line.to_string())
                    tokens_end_at = Some(pos - 1);
                }
                (_, &TestSection::None) => {
                    panic!("empty state while parsing fixture on line {:#?}", line)
                }
            }
        }

        let input = &src[input_starts_at.unwrap()..input_ends_at.unwrap()];
        let tokens = &src[tokens_start_at.unwrap()..tokens_end_at.unwrap()];

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

pub(crate) fn test_file(test_name: &'static str, src: &'static str) {
    let mut mem = [0; 1000];
    let blob = Blob::from(&mut mem);
    let mut scratch = [0; 1000];
    let scratch = Blob::from(&mut scratch);

    let fixture = Fixture::new(src);

    let mut lexer = Lexer::new(fixture.input.as_bytes(), test_name, None, &blob, &scratch);
    for var in fixture.vars.iter() {
        lexer.static_env.declare(var, &blob);
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

    let mut buf = [0; 1000];
    let mut writer = lib_ruby_parser_ast::Writer::new(&mut buf);
    for (idx, token) in tokens.iter().enumerate() {
        use core::fmt::Write;
        if idx != 0 {
            write!(&mut writer, "\n").unwrap();
        }
        write!(
            &mut writer,
            "{} {:?} [{}, {}]",
            token.token_name(),
            token.token_value.try_as_str().unwrap_or_default(),
            token.loc.begin(),
            token.loc.end()
        )
        .unwrap();
    }
    let tokens = writer.as_str().unwrap();
    assert_eq!(
        tokens, fixture.tokens,
        "actual:\n{}\nexpected:\n{}\n",
        tokens, fixture.tokens
    );
}
