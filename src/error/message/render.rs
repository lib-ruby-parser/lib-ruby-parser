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

        if self.is_unterminated_heredoc() {
            return format!(
                "can't find string \"{}\" anywhere before EOF",
                self.unterminated_heredoc_get_heredoc_id().as_str()
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

        if self.is_ambiguous_first_argument() {
            return format!(
                "ambiguous first argument; put parentheses or a space even after `{}' operator",
                self.ambiguous_first_argument_get_operator() as char
            );
        }

        if self.is_ambiguous_operator() {
            return format!(
                "`{}' after local variable or literal is interpreted as binary operator even though it seems like {}",
                self.ambiguous_operator_get_operator().as_str(),
                self.ambiguous_operator_get_interpreted_as().as_str(),
            );
        }

        if self.is_invalid_character_syntax() {
            return format!(
                "invalid character syntax; use {}",
                self.invalid_character_syntax_get_suggestion().as_str()
            );
        }

        if self.is_invalid_octal_digit() {
            return "Invalid octal digit".to_string();
        }

        if self.is_trailing_char_in_number() {
            return format!(
                "trailing `{}' in number",
                self.trailing_char_in_number_get_c() as char
            );
        }

        if self.is_embedded_document_meets_eof() {
            return "embedded document meets end of file".to_string();
        }

        if self.is_invalid_char() {
            return format!(
                "Invalid char `{}' in expression",
                self.invalid_char_get_c() as char
            );
        }

        if self.is_incomplete_character_syntax() {
            return "incomplete character syntax".to_string();
        }

        if self.is_gvar_without_id() {
            return "`$' without identifiers is not allowed as a global variable name".to_string();
        }

        if self.is_invalid_gvar_name() {
            return format!(
                "`${}' is not allowed as a global variable name",
                self.invalid_gvar_name_get_c() as char
            );
        }

        if self.is_ivar_without_id() {
            return "`@' without identifiers is not allowed as an instance variable name"
                .to_string();
        }

        if self.is_invalid_ivar_name() {
            return format!(
                "`@{}' is not allowed as an instance variable name",
                self.invalid_ivar_name_get_c() as char
            );
        }

        if self.is_cvar_without_id() {
            return "`@@' without identifiers is not allowed as a class variable name".to_string();
        }

        if self.is_invalid_cvar_name() {
            return format!(
                "`@@{}' is not allowed as a class variable name",
                self.invalid_cvar_name_get_c() as char
            );
        }

        if self.is_unknown_regex_options() {
            return format!(
                "unknown regexp options - {}",
                self.unknown_regex_options_get_options().as_str()
            );
        }

        if self.is_ambiguous_ternary_operator() {
            return format!(
                "`?' just followed by `{}' is interpreted as a conditional operator, put a space after `?'",
                self.ambiguous_ternary_operator_get_condition().as_str()
            );
        }

        if self.is_ambiguous_regexp() {
            return "ambiguity between regexp and two divisions: wrap regexp in parentheses or add a space after `/' operator"
                    .to_string();
        }

        if self.is_unterminated_unicode_escape() {
            return "unterminated Unicode escape".to_string();
        }

        if self.is_encoding_error() {
            return format!(
                "encoding error: {}",
                self.encoding_error_get_error().as_str()
            );
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

        if self.is_unexpected_token() {
            return format!(
                "unexpected {}",
                self.unexpected_token_get_token_name().as_str()
            );
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

        if self.is_no_such_local_variable() {
            return format!(
                "{}: no such local variable",
                self.no_such_local_variable_get_var_name().as_str()
            );
        }

        if self.is_ordinary_param_defined() {
            return "ordinary parameter is defined".to_string();
        }

        if self.is_numparam_used() {
            return "numbered parameter is already used".to_string();
        }

        if self.is_tok_at_eol_without_expression() {
            return format!(
                "`{}' at the end of line without an expression",
                self.tok_at_eol_without_expression_get_token_name().as_str()
            );
        }

        // Parser warnings
        if self.is_end_in_method() {
            return "END in method; use at_exit".to_string();
        }

        if self.is_comparison_after_comparison() {
            return format!(
                "comparison '{}' after comparison",
                self.comparison_after_comparison_get_comparison().as_str()
            );
        }

        // // Builder errors
        if self.is_circular_argument_reference() {
            return format!(
                "circular argument reference - {}",
                self.circular_argument_reference_get_arg_name().as_str()
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

        if self.is_cant_assign_to_numparam() {
            return format!(
                "Can't assign to numbered parameter {}",
                self.cant_assign_to_numparam_get_numparam().as_str()
            );
        }

        if self.is_cant_set_variable() {
            return format!(
                "Can't set variable {}",
                self.cant_set_variable_get_var_name().as_str()
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

        if self.is_reserved_for_numparam() {
            return format!(
                "{} is reserved for numbered parameter",
                self.reserved_for_numparam_get_numparam().as_str()
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

        if self.is_nth_ref_is_too_big() {
            return format!(
                "`{}' is too big for a number variable, always nil",
                self.nth_ref_is_too_big_get_nth_ref().as_str()
            );
        }

        if self.is_duplicated_argument_name() {
            return "duplicated argument name".to_string();
        }

        if self.is_regex_error() {
            return self.regex_error_get_error().as_str().to_string();
        }

        if self.is_invalid_symbol() {
            return format!(
                "invalid symbol in encoding {}",
                self.invalid_symbol_get_symbol().as_str()
            );
        }

        if self.is_void_value_expression() {
            return "void value expression".to_string();
        }

        unreachable!("Unknown type of diagnostic message")
    }
}
