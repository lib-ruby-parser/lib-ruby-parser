use lib_ruby_parser::{
    source::MagicComment, source::MagicCommentKind, source::Range, Parser, ParserOptions,
    ParserResult,
};
use std::fs;
use std::panic;

mod files_under_dir;
use files_under_dir::files_under_dir;

mod loc_matcher;
use loc_matcher::LocMatcher;

mod diagnostic_matcher;
use diagnostic_matcher::render_diagnostic_for_testing;

enum TestSection {
    None,
    Input,
    AST,
    Locations,
    Diagnostic,
    SkipIfFeatureEnabled,
}

#[derive(Debug)]
struct Fixture {
    input: String,
    ast: Option<String>,
    locs: Option<Vec<String>>,
    diagnostics: Option<Vec<String>>,
    skip_if_feature_enabled: Option<String>,
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
        let mut skip_if_feature_enabled: Option<String> = None;
        let mut current_section = TestSection::None;

        for line in content.lines() {
            match (line.as_bytes(), &current_section) {
                (&[b'/', b'/', b' ', ..], _) => { /* skip comment */ }

                (b"--INPUT", _) => current_section = TestSection::Input,
                (b"--AST", _) => current_section = TestSection::AST,
                (b"--LOCATIONS", _) => current_section = TestSection::Locations,
                (b"--DIAGNOSTIC", _) => current_section = TestSection::Diagnostic,
                (b"--SKIP-IF-FEATURE-ENABLED", _) => {
                    current_section = TestSection::SkipIfFeatureEnabled
                }

                (_, &TestSection::Input) => input.push(line.to_owned()),
                (_, &TestSection::AST) => ast.push(line.to_owned()),
                (_, &TestSection::Locations) => locs.push(line.to_owned()),
                (_, &TestSection::Diagnostic) => diagnostics.push(line.to_owned()),
                (_, &TestSection::SkipIfFeatureEnabled) => {
                    skip_if_feature_enabled = Some(line.to_owned())
                }

                (_, &TestSection::None) => {
                    panic!("empty state while parsing fixture on line {:#?}", line)
                }
            }
        }

        let input = input.join("\n");
        let ast = none_if_empty(ast).map(|lines| lines.join("\n"));
        let locs = none_if_empty(locs);
        let diagnostics = none_if_empty(diagnostics);

        match (&ast, &locs, &diagnostics) {
            (None, None, None) => panic!("empty test"),
            _ => {}
        }

        Self {
            input,
            ast,
            locs,
            diagnostics,
            skip_if_feature_enabled,
        }
    }

    fn compare(&self, actual: &ParserResult) -> TestOutput {
        match &self.ast {
            Some(expected_ast) => {
                let actual_ast = actual
                    .ast
                    .as_ref()
                    .map(|node| node.inspect(0))
                    .unwrap_or_else(|| "nil".to_owned());

                if &actual_ast != expected_ast {
                    println!("{:?}", self.input);
                    return TestOutput::Failure(format!(
                        "AST diff:\nactual:\n{}\nexpected:\n{}\n",
                        actual_ast, expected_ast
                    ));
                }
            }
            None => {}
        }

        match &self.locs {
            Some(locs) => {
                let ast = if let Some(ast) = actual.ast.as_ref() {
                    ast
                } else {
                    return TestOutput::Failure("can't compare locs, ast is empty".to_owned());
                };

                for loc in locs {
                    match LocMatcher::new(loc).test(ast) {
                        Ok(_) => {}
                        Err(err) => return TestOutput::Failure(err),
                    }
                }
            }
            None => {}
        }

        let actual_diagnostics = actual
            .diagnostics
            .iter()
            .map(|d| render_diagnostic_for_testing(d))
            .collect::<Vec<_>>();

        match &self.diagnostics {
            None => {
                if actual.diagnostics.len() == 0 {
                    // ok
                } else {
                    return TestOutput::Failure(format!(
                        "expected no diagnostics to be emitted, got:\n{}",
                        actual_diagnostics.join("\n")
                    ));
                }
            }
            Some(diagnostics) => {
                let expected = diagnostics;
                let actual = actual_diagnostics;

                if expected == &actual {
                    // ok
                } else {
                    return TestOutput::Failure(format!(
                        "expected diagnostcs:\n{}\nactual diagnostics:\n{}",
                        expected.join("\n"),
                        actual.join("\n")
                    ));
                }
            }
        }

        TestOutput::Pass
    }
}

