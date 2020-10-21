use ruby_parser::source::Range;
use ruby_parser::Node;

fn print_loc(name: &str, loc: &Range) {
    println!(
        "{}{} {}",
        " ".repeat(loc.begin_pos),
        "~".repeat(loc.size()),
        name
    )
}

fn maybe_print_loc(name: &str, loc: &Option<Range>) {
    if let Some(loc) = loc {
        print_loc(name, loc)
    }
}

fn maybe_print_all_locs(src: &str, node: &Option<Box<Node>>) {
    if let Some(node) = node {
        print_all_locs(src, node)
    }
}
fn foeach_print_all_locs(src: &str, nodes: &Vec<Node>) {
    for node in nodes {
        print_all_locs(src, node)
    }
}

#[allow(dead_code)]
pub fn print_all_locs(src: &str, node: &Node) {
    println!("{}", node.inspect(0));
    println!("{}", src);

    match node {
        Node::Alias(node) => {
            print_loc("alias", &node.keyword_l);
            print_loc("expression", &node.expression_l);

            print_all_locs(src, &node.to);
            print_all_locs(src, &node.from);
        }
        Node::And(node) => {
            print_loc("operator", &node.operator_l);
            print_loc("expression", &node.expression_l);

            print_all_locs(src, &node.lhs);
            print_all_locs(src, &node.rhs);
        }
        Node::AndAsgn(node) => {
            print_loc("operator", &node.operator_l);
            print_loc("expression", &node.expression_l);

            print_all_locs(src, &node.recv);
            print_all_locs(src, &node.value);
        }
        Node::Arg(node) => {
            print_loc("expression", &node.expression_l);
        }
        Node::Args(node) => {
            maybe_print_loc("begin", &node.begin_l);
            maybe_print_loc("end", &node.end_l);
            print_loc("expression", &node.expression_l);

            foeach_print_all_locs(src, &node.args)
        }
        Node::Array(node) => {
            maybe_print_loc("begin", &node.begin_l);
            maybe_print_loc("end", &node.end_l);
            print_loc("expression", &node.expression_l);

            foeach_print_all_locs(src, &node.elements)
        }
        Node::ArrayPattern(node) => {
            maybe_print_loc("begin", &node.begin_l);
            maybe_print_loc("end", &node.end_l);
            print_loc("expression", &node.expression_l);

            foeach_print_all_locs(src, &node.elements)
        }
        Node::ArrayPatternWithTail(node) => {
            maybe_print_loc("begin", &node.begin_l);
            maybe_print_loc("end", &node.end_l);
            print_loc("expression", &node.expression_l);

            foeach_print_all_locs(src, &node.elements)
        }
        Node::BackRef(node) => {
            print_loc("expression", &node.expression_l);
        }
        Node::Begin(node) => {
            maybe_print_loc("begin", &node.begin_l);
            maybe_print_loc("end", &node.end_l);
            print_loc("expression", &node.expression_l);

            foeach_print_all_locs(src, &node.statements)
        }
        Node::Block(node) => {
            print_loc("begin", &node.begin_l);
            print_loc("end", &node.end_l);
            print_loc("expression", &node.expression_l);

            print_all_locs(src, &node.call);
            maybe_print_all_locs(src, &node.args);
            maybe_print_all_locs(src, &node.body);
        }
        Node::Blockarg(node) => {
            print_loc("operator", &node.amper_l);
            print_loc("name", &node.name_l);
            print_loc("expression", &node.expression_l);
        }
        Node::BlockPass(node) => {
            print_loc("operator", &node.operator_l);
            print_loc("expression", &node.expression_l);

            print_all_locs(src, &node.value);
        }
        Node::Break(node) => {
            print_loc("keyword", &node.keyword_l);
            maybe_print_loc("begin", &node.begin_l);
            maybe_print_loc("end", &node.end_l);
            print_loc("expression", &node.expression_l);

            foeach_print_all_locs(src, &node.args);
        }
        Node::Case(node) => {
            print_loc("keyword", &node.keyword_l);
            maybe_print_loc("else", &node.else_l);
            print_loc("end", &node.end_l);
            print_loc("expression", &node.expression_l);

            maybe_print_all_locs(src, &node.expr);
            foeach_print_all_locs(src, &node.when_bodies);
            maybe_print_all_locs(src, &node.else_body);
        }
        Node::CaseMatch(node) => {
            print_loc("keyword", &node.keyword_l);
            maybe_print_loc("else", &node.else_l);
            print_loc("end", &node.end_l);
            print_loc("expression", &node.expression_l);

            print_all_locs(src, &node.expr);
            foeach_print_all_locs(src, &node.in_bodies);
            maybe_print_all_locs(src, &node.else_body);
        }
        Node::Casgn(node) => {
            maybe_print_loc("double_colon", &node.double_colon_l);
            print_loc("name", &node.name_l);
            maybe_print_loc("operator", &node.operator_l);
            print_loc("expression", &node.expression_l);

            maybe_print_all_locs(src, &node.scope);
            maybe_print_all_locs(src, &node.value);
        }
        Node::Cbase(node) => {
            print_loc("expression", &node.expression_l);
        }
        Node::Class(node) => {
            print_loc("keyword", &node.keyword_l);
            maybe_print_loc("operator", &node.operator_l);
            print_loc("end", &node.end_l);
            print_loc("expression", &node.expression_l);

            print_all_locs(src, &node.name);
            maybe_print_all_locs(src, &node.superclass);
            maybe_print_all_locs(src, &node.body);
        }
        Node::Complex(node) => {
            maybe_print_loc("operator", &node.operator_l);
            print_loc("expression", &node.expression_l);
        }
        Node::Const(node) => {
            maybe_print_loc("double_colon", &node.double_colon_l);
            print_loc("name", &node.name_l);
            print_loc("expression", &node.expression_l);

            maybe_print_all_locs(src, &node.scope);
        }
        Node::ConstPattern(node) => {
            print_loc("begin", &node.begin_l);
            print_loc("end", &node.end_l);
            print_loc("expression", &node.expression_l);

            print_all_locs(src, &node.const_);
            print_all_locs(src, &node.pattern);
        }
        Node::CSend(node) => {
            maybe_print_loc("dot", &node.dot_l);
            maybe_print_loc("selector", &node.selector_l);
            maybe_print_loc("begin", &node.begin_l);
            maybe_print_loc("end", &node.end_l);
            maybe_print_loc("operator", &node.operator_l);
            print_loc("expression", &node.expression_l);

            print_all_locs(src, &node.recv);
            foeach_print_all_locs(src, &node.args);
        }
        Node::Cvar(node) => {
            print_loc("expression", &node.expression_l);
        }
        Node::Cvasgn(node) => {
            print_loc("name", &node.name_l);
            maybe_print_loc("operator", &node.operator_l);
            print_loc("expression", &node.expression_l);

            maybe_print_all_locs(src, &node.value);
        }
        Node::Def(node) => {
            print_loc("keyword", &node.keyword_l);
            print_loc("name", &node.name_l);
            maybe_print_loc("end", &node.end_l);
            maybe_print_loc("assignment", &node.assignment_l);
            print_loc("expression", &node.expression_l);

            maybe_print_all_locs(src, &node.args);
            maybe_print_all_locs(src, &node.body);
        }
        Node::Defined(node) => {
            print_loc("keyword", &node.keyword_l);
            maybe_print_loc("begin", &node.begin_l);
            maybe_print_loc("end", &node.end_l);
            print_loc("expression", &node.expression_l);

            print_all_locs(src, &node.value);
        }
        Node::Defs(node) => {
            print_loc("keyword", &node.keyword_l);
            print_loc("operator", &node.operator_l);
            print_loc("name", &node.name_l);
            maybe_print_loc("assignment", &node.assignment_l);
            maybe_print_loc("end", &node.end_l);
            print_loc("expression", &node.expression_l);

            print_all_locs(src, &node.definee);
            maybe_print_all_locs(src, &node.args);
            maybe_print_all_locs(src, &node.body);
        }
        Node::Dstr(node) => {
            maybe_print_loc("begin", &node.begin_l);
            maybe_print_loc("end", &node.end_l);
            print_loc("expression", &node.expression_l);

            foeach_print_all_locs(src, &node.parts)
        }
        Node::Dsym(node) => {
            maybe_print_loc("begin", &node.begin_l);
            maybe_print_loc("end", &node.end_l);
            print_loc("expression", &node.expression_l);

            foeach_print_all_locs(src, &node.parts)
        }
        Node::EFlipFlop(node) => {
            print_loc("operator", &node.operator_l);
            print_loc("expression", &node.expression_l);

            maybe_print_all_locs(src, &node.left);
            maybe_print_all_locs(src, &node.right);
        }
        Node::EmptyElse(node) => {
            print_loc("expression", &node.expression_l);
        }
        Node::Encoding(node) => {
            print_loc("expression", &node.expression_l);
        }
        Node::Ensure(node) => {
            print_loc("keyword", &node.keyword_l);
            print_loc("expression", &node.expression_l);

            maybe_print_all_locs(src, &node.body);
            maybe_print_all_locs(src, &node.ensure);
        }
        Node::Erange(node) => {
            print_loc("operator", &node.operator_l);
            print_loc("expression", &node.expression_l);

            maybe_print_all_locs(src, &node.left);
            maybe_print_all_locs(src, &node.right);
        }
        Node::False(node) => {
            print_loc("expression", &node.expression_l);
        }
        Node::File(node) => {
            print_loc("expression", &node.expression_l);
        }
        Node::FindPattern(node) => {
            maybe_print_loc("begin", &node.begin_l);
            maybe_print_loc("end", &node.end_l);
            print_loc("expression", &node.expression_l);

            foeach_print_all_locs(src, &node.elements)
        }
        Node::Float(node) => {
            maybe_print_loc("operator", &node.operator_l);
            print_loc("expression", &node.expression_l);
        }
        Node::For(node) => {
            print_loc("keyword", &node.keyword_l);
            print_loc("in", &node.in_l);
            print_loc("begin", &node.begin_l);
            print_loc("end", &node.end_l);
            print_loc("expression", &node.expression_l);

            print_all_locs(src, &node.iterator);
            print_all_locs(src, &node.iteratee);
            maybe_print_all_locs(src, &node.body);
        }
        Node::ForwardArg(node) => {
            print_loc("expression", &node.expression_l);
        }
        Node::ForwardedArgs(node) => {
            print_loc("expression", &node.expression_l);
        }
        Node::Gvar(node) => {
            print_loc("expression", &node.expression_l);
        }
        Node::Gvasgn(node) => {
            print_loc("name", &node.name_l);
            maybe_print_loc("operator", &node.operator_l);
            print_loc("expression", &node.expression_l);

            maybe_print_all_locs(src, &node.value);
        }
        Node::Hash(node) => {
            maybe_print_loc("begin", &node.begin_l);
            maybe_print_loc("end", &node.end_l);
            print_loc("expression", &node.expression_l);

            foeach_print_all_locs(src, &node.pairs)
        }
        Node::HashPattern(node) => {
            maybe_print_loc("begin", &node.begin_l);
            maybe_print_loc("end", &node.end_l);
            print_loc("expression", &node.expression_l);

            foeach_print_all_locs(src, &node.elements)
        }
        Node::Heredoc(node) => {
            print_loc("heredoc_body", &node.heredoc_body_l);
            print_loc("heredoc_end", &node.heredoc_end_l);
            print_loc("expression", &node.expression_l);

            foeach_print_all_locs(src, &node.parts)
        }
        Node::If(node) => {
            print_loc("keyword", &node.keyword_l);
            print_loc("begin", &node.begin_l);
            maybe_print_loc("else", &node.else_l);
            maybe_print_loc("end", &node.end_l);
            print_loc("expression", &node.expression_l);

            print_all_locs(src, &node.cond);
            maybe_print_all_locs(src, &node.if_true);
            maybe_print_all_locs(src, &node.if_false);
        }
        Node::IfGuard(node) => {
            print_loc("keyword", &node.keyword_l);
            print_loc("expression", &node.expression_l);

            print_all_locs(src, &node.cond);
        }
        Node::IFlipFlop(node) => {
            print_loc("operator", &node.operator_l);
            print_loc("expression", &node.expression_l);

            maybe_print_all_locs(src, &node.left);
            maybe_print_all_locs(src, &node.right);
        }
        Node::IfMod(node) => {
            print_loc("keyword", &node.keyword_l);
            print_loc("expression", &node.expression_l);

            print_all_locs(src, &node.cond);
            maybe_print_all_locs(src, &node.if_true);
            maybe_print_all_locs(src, &node.if_false);
        }
        Node::IfTernary(node) => {
            print_loc("question", &node.question_l);
            print_loc("colon", &node.colon_l);
            print_loc("expression", &node.expression_l);

            print_all_locs(src, &node.cond);
            print_all_locs(src, &node.if_true);
            print_all_locs(src, &node.if_false);
        }
        Node::Index(node) => {
            print_loc("begin", &node.begin_l);
            print_loc("end", &node.end_l);
            print_loc("expression", &node.expression_l);

            print_all_locs(src, &node.recv);
            foeach_print_all_locs(src, &node.indexes);
        }
        Node::IndexAsgn(node) => {
            print_loc("begin", &node.begin_l);
            print_loc("end", &node.end_l);
            maybe_print_loc("operator", &node.operator_l);
            print_loc("expression", &node.expression_l);

            print_all_locs(src, &node.recv);
            foeach_print_all_locs(src, &node.indexes);
            maybe_print_all_locs(src, &node.value);
        }
        Node::InMatch(node) => {
            print_loc("operator", &node.operator_l);
            print_loc("expression", &node.expression_l);

            print_all_locs(src, &node.value);
            print_all_locs(src, &node.pattern);
        }
        Node::InPattern(node) => {
            print_loc("keyword", &node.keyword_l);
            print_loc("begin", &node.begin_l);
            print_loc("expression", &node.expression_l);

            print_all_locs(src, &node.pattern);
            maybe_print_all_locs(src, &node.guard);
            maybe_print_all_locs(src, &node.body);
        }
        Node::Int(node) => {
            maybe_print_loc("operator", &node.operator_l);
            print_loc("expression", &node.expression_l);
        }
        Node::Irange(node) => {
            print_loc("operator", &node.operator_l);
            print_loc("expression", &node.expression_l);

            maybe_print_all_locs(src, &node.left);
            maybe_print_all_locs(src, &node.right);
        }
        Node::Ivar(node) => {
            print_loc("expression", &node.expression_l);
        }
        Node::Ivasgn(node) => {
            print_loc("name", &node.name_l);
            maybe_print_loc("operator", &node.operator_l);
            print_loc("expression", &node.expression_l);

            maybe_print_all_locs(src, &node.value);
        }
        Node::Kwarg(node) => {
            print_loc("name", &node.name_l);
            print_loc("expression", &node.expression_l);
        }
        Node::KwBegin(node) => {
            maybe_print_loc("begin", &node.begin_l);
            maybe_print_loc("end", &node.end_l);
            print_loc("expression", &node.expression_l);

            foeach_print_all_locs(src, &node.statements)
        }
        Node::Kwnilarg(node) => {
            print_loc("name", &node.name_l);
            print_loc("expression", &node.expression_l);
        }
        Node::Kwoptarg(node) => {
            print_loc("name", &node.name_l);
            print_loc("expression", &node.expression_l);

            print_all_locs(src, &node.default);
        }
        Node::Kwrestarg(node) => {
            print_loc("operator", &node.dstar_l);
            maybe_print_loc("name", &node.name_l);
            print_loc("expression", &node.expression_l);
        }
        Node::Kwsplat(node) => {
            print_loc("operator", &node.operator_l);
            print_loc("expression", &node.expression_l);

            print_all_locs(src, &node.value);
        }
        Node::Lambda(node) => {
            print_loc("expression", &node.expression_l);
        }
        Node::Line(node) => {
            print_loc("expression", &node.expression_l);
        }
        Node::Lvar(node) => {
            print_loc("expression", &node.expression_l);
        }
        Node::Lvasgn(node) => {
            print_loc("name", &node.name_l);
            maybe_print_loc("operator", &node.operator_l);
            print_loc("expression", &node.expression_l);

            maybe_print_all_locs(src, &node.value);
        }
        Node::Masgn(node) => {
            print_loc("operator", &node.operator_l);
            print_loc("expression", &node.expression_l);

            print_all_locs(src, &node.lhs);
            print_all_locs(src, &node.rhs);
        }
        Node::MatchAlt(node) => {
            print_loc("operator", &node.operator_l);
            print_loc("expression", &node.expression_l);

            print_all_locs(src, &node.lhs);
            print_all_locs(src, &node.rhs);
        }
        Node::MatchAs(node) => {
            print_loc("operator", &node.operator_l);
            print_loc("expression", &node.expression_l);

            print_all_locs(src, &node.value);
            print_all_locs(src, &node.as_);
        }
        Node::MatchCurrentLine(node) => {
            print_loc("expression", &node.expression_l);
        }
        Node::MatchNilPattern(node) => {
            print_loc("operator", &node.operator_l);
            print_loc("name", &node.name_l);
            print_loc("expression", &node.expression_l);
        }
        Node::MatchRest(node) => {
            print_loc("operator", &node.operator_l);
            print_loc("expression", &node.expression_l);

            maybe_print_all_locs(src, &node.name);
        }
        Node::MatchVar(node) => {
            print_loc("name", &node.name_l);
            print_loc("expression", &node.expression_l);
        }
        Node::MatchWithLvasgn(node) => {
            print_loc("operator", &node.operator_l);
            print_loc("expression", &node.expression_l);

            print_all_locs(src, &node.re);
            print_all_locs(src, &node.value);
        }
        Node::MatchWithTrailingComma(node) => {
            print_loc("expression", &node.expression_l);

            print_all_locs(src, &node.match_);
        }
        Node::Mlhs(node) => {
            maybe_print_loc("begin", &node.begin_l);
            maybe_print_loc("end", &node.end_l);
            print_loc("expression", &node.expression_l);

            foeach_print_all_locs(src, &node.items);
        }
        Node::Module(node) => {
            print_loc("keyword", &node.keyword_l);
            print_loc("end", &node.end_l);
            print_loc("expression", &node.expression_l);

            print_all_locs(src, &node.name);
            maybe_print_all_locs(src, &node.body);
        }
        Node::Next(node) => {
            print_loc("keyword", &node.keyword_l);
            print_loc("expression", &node.expression_l);

            foeach_print_all_locs(src, &node.args);
        }
        Node::Nil(node) => {
            print_loc("expression", &node.expression_l);
        }
        Node::NthRef(node) => {
            print_loc("expression", &node.expression_l);
        }
        Node::Numblock(node) => {
            print_loc("begin", &node.begin_l);
            print_loc("end", &node.end_l);
            print_loc("expression", &node.expression_l);

            print_all_locs(src, &node.call);
            print_all_locs(src, &node.body);
        }
        Node::OpAsgn(node) => {
            print_loc("operator", &node.operator_l);
            print_loc("expression", &node.expression_l);

            print_all_locs(src, &node.recv);
            print_all_locs(src, &node.value);
        }
        Node::Optarg(node) => {
            print_loc("name", &node.name_l);
            print_loc("expression", &node.expression_l);

            print_all_locs(src, &node.default);
        }
        Node::Or(node) => {
            print_loc("operator", &node.operator_l);
            print_loc("expression", &node.expression_l);

            print_all_locs(src, &node.lhs);
            print_all_locs(src, &node.rhs);
        }
        Node::OrAsgn(node) => {
            print_loc("operator", &node.operator_l);
            print_loc("expression", &node.expression_l);

            print_all_locs(src, &node.recv);
            print_all_locs(src, &node.value);
        }
        Node::Pair(node) => {
            print_loc("operator", &node.operator_l);
            print_loc("expression", &node.expression_l);

            print_all_locs(src, &node.key);
            print_all_locs(src, &node.value);
        }
        Node::Pin(node) => {
            print_loc("selector", &node.selector_l);
            print_loc("expression", &node.expression_l);

            print_all_locs(src, &node.var);
        }
        Node::Postexe(node) => {
            print_loc("begin", &node.begin_l);
            print_loc("end", &node.end_l);
            print_loc("expression", &node.expression_l);

            maybe_print_all_locs(src, &node.body);
        }
        Node::Preexe(node) => {
            print_loc("begin", &node.begin_l);
            print_loc("end", &node.end_l);
            print_loc("expression", &node.expression_l);

            maybe_print_all_locs(src, &node.body);
        }
        Node::Procarg0(node) => {
            maybe_print_loc("begin", &node.begin_l);
            maybe_print_loc("end", &node.end_l);
            print_loc("expression", &node.expression_l);

            foeach_print_all_locs(src, &node.args);
        }
        Node::Rational(node) => {
            maybe_print_loc("operator", &node.operator_l);
            print_loc("expression", &node.expression_l);
        }
        Node::Redo(node) => {
            print_loc("expression", &node.expression_l);
        }
        Node::Regexp(node) => {
            print_loc("begin", &node.begin_l);
            print_loc("end", &node.end_l);
            print_loc("expression", &node.expression_l);

            foeach_print_all_locs(src, &node.parts);
            print_all_locs(src, &node.options);
        }
        Node::RegOpt(node) => {
            print_loc("expression", &node.expression_l);
        }
        Node::Rescue(node) => {
            maybe_print_loc("else", &node.else_l);
            print_loc("expression", &node.expression_l);

            maybe_print_all_locs(src, &node.body);
            foeach_print_all_locs(src, &node.rescue_bodies);
            maybe_print_all_locs(src, &node.else_);
        }
        Node::RescueBody(node) => {
            print_loc("keyword", &node.keyword_l);
            maybe_print_loc("assoc", &node.assoc_l);
            maybe_print_loc("begin", &node.begin_l);
            print_loc("expression", &node.expression_l);

            maybe_print_all_locs(src, &node.exc_list);
            maybe_print_all_locs(src, &node.exc_var);
            maybe_print_all_locs(src, &node.body);
        }
        Node::Restarg(node) => {
            print_loc("operator", &node.star_l);
            maybe_print_loc("name", &node.name_l);
            print_loc("expression", &node.expression_l);
        }
        Node::Retry(node) => {
            print_loc("expression", &node.expression_l);
        }
        Node::Return(node) => {
            print_loc("keyword", &node.keyword_l);
            print_loc("expression", &node.expression_l);

            foeach_print_all_locs(src, &node.args);
        }
        Node::SClass(node) => {
            print_loc("keyword", &node.keyword_l);
            print_loc("operator", &node.operator_l);
            print_loc("end", &node.end_l);
            print_loc("expression", &node.expression_l);

            print_all_locs(src, &node.expr);
            maybe_print_all_locs(src, &node.body);
        }
        Node::Self_(node) => {
            print_loc("expression", &node.expression_l);
        }
        Node::Send(node) => {
            maybe_print_loc("dot", &node.dot_l);
            maybe_print_loc("selector", &node.selector_l);
            maybe_print_loc("begin", &node.begin_l);
            maybe_print_loc("end", &node.end_l);
            maybe_print_loc("operator", &node.operator_l);
            print_loc("expression", &node.expression_l);

            maybe_print_all_locs(src, &node.recv);
            foeach_print_all_locs(src, &node.args);
        }
        Node::Shadowarg(node) => {
            print_loc("expression", &node.expression_l);
        }
        Node::Splat(node) => {
            print_loc("operator", &node.operator_l);
            print_loc("expression", &node.expression_l);

            maybe_print_all_locs(src, &node.value);
        }
        Node::Str(node) => {
            maybe_print_loc("begin", &node.begin_l);
            maybe_print_loc("end", &node.end_l);
            print_loc("expression", &node.expression_l);
        }
        Node::Super(node) => {
            print_loc("keyword", &node.keyword_l);
            maybe_print_loc("begin", &node.begin_l);
            maybe_print_loc("end", &node.end_l);
            print_loc("expression", &node.expression_l);

            foeach_print_all_locs(src, &node.args)
        }
        Node::Sym(node) => {
            maybe_print_loc("begin", &node.begin_l);
            maybe_print_loc("end", &node.end_l);
            print_loc("expression", &node.expression_l);
        }
        Node::True(node) => {
            print_loc("expression", &node.expression_l);
        }
        Node::Undef(node) => {
            print_loc("keyword", &node.keyword_l);
            print_loc("expression", &node.expression_l);

            foeach_print_all_locs(src, &node.names);
        }
        Node::UnlessGuard(node) => {
            print_loc("keyword", &node.keyword_l);
            print_loc("expression", &node.expression_l);

            print_all_locs(src, &node.cond);
        }
        Node::Until(node) => {
            print_loc("keyword_l", &node.keyword_l);
            maybe_print_loc("begin_l", &node.begin_l);
            maybe_print_loc("end_l", &node.end_l);
            print_loc("expression_l", &node.expression_l);

            print_all_locs(src, &node.cond);
            maybe_print_all_locs(src, &node.body);
        }
        Node::UntilPost(node) => {
            print_loc("keyword_l", &node.keyword_l);
            print_loc("expression_l", &node.expression_l);

            print_all_locs(src, &node.cond);
            print_all_locs(src, &node.body);
        }
        Node::When(node) => {
            print_loc("keyword", &node.keyword_l);
            print_loc("begin", &node.begin_l);
            print_loc("expression", &node.expression_l);

            foeach_print_all_locs(src, &node.patterns);
            maybe_print_all_locs(src, &node.body);
        }
        Node::While(node) => {
            print_loc("keyword_l", &node.keyword_l);
            maybe_print_loc("begin_l", &node.begin_l);
            maybe_print_loc("end_l", &node.end_l);
            print_loc("expression_l", &node.expression_l);

            print_all_locs(src, &node.cond);
            maybe_print_all_locs(src, &node.body);
        }
        Node::WhilePost(node) => {
            print_loc("keyword_l", &node.keyword_l);
            print_loc("expression_l", &node.expression_l);

            print_all_locs(src, &node.cond);
            print_all_locs(src, &node.body);
        }
        Node::XHeredoc(node) => {
            print_loc("heredoc_body", &node.heredoc_body_l);
            print_loc("heredoc_end", &node.heredoc_end_l);
            print_loc("expression", &node.expression_l);

            foeach_print_all_locs(src, &node.parts)
        }
        Node::Xstr(node) => {
            print_loc("begin", &node.begin_l);
            print_loc("end", &node.end_l);
            print_loc("expression", &node.expression_l);

            foeach_print_all_locs(src, &node.parts)
        }
        Node::Yield(node) => {
            print_loc("keyword", &node.keyword_l);
            maybe_print_loc("begin", &node.begin_l);
            maybe_print_loc("end", &node.end_l);
            print_loc("expression", &node.expression_l);

            foeach_print_all_locs(src, &node.args)
        }
        Node::ZSuper(node) => {
            print_loc("expression", &node.expression_l);
        }
    }
}
