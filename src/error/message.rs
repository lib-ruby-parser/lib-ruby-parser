#[derive(Debug, Clone)]
pub enum DiagnosticMessage {
    // Lexer errors
    FractionAfterNumeric,
    NoDigitsAfterDot,
    UnknownTypeOfPercentString,
    NumericLiteralWithoutDigits,
    UnterminatedList,
    UnterminatedRegexp,
    UnterminatedString,
    UnterminatedQuotedString,
    InvalidUnicodeEscape,
    TooLargeUnicodeCodepoint,
    InvalidUnicodeCodepoint,
    MultipleCodepointAtSingleChar,
    InvalidEscapeCharacter,
    InvalidHexEscape,
    UnterminatedHeredoc(String),
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
    EmbeddedDocumentMeetsEof,
    InvalidChar(u8),
    IncompleteCharacterSyntax,
    GvarWithoutId,
    InvalidGvarName(u8),
    IvarWithoutId,
    InvalidIvarName(u8),
    CvarWithoutId,
    InvalidCvarName(u8),
    UnknownRegexOptions(String),

    // Lexer warnings
    AmbiguousTernaryOperator(String),

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
    TokAtEolWithoutExpression(String),

    // Parser warnings
    EndInMethod,
    ComparisonAfterComparison(String),

    // Builder errors
    CircularArgumentReference(String),
    DynamicConstantAssignment,
    CantAssignToSelf,
    CantAssignToNil,
    CantAssignToTrue,
    CantAssignToFalse,
    CantAssignToFile,
    CantAssignToLine,
    CantAssignToEncoding,
    CantAssignToNumparam(String),
    CantSetVariable(String),
    BlockGivenToYield,
    BlockAndBlockArgGiven,
    SymbolLiteralWithInterpolation,
    ReservedForNumparam(String),
    KeyMustBeValidAsLocalVariable,
    DuplicateVariableName,
    DuplicateKeyName,
    SingletonLiteral,
    NthRefIsTooBig(String),
}

impl DiagnosticMessage {
    pub fn render(&self) -> String {
        match self {
            // Lexer errors
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
            Self::UnterminatedQuotedString => "unterminated quoted string meets end of file".to_owned(),
            Self::InvalidUnicodeEscape => "invalid Unicode escape".to_owned(),
            Self::TooLargeUnicodeCodepoint => "invalid Unicode codepoint (too large)".to_owned(),
            Self::InvalidUnicodeCodepoint => "invalid Unicode codepoint".to_owned(),
            Self::MultipleCodepointAtSingleChar => {
                "Multiple codepoints at single character literal".to_owned()
            }
            Self::InvalidEscapeCharacter => "Invalid escape character syntax".to_owned(),
            Self::InvalidHexEscape => "invalid hex escape".to_owned(),
            Self::UnterminatedHeredoc(id) => format!("can't find string \"{}\" anywhere before EOF", id),
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
                *operator as char
            ),
            Self::AmbiguousOperator { operator, interpreted_as } => format!("`{}' after local variable or literal is interpreted as binary operator even though it seems like {}", operator, interpreted_as),
            Self::InvalidCharacterSyntax { suggestion } => format!("invalid character syntax; use {}", suggestion),
            Self::InvalidOctalDigit => format!("Invalid octal digit"),
            Self::TrailingCharInNumber { c } => format!("trailing `{}' in number", *c as char),
            Self::EmbeddedDocumentMeetsEof => "embedded document meets end of file".to_owned(),
            Self::InvalidChar(byte) => format!("Invalid char `{}' in expression", *byte as char),
            Self::IncompleteCharacterSyntax => "incomplete character syntax".to_owned(),
            Self::GvarWithoutId => "`$' without identifiers is not allowed as a global variable name".to_owned(),
            Self::InvalidGvarName(name) => format!("`${}' is not allowed as a global variable name", *name as char),
            Self::IvarWithoutId => "`@' without identifiers is not allowed as an instance variable name".to_owned(),
            Self::InvalidIvarName(name) => format!("`@{}' is not allowed as an instance variable name", *name as char),
            Self::CvarWithoutId => "`@@' without identifiers is not allowed as a class variable name".to_owned(),
            Self::InvalidCvarName(name) => format!("`@@{}' is not allowed as a class variable name", *name as char),
            Self::UnknownRegexOptions(options) => format!("unknown regexp options - {}", options),

            Self::AmbiguousTernaryOperator(pre) => format!("`?' just followed by `{}' is interpreted as a conditional operator, put a space after `?'", pre),

            // Parser errors
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
            Self::NumparamUsed => "numbered parameter is already used".to_owned(),
            Self::TokAtEolWithoutExpression(tok) => format!("`{}' at the end of line without an expression", tok),

            // Parser warnings
            Self::EndInMethod => "END in method; use at_exit".to_owned(),
            Self::ComparisonAfterComparison(op) => format!("comparison '{}' after comparison", op),

            // Builder errors
            Self::CircularArgumentReference(name) => format!("circular argument reference - {}", name),
            Self::DynamicConstantAssignment => "dynamic constant assignment".to_owned(),
            Self::CantAssignToSelf => "Can't change the value of self".to_owned(),
            Self::CantAssignToNil => "Can't assign to nil".to_owned(),
            Self::CantAssignToTrue => "Can't assign to true".to_owned(),
            Self::CantAssignToFalse => "Can't assign to false".to_owned(),
            Self::CantAssignToFile => "Can't assign to __FILE__".to_owned(),
            Self::CantAssignToLine => "Can't assign to __LINE__".to_owned(),
            Self::CantAssignToEncoding => "Can't assign to __ENCODING__".to_owned(),
            Self::CantAssignToNumparam(name) => format!("Can't assign to numbered parameter {}", name),
            Self::CantSetVariable(name) => format!("Can't set variable {}", name),
            Self::BlockGivenToYield => "block given to yield".to_owned(),
            Self::BlockAndBlockArgGiven => "both block arg and actual block given".to_owned(),
            Self::SymbolLiteralWithInterpolation => "symbol literal with interpolation is not allowed".to_owned(),
            Self::ReservedForNumparam(name) => format!("{} is reserved for numbered parameter", name),
            Self::KeyMustBeValidAsLocalVariable => "key must be valid as local variables".to_owned(),
            Self::DuplicateVariableName => "duplicated variable name".to_owned(),
            Self::DuplicateKeyName => "duplicated key name".to_owned(),
            Self::SingletonLiteral => "can't define singleton method for literals".to_owned(),
            Self::NthRefIsTooBig(n) => format!("`{}' is too big for a number variable, always nil", n),
        }
    }
}
