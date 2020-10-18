use crate::nodes::*;
use crate::Node;

pub trait Visitor {
    fn visit_all(&mut self, nodes: &Vec<Node>) {
        for node in nodes {
            self.visit(node)
        }
    }

    fn maybe_visit(&mut self, node: &Option<Box<Node>>) {
        if let Some(node) = node {
            self.visit(node)
        }
    }

    #[allow(unused_variables)]
    fn visit_alias(&mut self, node: &Alias) {}

    fn visit_and(&mut self, node: &And) {
        self.visit(&node.lhs);
        self.visit(&node.rhs);
    }

    fn visit_and_asgn(&mut self, node: &AndAsgn) {
        self.visit(&node.recv);
        self.visit(&node.value);
    }

    #[allow(unused_variables)]
    fn visit_arg(&mut self, node: &Arg) {}

    fn visit_args(&mut self, node: &Args) {
        self.visit_all(&node.args);
    }

    fn visit_array(&mut self, node: &Array) {
        self.visit_all(&node.elements);
    }

    fn visit_array_pattern(&mut self, node: &ArrayPattern) {
        self.visit_all(&node.elements);
    }

    fn visit_array_pattern_with_tail(&mut self, node: &ArrayPatternWithTail) {
        self.visit_all(&node.elements);
    }

    #[allow(unused_variables)]
    fn visit_back_ref(&mut self, node: &BackRef) {}

    fn visit_begin(&mut self, node: &Begin) {
        self.visit_all(&node.statements);
    }

    fn visit_block(&mut self, node: &Block) {
        self.visit(&node.call);
        self.maybe_visit(&node.args);
        self.maybe_visit(&node.body);
    }

    #[allow(unused_variables)]
    fn visit_blockarg(&mut self, node: &Blockarg) {}

    #[allow(unused_variables)]
    fn visit_block_pass(&mut self, node: &BlockPass) {}

    fn visit_break(&mut self, node: &Break) {
        self.visit_all(&node.args)
    }

    fn visit_case(&mut self, node: &Case) {
        self.maybe_visit(&node.expr);
        self.visit_all(&node.when_bodies);
        self.maybe_visit(&node.else_body);
    }

    fn visit_case_match(&mut self, node: &CaseMatch) {
        self.visit(&node.expr);
        self.visit_all(&node.in_bodies);
        self.maybe_visit(&node.else_body);
    }

    fn visit_casgn(&mut self, node: &Casgn) {
        self.maybe_visit(&node.scope);
        self.maybe_visit(&node.value);
    }

    #[allow(unused_variables)]
    fn visit_cbase(&mut self, node: &Cbase) {}

    fn visit_class(&mut self, node: &Class) {
        self.visit(&node.name);
        self.maybe_visit(&node.superclass);
        self.maybe_visit(&node.body);
    }

    #[allow(unused_variables)]
    fn visit_complex(&mut self, node: &Complex) {}

    fn visit_const(&mut self, node: &Const) {
        self.maybe_visit(&node.scope);
    }

    fn visit_const_pattern(&mut self, node: &ConstPattern) {
        self.visit(&node.const_);
        self.visit(&node.pattern);
    }

    fn visit_csend(&mut self, node: &CSend) {
        self.visit(&node.receiver);
        self.visit_all(&node.args)
    }

    #[allow(unused_variables)]
    fn visit_cvar(&mut self, node: &Cvar) {}

    fn visit_cvasgn(&mut self, node: &Cvasgn) {
        self.maybe_visit(&node.value)
    }

    fn visit_def(&mut self, node: &Def) {
        self.maybe_visit(&node.args);
        self.maybe_visit(&node.body);
    }

    fn visit_defined(&mut self, node: &Defined) {
        self.visit(&node.value)
    }

    fn visit_defs(&mut self, node: &Defs) {
        self.visit(&node.definee);
        self.maybe_visit(&node.args);
        self.maybe_visit(&node.body);
    }

    fn visit_dstr(&mut self, node: &Dstr) {
        self.visit_all(&node.parts);
    }

    fn visit_dsym(&mut self, node: &Dsym) {
        self.visit_all(&node.parts);
    }

