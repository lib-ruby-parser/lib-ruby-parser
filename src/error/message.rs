use crate::error::DiagnosticMessage;

impl DiagnosticMessage {
    /// Renders DiagnosticMessage by interpolating all dynamic values into a template
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
            Self::UnterminatedQuotedString => {
                "unterminated quoted string meets end of file".to_owned()
            }
            Self::InvalidUnicodeEscape => "invalid Unicode escape".to_owned(),
            Self::TooLargeUnicodeCodepoint => "invalid Unicode codepoint (too large)".to_owned(),
            Self::InvalidUnicodeCodepoint => "invalid Unicode codepoint".to_owned(),
            Self::MultipleCodepointAtSingleChar => {
                "Multiple codepoints at single character literal".to_owned()
            }
            Self::InvalidEscapeCharacter => "Invalid escape character syntax".to_owned(),
            Self::InvalidHexEscape => "invalid hex escape".to_owned(),
            Self::UnterminatedHeredoc { heredoc_id } => {
                format!("can't find string \"{}\" anywhere before EOF", heredoc_id)
            }
            Self::UnterminatedHeredocId => "unterminated here document identifier".to_owned(),
            Self::SlashRAtMiddleOfLine => {
                "encountered \\r in middle of line, treated as a mere space".to_owned()
            }
            Self::DStarInterpretedAsArgPrefix => "`**' interpreted as argument prefix".to_owned(),
            Self::StarInterpretedAsArgPrefix => "`*' interpreted as argument prefix".to_owned(),
            Self::AmpersandInterpretedAsArgPrefix => {
                "`&' interpreted as argument prefix".to_owned()
            }
            Self::TripleDotAtEol => "... at EOL, should be parenthesized?".to_owned(),
            Self::ParenthesesIterpretedAsArglist => {
                "parentheses after method name is interpreted as \
an argument list, not a decomposed argument"
                    .to_owned()
            }
            Self::AmbiguousFirstArgument { operator } => format!(
                "ambiguous first argument; put parentheses or a space even after `{}' operator",
                *operator as char
            ),
            Self::AmbiguousOperator {
                operator,
                interpreted_as,
            } => format!(
                "`{}' after local variable or \
literal is interpreted as binary operator even though it seems like {}",
                operator, interpreted_as
            ),
            Self::InvalidCharacterSyntax { suggestion } => {
                format!("invalid character syntax; use {}", suggestion)
            }
            Self::InvalidOctalDigit => "Invalid octal digit".to_owned(),
            Self::TrailingCharInNumber { c } => format!("trailing `{}' in number", *c as char),
            Self::EmbeddedDocumentMeetsEof => "embedded document meets end of file".to_owned(),
            Self::InvalidChar { c } => format!("Invalid char `{}' in expression", *c as char),
            Self::IncompleteCharacterSyntax => "incomplete character syntax".to_owned(),
            Self::GvarWithoutId => {
                "`$' without identifiers is not allowed as a global variable name".to_owned()
            }
            Self::InvalidGvarName { c } => {
                format!("`${}' is not allowed as a global variable name", *c as char)
            }
            Self::IvarWithoutId => {
                "`@' without identifiers is not allowed as an instance variable name".to_owned()
            }
            Self::InvalidIvarName { c } => format!(
                "`@{}' is not allowed as an instance variable name",
                *c as char
            ),
            Self::CvarWithoutId => {
                "`@@' without identifiers is not allowed as a class variable name".to_owned()
            }
            Self::InvalidCvarName { c } => {
                format!("`@@{}' is not allowed as a class variable name", *c as char)
            }
            Self::UnknownRegexOptions { options } => {
                format!("unknown regexp options - {}", options)
            }
            Self::AmbiguousTernaryOperator { condition } => format!(
                "`?' just followed by `{}' is interpreted as \
a conditional operator, put a space after `?'",
                condition
            ),
            Self::AmbiguousRegexp => "ambiguity between regexp and two divisions: wrap \
regexp in parentheses or add a space after `/' operator"
                .to_owned(),
            Self::UnterminatedUnicodeEscape => "unterminated Unicode escape".to_owned(),
            Self::EncodingError { error } => format!("encoding error: {}", error),

