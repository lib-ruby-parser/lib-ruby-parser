use ruby_parser::Message;

// Lexer errors
#[test]
fn test_render_unicode_point_too_large() {
    assert_eq!(Message::UnicodePointTooLarge {}.render(), String::from("invalid Unicode codepoint (too large)"));
}

#[test]
fn test_render_invalid_escape() {
    assert_eq!(Message::InvalidEscape {}.render(), String::from("invalid escape character syntax"));
}

#[test]
fn test_render_incomplete_escape() {
    assert_eq!(Message::IncompleteEscape {}.render(), String::from("incomplete character syntax"));
}

#[test]
fn test_render_invalid_hex_escape() {
    assert_eq!(Message::InvalidHexEscape {}.render(), String::from("invalid hex escape"));
}

#[test]
fn test_render_invalid_unicode_escape() {
    assert_eq!(Message::InvalidUnicodeEscape {}.render(), String::from("invalid Unicode escape"));
}

#[test]
fn test_render_unterminated_unicode() {
    assert_eq!(Message::UnterminatedUnicode {}.render(), String::from("unterminated Unicode escape"));
}

#[test]
fn test_render_escape_eof() {
    assert_eq!(Message::EscapeEof {}.render(), String::from("escape sequence meets end of file"));
}

#[test]
fn test_render_string_eof() {
    assert_eq!(Message::StringEof {}.render(), String::from("unterminated string meets end of file"));
}

#[test]
fn test_render_regexp_options() {
    assert_eq!(Message::RegexpOptions { options: "u".into() }.render(), String::from("unknown regexp options: u"));
}

#[test]
fn test_render_cvar_name() {
    assert_eq!(Message::CvarName { name: "@@cvar".into() }.render(), String::from("`@@cvar' is not allowed as a class variable name"));
}

#[test]
fn test_render_ivar_name() {
    assert_eq!(Message::IvarName { name: "@ivar".into() }.render(), String::from("`@ivar' is not allowed as an instance variable name"));
}

#[test]
fn test_render_trailing_in_number() {
    assert_eq!(Message::TrailingInNumber { character: "a".into() }.render(), String::from("trailing `a' in number"));
}

#[test]
fn test_render_empty_numeric() {
    assert_eq!(Message::EmptyNumeric {}.render(), String::from("numeric literal without digits"));
}

#[test]
fn test_render_invalid_octal() {
    assert_eq!(Message::InvalidOctal {}.render(), String::from("invalid octal digit"));
}

#[test]
fn test_render_no_dot_digit_literal() {
    assert_eq!(Message::NoDotDigitLiteral {}.render(), String::from("no .<digit> floating literal anymore; put 0 before dot"));
}

#[test]
fn test_render_bare_backslash() {
    assert_eq!(Message::BareBackslash {}.render(), String::from("bare backslash only allowed before newline"));
}

#[test]
fn test_render_unexpected() {
    assert_eq!(Message::Unexpected { character: "tINTEGER".into() }.render(), String::from("unexpected `tINTEGER'"));
}

#[test]
fn test_render_embedded_document() {
    assert_eq!(Message::EmbeddedDocument {}.render(), String::from("embedded document meets end of file (and they embark on a romantic journey)"));
}

#[test]
fn test_render_heredoc_id_has_newline() {
    assert_eq!(Message::HeredocIdHasNewline {}.render(), String::from("here document identifier across newlines, never match"));
}

#[test]
fn test_render_heredoc_id_ends_with_nl() {
    assert_eq!(Message::HeredocIdEndsWithNl {}.render(), String::from("here document identifier ends with a newline"));
}

#[test]
fn test_render_unterminated_heredoc_id() {
    assert_eq!(Message::UnterminatedHeredocId {}.render(), String::from("unterminated heredoc id"));
}

// Lexer warnings
#[test]
fn test_render_invalid_escape_use() {
    assert_eq!(Message::InvalidEscapeUse { escape: "x".into() }.render(), String::from("invalid character syntax; use ?x"));
}

#[test]
fn test_render_ambiguous_literal() {
    assert_eq!(Message::AmbiguousLiteral {}.render(), String::from("ambiguous first argument; put parentheses or a space even after the operator"));
}

