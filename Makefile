GENERATED_FILES = src/parser/parse.rs
GENERATED_FILES += src/error/messages/message_enum.rs
GENERATED_FILES += src/nodes/node_enum.rs
GENERATED_FILES += src/nodes/types/*.rs
GENERATED_FILES += src/reserved_words/list.rs
GENERATED_FILES += src/traverse/visitor/visit_gen.rs
GENERATED_FILES += src/traverse/finder/finder_gen.rs

clean:
	rm -f $(GENERATED_FILES)

codegen-tests:
	cd vendor/parser && bundle install && bundle exec rake generate
	ruby vendor/codegen/parser.rb
	ruby vendor/codegen/lexer.rb

codegen-rust:
	cargo build --features="lib-ruby-parser/codegen-y,lib-ruby-parser/codegen-rust" --package lib-ruby-parser

test:
	cargo test --package tests --package lib-ruby-parser $(CARGOFLAGS)

test-cov:
	cargo tarpaulin -v --packages tests,lib-ruby-parser --out Xml

lib-ruby-parser/src/parser/parse.rs:
	# codegen parse.y -> parse.rs
	cargo build --package lib-ruby-parser

target/tokens.rs: lib-ruby-parser/src/parser/parse.rs
	# generate target/tokens.rs using cbindgen
	cargo run --bin prepare_token_ids --package scripts

create-codegen: target/tokens.rs
	# create `codegen` executable
	cargo build --bin codegen --features codegen --release --package scripts

.PHONY: codegen