    fn visit_eflipflop(&mut self, node: &EFlipFlop) {
        self.maybe_visit(&node.left);
        self.maybe_visit(&node.right);
    }

    #[allow(unused_variables)]
    fn visit_empty_else(&mut self, node: &EmptyElse) {}

    #[allow(unused_variables)]
    fn visit_encoding(&mut self, node: &Encoding) {}

    fn visit_ensure(&mut self, node: &Ensure) {
        self.maybe_visit(&node.body);
        self.maybe_visit(&node.ensure);
    }

    fn visit_erange(&mut self, node: &Erange) {
        self.maybe_visit(&node.left);
        self.maybe_visit(&node.right);
    }

    #[allow(unused_variables)]
    fn visit_false(&mut self, node: &False) {}

    #[allow(unused_variables)]
    fn visit_file(&mut self, node: &File) {}

    fn visit_find_pattern(&mut self, node: &FindPattern) {
        self.visit_all(&node.elements);
    }

    #[allow(unused_variables)]
    fn visit_float(&mut self, node: &Float) {}

    fn visit_for(&mut self, node: &For) {
        self.visit(&node.iterator);
        self.visit(&node.iteratee);
        self.maybe_visit(&node.body);
    }

    #[allow(unused_variables)]
    fn visit_forward_arg(&mut self, node: &ForwardArg) {}

    #[allow(unused_variables)]
    fn visit_forwarded_args(&mut self, node: &ForwardedArgs) {}

    #[allow(unused_variables)]
    fn visit_gvar(&mut self, node: &Gvar) {}

    fn visit_gvasgn(&mut self, node: &Gvasgn) {
        self.maybe_visit(&node.value)
    }

    fn visit_hash(&mut self, node: &Hash) {
        self.visit_all(&node.pairs);
    }

    fn visit_hash_pattern(&mut self, node: &HashPattern) {
        self.visit_all(&node.elements);
    }

    fn visit_heredoc(&mut self, node: &Heredoc) {
        self.visit_all(&node.parts);
    }

    fn visit_if(&mut self, node: &If) {
        self.visit(&node.cond);
        self.maybe_visit(&node.if_true);
        self.maybe_visit(&node.if_false);
    }

    fn visit_if_guard(&mut self, node: &IfGuard) {
        self.visit(&node.cond)
    }

    fn visit_iflipflop(&mut self, node: &IFlipFlop) {
        self.maybe_visit(&node.left);
        self.maybe_visit(&node.right);
    }

    fn visit_if_mod(&mut self, node: &IfMod) {
        self.visit(&node.cond);
        self.maybe_visit(&node.if_true);
        self.maybe_visit(&node.if_false);
    }

    fn visit_if_ternary(&mut self, node: &IfTernary) {
        self.visit(&node.cond);
        self.visit(&node.if_true);
        self.visit(&node.if_false);
    }

    fn visit_index(&mut self, node: &Index) {
        self.visit(&node.recv);
        self.visit_all(&node.indexes);
    }

    fn visit_index_asgn(&mut self, node: &IndexAsgn) {
        self.visit(&node.recv);
        self.visit_all(&node.indexes);
        self.maybe_visit(&node.value);
    }

    fn visit_in_match(&mut self, node: &InMatch) {
        self.visit(&node.value);
        self.visit(&node.pattern);
    }

    fn visit_in_pattern(&mut self, node: &InPattern) {
        self.visit(&node.pattern);
        self.maybe_visit(&node.guard);
        self.maybe_visit(&node.body);
    }

    #[allow(unused_variables)]
    fn visit_int(&mut self, node: &Int) {}

    fn visit_irange(&mut self, node: &Irange) {
        self.maybe_visit(&node.left);
        self.maybe_visit(&node.right);
    }

    #[allow(unused_variables)]
    fn visit_ivar(&mut self, node: &Ivar) {}

    fn visit_ivasgn(&mut self, node: &Ivasgn) {
        self.maybe_visit(&node.value)
    }

    #[allow(unused_variables)]
    fn visit_kwarg(&mut self, node: &Kwarg) {}

