use crate::source::Range;
use crate::source::map::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Begin { statements: Vec<Node>, loc: CollectionMap },
    Int { value: String, loc: OperatorMap },
    Float { value: String, loc: OperatorMap },
    Rational { value: String, loc: OperatorMap },
    Complex { value: String, loc: OperatorMap },
    Send { receiver: Option<Box<Node>>, operator: String, args: Vec<Node>, loc: SendMap },
    CSend { receiver: Option<Box<Node>>, operator: String, args: Vec<Node>, loc: SendMap },
    Nil { loc: Map },
    True { loc: Map },
    False { loc: Map },
    Self_ { loc: Map },
    __FILE__ { loc: Map },
    __LINE__ { loc: Map },
    __ENCODING__ { loc: Map },
    Preexe { body: Option<Box<Node>>, loc: KeywordMap },
    Postexe { body: Option<Box<Node>>, loc: KeywordMap },
    Lvar { name: String, loc: VariableMap },
    Rescue { body: Option<Box<Node>>, rescue_bodies: Vec<Node>, else_: Option<Box<Node>>, loc: ConditionMap },
    Ensure { body: Option<Box<Node>>, ensure: Box<Node>, loc: ConditionMap },
    KwBegin { statements: Vec<Node>, loc: CollectionMap },
    Args { args: Vec<Node>, loc: CollectionMap },
    Def { name: String, args: Option<Box<Node>>, body: Option<Box<Node>>, loc: MethodDefinitionMap },
    Defs { definee: Box<Node>, name: String, args: Option<Box<Node>>, body: Option<Box<Node>>, loc: MethodDefinitionMap },
    Arg { name: String, loc: VariableMap },
    Sym { name: Vec<u8>, loc: CollectionMap },
    Alias { to: Box<Node>, from: Box<Node>, loc: KeywordMap },
    Ivar { name: String, loc: VariableMap },
    Gvar { name: String, loc: VariableMap },
    Cvar { name: String, loc: VariableMap },
    BackRef { name: String, loc: VariableMap },
    NthRef { name: String, loc: VariableMap },
    Lvasgn { name: String, rhs: Option<Box<Node>>, loc: VariableMap },
    Cvasgn { name: String, rhs: Option<Box<Node>>, loc: VariableMap },
    Ivasgn { name: String, rhs: Option<Box<Node>>, loc: VariableMap },
    Gvasgn { name: String, rhs: Option<Box<Node>>, loc: VariableMap },
    Const { scope: Option<Box<Node>>, name: String, loc: ConstantMap },
    Casgn { scope: Option<Box<Node>>, name: String, rhs: Option<Box<Node>>, loc: ConstantMap },
    Index { receiver: Box<Node>, indexes: Vec<Node>, loc: IndexMap },
    IndexAsgn { receiver: Box<Node>, indexes: Vec<Node>, rhs: Option<Box<Node>>, loc: IndexMap },
    Undef { names: Vec<Node>, loc: KeywordMap },
    Pair { key: Box<Node>, value: Box<Node>, loc: OperatorMap },
    Hash { pairs: Vec<Node>, loc: CollectionMap },
    Array { elements: Vec<Node>, loc: CollectionMap },
    Str { value: Vec<u8>, loc: CollectionMap },
    Dstr { children: Vec<Node>, loc: CollectionMap },
    Xstr { children: Vec<Node>, loc: CollectionMap },
    Dsym { children: Vec<Node>, loc: CollectionMap },
    IfMod { cond: Box<Node>, if_true: Option<Box<Node>>, if_false: Option<Box<Node>>, loc: KeywordMap },
    IfTernary { cond: Box<Node>, if_true: Box<Node>, if_false: Box<Node>, loc: TernaryMap },
    If { cond: Box<Node>, if_true: Option<Box<Node>>, if_false: Option<Box<Node>>, loc: ConditionMap },
    WhilePost { cond: Box<Node>, body: Box<Node>, loc: KeywordMap },
    While { cond: Box<Node>, body: Box<Node>, loc: KeywordMap },
    UntilPost { cond: Box<Node>, body: Box<Node>, loc: KeywordMap },
    Until { cond: Box<Node>, body: Box<Node>, loc: KeywordMap },
    RescueBody { exc_list: Option<Box<Node>>, exc_var: Option<Box<Node>>, stmt: Option<Box<Node>>, loc: RescueBodyMap },
    Mlhs { items: Vec<Node>, loc: CollectionMap },
    Splat { arg: Option<Box<Node>>, loc: OperatorMap },
    Masgn { lhs: Box<Node>, rhs: Box<Node>, loc: OperatorMap },
    Cbase { loc: Map },
    Break { args: Vec<Node>, loc: KeywordMap },
    Defined { args: Vec<Node>, loc: KeywordMap },
    Next { args: Vec<Node>, loc: KeywordMap },
    Redo { args: Vec<Node>, loc: KeywordMap },
    Retry { args: Vec<Node>, loc: KeywordMap },
    Return { args: Vec<Node>, loc: KeywordMap },
    Super { args: Vec<Node>, loc: KeywordMap },
    Yield { args: Vec<Node>, loc: KeywordMap },
    Zsuper { args: Vec<Node>, loc: KeywordMap },
    AndAsgn { lhs: Box<Node>, rhs: Box<Node>, loc: OpAssignMap },
    OrAsgn { lhs: Box<Node>, rhs: Box<Node>, loc: OpAssignMap },
    OpAsgn { lhs: Box<Node>, rhs: Box<Node>, operator: String, loc: OpAssignMap },
    And { lhs: Box<Node>, rhs: Box<Node>, loc: OperatorMap },
    Or { lhs: Box<Node>, rhs: Box<Node>, loc: OperatorMap },
    RegOpt { options: Vec<char>, loc: Map },
    Regexp { parts: Vec<Node>, options: Box<Node>, loc: CollectionMap },
    Kwsplat { value: Box<Node>, loc: OperatorMap },
    Irange { left: Option<Box<Node>>, right: Option<Box<Node>>, loc: OperatorMap  },
    Erange { left: Option<Box<Node>>, right: Option<Box<Node>>, loc: OperatorMap  },
    Class { name: Box<Node>, superclass: Option<Box<Node>>, body: Option<Box<Node>>, loc: DefinitionMap },
    Sclass { expr: Box<Node>, body: Option<Box<Node>>, loc: DefinitionMap },
    Module { name: Box<Node>, body: Option<Box<Node>>, loc: DefinitionMap },
    ForwardArg { loc: Map },
    Optarg { name: String, value: Box<Node>, loc: VariableMap },
    Restarg { name: Option<String>, loc: VariableMap },
    Kwarg { name: String, loc: VariableMap  },
    Kwoptarg { name: String, value: Box<Node>, loc: VariableMap  },
    Kwrestarg { name: Option<String>, loc: VariableMap },
    Kwnilarg { loc: VariableMap },
    Shadowarg { name: String, loc: VariableMap },
    Blockarg { name: String, loc: VariableMap },
    Procarg0 { arg: Box<Node>, loc: CollectionMap },
    ForwardedArgs { loc: Map },
    Block { call: Box<Node>, args: Option<Box<Node>>, body: Option<Box<Node>>, loc: CollectionMap },
    Lambda { loc: Map },
    BlockPass { arg: Box<Node>, loc: OperatorMap },
    MatchWithLvasgn { receiver: Box<Node>, arg: Box<Node>, loc: SendMap },
    For { iterator: Box<Node>, iteratee: Box<Node>, body: Option<Box<Node>>, loc: ForMap },
    When { patterns: Vec<Node>, body: Option<Box<Node>>, loc: KeywordMap },
    Case { expr: Option<Box<Node>>, when_bodies: Vec<Node>, else_body: Option<Box<Node>>, loc: ConditionMap },
}

