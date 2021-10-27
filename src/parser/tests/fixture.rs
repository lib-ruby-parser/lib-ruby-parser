use crate::test_helpers::{render_diagnostic_for_testing, LocMatcher};
use crate::{Parser, ParserOptions, ParserResult};

enum TestSection {
    None,
    Input,
    AST,
    Locations,
    Diagnostic,
    DependsOnFeature,
}

#[derive(Debug)]
struct Fixture {
    input: String,
    ast: Option<String>,
    locs: Option<Vec<String>>,
    diagnostics: Option<Vec<String>>,
    depends_on_features: Option<Vec<String>>,
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
        let content = std::fs::read_to_string(path)
            .unwrap_or_else(|_| panic!("failed to read file {:?}", path));

        let mut input: Vec<String> = vec![];
        let mut ast: Vec<String> = vec![];
        let mut locs: Vec<String> = vec![];
        let mut diagnostics: Vec<String> = vec![];
        let mut depends_on_features: Vec<String> = vec![];
        let mut current_section = TestSection::None;

        for line in content.lines() {
            match (line.as_bytes(), &current_section) {
                (&[b'/', b'/', b' ', ..], _) => { /* skip comment */ }

                (b"--INPUT", _) => current_section = TestSection::Input,
                (b"--AST", _) => current_section = TestSection::AST,
                (b"--LOCATIONS", _) => current_section = TestSection::Locations,
                (b"--DIAGNOSTIC", _) => current_section = TestSection::Diagnostic,
                (b"--DEPENDS-ON-FEATURES", _) => current_section = TestSection::DependsOnFeature,

                (_, &TestSection::Input) => input.push(line.to_string()),
                (_, &TestSection::AST) => ast.push(line.to_string()),
                (_, &TestSection::Locations) => locs.push(line.to_string()),
                (_, &TestSection::Diagnostic) => diagnostics.push(line.to_string()),
                (_, &TestSection::DependsOnFeature) => depends_on_features.push(line.to_string()),

                (_, &TestSection::None) => {
                    panic!("empty state while parsing fixture on line {:#?}", line)
                }
            }
        }

        let input = input.join("\n");
        let ast = none_if_empty(ast).map(|lines| lines.join("\n"));
        let locs = none_if_empty(locs);
        let diagnostics = none_if_empty(diagnostics);
        let depends_on_features = none_if_empty(depends_on_features);

        match (&ast, &locs, &diagnostics) {
            (None, None, None) => panic!("empty test"),
            _ => {}
        }

        Self {
            input,
            ast,
            locs,
            diagnostics,
            depends_on_features,
        }
    }

    fn compare(&self, actual: &ParserResult) {
        match &self.ast {
            Some(expected_ast) => {
                let actual_ast = actual
                    .ast()
                    .as_ref()
                    .map(|node| node.inspect(0))
                    .unwrap_or_else(|| "nil".to_string());

                assert_eq!(
                    &actual_ast, expected_ast,
                    "AST diff:\nactual:\n{}\nexpected:\n{}\n",
                    actual_ast, expected_ast
                );
            }
            None => {}
        }

        match &self.locs {
            Some(locs) => {
                let ast = if let Some(ast) = actual.ast().as_ref() {
                    ast
                } else {
                    panic!("can't compare locs, ast is empty");
                };

                for loc in locs {
                    match LocMatcher::new(loc).test(ast) {
                        Ok(_) => {}
                        Err(err) => panic!("{}", err),
                    }
                }
            }
            None => {}
        }

        let actual_diagnostics = actual
            .diagnostics()
            .iter()
            .map(|d| render_diagnostic_for_testing(d))
            .collect::<Vec<_>>();

        match &self.diagnostics {
            None => {
                assert_eq!(
                    actual.diagnostics().len(),
                    0,
                    "expected no diagnostics to be emitted, got:\n{}",
                    actual_diagnostics.join("\n")
                );
            }
            Some(diagnostics) => {
                let expected = diagnostics;
                let actual = actual_diagnostics;

                assert_eq!(
                    expected,
                    &actual,
                    "expected diagnostcs:\n{}\nactual diagnostics:\n{}",
                    expected.join("\n"),
                    actual.join("\n")
                );
            }
        }
    }
}

pub(crate) fn test_file(fixture_path: &str) {
    let fixture = Fixture::new(fixture_path);

    if let Some(depends_on_features) = &fixture.depends_on_features {
        for feature in depends_on_features.iter() {
            match &feature[..] {
                "onig" => {
                    if cfg!(feature = "onig") {
                        // ok, keep going
                    } else {
                        // skip
                        return;
                    }
                }
                unsupported => panic!("Unsupported feature {:?}", unsupported),
            }
        }
    }

    let options = ParserOptions {
        buffer_name: format!("(test {})", fixture_path),
        record_tokens: false,
        ..Default::default()
    };
    let parser = Parser::new(fixture.input.as_bytes(), options);

    parser.static_env.declare("foo");
    parser.static_env.declare("bar");
    parser.static_env.declare("baz");

    let result = if fixture.diagnostics.is_some() {
        parser.do_parse()
    } else {
        parser.do_parse_with_state_validation()
    };

    fixture.compare(&result)
}
