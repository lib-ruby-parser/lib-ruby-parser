use super::{fixture_file, test_file};

fixture_file!("fixtures/parser/manual", ambiguous_ternary_operator);
fixture_file!("fixtures/parser/manual", invalid_escape_char_syntax_c);
fixture_file!("fixtures/parser/manual", invalid_escape_char_syntax_c_m);
fixture_file!("fixtures/parser/manual", invalid_escape_char_syntax_m);
fixture_file!("fixtures/parser/manual", invalid_escape_char_syntax_m_c);
fixture_file!("fixtures/parser/manual", invalid_hex);
fixture_file!("fixtures/parser/manual", invalid_hex_escape_at_eof);
fixture_file!("fixtures/parser/manual", invalid_unicode_codepoint);
fixture_file!("fixtures/parser/manual", invalid_unicode_escape);
fixture_file!("fixtures/parser/manual", multiple_char_codepoints);
fixture_file!("fixtures/parser/manual", multiple_string_codepoints);
fixture_file!(
    "fixtures/parser/manual",
    numeric_literal_without_digits_binary
);
fixture_file!(
    "fixtures/parser/manual",
    numeric_literal_without_digits_decimal
);
fixture_file!("fixtures/parser/manual", numeric_literal_without_digits_hex);
fixture_file!(
    "fixtures/parser/manual",
    numeric_literal_without_digits_octal
);
fixture_file!("fixtures/parser/manual", test_assignment_to_numparams_1);
fixture_file!("fixtures/parser/manual", test_kwarg_combinations_0);
fixture_file!("fixtures/parser/manual", test_kwarg_combinations_1);
fixture_file!("fixtures/parser/manual", test_lvar_injecting_match_0);
fixture_file!("fixtures/parser/manual", test_marg_combinations_0);
fixture_file!("fixtures/parser/manual", test_marg_combinations_1);
fixture_file!("fixtures/parser/manual", test_marg_combinations_2);
fixture_file!("fixtures/parser/manual", test_marg_combinations_3);
fixture_file!("fixtures/parser/manual", test_marg_combinations_4);
fixture_file!("fixtures/parser/manual", test_marg_combinations_5);
fixture_file!("fixtures/parser/manual", test_marg_combinations_6);
fixture_file!("fixtures/parser/manual", test_marg_combinations_7);
fixture_file!("fixtures/parser/manual", test_marg_combinations_8);
fixture_file!("fixtures/parser/manual", test_marg_combinations_9);
fixture_file!("fixtures/parser/manual", test_range_endless_1);
fixture_file!("fixtures/parser/manual", test_regex_error_0);
fixture_file!("fixtures/parser/manual", test_regex_error_1);
fixture_file!("fixtures/parser/manual", test_regexp_encoding_0);
fixture_file!("fixtures/parser/manual", unterminated_heredoc_id);
fixture_file!("fixtures/parser/manual", unterminated_unicode_escape);
fixture_file!(
    "fixtures/parser/manual",
    unterminated_string_after_percent_at_eof
);
fixture_file!(
    "fixtures/parser/manual",
    random_characters_before_encoding_comment
);
fixture_file!(
    "fixtures/parser/manual",
    test_private_endless_method_command_syntax_0
);
fixture_file!(
    "fixtures/parser/manual",
    test_private_endless_method_command_syntax_1
);
fixture_file!(
    "fixtures/parser/manual",
    test_private_endless_method_command_syntax_3
);
fixture_file!(
    "fixtures/parser/manual",
    test_private_endless_method_command_syntax_4
);
fixture_file!("fixtures/parser/manual", control_meta_chars_in_regexes);
fixture_file!("fixtures/parser/manual", slash_u_after_meta_control_chars_0);
fixture_file!(
    "fixtures/parser/manual",
    test_erange_without_parentheses_at_eol_0
);
fixture_file!(
    "fixtures/parser/manual",
    test_numbered_and_ordinary_parameters_13
);
fixture_file!(
    "fixtures/parser/manual",
    test_pattern_matching_hash_with_heredoc_keys_0
);
fixture_file!("fixtures/parser/manual", test_rasgn_line_continuation_0);
fixture_file!("fixtures/parser/manual", test_unterimated_heredoc_id_27_0);
fixture_file!("fixtures/parser/manual", test_unterimated_heredoc_id_27_1);
fixture_file!("fixtures/parser/manual", test_unterimated_heredoc_id_27_2);
fixture_file!("fixtures/parser/manual", test_unterimated_heredoc_id_27_3);
fixture_file!("fixtures/parser/manual", test_unterminated_embedded_doc_0);
fixture_file!("fixtures/parser/manual", test_unterminated_embedded_doc_1);
fixture_file!("fixtures/parser/manual", case_with_multiple_whens_and_else);
fixture_file!("fixtures/parser/manual", casematch_with_multiple_ins_and_else);