impl Node {
    pub fn expression(&self) -> &Range {
        match self {
            Self::Begin { loc, .. } => &loc.expression,
            Self::Int { loc, .. } => &loc.expression,
            Self::Float { loc, .. } => &loc.expression,
            Self::Rational { loc, .. } => &loc.expression,
            Self::Complex { loc, .. } => &loc.expression,
            Self::Send { loc, .. } => &loc.expression,
            Self::CSend { loc, .. } => &loc.expression,
            Self::Nil { loc, .. } => &loc.expression,
            Self::True { loc, .. } => &loc.expression,
            Self::False { loc, .. } => &loc.expression,
            Self::Self_ { loc, .. } => &loc.expression,
            Self::__FILE__ { loc, .. } => &loc.expression,
            Self::__LINE__ { loc, .. } => &loc.expression,
            Self::__ENCODING__ { loc, .. } => &loc.expression,
            Self::Preexe { loc, .. } => &loc.expression,
            Self::Postexe { loc, .. } => &loc.expression,
            Self::Lvar { loc, .. } => &loc.expression,
            Self::Rescue { loc, .. } => &loc.expression,
            Self::Ensure { loc, .. } => &loc.expression,
            Self::KwBegin { loc, .. } => &loc.expression,
            Self::Args { loc, .. } => &loc.expression,
            Self::Def { loc, .. } => &loc.expression,
            Self::Defs { loc, .. } => &loc.expression,
            Self::Arg { loc, .. } => &loc.expression,
            Self::Sym { loc, .. } => &loc.expression,
            Self::Alias { loc, .. } => &loc.expression,
            Self::Ivar { loc, .. } => &loc.expression,
            Self::Gvar { loc, .. } => &loc.expression,
            Self::Cvar { loc, .. } => &loc.expression,
            Self::BackRef { loc, .. } => &loc.expression,
            Self::NthRef { loc, .. } => &loc.expression,
            Self::Lvasgn { loc, .. } => &loc.expression,
            Self::Cvasgn { loc, .. } => &loc.expression,
            Self::Ivasgn { loc, .. } => &loc.expression,
            Self::Gvasgn { loc, .. } => &loc.expression,
            Self::Const { loc, .. } => &loc.expression,
            Self::Casgn { loc, .. } => &loc.expression,
            Self::Index { loc, .. } => &loc.expression,
            Self::IndexAsgn { loc, .. } => &loc.expression,
            Self::Undef { loc, .. } => &loc.expression,
            Self::Pair { loc, .. } => &loc.expression,
            Self::Hash { loc, .. } => &loc.expression,
            Self::Array { loc, .. } => &loc.expression,
            Self::Str { loc, .. } => &loc.expression,
            Self::Dstr { loc, .. } => &loc.expression,
            Self::Xstr { loc, .. } => &loc.expression,
            Self::Dsym { loc, .. } => &loc.expression,
            Self::IfMod { loc, .. } => &loc.expression,
            Self::IfTernary { loc, .. } => &loc.expression,
            Self::If { loc, .. } => &loc.expression,
            Self::While { loc, .. } => &loc.expression,
            Self::WhilePost { loc, .. } => &loc.expression,
            Self::Until { loc, .. } => &loc.expression,
            Self::UntilPost { loc, .. } => &loc.expression,
            Self::RescueBody { loc, .. } => &loc.expression,
            Self::Mlhs { loc, .. } => &loc.expression,
            Self::Splat { loc, .. } => &loc.expression,
            Self::Masgn { loc, .. } => &loc.expression,
            Self::Cbase { loc, .. } => &loc.expression,
            Self::Break { loc, .. } => &loc.expression,
            Self::Defined { loc, .. } => &loc.expression,
            Self::Next { loc, .. } => &loc.expression,
            Self::Redo { loc, .. } => &loc.expression,
            Self::Retry { loc, .. } => &loc.expression,
            Self::Return { loc, .. } => &loc.expression,
            Self::Super { loc, .. } => &loc.expression,
            Self::Yield { loc, .. } => &loc.expression,
            Self::Zsuper { loc, .. } => &loc.expression,
            Self::AndAsgn { loc, .. } => &loc.expression,
            Self::OrAsgn { loc, .. } => &loc.expression,
            Self::OpAsgn { loc, .. } => &loc.expression,
            Self::And { loc, .. } => &loc.expression,
            Self::Or { loc, .. } => &loc.expression,
            Self::RegOpt { loc, .. } => &loc.expression,
            Self::Regexp { loc, .. } => &loc.expression,
            Self::Kwsplat { loc, .. } => &loc.expression,
            Self::Irange { loc, .. } => &loc.expression,
            Self::Erange { loc, .. } => &loc.expression,
            Self::Class { loc, .. } => &loc.expression,
            Self::Module { loc, .. } => &loc.expression,
            Self::Sclass { loc, .. } => &loc.expression,
            Self::ForwardArg { loc, .. } => &loc.expression,
            Self::Optarg { loc, .. } => &loc.expression,
            Self::Restarg { loc, .. } => &loc.expression,
            Self::Kwarg { loc, .. } => &loc.expression,
            Self::Kwoptarg { loc, .. } => &loc.expression,
            Self::Kwrestarg { loc, .. } => &loc.expression,
            Self::Kwnilarg { loc, .. } => &loc.expression,
            Self::Shadowarg { loc, .. } => &loc.expression,
            Self::Blockarg { loc, .. } => &loc.expression,
            Self::Procarg0 { loc, .. } => &loc.expression,
            Self::ForwardedArgs { loc } => &loc.expression,
            Self::Block { loc, .. } => &loc.expression,
            Self::Lambda { loc, .. } => &loc.expression,
            Self::BlockPass { loc, .. } => &loc.expression,
            Self::MatchWithLvasgn { loc, .. } => &loc.expression,
            Self::For { loc, .. } => &loc.expression,
            Self::When { loc, .. } => &loc.expression,
            Self::Case { loc, .. } => &loc.expression,
        }
    }

