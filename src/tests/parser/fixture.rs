use core::fmt::Write;
use lib_ruby_parser_ast::{Blob, Writer};

use crate::tests::test_helpers::{render_diagnostic_for_testing, InlineArray, LocMatcher};
use crate::{Parser, ParserOptions, ParserResult, YYStackItem};

enum TestSection {
    None,
    Input,
    Ast,
    Locations,
    Diagnostic,
    DependsOnFeature,
}

#[derive(Debug)]
struct Fixture<'s> {
    input: &'s str,
    ast: Option<&'s str>,
    locs: Option<InlineArray<50, &'s str>>,
    diagnostics: Option<InlineArray<50, &'s str>>,
    depends_on_features: Option<InlineArray<5, &'s str>>,
}

fn none_if_empty<'s, const MAX: usize>(
    v: InlineArray<MAX, &'s str>,
) -> Option<InlineArray<MAX, &'s str>> {
    if v.len == 0 || (v.len == 1 && v.iter().next().unwrap() == "") {
        None
    } else {
        Some(v)
    }
}

impl<'s> Fixture<'s> {
    fn new(content: &'s str) -> Self {
        let mut input_start_at = None;
        let mut input_ends_at = None;

        let mut ast_start_at = None;
        let mut ast_ends_at = None;

        let mut locs = InlineArray::new();
        let mut diagnostics = InlineArray::new();
        let mut depends_on_features = InlineArray::new();

        let mut current_section = TestSection::None;
        let mut pos = 0;

        for line in content.lines() {
            pos += line.len() + 1;

            match (line.as_bytes(), &current_section) {
                (&[b'/', b'/', b' ', ..], _) => { /* skip comment */ }

                (b"--INPUT", _) => {
                    current_section = TestSection::Input;
                    input_start_at = Some(pos);
                }
                (b"--AST", _) => {
                    current_section = TestSection::Ast;
                    ast_start_at = Some(pos);
                }
                (b"--LOCATIONS", _) => current_section = TestSection::Locations,
                (b"--DIAGNOSTIC", _) => current_section = TestSection::Diagnostic,
                (b"--DEPENDS-ON-FEATURES", _) => current_section = TestSection::DependsOnFeature,

                (_, &TestSection::Input) => {
                    input_ends_at = Some(pos - 1);
                }
                (_, &TestSection::Ast) => {
                    ast_ends_at = Some(pos - 1);
                }
                (_, &TestSection::Locations) => locs.push(line),
                (_, &TestSection::Diagnostic) => diagnostics.push(line),
                (_, &TestSection::DependsOnFeature) => depends_on_features.push(line),

                (_, &TestSection::None) => {
                    panic!("empty state while parsing fixture on line {:#?}", line)
                }
            }
        }

        let input = &content[input_start_at.unwrap()..input_ends_at.unwrap()];
        let ast = if ast_start_at == ast_ends_at {
            None
        } else {
            Some(&content[ast_start_at.unwrap()..ast_ends_at.unwrap()])
        };
        let locs = none_if_empty(locs);
        let diagnostics = none_if_empty(diagnostics);
        let depends_on_features = none_if_empty(depends_on_features);

        if let (None, None, None) = (&ast, &locs, &diagnostics) {
            panic!("empty test")
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
                let mut buf = [0; 1000];
                let mut writer = Writer::new(&mut buf);
                let inspected = if let Some(actual_ast) = actual.ast {
                    actual_ast.inspect(0, &mut writer).unwrap();
                    writer.as_str().unwrap_or("<nothing>")
                } else {
                    "nil"
                };

                assert_eq!(
                    inspected, *expected_ast,
                    "AST diff:\nactual:\n{}\nexpected:\n{}\n",
                    inspected, expected_ast
                );
            }
            None => {}
        }

        match &self.locs {
            Some(locs) => {
                let ast = if let Some(ast) = actual.ast {
                    ast
                } else {
                    panic!("can't compare locs, ast is empty");
                };

                for loc in locs.iter() {
                    match LocMatcher::new(loc).test(ast) {
                        Ok(_) => {}
                        Err(err) => panic!("{}", core::str::from_utf8(&err).unwrap()),
                    }
                }
            }
            None => {}
        }

        match &self.diagnostics {
            None => {
                let mut buf = [0; 1000];
                let mut writer = Writer::new(&mut buf);
                for d in actual.diagnostics.iter() {
                    render_diagnostic_for_testing(d, &mut writer).unwrap();
                    writeln!(&mut writer).unwrap();
                }
                let emitted = writer.as_str().unwrap_or_default();

                assert_eq!(
                    actual.diagnostics.len(),
                    0,
                    "expected no diagnostics to be emitted, got:\n{}",
                    emitted
                );
            }
            Some(diagnostics) => {
                let expected = diagnostics;

                for (expected, actual) in expected.iter().zip(actual.diagnostics.iter()) {
                    let mut buf = [0; 1000];
                    let mut writer = Writer::new(&mut buf);
                    render_diagnostic_for_testing(actual, &mut writer).unwrap();
                    let actual = writer.as_str().unwrap();

                    assert_eq!(
                        expected, actual,
                        "expected diagnostc:\n{}\nactual diagnostic:\n{}",
                        expected, actual
                    )
                }
            }
        }
    }
}

pub(crate) fn test_file(fixture_path: &str) {
    let mut stack = vec![YYStackItem::none(); 1_000];
    let mut mem = [0; 2000];
    let blob = Blob::from(&mut mem);

    let src = std::fs::read_to_string(fixture_path)
        .unwrap_or_else(|_| panic!("failed to read file {:?}", fixture_path));
    let fixture = Fixture::new(&src);

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
        buffer_name: &format!("(test {})", fixture_path),
        record_tokens: false,
        ..Default::default()
    };

    let parser = Parser::new(fixture.input.as_bytes(), options, &blob);

    parser.static_env.declare("foo", &blob);
    parser.static_env.declare("bar", &blob);
    parser.static_env.declare("baz", &blob);

    let result = if fixture.diagnostics.is_some() {
        parser.do_parse(&mut stack)
    } else {
        parser.do_parse_with_state_validation(&mut stack)
    };

    fixture.compare(&result);
}
