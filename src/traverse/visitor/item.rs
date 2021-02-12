#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum Item {
    Root,

    Recv,
    Lhs,
    Rhs,
    Value,
    MethodCall,
    Body,
    Args,
    Expr,
    ElseBody,
    Scope,
    Name,
    Superclass,
    Const,
    Definee,
    Iterator,
    Iteratee,
    Pattern,
    Left,
    Right,
    IfTrue,
    IfFalse,
    Cond,
    DefaultValue,
    Ensure,
    Guard,
    As,
    Re,
    Key,
    ExcList,
    ExcVar,
    Match,
    Else,
    Var,
    Options,
    To,
    From,

    MlhsItems,
    Arglist,
    Elements,
    Stmts,
    WhenBodies,
    InBodies,
    Parts,
    Indexes,
    Pairs,
    RescueBodies,

    Idx(usize),
}

#[derive(Debug)]
pub(crate) struct ItemFromStringError {
    unknown_pattern: String,
}

impl std::fmt::Display for ItemFromStringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown pattern {}", self.unknown_pattern)
    }
}

impl Item {
    pub(crate) fn from_string(s: &str) -> Result<Self, ItemFromStringError> {
        let result = match &s[..] {
            "root" => Self::Root,

            "recv" => Self::Recv,
            "lhs" => Self::Lhs,
            "rhs" => Self::Rhs,
            "value" => Self::Value,
            "method_call" => Self::MethodCall,
            "body" => Self::Body,
            "args" => Self::Args,
            "expr" => Self::Expr,
            "else_body" => Self::ElseBody,
            "scope" => Self::Scope,
            "name" => Self::Name,
            "superclass" => Self::Superclass,
            "const" => Self::Const,
            "definee" => Self::Definee,
            "iterator" => Self::Iterator,
            "iteratee" => Self::Iteratee,
            "pattern" => Self::Pattern,
            "left" => Self::Left,
            "right" => Self::Right,
            "if_true" => Self::IfTrue,
            "if_false" => Self::IfFalse,
            "cond" => Self::Cond,
            "default_value" => Self::DefaultValue,
            "ensure" => Self::Ensure,
            "guard" => Self::Guard,
            "as" => Self::As,
            "re" => Self::Re,
            "key" => Self::Key,
            "exc_list" => Self::ExcList,
            "exc_var" => Self::ExcVar,
            "match" => Self::Match,
            "else" => Self::Else,
            "var" => Self::Var,
            "options" => Self::Options,
            "to" => Self::To,
            "from" => Self::From,

            "mlhs_items" => Self::MlhsItems,
            "arglist" => Self::Arglist,
            "elements" => Self::Elements,
            "stmts" => Self::Stmts,
            "when_bodies" => Self::WhenBodies,
            "in_bodies" => Self::InBodies,
            "parts" => Self::Parts,
            "indexes" => Self::Indexes,
            "pairs" => Self::Pairs,
            "rescue_bodies" => Self::RescueBodies,

            other if other.parse::<usize>().is_ok() => {
                let idx = other.parse::<usize>().unwrap();
                Self::Idx(idx)
            }

            unknown_pattern => {
                return Err(ItemFromStringError {
                    unknown_pattern: unknown_pattern.to_owned(),
                })
            }
        };
        Ok(result)
    }
}