    pub fn inspect(&self, indent: usize) -> String {
        let indented = "  ".repeat(indent);
        let mut sexp = format!("{}s(:{}", indented, self.str_type());

        for child in self.inspected_children(indent) {
            sexp.push_str(&child);
        }

        sexp.push_str(")");

        sexp
    }

    fn str_type(&self) -> &'static str {
        match self {
            Node::Begin { .. } => "begin",
            Node::Int { .. } => "int",
            Node::Float { .. } => "float",
            Node::Rational { .. } => "rational",
            Node::Complex { .. } => "complex",
            Node::Send { .. } => "send",
            Node::CSend { .. } => "csend",
            Node::Nil { .. } => "nil",
            Node::True { .. } => "true",
            Node::False { .. } => "false",
            Node::Self_ { .. } => "self",
            Node::__FILE__ { .. } => "__FILE__",
            Node::__LINE__ { .. } => "__LINE__",
            Node::__ENCODING__ { .. } => "__ENCODING__",
            Node::Preexe { .. } => "preexe",
            Node::Postexe { .. } => "postexe",
            Node::Lvar { .. } => "lvar",
            Node::Rescue { .. } => "rescue",
            Node::Ensure { .. } => "ensure",
            Node::KwBegin { .. } => "kwbegin",
            Node::Args { .. } => "args",
            Node::Def { .. } => "def",
            Node::Defs { .. } => "defs",
            Node::Arg { .. } => "arg",
            Node::Sym { .. } => "sym",
            Node::Alias { .. } => "alias",
            Node::Ivar { .. } => "ivar",
            Node::Gvar { .. } => "gvar",
            Node::Cvar { .. } => "cvar",
            Node::BackRef { .. } => "backref",
            Node::NthRef { .. } => "nthref",
            Node::Lvasgn { .. } => "lvasgn",
            Node::Cvasgn { .. } => "cvasgn",
            Node::Ivasgn { .. } => "ivasgn",
            Node::Gvasgn { .. } => "gvasgn",
            Node::Const { .. } => "const",
            Node::Casgn { .. } => "casgn",
            Node::Index { .. } => "index",
            Node::IndexAsgn { .. } => "index_asgn",
            Node::Undef { .. } => "undef",
            Node::Pair { .. } => "pair",
            Node::Hash { .. } => "hash",
            Node::Array { .. } => "array",
            Node::Str { .. } => "str",
            Node::Dstr { .. } => "dstr",
            Node::Xstr { .. } => "dstr",
            Node::Dsym { .. } => "dsym",
            Node::IfMod { .. }
            | Node::IfTernary { .. }
            | Node::If { .. } => "if",
            Node::WhilePost { .. } => "while_post",
            Node::While { .. } => "while",
            Node::UntilPost { .. } => "until_post",
            Node::Until { .. } => "until",
            Node::RescueBody { .. } => "resbody",
            Node::Mlhs { .. } => "mlhs",
            Node::Splat { .. } => "splat",
            Node::Masgn { .. } => "masgn",
            Node::Cbase { .. } => "cbase",
            Node::Break { .. } => "break",
            Node::Defined { .. } => "defined",
            Node::Next { .. } => "next",
            Node::Redo { .. } => "redo",
            Node::Retry { .. } => "retry",
            Node::Return { .. } => "return",
            Node::Super { .. } => "super",
            Node::Yield { .. } => "yield",
            Node::Zsuper { .. } => "zsuper",
            Node::AndAsgn { .. } => "and_asgn",
            Node::OrAsgn { .. } => "or_asgn",
            Node::OpAsgn { .. } => "op_asgn",
            Node::And { .. } => "and",
            Node::Or { .. } => "or",
            Node::RegOpt { .. } => "regopt",
            Node::Regexp { .. } => "regexp",
            Node::Kwsplat { .. } => "kwsplat",
            Node::Irange { .. } => "irange",
            Node::Erange { .. } => "Erange",
            Node::Class { .. } => "class",
            Node::Sclass { .. } => "sclass",
            Node::Module { .. } => "sclass",
            Node::ForwardArg { .. } => "forward_arg",
            Node::Optarg { .. } => "optarg",
            Node::Restarg { .. } => "restarg",
            Node::Kwarg { .. } => "kwarg",
            Node::Kwoptarg { .. } => "kwoptarg",
            Node::Kwrestarg { .. } => "kwrestarg",
            Node::Kwnilarg { .. } => "kwnilarg",
            Node::Shadowarg { .. } => "shadowarg",
            Node::Blockarg { .. } => "blockarg",
            Node::Procarg0 { .. } => "procarg0",
            Node::ForwardedArgs { .. } => "forwarded_args",
            Node::Block { .. } => "block",
            Node::Lambda { .. } => "lambda",
            Node::BlockPass { .. } => "block_pass",
            Node::MatchWithLvasgn { .. } => "match_with_lvasgn",
            Node::For { .. } => "for",
            Node::When { .. } => "when",
            Node::Case { .. } => "case",
        }
    }

    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);

        match self {
            Node::Begin { statements, .. } => {
                result.push_nodes(statements);
            },
            Node::Int { value, .. }
            | Node::Float { value, .. }
            | Node::Rational { value, .. }
            | Node::Complex { value, .. } => {
                result.push_str(value);
            }
            Node::Send { receiver, operator, args, .. }
            | Node::CSend { receiver, operator, args, .. } => {
                if let Some(receiver) = receiver {
                    result.push_node(&receiver);
                } else {
                    result.push_nil();
                }

                result.push_str(operator);
                result.push_nodes(args);
            }
            Node::Nil { .. }
            | Node::True { .. }
            | Node::False { .. }
            | Node::Self_ { .. }
            | Node::__FILE__ { .. }
            | Node::__LINE__ { .. }
            | Node::__ENCODING__ { .. } => {}
            Node::Preexe { body, .. }
            | Node::Postexe { body, .. } => {
                if let Some(body) = body {
                    result.push_node(body);
                }
            }
            Node::Lvar { name, .. } => {
                result.push_str(name);
            }
            Node::Rescue { body, rescue_bodies, else_, .. } => {
                if let Some(body) = body {
                    result.push_node(body);
                } else {
                    result.push_nil();
                }
                result.push_nodes(rescue_bodies);
                if let Some(else_) = else_ {
                    result.push_node(else_);
                } else {
                    result.push_nil();
                }
            }
            Node::Ensure { body, ensure, .. } => {
                if let Some(body) = body {
                    result.push_node(body)
                } else {
                    result.push_nil()
                }
                result.push_node(ensure)
            }
            Node::KwBegin { statements, .. } => {
                result.push_nodes(statements)
            }
            Node::Args { args, .. } => {
                result.push_nodes(args)
            }
            Node::Def { name, args, body, .. } => {
                result.push_str(name);
                if let Some(args) = args {
                    result.push_node(args)
                } else {
                    result.push_nil()
                }
                if let Some(body) = body {
                    result.push_node(body)
                } else {
                    result.push_nil()
                }
            }
            Node::Defs { definee, name, args, body, .. } => {
                result.push_node(definee);
                result.push_str(name);
                if let Some(args) = args {
                    result.push_node(args)
                } else {
                    result.push_nil()
                }
                if let Some(body) = body {
                    result.push_node(body)
                } else {
                    result.push_nil()
                }
            }
            Node::Arg { name, .. } => {
                result.push_str(name)
            }
            Node::Alias { to, from, .. } => {
                result.push_node(to);
                result.push_node(from)
            }
            Node::Ivar { name, .. }
            | Node::Gvar { name, .. }
            | Node::Cvar { name, .. }
            | Node::BackRef { name, .. }
            | Node::NthRef { name, .. } => {
                result.push_str(name)
            }
            Node::Lvasgn { name, rhs, .. }
            | Node::Cvasgn { name, rhs, .. }
            | Node::Ivasgn { name, rhs, .. }
            | Node::Gvasgn { name, rhs, .. } => {
                result.push_str(name);
                if let Some(rhs) = rhs {
                    result.push_node(rhs)
                }
            }
            Node::Const { scope, name, .. } => {
                if let Some(scope) = scope {
                    result.push_node(scope)
                } else {
                    result.push_nil()
                }
                result.push_str(name)
            }
            Node::Casgn { scope, name, rhs, .. } => {
                if let Some(scope) = scope {
                    result.push_node(scope)
                } else {
                    result.push_nil()
                }
                result.push_str(name);
                if let Some(rhs) = rhs {
                    result.push_node(rhs)
                }
            }
            Node::Index { receiver, indexes, .. } => {
                result.push_node(receiver);
                result.push_nodes(indexes);
            }
            Node::IndexAsgn { receiver, indexes, rhs, .. } => {
                result.push_node(receiver);
                result.push_nodes(indexes);
                if let Some(rhs) = rhs {
                    result.push_node(rhs)
                }
            }
            Node::Undef { names, .. } => {
                result.push_nodes(names)
            }
            Node::Pair { key, value, .. } => {
                result.push_node(key);
                result.push_node(value)
            }
            Node::Hash { pairs, .. } => {
                result.push_nodes(pairs)
            }
            Node::Array { elements, .. } => {
                result.push_nodes(elements)
            }
            Node::Str { value, .. }
            | Node::Sym { name: value, .. } => {
                match String::from_utf8(value.to_owned()) {
                    Ok(string) => result.push_str(&string),
                    Err(_) => {
                        let string = String::from_utf8_lossy(&value).into_owned();
                        result.push_str(&string)
                    }
                }
            }
            Node::Dstr { children, .. }
            | Node::Dsym { children, .. }
            | Node::Xstr { children, .. } => {
                result.push_nodes(children)
            }
            Node::If { cond, if_true, if_false, .. }
            | Node::IfMod { cond, if_true, if_false, .. } => {
                result.push_node(cond);
                if let Some(if_true) = if_true {
                    result.push_node(if_true)
                } else {
                    result.push_nil()
                }
                if let Some(if_false) = if_false {
                    result.push_node(if_false)
                } else {
                    result.push_nil()
                }
            }
            Node::IfTernary { cond, if_true, if_false, .. } => {
                result.push_node(cond);
                result.push_node(if_true);
                result.push_node(if_false);
            }
            Node::WhilePost { cond, body, .. }
            | Node::While { cond, body, .. }
            | Node::UntilPost { cond, body, .. }
            | Node::Until { cond, body, .. } => {
                result.push_node(cond);
                result.push_node(body)
            }
            Node::RescueBody { exc_list, exc_var, stmt, .. } => {
                if let Some(exc_list) = exc_list {
                    result.push_node(exc_list);
                } else {
                    result.push_nil();
                }
                if let Some(exc_var) = exc_var {
                    result.push_node(exc_var)
                } else {
                    result.push_nil()
                }
                if let Some(stmt) = stmt {
                    result.push_node(stmt)
                } else {
                    result.push_nil()
                }
            }
            Node::Mlhs { items, .. } => {
                result.push_nodes(items)
            }
            Node::Splat { arg, .. } => {
                if let Some(arg) = arg {
                    result.push_node(arg)
                }
            }
            Node::Masgn { lhs, rhs, .. } => {
                result.push_node(lhs);
                result.push_node(rhs);
            }
            Node::Cbase { .. } => {}
            Node::Break { args, .. }
            | Node::Defined { args, .. }
            | Node::Next { args, .. }
            | Node::Redo { args, .. }
            | Node::Retry { args, .. }
            | Node::Return { args, .. }
            | Node::Super { args, .. }
            | Node::Yield { args, .. }
            | Node::Zsuper { args, .. } => {
                result.push_nodes(args)
            }
            Node::AndAsgn { lhs, rhs, .. }
            | Node::OrAsgn { lhs, rhs, .. }
            | Node::And { lhs, rhs, .. }
            | Node::Or { lhs, rhs, .. } => {
                result.push_node(lhs);
                result.push_node(rhs);
            }
            Node::OpAsgn { lhs, rhs, operator, .. } => {
                result.push_node(lhs);
                result.push_str(operator);
                result.push_node(rhs);
            },
            Node::RegOpt { options, .. } => {
                for option in options {
                    result.push_str(&format!("{}", option));
                }
            },
            Node::Regexp { parts, options, .. } => {
                result.push_nodes(parts);
                result.push_node(options);
            },
            Node::Kwsplat { value, .. } => {
                result.push_node(value);
            },
            Node::Irange { left, right, .. }
            | Node::Erange { left, right, .. } => {
                if let Some(left) = left {
                    result.push_node(left);
                } else {
                    result.push_nil()
                }
                if let Some(right) = right {
                    result.push_node(right);
                } else {
                    result.push_nil()
                }
            },
            Node::Class { name, superclass, body, .. } => {
                result.push_node(name);
                if let Some(superclass) = superclass {
                    result.push_node(superclass);
                }
                if let Some(body) = body {
                    result.push_node(body);
                }
            },
            Node::Sclass { expr, body, .. } => {
                result.push_node(expr);
                if let Some(body) = body {
                    result.push_node(body);
                } else {
                    result.push_nil();
                }
            },
            Node::Module { name, body, .. } => {
                result.push_node(name);
                if let Some(body) = body {
                    result.push_node(body);
                } else {
                    result.push_nil();
                }
            },
            Node::ForwardArg { .. } => {},
            Node::Optarg { name, value, .. } => {
                result.push_str(name);
                result.push_node(value);
            }
            Node::Restarg { name, .. } => {
                if let Some(name) = name {
                    result.push_str(name);
                }
            }
            Node::Kwarg { name, .. } => {
                result.push_str(name);

            }
            Node::Kwoptarg { name, value, .. } => {
                result.push_str(name);
                result.push_node(value);
            }
            Node::Kwrestarg { name, .. } => {
                if let Some(name) = name {
                    result.push_str(name);
                }
            }
            Node::Kwnilarg { .. } => {}
            Node::Shadowarg { name, .. } => {
                result.push_str(name);
            }
            Node::Blockarg { name, .. } => {
                result.push_str(name);

            }
            Node::Procarg0 { arg, .. } => {
                result.push_node(arg);
            }
            Node::ForwardedArgs { .. } => {}
            Node::Block { call, args, body, .. } => {
                result.push_node(call);
                if let Some(args) = args {
                    result.push_node(args)
                } else {
                    result.push_nil()
                }
                if let Some(body) = body {
                    result.push_node(body)
                } else {
                    result.push_nil()
                }
            }
            Node::Lambda { .. } => {}
            Node::BlockPass { arg, .. } => {
                result.push_node(arg)
            }
            Node::MatchWithLvasgn { receiver, arg, .. } => {
                result.push_node(receiver);
                result.push_node(arg);
            }
            Node::For { iterator, iteratee, body, .. } => {
                result.push_node(iterator);
                result.push_node(iteratee);
                if let Some(body) = body {
                    result.push_node(body)
                } else {
                    result.push_nil()
                }
            }
            Node::When { patterns, body, .. } => {
                result.push_nodes(patterns);
                if let Some(body) = body {
                    result.push_node(body)
                } else {
                    result.push_nil()
                }
            }
            Node::Case { expr, when_bodies, else_body, .. } => {
                if let Some(expr) = expr {
                    result.push_node(expr)
                } else {
                    result.push_nil()
                }
                result.push_nodes(when_bodies);
                if let Some(else_body) = else_body {
                    result.push_node(else_body);
                } else {
                    result.push_nil()
                }
            }
        }

        result.strings()
    }
}

struct InspectVec {
    indent: usize,
    strings: Vec<String>
}

impl InspectVec {
    pub fn new(indent: usize) -> Self {
        Self { indent, strings: vec![] }
    }

    pub fn push_str(&mut self, string: &str) {
        self.strings.push(format!(", {:?}", string));
    }

    pub fn push_nil(&mut self) {
        self.strings.push(", nil".to_owned());
    }

    pub fn push_node(&mut self, node: &Node) {
        self.strings.push(format!(",\n{}", node.inspect(self.indent + 1)))
    }

    pub fn push_nodes(&mut self, nodes: &Vec<Node>) {
        for node in nodes {
            self.push_node(node)
        }
    }

    pub fn strings(self) -> Vec<String> {
        self.strings
    }
}
