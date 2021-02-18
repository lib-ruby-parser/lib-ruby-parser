/// Enum of all types of parent->child transitions during traversing
#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum Item {
    /// Root transition
    Root,

    /// Transition into `.recv` from:
    /// 1. `Send`
    /// 2. `CSend`
    /// 3. `Index`
    /// 4. `IndexAsgn`
    /// 5. `OpAsgn`
    /// 6. `OrAsgn`
    /// 7. `AndAsgn`
    Recv,

    /// Transition into `.lhs` from:
    /// 1. `And`
    /// 2. `Masgn`
    /// 3. `MatchAlt`
    /// 4. `Or`
    Lhs,

    /// Transition into `.rhs` from:
    /// 1. `And`
    /// 2. `Masgn`
    /// 3. `MatchAlt`
    /// 4. `Or`
    Rhs,

    /// Transition into `.value` from:
    /// 1. `AndAsgn`
    /// 2. `BlockPass`
    /// 3. `Casgn`
    /// 4. `Cvasgn`
    /// 5. `Defined`
    /// 6. `Gvasgn`
    /// 7. `MatchPattern`
    /// 8. `MatchPattern`
    /// 9. `IndexAsgn`
    /// 10. `Ivasgn`
    /// 11. `Kwsplat`
    /// 12. `Lvasgn`
    /// 13. `MatchAs`
    /// 14. `MatchWithLvasgn`
    /// 15. `OpAsgn`
    /// 16. `OrAsgn`
    /// 17. `Pair`
    /// 18. `Splat`
    Value,

    /// Transitions into `.call` from:
    /// 1. `Block`
    /// 2. `Numblock`
    MethodCall,

    /// Transitions into `.body` from:
    /// 1. `Block`
    /// 2. `Class`
    /// 3. `Def`
    /// 4. `Defs`
    /// 5. `Ensure`
    /// 6. `For`
    /// 7. `InPattern`
    /// 8. `Module`
    /// 9. `Numblock`
    /// 10. `Postexe`
    /// 11. `Preexe`
    /// 12. `Rescue`
    /// 13. `RescueBody`
    /// 14. `Sclass`
    /// 15. `Until`
    /// 16. `UntilPost`
    /// 17. `When`
    /// 18. `While`
    /// 19. `WhilePost`
    Body,

    /// Transitions into `.args` from:
    /// 1. `Block`
    /// 2. `Break`
    /// 3. `Csend`
    /// 4. `Def`
    /// 5. `Defs`
    /// 6. `Next`
    /// 7. `Return`
    /// 8. `Send`
    /// 9. `Super`
    /// 10. `Undef`
    /// 11. `When`
    /// 12. `Yield`
    Args,

    /// Transitions into `.expr` from:
    /// 1. `Case`
    /// 2. `CaseMatch`
    /// 3. `Sclass`
    Expr,

    /// Transitions into `.else_body` from:
    /// 1. `Case`
    /// 2. `CaseMatch`
    /// 3. `Rescue`
    ElseBody,

    /// Transitions into `.scope` from:
    /// 1. `Casgn`
    /// 2. `Const`
    Scope,

    /// Transitions into `.name` from:
    /// 1. `Class`
    /// 2. `MatchRest`
    /// 3. `Module`
    Name,

    /// Transitions into `.superclass` from:
    // 1. `Class`
    Superclass,

    /// Transitions into `.const` from:
    /// 1. `ConstPattern`
    Const,

    /// Transitions into `.definee` from:
    /// 1. `Defs`
    Definee,

    /// Transitions into `.iterator` from:
    /// 1. `For`
    Iterator,

    /// Transitions into `.iteratee` from:
    /// 1. `For`
    Iteratee,

    /// Transitions into `.pattern` from:
    /// 1. `ConstPattern`
    /// 2. `MatchPattern`
    /// 3. `MatchPatternP`
    /// 4. `InPattern`
    Pattern,

    /// Transitions into `.left` from:
    /// 1. `EFlipFlop`
    /// 2. `Erange`
    /// 3. `IFlipFlop`
    /// 4. `Irange`
    Left,

    /// Transitions into `.right` from:
    /// 1. `EFlipFlop`
    /// 2. `Erange`
    /// 3. `IFlipFlop`
    /// 4. `Irange`
    Right,

    /// Transitions into `.if_true` from:
    /// 1. `If`
    /// 2. `IfMod`
    /// 3. `IfTernary`
    IfTrue,

    /// Transitions into `.if_false` from:
    /// 1. `If`
    /// 2. `IfMod`
    /// 3. `IfTernary`
    IfFalse,

    /// Transitions into `.cond` from:
    /// 1. `If`
    /// 2. `IfGuard`
    /// 3. `IfMod`
    /// 4. `IfTernary`
    /// 5. `UnlessGuard`
    /// 6. `Until`
    /// 7. `UntilPost`
    /// 8. `While`
    /// 9. `WhilePost`
    Cond,

    /// Transitions into `.default` from:
    /// 1. `Kwoptarg`
    /// 2. `Optarg`
    DefaultValue,

    /// Transitions into `.ensure` from:
    /// 1. `Ensure`
    Ensure,

    /// Transitions into `.guard` from:
    /// 1. `InPattern`
    Guard,

    /// Transitions into `.as` from:
    /// 1. `MatchAs`
    As,

    /// Transitions into `.re` from:
    /// 1. `MatchCurrentLine`
    /// 2. `MatchWithLvasgn`
    Re,

    /// Transitions into `.key` from:
    /// 1. `Pair`
    Key,

    /// Transitions into `.exc_list` from:
    /// 1. `RescueBody`
    ExcList,

    /// Transitions into `.exc_var` from:
    /// 1. `RescueBody`
    ExcVar,

    /// Transitions into `.var` from:
    /// 1. `Pin`
    Var,

    /// Transitions into `.options` from:
    /// 1. `Regexp`
    Options,

    /// Transitions into `.to` from:
    /// 1. `Alias`
    To,

    /// Transitions into `.from` from:
    /// 1. `Alias`
    From,

    // -- arrays --
    /// Transitions into `.items` from:
    /// 1. `Mlhs`
    MlhsItems,

    /// Transitions into `.args` from:
    /// 1. `Args`
    /// 2. `Procarg0`
    Arglist,

    /// Transitions into `.elements` from:
    /// 1. `Array`
    /// 2. `ArrayPattern`
    /// 3. `ArrayPatternWithTail`
    /// 4. `FindPattern`
    /// 5. `HashPattern`
    Elements,

    /// Transitions into `.statements` from:
    /// 1. `Begin`
    /// 2. `KwBegin`
    Stmts,

    /// Transitions into `.whn_bodies` from:
    /// 1. `Case`
    WhenBodies,

    /// Transitions into `.in_bodies` from:
    /// 1. `CaseMatch`
    InBodies,

    /// Transitions into `.parts` from:
    /// 1. `Dstr`
    /// 2. `Dsym`
    /// 3. `Heredoc`
    /// 4. `Regexp`
    /// 5. `XHeredoc`
    /// 6. `Xstr`
    Parts,

    /// Transitions into `.indexes` from:
    /// 1. `Index`
    /// 2. `IndexAsgn`
    Indexes,

    /// Transitions into `.pairs` from:
    /// 1. `Hash`
    /// 2. `Kwargs`
    Pairs,

    /// Transitions into `.rescue_bodies` from:
    /// 1. `Rescue`
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