enum TestOutput {
    Pass,
    Failure(String),
}

enum TestResult {
    Segfault,
    Some(TestOutput),
    Skip,
}

fn test_file(fixture_path: &str) -> TestResult {
    let result = panic::catch_unwind(|| {
        let test_case = Fixture::new(fixture_path);

        match &test_case.skip_if_feature_enabled {
            Some(feature) if feature == "lsp-error-recovery" => {
                if cfg!(feature = "lsp-error-recovery") {
                    return TestResult::Skip;
                }
            }
            Some(unknown) => {
                return TestResult::Some(TestOutput::Failure(format!(
                    "unknown --SKIP-IF-ENABLED feature {:?}",
                    unknown
                )));
            }
            _ => {}
        }

        let options = ParserOptions {
            buffer_name: format!("(test {})", fixture_path),
            debug: false,
            ..Default::default()
        };
        let parser = Parser::new(test_case.input.as_bytes(), options);

        parser.static_env.declare("foo");
        parser.static_env.declare("bar");
        parser.static_env.declare("baz");

        let result = if test_case.diagnostics.is_some() {
            parser.do_parse()
        } else {
            parser.do_parse_with_state_validation()
        };

        TestResult::Some(test_case.compare(&result))
    });

    match result {
        Err(_) => TestResult::Segfault,
        Ok(output) => output,
    }
}

fn test_dir(dir: &str) {
    eprintln!("Running parser tests {}\n", dir);

    let mut passed: usize = 0;
    let mut failed: usize = 0;
    let mut segfaults: usize = 0;
    let mut skipped: usize = 0;

    for filename in files_under_dir(dir) {
        eprint!("test {} ... ", filename);
        match test_file(&filename) {
            TestResult::Segfault => {
                eprintln!("SEG");
                segfaults += 1;
            }
            TestResult::Some(TestOutput::Pass) => {
                eprintln!("OK");
                passed += 1;
            }
            TestResult::Some(TestOutput::Failure(output)) => {
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
    test_dir("tests/fixtures/parser/gen")
}

#[test]
fn test_manual() {
    test_dir("tests/fixtures/parser/manual")
}

fn read_fixture(path: &str) -> Vec<u8> {
    fs::read(path).unwrap()
}

fn parse(input: &[u8]) -> ParserResult {
    let options = ParserOptions {
        buffer_name: "(eval)".to_string(),
        debug: false,
        ..Default::default()
    };
    let parser = Parser::new(input, options);
    parser.do_parse()
}

#[test]
fn test_magic_comment() {
    let result = parse(&read_fixture("tests/fixtures/magic_comments.rb"));

    assert_eq!(
        result.magic_comments,
        vec![
            MagicComment {
                kind: MagicCommentKind::Encoding,
                key_l: Range::new(2, 10),
                value_l: Range::new(12, 17),
            },
            MagicComment {
                kind: MagicCommentKind::FrozenStringLiteral,
                key_l: Range::new(20, 41),
                value_l: Range::new(43, 47),
            },
            MagicComment {
                kind: MagicCommentKind::Encoding,
                key_l: Range::new(50, 56),
                value_l: Range::new(58, 63),
            },
            MagicComment {
                kind: MagicCommentKind::ShareableContstantValue,
                key_l: Range::new(66, 90),
                value_l: Range::new(92, 99),
            },
            MagicComment {
                kind: MagicCommentKind::WarnIndent,
                key_l: Range::new(102, 113),
                value_l: Range::new(115, 119),
            },
        ]
    );
}
