use crate::nodes::*;
use crate::Node;

pub trait Visitor<T: Default = ()> {
    fn visit_all(&mut self, nodes: &Vec<Node>) -> T {
        for node in nodes {
            self.visit(node);
        }
        T::default()
    }

    fn maybe_visit(&mut self, node: &Option<Box<Node>>) -> T {
        if let Some(node) = node {
            self.visit(node);
        }
        T::default()
    }

    fn on_alias(&mut self, node: &Alias) -> T {
        self.visit(&node.to);
        self.visit(&node.from)
    }

    fn on_and(&mut self, node: &And) -> T {
        self.visit(&node.lhs);
        self.visit(&node.rhs)
    }

    fn on_and_asgn(&mut self, node: &AndAsgn) -> T {
        self.visit(&node.recv);
        self.visit(&node.value)
    }

    #[allow(unused_variables)]
    fn on_arg(&mut self, node: &Arg) -> T {
        T::default()
    }

    fn on_args(&mut self, node: &Args) -> T {
        self.visit_all(&node.args)
    }

    fn on_array(&mut self, node: &Array) -> T {
        self.visit_all(&node.elements)
    }

    fn on_array_pattern(&mut self, node: &ArrayPattern) -> T {
        self.visit_all(&node.elements)
    }

    fn on_array_pattern_with_tail(&mut self, node: &ArrayPatternWithTail) -> T {
        self.visit_all(&node.elements)
    }

    #[allow(unused_variables)]
    fn on_back_ref(&mut self, node: &BackRef) -> T {
        T::default()
    }

    fn on_begin(&mut self, node: &Begin) -> T {
        self.visit_all(&node.statements)
    }

    fn on_block(&mut self, node: &Block) -> T {
        self.visit(&node.call);
        self.maybe_visit(&node.args);
        self.maybe_visit(&node.body)
    }

    #[allow(unused_variables)]
    fn on_blockarg(&mut self, node: &Blockarg) -> T {
        T::default()
    }

    fn on_block_pass(&mut self, node: &BlockPass) -> T {
        self.visit(&node.value)
    }

    fn on_break(&mut self, node: &Break) -> T {
        self.visit_all(&node.args)
    }

    fn on_case(&mut self, node: &Case) -> T {
        self.maybe_visit(&node.expr);
        self.visit_all(&node.when_bodies);
        self.maybe_visit(&node.else_body)
    }

    fn on_case_match(&mut self, node: &CaseMatch) -> T {
        self.visit(&node.expr);
        self.visit_all(&node.in_bodies);
        self.maybe_visit(&node.else_body)
    }

    fn on_casgn(&mut self, node: &Casgn) -> T {
        self.maybe_visit(&node.scope);
        self.maybe_visit(&node.value)
    }

    #[allow(unused_variables)]
    fn on_cbase(&mut self, node: &Cbase) -> T {
        T::default()
    }

    fn on_class(&mut self, node: &Class) -> T {
        self.visit(&node.name);
        self.maybe_visit(&node.superclass);
        self.maybe_visit(&node.body)
    }

    #[allow(unused_variables)]
    fn on_complex(&mut self, node: &Complex) -> T {
        T::default()
    }

    fn on_const(&mut self, node: &Const) -> T {
        self.maybe_visit(&node.scope)
    }

    fn on_const_pattern(&mut self, node: &ConstPattern) -> T {
        self.visit(&node.const_);
        self.visit(&node.pattern)
    }

    fn on_csend(&mut self, node: &CSend) -> T {
        self.visit(&node.recv);
        self.visit_all(&node.args)
    }

    #[allow(unused_variables)]
    fn on_cvar(&mut self, node: &Cvar) -> T {
        T::default()
    }

    fn on_cvasgn(&mut self, node: &Cvasgn) -> T {
        self.maybe_visit(&node.value)
    }

    fn on_def(&mut self, node: &Def) -> T {
        self.maybe_visit(&node.args);
        self.maybe_visit(&node.body)
    }

    fn on_defined(&mut self, node: &Defined) -> T {
        self.visit(&node.value)
    }

