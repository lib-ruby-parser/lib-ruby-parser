#![feature(custom_test_frameworks)]
#![test_runner(runner)]

use ruby_parser::{Node, Parser, Lexer};
use std::panic;
use std::fs;
use std::process::exit;

enum TestSection {
    None,
    Input,
    AST,
}

#[derive(Debug)]
struct TestCase {
    input: String,
    ast: String
}

impl TestCase {
    fn new(path: &str) -> Self {
        let content = fs::read_to_string(path).unwrap();

        let mut input: Vec<String> = vec![];
        let mut ast: Vec<String> = vec![];
        let mut current_section = TestSection::None;

        for line in content.lines() {
            match (line, &current_section) {
                ("--INPUT", _) => current_section = TestSection::Input,
                ("--AST",   _) => current_section = TestSection::AST,
                (_, &TestSection::Input) => input.push(line.to_owned()),
                (_, &TestSection::AST)   => ast.push(line.to_owned()),
                (_, &TestSection::None)  => panic!("empty state while parsing fixture on line {:#?}", line)
            }
        }

        let input = input.join("\n");
        let ast = ast.join("\n");

        Self { input, ast }
    }
}

enum TestResult {
    Segfault,
    Pass,
    Failure(String)
}

fn test(fixture_path: &str) -> TestResult {
    let result = panic::catch_unwind(|| {
        let test_case = TestCase::new(fixture_path);
        let mut parser = Parser::new(Lexer::new(&test_case.input));
        parser.static_env.declare("foo");
        parser.static_env.declare("bar");
        parser.static_env.declare("baz");
        parser.set_debug(false);
        let ast = parser.do_parse().unwrap().inspect(0);

        if ast == test_case.ast {
            Ok(())
        } else {
            Err(format!("actual:\n{:?}\nexpected:\n{:?}\n", ast, test_case.ast))
        }
    });

    match result {
        Err(_) => TestResult::Segfault,
        Ok(Err(output)) => TestResult::Failure(output),
        Ok(Ok(_)) => TestResult::Pass
    }
}

fn runner(dirs: &[&'static str]) {
    eprintln!("Running parser tests\n");

    let mut passed: usize = 0;
    let mut failed: usize = 0;
    let mut segfaults: usize = 0;

    for dir in dirs {
        let tests = fs::read_dir(dir).expect(&format!("{} doesn't exist", dir))
            .map(|res| res.unwrap().path())
            // .filter(|path| path.extension().unwrap() == "in" )
            // .map(|mut path| { path.set_extension(""); path })
            .map(|path| path.to_str().unwrap().to_owned())
            // .map(|path| path.file_name().unwrap().to_str().unwrap().to_owned() )
            .collect::<Vec<_>>();

        for filename in tests {
            eprint!("test {} ... ", filename);
            match test(&filename) {
                TestResult::Segfault => {
                    eprintln!("SEG");
                    segfaults += 1;
                },
                TestResult::Pass => {
                    eprintln!("OK");
                    passed += 1;
                },
                TestResult::Failure(output) => {
                    eprintln!("Err:\n{}\n", output);
                    failed += 1;
                }
            }
        }
    }

    eprintln!("{} tests passed, {} failed, {} segfaults", passed, failed, segfaults);
    match failed + segfaults {
        0 => exit(0),
        _ => exit(1)
    }
}

#[test_case]
const GENERATED_TESTS_DIR: &'static str = "tests/fixtures/parser/gen";
