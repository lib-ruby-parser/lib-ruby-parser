use crate::traverse::finder::PatternError;

/// Enum of all types of parent->child transitions during traversing
#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum Item {
    /// Root transition
    Root,

    /// Transition into `.recv` from:
    ///
    /// + `Send`
    /// + `CSend`
    /// + `Index`
    /// + `IndexAsgn`
    /// + `OpAsgn`
    /// + `OrAsgn`
    /// + `AndAsgn`
    Recv,

    /// Transition into `.lhs` from:
    ///
    /// + `And`
    /// + `Masgn`
    /// + `MatchAlt`
    /// + `Or`
    Lhs,

    /// Transition into `.rhs` from:
    ///
    /// + `And`
    /// + `Masgn`
    /// + `MatchAlt`
    /// + `Or`
    Rhs,

    /// Transition into `.value` from:
    ///
    /// + `AndAsgn`
    /// + `BlockPass`
    /// + `Casgn`
    /// + `Cvasgn`
    /// + `Defined`
    /// + `Gvasgn`
    /// + `MatchPattern`
    /// + `MatchPattern`
    /// + `IndexAsgn`
    /// + `Ivasgn`
    /// + `Kwsplat`
    /// + `Lvasgn`
    /// + `MatchAs`
    /// + `MatchWithLvasgn`
    /// + `OpAsgn`
    /// + `OrAsgn`
    /// + `Pair`
    /// + `Splat`
    Value,

    /// Transitions into `.call` from:
    ///
    /// + `Block`
    /// + `Numblock`
    MethodCall,

    /// Transitions into `.body` from:
    ///
    /// + `Block`
    /// + `Class`
    /// + `Def`
    /// + `Defs`
    /// + `Ensure`
    /// + `For`
    /// + `InPattern`
    /// + `Module`
    /// + `Numblock`
    /// + `Postexe`
    /// + `Preexe`
    /// + `Rescue`
    /// + `RescueBody`
    /// + `Sclass`
    /// + `Until`
    /// + `UntilPost`
    /// + `When`
    /// + `While`
    /// + `WhilePost`
    Body,

    /// Transitions into `.args` from:
    ///
    /// + `Block`
    /// + `Break`
    /// + `Csend`
    /// + `Def`
    /// + `Defs`
    /// + `Next`
    /// + `Return`
    /// + `Send`
    /// + `Super`
    /// + `Undef`
    /// + `When`
    /// + `Yield`
    Args,

    /// Transitions into `.expr` from:
    ///
    /// + `Case`
    /// + `CaseMatch`
    /// + `Sclass`
    Expr,

    /// Transitions into `.else_body` from:
    ///
    /// + `Case`
    /// + `CaseMatch`
    /// + `Rescue`
    ElseBody,

    /// Transitions into `.scope` from:
    ///
    /// + `Casgn`
    /// + `Const`
    Scope,

    /// Transitions into `.name` from:
    ///
    /// + `Class`
    /// + `MatchRest`
    /// + `Module`
    Name,

    /// Transitions into `.superclass` from:
    ///
    /// + `Class`
    Superclass,

    /// Transitions into `.const` from:
    ///
    /// + `ConstPattern`
    Const,

    /// Transitions into `.definee` from:
    ///
    /// + `Defs`
    Definee,

    /// Transitions into `.iterator` from:
    ///
    /// + `For`
    Iterator,

    /// Transitions into `.iteratee` from:
    ///
    /// + `For`
    Iteratee,

    /// Transitions into `.pattern` from:
    ///
    /// + `ConstPattern`
    /// + `MatchPattern`
    /// + `MatchPatternP`
    /// + `InPattern`
    Pattern,

    /// Transitions into `.left` from:
    ///
    /// + `EFlipFlop`
    /// + `Erange`
    /// + `IFlipFlop`
    /// + `Irange`
    Left,

    /// Transitions into `.right` from:
    ///
    /// + `EFlipFlop`
    /// + `Erange`
    /// + `IFlipFlop`
    /// + `Irange`
    Right,

    /// Transitions into `.if_true` from:
    ///
    /// + `If`
    /// + `IfMod`
    /// + `IfTernary`
    IfTrue,

    /// Transitions into `.if_false` from:
    ///
    /// + `If`
    /// + `IfMod`
    /// + `IfTernary`
    IfFalse,

    /// Transitions into `.cond` from:
    ///
    /// + `If`
    /// + `IfGuard`
    /// + `IfMod`
    /// + `IfTernary`
    /// + `UnlessGuard`
    /// + `Until`
    /// + `UntilPost`
    /// + `While`
    /// + `WhilePost`
    Cond,

    /// Transitions into `.default` from:
    ///
    /// + `Kwoptarg`
    /// + `Optarg`
    DefaultValue,

    /// Transitions into `.ensure` from:
    ///
    /// + `Ensure`
    Ensure,

    /// Transitions into `.guard` from:
    ///
    /// + `InPattern`
    Guard,

    /// Transitions into `.as` from:
    ///
    /// + `MatchAs`
    As,

    /// Transitions into `.re` from:
    ///
    /// + `MatchCurrentLine`
    /// + `MatchWithLvasgn`
    Re,

    /// Transitions into `.key` from:
    ///
    /// + `Pair`
    Key,

    /// Transitions into `.exc_list` from:
    ///
    /// + `RescueBody`
    ExcList,

    /// Transitions into `.exc_var` from:
    ///
    /// + `RescueBody`
    ExcVar,

    /// Transitions into `.var` from:
    ///
    /// + `Pin`
    Var,

    /// Transitions into `.options` from:
    ///
    /// + `Regexp`
    Options,

    /// Transitions into `.to` from:
    ///
    /// + `Alias`
    To,

    /// Transitions into `.from` from:
    ///
    /// + `Alias`
    From,

    // -- arrays --
    /// Transitions into `.items` from:
    ///
    /// + `Mlhs`
    MlhsItems,

    /// Transitions into `.args` from:
    ///
    /// + `Args`
    /// + `Procarg0`
    Arglist,

    /// Transitions into `.elements` from:
    ///
    /// + `Array`
    /// + `ArrayPattern`
    /// + `ArrayPatternWithTail`
    /// + `FindPattern`
    /// + `HashPattern`
    Elements,

    /// Transitions into `.statements` from:
    ///
    /// + `Begin`
    /// + `KwBegin`
    Stmts,

    /// Transitions into `.when_bodies` from:
    ///
    /// + `Case`
    WhenBodies,

    /// Transitions into `.in_bodies` from:
    ///
    /// + `CaseMatch`
    InBodies,

    /// Transitions into `.parts` from:
    ///
    /// + `Dstr`
    /// + `Dsym`
    /// + `Heredoc`
    /// + `Regexp`
    /// + `XHeredoc`
    /// + `Xstr`
    Parts,

    /// Transitions into `.indexes` from:
    ///
    /// + `Index`
    /// + `IndexAsgn`
    Indexes,

    /// Transitions into `.pairs` from:
    ///
    /// + `Hash`
    /// + `Kwargs`
    Pairs,

    /// Transitions into `.rescue_bodies` from:
    ///
    /// + `Rescue`
    RescueBodies,

    /// Transitions into any element of `Vec<Node>
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
        let result = match s {
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
                    unknown_pattern: unknown_pattern.to_string(),
                })
            }
        };
        Ok(result)
    }
}

impl Item {
    /// Parses given string slice and constructs an `Item` (if possible)
    pub fn new(s: &str) -> Result<Self, PatternError> {
        Item::from_string(s).map_err(|e| PatternError {
            pattern: e.to_string(),
        })
    }
}
