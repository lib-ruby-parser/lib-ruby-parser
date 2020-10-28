#[derive(Debug, Clone)]
pub enum DiagnosticMessage {
    FractionAfterNumeric,
    NoDigitsAfterDot,
    UnknownTypeOfPercentString,
    NumericLiteralWithoutDigits,
    UnterminatedList,
    UnterminatedRegexp,
    UnterminatedString,
    InvalidUnicodeEscape,
    TooLargeUnicodeCodepoint,
    InvalidUnicodeCodepoint,
    MultipleCodepointAtSingleChar,
    InvalidEscapeCharacter,
    InvalidHexEscape,
    UnterminatedHeredocId,
    SlashRAtMiddleOfLine,
    DStarInterpretedAsArgPrefix,
    StarInterpretedAsArgPrefix,
    AmpersandInterpretedAsArgPrefix,
    TripleDotAtEol,
    ParenthesesIterpretedAsArglist,
    AmbiguousFirstArgument {
        operator: u8,
    },
    AmbiguousOperator {
        operator: &'static str,
        interpreted_as: &'static str,
    },
    InvalidCharacterSyntax {
        suggestion: String,
    },
    InvalidOctalDigit,
    TrailingCharInNumber {
        c: u8,
    },
}

impl DiagnosticMessage {
    pub fn render(&self) -> String {
        match self {
            Self::FractionAfterNumeric => {
                "unexpected fraction part after numeric literal".to_owned()
            }
            Self::NoDigitsAfterDot => {
                "no .<digit> floating literal anymore; put 0 before dot".to_owned()
            }
            Self::UnknownTypeOfPercentString => "unknown type of %string".to_owned(),
            Self::NumericLiteralWithoutDigits => "numeric literal without digits".to_owned(),
            Self::UnterminatedList => "unterminated list meets end of file".to_owned(),
            Self::UnterminatedRegexp => "unterminated regexp meets end of file".to_owned(),
            Self::UnterminatedString => "unterminated string meets end of file".to_owned(),
            Self::InvalidUnicodeEscape => "invalid Unicode escape".to_owned(),
            Self::TooLargeUnicodeCodepoint => "invalid Unicode codepoint (too large)".to_owned(),
            Self::InvalidUnicodeCodepoint => "invalid Unicode codepoint".to_owned(),
            Self::MultipleCodepointAtSingleChar => {
                "Multiple codepoints at single character literal".to_owned()
            }
            Self::InvalidEscapeCharacter => "Invalid escape character syntax".to_owned(),
            Self::InvalidHexEscape => "invalid hex escape".to_owned(),
            Self::UnterminatedHeredocId => "unterminated here document identifier".to_owned(),
            Self::SlashRAtMiddleOfLine => {
                "encountered \\r in middle of line, treated as a mere space".to_owned()
            }
            Self::DStarInterpretedAsArgPrefix => "`**' interpreted as argument prefix".to_owned(),
            Self::StarInterpretedAsArgPrefix => "`*' interpreted as argument prefix".to_owned(),
            Self::AmpersandInterpretedAsArgPrefix => "`&' interpreted as argument prefix".to_owned(),
            Self::TripleDotAtEol => "... at EOL, should be parenthesized?".to_owned(),
            Self::ParenthesesIterpretedAsArglist => "parentheses after method name is interpreted as an argument list, not a decomposed argument".to_owned(),
            Self::AmbiguousFirstArgument { operator} => format!(
                "ambiguous first argument; put parentheses or a space even after `{}' operator",
                operator
            ),
            Self::AmbiguousOperator { operator, interpreted_as } => format!("`{}' after local variable or literal is interpreted as binary operator even though it seems like {}", operator, interpreted_as),
            Self::InvalidCharacterSyntax { suggestion } => format!("invalid character syntax; use {}", suggestion),
            Self::InvalidOctalDigit => format!("Invalid octal digit"),
            Self::TrailingCharInNumber { c } => format!("trailing `{}' in number", c),
        }
    }
}
