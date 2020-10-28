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

    // Parser errors
    ElseWithoutRescue,
    BeginNotAtTopLevel,
    AliasNthRef,
    CsendInsideMasgn,
    ClassOrModuleNameMustBeConstant,
    EndlessSetterDefinition,
    UnexpectedToken(String),
    ClassDefinitionInMethodBody,
    ModuleDefinitionInMethodBody,
    InvalidReturnInClassOrModuleBody,
    ConstArgument,
    IvarArgument,
    GvarArgument,
    CvarArgument,
    NoSuchLocalVariable(String),
    OrdinaryParamDefined,
    NumparamUsed,
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

            Self::ElseWithoutRescue => "else without rescue is useless".to_owned(),
            Self::BeginNotAtTopLevel => "BEGIN is permitted only at toplevel".to_owned(),
            Self::AliasNthRef => "can't make alias for the number variables".to_owned(),
            Self::CsendInsideMasgn => "&. inside multiple assignment destination".to_owned(),
            Self::ClassOrModuleNameMustBeConstant => "class/module name must be CONSTANT".to_owned(),
            Self::EndlessSetterDefinition => "setter method cannot be defined in an endless method definition".to_owned(),
            Self::UnexpectedToken(tok) => format!("unexpected {}", tok),
            Self::ClassDefinitionInMethodBody => "class definition in method body".to_owned(),
            Self::ModuleDefinitionInMethodBody => "module definition in method body".to_owned(),
            Self::InvalidReturnInClassOrModuleBody => "Invalid return in class/module body".to_owned(),
            Self::ConstArgument => "formal argument cannot be a constant".to_owned(),
            Self::IvarArgument => "formal argument cannot be an instance variable".to_owned(),
            Self::GvarArgument => "formal argument cannot be a global variable".to_owned(),
            Self::CvarArgument => "formal argument cannot be a class variable".to_owned(),
            Self::NoSuchLocalVariable(name) => format!("{}: no such local variable", name),
            Self::OrdinaryParamDefined => "ordinary parameter is defined".to_owned(),
            Self::NumparamUsed => "numbered parameter is already used".to_owned()
        }
    }
}
