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
  'test_mod_not_command_start__19',
  'test_question_eh_escape_M_escape_C__18',
  'test_question_eh_escape_M_escape_C__19',
  'test_question_eh_a__18',
  # just a bug
  'test_float_suffix_12',
  'test_float_suffix_16',
  # requires static env manipulation
  'test_static_env_0',
  # just a bug (that doesn't affect anything)
  'test_rcurly_0',
  # MRI relies on other data to emit kDO_BLOCK instead of kDO
  'test_do_block_0',

  # 2.7.1 :003 > Ripper.lex('def a=~').last
  # => [[1, 5], :on_op, "=~", BEG]
  # parser expects 'a=' and '~'
  'test_identifier_equals_tilde_0',

  # we emit 'd' in '0d10' as 'D'
  'test_numbers_3',
  'test_numbers_5',
  'test_numbers_1',

  # parser emits :~@ as :~
  'test_parser_bug_486_0',
  'test_parser_bug_486_1',

  # seems to be a bug, parser emits || as tOROP on expr_beg, MRI emits tPIPE
  'test_or2_0',

  # problematic escaping
  'test_bug_string_utf_escape_composition',
  'test_string_double_escape_C',
  'test_string_double_escape_c',
  'test_string_double_escape_hex',
  'test_string_double_escape_C_backslash',
  'test_string_double_escape_octal',
  'test_bug_hidden_eof',
  'test_string_double_escape_bs1',
  'test_string_double_escape_M_escape',
  'test_string_double_escape_M_backslash',
  'test_heredoc_with_identifier_ending_newline__24',
  'test_bug_string_non_utf',
  'test_string_double_escape_M',
  'test_string_double_escape_octal_wrap',
  'test_string_double_escape_C_escape',
  'test_string_double_escape_c_escape',
  'test_question_eh_escape_space_around_unicode_point_24_0',


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
  'test_string_single_escape_chars',
  'test_string_escape_x_single',
  'test_string_double_escape_C_question',
  'test_string_double_escape_chars',
  'test_string_double_escape_c_question',

  # In regexes REGEXP_END and REGOPT are merged (AST is the same)
  'test_regexp_escape_other_meta',
  'test_regexp_escape_chars',
  'test_regexp_escape_hex',
  'test_regexp_escape_C_M',
  'test_regexp_escape_C_M_craaaazy',
  'test_regexp_nm',
  'test_regexp_escape_bs',
  'test_regexp_escape_oct2',
  'test_regexp_escape_double_backslash',
  'test_regexp',
  'test_regexp_escape_oct1',
  'test_regexp_escape_backslash_slash',
  'test_regexp_ambiguous',
  'test_regexp_escape_c_backslash',
  'test_bug_expr_beg_backspace_nl_0',
  'test_regexp_escape_hex_one',
  'test_regexp_escape_C',
  'test_regexp_escape_M',
  'test_regexp_escape_oct3',
  'test_bug_expr_beg_div',
  'test_regexp_escape_return',
  'test_bug_expr_arg_slash_2',
  'test_regexp_escape_backslash_terminator_meta3',
  'test_question_eh_escape_space_around_unicode_point_24_1',

  # interpolation involves parser
  'test_bug_ragel_stack_0',
  'test_heredoc_none_0',
  'test_bug_heredoc_lshft_0',
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
    end
  end
end
