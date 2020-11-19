use crate::nodes::*;
use crate::traverse::Visitor;
use crate::Node;

#[derive(Clone, Debug)]
pub enum PatternItem {
    Root,
    Recv,
    Lhs,
    Rhs,
    Value,
    Call,
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
    Default,
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
    Item(usize),
    Arg(usize),
    Element(usize),
    Stmt(usize),
    WhenBody(usize),
    InBody(usize),
    Part(usize),
    Index(usize),
    Pair(usize),
    RescueBody(usize),

    None,
}

impl PatternItem {
    pub fn new(s: &str) -> Result<Self, PatternError> {
        let try_value = |prefix: &str| {
            s.replace(prefix, "")
                .replace("[", "")
                .replace("]", "")
                .parse::<usize>()
                .map_err(|_| PatternError {
                    pattern: s.to_owned(),
                })
        };

        let this = match s {
            "recv" => Self::Recv,
            "lhs" => Self::Lhs,
            "rhs" => Self::Rhs,
            "value" => Self::Value,
            "call" => Self::Call,
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
            "default" => Self::Default,
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

            other if other.starts_with("item[") => Self::Item(try_value("item")?),
            other if other.starts_with("arg[") => Self::Arg(try_value("arg")?),
            other if other.starts_with("element[") => Self::Element(try_value("element")?),
            other if other.starts_with("stmt[") => Self::Stmt(try_value("stmt")?),
            other if other.starts_with("when_body[") => Self::WhenBody(try_value("when_body")?),
            other if other.starts_with("in_body[") => Self::InBody(try_value("in_body")?),
            other if other.starts_with("part[") => Self::Part(try_value("part")?),
            other if other.starts_with("index[") => Self::Index(try_value("index")?),
            other if other.starts_with("pair[") => Self::Pair(try_value("pair")?),
            other if other.starts_with("rescue_body[") => {
                Self::RescueBody(try_value("rescue_body")?)
            }

            unsupported => {
                return Err(PatternError {
                    pattern: unsupported.to_owned(),
                })
            }
        };

        Ok(this)
    }
}

#[derive(Debug)]
pub struct PatternError {
    pub pattern: String,
}

impl std::fmt::Display for PatternError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "PatternError: unsupported pattern {}",
            self.pattern
        ))
    }
}

#[derive(Debug)]
struct Pattern {
    parts: Vec<PatternItem>,
}

impl Pattern {
    fn new(str_parts: &[String]) -> Result<Self, PatternError> {
        let mut parts: Vec<PatternItem> = vec![];
        for str_part in str_parts {
            let part = PatternItem::new(str_part)?;
            parts.push(part)
        }

        parts.reverse();
        parts.push(PatternItem::Root);

        Ok(Self { parts })
    }

    fn current(&self) -> PatternItem {
        self.parts.last().cloned().unwrap_or(PatternItem::None)
    }

    fn pop(&mut self) {
        self.parts.pop();
    }

    fn is_empty(&self) -> bool {
        self.parts.is_empty()
    }
}

pub struct Find {
    pattern: Pattern,
}

impl<'a> Find {
    pub fn run(pattern: &[String], root: &Node) -> Result<Option<Node>, PatternError> {
        let pattern = Pattern::new(pattern)?;
        let mut this = Self { pattern };
        Ok(this.find(&root))
    }

    fn current_pattern(&self) -> PatternItem {
        self.pattern.current()
    }

    fn find(&mut self, node: &Node) -> Option<Node> {
        self.pattern.pop();

        if self.pattern.is_empty() {
            return Some(node.clone());
        }

        self.visit(node)
    }

    fn maybe_find(&mut self, node: &Option<Node>) -> Option<Node> {
        if let Some(node) = node {
            self.find(node)
        } else {
            None
        }
    }
}

