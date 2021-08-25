use lib_ruby_parser::debug_level;

#[derive(Debug, Clone, Copy)]
pub struct DebugLevel {
    pub level: u8,
}

impl DebugLevel {
    pub const ABOUT: &'static str =
        "supported values are 'parser', 'lexer', 'buffer' (comma-separated)";
}

impl std::str::FromStr for DebugLevel {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut level = debug_level::NONE;
        for part in s.split(",") {
            level |= match part {
                "parser" => debug_level::PARSER,
                "lexer" => debug_level::LEXER,
                "buffer" => debug_level::BUFFER,
                _ => return Err(Self::ABOUT),
            }
        }
        Ok(Self { level })
    }
}

impl Default for DebugLevel {
    fn default() -> Self {
        Self {
            level: debug_level::NONE,
        }
    }
}