#[test]
fn test_render_ambiguous_prefix() {
    assert_eq!(Message::AmbiguousPrefix { prefix: "pref".into() }.render(), String::from("`pref' interpreted as argument prefix"));
}

#[test]
fn test_render_triple_dot_at_eol() {
    assert_eq!(Message::TripleDotAtEol {}.render(), String::from("... at EOL, should be parenthesized"));
}

// Parser errors
#[test]
fn test_render_nth_ref_alias() {
    assert_eq!(Message::NthRefAlias {}.render(), String::from("cannot define an alias for a back-reference variable"));
}

#[test]
fn test_render_begin_in_method() {
    assert_eq!(Message::BeginInMethod {}.render(), String::from("BEGIN in method"));
}

#[test]
fn test_render_backref_assignment() {
    assert_eq!(Message::BackrefAssignment {}.render(), String::from("cannot assign to a back-reference variable"));
}

#[test]
fn test_render_invalid_assignment() {
    assert_eq!(Message::InvalidAssignment {}.render(), String::from("cannot assign to a keyword"));
}

#[test]
fn test_render_module_name_const() {
    assert_eq!(Message::ModuleNameConst {}.render(), String::from("class or module name must be a constant literal"));
}

#[test]
fn test_render_unexpected_token() {
    assert_eq!(Message::UnexpectedToken { token: "kIF".into() }.render(), String::from("unexpected token kIF"));
}

#[test]
fn test_render_argument_const() {
    assert_eq!(Message::ArgumentConst {}.render(), String::from("formal argument cannot be a constant"));
}

#[test]
fn test_render_argument_ivar() {
    assert_eq!(Message::ArgumentIvar {}.render(), String::from("formal argument cannot be an instance variable"));
}

#[test]
fn test_render_argument_gvar() {
    assert_eq!(Message::ArgumentGvar {}.render(), String::from("formal argument cannot be a global variable"));
}

#[test]
fn test_render_argument_cvar() {
    assert_eq!(Message::ArgumentCvar {}.render(), String::from("formal argument cannot be a class variable"));
}

#[test]
fn test_render_duplicate_argument() {
    assert_eq!(Message::DuplicateArgument {}.render(), String::from("duplicate argument name"));
}

#[test]
fn test_render_empty_symbol() {
    assert_eq!(Message::EmptySymbol {}.render(), String::from("empty symbol literal"));
}

#[test]
fn test_render_odd_hash() {
    assert_eq!(Message::OddHash {}.render(), String::from("odd number of entries for a hash"));
}

#[test]
fn test_render_singleton_literal() {
    assert_eq!(Message::SingletonLiteral {}.render(), String::from("cannot define a singleton method for a literal"));
}

#[test]
fn test_render_dynamic_const() {
    assert_eq!(Message::DynamicConst {}.render(), String::from("dynamic constant assignment"));
}

#[test]
fn test_render_const_reassignment() {
    assert_eq!(Message::ConstReassignment {}.render(), String::from("constant re-assignment"));
}

#[test]
fn test_render_module_in_def() {
    assert_eq!(Message::ModuleInDef {}.render(), String::from("module definition in method body"));
}

#[test]
fn test_render_class_in_def() {
    assert_eq!(Message::ClassInDef {}.render(), String::from("class definition in method body"));
}

#[test]
fn test_render_unexpected_percent_str() {
    assert_eq!(Message::UnexpectedPercentStr { str_type: "%qq".into() }.render(), String::from("%qq: unknown type of percent-literal"));
}

#[test]
fn test_render_block_and_blockarg() {
    assert_eq!(Message::BlockAndBlockarg {}.render(), String::from("both block argument and literal block are passed"));
}

#[test]
fn test_render_masgn_as_condition() {
    assert_eq!(Message::MasgnAsCondition {}.render(), String::from("multiple assignment in conditional context"));
}

#[test]
fn test_render_block_given_to_yield() {
    assert_eq!(Message::BlockGivenToYield {}.render(), String::from("block given to yield"));
}

#[test]
fn test_render_invalid_regexp() {
    assert_eq!(Message::InvalidRegexp { message: "invalid regexp foo".into() }.render(), String::from("invalid regexp foo"));
}

#[test]
fn test_render_invalid_return() {
    assert_eq!(Message::InvalidReturn {}.render(), String::from("Invalid return in class/module body"));
}

