use ruby_parser::{Parser, ParserOptions, ParserResult};
use std::fs;
use std::panic;

mod files_under_dir;
use files_under_dir::files_under_dir;

mod loc_matcher;
use loc_matcher::LocMatcher;

mod diagnostic_matcher;
use diagnostic_matcher::DiagnosticMatcher;

enum TestSection {
    None,
    Input,
    AST,
    Locations,
    Diagnostic,
}

#[derive(Debug)]
struct Fixture {
    input: String,
    ast: Option<String>,
    locs: Option<Vec<String>>,
    diagnostic: Option<String>,
}

fn none_if_empty<T: PartialEq<&'static str>>(v: Vec<T>) -> Option<Vec<T>> {
    if v.is_empty() || (v.len() == 1 && v[0] == "") {
        None
    } else {
        Some(v)
    }
}

impl Fixture {
    fn new(path: &str) -> Self {
        let content =
            fs::read_to_string(path).unwrap_or_else(|_| panic!("failed to read file {:?}", path));

        let mut input: Vec<String> = vec![];
        let mut ast: Vec<String> = vec![];
        let mut locs: Vec<String> = vec![];
        let mut diagnostics: Vec<String> = vec![];
        let mut current_section = TestSection::None;

        for line in content.lines() {
            match (line.as_bytes(), &current_section) {
                (&[b'/', b'/', b' ', ..], _) => { /* skip comment */ }
                (b"--INPUT", _) => current_section = TestSection::Input,
                (b"--AST", _) => current_section = TestSection::AST,
                (b"--LOCATIONS", _) => current_section = TestSection::Locations,
                (b"--DIAGNOSTIC", _) => current_section = TestSection::Diagnostic,
                (_, &TestSection::Input) => input.push(line.to_owned()),
                (_, &TestSection::AST) => ast.push(line.to_owned()),
                (_, &TestSection::Locations) => locs.push(line.to_owned()),
                (_, &TestSection::Diagnostic) => diagnostics.push(line.to_owned()),
                (_, &TestSection::None) => {
                    panic!("empty state while parsing fixture on line {:#?}", line)
                }
            }
        }

        let input = input.join("\n");
        let ast = none_if_empty(ast).map(|lines| lines.join("\n"));
        let locs = none_if_empty(locs);
        let diagnostic = match diagnostics.len() {
            1 => diagnostics.pop(),
            0 => None,
            _ => panic!("only one diagnostic per file is supported"),
        };

        Self {
            input,
            ast,
            locs,
            diagnostic,
        }
    }

    fn compare(&self, actual: &ParserResult) -> Result<(), String> {
        match &self.ast {
            Some(expected_ast) => {
                let actual_ast = actual
                    .ast
                    .as_ref()
                    .map(|node| node.inspect(0))
                    .unwrap_or_else(|| "nil".to_owned());

                if &actual_ast != expected_ast {
                    println!("{:?}", self.input);
                    return Err(format!(
                        "AST diff:\nactual:\n{}\nexpected:\n{}\n",
                        actual_ast, expected_ast
                    ));
                }
            }
            None => {}
        }

        match &self.locs {
            Some(locs) => {
                let ast = actual
                    .ast
                    .as_ref()
                    .ok_or_else(|| "can't compare locs, ast is empty".to_owned())?;

                for loc in locs {
                    LocMatcher::new(loc).test(ast)?
                }
            }
            None => {}
        }

        match &self.diagnostic {
            Some(diagnostic) => {
                let actual =
                    match actual.diagnostics.len() {
                        1 => actual.diagnostics[0].clone(),
                        0 => {
                            return Err(format!(
                                "expected diagnostic {:?} to be emitted",
                                diagnostic
                            ))
                        }
                        _ => return Err(
                            "your input returns multiple diagnostics, don't know how to match them"
                                .to_owned(),
                        ),
                    };
                DiagnosticMatcher::new(diagnostic)?.test(&actual)?
            }
            None => {}
        }

        Ok(())
    }
}

enum TestResult {
    Segfault,
    Pass,
    Failure(String),
}

fn test_file(fixture_path: &str) -> TestResult {
    let result = panic::catch_unwind(|| {
        let test_case = Fixture::new(fixture_path);

        let options = ParserOptions {
            buffer_name: format!("(test {})", fixture_path),
            debug: false,
            ..Default::default()
        };
        let mut parser =
            Parser::new(test_case.input.as_bytes(), options).expect("failed to construct parser");

        parser.static_env.declare("foo");
        parser.static_env.declare("bar");
        parser.static_env.declare("baz");

        let result = parser.do_parse();

        test_case.compare(&result)
    });

    match result {
        Err(_) => TestResult::Segfault,
        Ok(Err(output)) => TestResult::Failure(output),
        Ok(Ok(_)) => TestResult::Pass,
    }
}

fn test_dir(dir: &str) {
    eprintln!("Running parser tests {}\n", dir);

    let mut passed: usize = 0;
    let mut failed: usize = 0;
    let mut segfaults: usize = 0;

    for filename in files_under_dir(dir) {
        eprint!("test {} ... ", filename);
        match test_file(&filename) {
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
        }
    }

    eprintln!(
        "{} tests passed, {} failed, {} segfaults",
        passed, failed, segfaults
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
    test_dir("tests/fixtures/parser/gen")
}

#[test]
fn test_manual() {
    test_dir("tests/fixtures/parser/manual")
}