    fn on_defs(&mut self, node: &Defs) -> T {
        self.visit(&node.definee);
        self.maybe_visit(&node.args);
        self.maybe_visit(&node.body)
    }

    fn on_dstr(&mut self, node: &Dstr) -> T {
        self.visit_all(&node.parts)
    }

    fn on_dsym(&mut self, node: &Dsym) -> T {
        self.visit_all(&node.parts)
    }

    fn on_eflipflop(&mut self, node: &EFlipFlop) -> T {
        self.maybe_visit(&node.left);
        self.maybe_visit(&node.right)
    }

    #[allow(unused_variables)]
    fn on_empty_else(&mut self, node: &EmptyElse) -> T {
        T::default()
    }

    #[allow(unused_variables)]
    fn on_encoding(&mut self, node: &Encoding) -> T {
        T::default()
    }

    fn on_ensure(&mut self, node: &Ensure) -> T {
        self.maybe_visit(&node.body);
        self.maybe_visit(&node.ensure)
    }

    fn on_erange(&mut self, node: &Erange) -> T {
        self.maybe_visit(&node.left);
        self.maybe_visit(&node.right)
    }

    #[allow(unused_variables)]
    fn on_false(&mut self, node: &False) -> T {
        T::default()
    }

    #[allow(unused_variables)]
    fn on_file(&mut self, node: &File) -> T {
        T::default()
    }

    fn on_find_pattern(&mut self, node: &FindPattern) -> T {
        self.visit_all(&node.elements)
    }

    #[allow(unused_variables)]
    fn on_float(&mut self, node: &Float) -> T {
        T::default()
    }

    fn on_for(&mut self, node: &For) -> T {
        self.visit(&node.iterator);
        self.visit(&node.iteratee);
        self.maybe_visit(&node.body)
    }

    #[allow(unused_variables)]
    fn on_forward_arg(&mut self, node: &ForwardArg) -> T {
        T::default()
    }

    #[allow(unused_variables)]
    fn on_forwarded_args(&mut self, node: &ForwardedArgs) -> T {
        T::default()
    }

    #[allow(unused_variables)]
    fn on_gvar(&mut self, node: &Gvar) -> T {
        T::default()
    }

    fn on_gvasgn(&mut self, node: &Gvasgn) -> T {
        self.maybe_visit(&node.value)
    }

    fn on_hash(&mut self, node: &Hash) -> T {
        self.visit_all(&node.pairs)
    }

    fn on_hash_pattern(&mut self, node: &HashPattern) -> T {
        self.visit_all(&node.elements)
    }

    fn on_heredoc(&mut self, node: &Heredoc) -> T {
        self.visit_all(&node.parts)
    }

    fn on_if(&mut self, node: &If) -> T {
        self.visit(&node.cond);
        self.maybe_visit(&node.if_true);
        self.maybe_visit(&node.if_false)
    }

    fn on_if_guard(&mut self, node: &IfGuard) -> T {
        self.visit(&node.cond)
    }

    fn on_iflipflop(&mut self, node: &IFlipFlop) -> T {
        self.maybe_visit(&node.left);
        self.maybe_visit(&node.right)
    }

    fn on_if_mod(&mut self, node: &IfMod) -> T {
        self.visit(&node.cond);
        self.maybe_visit(&node.if_true);
        self.maybe_visit(&node.if_false)
    }

    fn on_if_ternary(&mut self, node: &IfTernary) -> T {
        self.visit(&node.cond);
        self.visit(&node.if_true);
        self.visit(&node.if_false)
    }

    fn on_index(&mut self, node: &Index) -> T {
        self.visit(&node.recv);
        self.visit_all(&node.indexes)
    }

    fn on_index_asgn(&mut self, node: &IndexAsgn) -> T {
        self.visit(&node.recv);
        self.visit_all(&node.indexes);
        self.maybe_visit(&node.value)
    }

    fn on_in_match(&mut self, node: &InMatch) -> T {
        self.visit(&node.value);
        self.visit(&node.pattern)
    }

    fn on_in_pattern(&mut self, node: &InPattern) -> T {
        self.visit(&node.pattern);
        self.maybe_visit(&node.guard);
        self.maybe_visit(&node.body)
    }

