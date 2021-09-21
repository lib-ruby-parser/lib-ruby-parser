ROOT = File.expand_path('../..', __dir__)
PARSER_DIR = File.expand_path('../parser', __dir__)
TARGET_RUBY_VERSION = "3.0"
TARGET_DIR = File.expand_path('../../fixtures/lexer/gen', __dir__)

require 'fileutils'
FileUtils.rm_rf(TARGET_DIR)
FileUtils.mkdir_p(TARGET_DIR)

puts "Importing test cases from #{PARSER_DIR}"

$LOAD_PATH << File.join(PARSER_DIR, 'lib')
$LOAD_PATH << File.join(PARSER_DIR, 'test')

ENV['BUNDLE_GEMFILE'] = File.join(PARSER_DIR, 'Gemfile')
require 'bundler/setup'
require 'helper'
require 'parse_helper'

TESTS = Hash.new { |hash, test_name| hash[test_name] = [] }

class Blackhole
  def method_missing(*); self; end
end

class Parser::Lexer
  def comments
    Blackhole.new
  end
end

require 'test_lexer'

class TestLexer
  i_suck_and_my_tests_are_order_dependent!

  UNPARSABLE_TOKENS = %i[tRATIONAL tFLOAT tCOMPLEX tIMAGINARY]

  def assert_scanned(input, *tokens)
    variables = []
    static_env = @lex.static_env
    if static_env
      variables = static_env.instance_variable_get(:@variables).to_a
    end

    input = input.dup.force_encoding('utf-8')

    tokens = tokens.each_slice(3).map do |(name, value, range)|
      raw_value = value
      if value.is_a?(Numeric)
        value = value.to_s
      end

      if value.is_a?(String)
        value = value.dup.force_encoding('utf-8')
        value = value.gsub("'") { "\\'" }
        if value.valid_encoding?
          value = value.inspect
        end
      elsif value.nil?
        value = "".inspect
      else
        raise "Unknown token value type #{value.inspect}"
      end

      case name
      when :tINTEGER
        if (raw_value = input[range[0]...range[1]]) && raw_value.start_with?('0')
          value = raw_value.inspect
        else
          value = raw_value.to_s.inspect
        end
      when *UNPARSABLE_TOKENS
        value = input[range[0]...range[1]].inspect
      end

      [name, value, range]
    end

    tokens.each_cons(2) do |(t1, t2)|
      if t1[0] == :tUNARY_NUM && %i[tFLOAT tINTEGER].include?(t2[0])
        t1[0] = :tUMINUS_NUM
      end
    end

    tokens = tokens.flat_map do |(name, value, range)|
      if name == :tSYMBOL
        [
          [:tSYMBEG, ':'.inspect, [range[0], range[0] + 1]],
          [:tIDENTIFIER, value, [range[0] + 1, range[1]]]
        ]
      elsif name == :tSTRING
        sep = input[range[0]]
        [
          [:tSTRING_BEG, "\"\\#{sep}\"", [range[0], range[0] + 1]],
          [:tSTRING_CONTENT, value, [range[0] + 1, range[1] - 1]],
          [:tSTRING_END, "\"\\#{sep}\"", [range[1] - 1, range[1]]]
        ]
      elsif name == :tNL
        [[:tNL, "\n".inspect, range]]
      else
        [[name, value, range]]
      end
    end

    TESTS[name] << {
      state: @lex.state,
      input: input,
      tokens: tokens,
      variables: variables,
      cond: @lex.cond.active?,
      cmdarg: @lex.cmdarg.active?,
    }
  end

  def refute_scanned(*); end
  def assert_equal(*); end
  def assert_nil(*); end
  def assert_raises(*); Blackhole.new; end
  def test_eof; end
end

