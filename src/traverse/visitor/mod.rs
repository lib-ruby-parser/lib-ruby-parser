mod item;
pub use item::Item;

use crate::nodes::*;
use crate::Node;

pub trait Observer {
    #[allow(unused_variables)]
    fn on_alias(&mut self, node: &Alias) {}
    #[allow(unused_variables)]
    fn on_and(&mut self, node: &And) {}
    #[allow(unused_variables)]
    fn on_and_asgn(&mut self, node: &AndAsgn) {}
    #[allow(unused_variables)]
    fn on_arg(&mut self, node: &Arg) {}
    #[allow(unused_variables)]
    fn on_args(&mut self, node: &Args) {}
    #[allow(unused_variables)]
    fn on_array(&mut self, node: &Array) {}
    #[allow(unused_variables)]
    fn on_array_pattern(&mut self, node: &ArrayPattern) {}
    #[allow(unused_variables)]
    fn on_array_pattern_with_tail(&mut self, node: &ArrayPatternWithTail) {}
    #[allow(unused_variables)]
    fn on_back_ref(&mut self, node: &BackRef) {}
    #[allow(unused_variables)]
    fn on_begin(&mut self, node: &Begin) {}
    #[allow(unused_variables)]
    fn on_block(&mut self, node: &Block) {}
    #[allow(unused_variables)]
    fn on_blockarg(&mut self, node: &Blockarg) {}
    #[allow(unused_variables)]
    fn on_block_pass(&mut self, node: &BlockPass) {}
    #[allow(unused_variables)]
    fn on_break(&mut self, node: &Break) {}
    #[allow(unused_variables)]
    fn on_case(&mut self, node: &Case) {}
    #[allow(unused_variables)]
    fn on_case_match(&mut self, node: &CaseMatch) {}
    #[allow(unused_variables)]
    fn on_casgn(&mut self, node: &Casgn) {}
    #[allow(unused_variables)]
    fn on_cbase(&mut self, node: &Cbase) {}
    #[allow(unused_variables)]
    fn on_class(&mut self, node: &Class) {}
    #[allow(unused_variables)]
    fn on_complex(&mut self, node: &Complex) {}
    #[allow(unused_variables)]
    fn on_const(&mut self, node: &Const) {}
    #[allow(unused_variables)]
    fn on_const_pattern(&mut self, node: &ConstPattern) {}
    #[allow(unused_variables)]
    fn on_csend(&mut self, node: &CSend) {}
    #[allow(unused_variables)]
    fn on_cvar(&mut self, node: &Cvar) {}
    #[allow(unused_variables)]
    fn on_cvasgn(&mut self, node: &Cvasgn) {}
    #[allow(unused_variables)]
    fn on_def(&mut self, node: &Def) {}
    #[allow(unused_variables)]
    fn on_defined(&mut self, node: &Defined) {}
    #[allow(unused_variables)]
    fn on_defs(&mut self, node: &Defs) {}
    #[allow(unused_variables)]
    fn on_dstr(&mut self, node: &Dstr) {}
    #[allow(unused_variables)]
    fn on_dsym(&mut self, node: &Dsym) {}
    #[allow(unused_variables)]
    fn on_eflipflop(&mut self, node: &EFlipFlop) {}
    #[allow(unused_variables)]
    fn on_empty_else(&mut self, node: &EmptyElse) {}
    #[allow(unused_variables)]
    fn on_encoding(&mut self, node: &Encoding) {}
    #[allow(unused_variables)]
    fn on_ensure(&mut self, node: &Ensure) {}
    #[allow(unused_variables)]
    fn on_erange(&mut self, node: &Erange) {}
    #[allow(unused_variables)]
    fn on_false(&mut self, node: &False) {}
    #[allow(unused_variables)]
    fn on_file(&mut self, node: &File) {}
    #[allow(unused_variables)]
    fn on_find_pattern(&mut self, node: &FindPattern) {}
    #[allow(unused_variables)]
    fn on_float(&mut self, node: &Float) {}
    #[allow(unused_variables)]
    fn on_for(&mut self, node: &For) {}
    #[allow(unused_variables)]
    fn on_forward_arg(&mut self, node: &ForwardArg) {}
    #[allow(unused_variables)]
    fn on_forwarded_args(&mut self, node: &ForwardedArgs) {}
    #[allow(unused_variables)]
    fn on_gvar(&mut self, node: &Gvar) {}
    #[allow(unused_variables)]
    fn on_gvasgn(&mut self, node: &Gvasgn) {}
    #[allow(unused_variables)]
    fn on_hash(&mut self, node: &Hash) {}
    #[allow(unused_variables)]
    fn on_hash_pattern(&mut self, node: &HashPattern) {}
    #[allow(unused_variables)]
    fn on_heredoc(&mut self, node: &Heredoc) {}
    #[allow(unused_variables)]
    fn on_if(&mut self, node: &If) {}
    #[allow(unused_variables)]
    fn on_if_guard(&mut self, node: &IfGuard) {}
    #[allow(unused_variables)]
    fn on_iflipflop(&mut self, node: &IFlipFlop) {}
    #[allow(unused_variables)]
    fn on_if_mod(&mut self, node: &IfMod) {}
    #[allow(unused_variables)]
    fn on_if_ternary(&mut self, node: &IfTernary) {}
    #[allow(unused_variables)]
    fn on_index(&mut self, node: &Index) {}
    #[allow(unused_variables)]
    fn on_index_asgn(&mut self, node: &IndexAsgn) {}
    #[allow(unused_variables)]
    fn on_in_pattern(&mut self, node: &InPattern) {}
    #[allow(unused_variables)]
    fn on_int(&mut self, node: &Int) {}
    #[allow(unused_variables)]
    fn on_irange(&mut self, node: &Irange) {}
    #[allow(unused_variables)]
    fn on_ivar(&mut self, node: &Ivar) {}
    #[allow(unused_variables)]
    fn on_ivasgn(&mut self, node: &Ivasgn) {}
    #[allow(unused_variables)]
    fn on_kwarg(&mut self, node: &Kwarg) {}
    #[allow(unused_variables)]
    fn on_kwargs(&mut self, node: &Kwargs) {}
    #[allow(unused_variables)]
    fn on_kwbegin(&mut self, node: &KwBegin) {}
    #[allow(unused_variables)]
    fn on_kwnilarg(&mut self, node: &Kwnilarg) {}
    #[allow(unused_variables)]
    fn on_kwoptarg(&mut self, node: &Kwoptarg) {}
    #[allow(unused_variables)]
    fn on_kwrestarg(&mut self, node: &Kwrestarg) {}
    #[allow(unused_variables)]
    fn on_kwsplat(&mut self, node: &Kwsplat) {}
    #[allow(unused_variables)]
    fn on_lambda(&mut self, node: &Lambda) {}
    #[allow(unused_variables)]
    fn on_line(&mut self, node: &Line) {}
    #[allow(unused_variables)]
    fn on_lvar(&mut self, node: &Lvar) {}
    #[allow(unused_variables)]
    fn on_lvasgn(&mut self, node: &Lvasgn) {}
    #[allow(unused_variables)]
    fn on_masgn(&mut self, node: &Masgn) {}
    #[allow(unused_variables)]
    fn on_match_alt(&mut self, node: &MatchAlt) {}
    #[allow(unused_variables)]
    fn on_match_as(&mut self, node: &MatchAs) {}
    #[allow(unused_variables)]
    fn on_match_current_line(&mut self, node: &MatchCurrentLine) {}
    #[allow(unused_variables)]
    fn on_match_nil_pattern(&mut self, node: &MatchNilPattern) {}
    #[allow(unused_variables)]
    fn on_match_pattern(&mut self, node: &MatchPattern) {}
    #[allow(unused_variables)]
    fn on_match_pattern_p(&mut self, node: &MatchPatternP) {}
    #[allow(unused_variables)]
    fn on_match_rest(&mut self, node: &MatchRest) {}
    #[allow(unused_variables)]
    fn on_match_var(&mut self, node: &MatchVar) {}
    #[allow(unused_variables)]
    fn on_match_with_lvasgn(&mut self, node: &MatchWithLvasgn) {}
    #[allow(unused_variables)]
    fn on_mlhs(&mut self, node: &Mlhs) {}
    #[allow(unused_variables)]
    fn on_module(&mut self, node: &Module) {}
    #[allow(unused_variables)]
    fn on_next(&mut self, node: &Next) {}
    #[allow(unused_variables)]
    fn on_nil(&mut self, node: &Nil) {}
    #[allow(unused_variables)]
    fn on_nth_ref(&mut self, node: &NthRef) {}
    #[allow(unused_variables)]
    fn on_numblock(&mut self, node: &Numblock) {}
    #[allow(unused_variables)]
    fn on_op_asgn(&mut self, node: &OpAsgn) {}
    #[allow(unused_variables)]
    fn on_optarg(&mut self, node: &Optarg) {}
    #[allow(unused_variables)]
    fn on_or(&mut self, node: &Or) {}
    #[allow(unused_variables)]
    fn on_or_asgn(&mut self, node: &OrAsgn) {}
    #[allow(unused_variables)]
    fn on_pair(&mut self, node: &Pair) {}
    #[allow(unused_variables)]
    fn on_pin(&mut self, node: &Pin) {}
    #[allow(unused_variables)]
    fn on_postexe(&mut self, node: &Postexe) {}
    #[allow(unused_variables)]
    fn on_preexe(&mut self, node: &Preexe) {}
    #[allow(unused_variables)]
    fn on_procarg0(&mut self, node: &Procarg0) {}
    #[allow(unused_variables)]
    fn on_rational(&mut self, node: &Rational) {}
    #[allow(unused_variables)]
    fn on_redo(&mut self, node: &Redo) {}
    #[allow(unused_variables)]
    fn on_regexp(&mut self, node: &Regexp) {}
    #[allow(unused_variables)]
    fn on_regopt(&mut self, node: &RegOpt) {}
    #[allow(unused_variables)]
    fn on_rescue(&mut self, node: &Rescue) {}
    #[allow(unused_variables)]
    fn on_rescue_body(&mut self, node: &RescueBody) {}
    #[allow(unused_variables)]
    fn on_restarg(&mut self, node: &Restarg) {}
    #[allow(unused_variables)]
    fn on_retry(&mut self, node: &Retry) {}
    #[allow(unused_variables)]
    fn on_return(&mut self, node: &Return) {}
    #[allow(unused_variables)]
    fn on_sclass(&mut self, node: &SClass) {}
    #[allow(unused_variables)]
    fn on_self_(&mut self, node: &Self_) {}
    #[allow(unused_variables)]
    fn on_send(&mut self, node: &Send) {}
    #[allow(unused_variables)]
    fn on_shadowarg(&mut self, node: &Shadowarg) {}
    #[allow(unused_variables)]
    fn on_splat(&mut self, node: &Splat) {}
    #[allow(unused_variables)]
    fn on_str(&mut self, node: &Str) {}
    #[allow(unused_variables)]
    fn on_super(&mut self, node: &Super) {}
    #[allow(unused_variables)]
    fn on_sym(&mut self, node: &Sym) {}
    #[allow(unused_variables)]
    fn on_true(&mut self, node: &True) {}
    #[allow(unused_variables)]
    fn on_undef(&mut self, node: &Undef) {}
    #[allow(unused_variables)]
    fn on_unless_guard(&mut self, node: &UnlessGuard) {}
    #[allow(unused_variables)]
    fn on_until(&mut self, node: &Until) {}
    #[allow(unused_variables)]
    fn on_until_post(&mut self, node: &UntilPost) {}
    #[allow(unused_variables)]
    fn on_when(&mut self, node: &When) {}
    #[allow(unused_variables)]
    fn on_while(&mut self, node: &While) {}
    #[allow(unused_variables)]
    fn on_while_post(&mut self, node: &WhilePost) {}
    #[allow(unused_variables)]
    fn on_xheredoc(&mut self, node: &XHeredoc) {}
    #[allow(unused_variables)]
    fn on_xstr(&mut self, node: &Xstr) {}
    #[allow(unused_variables)]
    fn on_yield(&mut self, node: &Yield) {}
    #[allow(unused_variables)]
    fn on_zsuper(&mut self, node: &ZSuper) {}