    #[allow(unused_variables)]
    fn on_int(&mut self, node: &Int) -> T {
        T::default()
    }

    fn on_irange(&mut self, node: &Irange) -> T {
        self.maybe_visit(&node.left);
        self.maybe_visit(&node.right)
    }

    #[allow(unused_variables)]
    fn on_ivar(&mut self, node: &Ivar) -> T {
        T::default()
    }

    fn on_ivasgn(&mut self, node: &Ivasgn) -> T {
        self.maybe_visit(&node.value)
    }

    #[allow(unused_variables)]
    fn on_kwarg(&mut self, node: &Kwarg) -> T {
        T::default()
    }

    fn on_kwbegin(&mut self, node: &KwBegin) -> T {
        self.visit_all(&node.statements)
    }

    #[allow(unused_variables)]
    fn on_kwnilarg(&mut self, node: &Kwnilarg) -> T {
        T::default()
    }

    fn on_kwoptarg(&mut self, node: &Kwoptarg) -> T {
        self.visit(&node.default)
    }

    #[allow(unused_variables)]
    fn on_kwrestarg(&mut self, node: &Kwrestarg) -> T {
        T::default()
    }

    fn on_kwsplat(&mut self, node: &Kwsplat) -> T {
        self.visit(&node.value)
    }

    #[allow(unused_variables)]
    fn on_lambda(&mut self, node: &Lambda) -> T {
        T::default()
    }

    #[allow(unused_variables)]
    fn on_line(&mut self, node: &Line) -> T {
        T::default()
    }

    #[allow(unused_variables)]
    fn on_lvar(&mut self, node: &Lvar) -> T {
        T::default()
    }

    fn on_lvasgn(&mut self, node: &Lvasgn) -> T {
        self.maybe_visit(&node.value)
    }

    fn on_masgn(&mut self, node: &Masgn) -> T {
        self.visit(&node.lhs);
        self.visit(&node.rhs)
    }

    fn on_match_alt(&mut self, node: &MatchAlt) -> T {
        self.visit(&node.lhs);
        self.visit(&node.rhs)
    }

    fn on_match_as(&mut self, node: &MatchAs) -> T {
        self.visit(&node.value);
        self.visit(&node.as_)
    }

    fn on_match_current_line(&mut self, node: &MatchCurrentLine) -> T {
        self.visit(&node.re)
    }

    #[allow(unused_variables)]
    fn on_match_nil_pattern(&mut self, node: &MatchNilPattern) -> T {
        T::default()
    }

    fn on_match_rest(&mut self, node: &MatchRest) -> T {
        self.maybe_visit(&node.name)
    }

    #[allow(unused_variables)]
    fn on_match_var(&mut self, node: &MatchVar) -> T {
        T::default()
    }

    fn on_match_with_lvasgn(&mut self, node: &MatchWithLvasgn) -> T {
        self.visit(&node.re);
        self.visit(&node.value)
    }

    fn on_match_with_trailing_comma(&mut self, node: &MatchWithTrailingComma) -> T {
        self.visit(&node.match_)
    }

    fn on_mlhs(&mut self, node: &Mlhs) -> T {
        self.visit_all(&node.items)
    }
    fn on_module(&mut self, node: &Module) -> T {
        self.visit(&node.name);
        self.maybe_visit(&node.body)
    }

    fn on_next(&mut self, node: &Next) -> T {
        self.visit_all(&node.args)
    }

    #[allow(unused_variables)]
    fn on_nil(&mut self, node: &Nil) -> T {
        T::default()
    }

    #[allow(unused_variables)]
    fn on_nth_ref(&mut self, node: &NthRef) -> T {
        T::default()
    }

    fn on_numblock(&mut self, node: &Numblock) -> T {
        self.visit(&node.call);
        self.visit(&node.body)
    }

    fn on_op_asgn(&mut self, node: &OpAsgn) -> T {
        self.visit(&node.recv);
        self.visit(&node.value)
    }

    fn on_optarg(&mut self, node: &Optarg) -> T {
        self.visit(&node.default)
    }

    fn on_or(&mut self, node: &Or) -> T {
        self.visit(&node.lhs);
        self.visit(&node.rhs)
    }

