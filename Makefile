codegen:
	cd vendor/parser && rake generate
	ruby vendor/codegen/parser.rb
	ruby vendor/codegen/lexer.rb

test:
	echo "foo"

.PHONY: codegen
