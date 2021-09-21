crate::use_native_or_external!(Maybe);

mod fixture;
pub(crate) use fixture::test_file;

#[allow(non_snake_case)]
mod gen;
#[allow(non_snake_case)]
mod manual;

use crate::{
    debug_level,
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
    let options = ParserOptions::new(
        "(eval)".into(),
        debug_level::NONE,
        Maybe::none(),
        Maybe::none(),
        false,
    );
    let parser = Parser::new(input, options);
    parser.do_parse()
}

#[test]
fn test_magic_comment() {
    let fixture = std::fs::read("fixtures/magic_comments.rb").unwrap();
    let result = parse(&fixture);
    let magic_comments: Vec<MagicComment> = result.magic_comments().to_owned().into();
    assert_eq!(
        magic_comments,
        vec![
            MagicComment::new(
                MagicCommentKind::encoding(),
                Loc::new(2, 10),
                Loc::new(12, 17),
            ),
            MagicComment::new(
                MagicCommentKind::frozen_string_literal(),
                Loc::new(20, 41),
                Loc::new(43, 47),
            ),
            MagicComment::new(
                MagicCommentKind::encoding(),
                Loc::new(50, 56),
                Loc::new(58, 63),
            ),
            MagicComment::new(
                MagicCommentKind::shareable_constant_value(),
                Loc::new(66, 90),
                Loc::new(92, 99),
            ),
            MagicComment::new(
                MagicCommentKind::warn_indent(),
                Loc::new(102, 113),
                Loc::new(115, 119),
            ),
        ]
    );
}
