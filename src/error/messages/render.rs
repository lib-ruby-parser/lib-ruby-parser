use crate::error::DiagnosticMessage;

impl DiagnosticMessage {
    /// Renders DiagnosticMessage by interpolating all dynamic values into a template
    pub fn render(&self) -> String {
        // Lexer errors

        match self {
            Self::FractionAfterNumeric {} => {
                return "unexpected fraction part after numeric literal".to_string();
            }

            Self::NoDigitsAfterDot {} => {
                return "no .<digit> floating literal anymore; put 0 before dot".to_string();
            }

            Self::UnknownTypeOfPercentString {} => {
                return "unknown type of %string".to_string();
            }

            Self::NumericLiteralWithoutDigits {} => {
                return "numeric literal without digits".to_string();
            }

            Self::UnterminatedList {} => {
                return "unterminated list meets end of file".to_string();
            }

            Self::UnterminatedRegexp {} => {
                return "unterminated regexp meets end of file".to_string();
            }

            Self::UnterminatedString {} => {
                return "unterminated string meets end of file".to_string();
            }

            Self::UnterminatedQuotedString {} => {
                return "unterminated quoted string meets end of file".to_string();
            }

            Self::InvalidUnicodeEscape {} => {
                return "invalid Unicode escape".to_string();
            }

            Self::TooLargeUnicodeCodepoint {} => {
                return "invalid Unicode codepoint (too large)".to_string();
            }

            Self::InvalidUnicodeCodepoint {} => {
                return "invalid Unicode codepoint".to_string();
            }

            Self::MultipleCodepointAtSingleChar {} => {
                return "Multiple codepoints at single character literal".to_string();
            }

            Self::InvalidEscapeCharacter {} => {
                return "Invalid escape character syntax".to_string();
            }

            Self::InvalidHexEscape {} => {
                return "invalid hex escape".to_string();
            }

            Self::UnterminatedHeredoc { heredoc_id } => {
                return format!("can't find string \"{}\" anywhere before EOF", heredoc_id);
            }

            Self::UnterminatedHeredocId {} => {
                return "unterminated here document identifier".to_string();
            }

            Self::SlashRAtMiddleOfLine {} => {
                return "encountered \\r in middle of line, treated as a mere space".to_string();
            }

            Self::DStarInterpretedAsArgPrefix {} => {
                return "`**' interpreted as argument prefix".to_string();
            }

            Self::StarInterpretedAsArgPrefix {} => {
                return "`*' interpreted as argument prefix".to_string();
            }

            Self::AmpersandInterpretedAsArgPrefix {} => {
                return "`&' interpreted as argument prefix".to_string();
            }

            Self::TripleDotAtEol {} => {
                return "... at EOL, should be parenthesized?".to_string();
            }

            Self::ParenthesesIterpretedAsArglist {} => {
                return "parentheses after method name is interpreted as an argument list, not a decomposed argument"
                    .to_string();
            }

            Self::AmbiguousFirstArgument { operator } => {
                return format!(
                    "ambiguous first argument; put parentheses or a space even after `{}' operator",
                    *operator as char
                );
            }

            Self::AmbiguousOperator {
                operator,
                interpreted_as,
            } => {
                return format!(
                    "`{}' after local variable or literal is interpreted as binary operator even though it seems like {}",
                    operator,
                    interpreted_as,
                );
            }

            Self::InvalidCharacterSyntax { suggestion } => {
                return format!("invalid character syntax; use {}", suggestion);
            }

            Self::InvalidOctalDigit {} => {
                return "Invalid octal digit".to_string();
            }

            Self::TrailingCharInNumber { c } => {
                return format!("trailing `{}' in number", *c as char);
            }

            Self::EmbeddedDocumentMeetsEof {} => {
                return "embedded document meets end of file".to_string();
            }

            Self::InvalidChar { c } => {
                return format!("Invalid char `{}' in expression", *c as char);
            }

            Self::IncompleteCharacterSyntax {} => {
                return "incomplete character syntax".to_string();
            }

            Self::GvarWithoutId {} => {
                return "`$' without identifiers is not allowed as a global variable name"
                    .to_string();
            }

            Self::InvalidGvarName { c } => {
                return format!("`${}' is not allowed as a global variable name", *c as char);
            }

            Self::IvarWithoutId {} => {
                return "`@' without identifiers is not allowed as an instance variable name"
                    .to_string();
            }

            Self::InvalidIvarName { c } => {
                return format!(
                    "`@{}' is not allowed as an instance variable name",
                    *c as char
                );
            }

            Self::CvarWithoutId {} => {
                return "`@@' without identifiers is not allowed as a class variable name"
                    .to_string();
            }

            Self::InvalidCvarName { c } => {
                return format!("`@@{}' is not allowed as a class variable name", *c as char);
            }

            Self::UnknownRegexOptions { options } => {
                return format!("unknown regexp options - {}", options);
            }

            Self::AmbiguousTernaryOperator { condition } => {
                return format!(
                    "`?' just followed by `{}' is interpreted as a conditional operator, put a space after `?'",
                    condition
                );
            }

            Self::AmbiguousRegexp {} => {
                return "ambiguity between regexp and two divisions: wrap regexp in parentheses or add a space after `/' operator"
                        .to_string();
            }

            Self::UnterminatedUnicodeEscape {} => {
                return "unterminated Unicode escape".to_string();
            }

            Self::EncodingError { error } => {
                return format!("encoding error: {}", error);
            }

            Self::InvalidMultibyteChar {} => {
                return "invalid multibyte char (UTF-8)".to_string();
            }

            // Parser errors
            Self::ElseWithoutRescue {} => {
                return "else without rescue is useless".to_string();
            }

            Self::BeginNotAtTopLevel {} => {
                return "BEGIN is permitted only at toplevel".to_string();
            }

            Self::AliasNthRef {} => {
                return "can't make alias for the number variables".to_string();
            }

            Self::CsendInsideMasgn {} => {
                return "&. inside multiple assignment destination".to_string();
            }

            Self::ClassOrModuleNameMustBeConstant {} => {
                return "class/module name must be CONSTANT".to_string();
            }

            Self::EndlessSetterDefinition {} => {
                return "setter method cannot be defined in an endless method definition"
                    .to_string();
            }

            Self::UnexpectedToken { token_name } => {
                return format!("unexpected {}", token_name);
            }

            Self::ClassDefinitionInMethodBody {} => {
                return "class definition in method body".to_string();
            }

            Self::ModuleDefinitionInMethodBody {} => {
                return "module definition in method body".to_string();
            }

            Self::InvalidReturnInClassOrModuleBody {} => {
                return "Invalid return in class/module body".to_string();
            }

            Self::ConstArgument {} => {
                return "formal argument cannot be a constant".to_string();
            }

            Self::IvarArgument {} => {
                return "formal argument cannot be an instance variable".to_string();
            }

            Self::GvarArgument {} => {
                return "formal argument cannot be a global variable".to_string();
            }

            Self::CvarArgument {} => {
                return "formal argument cannot be a class variable".to_string();
            }

            Self::NoSuchLocalVariable { var_name } => {
                return format!("{}: no such local variable", var_name);
            }

            Self::OrdinaryParamDefined {} => {
                return "ordinary parameter is defined".to_string();
            }

            Self::NumparamUsed {} => {
                return "numbered parameter is already used".to_string();
            }

            Self::TokAtEolWithoutExpression { token_name } => {
                return format!("`{}' at the end of line without an expression", token_name);
            }

            // Parser warnings
            Self::EndInMethod {} => {
                return "END in method; use at_exit".to_string();
            }

            Self::ComparisonAfterComparison { comparison } => {
                return format!("comparison '{}' after comparison", comparison);
            }

            // Builder errors
            Self::CircularArgumentReference { arg_name } => {
                return format!("circular argument reference - {}", arg_name);
            }

            Self::DynamicConstantAssignment {} => {
                return "dynamic constant assignment".to_string();
            }

            Self::CantAssignToSelf {} => {
                return "Can't change the value of self".to_string();
            }

            Self::CantAssignToNil {} => {
                return "Can't assign to nil".to_string();
            }

            Self::CantAssignToTrue {} => {
                return "Can't assign to true".to_string();
            }

            Self::CantAssignToFalse {} => {
                return "Can't assign to false".to_string();
            }

            Self::CantAssignToFile {} => {
                return "Can't assign to __FILE__".to_string();
            }

            Self::CantAssignToLine {} => {
                return "Can't assign to __LINE__".to_string();
            }

            Self::CantAssignToEncoding {} => {
                return "Can't assign to __ENCODING__".to_string();
            }

            Self::CantAssignToNumparam { numparam } => {
                return format!("Can't assign to numbered parameter {}", numparam);
            }

            Self::CantSetVariable { var_name } => {
                return format!("Can't set variable {}", var_name);
            }

            Self::BlockGivenToYield {} => {
                return "block given to yield".to_string();
            }

            Self::BlockAndBlockArgGiven {} => {
                return "both block arg and actual block given".to_string();
            }

            Self::SymbolLiteralWithInterpolation {} => {
                return "symbol literal with interpolation is not allowed".to_string();
            }

            Self::ReservedForNumparam { numparam } => {
                return format!("{} is reserved for numbered parameter", numparam);
            }

            Self::KeyMustBeValidAsLocalVariable {} => {
                return "key must be valid as local variables".to_string();
            }

            Self::DuplicateVariableName {} => {
                return "duplicated variable name".to_string();
            }

            Self::DuplicateKeyName {} => {
                return "duplicated key name".to_string();
            }

            Self::SingletonLiteral {} => {
                return "can't define singleton method for literals".to_string();
            }

            Self::NthRefIsTooBig { nth_ref } => {
                return format!("`{}' is too big for a number variable, always nil", nth_ref);
            }

            Self::DuplicatedArgumentName {} => {
                return "duplicated argument name".to_string();
            }

            Self::RegexError { error } => {
                return error.to_string();
            }

            Self::InvalidSymbol { symbol } => {
                return format!("invalid symbol in encoding {}", symbol);
            }

            Self::VoidValueExpression {} => {
                return "void value expression".to_string();
            }
        }
    }
}
