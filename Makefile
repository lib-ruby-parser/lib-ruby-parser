codegen-tests:
	cd vendor/parser && bundle install && bundle exec rake generate
	ruby vendor/codegen/parser.rb
	ruby vendor/codegen/lexer.rb
