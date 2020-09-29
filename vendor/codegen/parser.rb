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

puts ARGV.inspect

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

  # we emit flags as a string s(:regopt, "mix")
  def on_regopt(node)
    node.updated(nil, [node.children.join])
  end

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

      TESTS[name] << { input: code, output: parsed_ast.inspect }
    end
  end

  def assert_diagnoses(diagnostic, code, source_maps='', versions=ParseHelper::ALL_VERSIONS)
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
end

ParseHelper.prepend(ParseHelperPatch)

class Minitest::Test
  IGNORE = [
    'test___ENCODING___legacy_',
  ]

  def after_teardown
    TESTS.each do |test_name, cases|
      next if IGNORE.include?(test_name)

      cases.each_with_index do |capture, idx|
        full_test_name = "#{test_name}_#{idx}".gsub(/_{2,}/, '_')
        # puts "Creating input/output files for #{full_test_name}"

        input_filepath = File.join(TARGET_DIR, full_test_name)

        File.write(input_filepath, <<-TEXT)
--INPUT
#{capture[:input]}
--AST
#{capture[:output]}
TEXT
      end
    end
  end
end

class Parser::AST::Node
  def inspect(indent=0)
    indented = "  " * indent
    sexp = "#{indented}s(:#{@type}"

    children.each do |child|
      if child.is_a?(Parser::AST::Node)
        sexp += ",\n#{child.inspect(indent + 1)}"
      elsif child.is_a?(Symbol)
        sexp += ", #{child.to_s.inspect}"
      else
        sexp += ", #{child.inspect}"
      end
    end

    sexp += ")"

    sexp
  end
end

require 'test_parser'
