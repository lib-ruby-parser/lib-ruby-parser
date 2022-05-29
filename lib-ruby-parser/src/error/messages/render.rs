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

#[allow(non_snake_case)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_render_FractionAfterNumeric() {
        assert_eq!(
            DiagnosticMessage::FractionAfterNumeric {}.render(),
            "unexpected fraction part after numeric literal",
        );
    }
    #[test]
    fn test_render_NoDigitsAfterDot() {
        assert_eq!(
            DiagnosticMessage::NoDigitsAfterDot {}.render(),
            "no .<digit> floating literal anymore; put 0 before dot",
        );
    }
    #[test]
    fn test_render_UnknownTypeOfPercentString() {
        assert_eq!(
            DiagnosticMessage::UnknownTypeOfPercentString {}.render(),
            "unknown type of %string",
        );
    }
    #[test]
    fn test_render_NumericLiteralWithoutDigits() {
        assert_eq!(
            DiagnosticMessage::NumericLiteralWithoutDigits {}.render(),
            "numeric literal without digits",
        );
    }
    #[test]
    fn test_render_UnterminatedList() {
        assert_eq!(
            DiagnosticMessage::UnterminatedList {}.render(),
            "unterminated list meets end of file",
        );
    }
    #[test]
    fn test_render_UnterminatedRegexp() {
        assert_eq!(
            DiagnosticMessage::UnterminatedRegexp {}.render(),
            "unterminated regexp meets end of file",
        );
    }
    #[test]
    fn test_render_UnterminatedString() {
        assert_eq!(
            DiagnosticMessage::UnterminatedString {}.render(),
            "unterminated string meets end of file",
        );
    }
    #[test]
    fn test_render_UnterminatedQuotedString() {
        assert_eq!(
            DiagnosticMessage::UnterminatedQuotedString {}.render(),
            "unterminated quoted string meets end of file",
        );
    }
    #[test]
    fn test_render_InvalidUnicodeEscape() {
        assert_eq!(
            DiagnosticMessage::InvalidUnicodeEscape {}.render(),
            "invalid Unicode escape",
        );
    }
    #[test]
    fn test_render_TooLargeUnicodeCodepoint() {
        assert_eq!(
            DiagnosticMessage::TooLargeUnicodeCodepoint {}.render(),
            "invalid Unicode codepoint (too large)",
        );
    }
    #[test]
    fn test_render_InvalidUnicodeCodepoint() {
        assert_eq!(
            DiagnosticMessage::InvalidUnicodeCodepoint {}.render(),
            "invalid Unicode codepoint",
        );
    }
    #[test]
    fn test_render_MultipleCodepointAtSingleChar() {
        assert_eq!(
            DiagnosticMessage::MultipleCodepointAtSingleChar {}.render(),
            "Multiple codepoints at single character literal",
        );
    }
    #[test]
    fn test_render_InvalidEscapeCharacter() {
        assert_eq!(
            DiagnosticMessage::InvalidEscapeCharacter {}.render(),
            "Invalid escape character syntax",
        );
    }
    #[test]
    fn test_render_InvalidHexEscape() {
        assert_eq!(
            DiagnosticMessage::InvalidHexEscape {}.render(),
            "invalid hex escape",
        );
    }

    #[test]
    fn test_render_UnterminatedHeredoc() {
        assert_eq!(
            DiagnosticMessage::UnterminatedHeredoc {
                heredoc_id: String::from("FOO")
            }
            .render(),
            "can't find string \"FOO\" anywhere before EOF",
        );
    }
    #[test]
    fn test_render_UnterminatedHeredocId() {
        assert_eq!(
            DiagnosticMessage::UnterminatedHeredocId {}.render(),
            "unterminated here document identifier",
        );
    }
    #[test]
    fn test_render_SlashRAtMiddleOfLine() {
        assert_eq!(
            DiagnosticMessage::SlashRAtMiddleOfLine {}.render(),
            "encountered \\r in middle of line, treated as a mere space",
        );
    }
    #[test]
    fn test_render_DStarInterpretedAsArgPrefix() {
        assert_eq!(
            DiagnosticMessage::DStarInterpretedAsArgPrefix {}.render(),
            "`**' interpreted as argument prefix",
        );
    }
    #[test]
    fn test_render_StarInterpretedAsArgPrefix() {
        assert_eq!(
            DiagnosticMessage::StarInterpretedAsArgPrefix {}.render(),
            "`*' interpreted as argument prefix",
        );
    }
    #[test]
    fn test_render_AmpersandInterpretedAsArgPrefix() {
        assert_eq!(
            DiagnosticMessage::AmpersandInterpretedAsArgPrefix {}.render(),
            "`&' interpreted as argument prefix",
        );
    }
    #[test]
    fn test_render_TripleDotAtEol() {
        assert_eq!(
            DiagnosticMessage::TripleDotAtEol {}.render(),
            "... at EOL, should be parenthesized?",
        );
    }
    #[test]
    fn test_render_ParenthesesIterpretedAsArglist() {
        assert_eq!(
        DiagnosticMessage::ParenthesesIterpretedAsArglist {}.render(),
        "parentheses after method name is interpreted as an argument list, not a decomposed argument",
    );
    }
    #[test]
    fn test_render_AmbiguousFirstArgument() {
        assert_eq!(
            DiagnosticMessage::AmbiguousFirstArgument { operator: b'+' }.render(),
            "ambiguous first argument; put parentheses or a space even after `+' operator",
        );
    }
    #[test]
    fn test_render_AmbiguousOperator() {
        assert_eq!(
        DiagnosticMessage::AmbiguousOperator {
            operator: String::from("+"),
            interpreted_as: String::from("-")
        }
        .render(),
        "`+' after local variable or literal is interpreted as binary operator even though it seems like -",
    );
    }
    #[test]
    fn test_render_InvalidCharacterSyntax() {
        assert_eq!(
            DiagnosticMessage::InvalidCharacterSyntax {
                suggestion: String::from("foo")
            }
            .render(),
            "invalid character syntax; use foo",
        );
    }
    #[test]
    fn test_render_InvalidOctalDigit() {
        assert_eq!(
            DiagnosticMessage::InvalidOctalDigit {}.render(),
            "Invalid octal digit"
        );
    }
    #[test]
    fn test_render_TrailingCharInNumber() {
        assert_eq!(
            DiagnosticMessage::TrailingCharInNumber { c: b'!' }.render(),
            "trailing `!' in number",
        );
    }
    #[test]
    fn test_render_EmbeddedDocumentMeetsEof() {
        assert_eq!(
            DiagnosticMessage::EmbeddedDocumentMeetsEof {}.render(),
            "embedded document meets end of file",
        );
    }
    #[test]
    fn test_render_InvalidChar() {
        assert_eq!(
            DiagnosticMessage::InvalidChar { c: b'!' }.render(),
            "Invalid char `!' in expression",
        );
    }
    #[test]
    fn test_render_IncompleteCharacterSyntax() {
        assert_eq!(
            DiagnosticMessage::IncompleteCharacterSyntax {}.render(),
            "incomplete character syntax",
        );
    }
    #[test]
    fn test_render_GvarWithoutId() {
        assert_eq!(
            DiagnosticMessage::GvarWithoutId {}.render(),
            "`$' without identifiers is not allowed as a global variable name",
        );
    }
    #[test]
    fn test_render_InvalidGvarName() {
        assert_eq!(
            DiagnosticMessage::InvalidGvarName { c: b'!' }.render(),
            "`$!' is not allowed as a global variable name",
        );
    }
    #[test]
    fn test_render_IvarWithoutId() {
        assert_eq!(
            DiagnosticMessage::IvarWithoutId {}.render(),
            "`@' without identifiers is not allowed as an instance variable name",
        );
    }
    #[test]
    fn test_render_InvalidIvarName() {
        assert_eq!(
            DiagnosticMessage::InvalidIvarName { c: b'!' }.render(),
            "`@!' is not allowed as an instance variable name",
        );
    }
    #[test]
    fn test_render_CvarWithoutId() {
        assert_eq!(
            DiagnosticMessage::CvarWithoutId {}.render(),
            "`@@' without identifiers is not allowed as a class variable name",
        );
    }
    #[test]
    fn test_render_InvalidCvarName() {
        assert_eq!(
            DiagnosticMessage::InvalidCvarName { c: b'!' }.render(),
            "`@@!' is not allowed as a class variable name",
        );
    }
    #[test]
    fn test_render_UnknownRegexOptions() {
        assert_eq!(
            DiagnosticMessage::UnknownRegexOptions {
                options: String::from("foo")
            }
            .render(),
            "unknown regexp options - foo",
        );
    }
    #[test]
    fn test_render_AmbiguousTernaryOperator() {
        assert_eq!(
            DiagnosticMessage::AmbiguousTernaryOperator {
                condition: String::from("foo")
            }
            .render(),
            "`?' just followed by `foo' is interpreted as a conditional operator, put a space after `?'",
        );
    }
    #[test]
    fn test_render_AmbiguousRegexp() {
        assert_eq!(DiagnosticMessage::AmbiguousRegexp {}.render(), "ambiguity between regexp and two divisions: wrap regexp in parentheses or add a space after `/' operator",);
    }
    #[test]
    fn test_render_UnterminatedUnicodeEscape() {
        assert_eq!(
            DiagnosticMessage::UnterminatedUnicodeEscape {}.render(),
            "unterminated Unicode escape",
        );
    }
    #[test]
    fn test_render_EncodingError() {
        assert_eq!(
            DiagnosticMessage::EncodingError {
                error: String::from("foo")
            }
            .render(),
            "encoding error: foo",
        );
    }
    #[test]
    fn test_render_InvalidMultibyteChar() {
        assert_eq!(
            DiagnosticMessage::InvalidMultibyteChar {}.render(),
            "invalid multibyte char (UTF-8)",
        );
    }
    #[test]
    fn test_render_ElseWithoutRescue() {
        assert_eq!(
            DiagnosticMessage::ElseWithoutRescue {}.render(),
            "else without rescue is useless",
        );
    }
    #[test]
    fn test_render_BeginNotAtTopLevel() {
        assert_eq!(
            DiagnosticMessage::BeginNotAtTopLevel {}.render(),
            "BEGIN is permitted only at toplevel",
        );
    }
    #[test]
    fn test_render_AliasNthRef() {
        assert_eq!(
            DiagnosticMessage::AliasNthRef {}.render(),
            "can't make alias for the number variables",
        );
    }
    #[test]
    fn test_render_CsendInsideMasgn() {
        assert_eq!(
            DiagnosticMessage::CsendInsideMasgn {}.render(),
            "&. inside multiple assignment destination",
        );
    }
    #[test]
    fn test_render_ClassOrModuleNameMustBeConstant() {
        assert_eq!(
            DiagnosticMessage::ClassOrModuleNameMustBeConstant {}.render(),
            "class/module name must be CONSTANT",
        );
    }
    #[test]
    fn test_render_EndlessSetterDefinition() {
        assert_eq!(
            DiagnosticMessage::EndlessSetterDefinition {}.render(),
            "setter method cannot be defined in an endless method definition",
        );
    }
    #[test]
    fn test_render_InvalidIdToGet() {
        assert_eq!(
            DiagnosticMessage::InvalidIdToGet {
                identifier: String::from("foo")
            }
            .render(),
            "identifier foo is not valid to get",
        );
    }
    #[test]
    fn test_render_ForwardArgAfterRestarg() {
        assert_eq!(
            DiagnosticMessage::ForwardArgAfterRestarg {}.render(),
            "... after rest argument",
        );
    }
    #[test]
    fn test_render_NoAnonymousBlockarg() {
        assert_eq!(
            DiagnosticMessage::NoAnonymousBlockarg {}.render(),
            "no anonymous block parameter",
        );
    }
    #[test]
    fn test_render_UnexpectedToken() {
        assert_eq!(
            DiagnosticMessage::UnexpectedToken {
                token_name: String::from("tUNKNOWN")
            }
            .render(),
            "unexpected tUNKNOWN",
        );
    }
    #[test]
    fn test_render_ClassDefinitionInMethodBody() {
        assert_eq!(
            DiagnosticMessage::ClassDefinitionInMethodBody {}.render(),
            "class definition in method body",
        );
    }
    #[test]
    fn test_render_ModuleDefinitionInMethodBody() {
        assert_eq!(
            DiagnosticMessage::ModuleDefinitionInMethodBody {}.render(),
            "module definition in method body",
        );
    }
    #[test]
    fn test_render_InvalidReturnInClassOrModuleBody() {
        assert_eq!(
            DiagnosticMessage::InvalidReturnInClassOrModuleBody {}.render(),
            "Invalid return in class/module body",
        );
    }
    #[test]
    fn test_render_ConstArgument() {
        assert_eq!(
            DiagnosticMessage::ConstArgument {}.render(),
            "formal argument cannot be a constant",
        );
    }
    #[test]
    fn test_render_IvarArgument() {
        assert_eq!(
            DiagnosticMessage::IvarArgument {}.render(),
            "formal argument cannot be an instance variable",
        );
    }
    #[test]
    fn test_render_GvarArgument() {
        assert_eq!(
            DiagnosticMessage::GvarArgument {}.render(),
            "formal argument cannot be a global variable",
        );
    }
    #[test]
    fn test_render_CvarArgument() {
        assert_eq!(
            DiagnosticMessage::CvarArgument {}.render(),
            "formal argument cannot be a class variable",
        );
    }
    #[test]
    fn test_render_NoSuchLocalVariable() {
        assert_eq!(
            DiagnosticMessage::NoSuchLocalVariable {
                var_name: String::from("foo")
            }
            .render(),
            "foo: no such local variable",
        );
    }
    #[test]
    fn test_render_OrdinaryParamDefined() {
        assert_eq!(
            DiagnosticMessage::OrdinaryParamDefined {}.render(),
            "ordinary parameter is defined",
        );
    }
    #[test]
    fn test_render_NumparamUsed() {
        assert_eq!(
            DiagnosticMessage::NumparamUsed {}.render(),
            "numbered parameter is already used",
        );
    }
    #[test]
    fn test_render_TokAtEolWithoutExpression() {
        assert_eq!(
            DiagnosticMessage::TokAtEolWithoutExpression {
                token_name: String::from("tTOKEN")
            }
            .render(),
            "`tTOKEN' at the end of line without an expression",
        );
    }
    #[test]
    fn test_render_EndInMethod() {
        assert_eq!(
            DiagnosticMessage::EndInMethod {}.render(),
            String::from("END in method; use at_exit"),
        );
    }
    #[test]
    fn test_render_ComparisonAfterComparison() {
        assert_eq!(
            DiagnosticMessage::ComparisonAfterComparison {
                comparison: String::from("<=>")
            }
            .render(),
            "comparison '<=>' after comparison",
        );
    }
    #[test]
    fn test_render_DuplicateHashKey() {
        assert_eq!(
            DiagnosticMessage::DuplicateHashKey {}.render(),
            "key is duplicated and overwritten",
        );
    }
    #[test]
    fn test_render_CircularArgumentReference() {
        assert_eq!(
            DiagnosticMessage::CircularArgumentReference {
                arg_name: String::from("foo")
            }
            .render(),
            "circular argument reference - foo",
        );
    }
    #[test]
    fn test_render_DynamicConstantAssignment() {
        assert_eq!(
            DiagnosticMessage::DynamicConstantAssignment {}.render(),
            "dynamic constant assignment",
        );
    }
    #[test]
    fn test_render_CantAssignToSelf() {
        assert_eq!(
            DiagnosticMessage::CantAssignToSelf {}.render(),
            "Can't change the value of self",
        );
    }
    #[test]
    fn test_render_CantAssignToNil() {
        assert_eq!(
            DiagnosticMessage::CantAssignToNil {}.render(),
            "Can't assign to nil",
        );
    }
    #[test]
    fn test_render_CantAssignToTrue() {
        assert_eq!(
            DiagnosticMessage::CantAssignToTrue {}.render(),
            "Can't assign to true",
        );
    }
    #[test]
    fn test_render_CantAssignToFalse() {
        assert_eq!(
            DiagnosticMessage::CantAssignToFalse {}.render(),
            "Can't assign to false",
        );
    }
    #[test]
    fn test_render_CantAssignToFile() {
        assert_eq!(
            DiagnosticMessage::CantAssignToFile {}.render(),
            "Can't assign to __FILE__",
        );
    }
    #[test]
    fn test_render_CantAssignToLine() {
        assert_eq!(
            DiagnosticMessage::CantAssignToLine {}.render(),
            "Can't assign to __LINE__",
        );
    }
    #[test]
    fn test_render_CantAssignToEncoding() {
        assert_eq!(
            DiagnosticMessage::CantAssignToEncoding {}.render(),
            "Can't assign to __ENCODING__",
        );
    }
    #[test]
    fn test_render_CantAssignToNumparam() {
        assert_eq!(
            DiagnosticMessage::CantAssignToNumparam {
                numparam: String::from("_42")
            }
            .render(),
            "Can't assign to numbered parameter _42",
        );
    }
    #[test]
    fn test_render_CantSetVariable() {
        assert_eq!(
            DiagnosticMessage::CantSetVariable {
                var_name: String::from("foo")
            }
            .render(),
            "Can't set variable foo",
        );
    }
    #[test]
    fn test_render_BlockGivenToYield() {
        assert_eq!(
            DiagnosticMessage::BlockGivenToYield {}.render(),
            "block given to yield",
        );
    }
    #[test]
    fn test_render_BlockAndBlockArgGiven() {
        assert_eq!(
            DiagnosticMessage::BlockAndBlockArgGiven {}.render(),
            "both block arg and actual block given",
        );
    }
    #[test]
    fn test_render_SymbolLiteralWithInterpolation() {
        assert_eq!(
            DiagnosticMessage::SymbolLiteralWithInterpolation {}.render(),
            "symbol literal with interpolation is not allowed",
        );
    }
    #[test]
    fn test_render_ReservedForNumparam() {
        assert_eq!(
            DiagnosticMessage::ReservedForNumparam {
                numparam: String::from("_42")
            }
            .render(),
            "_42 is reserved for numbered parameter",
        );
    }
    #[test]
    fn test_render_KeyMustBeValidAsLocalVariable() {
        assert_eq!(
            DiagnosticMessage::KeyMustBeValidAsLocalVariable {}.render(),
            "key must be valid as local variables",
        );
    }
    #[test]
    fn test_render_DuplicateVariableName() {
        assert_eq!(
            DiagnosticMessage::DuplicateVariableName {}.render(),
            "duplicated variable name",
        );
    }
    #[test]
    fn test_render_DuplicateKeyName() {
        assert_eq!(
            DiagnosticMessage::DuplicateKeyName {}.render(),
            "duplicated key name",
        );
    }
    #[test]
    fn test_render_SingletonLiteral() {
        assert_eq!(
            DiagnosticMessage::SingletonLiteral {}.render(),
            "can't define singleton method for literals",
        );
    }
    #[test]
    fn test_render_NthRefIsTooBig() {
        assert_eq!(
            DiagnosticMessage::NthRefIsTooBig {
                nth_ref: String::from("42")
            }
            .render(),
            "`42' is too big for a number variable, always nil",
        );
    }
    #[test]
    fn test_render_DuplicatedArgumentName() {
        assert_eq!(
            DiagnosticMessage::DuplicatedArgumentName {}.render(),
            "duplicated argument name",
        );
    }
    #[test]
    fn test_render_RegexError() {
        assert_eq!(
            DiagnosticMessage::RegexError {
                error: String::from("foo")
            }
            .render(),
            "foo",
        );
    }
    #[test]
    fn test_render_InvalidSymbol() {
        assert_eq!(
            DiagnosticMessage::InvalidSymbol {
                symbol: String::from("foo")
            }
            .render(),
            "invalid symbol in encoding foo",
        );
    }
    #[test]
    fn test_render_VoidValueExpression() {
        assert_eq!(
            DiagnosticMessage::VoidValueExpression {}.render(),
            "void value expression",
        );
    }
}
