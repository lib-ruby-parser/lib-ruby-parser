#![feature(custom_test_frameworks)]
#![test_runner(runner)]

use ruby_parser::{Node, Parser};
use std::fs;
use std::panic;
use std::process::exit;

mod loc;
use loc::Loc;

enum TestSection {
    None,
    Input,
    AST,
    Locations,
}

#[derive(Debug)]
struct TestCase {
    input: String,
    ast: String,
    locs: Vec<String>,
}

impl TestCase {
    fn new(path: &str) -> Self {
        let content = fs::read_to_string(path).unwrap();

        let mut input: Vec<String> = vec![];
        let mut ast: Vec<String> = vec![];
        let mut locs: Vec<String> = vec![];
        let mut current_section = TestSection::None;

        for line in content.lines() {
            match (line, &current_section) {
                ("--INPUT", _) => current_section = TestSection::Input,
                ("--AST", _) => current_section = TestSection::AST,
                ("--LOCATIONS", _) => current_section = TestSection::Locations,
                (_, &TestSection::Input) => input.push(line.to_owned()),
                (_, &TestSection::AST) => ast.push(line.to_owned()),
                (_, &TestSection::Locations) => locs.push(line.to_owned()),
                (_, &TestSection::None) => {
                    panic!("empty state while parsing fixture on line {:#?}", line)
                }
            }
        }

        let input = input.join("\n");
        let ast = ast.join("\n");

        Self { input, ast, locs }
    }
}

enum TestResult {
    Segfault,
    Pass,
    Failure(String),
}

fn match_locs(locs: Vec<String>, ast: Node) -> Result<(), String> {
    for loc in locs {
        Loc::new(&loc).test(&ast)?
    }
    Ok(())
}

fn test(fixture_path: &str) -> TestResult {
    let result = panic::catch_unwind(|| {
        let test_case = TestCase::new(fixture_path);
        let mut parser = Parser::new(
            &test_case.input.as_bytes().to_vec(),
            &format!("(test {})", fixture_path),
        )
        .unwrap();
        parser.static_env.declare("foo");
        parser.static_env.declare("bar");
        parser.static_env.declare("baz");
        parser.set_debug(false);

        let ast = parser.do_parse();

        let ast_output = ast
            .as_ref()
            .map(|node| node.inspect(0))
            .unwrap_or("nil".to_owned());
        if ast_output != test_case.ast {
            println!("{:?}", test_case.input);
            return Err(format!(
                "AST diff:\nactual:\n{}\nexpected:\n{}\n",
                ast_output, test_case.ast
            ));
        }

        match ast {
            Some(ast) => match_locs(test_case.locs, ast)?,
            None => {}
        }

        Ok(())
    });

    match result {
        Err(_) => TestResult::Segfault,
        Ok(Err(output)) => TestResult::Failure(output),
        Ok(Ok(_)) => TestResult::Pass,
    }
}

fn files_under_dir(dir: &str) -> Vec<String> {
    fs::read_dir(dir)
        .expect(&format!("{} doesn't exist", dir))
        .map(|res| res.unwrap().path())
        .map(|path| path.to_str().unwrap().to_owned())
        .collect::<Vec<_>>()
}

fn runner(dirs: &[&'static str]) {
    eprintln!("Running parser tests\n");

    let mut passed: usize = 0;
    let mut failed: usize = 0;
    let mut segfaults: usize = 0;

    for dir in dirs {
        for filename in files_under_dir(*dir) {
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
            }
        }
    }

    eprintln!(
        "{} tests passed, {} failed, {} segfaults",
        passed, failed, segfaults
    );

    match failed + segfaults {
        0 => exit(0),
        _ => exit(1),
    }
}

#[test_case]
const GENERATED_TESTS_DIR: &'static str = "tests/fixtures/parser/gen";
