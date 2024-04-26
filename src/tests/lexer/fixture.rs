use lib_ruby_parser_ast::Blob;
use lib_ruby_parser_ast::ByteArray;

use crate::lex_states::*;
use crate::tests::test_helpers::InlineArray;
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
struct Fixture<'b> {
    cond: bool,
    cmdarg: bool,
    vars: InlineArray<10, &'b str>,
    state: Option<String>,
    input: String,
    tokens: String,
}
impl<'b> Fixture<'b> {
    fn new(path: &str, blob: &'b Blob<'b>) -> Self {
        let content =
            fs::read_to_string(path).unwrap_or_else(|_| panic!("failed to read fixture {}", path));

        let mut vars = InlineArray::new();
        let mut input: Vec<String> = vec![];
        let mut tokens: Vec<String> = vec![];
        let mut state: Option<String> = None;
        let mut current_section = TestSection::None;
        let mut cond = false;
        let mut cmdarg = false;

        for line in content.lines() {
            match (line.as_bytes(), &current_section) {
                (&[b'/', b'/', b' ', ..], _) => { /* skip comment */ }

                (b"--COND", _) => cond = true,
                (b"--CMDARG", _) => cmdarg = true,
                (b"--VARS", _) => current_section = TestSection::Vars,
                (b"--STATE", _) => current_section = TestSection::State,
                (b"--INPUT", _) => current_section = TestSection::Input,
                (b"--TOKENS", _) => current_section = TestSection::Tokens,
                (_, &TestSection::Vars) => {
                    for var in line.split(' ') {
                        let var_b = ByteArray::new(blob);
                        var_b.push_str(var, blob);
                        vars.push(var_b.try_as_str().unwrap());
                    }
                }
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
    let mut mem = vec![0; 1000];
    let blob = Blob::from(mem.as_mut_slice());

    let fixture = Fixture::new(fixture_path, &blob);

    let buffer_name = format!("(test {})", fixture_path);

    let mut lexer = Lexer::new(
        fixture.input.as_bytes(),
        {
            let name = ByteArray::new(&blob);
            name.push_str(&buffer_name, &blob);
            name.try_as_str().unwrap()
        },
        None,
        &blob,
    );
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
    let tokens = tokens
        .iter()
        .map(|token| {
            format!(
                "{} {:?} [{}, {}]",
                token.token_name(),
                token.token_value.try_as_str().unwrap_or_default(),
                token.loc.begin(),
                token.loc.end()
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
