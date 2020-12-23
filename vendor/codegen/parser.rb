# This script imports test cases from whitequark/parser repo
# and saves them into tests/fixtures/parser/gen/
#
# Each assertion from whitequark/parser/test/test_parser.rb
# produces files
#   - test_<TEST_NAME>_<N>
#
# Where:
#   - TEST_NAME is the name of the test (i.e. the name of the minitest method)
#   - N is the number of assertion in this method
#
# Here we run a single test_parser.rb file with a patch that
# instead of running an assertion records expected/actual values
# (see ParseHelperPatch module)
#
# Also, there's a small difference between whitequark/parser and our parser
# that is handled by the "Rewriter" class which is a simple recursive visitor

PARSER_DIR = File.expand_path('../parser', __dir__)
TARGET_RUBY_VERSION = "3.0"
TARGET_DIR = File.expand_path('../../tests/fixtures/parser/gen', __dir__)

require 'fileutils'
FileUtils.rm_rf(TARGET_DIR)
FileUtils.mkdir(TARGET_DIR)

puts "Importing test cases from #{PARSER_DIR}"

$LOAD_PATH << File.join(PARSER_DIR, 'lib')
$LOAD_PATH << File.join(PARSER_DIR, 'test')

ENV['BUNDLE_GEMFILE'] = File.join(PARSER_DIR, 'Gemfile')
require 'bundler/setup'
require 'helper'
require 'parse_helper'

TESTS = Hash.new { |hash, test_name| hash[test_name] = [] }

class Rewriter < Parser::AST::Processor
  # For numeric literals we emit their source, not the value
  def replace_to_original_source(node)
    node.updated(nil, [node.location.expression.source])
  end

  alias on_int replace_to_original_source
  alias on_float replace_to_original_source
  alias on_rational replace_to_original_source
  alias on_complex replace_to_original_source

  # we don't emit implicit empty s(:args)
  def on_def(node)
    node = super
    name, args, body_node = *node
    args = nil if !args.nil? && args.children.empty? && args.loc.expression.nil?
    node.updated(nil, [name, args, body_node])
  end

  # we don't emit implicit empty s(:args)
  def on_defs(node)
    node = super
    definee_node, name, args, body_node = *node
    args = nil if !args.nil? && args.children.empty? && args.loc.expression.nil?
    node.updated(nil, [definee_node, name, args, body_node])
  end

  # we don't emit implicit empty s(:args)
  def on_block(node)
    node = super
    send, args, body = *node
    args = nil if !args.nil? && args.children.empty? && args.loc.expression.nil?
    node.updated(nil, [send, args, body])
  end

  alias on_assign process_regular_node

  private

  def s(type, *children)
    Parser::AST::Node.new(type, children)
  end
end

REWRITER = Rewriter.new