#[test]
fn test_render_csend_in_lhs_of_masgn() {
    assert_eq!(Message::CsendInLhsOfMasgn {}.render(), String::from("&. inside multiple assignment destination"));
}

#[test]
fn test_render_cant_assign_to_numparam() {
    assert_eq!(Message::CantAssignToNumparam { name: "_2".into() }.render(), String::from("cannot assign to numbered parameter _2"));
}

#[test]
fn test_render_reserved_for_numparam() {
    assert_eq!(Message::ReservedForNumparam { name: "_3".into() }.render(), String::from("_3 is reserved for numbered parameter"));
}

#[test]
fn test_render_ordinary_param_defined() {
    assert_eq!(Message::OrdinaryParamDefined {}.render(), String::from("ordinary parameter is defined"));
}

#[test]
fn test_render_numparam_used_in_outer_scope() {
    assert_eq!(Message::NumparamUsedInOuterScope {}.render(), String::from("numbered parameter is already used in an outer scope"));
}

#[test]
fn test_render_circular_argument_reference() {
    assert_eq!(Message::CircularArgumentReference { var_name: "bar".into() }.render(), String::from("circular argument reference bar"));
}

#[test]
fn test_render_pm_interp_in_var_name() {
    assert_eq!(Message::PmInterpInVarName {}.render(), String::from("symbol literal with interpolation is not allowed"));
}

#[test]
fn test_render_lvar_name() {
    assert_eq!(Message::LvarName { name: "self".into() }.render(), String::from("`self' is not allowed as a local variable name"));
}

#[test]
fn test_render_undefined_lvar() {
    assert_eq!(Message::UndefinedLvar { name: "foo".into() }.render(), String::from("no such local variable: `foo'"));
}

#[test]
fn test_render_duplicate_variable_name() {
    assert_eq!(Message::DuplicateVariableName { name: "baz".into() }.render(), String::from("duplicate variable name baz"));
}

#[test]
fn test_render_duplicate_pattern_key() {
    assert_eq!(Message::DuplicatePatternKey { name: "key1".into() }.render(), String::from("duplicate hash pattern key key1"));
}

#[test]
fn test_render_endless_setter() {
    assert_eq!(Message::EndlessSetter {}.render(), String::from("setter method cannot be defined in an endless method definition"));
}

// Parser warnings
#[test]
fn test_render_useless_else() {
    assert_eq!(Message::UselessElse {}.render(), String::from("else without rescue is useless"));
}

// Parser errors that are not Ruby errors
#[test]
fn test_render_invalid_encoding() {
    assert_eq!(Message::InvalidEncoding {}.render(), String::from("literal contains escape sequences incompatible with UTF-8"));
}

// Rewriter diagnostics
#[test]
fn test_render_invalid_action() {
    assert_eq!(Message::InvalidAction { action: "action1".into() }.render(), String::from("cannot action1"));
}

#[test]
fn test_render_clobbered() {
    assert_eq!(Message::Clobbered { action: "action1".into() }.render(), String::from("clobbered by: action1"));
}

// Rewriter diagnostics
#[test]
fn test_render_different_replacements() {
    assert_eq!(
        Message::DifferentReplacements {
            replacement: "rep".into(),
            other_replacement: "other_rep".into()
        }.render(),
        String::from("different replacements: rep vs other_rep")
    );
}

#[test]
fn test_render_swallowed_insertions() {
    assert_eq!(Message::SwallowedInsertions {}.render(), String::from("this replacement:"));
}

#[test]
fn test_render_swallowed_insertions_conflict() {
    assert_eq!(Message::SwallowedInsertionsConflict {}.render(), String::from("swallows some inner rewriting actions:"));
}

#[test]
fn test_render_crossing_deletions() {
    assert_eq!(Message::CrossingDeletions {}.render(), String::from("the deletion of:"));
}

#[test]
fn test_render_crossing_deletions_conflict() {
    assert_eq!(Message::CrossingDeletionsConflict {}.render(), String::from("is crossing:"));
}

#[test]
fn test_render_crossing_insertions() {
    assert_eq!(Message::CrossingInsertions {}.render(), String::from("the rewriting action on:"));
}

#[test]
fn test_render_crossing_insertions_conflict() {
    assert_eq!(Message::CrossingInsertionsConflict {}.render(), String::from("is crossing that on:"));
}
