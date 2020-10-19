#[derive(Debug, Clone)]
pub enum ErrorMessage {
    UnicodePointTooLarge {},
    InvalidEscape {},
    IncompleteEscape {},
    InvalidHexEscape {},
    InvalidUnicodeEscape {},
    UnterminatedUnicode {},
    EscapeEof {},
    StringEof {},
    RegexpOptions { options: String },
    CvarName { name: String },
    IvarName { name: String },
    TrailingInNumber { character: String },
    EmptyNumeric {},
    InvalidOctal {},
    NoDotDigitLiteral {},
    BareBackslash {},
    Unexpected { character: String },
    EmbeddedDocument {},
    HeredocIdHasNewline {},
    HeredocIdEndsWithNl {},
    UnterminatedHeredocId {},
    InvalidEscapeUse { escape: String },
    AmbiguousLiteral {},
    AmbiguousPrefix { prefix: String },
    TripleDotAtEol {},
    NthRefAlias {},
    BeginInMethod {},
    BackrefAssignment {},
    InvalidAssignment {},
    ModuleNameConst {},
    UnexpectedToken { token: String },
    ArgumentConst {},
    ArgumentIvar {},
    ArgumentGvar {},
    ArgumentCvar {},
    DuplicateArgument {},
    EmptySymbol {},
    OddHash {},
    SingletonLiteral {},
    DynamicConst {},
    ConstReassignment {},
    ModuleInDef {},
    ClassInDef {},
    UnexpectedPercentStr { str_type: String },
    BlockAndBlockarg {},
    MasgnAsCondition {},
    BlockGivenToYield {},
    InvalidRegexp { message: String },
    InvalidReturn {},
    CsendInLhsOfMasgn {},
    CantAssignToNumparam { name: String },
    ReservedForNumparam { name: String },
    OrdinaryParamDefined {},
    NumparamUsedInOuterScope {},
    CircularArgumentReference { var_name: String },
    PmInterpInVarName {},
    LvarName { name: String },
    UndefinedLvar { name: String },
    DuplicateVariableName { name: String },
    DuplicatePatternKey { name: String },
    EndlessSetter {},
    UselessElse {},
}