impl<'a> Visitor<Option<Node>> for Find {
    fn visit_all(&mut self, _: &[Node]) -> Option<Node> {
        unreachable!("arrays should be handled manually")
    }

    fn on_alias(&mut self, node: &Alias) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::To => self.find(&node.to),
            PatternItem::From => self.find(&node.from),
            _ => None,
        }
    }

    fn on_and(&mut self, node: &And) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Lhs => self.find(&node.lhs),
            PatternItem::Rhs => self.find(&node.rhs),
            _ => None,
        }
    }

    fn on_and_asgn(&mut self, node: &AndAsgn) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Recv => self.find(&node.recv),
            PatternItem::Value => self.find(&node.value),
            _ => None,
        }
    }

    fn on_arg(&mut self, _: &Arg) -> Option<Node> {
        None
    }

    fn on_args(&mut self, node: &Args) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Arg(n) => self.find(&node.args[n]),
            _ => None,
        }
    }

    fn on_array(&mut self, node: &Array) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Element(n) => self.find(&node.elements[n]),
            _ => None,
        }
    }

    fn on_array_pattern(&mut self, node: &ArrayPattern) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Element(n) => self.find(&node.elements[n]),
            _ => None,
        }
    }

    fn on_array_pattern_with_tail(&mut self, node: &ArrayPatternWithTail) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Element(n) => self.find(&node.elements[n]),
            _ => None,
        }
    }

    fn on_back_ref(&mut self, _: &BackRef) -> Option<Node> {
        None
    }

    fn on_begin(&mut self, node: &Begin) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Stmt(n) => self.find(&node.statements[n]),
            _ => None,
        }
    }

    fn on_block(&mut self, node: &Block) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Call => self.find(&node.call),
            PatternItem::Args => self.maybe_find(&node.args),
            PatternItem::Body => self.maybe_find(&node.body),
            _ => None,
        }
    }

    fn on_blockarg(&mut self, _: &Blockarg) -> Option<Node> {
        None
    }

    fn on_block_pass(&mut self, node: &BlockPass) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Value => self.find(&node.value),
            _ => None,
        }
    }

    fn on_break(&mut self, node: &Break) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Arg(n) => self.find(&node.args[n]),
            _ => None,
        }
    }

    fn on_case(&mut self, node: &Case) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Expr => self.maybe_find(&node.expr),
            PatternItem::WhenBody(n) => self.find(&node.when_bodies[n]),
            PatternItem::ElseBody => self.maybe_find(&node.else_body),
            _ => None,
        }
    }

    fn on_case_match(&mut self, node: &CaseMatch) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Expr => self.find(&node.expr),
            PatternItem::InBody(n) => self.find(&node.in_bodies[n]),
            PatternItem::ElseBody => self.maybe_find(&node.else_body),
            _ => None,
        }
    }

    fn on_casgn(&mut self, node: &Casgn) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Scope => self.maybe_find(&node.scope),
            PatternItem::Value => self.maybe_find(&node.value),
            _ => None,
        }
    }

    fn on_cbase(&mut self, _: &Cbase) -> Option<Node> {
        None
    }

    fn on_class(&mut self, node: &Class) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Name => self.find(&node.name),
            PatternItem::Superclass => self.maybe_find(&node.superclass),
            PatternItem::Body => self.maybe_find(&node.body),
            _ => None,
        }
    }

    fn on_complex(&mut self, _: &Complex) -> Option<Node> {
        None
    }

    fn on_const(&mut self, node: &Const) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Scope => self.maybe_find(&node.scope),
            _ => None,
        }
    }

    fn on_const_pattern(&mut self, node: &ConstPattern) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Const => self.find(&node.const_),
            PatternItem::Pattern => self.find(&node.pattern),
            _ => None,
        }
    }

    fn on_csend(&mut self, node: &CSend) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Recv => self.find(&node.recv),
            PatternItem::Arg(n) => self.find(&node.args[n]),
            _ => None,
        }
    }

    fn on_cvar(&mut self, _: &Cvar) -> Option<Node> {
        None
    }

    fn on_cvasgn(&mut self, node: &Cvasgn) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Value => self.maybe_find(&node.value),
            _ => None,
        }
    }

    fn on_def(&mut self, node: &Def) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Args => self.maybe_find(&node.args),
            PatternItem::Body => self.maybe_find(&node.body),
            _ => None,
        }
    }

    fn on_defined(&mut self, node: &Defined) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Value => self.find(&node.value),
            _ => None,
        }
    }

    fn on_defs(&mut self, node: &Defs) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Definee => self.find(&node.definee),
            PatternItem::Args => self.maybe_find(&node.args),
            PatternItem::Body => self.maybe_find(&node.body),
            _ => None,
        }
    }

    fn on_dstr(&mut self, node: &Dstr) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Part(n) => self.find(&node.parts[n]),
            _ => None,
        }
    }

    fn on_dsym(&mut self, node: &Dsym) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Part(n) => self.find(&node.parts[n]),
            _ => None,
        }
    }

    fn on_eflipflop(&mut self, node: &EFlipFlop) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Left => self.maybe_find(&node.left),
            PatternItem::Right => self.maybe_find(&node.right),
            _ => None,
        }
    }

    fn on_empty_else(&mut self, _: &EmptyElse) -> Option<Node> {
        None
    }

    fn on_encoding(&mut self, _: &Encoding) -> Option<Node> {
        None
    }

    fn on_ensure(&mut self, node: &Ensure) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Body => self.maybe_find(&node.body),
            PatternItem::Ensure => self.maybe_find(&node.ensure),
            _ => None,
        }
    }

    fn on_erange(&mut self, node: &Erange) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Left => self.maybe_find(&node.left),
            PatternItem::Right => self.maybe_find(&node.right),
            _ => None,
        }
    }

    fn on_false(&mut self, _: &False) -> Option<Node> {
        None
    }

    fn on_file(&mut self, _: &File) -> Option<Node> {
        None
    }

    fn on_find_pattern(&mut self, node: &FindPattern) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Element(n) => self.find(&node.elements[n]),
            _ => None,
        }
    }

    fn on_float(&mut self, _: &Float) -> Option<Node> {
        None
    }

    fn on_for(&mut self, node: &For) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Iterator => self.find(&node.iterator),
            PatternItem::Iteratee => self.find(&node.iteratee),
            PatternItem::Body => self.maybe_find(&node.body),
            _ => None,
        }
    }

    fn on_forward_arg(&mut self, _: &ForwardArg) -> Option<Node> {
        None
    }

    fn on_forwarded_args(&mut self, _: &ForwardedArgs) -> Option<Node> {
        None
    }

    fn on_gvar(&mut self, _: &Gvar) -> Option<Node> {
        None
    }

    fn on_gvasgn(&mut self, node: &Gvasgn) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Value => self.maybe_find(&node.value),
            _ => None,
        }
    }

    fn on_hash(&mut self, node: &Hash) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Pair(n) => self.find(&node.pairs[n]),
            _ => None,
        }
    }

    fn on_hash_pattern(&mut self, node: &HashPattern) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Element(n) => self.find(&node.elements[n]),
            _ => None,
        }
    }

    fn on_heredoc(&mut self, node: &Heredoc) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Part(n) => self.find(&node.parts[n]),
            _ => None,
        }
    }

    fn on_if(&mut self, node: &If) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Cond => self.find(&node.cond),
            PatternItem::IfTrue => self.maybe_find(&node.if_true),
            PatternItem::IfFalse => self.maybe_find(&node.if_false),
            _ => None,
        }
    }

    fn on_if_guard(&mut self, node: &IfGuard) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Cond => self.find(&node.cond),
            _ => None,
        }
    }

    fn on_iflipflop(&mut self, node: &IFlipFlop) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Left => self.maybe_find(&node.left),
            PatternItem::Right => self.maybe_find(&node.right),
            _ => None,
        }
    }

    fn on_if_mod(&mut self, node: &IfMod) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Cond => self.find(&node.cond),
            PatternItem::IfTrue => self.maybe_find(&node.if_true),
            PatternItem::IfFalse => self.maybe_find(&node.if_false),
            _ => None,
        }
    }

    fn on_if_ternary(&mut self, node: &IfTernary) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Cond => self.find(&node.cond),
            PatternItem::IfTrue => self.find(&node.if_true),
            PatternItem::IfFalse => self.find(&node.if_false),
            _ => None,
        }
    }

    fn on_index(&mut self, node: &Index) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Recv => self.find(&node.recv),
            PatternItem::Index(n) => self.find(&node.indexes[n]),
            _ => None,
        }
    }

    fn on_index_asgn(&mut self, node: &IndexAsgn) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Recv => self.find(&node.recv),
            PatternItem::Index(n) => self.find(&node.indexes[n]),
            PatternItem::Value => self.maybe_find(&node.value),
            _ => None,
        }
    }

    fn on_in_match(&mut self, node: &InMatch) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Value => self.find(&node.value),
            PatternItem::Pattern => self.find(&node.pattern),
            _ => None,
        }
    }

    fn on_in_pattern(&mut self, node: &InPattern) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Pattern => self.find(&node.pattern),
            PatternItem::Guard => self.maybe_find(&node.guard),
            PatternItem::Body => self.maybe_find(&node.body),
            _ => None,
        }
    }

    fn on_int(&mut self, _: &Int) -> Option<Node> {
        None
    }

    fn on_irange(&mut self, node: &Irange) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Left => self.maybe_find(&node.left),
            PatternItem::Right => self.maybe_find(&node.right),
            _ => None,
        }
    }

    fn on_ivar(&mut self, _: &Ivar) -> Option<Node> {
        None
    }

    fn on_ivasgn(&mut self, node: &Ivasgn) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Value => self.maybe_find(&node.value),
            _ => None,
        }
    }

    fn on_kwarg(&mut self, _: &Kwarg) -> Option<Node> {
        None
    }

    fn on_kwbegin(&mut self, node: &KwBegin) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Stmt(n) => self.find(&node.statements[n]),
            _ => None,
        }
    }

    fn on_kwnilarg(&mut self, _: &Kwnilarg) -> Option<Node> {
        None
    }

    fn on_kwoptarg(&mut self, node: &Kwoptarg) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Default => self.find(&node.default),
            _ => None,
        }
    }

    fn on_kwrestarg(&mut self, _: &Kwrestarg) -> Option<Node> {
        None
    }

    fn on_kwsplat(&mut self, node: &Kwsplat) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Value => self.find(&node.value),
            _ => None,
        }
    }

    fn on_lambda(&mut self, _: &Lambda) -> Option<Node> {
        None
    }

    fn on_line(&mut self, _: &Line) -> Option<Node> {
        None
    }

    fn on_lvar(&mut self, _: &Lvar) -> Option<Node> {
        None
    }

    fn on_lvasgn(&mut self, node: &Lvasgn) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Value => self.maybe_find(&node.value),
            _ => None,
        }
    }

    fn on_masgn(&mut self, node: &Masgn) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Lhs => self.find(&node.lhs),
            PatternItem::Rhs => self.find(&node.rhs),
            _ => None,
        }
    }

    fn on_match_alt(&mut self, node: &MatchAlt) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Lhs => self.find(&node.lhs),
            PatternItem::Rhs => self.find(&node.rhs),
            _ => None,
        }
    }

    fn on_match_as(&mut self, node: &MatchAs) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Value => self.find(&node.value),
            PatternItem::As => self.find(&node.as_),
            _ => None,
        }
    }

    fn on_match_current_line(&mut self, node: &MatchCurrentLine) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Re => self.find(&node.re),
            _ => None,
        }
    }

    fn on_match_nil_pattern(&mut self, _: &MatchNilPattern) -> Option<Node> {
        None
    }

    fn on_match_rest(&mut self, node: &MatchRest) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Name => self.maybe_find(&node.name),
            _ => None,
        }
    }

    fn on_match_var(&mut self, _: &MatchVar) -> Option<Node> {
        None
    }

    fn on_match_with_lvasgn(&mut self, node: &MatchWithLvasgn) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Re => self.find(&node.re),
            PatternItem::Value => self.find(&node.value),
            _ => None,
        }
    }

    fn on_mlhs(&mut self, node: &Mlhs) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Item(n) => self.find(&node.items[n]),
            _ => None,
        }
    }

    fn on_module(&mut self, node: &Module) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Name => self.find(&node.name),
            PatternItem::Body => self.maybe_find(&node.body),
            _ => None,
        }
    }

    fn on_next(&mut self, node: &Next) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Arg(n) => self.find(&node.args[n]),
            _ => None,
        }
    }

    fn on_nil(&mut self, _: &Nil) -> Option<Node> {
        None
    }

    fn on_nth_ref(&mut self, _: &NthRef) -> Option<Node> {
        None
    }

    fn on_numblock(&mut self, node: &Numblock) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Call => self.find(&node.call),
            PatternItem::Body => self.find(&node.body),
            _ => None,
        }
    }

    fn on_op_asgn(&mut self, node: &OpAsgn) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Recv => self.find(&node.recv),
            PatternItem::Value => self.find(&node.value),
            _ => None,
        }
    }

    fn on_optarg(&mut self, node: &Optarg) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Default => self.find(&node.default),
            _ => None,
        }
    }

    fn on_or(&mut self, node: &Or) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Lhs => self.find(&node.lhs),
            PatternItem::Rhs => self.find(&node.rhs),
            _ => None,
        }
    }

    fn on_or_asgn(&mut self, node: &OrAsgn) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Recv => self.find(&node.recv),
            PatternItem::Value => self.find(&node.value),
            _ => None,
        }
    }

    fn on_pair(&mut self, node: &Pair) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Key => self.find(&node.key),
            PatternItem::Value => self.find(&node.value),
            _ => None,
        }
    }

    fn on_pin(&mut self, node: &Pin) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Var => self.find(&node.var),
            _ => None,
        }
    }

    fn on_postexe(&mut self, node: &Postexe) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Body => self.maybe_find(&node.body),
            _ => None,
        }
    }

    fn on_preexe(&mut self, node: &Preexe) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Body => self.maybe_find(&node.body),
            _ => None,
        }
    }

    fn on_procarg0(&mut self, node: &Procarg0) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Arg(n) => self.find(&node.args[n]),
            _ => None,
        }
    }

    fn on_rational(&mut self, _: &Rational) -> Option<Node> {
        None
    }

    fn on_redo(&mut self, _: &Redo) -> Option<Node> {
        None
    }

    fn on_regexp(&mut self, node: &Regexp) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Part(n) => self.find(&node.parts[n]),
            PatternItem::Options => self.maybe_find(&node.options),
            _ => None,
        }
    }

    fn on_regopt(&mut self, _: &RegOpt) -> Option<Node> {
        None
    }

    fn on_rescue(&mut self, node: &Rescue) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Body => self.maybe_find(&node.body),
            PatternItem::RescueBody(n) => self.find(&node.rescue_bodies[n]),
            PatternItem::Else => self.maybe_find(&node.else_),
            _ => None,
        }
    }

    fn on_rescue_body(&mut self, node: &RescueBody) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::ExcList => self.maybe_find(&node.exc_list),
            PatternItem::ExcVar => self.maybe_find(&node.exc_var),
            PatternItem::Body => self.maybe_find(&node.body),
            _ => None,
        }
    }

    fn on_restarg(&mut self, _: &Restarg) -> Option<Node> {
        None
    }

    fn on_retry(&mut self, _: &Retry) -> Option<Node> {
        None
    }

    fn on_return(&mut self, node: &Return) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Arg(n) => self.find(&node.args[n]),
            _ => None,
        }
    }

    fn on_sclass(&mut self, node: &SClass) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Expr => self.find(&node.expr),
            PatternItem::Body => self.maybe_find(&node.body),
            _ => None,
        }
    }

    fn on_self_(&mut self, _: &Self_) -> Option<Node> {
        None
    }

    fn on_send(&mut self, node: &Send) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Recv => self.maybe_find(&node.recv),
            PatternItem::Arg(n) => self.find(&node.args[n]),
            _ => None,
        }
    }

    fn on_shadowarg(&mut self, _: &Shadowarg) -> Option<Node> {
        None
    }

    fn on_splat(&mut self, node: &Splat) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Value => self.maybe_find(&node.value),
            _ => None,
        }
    }

    fn on_str(&mut self, _: &Str) -> Option<Node> {
        None
    }

    fn on_super(&mut self, node: &Super) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Arg(n) => self.find(&node.args[n]),
            _ => None,
        }
    }

    fn on_sym(&mut self, _: &Sym) -> Option<Node> {
        None
    }

    fn on_true(&mut self, _: &True) -> Option<Node> {
        None
    }

    fn on_undef(&mut self, node: &Undef) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Arg(n) => self.find(&node.names[n]),
            _ => None,
        }
    }

    fn on_unless_guard(&mut self, node: &UnlessGuard) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Cond => self.find(&node.cond),
            _ => None,
        }
    }

    fn on_until(&mut self, node: &Until) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Cond => self.find(&node.cond),
            PatternItem::Body => self.maybe_find(&node.body),
            _ => None,
        }
    }

    fn on_until_post(&mut self, node: &UntilPost) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Cond => self.find(&node.cond),
            PatternItem::Body => self.find(&node.body),
            _ => None,
        }
    }

    fn on_when(&mut self, node: &When) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Arg(n) => self.find(&node.patterns[n]),
            PatternItem::Body => self.maybe_find(&node.body),
            _ => None,
        }
    }

    fn on_while(&mut self, node: &While) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Cond => self.find(&node.cond),
            PatternItem::Body => self.maybe_find(&node.body),
            _ => None,
        }
    }

    fn on_while_post(&mut self, node: &WhilePost) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Cond => self.find(&node.cond),
            PatternItem::Body => self.find(&node.body),
            _ => None,
        }
    }

    fn on_xheredoc(&mut self, node: &XHeredoc) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Part(n) => self.find(&node.parts[n]),
            _ => None,
        }
    }

    fn on_xstr(&mut self, node: &Xstr) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Part(n) => self.find(&node.parts[n]),
            _ => None,
        }
    }

    fn on_yield(&mut self, node: &Yield) -> Option<Node> {
        match self.current_pattern() {
            PatternItem::Arg(n) => self.find(&node.args[n]),
            _ => None,
        }
    }

    fn on_zsuper(&mut self, _: &ZSuper) -> Option<Node> {
        None
    }
}