    #[allow(unused_variables)]
    fn on_node(&mut self, node: &Node) {}
    #[allow(unused_variables)]
    fn on_node_moving_up(&mut self, node: &Node) {}

    #[allow(unused_variables)]
    fn on_option_node(&mut self, node: &Option<Box<Node>>) {}
    #[allow(unused_variables)]
    fn on_node_list(&mut self, nodes: &[Node]) {}

    #[allow(unused_variables)]
    fn on_subitem(&mut self, subitem: Item) {}
    #[allow(unused_variables)]
    fn on_subitem_moving_up(&mut self, subitem: Item) {}
}

pub struct Visitor<T>
where
    T: Observer,
{
    pub handler: T,
}

trait Visit<TItem> {
    fn visit(&mut self, item: TItem, visit_as: Item);
}

impl<TObserver: Observer> Visit<&[Node]> for Visitor<TObserver> {
    fn visit(&mut self, nodes: &[Node], visit_as: Item) {
        self.handler.on_subitem(visit_as);
        self.handler.on_node_list(nodes);

        for (idx, node) in nodes.iter().enumerate() {
            self.visit(node, Item::Idx(idx));
        }

        self.handler.on_subitem_moving_up(visit_as);
    }
}

impl<TObserver: Observer> Visit<&Vec<Node>> for Visitor<TObserver> {
    fn visit(&mut self, nodes: &Vec<Node>, visit_as: Item) {
        let nodes: &[Node] = nodes;
        self.visit(nodes, visit_as);
    }
}

