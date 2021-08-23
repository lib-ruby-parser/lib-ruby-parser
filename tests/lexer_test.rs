use lib_ruby_parser::lex_states::*;
use lib_ruby_parser::{source::Decoder, Lexer};
use std::fs;
use std::panic;

mod files_under_dir;
use files_under_dir::files_under_dir;

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

enum TestResult {
    Segfault,
    Pass,
    #[allow(dead_code)]
    Skip,
    Failure(String),
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

fn test(fixture_path: &str) -> TestResult {
    let result = panic::catch_unwind(|| {
        let test_case = Fixture::new(fixture_path);

        let mut lexer = Lexer::new(
            test_case.input.as_str(),
            format!("(test {})", fixture_path),
            Decoder::none(),
        );
        for var in test_case.vars {
            lexer.static_env.declare(&var);
        }
        if let Some(state) = test_case.state {
            lexer.lex_state.set(lex_state(&state).unwrap());
        }
        if test_case.cond {
            lexer.cond.push(true)
        }
        if test_case.cmdarg {
            lexer.cmdarg.push(true)
        }
        lexer.set_debug(lib_ruby_parser::debug_level::NONE);
        let tokens = lexer.tokenize_until_eof();
        let tokens = tokens
            .iter()
            .map(|token| {
                format!(
                    "{} {:?} [{}, {}]",
                    token.token_name(),
                    token.to_string_lossy(),
                    token.loc().begin(),
                    token.loc().end()
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        if tokens == test_case.tokens {
            TestResult::Pass
        } else {
            TestResult::Failure(format!(
                "actual:\n{}\nexpected:\n{}\n",
                tokens, test_case.tokens
            ))
        }
    });

    match result {
        Ok(test_result) => test_result,
        Err(_) => TestResult::Segfault,
    }
}

fn test_dir(dir: &str) {
    eprintln!("Running parser tests\n");

    let mut passed: usize = 0;
    let mut failed: usize = 0;
    let mut segfaults: usize = 0;
    let mut skipped: usize = 0;

    for filename in files_under_dir(dir) {
        eprint!("test {} ... ", filename);
        match test(&filename) {
            TestResult::Segfault => {
                eprintln!("SEG");
                segfaults += 1;
            }
            TestResult::Pass => {
                eprintln!("OK");
                passed += 1;
            }
            TestResult::Failure(output) => {
                eprintln!("Err:\n{}\n", output);
                failed += 1;
            }
            TestResult::Skip => {
                eprintln!("Skip");
                skipped += 1;
            }
        }
    }

    eprintln!(
        "{} tests passed, {} failed, {} segfaults, {} skipped",
        passed, failed, segfaults, skipped
    );

    assert_eq!(
        failed + segfaults,
        0,
        "expected tests to pass, got {} failures and {} segfaults",
        failed,
        segfaults
    );
}

#[test]
fn test_gen() {
    test_dir("tests/fixtures/lexer/gen")
}

#[test]
fn test_manual() {
    test_dir("tests/fixtures/lexer/manual")
}
