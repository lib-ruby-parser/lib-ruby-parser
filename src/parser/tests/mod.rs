mod fixture;
pub(crate) use fixture::test_file;

#[allow(non_snake_case)]
mod gen;
#[allow(non_snake_case)]
mod manual;

use crate::{
    source::{MagicComment, MagicCommentKind},
    Loc, Parser, ParserOptions, ParserResult,
};

macro_rules! fixture_file {
    ($dir:literal, $fixture:ident) => {
        #[test]
        fn $fixture() {
            let fixture_path = format!("{}/{}", $dir, stringify!($fixture));
            test_file(&fixture_path);
        }
    };
}
pub(crate) use fixture_file;

fn parse(input: &[u8]) -> ParserResult {
    let options = ParserOptions {
        buffer_name: "(eval)".into(),
        record_tokens: false,
        ..Default::default()
    };
    let parser = Parser::new(input, options);
    parser.do_parse()
}

#[test]
fn test_magic_comment() {
    let fixture = std::fs::read("fixtures/magic_comments.rb").unwrap();
    let result = parse(&fixture);
    let magic_comments: Vec<MagicComment> = result.magic_comments.to_owned().into();
    assert_eq!(
        magic_comments,
        vec![
            MagicComment::new(
                MagicCommentKind::encoding(),
                Loc { begin: 2, end: 10 },
                Loc { begin: 12, end: 17 },
            ),
            MagicComment::new(
                MagicCommentKind::frozen_string_literal(),
                Loc { begin: 20, end: 41 },
                Loc { begin: 43, end: 47 },
            ),
            MagicComment::new(
                MagicCommentKind::encoding(),
                Loc { begin: 50, end: 56 },
                Loc { begin: 58, end: 63 },
            ),
            MagicComment::new(
                MagicCommentKind::shareable_constant_value(),
                Loc { begin: 66, end: 90 },
                Loc { begin: 92, end: 99 },
            ),
            MagicComment::new(
                MagicCommentKind::warn_indent(),
                Loc {
                    begin: 102,
                    end: 113
                },
                Loc {
                    begin: 115,
                    end: 119
                },
            ),
        ]
    );
}