            // // Parser errors
            Self::ElseWithoutRescue => "else without rescue is useless".to_owned(),
            Self::BeginNotAtTopLevel => "BEGIN is permitted only at toplevel".to_owned(),
            Self::AliasNthRef => "can't make alias for the number variables".to_owned(),
            Self::CsendInsideMasgn => "&. inside multiple assignment destination".to_owned(),
            Self::ClassOrModuleNameMustBeConstant => {
                "class/module name must be CONSTANT".to_owned()
            }
            Self::EndlessSetterDefinition => {
                "setter method cannot be defined in an endless method definition".to_owned()
            }
            Self::UnexpectedToken { token_name } => format!("unexpected {}", token_name),
            Self::ClassDefinitionInMethodBody => "class definition in method body".to_owned(),
            Self::ModuleDefinitionInMethodBody => "module definition in method body".to_owned(),
            Self::InvalidReturnInClassOrModuleBody => {
                "Invalid return in class/module body".to_owned()
            }
            Self::ConstArgument => "formal argument cannot be a constant".to_owned(),
            Self::IvarArgument => "formal argument cannot be an instance variable".to_owned(),
            Self::GvarArgument => "formal argument cannot be a global variable".to_owned(),
            Self::CvarArgument => "formal argument cannot be a class variable".to_owned(),
            Self::NoSuchLocalVariable { var_name } => {
                format!("{}: no such local variable", var_name)
            }
            Self::OrdinaryParamDefined => "ordinary parameter is defined".to_owned(),
            Self::NumparamUsed => "numbered parameter is already used".to_owned(),
            Self::TokAtEolWithoutExpression { token_name } => {
                format!("`{}' at the end of line without an expression", token_name)
            }

            // Parser warnings
            Self::EndInMethod => "END in method; use at_exit".to_owned(),
            Self::ComparisonAfterComparison { comparison } => {
                format!("comparison '{}' after comparison", comparison)
            }
            // // Builder errors
            Self::CircularArgumentReference { arg_name } => {
                format!("circular argument reference - {}", arg_name)
            }
            Self::DynamicConstantAssignment => "dynamic constant assignment".to_owned(),
            Self::CantAssignToSelf => "Can't change the value of self".to_owned(),
            Self::CantAssignToNil => "Can't assign to nil".to_owned(),
            Self::CantAssignToTrue => "Can't assign to true".to_owned(),
            Self::CantAssignToFalse => "Can't assign to false".to_owned(),
            Self::CantAssignToFile => "Can't assign to __FILE__".to_owned(),
            Self::CantAssignToLine => "Can't assign to __LINE__".to_owned(),
            Self::CantAssignToEncoding => "Can't assign to __ENCODING__".to_owned(),
            Self::CantAssignToNumparam { numparam } => {
                format!("Can't assign to numbered parameter {}", numparam)
            }
            Self::CantSetVariable { var_name } => format!("Can't set variable {}", var_name),
            Self::BlockGivenToYield => "block given to yield".to_owned(),
            Self::BlockAndBlockArgGiven => "both block arg and actual block given".to_owned(),
            Self::SymbolLiteralWithInterpolation => {
                "symbol literal with interpolation is not allowed".to_owned()
            }
            Self::ReservedForNumparam { numparam } => {
                format!("{} is reserved for numbered parameter", numparam)
            }
            Self::KeyMustBeValidAsLocalVariable => {
                "key must be valid as local variables".to_owned()
            }
            Self::DuplicateVariableName => "duplicated variable name".to_owned(),
            Self::DuplicateKeyName => "duplicated key name".to_owned(),
            Self::SingletonLiteral => "can't define singleton method for literals".to_owned(),
            Self::NthRefIsTooBig { nth_ref } => {
                format!("`{}' is too big for a number variable, always nil", nth_ref)
            }
            Self::DuplicatedArgumentName => "duplicated argument name".to_owned(),
            Self::RegexError { error } => error.to_owned(),
            Self::InvalidSymbol { symbol } => format!("invalid symbol in encoding {}", symbol),
            Self::VoidValueExpression => "void value expression".to_owned(),
        }
    }
}