impl<TObserver: Observer> Visit<&Node> for Visitor<TObserver> {
    fn visit(&mut self, node: &Node, visit_as: Item) {
        self.handler.on_subitem(visit_as);
        self.handler.on_node(node);

        match node {
            Node::Alias(inner) => self.visit_alias(inner),
            Node::And(inner) => self.visit_and(inner),
            Node::AndAsgn(inner) => self.visit_and_asgn(inner),
            Node::Arg(inner) => self.visit_arg(inner),
            Node::Args(inner) => self.visit_args(inner),
            Node::Array(inner) => self.visit_array(inner),
            Node::ArrayPattern(inner) => self.visit_array_pattern(inner),
            Node::ArrayPatternWithTail(inner) => self.visit_array_pattern_with_tail(inner),
            Node::BackRef(inner) => self.visit_back_ref(inner),
            Node::Begin(inner) => self.visit_begin(inner),
            Node::Block(inner) => self.visit_block(inner),
            Node::Blockarg(inner) => self.visit_blockarg(inner),
            Node::BlockPass(inner) => self.visit_block_pass(inner),
            Node::Break(inner) => self.visit_break(inner),
            Node::Case(inner) => self.visit_case(inner),
            Node::CaseMatch(inner) => self.visit_case_match(inner),
            Node::Casgn(inner) => self.visit_casgn(inner),
            Node::Cbase(inner) => self.visit_cbase(inner),
            Node::Class(inner) => self.visit_class(inner),
            Node::Complex(inner) => self.visit_complex(inner),
            Node::Const(inner) => self.visit_const(inner),
            Node::ConstPattern(inner) => self.visit_const_pattern(inner),
            Node::CSend(inner) => self.visit_csend(inner),
            Node::Cvar(inner) => self.visit_cvar(inner),
            Node::Cvasgn(inner) => self.visit_cvasgn(inner),
            Node::Def(inner) => self.visit_def(inner),
            Node::Defined(inner) => self.visit_defined(inner),
            Node::Defs(inner) => self.visit_defs(inner),
            Node::Dstr(inner) => self.visit_dstr(inner),
            Node::Dsym(inner) => self.visit_dsym(inner),
            Node::EFlipFlop(inner) => self.visit_eflipflop(inner),
            Node::EmptyElse(inner) => self.visit_empty_else(inner),
            Node::Encoding(inner) => self.visit_encoding(inner),
            Node::Ensure(inner) => self.visit_ensure(inner),
            Node::Erange(inner) => self.visit_erange(inner),
            Node::False(inner) => self.visit_false(inner),
            Node::File(inner) => self.visit_file(inner),
            Node::FindPattern(inner) => self.visit_find_pattern(inner),
            Node::Float(inner) => self.visit_float(inner),
            Node::For(inner) => self.visit_for(inner),
            Node::ForwardArg(inner) => self.visit_forward_arg(inner),
            Node::ForwardedArgs(inner) => self.visit_forwarded_args(inner),
            Node::Gvar(inner) => self.visit_gvar(inner),
            Node::Gvasgn(inner) => self.visit_gvasgn(inner),
            Node::Hash(inner) => self.visit_hash(inner),
            Node::HashPattern(inner) => self.visit_hash_pattern(inner),
            Node::Heredoc(inner) => self.visit_heredoc(inner),
            Node::If(inner) => self.visit_if(inner),
            Node::IfGuard(inner) => self.visit_if_guard(inner),
            Node::IFlipFlop(inner) => self.visit_iflipflop(inner),
            Node::IfMod(inner) => self.visit_if_mod(inner),
            Node::IfTernary(inner) => self.visit_if_ternary(inner),
            Node::Index(inner) => self.visit_index(inner),
            Node::IndexAsgn(inner) => self.visit_index_asgn(inner),
            Node::InPattern(inner) => self.visit_in_pattern(inner),
            Node::Int(inner) => self.visit_int(inner),
            Node::Irange(inner) => self.visit_irange(inner),
            Node::Ivar(inner) => self.visit_ivar(inner),
            Node::Ivasgn(inner) => self.visit_ivasgn(inner),
            Node::Kwarg(inner) => self.visit_kwarg(inner),
            Node::Kwargs(inner) => self.visit_kwargs(inner),
            Node::KwBegin(inner) => self.visit_kwbegin(inner),
            Node::Kwnilarg(inner) => self.visit_kwnilarg(inner),
            Node::Kwoptarg(inner) => self.visit_kwoptarg(inner),
            Node::Kwrestarg(inner) => self.visit_kwrestarg(inner),
            Node::Kwsplat(inner) => self.visit_kwsplat(inner),
            Node::Lambda(inner) => self.visit_lambda(inner),
            Node::Line(inner) => self.visit_line(inner),
            Node::Lvar(inner) => self.visit_lvar(inner),
            Node::Lvasgn(inner) => self.visit_lvasgn(inner),
            Node::Masgn(inner) => self.visit_masgn(inner),
            Node::MatchAlt(inner) => self.visit_match_alt(inner),
            Node::MatchAs(inner) => self.visit_match_as(inner),
            Node::MatchCurrentLine(inner) => self.visit_match_current_line(inner),
            Node::MatchNilPattern(inner) => self.visit_match_nil_pattern(inner),
            Node::MatchPattern(inner) => self.visit_match_pattern(inner),
            Node::MatchPatternP(inner) => self.visit_match_pattern_p(inner),
            Node::MatchRest(inner) => self.visit_match_rest(inner),
            Node::MatchVar(inner) => self.visit_match_var(inner),
            Node::MatchWithLvasgn(inner) => self.visit_match_with_lvasgn(inner),
            Node::Mlhs(inner) => self.visit_mlhs(inner),
            Node::Module(inner) => self.visit_module(inner),
            Node::Next(inner) => self.visit_next(inner),
            Node::Nil(inner) => self.visit_nil(inner),
            Node::NthRef(inner) => self.visit_nth_ref(inner),
            Node::Numblock(inner) => self.visit_numblock(inner),
            Node::OpAsgn(inner) => self.visit_op_asgn(inner),
            Node::Optarg(inner) => self.visit_optarg(inner),
            Node::Or(inner) => self.visit_or(inner),
            Node::OrAsgn(inner) => self.visit_or_asgn(inner),
            Node::Pair(inner) => self.visit_pair(inner),
            Node::Pin(inner) => self.visit_pin(inner),
            Node::Postexe(inner) => self.visit_postexe(inner),
            Node::Preexe(inner) => self.visit_preexe(inner),
            Node::Procarg0(inner) => self.visit_procarg0(inner),
            Node::Rational(inner) => self.visit_rational(inner),
            Node::Redo(inner) => self.visit_redo(inner),
            Node::Regexp(inner) => self.visit_regexp(inner),
            Node::RegOpt(inner) => self.visit_regopt(inner),
            Node::Rescue(inner) => self.visit_rescue(inner),
            Node::RescueBody(inner) => self.visit_rescue_body(inner),
            Node::Restarg(inner) => self.visit_restarg(inner),
            Node::Retry(inner) => self.visit_retry(inner),
            Node::Return(inner) => self.visit_return(inner),
            Node::SClass(inner) => self.visit_sclass(inner),
            Node::Self_(inner) => self.visit_self_(inner),
            Node::Send(inner) => self.visit_send(inner),
            Node::Shadowarg(inner) => self.visit_shadowarg(inner),
            Node::Splat(inner) => self.visit_splat(inner),
            Node::Str(inner) => self.visit_str(inner),
            Node::Super(inner) => self.visit_super(inner),
            Node::Sym(inner) => self.visit_sym(inner),
            Node::True(inner) => self.visit_true(inner),
            Node::Undef(inner) => self.visit_undef(inner),
            Node::UnlessGuard(inner) => self.visit_unless_guard(inner),
            Node::Until(inner) => self.visit_until(inner),
            Node::UntilPost(inner) => self.visit_until_post(inner),
            Node::When(inner) => self.visit_when(inner),
            Node::While(inner) => self.visit_while(inner),
            Node::WhilePost(inner) => self.visit_while_post(inner),
            Node::XHeredoc(inner) => self.visit_xheredoc(inner),
            Node::Xstr(inner) => self.visit_xstr(inner),
            Node::Yield(inner) => self.visit_yield(inner),
            Node::ZSuper(inner) => self.visit_zsuper(inner),
        }

        self.handler.on_node_moving_up(&node);
        self.handler.on_subitem_moving_up(visit_as);
    }
}