    fn on_or_asgn(&mut self, node: &OrAsgn) -> T {
        self.visit(&node.recv);
        self.visit(&node.value)
    }

    fn on_pair(&mut self, node: &Pair) -> T {
        self.visit(&node.key);
        self.visit(&node.value)
    }

    fn on_pin(&mut self, node: &Pin) -> T {
        self.visit(&node.var)
    }

    fn on_postexe(&mut self, node: &Postexe) -> T {
        self.maybe_visit(&node.body)
    }

    fn on_preexe(&mut self, node: &Preexe) -> T {
        self.maybe_visit(&node.body)
    }

    fn on_procarg0(&mut self, node: &Procarg0) -> T {
        self.visit_all(&node.args)
    }

    #[allow(unused_variables)]
    fn on_rational(&mut self, node: &Rational) -> T {
        T::default()
    }

    #[allow(unused_variables)]
    fn on_redo(&mut self, node: &Redo) -> T {
        T::default()
    }

    fn on_regexp(&mut self, node: &Regexp) -> T {
        self.visit_all(&node.parts);
        self.maybe_visit(&node.options)
    }

    #[allow(unused_variables)]
    fn on_regopt(&mut self, node: &RegOpt) -> T {
        T::default()
    }

    fn on_rescue(&mut self, node: &Rescue) -> T {
        self.maybe_visit(&node.body);
        self.visit_all(&node.rescue_bodies);
        self.maybe_visit(&node.else_)
    }

    fn on_rescue_body(&mut self, node: &RescueBody) -> T {
        self.maybe_visit(&node.exc_list);
        self.maybe_visit(&node.exc_var);
        self.maybe_visit(&node.body)
    }

    #[allow(unused_variables)]
    fn on_restarg(&mut self, node: &Restarg) -> T {
        T::default()
    }

    #[allow(unused_variables)]
    fn on_retry(&mut self, node: &Retry) -> T {
        T::default()
    }

    fn on_return(&mut self, node: &Return) -> T {
        self.visit_all(&node.args)
    }

    fn on_sclass(&mut self, node: &SClass) -> T {
        self.visit(&node.expr);
        self.maybe_visit(&node.body)
    }

    #[allow(unused_variables)]
    fn on_self_(&mut self, node: &Self_) -> T {
        T::default()
    }

    fn on_send(&mut self, node: &Send) -> T {
        self.maybe_visit(&node.recv);
        self.visit_all(&node.args)
    }

    #[allow(unused_variables)]
    fn on_shadowarg(&mut self, node: &Shadowarg) -> T {
        T::default()
    }

    fn on_splat(&mut self, node: &Splat) -> T {
        self.maybe_visit(&node.value)
    }

    #[allow(unused_variables)]
    fn on_str(&mut self, node: &Str) -> T {
        T::default()
    }

    fn on_super(&mut self, node: &Super) -> T {
        self.visit_all(&node.args)
    }

    #[allow(unused_variables)]
    fn on_sym(&mut self, node: &Sym) -> T {
        T::default()
    }

    #[allow(unused_variables)]
    fn on_true(&mut self, node: &True) -> T {
        T::default()
    }

    fn on_undef(&mut self, node: &Undef) -> T {
        self.visit_all(&node.names)
    }

    fn on_unless_guard(&mut self, node: &UnlessGuard) -> T {
        self.visit(&node.cond)
    }

    fn on_until(&mut self, node: &Until) -> T {
        self.visit(&node.cond);
        self.maybe_visit(&node.body)
    }

    fn on_until_post(&mut self, node: &UntilPost) -> T {
        self.visit(&node.cond);
        self.visit(&node.body)
    }

    fn on_when(&mut self, node: &When) -> T {
        self.visit_all(&node.patterns);
        self.maybe_visit(&node.body)
    }

    fn on_while(&mut self, node: &While) -> T {
        self.visit(&node.cond);
        self.maybe_visit(&node.body)
    }

    fn on_while_post(&mut self, node: &WhilePost) -> T {
        self.visit(&node.cond);
        self.visit(&node.body)
    }

    fn on_xheredoc(&mut self, node: &XHeredoc) -> T {
        self.visit_all(&node.parts)
    }

