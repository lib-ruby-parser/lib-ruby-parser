mod fixture;
pub(crate) use fixture::test_file;
use lib_ruby_parser_ast::Blob;

#[allow(non_snake_case)]
mod gen;
#[allow(non_snake_case)]
mod manual;

use crate::{
    source::{MagicComment, MagicCommentKind},
    Loc, Parser, ParserOptions, ParserResult, YYStackItem,
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

fn parse<'b, 's: 'b>(
    input: &[u8],
    blob: &'b Blob<'b>,
    stack: &'s mut [YYStackItem],
) -> ParserResult<'b> {
    let options = ParserOptions {
        buffer_name: "(eval)".into(),
        record_tokens: false,
        ..Default::default()
    };
    let input = blob.push_bytes(input);
    let parser = Parser::new(input, options, &blob);
    let result = parser.do_parse(stack);
    result
}

#[test]
fn test_magic_comment() {
    let fixture = std::fs::read("src/tests/fixtures/magic_comments.rb").unwrap();
    let mut stack = [YYStackItem::none(); 100];
    let mut mem = [0; 1000];
    let blob = Blob::from(&mut mem);

    let ParserResult { magic_comments, .. } = parse(&fixture, &blob, &mut stack);
    let mut iter = magic_comments.iter();

    assert_eq!(
        iter.next(),
        Some(MagicComment::new(
            MagicCommentKind::Encoding,
            Loc::new(2, 10),
            Loc::new(12, 17),
            &blob
        ))
    );
    assert_eq!(
        iter.next(),
        Some(MagicComment::new(
            MagicCommentKind::FrozenStringLiteral,
            Loc::new(20, 41),
            Loc::new(43, 47),
            &blob
        ))
    );
    assert_eq!(
        iter.next(),
        Some(MagicComment::new(
            MagicCommentKind::Encoding,
            Loc::new(50, 56),
            Loc::new(58, 63),
            &blob
        ))
    );
    assert_eq!(
        iter.next(),
        Some(MagicComment::new(
            MagicCommentKind::ShareableConstantValue,
            Loc::new(66, 90),
            Loc::new(92, 99),
            &blob
        ))
    );
    assert_eq!(
        iter.next(),
        Some(MagicComment::new(
            MagicCommentKind::WarnIndent,
            Loc::new(102, 113),
            Loc::new(115, 119),
            &blob
        ))
    );
    assert_eq!(iter.next(), None);
}