    fn visit_kwbegin(&mut self, node: &KwBegin) {
        self.visit_all(&node.statements)
    }

    #[allow(unused_variables)]
    fn visit_kwnilarg(&mut self, node: &Kwnilarg) {}

    fn visit_kwoptarg(&mut self, node: &Kwoptarg) {
        self.visit(&node.default)
    }

    #[allow(unused_variables)]
    fn visit_kwrestarg(&mut self, node: &Kwrestarg) {}

    fn visit_kwsplat(&mut self, node: &Kwsplat) {
        self.visit(&node.value)
    }

    #[allow(unused_variables)]
    fn visit_lambda(&mut self, node: &Lambda) {}

    #[allow(unused_variables)]
    fn visit_line(&mut self, node: &Line) {}

    #[allow(unused_variables)]
    fn visit_lvar(&mut self, node: &Lvar) {}

    fn visit_lvasgn(&mut self, node: &Lvasgn) {
        self.maybe_visit(&node.value)
    }

    fn visit_masgn(&mut self, node: &Masgn) {
        self.visit(&node.lhs);
        self.visit(&node.rhs);
    }

    fn visit_match_alt(&mut self, node: &MatchAlt) {
        self.visit(&node.lhs);
        self.visit(&node.rhs);
    }

    fn visit_match_as(&mut self, node: &MatchAs) {
        self.visit(&node.value);
        self.visit(&node.as_);
    }

    fn visit_match_current_line(&mut self, node: &MatchCurrentLine) {
        self.visit(&node.re);
    }

    #[allow(unused_variables)]
    fn visit_match_nil_pattern(&mut self, node: &MatchNilPattern) {}

    fn visit_match_rest(&mut self, node: &MatchRest) {
        self.maybe_visit(&node.name);
    }

    #[allow(unused_variables)]
    fn visit_match_var(&mut self, node: &MatchVar) {}

    fn visit_match_with_lvasgn(&mut self, node: &MatchWithLvasgn) {
        self.visit(&node.re);
        self.visit(&node.arg);
    }

    fn visit_match_with_trailing_comma(&mut self, node: &MatchWithTrailingComma) {
        self.visit(&node.match_)
    }

    fn visit_mlhs(&mut self, node: &Mlhs) {
        self.visit_all(&node.items)
    }
    fn visit_module(&mut self, node: &Module) {
        self.visit(&node.name);
        self.maybe_visit(&node.body)
    }

    fn visit_next(&mut self, node: &Next) {
        self.visit_all(&node.args)
    }

    #[allow(unused_variables)]
    fn visit_nil(&mut self, node: &Nil) {}

    #[allow(unused_variables)]
    fn visit_nth_ref(&mut self, node: &NthRef) {}

    fn visit_numblock(&mut self, node: &Numblock) {
        self.visit(&node.call);
        self.visit(&node.body);
    }

    fn visit_op_asgn(&mut self, node: &OpAsgn) {
        self.visit(&node.recv);
        self.visit(&node.value);
    }

    fn visit_optarg(&mut self, node: &Optarg) {
        self.visit(&node.default)
    }

    fn visit_or(&mut self, node: &Or) {
        self.visit(&node.lhs);
        self.visit(&node.rhs);
    }

    fn visit_or_asgn(&mut self, node: &OrAsgn) {
        self.visit(&node.recv);
        self.visit(&node.value);
    }

    fn visit_pair(&mut self, node: &Pair) {
        self.visit(&node.key);
        self.visit(&node.value);
    }

    fn visit_pin(&mut self, node: &Pin) {
        self.visit(&node.var);
    }

    fn visit_postexe(&mut self, node: &Postexe) {
        self.maybe_visit(&node.body)
    }

    fn visit_preexe(&mut self, node: &Preexe) {
        self.maybe_visit(&node.body)
    }

    fn visit_procarg0(&mut self, node: &Procarg0) {
        self.visit_all(&node.args);
    }

    #[allow(unused_variables)]
    fn visit_rational(&mut self, node: &Rational) {}

    #[allow(unused_variables)]
    fn visit_redo(&mut self, node: &Redo) {}

