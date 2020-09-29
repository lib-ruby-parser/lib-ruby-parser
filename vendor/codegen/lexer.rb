PARSER_DIR = File.expand_path('../parser', __dir__)
TARGET_RUBY_VERSION = "3.0"
TARGET_DIR = File.expand_path('../../tests/fixtures/lexer/gen', __dir__)

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
        if value.valid_encoding?
          value = value.inspect
        end
      elsif value.nil?
        value = "".inspect
      else
        raise "Unknown token value type #{value.inspect}"
      end

      if [:tSTRING_BEG, :tSTRING_END].include?(name) && value == "\"'\""
        value = "\"\\'\""
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

    TESTS[name] << { state: @lex.state, input: input, tokens: tokens, variables: variables }
  end

  def refute_scanned(*); end
  def assert_equal(*); end
  def assert_nil(*); end
  def assert_raises(*); Blackhole.new; end
  def test_eof; end
end

IGNORE = [
  # Legacy test
  'test___ENCODING___legacy_',

  # That's the difference between MRI lexer and parser gem
  # when "||" is used without arguments.
  'test_or2_after_27_0',

  # parser bugs:
  # '+1.0' is a single literal, not a unary plus and a number
  'test_ambiguous_uplus_0',
  'test_float_dot_e_pos_0',
  'test_float_dot_E_pos_0',
  'test_float_dot_e_upper_pos_0',
  'test_float_e_pos_0',
  'test_float_e_pos_minus_0',
  'test_float_e_pos_plus_0',
  'test_float_pos_0',
  'test_minus_unary_whitespace_number_0',
  'test_plus_unary_number_0',
  'test_plus_unary_whitespace_number_0',
  'test_whitespace_end_2',
  # these are recordings for olrder rubies
  'test_float_suffix_0',
  'test_float_suffix_3',
  'test_float_suffix_6',
  'test_int_suffix_0',
  'test_int_suffix_2',
  'test_int_suffix_4',
  'test_dot3_0',
  'test_dot2_0',
  'test_label__18',
  # just a bug
  'test_float_suffix_12',
  'test_float_suffix_16',
  # requires static env manipulation
  'test_static_env_0',
  # just a bug (that doesn't affect anything)
  'test_rcurly_0',

  # 2.7.1 :003 > Ripper.lex('def a=~').last
  # => [[1, 5], :on_op, "=~", BEG]
  # parser expects 'a=' and '~'
  'test_identifier_equals_tilde_0',

  # we emit string/symbols literals like MRI (STRING_BEG + STRING_CONTENT + STRING_END)
  'test_string_single_escape_chars_0',
  'test_bug_expr_end_colon_0',
  'test_bug_string_utf_escape_composition_1',
  'test_label_colon2_22_0',
  'test_command_start_19_5',
  'test_command_start_19_2',
  'test_string_escape_x_single_0',
  'test_string_double_escape_C_question_0',
  'test_bug_string_utf_escape_composition_0',
  'test_string_double_interp_0',
  'test_command_start_19_3',
  'test_command_start_19_4',
  'test_mod_not_command_start_19_3',
  'test_string_double_escape_chars_0',
  'test_string_double_interp_label_0',
  'test_string_single_0',
  'test_transcoded_source_is_converted_back_to_original_encoding_0',
  'test_mod_not_command_start_19_2',
  'test_string_double_0',
  'test_whitespace_value_1',
  'test_whitespace_value_0',
  'test_string_double_escape_c_question_0',
  'test_command_start_19_1',
  'test_command_start_19_6',
  'test_command_start_19_8',
  'test_identifier_equals_equals_arrow_0',
  'test_command_start_19_9',
  'test_command_start_19_7',
  'test_identifier_equals3_0',
  'test_command_start_19_0',
  'test_string_double_escape_bs2_0',
  'test_bug_utf32le_leak',
  'test_symbol_0',
  'test_whitespace_endfn_2',
  'test_sclass_label',
  'test_mod_not_command_start_19_0',
  'test_label_in_params_18_0',
  'test_mod_not_command_start_19_1',
  'test_bug_line_begin_label_0',
  'test_whitespace_value_2',
  'test_string_double_no_interp_0',
  'test_identifier_equals_arrow_0',
  'test_whitespace_value_3',
  'test_string_double_no_interp_1',
  'test_command_start_19_10',
  'test_bug_symbol_newline_0',
  'test_bug_symbol_newline_1',

  # we emit 'd' in '0d10' as 'D'
  'test_numbers_3',
  'test_numbers_5',
  'test_numbers_1',

  # parser emits tUNARY_NUM instead of tUMINUS_NUM
  'test_minus_unary_number_0',
  'test_float_neg_0',
  'test_float_dot_E_neg_0',
  'test_float_dot_e_neg_0',
  'test_float_e_neg_plus_0',
  'test_ambiguous_uminus_0',
  'test_float_e_neg_minus_0',
  'test_float_e_neg_0',

  # parser emits :~@ as :~
  'test_parser_bug_486_0',
  'test_parser_bug_486_1',

  # seems to be a bug, parser emits || as tOROP on expr_beg, MRI emits tPIPE
  'test_or2_0',

  # requires cond/cmdarg manipulation
  'test_do_cond_0',
  'test_do_block_0',

  # problematic escaping
  'test_bug_string_utf_escape_composition',
  'test_string_double_escape_C',
  'test_string_double_escape_c',
  'test_string_double_escape_hex',
  'test_string_double_escape_C_backslash',
  'test_string_double_escape_octal',
  'test_bug_hidden_eof',
  'test_string_double_escape_bs1',

  # we exclude some chars from number's source (like _ from ints)
  # FIXME: these should be fixed.
  'test_numbers_12',
  'test_integer_underscore_0',
  'test_integer_oct_O_not_bad_none_0',
  'test_integer_oct_O_0',
  'test_numbers_11',
  'test_bug_expr_beg_number_0',
  'test_numbers_7',
  'test_numbers_9',
  'test_numbers_8',
  'test_numbers_6',
  'test_integer_oct_o_0',
  'test_numbers_10',
  'test_integer_oct_o_not_bad_none_0',
]

Minitest.after_run do
  TESTS.each do |test_name, cases|
    next if IGNORE.include?(test_name)

    cases.each_with_index do |capture, idx|
      full_test_name = "#{test_name}_#{idx}".gsub(/_{2,}/, '_')
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

      fixture << ''

      File.write(input_filepath, fixture.join("\n"))
    end
  end
end