    fn on_xstr(&mut self, node: &Xstr) -> T {
        self.visit_all(&node.parts)
    }

    fn on_yield(&mut self, node: &Yield) -> T {
        self.visit_all(&node.args)
    }

    #[allow(unused_variables)]
    fn on_zsuper(&mut self, node: &ZSuper) -> T {
        T::default()
    }

    fn visit(&mut self, node: &Node) -> T {
        match node {
            Node::Alias(inner) => self.on_alias(inner),
            Node::And(inner) => self.on_and(inner),
            Node::AndAsgn(inner) => self.on_and_asgn(inner),
            Node::Arg(inner) => self.on_arg(inner),
            Node::Args(inner) => self.on_args(inner),
            Node::Array(inner) => self.on_array(inner),
            Node::ArrayPattern(inner) => self.on_array_pattern(inner),
            Node::ArrayPatternWithTail(inner) => self.on_array_pattern_with_tail(inner),
            Node::BackRef(inner) => self.on_back_ref(inner),
            Node::Begin(inner) => self.on_begin(inner),
            Node::Block(inner) => self.on_block(inner),
            Node::Blockarg(inner) => self.on_blockarg(inner),
            Node::BlockPass(inner) => self.on_block_pass(inner),
            Node::Break(inner) => self.on_break(inner),
            Node::Case(inner) => self.on_case(inner),
            Node::CaseMatch(inner) => self.on_case_match(inner),
            Node::Casgn(inner) => self.on_casgn(inner),
            Node::Cbase(inner) => self.on_cbase(inner),
            Node::Class(inner) => self.on_class(inner),
            Node::Complex(inner) => self.on_complex(inner),
            Node::Const(inner) => self.on_const(inner),
            Node::ConstPattern(inner) => self.on_const_pattern(inner),
            Node::CSend(inner) => self.on_csend(inner),
            Node::Cvar(inner) => self.on_cvar(inner),
            Node::Cvasgn(inner) => self.on_cvasgn(inner),
            Node::Def(inner) => self.on_def(inner),
            Node::Defined(inner) => self.on_defined(inner),
            Node::Defs(inner) => self.on_defs(inner),
            Node::Dstr(inner) => self.on_dstr(inner),
            Node::Dsym(inner) => self.on_dsym(inner),
            Node::EFlipFlop(inner) => self.on_eflipflop(inner),
            Node::EmptyElse(inner) => self.on_empty_else(inner),
            Node::Encoding(inner) => self.on_encoding(inner),
            Node::Ensure(inner) => self.on_ensure(inner),
            Node::Erange(inner) => self.on_erange(inner),
            Node::False(inner) => self.on_false(inner),
            Node::File(inner) => self.on_file(inner),
            Node::FindPattern(inner) => self.on_find_pattern(inner),
            Node::Float(inner) => self.on_float(inner),
            Node::For(inner) => self.on_for(inner),
            Node::ForwardArg(inner) => self.on_forward_arg(inner),
            Node::ForwardedArgs(inner) => self.on_forwarded_args(inner),
            Node::Gvar(inner) => self.on_gvar(inner),
            Node::Gvasgn(inner) => self.on_gvasgn(inner),
            Node::Hash(inner) => self.on_hash(inner),
            Node::HashPattern(inner) => self.on_hash_pattern(inner),
            Node::Heredoc(inner) => self.on_heredoc(inner),
            Node::If(inner) => self.on_if(inner),
            Node::IfGuard(inner) => self.on_if_guard(inner),
            Node::IFlipFlop(inner) => self.on_iflipflop(inner),
            Node::IfMod(inner) => self.on_if_mod(inner),
            Node::IfTernary(inner) => self.on_if_ternary(inner),
            Node::Index(inner) => self.on_index(inner),
            Node::IndexAsgn(inner) => self.on_index_asgn(inner),
            Node::InMatch(inner) => self.on_in_match(inner),
            Node::InPattern(inner) => self.on_in_pattern(inner),
            Node::Int(inner) => self.on_int(inner),
            Node::Irange(inner) => self.on_irange(inner),
            Node::Ivar(inner) => self.on_ivar(inner),
            Node::Ivasgn(inner) => self.on_ivasgn(inner),
            Node::Kwarg(inner) => self.on_kwarg(inner),
            Node::KwBegin(inner) => self.on_kwbegin(inner),
            Node::Kwnilarg(inner) => self.on_kwnilarg(inner),
            Node::Kwoptarg(inner) => self.on_kwoptarg(inner),
            Node::Kwrestarg(inner) => self.on_kwrestarg(inner),
            Node::Kwsplat(inner) => self.on_kwsplat(inner),
            Node::Lambda(inner) => self.on_lambda(inner),
            Node::Line(inner) => self.on_line(inner),
            Node::Lvar(inner) => self.on_lvar(inner),
            Node::Lvasgn(inner) => self.on_lvasgn(inner),
            Node::Masgn(inner) => self.on_masgn(inner),
            Node::MatchAlt(inner) => self.on_match_alt(inner),
            Node::MatchAs(inner) => self.on_match_as(inner),
            Node::MatchCurrentLine(inner) => self.on_match_current_line(inner),
            Node::MatchNilPattern(inner) => self.on_match_nil_pattern(inner),
            Node::MatchRest(inner) => self.on_match_rest(inner),
            Node::MatchVar(inner) => self.on_match_var(inner),
            Node::MatchWithLvasgn(inner) => self.on_match_with_lvasgn(inner),
            Node::MatchWithTrailingComma(inner) => self.on_match_with_trailing_comma(inner),
            Node::Mlhs(inner) => self.on_mlhs(inner),
            Node::Module(inner) => self.on_module(inner),
            Node::Next(inner) => self.on_next(inner),
            Node::Nil(inner) => self.on_nil(inner),
            Node::NthRef(inner) => self.on_nth_ref(inner),
            Node::Numblock(inner) => self.on_numblock(inner),
            Node::OpAsgn(inner) => self.on_op_asgn(inner),
            Node::Optarg(inner) => self.on_optarg(inner),
            Node::Or(inner) => self.on_or(inner),
            Node::OrAsgn(inner) => self.on_or_asgn(inner),
            Node::Pair(inner) => self.on_pair(inner),
            Node::Pin(inner) => self.on_pin(inner),
            Node::Postexe(inner) => self.on_postexe(inner),
            Node::Preexe(inner) => self.on_preexe(inner),
            Node::Procarg0(inner) => self.on_procarg0(inner),
            Node::Rational(inner) => self.on_rational(inner),
            Node::Redo(inner) => self.on_redo(inner),
            Node::Regexp(inner) => self.on_regexp(inner),
            Node::RegOpt(inner) => self.on_regopt(inner),
            Node::Rescue(inner) => self.on_rescue(inner),
            Node::RescueBody(inner) => self.on_rescue_body(inner),
            Node::Restarg(inner) => self.on_restarg(inner),
            Node::Retry(inner) => self.on_retry(inner),
            Node::Return(inner) => self.on_return(inner),
            Node::SClass(inner) => self.on_sclass(inner),
            Node::Self_(inner) => self.on_self_(inner),
            Node::Send(inner) => self.on_send(inner),
            Node::Shadowarg(inner) => self.on_shadowarg(inner),
            Node::Splat(inner) => self.on_splat(inner),
            Node::Str(inner) => self.on_str(inner),
            Node::Super(inner) => self.on_super(inner),
            Node::Sym(inner) => self.on_sym(inner),
            Node::True(inner) => self.on_true(inner),
            Node::Undef(inner) => self.on_undef(inner),
            Node::UnlessGuard(inner) => self.on_unless_guard(inner),
            Node::Until(inner) => self.on_until(inner),
            Node::UntilPost(inner) => self.on_until_post(inner),
            Node::When(inner) => self.on_when(inner),
            Node::While(inner) => self.on_while(inner),
            Node::WhilePost(inner) => self.on_while_post(inner),
            Node::XHeredoc(inner) => self.on_xheredoc(inner),
            Node::Xstr(inner) => self.on_xstr(inner),
            Node::Yield(inner) => self.on_yield(inner),
            Node::ZSuper(inner) => self.on_zsuper(inner),
        }
    }
}