IGNORE = [
  'test_ambiguous_uplus_0',
  'test_and_equals_0',
  'test_and2_equals_0',
  'test_back_ref_0',
  'test_bug_eh_symbol_no_newline_0',
  'test_bug_expr_arg_lt_lt_1',
  'test_bug_expr_arg_percent_1',
  'test_bug_expr_arg_percent_3',
  'test_bug_expr_arg_slash_2',
  'test_bug_expr_beg_backspace_nl_0',
  'test_bug_expr_beg_div_0',
  'test_bug_expr_beg_div_1',
  'test_bug_expr_beg_heredoc_0',
  'test_bug_expr_beg_number_0',
  'test_bug_expr_endarg_braces_0',
  'test_bug_fid_char_0',
  'test_bug_heredoc_backspace_nl_0',
  'test_bug_heredoc_continuation_0',
  'test_bug_heredoc_cr_lf_0',
  'test_bug_heredoc_lshft_0',
  'test_bug_hidden_eof_0',
  'test_bug_hidden_eof_1',
  'test_bug_hidden_eof_2',
  'test_bug_hidden_eof_3',
  'test_bug_interleaved_heredoc_0',
  'test_bug_interleaved_heredoc_1',
  'test_bug_interleaved_heredoc_2',
  'test_bug_interp_expr_value_0',
  'test_bug_ragel_stack_0',
  'test_bug_string_non_utf_0',
  'test_bug_string_non_utf_2',
  'test_bug_string_percent_zero_0',
  'test_bug_string_utf_escape_noop_0',
  'test_carat_equals_0',
  'test_comment_0',
  'test_comment_1',
  'test_div_equals_0',
  'test_do_block_0',
  'test_dot2_0',
  'test_dot3_0',
  'test_escapes_in_squiggly_heredoc_0',
  'test_escapes_in_squiggly_heredoc_1',
  'test_float_dot_e_pos_0',
  'test_float_dot_E_pos_0',
  'test_float_e_pos_0',
  'test_float_e_pos_minus_0',
  'test_float_e_pos_plus_0',
  'test_float_pos_0',
  'test_float_suffix_0',
  'test_float_suffix_12',
  'test_float_suffix_16',
  'test_float_suffix_3',
  'test_float_suffix_6',
  'test_fname_pct_s_23_0',
  'test_global_number_0',
  'test_heredoc_backtick_0',
  'test_heredoc_cr_0',
  'test_heredoc_double_0',
  'test_heredoc_double_dash_0',
  'test_heredoc_double_interp_0',
  'test_heredoc_empty_0',
  'test_heredoc_none_0',
  'test_heredoc_none_dash_0',
  'test_heredoc_one_character_0',
  'test_heredoc_single_0',
  'test_heredoc_single_dash_0',
  'test_heredoc_with_identifier_ending_newline_24_0',
  'test_identifier_eh_1',
  'test_identifier_equals_tilde_0',
  'test_int_suffix_0',
  'test_int_suffix_2',
  'test_int_suffix_4',
  'test_integer_oct_o_0',
  'test_integer_oct_O_0',
  'test_integer_oct_o_not_bad_none_0',
  'test_integer_oct_O_not_bad_none_0',
  'test_integer_underscore_0',
  'test_label_18_0',
  'test_label_22_0',
  'test_label_in_params_18_0',
  'test_label_nested_22_0',
  'test_lt2_equals_0',
  'test_minus_equals_0',
  'test_minus_unary_whitespace_number_0',
  'test_mod_not_command_start_19_0',
  'test_mod_not_command_start_19_1',
  'test_mod_not_command_start_19_2',
  'test_mod_not_command_start_19_3',
  'test_nth_ref_0',
  'test_numbers_1',
  'test_numbers_10',
  'test_numbers_11',
  'test_numbers_12',
  'test_numbers_3',
  'test_numbers_5',
  'test_numbers_6',
  'test_numbers_7',
  'test_numbers_8',
  'test_numbers_9',
  'test_or_equals_0',
  'test_or2_0',
  'test_or2_after_27_0',
  'test_or2_equals_0',
  'test_parser_bug_486_0',
  'test_parser_bug_486_1',
  'test_pct_string_colon_22_0',
  'test_percent_equals_0',
  'test_plus_equals_0',
  'test_plus_unary_number_0',
  'test_plus_unary_whitespace_number_0',
  'test_question_18_0',
  'test_question_19_0',
  'test_question_bad_ws_4',
  'test_question_eh_a_18_0',
  'test_question_eh_a_19_0',
  'test_question_eh_escape_M_escape_C_18_0',
  'test_question_eh_escape_M_escape_C_19_0',
  'test_question_eh_escape_space_around_unicode_point_24_0',
  'test_question_eh_escape_space_around_unicode_point_24_1',
  'test_question_eh_escape_space_around_unicode_point_24_2',
  'test_question_eh_escape_space_around_unicode_point_24_3',
  'test_question_eh_escape_space_around_unicode_point_24_4',
  'test_question_eh_escape_space_around_unicode_point_24_5',
  'test_question_eh_escape_u_4_digits_0',
  'test_question_eh_single_unicode_point_0',
  'test_question_eh_single_unicode_point_1',
  'test_question_ws_backslashed_18_0',
  'test_question_ws_backslashed_18_1',
  'test_question_ws_backslashed_18_2',
  'test_question_ws_backslashed_18_3',
  'test_question_ws_backslashed_18_4',
  'test_question_ws_backslashed_18_5',
  'test_question_ws_backslashed_19_0',
  'test_question_ws_backslashed_19_1',
  'test_question_ws_backslashed_19_2',
  'test_question_ws_backslashed_19_3',
  'test_question_ws_backslashed_19_4',
  'test_question_ws_backslashed_19_5',
  'test_rcurly_0',
  'test_regexp_0',
  'test_regexp_ambiguous_0',
  'test_regexp_escape_backslash_slash_0',
  'test_regexp_escape_backslash_terminator_0',
  'test_regexp_escape_backslash_terminator_meta1_0',
  'test_regexp_escape_backslash_terminator_meta2_0',
  'test_regexp_escape_backslash_terminator_meta3_0',
  'test_regexp_escape_bs_0',
  'test_regexp_escape_c_0',
  'test_regexp_escape_C_0',
  'test_regexp_escape_c_backslash_0',
  'test_regexp_escape_C_M_0',
  'test_regexp_escape_C_M_craaaazy_0',
  'test_regexp_escape_chars_0',
  'test_regexp_escape_delimiter_meta_0',
  'test_regexp_escape_delimiter_nonmeta_0',
  'test_regexp_escape_double_backslash_0',
  'test_regexp_escape_hex_0',
  'test_regexp_escape_hex_one_0',
  'test_regexp_escape_M_0',
  'test_regexp_escape_M_C_0',
  'test_regexp_escape_oct1_0',
  'test_regexp_escape_oct2_0',
  'test_regexp_escape_oct3_0',
  'test_regexp_escape_other_meta_0',
  'test_regexp_escape_return_0',
  'test_regexp_nm_0',
  'test_rshft_equals_0',
  'test_star_equals_0',
  'test_star2_equals_0',
  'test_string_double_escape_bs1_0',
  'test_string_double_escape_c_0',
  'test_string_double_escape_C_0',
  'test_string_double_escape_C_backslash_0',
  'test_string_double_escape_c_escape_0',
  'test_string_double_escape_C_escape_0',
  'test_string_double_escape_c_question_0',
  'test_string_double_escape_C_question_0',
  'test_string_double_escape_M_0',
  'test_string_double_escape_M_backslash_0',
  'test_string_double_escape_M_escape_0',
  'test_string_double_escape_octal_wrap_0',
  'test_string_double_interp_0',
  'test_string_double_interp_label_0',
  'test_string_escape_x_single_0',
  'test_string_pct_i_0',
  'test_string_pct_I_0',
  'test_string_pct_intertwined_with_heredoc_0',
  'test_string_pct_w_0',
  'test_string_pct_W_0',
  'test_string_pct_w_backslash_0',
  'test_string_pct_w_backslash_interp_nl_0',
  'test_string_pct_w_backslash_nl_0',
  'test_string_pct_w_bs_nl_0',
  'test_string_pct_W_bs_nl_0',
  'test_string_pct_w_bs_sp_0',
  'test_string_pct_W_interp_0',
  'test_string_pct_w_tab_0',
  'test_string_single_nl_0',
  'test_symbol_single_0',
  'test_ternary_1',
  'test_underscore_end_3',
  'test_whitespace_cr_0',
  'test_whitespace_end_2',
  'test_whitespace_end_4',
  'test_whitespace_endfn_2',
  'test_whitespace_endfn_5',
  'test_whitespace_arg',
  'test_float_dot_E_0',
  'test_float_dot_E_neg_0',
  'test_float_dot_e_neg_0',
  'test_string_pct_Q_backslash_0',

  # fail on rust nightly due to diff in escaping
  'test_string_single_0',
  'test_label_colon2_22_0',
  'test_bug_expr_end_colon_0',
  'test_string_single_escape_chars_0',
]

