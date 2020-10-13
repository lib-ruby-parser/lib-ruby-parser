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
    node.updated(nil, [node.location.expression.source.gsub('_', '')])
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

      TESTS[name] << { input: code, ast: parsed_ast.inspect, locs: locs(parsed_ast) }
    end
  end

  def assert_diagnoses(diagnostic, code, source_maps='', versions=ParseHelper::ALL_VERSIONS)
    # Do not record errors for now
    return

    if versions.include?(TARGET_RUBY_VERSION)
      with_versions([TARGET_RUBY_VERSION]) do |version, parser|
        level, reason, arguments = diagnostic
        arguments ||= {}
        message     = Parser::MESSAGES[reason] % arguments

        if code.split("\n").length > 1
          # importing multi-line errors is complicated
          # all of them are related:
          # 1. multiline block comments =begin/=end
          # 2. heredocs
          next
        end

        if level == :error
          input = "#{code} # error: #{message}"

          TESTS[name] << { input: input }
        end
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
         :kwnilarg, :redo, :match_rest, :kwarg, :shadowarg, :blockarg, :forward_arg
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
    when :block_pass, :splat, :defined?, :match_current_line, :kwsplat, :pin
      value, _ = *ast
      [
        locs(value, path + ['value'])
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
    when :break, :super, :retry, :next, :yield, :return
      value, _ = *ast
      [
        locs(value, path + ['value'])
      ]
    when :kwbegin, :begin, :preexe, :postexe, :if_guard, :unless_guard
      body, * = *ast
      [
        locs(body, path + ['body'])
      ]
    when :array, :array_pattern, :undef, :array_pattern_with_tail, :hash_pattern, :find_pattern
      items = *ast
      [ *items.map.with_index { |item, idx| locs(item, path + ["item[#{idx}]"]) }, ]
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
      from, to = *ast
      [
        locs(from, path + ['from']),
        locs(to, path + ['to']),
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
    when :masgn, :and, :or, :in_match
      lhs, rhs = *ast
      [
        locs(lhs, path + ['lhs']),
        locs(rhs, path + ['rhs'])
      ]
    when :rescue
      body, *rescue_bodies, else_ = *ast
      [
        locs(body, path + ['body']),
        *rescue_bodies.map.with_index { |rescue_body, idx| locs(rescue_body, path + ["rescue_body[#{idx}]"]) },
        locs(else_, path + ['else'])
      ]
    when :defs
      recv, _mid, args, body = *ast
      [
        locs(recv, path + ['recv']),
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
        locs(expr, path + ['expr']),
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
      cond, truthy, falsey = *ast
      [
        locs(cond, path + ['cond']),
        locs(truthy, path + ['truthy']),
        locs(falsey, path + ['falsey']),
      ]
    when :const
      scope, _name = *ast
      [ locs(scope, path + ['scope']) ]
    when :mlhs, :procarg0
      items = *ast
      [ *items.map.with_index { |item, idx| locs(item, path + ["item[#{idx}]"]) }, ]
    when :irange, :erange, :iflipflop, :eflipflop
      left, right = *ast
      [
        locs(left, path + ['left']),
        locs(right, path + ['right']),
      ]
    when :sclass
      of, body = *ast
      [
        locs(of, path + ['of']),
        locs(body, path + ['body']),
      ]
    when :hash
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
        locs(var, path + ['var']),
        locs(body, path + ['body']),
      ]
    when :match_alt
      left, right = *ast
      [
        locs(left, path + ['left']),
        locs(right, path + ['right']),
      ]
    when :match_with_lvasgn
      re, value = *ast
      [
        locs(re, path + ['re']),
        locs(value, path + ['value']),
      ]
    when :indexasgn
      recv, *indexes, value = *ast
      [
        locs(recv, path + ['recv']),
        *indexes.map.with_index { |index, idx| locs(index, path + ["index[#{idx}]"]) },
        locs(value, path + ['value']),
      ]
    when :when
      cond, body = *ast
      [
        locs(cond, path + ['cond']),
        locs(body, path + ['body']),
      ]
    when :regexp
      src, opts = *ast
      [
        locs(src, path + ['src']),
        locs(opts, path + ['opts']),
      ]
    when :ensure
      body, ensure_body = *ast
      [
        locs(body, path + ['body']),
        locs(ensure_body, path + ['ensure_body']),
      ]
    when :match_as
      value, var = *ast
      [
        locs(value, path + ['value']),
        locs(var, path + ['var']),
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
      fields = [:operator]
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

  'test_dedenting_heredoc',
  'test_heredoc',
  'test_slash_newline_in_heredocs',
  'test_parser_slash_slash_n_escaping_in_literals',
  'test_bug_heredoc_do_0',

  # parser bug
  'test_unary_num_pow_precedence_0',

  # heredocs difference
  'test_ruby_bug_11989',
  'test_ruby_bug_11990',
  'test_interp_digit_var_2',
  'test_interp_digit_var_3',
  'test_interp_digit_var_8',
  'test_interp_digit_var_9',
  'test_parser_bug_640',
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
        '--LOCATIONS',
        capture[:locs].join("\n"),
        '--AST',
        capture[:ast]
      ]

      fixture << ''

      File.write(input_filepath, fixture.join("\n"))
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
