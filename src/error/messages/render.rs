use crate::error::DiagnosticMessage;

impl DiagnosticMessage {
    /// Renders DiagnosticMessage by interpolating all dynamic values into a template
    pub fn render(&self) -> String {
        // Lexer errors

        match self {
            Self::FractionAfterNumeric {} => {
                "unexpected fraction part after numeric literal".to_string()
            }

            Self::NoDigitsAfterDot {} => {
                "no .<digit> floating literal anymore; put 0 before dot".to_string()
            }

            Self::UnknownTypeOfPercentString {} => {
                "unknown type of %string".to_string()
            }

            Self::NumericLiteralWithoutDigits {} => {
                "numeric literal without digits".to_string()
            }

            Self::UnterminatedList {} => {
                "unterminated list meets end of file".to_string()
            }

            Self::UnterminatedRegexp {} => {
                "unterminated regexp meets end of file".to_string()
            }

            Self::UnterminatedString {} => {
                "unterminated string meets end of file".to_string()
            }

            Self::UnterminatedQuotedString {} => {
                "unterminated quoted string meets end of file".to_string()
            }

            Self::InvalidUnicodeEscape {} => {
                "invalid Unicode escape".to_string()
            }

            Self::TooLargeUnicodeCodepoint {} => {
                "invalid Unicode codepoint (too large)".to_string()
            }

            Self::InvalidUnicodeCodepoint {} => {
                "invalid Unicode codepoint".to_string()
            }

            Self::MultipleCodepointAtSingleChar {} => {
                "Multiple codepoints at single character literal".to_string()
            }

            Self::InvalidEscapeCharacter {} => {
                "Invalid escape character syntax".to_string()
            }

            Self::InvalidHexEscape {} => {
                "invalid hex escape".to_string()
            }

            Self::UnterminatedHeredoc { heredoc_id } => {
                format!("can't find string \"{}\" anywhere before EOF", heredoc_id)
            }

            Self::UnterminatedHeredocId {} => {
                "unterminated here document identifier".to_string()
            }

            Self::SlashRAtMiddleOfLine {} => {
                "encountered \\r in middle of line, treated as a mere space".to_string()
            }

            Self::DStarInterpretedAsArgPrefix {} => {
                "`**' interpreted as argument prefix".to_string()
            }

            Self::StarInterpretedAsArgPrefix {} => {
                "`*' interpreted as argument prefix".to_string()
            }

            Self::AmpersandInterpretedAsArgPrefix {} => {
                "`&' interpreted as argument prefix".to_string()
            }

            Self::TripleDotAtEol {} => {
                "... at EOL, should be parenthesized?".to_string()
            }

            Self::ParenthesesIterpretedAsArglist {} => {
                "parentheses after method name is interpreted as an argument list, not a decomposed argument"
                    .to_string()
            }

            Self::AmbiguousFirstArgument { operator } => {
                format!(
                    "ambiguous first argument; put parentheses or a space even after `{}' operator",
                    *operator as char
                )
            }

            Self::AmbiguousOperator {
                operator,
                interpreted_as,
            } => {
                format!(
                    "`{}' after local variable or literal is interpreted as binary operator even though it seems like {}",
                    operator,
                    interpreted_as,
                )
            }

            Self::InvalidCharacterSyntax { suggestion } => {
                format!("invalid character syntax; use {}", suggestion)
            }

            Self::InvalidOctalDigit {} => {
                "Invalid octal digit".to_string()
            }

            Self::TrailingCharInNumber { c } => {
                format!("trailing `{}' in number", *c as char)
            }

            Self::EmbeddedDocumentMeetsEof {} => {
                "embedded document meets end of file".to_string()
            }

            Self::InvalidChar { c } => {
                format!("Invalid char `{}' in expression", *c as char)
            }

            Self::IncompleteCharacterSyntax {} => {
                "incomplete character syntax".to_string()
            }

            Self::GvarWithoutId {} => {
                "`$' without identifiers is not allowed as a global variable name".to_string()
            }

            Self::InvalidGvarName { c } => {
                format!("`${}' is not allowed as a global variable name", *c as char)
            }

            Self::IvarWithoutId {} => {
                "`@' without identifiers is not allowed as an instance variable name".to_string()
            }

            Self::InvalidIvarName { c } => {
                format!(
                    "`@{}' is not allowed as an instance variable name",
                    *c as char
                )
            }

            Self::CvarWithoutId {} => {
                "`@@' without identifiers is not allowed as a class variable name".to_string()
            }

            Self::InvalidCvarName { c } => {
                format!("`@@{}' is not allowed as a class variable name", *c as char)
            }

            Self::UnknownRegexOptions { options } => {
                format!("unknown regexp options - {}", options)
            }

            Self::AmbiguousTernaryOperator { condition } => {
                format!(
                    "`?' just followed by `{}' is interpreted as a conditional operator, put a space after `?'",
                    condition
                )
            }

            Self::AmbiguousRegexp {} => {
                "ambiguity between regexp and two divisions: wrap regexp in parentheses or add a space after `/' operator"
                        .to_string()
            }

            Self::UnterminatedUnicodeEscape {} => {
                "unterminated Unicode escape".to_string()
            }

            Self::EncodingError { error } => {
                format!("encoding error: {}", error)
            }

            Self::InvalidMultibyteChar {} => {
                "invalid multibyte char (UTF-8)".to_string()
            }

            // Parser errors
            Self::ElseWithoutRescue {} => {
                "else without rescue is useless".to_string()
            }

            Self::BeginNotAtTopLevel {} => {
                "BEGIN is permitted only at toplevel".to_string()
            }

            Self::AliasNthRef {} => {
                "can't make alias for the number variables".to_string()
            }

            Self::CsendInsideMasgn {} => {
                "&. inside multiple assignment destination".to_string()
            }

            Self::ClassOrModuleNameMustBeConstant {} => {
                "class/module name must be CONSTANT".to_string()
            }

            Self::EndlessSetterDefinition {} => {
                "setter method cannot be defined in an endless method definition".to_string()
            }

            Self::InvalidIdToGet { identifier } => {
                format!("identifier {} is not valid to get", identifier)
            }

            Self::ForwardArgAfterRestarg {} => {
                "... after rest argument".to_string()
            }

            Self::NoAnonymousBlockarg {} => {
                "no anonymous block parameter".to_string()
            }

            Self::UnexpectedToken { token_name } => {
                format!("unexpected {}", token_name)
            }

            Self::ClassDefinitionInMethodBody {} => {
                "class definition in method body".to_string()
            }

            Self::ModuleDefinitionInMethodBody {} => {
                "module definition in method body".to_string()
            }

            Self::InvalidReturnInClassOrModuleBody {} => {
                "Invalid return in class/module body".to_string()
            }

            Self::ConstArgument {} => {
                "formal argument cannot be a constant".to_string()
            }

            Self::IvarArgument {} => {
                "formal argument cannot be an instance variable".to_string()
            }

            Self::GvarArgument {} => {
                "formal argument cannot be a global variable".to_string()
            }

            Self::CvarArgument {} => {
                "formal argument cannot be a class variable".to_string()
            }

            Self::NoSuchLocalVariable { var_name } => {
                format!("{}: no such local variable", var_name)
            }

            Self::OrdinaryParamDefined {} => {
                "ordinary parameter is defined".to_string()
            }

            Self::NumparamUsed {} => {
                "numbered parameter is already used".to_string()
            }

            Self::TokAtEolWithoutExpression { token_name } => {
                format!("`{}' at the end of line without an expression", token_name)
            }

            // Parser warnings
            Self::EndInMethod {} => {
                "END in method; use at_exit".to_string()
            }

            Self::ComparisonAfterComparison { comparison } => {
                format!("comparison '{}' after comparison", comparison)
            }

            Self::DuplicateHashKey {} => {
                "key is duplicated and overwritten".to_string()
            }

            // Builder errors
            Self::CircularArgumentReference { arg_name } => {
                format!("circular argument reference - {}", arg_name)
            }

            Self::DynamicConstantAssignment {} => {
                "dynamic constant assignment".to_string()
            }

            Self::CantAssignToSelf {} => {
                "Can't change the value of self".to_string()
            }

            Self::CantAssignToNil {} => {
                "Can't assign to nil".to_string()
            }

            Self::CantAssignToTrue {} => {
                "Can't assign to true".to_string()
            }

            Self::CantAssignToFalse {} => {
                "Can't assign to false".to_string()
            }

            Self::CantAssignToFile {} => {
                "Can't assign to __FILE__".to_string()
            }

            Self::CantAssignToLine {} => {
                "Can't assign to __LINE__".to_string()
            }

            Self::CantAssignToEncoding {} => {
                "Can't assign to __ENCODING__".to_string()
            }

            Self::CantAssignToNumparam { numparam } => {
                format!("Can't assign to numbered parameter {}", numparam)
            }

            Self::CantSetVariable { var_name } => {
                format!("Can't set variable {}", var_name)
            }

            Self::BlockGivenToYield {} => {
                "block given to yield".to_string()
            }

            Self::BlockAndBlockArgGiven {} => {
                "both block arg and actual block given".to_string()
            }

            Self::SymbolLiteralWithInterpolation {} => {
                "symbol literal with interpolation is not allowed".to_string()
            }

            Self::ReservedForNumparam { numparam } => {
                format!("{} is reserved for numbered parameter", numparam)
            }

            Self::KeyMustBeValidAsLocalVariable {} => {
                "key must be valid as local variables".to_string()
            }

            Self::DuplicateVariableName {} => {
                "duplicated variable name".to_string()
            }

            Self::DuplicateKeyName {} => {
                "duplicated key name".to_string()
            }

            Self::SingletonLiteral {} => {
                "can't define singleton method for literals".to_string()
            }

            Self::NthRefIsTooBig { nth_ref } => {
                format!("`{}' is too big for a number variable, always nil", nth_ref)
            }

            Self::DuplicatedArgumentName {} => {
                "duplicated argument name".to_string()
            }

            Self::RegexError { error } => {
                error.to_string()
            }

            Self::InvalidSymbol { symbol } => {
                format!("invalid symbol in encoding {}", symbol)
            }

            Self::VoidValueExpression {} => {
                "void value expression".to_string()
            }
        }
    }
}
