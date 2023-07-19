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
	cargo check --features=development

test:
	cargo test $(CARGOFLAGS)
