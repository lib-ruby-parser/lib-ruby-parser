use super::{fixture_file, test_file};

fixture_file!(
    "src/tests/fixtures/lexer/manual",
    test_string_single_escape_chars_0
);
fixture_file!("src/tests/fixtures/lexer/manual", test_bug_expr_end_colon_0);
fixture_file!("src/tests/fixtures/lexer/manual", test_label_colon2_22_0);
fixture_file!("src/tests/fixtures/lexer/manual", test_string_single_0);
fixture_file!("src/tests/fixtures/lexer/manual", test_string_pct_null_0);
fixture_file!("src/tests/fixtures/lexer/manual", test_string_pct_w_null_0);