impl ErrorMessage {
    pub fn render(&self) -> String {
        match self {
            Self::UnicodePointTooLarge {} => format!("invalid Unicode codepoint (too large)"),
            Self::InvalidEscape {} => format!("invalid escape character syntax"),
            Self::IncompleteEscape {} => format!("incomplete character syntax"),
            Self::InvalidHexEscape {} => format!("invalid hex escape"),
            Self::InvalidUnicodeEscape {} => format!("invalid Unicode escape"),
            Self::UnterminatedUnicode {} => format!("unterminated Unicode escape"),
            Self::EscapeEof {} => format!("escape sequence meets end of file"),
            Self::StringEof {} => format!("unterminated string meets end of file"),
            Self::RegexpOptions { options } => {
                format!("unknown regexp options: {options}", options = options)
            }
            Self::CvarName { name } => format!(
                "`{name}' is not allowed as a class variable name",
                name = name
            ),
            Self::IvarName { name } => format!(
                "`{name}' is not allowed as an instance variable name",
                name = name
            ),
            Self::TrailingInNumber { character } => {
                format!("trailing `{character}' in number", character = character)
            }
            Self::EmptyNumeric {} => format!("numeric literal without digits"),
            Self::InvalidOctal {} => format!("invalid octal digit"),
            Self::NoDotDigitLiteral {} => {
                format!("no .<digit> floating literal anymore; put 0 before dot")
            }
            Self::BareBackslash {} => format!("bare backslash only allowed before newline"),
            Self::Unexpected { character } => {
                format!("unexpected `{character}'", character = character)
            }
            Self::EmbeddedDocument {} => format!(
                "embedded document meets end of file (and they embark on a romantic journey)"
            ),
            Self::HeredocIdHasNewline {} => {
                format!("here document identifier across newlines, never match")
            }
            Self::HeredocIdEndsWithNl {} => format!("here document identifier ends with a newline"),
            Self::UnterminatedHeredocId {} => format!("unterminated heredoc id"),
            Self::InvalidEscapeUse { escape } => {
                format!("invalid character syntax; use ?{escape}", escape = escape)
            }
            Self::AmbiguousLiteral {} => format!(
                "ambiguous first argument; put parentheses or a space even after the operator"
            ),
            Self::AmbiguousPrefix { prefix } => {
                format!("`{prefix}' interpreted as argument prefix", prefix = prefix)
            }
            Self::TripleDotAtEol {} => format!("... at EOL, should be parenthesized"),
            Self::NthRefAlias {} => format!("cannot define an alias for a back-reference variable"),
            Self::BeginInMethod {} => format!("BEGIN in method"),
            Self::BackrefAssignment {} => format!("cannot assign to a back-reference variable"),
            Self::InvalidAssignment {} => format!("cannot assign to a keyword"),
            Self::ModuleNameConst {} => format!("class or module name must be a constant literal"),
            Self::UnexpectedToken { token } => format!("unexpected token {token}", token = token),
            Self::ArgumentConst {} => format!("formal argument cannot be a constant"),
            Self::ArgumentIvar {} => format!("formal argument cannot be an instance variable"),
            Self::ArgumentGvar {} => format!("formal argument cannot be a global variable"),
            Self::ArgumentCvar {} => format!("formal argument cannot be a class variable"),
            Self::DuplicateArgument {} => format!("duplicate argument name"),
            Self::EmptySymbol {} => format!("empty symbol literal"),
            Self::OddHash {} => format!("odd number of entries for a hash"),
            Self::SingletonLiteral {} => format!("cannot define a singleton method for a literal"),
            Self::DynamicConst {} => format!("dynamic constant assignment"),
            Self::ConstReassignment {} => format!("constant re-assignment"),
            Self::ModuleInDef {} => format!("module definition in method body"),
            Self::ClassInDef {} => format!("class definition in method body"),
            Self::UnexpectedPercentStr { str_type } => {
                format!("{type}: unknown type of percent-literal", type = str_type)
            }
            Self::BlockAndBlockarg {} => {
                format!("both block argument and literal block are passed")
            }
            Self::MasgnAsCondition {} => format!("multiple assignment in conditional context"),
            Self::BlockGivenToYield {} => format!("block given to yield"),
            Self::InvalidRegexp { message } => message.clone(),
            Self::InvalidReturn {} => format!("Invalid return in class/module body"),
            Self::CsendInLhsOfMasgn {} => format!("&. inside multiple assignment destination"),
            Self::CantAssignToNumparam { name } => {
                format!("cannot assign to numbered parameter {name}", name = name)
            }
            Self::ReservedForNumparam { name } => {
                format!("{name} is reserved for numbered parameter", name = name)
            }
            Self::OrdinaryParamDefined {} => format!("ordinary parameter is defined"),
            Self::NumparamUsedInOuterScope {} => {
                format!("numbered parameter is already used in an outer scope")
            }
            Self::CircularArgumentReference { var_name } => format!(
                "circular argument reference {var_name}",
                var_name = var_name
            ),
            Self::PmInterpInVarName {} => {
                format!("symbol literal with interpolation is not allowed")
            }
            Self::LvarName { name } => format!(
                "`{name}' is not allowed as a local variable name",
                name = name
            ),
            Self::UndefinedLvar { name } => {
                format!("no such local variable: `{name}'", name = name)
            }
            Self::DuplicateVariableName { name } => {
                format!("duplicate variable name {name}", name = name)
            }
            Self::DuplicatePatternKey { name } => {
                format!("duplicate hash pattern key {name}", name = name)
            }
            Self::EndlessSetter {} => {
                format!("setter method cannot be defined in an endless method definition")
            }
            Self::UselessElse {} => format!("else without rescue is useless"),
        }
    }
}