module ParseHelperPatch
  def assert_parses(ast, code, source_maps='', versions=ParseHelper::ALL_VERSIONS)
    if versions.include?(TARGET_RUBY_VERSION)
      parsed_ast = nil

      with_versions([TARGET_RUBY_VERSION]) do |version, parser|
        source_file = Parser::Source::Buffer.new('(assert_parses)')
        source_file.source = code

        begin
          parsed_ast = parser.parse(source_file)
        rescue => exc
          backtrace = exc.backtrace
          Exception.instance_method(:initialize).bind(exc).
            call("(#{version}) #{exc.message}")
          exc.set_backtrace(backtrace)
          raise
        end

        if parsed_ast
          parsed_ast = REWRITER.process(parsed_ast)
        end
      end

      fixture = [
        '--INPUT',
        code,
        '--LOCATIONS',
        locs(parsed_ast).join("\n"),
        '--AST',
        parsed_ast.inspect
      ]

      fixture << ''

      TESTS[name] << fixture
    end
  end

  # We want to replicate MRI errors
  RUBY_MESSAGES = {
    :argument_const => ->(*) { 'formal argument cannot be a constant' },
    :argument_cvar => ->(*) { 'formal argument cannot be a class variable' },
    :argument_gvar => ->(*) { 'formal argument cannot be a global variable' },
    :argument_ivar => ->(*) { 'formal argument cannot be an instance variable' },
    :backref_assignment => ->(args, range) { "Can't set variable #{range.source}" },
    :begin_in_method => ->(*) { 'BEGIN is permitted only at toplevel' },
    :block_and_blockarg => ->(*) { 'both block arg and actual block given' },
    :block_given_to_yield => ->(*) { 'block given to yield' },
    :cant_assign_to_numparam => ->(args, range) { "Can't assign to numbered parameter #{args[:name]}" },
    :circular_argument_reference => ->(args, range) { "circular argument reference - #{args[:var_name]}" },
    :class_in_def => ->(*) { 'class definition in method body' },
    :const_reassignment => ->(*) { raise 'invalid for 3.0' },
    :csend_in_lhs_of_masgn => ->(*) { '&. inside multiple assignment destination' },
    :cvar_name => ->(args, range) {
      case args[:name]
      when '@@' then "`@@' without identifiers is not allowed as a class variable name"
      else
        "`#{args[:name]}' is not allowed as a class variable name"
      end
    },
    :duplicate_argument => ->(args, range) { 'duplicated argument name' },
    :duplicate_pattern_key => ->(*) { 'duplicated key name' },
    :duplicate_variable_name => ->(*) { 'duplicated variable name' },
    :dynamic_const => ->(*) { 'dynamic constant assignment' },
    :empty_symbol => ->(args, range) { 'empty_symbol' },
    :endless_setter => ->(args, range) { 'setter method cannot be defined in an endless method definition' },
    :invalid_assignment => ->(args, range) {
      case range.source
      when 'self' then "Can't change the value of self"
      when 'nil' then "Can't assign to nil"
      when 'true' then "Can't assign to true"
      when 'false' then "Can't assign to false"
      when '__FILE__' then "Can't assign to __FILE__"
      when '__LINE__' then "Can't assign to __LINE__"
      when '__ENCODING__' then "Can't assign to __ENCODING__"
      else
        raise "unknown invalid_assigment to #{range.source}"
      end
    },
    :invalid_encoding => ->(args, range) { "invalid symbol in encoding #{range.source.encoding.name}" },
    :invalid_regexp => ->(args, range) { args[:message].split(':').first },
    :invalid_return => ->(args, range) { 'Invalid return in class/module body' },
    :ivar_name => ->(args, range) {
      case args[:name]
      when '@' then "`@' without identifiers is not allowed as an instance variable name"
      else
        "`#{args[:name]}' is not allowed as an instance variable name"
      end
    },
    :lvar_name => ->(args, range) { 'key must be valid as local variables' },
    :masgn_as_condition => ->(args, range) { 'masgn_as_condition' },
    :module_in_def => ->(*) { 'module definition in method body' },
    :module_name_const => ->(args, range) { 'class/module name must be CONSTANT' },
    :nth_ref_alias => ->(args, range) { "can't make alias for the number variables" },
    :numparam_used_in_outer_scope => ->(args, range) { 'numbered parameter is already used' },
    :odd_hash => ->(args, range) { 'odd_hash' },
    :ordinary_param_defined => ->(*) { 'ordinary parameter is defined' },
    :pm_interp_in_var_name => ->(*) { 'symbol literal with interpolation is not allowed' },
    :reserved_for_numparam => ->(args, range) { "#{args[:name]} is reserved for numbered parameter" },
    :singleton_literal => ->(args, range) { "can't define singleton method for literals" },
    :undefined_lvar => ->(args, range) { "#{args[:name]}: no such local variable" },
    :unexpected_percent_str => ->(args, range) { 'unknown type of %string' },
    :unexpected_token => ->(args, range) { "unexpected #{args[:token]}" },
    :unicode_point_too_large => ->(args, range) { 'invalid Unicode codepoint (too large)' },
    :unterminated_heredoc_id => ->(args, range) { 'unterminated here document identifier' },
    :useless_else => ->(args, range) { 'else without rescue is useless' },

    :embedded_document => ->(*) { 'embedded document meets end of file' },
    :triple_dot_at_eol => ->(*) { '... at EOL, should be parenthesized?' },
    :ambiguous_prefix => ->(args, range) { "ambiguous first argument; put parentheses or a space even after `#{range.source}' operator" },
    :ambiguous_regexp => ->(*) { "ambiguity between regexp and two divisions: wrap regexp in parentheses or add a space after `/' operator" },
    :ambiguous_literal => ->(args, range) {
      op = range.source
      interpreted_as =
        case op
        when "**" then "argument prefix"
        when "*" then "argument prefix"
        when "<<" then "here document"
        when "&" then "argument prefix"
        when "+" then "unary operator"
        when "-" then "unary operator"
        when ":" then "symbol literal"
        when "/" then "regexp literal"
        when "%%" then "string literal"
        else
          binding.irb
          raise "unsupported ambiguous_literal op #{op}"
        end
      "`#{op}' after local variable or literal is interpreted as binary operator even though it seems like #{interpreted_as}"
    }
  }

  def assert_diagnoses(diagnostic, code, source_maps='', versions=ParseHelper::ALL_VERSIONS)
    # Do not record errors for now
    if versions.include?(TARGET_RUBY_VERSION)
      with_versions([TARGET_RUBY_VERSION]) do |version, parser|
        source_file = Parser::Source::Buffer.new('(assert_diagnoses)', source: code)

        begin
          parser = parser.parse(source_file)
        rescue Parser::SyntaxError
          # do nothing; the diagnostic was reported
        end

        assert_equal 1, @diagnostics.count
        emitted_diagnostic = @diagnostics.first

        level = emitted_diagnostic.level
        level = :error if level == :fatal
        reason = emitted_diagnostic.reason
        arguments = emitted_diagnostic.arguments
        location = emitted_diagnostic.location

        message = RUBY_MESSAGES.fetch(reason) { raise "unknown diagnostic #{reason}" }.call(arguments, location)

        if code.split("\n").length > 1
          # importing multi-line errors is complicated
          # all of them are related:
          # 1. multiline block comments =begin/=end
          # 2. heredocs
          next
        end

        diagnostic = (' ' * location.begin_pos) + ('~' * location.size) + ' (' + level.to_s + ') ' + message

        fixture = [
          '--INPUT',
          code,
          '--DIAGNOSTIC',
          diagnostic,
          ''
        ]

        TESTS[name] << fixture
      end
    end
  end

  def locs(ast, path = [])
    return [] unless ast.is_a?(Parser::AST::Node)

    type = ast.type
    loc = ast.loc

    result =
      case loc
      when Parser::Source::Map::Send
        ranges(loc, path, :begin, :end, :selector)
      when Parser::Source::Map::Collection
        ranges(loc, path, :begin, :end)
      when Parser::Source::Map::Keyword
        ranges(loc, path, :begin, :end, :keyword)
      when Parser::Source::Map::MethodDefinition
        ranges(loc, path, :keyword, :operator, :name, :end, :assignment)
      when Parser::Source::Map::Variable
        ranges(loc, path, :operator, :name)
      when Parser::Source::Map::Operator
        ranges(loc, path, :operator)
      when Parser::Source::Map::For
        ranges(loc, path, :keyword, :end, :begin)
      when Parser::Source::Map::Constant
        ranges(loc, path, :name, :double_colon)
      when Parser::Source::Map::Condition
        ranges(loc, path, :keyword, :end, :else, :begin)
      when Parser::Source::Map::Index
        ranges(loc, path, :begin, :end, :operator)
      when Parser::Source::Map::Heredoc
        ranges(loc, path, :heredoc_body, :heredoc_end)
      when Parser::Source::Map::Ternary
        ranges(loc, path, :colon, :question)
      when Parser::Source::Map::Definition
        ranges(loc, path, :keyword, :operator, :name, :end)
      when Parser::Source::Map::RescueBody
        ranges(loc, path, :keyword, :assoc, :begin)
      else
        if loc.instance_of? Parser::Source::Map
          ranges(loc, path)
        elsif loc.nil?
          []
        else
          raise "unsupported loc type #{loc.class}"
        end
      end

    children =
    case ast.type
    when :str, :sym, :lvar, :ivar, :cvar, :gvar, :nil, :true, :false, :self,
         :int, :float, :rational, :complex, :lambda, :empty_else, :cbase,
         :back_ref, :nth_ref, :match_var, :arg, :zsuper, :kwrestarg, :forward_args, :restarg,
         :forwarded_args, :__FILE__, :__LINE__, :__ENCODING__, :regopt, :match_nil_pattern,
         :kwnilarg, :redo, :match_rest, :kwarg, :shadowarg, :blockarg, :forward_arg, :retry
      []
    when :dstr, :xstr, :dsym
      parts = *ast
      [ *parts.map.with_index { |part, idx| locs(part, path + ["part[#{idx}]"]) } ]
    when :lvasgn, :ivasgn, :cvasgn, :gvasgn
      _name, value = *ast
      [
        locs(value, path + ['value'])
      ]
    when :send, :csend
      recv, _op, *args = *ast
      [
        locs(recv, path + ['recv']),
        *args.map.with_index { |arg, idx| locs(arg, path + ["arg[#{idx}]"]) }
      ]
    when :block_pass, :splat, :defined?, :kwsplat
      value, _ = *ast
      [
        locs(value, path + ['value'])
      ]
    when :pin
      var, _ = *ast
      [
        locs(var, path + ['var'])
      ]
    when :match_current_line
      re, _ = *ast
      [
        locs(re, path + ['re'])
      ]
    when :block
      call, args, body = *ast
      [
        locs(call, path + ['call']),
        locs(args, path + ['args']),
        locs(body, path + ['body'])
      ]
    when :numblock
      call, _n, body = *ast
      [
        locs(call, path + ['call']),
        locs(body, path + ['body'])
      ]
    when :super, :yield, :return, :break, :next
      args = *ast
      [ *args.map.with_index { |arg, idx| locs(arg, path + ["arg[#{idx}]"]) }, ]
    when :kwbegin, :begin
      stmts = *ast
      [ *stmts.map.with_index { |stmt, idx| locs(stmt, path + ["stmt[#{idx}]"]) }, ]
    when :preexe, :postexe
      body, * = *ast
      [
        locs(body, path + ['body'])
      ]
    when :if_guard, :unless_guard
      cond, * = *ast
      [
        locs(cond, path + ['cond'])
      ]
    when :array, :array_pattern, :array_pattern_with_tail, :hash_pattern, :find_pattern
      elements = *ast
      [ *elements.map.with_index { |element, idx| locs(element, path + ["element[#{idx}]"]) }, ]
    when :undef
      args = *ast
      [ *args.map.with_index { |arg, idx| locs(arg, path + ["arg[#{idx}]"]) }, ]
    when :class
      name, superclass, body = *ast
      [
        locs(name, path + ['name']),
        locs(superclass, path + ['superclass']),
        locs(body, path + ['body']),
      ]
    when :and_asgn, :or_asgn, :op_asgn
      recv, value = *ast
      [
        locs(recv, path + ['recv']),
        locs(value, path + ['value']),
      ]
    when :alias
      to, from = *ast
      [
        locs(to, path + ['to']),
        locs(from, path + ['from']),
      ]
    when :case_match
      expr, *in_bodies, else_body = *ast
      [
        locs(expr, path + ['expr']),
        *in_bodies.map.with_index { |in_body, idx| locs(in_body, path + ["in_body[#{idx}]"]) },
        locs(else_body, path + ['else_body'])
      ]
    when :case
      expr, *when_bodies, else_body = *ast
      [
        locs(expr, path + ['expr']),
        *when_bodies.map.with_index { |when_body, idx| locs(when_body, path + ["when_body[#{idx}]"]) },
        locs(else_body, path + ['else_body'])
      ]
    when :masgn, :and, :or
      lhs, rhs = *ast
      [
        locs(lhs, path + ['lhs']),
        locs(rhs, path + ['rhs'])
      ]
    when :match_pattern, :match_pattern_p
      value, pattern = *ast
      [
        locs(value, path + ['value']),
        locs(pattern, path + ['pattern'])
      ]
    when :rescue
      body, *rescue_bodies, else_ = *ast
      [
        locs(body, path + ['body']),
        *rescue_bodies.map.with_index { |rescue_body, idx| locs(rescue_body, path + ["rescue_body[#{idx}]"]) },
        locs(else_, path + ['else'])
      ]
    when :defs
      definee, _mid, args, body = *ast
      [
        locs(definee, path + ['definee']),
        locs(args, path + ['args']),
        locs(body, path + ['body']),
      ]
    when :def
      _mid, args, body = *ast
      [
        locs(args, path + ['args']),
        locs(body, path + ['body']),
      ]
    when :while, :until, :while_post, :until_post
      expr, body = *ast
      [
        locs(expr, path + ['cond']),
        locs(body, path + ['body']),
      ]
    when :in_pattern
      pattern, guard, body = *ast
      [
        locs(pattern, path + ['pattern']),
        locs(guard, path + ['guard']),
        locs(body, path + ['body']),
      ]
    when :if
      cond, if_true, if_false = *ast
      [
        locs(cond, path + ['cond']),
        locs(if_true, path + ['if_true']),
        locs(if_false, path + ['if_false']),
      ]
    when :const
      scope, _name = *ast
      [ locs(scope, path + ['scope']) ]
    when :mlhs
      items = *ast
      [ *items.map.with_index { |item, idx| locs(item, path + ["item[#{idx}]"]) }, ]
    when :procarg0
      args = *ast
      [ *args.map.with_index { |arg, idx| locs(arg, path + ["arg[#{idx}]"]) }, ]
    when :irange, :erange, :iflipflop, :eflipflop
      left, right = *ast
      [
        locs(left, path + ['left']),
        locs(right, path + ['right']),
      ]
    when :sclass
      expr, body = *ast
      [
        locs(expr, path + ['expr']),
        locs(body, path + ['body']),
      ]
    when :hash, :kwargs
      pairs = *ast
      [ *pairs.map.with_index { |pair, idx| locs(pair, path + ["pair[#{idx}]"]) } ]
    when :module
      name, body = *ast
      [
        locs(name, path + ['name']),
        locs(body, path + ['body']),
      ]
    when :args
      args = *ast
      [ *args.map.with_index { |arg, idx| locs(arg, path + ["arg[#{idx}]"]) }, ]
    when :optarg, :kwoptarg
      _name, default = *ast
      [ locs(default, path + ['default']) ]
    when :index
      recv, *indexes = *ast
      [
        locs(recv, path + ['recv']),
        *indexes.map.with_index { |index, idx| locs(index, path + ["index[#{idx}]"]) }
      ]
    when :pair
      key, value = *ast
      [
        locs(key, path + ['key']),
        locs(value, path + ['value']),
      ]
    when :casgn
      scope, _name, value = *ast
      [
        locs(scope, path + ['scope']),
        locs(value, path + ['value']),
      ]
    when :resbody
      exc_list, var, body = *ast
      [
        locs(exc_list, path + ['exc_list']),
        locs(var, path + ['exc_var']),
        locs(body, path + ['body']),
      ]
    when :match_alt
      lhs, rhs = *ast
      [
        locs(lhs, path + ['lhs']),
        locs(rhs, path + ['rhs']),
      ]
    when :match_with_lvasgn
      re, value = *ast
      [
        locs(re, path + ['re']),
        locs(value, path + ['value']),
      ]
    when :indexasgn
      recv, *indexes, value = *ast
      if ast.loc.end.end_pos > value.loc.expression.end_pos
        indexes << value
        value = nil
      end
      [
        locs(recv, path + ['recv']),
        *indexes.map.with_index { |index, idx| locs(index, path + ["index[#{idx}]"]) },
        value ? locs(value, path + ['value']) : [],
      ]
    when :when
      *args, body = *ast
      [
        *args.map.with_index { |arg, idx| locs(arg, path + ["arg[#{idx}]"]) },
        locs(body, path + ['body']),
      ]
    when :regexp
      *parts, options = *ast
      [
        *parts.map.with_index { |part, idx| locs(part, path + ["part[#{idx}]"]) },
        options.children.empty? ? [] : locs(options, path + ['options']),
      ]
    when :ensure
      body, ensure_ = *ast
      [
        locs(body, path + ['body']),
        locs(ensure_, path + ['ensure']),
      ]
    when :match_as
      value, as = *ast
      [
        locs(value, path + ['value']),
        locs(as, path + ['as']),
      ]
    when :const_pattern
      const, pattern = *ast
      [
        locs(const, path + ['const']),
        locs(pattern, path + ['pattern']),
      ]
    when :for
      iterator, iteratee, body = *ast
      [
        locs(iterator, path + ['iterator']),
        locs(iteratee, path + ['iteratee']),
        locs(body, path + ['body']),
      ]
    else
      puts "unsupported node type #{ast.type}"
      binding.irb
      Kernel.exit(1)
      # locs(ast, path + [subpath])
    end

    (result + children).flatten.compact
  end

  def ranges(loc, path, *fields)
    path = path.join('/')
    if %i[and_asgn or_asgn op_asgn].include?(loc.node.type)
      # drop locs that are copied from children
      fields = [:operator]
    end

    if %i[lvar ident arg shadowarg ivar cvar gvar back_ref nth_ref].include?(loc.node.type)
      # name_l == expression_l
      fields -= [:name]
    end

    if %i[class module].include?(loc.node.type)
      # name_l is copied from name.expression_l
      fields -= [:name]
    end

    if %i[retry zsuper redo].include?(loc.node.type)
      # keyword_l == expression_l
      fields -= [:keyword]
    end

    if %i[match_with_lvasgn].include?(loc.node.type)
      # drop locs that are copied from children
      fields = [:expression]
    end

    if %i[int].include?(loc.node.type) && !loc.operator.nil? && loc.operator.source == '+'
      # `+` in `+2` is not an operator
      fields -= [:operator]
    end

    [*fields, :expression].map do |field|
      range = loc.send(field)
      if range
        ' ' * range.begin_pos + '~' * range.length + " #{field} (#{path})"
      end
    end.compact
  end