    fn visit_regexp(&mut self, node: &Regexp) {
        self.visit_all(&node.parts);
        self.visit(&node.options)
    }

    #[allow(unused_variables)]
    fn visit_regopt(&mut self, node: &RegOpt) {}

    fn visit_rescue(&mut self, node: &Rescue) {
        self.maybe_visit(&node.body);
        self.visit_all(&node.rescue_bodies);
        self.maybe_visit(&node.else_);
    }

    fn visit_rescue_body(&mut self, node: &RescueBody) {
        self.maybe_visit(&node.exc_list);
        self.maybe_visit(&node.exc_var);
        self.maybe_visit(&node.body);
    }

    #[allow(unused_variables)]
    fn visit_restarg(&mut self, node: &Restarg) {}

    #[allow(unused_variables)]
    fn visit_retry(&mut self, node: &Retry) {}

    fn visit_return(&mut self, node: &Return) {
        self.visit_all(&node.args)
    }

    fn visit_sclass(&mut self, node: &SClass) {
        self.visit(&node.expr);
        self.maybe_visit(&node.body)
    }

    #[allow(unused_variables)]
    fn visit_self_(&mut self, node: &Self_) {}

    fn visit_send(&mut self, node: &Send) {
        self.maybe_visit(&node.receiver);
        self.visit_all(&node.args);
    }

    #[allow(unused_variables)]
    fn visit_shadowarg(&mut self, node: &Shadowarg) {}

    fn visit_splat(&mut self, node: &Splat) {
        self.maybe_visit(&node.value);
    }

    #[allow(unused_variables)]
    fn visit_str(&mut self, node: &Str) {}

    fn visit_super(&mut self, node: &Super) {
        self.visit_all(&node.args);
    }

    #[allow(unused_variables)]
    fn visit_sym(&mut self, node: &Sym) {}

    #[allow(unused_variables)]
    fn visit_true(&mut self, node: &True) {}

    fn visit_undef(&mut self, node: &Undef) {
        self.visit_all(&node.names)
    }

    fn visit_unless_guard(&mut self, node: &UnlessGuard) {
        self.visit(&node.cond)
    }

    fn visit_until(&mut self, node: &Until) {
        self.visit(&node.cond);
        self.maybe_visit(&node.body)
    }

    fn visit_until_post(&mut self, node: &UntilPost) {
        self.visit(&node.cond);
        self.visit(&node.body)
    }

    fn visit_when(&mut self, node: &When) {
        self.visit_all(&node.patterns);
        self.maybe_visit(&node.body)
    }

    fn visit_while(&mut self, node: &While) {
        self.visit(&node.cond);
        self.maybe_visit(&node.body)
    }

    fn visit_while_post(&mut self, node: &WhilePost) {
        self.visit(&node.cond);
        self.visit(&node.body)
    }

    fn visit_xheredoc(&mut self, node: &XHeredoc) {
        self.visit_all(&node.parts);
    }

    fn visit_xstr(&mut self, node: &Xstr) {
        self.visit_all(&node.parts);
    }

    fn visit_yield(&mut self, node: &Yield) {
        self.visit_all(&node.args)
    }

    #[allow(unused_variables)]
    fn visit_zsuper(&mut self, node: &ZSuper) {}

    fn visit(&mut self, node: &Node) {
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
            Node::InMatch(inner) => self.visit_in_match(inner),
            Node::InPattern(inner) => self.visit_in_pattern(inner),
            Node::Int(inner) => self.visit_int(inner),
            Node::Irange(inner) => self.visit_irange(inner),
            Node::Ivar(inner) => self.visit_ivar(inner),
            Node::Ivasgn(inner) => self.visit_ivasgn(inner),
            Node::Kwarg(inner) => self.visit_kwarg(inner),
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
            Node::MatchRest(inner) => self.visit_match_rest(inner),
            Node::MatchVar(inner) => self.visit_match_var(inner),
            Node::MatchWithLvasgn(inner) => self.visit_match_with_lvasgn(inner),
            Node::MatchWithTrailingComma(inner) => self.visit_match_with_trailing_comma(inner),
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
    }
}
