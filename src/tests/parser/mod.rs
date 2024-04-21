mod fixture;
pub(crate) use fixture::test_file;
use lib_ruby_parser_ast_arena::Blob;

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

fn parse<'b>(input: &[u8], blob: &'b Blob<'b>) -> ParserResult<'b> {
    let options = ParserOptions {
        buffer_name: "(eval)".into(),
        record_tokens: false,
        ..Default::default()
    };
    let input = blob.push_bytes(input);
    let parser = Parser::new(input, options, &blob);
    let result = parser.do_parse();
    result
}

#[test]
fn test_magic_comment() {
    let fixture = std::fs::read("src/tests/fixtures/magic_comments.rb").unwrap();
    let mut mem = vec![0; 1000];
    let blob = lib_ruby_parser_ast_arena::Blob::from(mem.as_mut_slice());

    let ParserResult { magic_comments, .. } = parse(&fixture, &blob);
    let magic_comments = magic_comments.iter().collect::<Vec<_>>();
    assert_eq!(
        magic_comments,
        &[
            &MagicComment::new(
                MagicCommentKind::Encoding,
                Loc { begin: 2, end: 10 },
                Loc { begin: 12, end: 17 },
            ),
            &MagicComment::new(
                MagicCommentKind::FrozenStringLiteral,
                Loc { begin: 20, end: 41 },
                Loc { begin: 43, end: 47 },
            ),
            &MagicComment::new(
                MagicCommentKind::Encoding,
                Loc { begin: 50, end: 56 },
                Loc { begin: 58, end: 63 },
            ),
            &MagicComment::new(
                MagicCommentKind::ShareableConstantValue,
                Loc { begin: 66, end: 90 },
                Loc { begin: 92, end: 99 },
            ),
            &MagicComment::new(
                MagicCommentKind::WarnIndent,
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