impl<TObserver: Observer> Visit<&Box<Node>> for Visitor<TObserver> {
    fn visit(&mut self, node: &Box<Node>, visit_as: Item) {
        let node: &Node = &*node;
        self.visit(node, visit_as);
    }
}

impl<TObserver: Observer> Visit<&Option<Box<Node>>> for Visitor<TObserver> {
    fn visit(&mut self, node: &Option<Box<Node>>, visit_as: Item) {
        if let Some(node) = node {
            self.visit(node, visit_as);
        }
    }
}

impl<T> Visitor<T>
where
    T: Observer,
{
    fn visit_alias(&mut self, node: &Alias) {
        self.handler.on_alias(node);

        self.visit(&node.to, Item::To);
        self.visit(&node.from, Item::From);
    }

    fn visit_and(&mut self, node: &And) {
        self.handler.on_and(node);

        self.visit(&node.lhs, Item::Lhs);
        self.visit(&node.rhs, Item::Rhs);
    }

    fn visit_and_asgn(&mut self, node: &AndAsgn) {
        self.handler.on_and_asgn(node);

        self.visit(&node.recv, Item::Recv);
        self.visit(&node.value, Item::Value);
    }

    fn visit_arg(&mut self, node: &Arg) {
        self.handler.on_arg(node);
    }

    fn visit_args(&mut self, node: &Args) {
        self.handler.on_args(node);

        self.visit(&node.args, Item::Arglist);
    }

    fn visit_array(&mut self, node: &Array) {
        self.handler.on_array(node);

        self.visit(&node.elements, Item::Elements);
    }

    fn visit_array_pattern(&mut self, node: &ArrayPattern) {
        self.handler.on_array_pattern(node);

        self.visit(&node.elements, Item::Elements);
    }

    fn visit_array_pattern_with_tail(&mut self, node: &ArrayPatternWithTail) {
        self.handler.on_array_pattern_with_tail(node);

        self.visit(&node.elements, Item::Elements);
    }

    fn visit_back_ref(&mut self, node: &BackRef) {
        self.handler.on_back_ref(node);
    }

    fn visit_begin(&mut self, node: &Begin) {
        self.handler.on_begin(node);

        self.visit(&node.statements, Item::Stmts);
    }

    fn visit_block(&mut self, node: &Block) {
        self.handler.on_block(node);

        self.visit(&node.call, Item::MethodCall);
        self.visit(&node.args, Item::Args);
        self.visit(&node.body, Item::Body);
    }

    fn visit_blockarg(&mut self, node: &Blockarg) {
        self.handler.on_blockarg(node);
    }

    fn visit_block_pass(&mut self, node: &BlockPass) {
        self.handler.on_block_pass(node);

        self.visit(&node.value, Item::Value);
    }

    fn visit_break(&mut self, node: &Break) {
        self.handler.on_break(node);

        self.visit(&node.args, Item::Args);
    }

    fn visit_case(&mut self, node: &Case) {
        self.handler.on_case(node);

        self.visit(&node.expr, Item::Expr);
        self.visit(&node.when_bodies, Item::WhenBodies);
        self.visit(&node.else_body, Item::ElseBody);
    }

    fn visit_case_match(&mut self, node: &CaseMatch) {
        self.handler.on_case_match(node);

        self.visit(&node.expr, Item::Expr);
        self.visit(&node.in_bodies, Item::InBodies);
        self.visit(&node.else_body, Item::ElseBody);
    }

    fn visit_casgn(&mut self, node: &Casgn) {
        self.handler.on_casgn(node);

        self.visit(&node.scope, Item::Scope);
        self.visit(&node.value, Item::Value);
    }

    fn visit_cbase(&mut self, node: &Cbase) {
        self.handler.on_cbase(node);
    }

    fn visit_class(&mut self, node: &Class) {
        self.handler.on_class(node);

        self.visit(&node.name, Item::Name);
        self.visit(&node.superclass, Item::Superclass);
        self.visit(&node.body, Item::Body);
    }

    fn visit_complex(&mut self, node: &Complex) {
        self.handler.on_complex(node);
    }

    fn visit_const(&mut self, node: &Const) {
        self.handler.on_const(node);

        self.visit(&node.scope, Item::Scope);
    }

    fn visit_const_pattern(&mut self, node: &ConstPattern) {
        self.handler.on_const_pattern(node);

        self.visit(&node.const_, Item::Const);
        self.visit(&node.pattern, Item::Pattern);
    }

    fn visit_csend(&mut self, node: &CSend) {
        self.handler.on_csend(node);

        self.visit(&node.recv, Item::Recv);
        self.visit(&node.args, Item::Args);
    }

    fn visit_cvar(&mut self, node: &Cvar) {
        self.handler.on_cvar(node);
    }

    fn visit_cvasgn(&mut self, node: &Cvasgn) {
        self.handler.on_cvasgn(node);

        self.visit(&node.value, Item::Value);
    }

    fn visit_def(&mut self, node: &Def) {
        self.handler.on_def(node);

        self.visit(&node.args, Item::Args);
        self.visit(&node.body, Item::Body);
    }

    fn visit_defined(&mut self, node: &Defined) {
        self.handler.on_defined(node);

        self.visit(&node.value, Item::Value);
    }

    fn visit_defs(&mut self, node: &Defs) {
        self.handler.on_defs(node);

        self.visit(&node.definee, Item::Definee);
        self.visit(&node.args, Item::Args);
        self.visit(&node.body, Item::Body);
    }

    fn visit_dstr(&mut self, node: &Dstr) {
        self.handler.on_dstr(node);

        self.visit(&node.parts, Item::Parts);
    }

    fn visit_dsym(&mut self, node: &Dsym) {
        self.handler.on_dsym(node);

        self.visit(&node.parts, Item::Parts);
    }

    fn visit_eflipflop(&mut self, node: &EFlipFlop) {
        self.handler.on_eflipflop(node);

        self.visit(&node.left, Item::Left);
        self.visit(&node.right, Item::Right);
    }

    fn visit_empty_else(&mut self, node: &EmptyElse) {
        self.handler.on_empty_else(node);
    }

    fn visit_encoding(&mut self, node: &Encoding) {
        self.handler.on_encoding(node);
    }

    fn visit_ensure(&mut self, node: &Ensure) {
        self.handler.on_ensure(node);

        self.visit(&node.body, Item::Body);
        self.visit(&node.ensure, Item::Ensure);
    }

    fn visit_erange(&mut self, node: &Erange) {
        self.handler.on_erange(node);

        self.visit(&node.left, Item::Left);
        self.visit(&node.right, Item::Right);
    }

    fn visit_false(&mut self, node: &False) {
        self.handler.on_false(node);
    }

    fn visit_file(&mut self, node: &File) {
        self.handler.on_file(node);
    }

    fn visit_find_pattern(&mut self, node: &FindPattern) {
        self.handler.on_find_pattern(node);

        self.visit(&node.elements, Item::Elements);
    }

    fn visit_float(&mut self, node: &Float) {
        self.handler.on_float(node);
    }

    fn visit_for(&mut self, node: &For) {
        self.handler.on_for(node);

        self.visit(&node.iterator, Item::Iterator);
        self.visit(&node.iteratee, Item::Iteratee);
        self.visit(&node.body, Item::Body);
    }

    fn visit_forward_arg(&mut self, node: &ForwardArg) {
        self.handler.on_forward_arg(node);
    }

    fn visit_forwarded_args(&mut self, node: &ForwardedArgs) {
        self.handler.on_forwarded_args(node);
    }

    fn visit_gvar(&mut self, node: &Gvar) {
        self.handler.on_gvar(node);
    }

    fn visit_gvasgn(&mut self, node: &Gvasgn) {
        self.handler.on_gvasgn(node);

        self.visit(&node.value, Item::Value);
    }

    fn visit_hash(&mut self, node: &Hash) {
        self.handler.on_hash(node);

        self.visit(&node.pairs, Item::Pairs);
    }

    fn visit_hash_pattern(&mut self, node: &HashPattern) {
        self.handler.on_hash_pattern(node);

        self.visit(&node.elements, Item::Elements);
    }

    fn visit_heredoc(&mut self, node: &Heredoc) {
        self.handler.on_heredoc(node);

        self.visit(&node.parts, Item::Parts);
    }

    fn visit_if(&mut self, node: &If) {
        self.handler.on_if(node);

        self.visit(&node.cond, Item::Cond);
        self.visit(&node.if_true, Item::IfTrue);
        self.visit(&node.if_false, Item::IfFalse);
    }

    fn visit_if_guard(&mut self, node: &IfGuard) {
        self.handler.on_if_guard(node);

        self.visit(&node.cond, Item::Cond);
    }

    fn visit_iflipflop(&mut self, node: &IFlipFlop) {
        self.handler.on_iflipflop(node);

        self.visit(&node.left, Item::Left);
        self.visit(&node.right, Item::Right);
    }

    fn visit_if_mod(&mut self, node: &IfMod) {
        self.handler.on_if_mod(node);

        self.visit(&node.cond, Item::Cond);
        self.visit(&node.if_true, Item::IfTrue);
        self.visit(&node.if_false, Item::IfFalse);
    }

    fn visit_if_ternary(&mut self, node: &IfTernary) {
        self.handler.on_if_ternary(node);

        self.visit(&node.cond, Item::Cond);
        self.visit(&node.if_true, Item::IfTrue);
        self.visit(&node.if_false, Item::IfFalse);
    }

    fn visit_index(&mut self, node: &Index) {
        self.handler.on_index(node);

        self.visit(&node.recv, Item::Recv);
        self.visit(&node.indexes, Item::Indexes);
    }

    fn visit_index_asgn(&mut self, node: &IndexAsgn) {
        self.handler.on_index_asgn(node);

        self.visit(&node.recv, Item::Recv);
        self.visit(&node.indexes, Item::Indexes);
        self.visit(&node.value, Item::Value);
    }

    fn visit_in_pattern(&mut self, node: &InPattern) {
        self.handler.on_in_pattern(node);

        self.visit(&node.pattern, Item::Pattern);
        self.visit(&node.guard, Item::Guard);
        self.visit(&node.body, Item::Body);
    }

    fn visit_int(&mut self, node: &Int) {
        self.handler.on_int(node);
    }

    fn visit_irange(&mut self, node: &Irange) {
        self.handler.on_irange(node);

        self.visit(&node.left, Item::Left);
        self.visit(&node.right, Item::Right);
    }

    fn visit_ivar(&mut self, node: &Ivar) {
        self.handler.on_ivar(node);
    }

    fn visit_ivasgn(&mut self, node: &Ivasgn) {
        self.handler.on_ivasgn(node);

        self.visit(&node.value, Item::Value);
    }

    fn visit_kwarg(&mut self, node: &Kwarg) {
        self.handler.on_kwarg(node);
    }

    fn visit_kwargs(&mut self, node: &Kwargs) {
        self.handler.on_kwargs(node);

        self.visit(&node.pairs, Item::Pairs);
    }

    fn visit_kwbegin(&mut self, node: &KwBegin) {
        self.handler.on_kwbegin(node);

        self.visit(&node.statements, Item::Stmts);
    }

    fn visit_kwnilarg(&mut self, node: &Kwnilarg) {
        self.handler.on_kwnilarg(node);
    }

    fn visit_kwoptarg(&mut self, node: &Kwoptarg) {
        self.handler.on_kwoptarg(node);

        self.visit(&node.default, Item::DefaultValue);
    }

    fn visit_kwrestarg(&mut self, node: &Kwrestarg) {
        self.handler.on_kwrestarg(node);
    }

    fn visit_kwsplat(&mut self, node: &Kwsplat) {
        self.handler.on_kwsplat(node);

        self.visit(&node.value, Item::Value);
    }

    fn visit_lambda(&mut self, node: &Lambda) {
        self.handler.on_lambda(node);
    }

    fn visit_line(&mut self, node: &Line) {
        self.handler.on_line(node);
    }

    fn visit_lvar(&mut self, node: &Lvar) {
        self.handler.on_lvar(node);
    }

    fn visit_lvasgn(&mut self, node: &Lvasgn) {
        self.handler.on_lvasgn(node);

        self.visit(&node.value, Item::Value);
    }

    fn visit_masgn(&mut self, node: &Masgn) {
        self.handler.on_masgn(node);

        self.visit(&node.lhs, Item::Lhs);
        self.visit(&node.rhs, Item::Rhs);
    }

    fn visit_match_alt(&mut self, node: &MatchAlt) {
        self.handler.on_match_alt(node);

        self.visit(&node.lhs, Item::Lhs);
        self.visit(&node.rhs, Item::Rhs);
    }

    fn visit_match_as(&mut self, node: &MatchAs) {
        self.handler.on_match_as(node);

        self.visit(&node.value, Item::Value);
        self.visit(&node.as_, Item::As);
    }

    fn visit_match_current_line(&mut self, node: &MatchCurrentLine) {
        self.handler.on_match_current_line(node);

        self.visit(&node.re, Item::Re);
    }

    fn visit_match_nil_pattern(&mut self, node: &MatchNilPattern) {
        self.handler.on_match_nil_pattern(node);
    }

    fn visit_match_pattern(&mut self, node: &MatchPattern) {
        self.handler.on_match_pattern(node);

        self.visit(&node.value, Item::Value);
        self.visit(&node.pattern, Item::Pattern);
    }

    fn visit_match_pattern_p(&mut self, node: &MatchPatternP) {
        self.handler.on_match_pattern_p(node);

        self.visit(&node.value, Item::Value);
        self.visit(&node.pattern, Item::Pattern);
    }

    fn visit_match_rest(&mut self, node: &MatchRest) {
        self.handler.on_match_rest(node);

        self.visit(&node.name, Item::Name);
    }

    fn visit_match_var(&mut self, node: &MatchVar) {
        self.handler.on_match_var(node);
    }

    fn visit_match_with_lvasgn(&mut self, node: &MatchWithLvasgn) {
        self.handler.on_match_with_lvasgn(node);

        self.visit(&node.re, Item::Re);
        self.visit(&node.value, Item::Value);
    }

    fn visit_mlhs(&mut self, node: &Mlhs) {
        self.handler.on_mlhs(node);

        self.visit(&node.items, Item::MlhsItems);
    }
    fn visit_module(&mut self, node: &Module) {
        self.handler.on_module(node);

        self.visit(&node.name, Item::Name);
        self.visit(&node.body, Item::Body);
    }

    fn visit_next(&mut self, node: &Next) {
        self.handler.on_next(node);

        self.visit(&node.args, Item::Args);
    }

    fn visit_nil(&mut self, node: &Nil) {
        self.handler.on_nil(node);
    }

    fn visit_nth_ref(&mut self, node: &NthRef) {
        self.handler.on_nth_ref(node);
    }

    fn visit_numblock(&mut self, node: &Numblock) {
        self.handler.on_numblock(node);

        self.visit(&node.call, Item::MethodCall);
        self.visit(&node.body, Item::Body);
    }

    fn visit_op_asgn(&mut self, node: &OpAsgn) {
        self.handler.on_op_asgn(node);

        self.visit(&node.recv, Item::Recv);
        self.visit(&node.value, Item::Value);
    }

    fn visit_optarg(&mut self, node: &Optarg) {
        self.handler.on_optarg(node);

        self.visit(&node.default, Item::DefaultValue);
    }

    fn visit_or(&mut self, node: &Or) {
        self.handler.on_or(node);

        self.visit(&node.lhs, Item::Lhs);
        self.visit(&node.rhs, Item::Rhs);
    }

    fn visit_or_asgn(&mut self, node: &OrAsgn) {
        self.handler.on_or_asgn(node);

        self.visit(&node.recv, Item::Recv);
        self.visit(&node.value, Item::Value);
    }

    fn visit_pair(&mut self, node: &Pair) {
        self.handler.on_pair(node);

        self.visit(&node.key, Item::Key);
        self.visit(&node.value, Item::Value);
    }

    fn visit_pin(&mut self, node: &Pin) {
        self.handler.on_pin(node);

        self.visit(&node.var, Item::Var);
    }

    fn visit_postexe(&mut self, node: &Postexe) {
        self.handler.on_postexe(node);

        self.visit(&node.body, Item::Body);
    }

    fn visit_preexe(&mut self, node: &Preexe) {
        self.handler.on_preexe(node);

        self.visit(&node.body, Item::Body);
    }

    fn visit_procarg0(&mut self, node: &Procarg0) {
        self.handler.on_procarg0(node);

        self.visit(&node.args, Item::Arglist);
    }

    fn visit_rational(&mut self, node: &Rational) {
        self.handler.on_rational(node);
    }

    fn visit_redo(&mut self, node: &Redo) {
        self.handler.on_redo(node);
    }

    fn visit_regexp(&mut self, node: &Regexp) {
        self.handler.on_regexp(node);

        self.visit(&node.parts, Item::Parts);
        self.visit(&node.options, Item::Options);
    }

    fn visit_regopt(&mut self, node: &RegOpt) {
        self.handler.on_regopt(node);
    }

    fn visit_rescue(&mut self, node: &Rescue) {
        self.handler.on_rescue(node);

        self.visit(&node.body, Item::Body);
        self.visit(&node.rescue_bodies, Item::RescueBodies);
        self.visit(&node.else_, Item::ElseBody);
    }

    fn visit_rescue_body(&mut self, node: &RescueBody) {
        self.handler.on_rescue_body(node);

        self.visit(&node.exc_list, Item::ExcList);
        self.visit(&node.exc_var, Item::ExcVar);
        self.visit(&node.body, Item::Body);
    }

    fn visit_restarg(&mut self, node: &Restarg) {
        self.handler.on_restarg(node);
    }

    fn visit_retry(&mut self, node: &Retry) {
        self.handler.on_retry(node);
    }

    fn visit_return(&mut self, node: &Return) {
        self.handler.on_return(node);

        self.visit(&node.args, Item::Args);
    }

    fn visit_sclass(&mut self, node: &SClass) {
        self.handler.on_sclass(node);

        self.visit(&node.expr, Item::Expr);
        self.visit(&node.body, Item::Body);
    }

    fn visit_self_(&mut self, node: &Self_) {
        self.handler.on_self_(node);
    }

    fn visit_send(&mut self, node: &Send) {
        self.handler.on_send(node);

        self.visit(&node.recv, Item::Recv);
        self.visit(&node.args, Item::Args);
    }

    fn visit_shadowarg(&mut self, node: &Shadowarg) {
        self.handler.on_shadowarg(node);
    }

    fn visit_splat(&mut self, node: &Splat) {
        self.handler.on_splat(node);

        self.visit(&node.value, Item::Value);
    }

    fn visit_str(&mut self, node: &Str) {
        self.handler.on_str(node);
    }

    fn visit_super(&mut self, node: &Super) {
        self.handler.on_super(node);

        self.visit(&node.args, Item::Args);
    }

    fn visit_sym(&mut self, node: &Sym) {
        self.handler.on_sym(node);
    }

    fn visit_true(&mut self, node: &True) {
        self.handler.on_true(node);
    }

    fn visit_undef(&mut self, node: &Undef) {
        self.handler.on_undef(node);

        self.visit(&node.names, Item::Args);
    }

    fn visit_unless_guard(&mut self, node: &UnlessGuard) {
        self.handler.on_unless_guard(node);

        self.visit(&node.cond, Item::Cond);
    }

    fn visit_until(&mut self, node: &Until) {
        self.handler.on_until(node);

        self.visit(&node.cond, Item::Cond);
        self.visit(&node.body, Item::Body);
    }

    fn visit_until_post(&mut self, node: &UntilPost) {
        self.handler.on_until_post(node);

        self.visit(&node.cond, Item::Cond);
        self.visit(&node.body, Item::Body);
    }

    fn visit_when(&mut self, node: &When) {
        self.handler.on_when(node);

        self.visit(&node.patterns, Item::Args);
        self.visit(&node.body, Item::Body);
    }

    fn visit_while(&mut self, node: &While) {
        self.handler.on_while(node);

        self.visit(&node.cond, Item::Cond);
        self.visit(&node.body, Item::Body);
    }

    fn visit_while_post(&mut self, node: &WhilePost) {
        self.handler.on_while_post(node);

        self.visit(&node.cond, Item::Cond);
        self.visit(&node.body, Item::Body);
    }

    fn visit_xheredoc(&mut self, node: &XHeredoc) {
        self.handler.on_xheredoc(node);

        self.visit(&node.parts, Item::Parts);
    }

    fn visit_xstr(&mut self, node: &Xstr) {
        self.handler.on_xstr(node);

        self.visit(&node.parts, Item::Parts);
    }

    fn visit_yield(&mut self, node: &Yield) {
        self.handler.on_yield(node);

        self.visit(&node.args, Item::Args);
    }

    fn visit_zsuper(&mut self, node: &ZSuper) {
        self.handler.on_zsuper(node);
    }

    pub fn visit_root(&mut self, node: &Node) {
        self.visit(node, Item::Root);
    }
}