Minitest.after_run do
  fixtures = []

  TESTS.each do |test_name, cases|
    next if IGNORE.include?(test_name)

    cases.each_with_index do |capture, idx|
      full_test_name = "#{test_name}_#{idx}".gsub(/_{2,}/, '_').gsub('?', '_q_')
      next if IGNORE.include?(full_test_name)

      puts "Creating input/output files for #{full_test_name}"

      input_filepath = File.join(TARGET_DIR, full_test_name)

      fixture = [
        '--INPUT',
        capture[:input],
        '--TOKENS',
        capture[:tokens].map { |(name, value, range)| "#{name} #{value} #{range}" }
      ]

      if (variables = capture[:variables]).any?
        fixture = [
          '--VARS',
          variables.join(' '),
          *fixture
        ]
      end

      if capture[:state] != :line_begin
        fixture = [
          '--STATE',
          capture[:state],
          *fixture
        ]
      end

      if capture[:cmdarg]
        fixture = [
          '--CMDARG',
          *fixture
        ]
      end

      if capture[:cond]
        fixture = [
          '--COND',
          *fixture
        ]
      end

      fixture << ''

      File.write(input_filepath, fixture.join("\n"))
      fixtures << full_test_name
    end
  end

  fixtures = fixtures
    .sort
    .map { |f| "fixture_file!(\"fixtures/lexer/gen\", #{f});" }
    .join("\n")

  fixtures = <<~RS
    // This file is autogenerated by #{__FILE__}

    use super::{test_file, fixture_file};

    #{fixtures}
  RS

  File.write(File.join(ROOT, 'src/lexer/tests/gen.rs'), fixtures)
end
