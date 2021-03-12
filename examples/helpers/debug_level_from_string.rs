use lib_ruby_parser::debug_level;

#[allow(dead_code)]
pub fn debug_level_from_string(s: &Option<String>) -> debug_level::Type {
    match s {
        Some(s) => {
            let mut result = debug_level::NONE;
            for part in s.split(",") {
                result |= match part {
                    "parser" => debug_level::PARSER,
                    "lexer" => debug_level::LEXER,
                    "buffer" => debug_level::BUFFER,
                    unsupported => {
                        panic!(
                            "unsupported debug_level {:?}, supported are parser, lexer, buffer",
                            unsupported
                        )
                    }
                }
            }
            result
        }
        None => debug_level::NONE,
    }
}