end

ParseHelper.prepend(ParseHelperPatch)

module BuilderPatch
  def initialize(*)
    super
    @emit_file_line_as_literals = false
  end
end

Parser::Builders::Default.prepend(BuilderPatch)

IGNORE = [
  # we don't support legacy behavior
  'test___ENCODING___legacy_',
  'test_emit_arg_inside_procarg0_legacy',
  'test_send_index_legacy',
  'test_send_index_asgn_legacy',
  'test_send_lambda_legacy',
  'test_endless_method_forwarded_args_legacy',
  'test_procarg0_legacy',
  'test_args_assocs_legacy',

  'test_dedenting_heredoc',
  'test_heredoc',
  'test_slash_newline_in_heredocs',
  'test_parser_slash_slash_n_escaping_in_literals',
  'test_bug_heredoc_do_0',

  # parser bug, '+1' is [:tINTEGER, '+1'], not a unary plus
  'test_unary_num_pow_precedence_0',

  # parser bug, wrong warning is emitted
  'test_send_plain_cmd_ambiguous_prefix_2',
  'test_send_plain_cmd_ambiguous_prefix_4',
  'test_send_plain_cmd_ambiguous_prefix_3',
  'test_send_plain_cmd_ambiguous_literal_0',

  # heredocs difference
  'test_ruby_bug_11989',
  'test_ruby_bug_11990',
  'test_interp_digit_var_2',
  'test_interp_digit_var_3',
  'test_interp_digit_var_8',
  'test_interp_digit_var_9',
  'test_parser_bug_640',

  # parser/MRI difference
  'test_pattern_matching_required_parentheses_for_in_match_1',
  # MRI accepts invalid chars in strings, but not symbols.
  # parser rejects them even in strings
  'test_bug_ascii_8bit_in_literal_0',
  'test_bug_ascii_8bit_in_literal_1',
  'test_bug_ascii_8bit_in_literal_2',
  'test_bug_ascii_8bit_in_literal_4',

  # we emit all diagnostics, these examples produce multiple syntax errors during error recovery
  'test_ambiuous_quoted_label_in_ternary_operator_2',
  'test_ambiuous_quoted_label_in_ternary_operator_3',
]

Minitest.after_run do
  TESTS.each do |test_name, cases|
    next if IGNORE.include?(test_name)

    cases.each_with_index do |capture, idx|
      full_test_name = "#{test_name}_#{idx}".gsub(/_{2,}/, '_')
      next if IGNORE.include?(full_test_name)
      puts "Creating input/output files for #{full_test_name}"

      input_filepath = File.join(TARGET_DIR, full_test_name)

      File.write(input_filepath, capture.join("\n"))
    end
  end
end

class Parser::AST::Node
  def inspect(indent=0)
    indented = "  " * indent
    sexp = "#{indented}s(:#{@type}"

    children.each do |child|
      child = child.to_s if child.is_a?(Symbol)

      if child.is_a?(Parser::AST::Node)
        sexp += ",\n#{child.inspect(indent + 1)}"
      else
        sexp += ", #{child.inspect.gsub("\\#") { "#" }}"
      end
    end

    sexp += ")"

    sexp
  end
end

require 'test_parser'

class TestParser
  def test_forward_args_legacy; end
end
