use crate::error::DiagnosticMessage;

impl DiagnosticMessage {
    /// Renders DiagnosticMessage by interpolating all dynamic values into a template
    pub fn render(&self) -> String {
        // Lexer errors

        if self.is_fraction_after_numeric() {
            return "unexpected fraction part after numeric literal".to_string();
        }

        if self.is_no_digits_after_dot() {
            return "no .<digit> floating literal anymore; put 0 before dot".to_string();
        }

        if self.is_unknown_type_of_percent_string() {
            return "unknown type of %string".to_string();
        }

        if self.is_numeric_literal_without_digits() {
            return "numeric literal without digits".to_string();
        }

        if self.is_unterminated_list() {
            return "unterminated list meets end of file".to_string();
        }

        if self.is_unterminated_regexp() {
            return "unterminated regexp meets end of file".to_string();
        }

        if self.is_unterminated_string() {
            return "unterminated string meets end of file".to_string();
        }

        if self.is_unterminated_quoted_string() {
            return "unterminated quoted string meets end of file".to_string();
        }

        if self.is_invalid_unicode_escape() {
            return "invalid Unicode escape".to_string();
        }

        if self.is_too_large_unicode_codepoint() {
            return "invalid Unicode codepoint (too large)".to_string();
        }

        if self.is_invalid_unicode_codepoint() {
            return "invalid Unicode codepoint".to_string();
        }

        if self.is_multiple_codepoint_at_single_char() {
            return "Multiple codepoints at single character literal".to_string();
        }

        if self.is_invalid_escape_character() {
            return "Invalid escape character syntax".to_string();
        }

        if self.is_invalid_hex_escape() {
            return "invalid hex escape".to_string();
        }

        if let Some(unterminated_heredoc) = self.as_unterminated_heredoc() {
            return format!(
                "can't find string \"{}\" anywhere before EOF",
                unterminated_heredoc.get_heredoc_id().as_str()
            );
        }

        if self.is_unterminated_heredoc_id() {
            return "unterminated here document identifier".to_string();
        }

        if self.is_slash_r_at_middle_of_line() {
            return "encountered \\r in middle of line, treated as a mere space".to_string();
        }

        if self.is_d_star_interpreted_as_arg_prefix() {
            return "`**' interpreted as argument prefix".to_string();
        }

        if self.is_star_interpreted_as_arg_prefix() {
            return "`*' interpreted as argument prefix".to_string();
        }

        if self.is_ampersand_interpreted_as_arg_prefix() {
            return "`&' interpreted as argument prefix".to_string();
        }

        if self.is_triple_dot_at_eol() {
            return "... at EOL, should be parenthesized?".to_string();
        }

        if self.is_parentheses_iterpreted_as_arglist() {
            return "parentheses after method name is interpreted as an argument list, not a decomposed argument"
                .to_string();
        }

        if let Some(ambiguous_first_argument) = self.as_ambiguous_first_argument() {
            return format!(
                "ambiguous first argument; put parentheses or a space even after `{}' operator",
                *ambiguous_first_argument.get_operator() as char
            );
        }

        if let Some(ambiguous_operator) = self.as_ambiguous_operator() {
            return format!(
                "`{}' after local variable or literal is interpreted as binary operator even though it seems like {}",
                ambiguous_operator.get_operator().as_str(),
                ambiguous_operator.get_interpreted_as().as_str(),
            );
        }

        if let Some(invalid_character_syntax) = self.as_invalid_character_syntax() {
            return format!(
                "invalid character syntax; use {}",
                invalid_character_syntax.get_suggestion().as_str()
            );
        }

        if self.is_invalid_octal_digit() {
            return "Invalid octal digit".to_string();
        }

        if let Some(trailing_char_in_number) = self.as_trailing_char_in_number() {
            return format!(
                "trailing `{}' in number",
                *trailing_char_in_number.get_c() as char
            );
        }

        if self.is_embedded_document_meets_eof() {
            return "embedded document meets end of file".to_string();
        }

        if let Some(invalid_char) = self.as_invalid_char() {
            return format!(
                "Invalid char `{}' in expression",
                *invalid_char.get_c() as char
            );
        }

        if self.is_incomplete_character_syntax() {
            return "incomplete character syntax".to_string();
        }

        if self.is_gvar_without_id() {
            return "`$' without identifiers is not allowed as a global variable name".to_string();
        }

        if let Some(invalid_gvar_name) = self.as_invalid_gvar_name() {
            return format!(
                "`${}' is not allowed as a global variable name",
                *invalid_gvar_name.get_c() as char
            );
        }

        if self.is_ivar_without_id() {
            return "`@' without identifiers is not allowed as an instance variable name"
                .to_string();
        }

        if let Some(invalid_ivar_name) = self.as_invalid_ivar_name() {
            return format!(
                "`@{}' is not allowed as an instance variable name",
                *invalid_ivar_name.get_c() as char
            );
        }

        if self.is_cvar_without_id() {
            return "`@@' without identifiers is not allowed as a class variable name".to_string();
        }

        if let Some(invalid_cvar_name) = self.as_invalid_cvar_name() {
            return format!(
                "`@@{}' is not allowed as a class variable name",
                *invalid_cvar_name.get_c() as char
            );
        }

        if let Some(unknown_regex_options) = self.as_unknown_regex_options() {
            return format!(
                "unknown regexp options - {}",
                unknown_regex_options.get_options().as_str()
            );
        }

        if let Some(ambiguous_ternary_operator) = self.as_ambiguous_ternary_operator() {
            return format!(
                "`?' just followed by `{}' is interpreted as a conditional operator, put a space after `?'",
                ambiguous_ternary_operator.get_condition().as_str()
            );
        }

        if self.is_ambiguous_regexp() {
            return "ambiguity between regexp and two divisions: wrap regexp in parentheses or add a space after `/' operator"
                    .to_string();
        }

        if self.is_unterminated_unicode_escape() {
            return "unterminated Unicode escape".to_string();
        }

        if let Some(encoding_error) = self.as_encoding_error() {
            return format!("encoding error: {}", encoding_error.get_error().as_str());
        }

        if self.is_invalid_multibyte_char() {
            return "invalid multibyte char (UTF-8)".to_string();
        }

        // Parser errors

        if self.is_else_without_rescue() {
            return "else without rescue is useless".to_string();
        }

        if self.is_begin_not_at_top_level() {
            return "BEGIN is permitted only at toplevel".to_string();
        }

        if self.is_alias_nth_ref() {
            return "can't make alias for the number variables".to_string();
        }

        if self.is_csend_inside_masgn() {
            return "&. inside multiple assignment destination".to_string();
        }

        if self.is_class_or_module_name_must_be_constant() {
            return "class/module name must be CONSTANT".to_string();
        }

        if self.is_endless_setter_definition() {
            return "setter method cannot be defined in an endless method definition".to_string();
        }

        if let Some(unexpected_token) = self.as_unexpected_token() {
            return format!("unexpected {}", unexpected_token.get_token_name().as_str());
        }

        if self.is_class_definition_in_method_body() {
            return "class definition in method body".to_string();
        }

        if self.is_module_definition_in_method_body() {
            return "module definition in method body".to_string();
        }

        if self.is_invalid_return_in_class_or_module_body() {
            return "Invalid return in class/module body".to_string();
        }

        if self.is_const_argument() {
            return "formal argument cannot be a constant".to_string();
        }

        if self.is_ivar_argument() {
            return "formal argument cannot be an instance variable".to_string();
        }

        if self.is_gvar_argument() {
            return "formal argument cannot be a global variable".to_string();
        }

        if self.is_cvar_argument() {
            return "formal argument cannot be a class variable".to_string();
        }

        if let Some(no_such_local_variable) = self.as_no_such_local_variable() {
            return format!(
                "{}: no such local variable",
                no_such_local_variable.get_var_name().as_str()
            );
        }

        if self.is_ordinary_param_defined() {
            return "ordinary parameter is defined".to_string();
        }

        if self.is_numparam_used() {
            return "numbered parameter is already used".to_string();
        }

        if let Some(tok_at_eol_without_expression) = self.as_tok_at_eol_without_expression() {
            return format!(
                "`{}' at the end of line without an expression",
                tok_at_eol_without_expression.get_token_name().as_str()
            );
        }

        // Parser warnings
        if self.is_end_in_method() {
            return "END in method; use at_exit".to_string();
        }

        if let Some(comparison_after_comparison) = self.as_comparison_after_comparison() {
            return format!(
                "comparison '{}' after comparison",
                comparison_after_comparison.get_comparison().as_str()
            );
        }

        // Builder errors
        if let Some(circular_argument_reference) = self.as_circular_argument_reference() {
            return format!(
                "circular argument reference - {}",
                circular_argument_reference.get_arg_name().as_str()
            );
        }

        if self.is_dynamic_constant_assignment() {
            return "dynamic constant assignment".to_string();
        }

        if self.is_cant_assign_to_self() {
            return "Can't change the value of self".to_string();
        }

        if self.is_cant_assign_to_nil() {
            return "Can't assign to nil".to_string();
        }

        if self.is_cant_assign_to_true() {
            return "Can't assign to true".to_string();
        }

        if self.is_cant_assign_to_false() {
            return "Can't assign to false".to_string();
        }

        if self.is_cant_assign_to_file() {
            return "Can't assign to __FILE__".to_string();
        }

        if self.is_cant_assign_to_line() {
            return "Can't assign to __LINE__".to_string();
        }

        if self.is_cant_assign_to_encoding() {
            return "Can't assign to __ENCODING__".to_string();
        }

        if let Some(cant_assign_to_numparam) = self.as_cant_assign_to_numparam() {
            return format!(
                "Can't assign to numbered parameter {}",
                cant_assign_to_numparam.get_numparam().as_str()
            );
        }

        if let Some(cant_set_variable) = self.as_cant_set_variable() {
            return format!(
                "Can't set variable {}",
                cant_set_variable.get_var_name().as_str()
            );
        }

        if self.is_block_given_to_yield() {
            return "block given to yield".to_string();
        }

        if self.is_block_and_block_arg_given() {
            return "both block arg and actual block given".to_string();
        }

        if self.is_symbol_literal_with_interpolation() {
            return "symbol literal with interpolation is not allowed".to_string();
        }

        if let Some(reserved_for_numparam) = self.as_reserved_for_numparam() {
            return format!(
                "{} is reserved for numbered parameter",
                reserved_for_numparam.get_numparam().as_str()
            );
        }

        if self.is_key_must_be_valid_as_local_variable() {
            return "key must be valid as local variables".to_string();
        }

        if self.is_duplicate_variable_name() {
            return "duplicated variable name".to_string();
        }

        if self.is_duplicate_key_name() {
            return "duplicated key name".to_string();
        }

        if self.is_singleton_literal() {
            return "can't define singleton method for literals".to_string();
        }

        if let Some(nth_ref_is_too_big) = self.as_nth_ref_is_too_big() {
            return format!(
                "`{}' is too big for a number variable, always nil",
                nth_ref_is_too_big.get_nth_ref().as_str()
            );
        }

        if self.is_duplicated_argument_name() {
            return "duplicated argument name".to_string();
        }

        if let Some(regex_error) = self.as_regex_error() {
            return regex_error.get_error().as_str().to_string();
        }

        if let Some(invalid_symbol) = self.as_invalid_symbol() {
            return format!(
                "invalid symbol in encoding {}",
                invalid_symbol.get_symbol().as_str()
            );
        }

        if self.is_void_value_expression() {
            return "void value expression".to_string();
        }

        unreachable!("Unknown type of diagnostic message")
    }
}
