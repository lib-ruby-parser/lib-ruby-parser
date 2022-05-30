codegen-tests:
	cd vendor/parser && bundle install && bundle exec rake generate
	ruby vendor/codegen/parser.rb
	ruby vendor/codegen/lexer.rb

test:
	cargo test --package tests --package lib-ruby-parser $(CARGOFLAGS)

test-cov:
	cargo tarpaulin -v --packages tests,lib-ruby-parser --out Xml

lib-ruby-parser/src/parser/parse.rs:
	# codegen parse.y -> parse.rs
	cargo build --package lib-ruby-parser

target/tokens.rs: lib-ruby-parser/src/parser/parse.rs
	# generate target/tokens.rs using cbindgen
	cargo run --bin prepare_token_ids --package examples

create-codegen: target/tokens.rs
	# create `codegen` executable
	cargo build --bin codegen --features codegen --profile release --package examples

.PHONY: codegen
