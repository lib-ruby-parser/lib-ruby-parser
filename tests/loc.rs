// use ruby_parser::Node;

// #[derive(Debug, PartialEq)]
// enum ParseLocState {
//     SkipWs,
//     Cursor,
//     Name,
//     Lparen,
//     Path,
//     Done,
// }

// #[derive(Debug)]
// enum LocName {
//     Begin,
//     End,
//     Expression,
//     Keyword,
//     Name,
//     Assignment,
//     Colon,
//     DoubleColon,
//     Else,
//     HeredocBody,
//     Operator,
//     Selector,
// }

// #[derive(Debug)]
// pub struct Loc {
//     begin: usize,
//     end: usize,
//     name: LocName,
//     path: Path,
// }

// impl Loc {
//     pub fn new(loc: &str) -> Self {
//         let mut state = ParseLocState::SkipWs;
//         let mut begin: Option<usize> = None;
//         let mut end: Option<usize> = None;
//         let mut name = String::from("");
//         let mut path = String::from("");

//         for (idx, c) in loc.chars().enumerate() {
//             match (&state, c) {
//                 (ParseLocState::SkipWs, ' ') => { /* skip */ }
//                 (ParseLocState::SkipWs, '~') => {
//                     state = ParseLocState::Cursor;
//                     begin = Some(idx);
//                 }
//                 (ParseLocState::Cursor, '~') => { /* keep reading */ }
//                 (ParseLocState::Cursor, ' ') => {
//                     state = ParseLocState::Name;
//                     end = Some(idx);
//                 }
//                 (ParseLocState::Name, ' ') => {
//                     state = ParseLocState::Lparen;
//                 }
//                 (ParseLocState::Name, c) => {
//                     name.push(c);
//                 }
//                 (ParseLocState::Lparen, '(') => {
//                     state = ParseLocState::Path;
//                 }
//                 (ParseLocState::Path, ')') => {
//                     state = ParseLocState::Done;
//                 }
//                 (ParseLocState::Path, c) => path.push(c),
//                 _ => {
//                     panic!("Got state = {:?} and c = {}", state, c);
//                 }
//             }
//         }

//         if state != ParseLocState::Done {
//             panic!("Failed to parse loc {}, state = {:?}", loc, state);
//         }
//         let begin = begin.unwrap_or_else(|| panic!("no begin captured"));
//         let end = end.unwrap_or_else(|| panic!("no begin captured"));

//         let path = path.split("/").map(|e| e.to_owned()).collect::<Vec<_>>();
//         let path = path
//             .into_iter()
//             .filter(|e| !e.is_empty())
//             .collect::<Vec<_>>();
//         let path = Path::new(path);

//         let name = match &name[..] {
//             "begin" => LocName::Begin,
//             "end" => LocName::End,
//             "expression" => LocName::Expression,
//             "keyword" => LocName::Keyword,
//             "name" => LocName::Name,
//             "assignment" => LocName::Assignment,
//             "colon" => LocName::Colon,
//             "double_colon" => LocName::DoubleColon,
//             "else" => LocName::Else,
//             "heredoc_body" => LocName::HeredocBody,
//             "operator" => LocName::Operator,
//             "selector" => LocName::Selector,
//             _ => panic!("unsupported loc name {}", name),
//         };

//         Loc {
//             begin,
//             end,
//             name,
//             path,
//         }
//     }

//     pub fn test(&self, node: &Node) -> Result<(), String> {
//         let node = match self.path.go(node) {
//             Some(node) => node,
//             None => return Err(format!("Failed to get node {:?}", self.path)),
//         };

//         let loc = match (&self.name, node) {
//             (LocName::Keyword, Node::Alias { loc, .. }) => Some(loc.keyword.clone()),
//             (LocName::Keyword, Node::Break { loc, .. }) => Some(loc.keyword.clone()),
//             (LocName::Keyword, Node::Case { loc, .. }) => loc.keyword.clone(),
//             (LocName::Keyword, Node::CaseMatch { loc, .. }) => loc.keyword.clone(),
//             (LocName::Keyword, Node::Class { loc, .. }) => Some(loc.keyword.clone()),
//             (LocName::Keyword, Node::Def { loc, .. }) => Some(loc.keyword.clone()),
//             (LocName::Keyword, Node::Defined { loc, .. }) => Some(loc.keyword.clone()),
//             (LocName::Keyword, Node::Defs { loc, .. }) => Some(loc.keyword.clone()),
//             (LocName::Keyword, Node::For { loc, .. }) => Some(loc.keyword.clone()),
//             (LocName::Keyword, Node::If { loc, .. }) => loc.keyword.clone(),
//             (LocName::Keyword, Node::IfMod { loc, .. }) => Some(loc.keyword.clone()),
//             (LocName::Keyword, Node::InPattern { loc, .. }) => Some(loc.keyword.clone()),
//             (LocName::Keyword, Node::Module { loc, .. }) => Some(loc.keyword.clone()),
//             (LocName::Keyword, Node::Next { loc, .. }) => Some(loc.keyword.clone()),
//             (LocName::Keyword, Node::Redo { loc, .. }) => Some(loc.keyword.clone()),
//             (LocName::Keyword, Node::Retry { loc, .. }) => Some(loc.keyword.clone()),
//             (LocName::Keyword, Node::Return { loc, .. }) => Some(loc.keyword.clone()),
//             (LocName::Keyword, Node::Sclass { loc, .. }) => Some(loc.keyword.clone()),
//             (LocName::Keyword, Node::Super { loc, .. }) => Some(loc.keyword.clone()),
//             (LocName::Keyword, Node::Undef { loc, .. }) => Some(loc.keyword.clone()),
//             (LocName::Keyword, Node::Until { loc, .. }) => Some(loc.keyword.clone()),
//             (LocName::Keyword, Node::UntilPost { loc, .. }) => Some(loc.keyword.clone()),
//             (LocName::Keyword, Node::While { loc, .. }) => Some(loc.keyword.clone()),
//             (LocName::Keyword, Node::WhilePost { loc, .. }) => Some(loc.keyword.clone()),
//             (LocName::Keyword, Node::Yield { loc, .. }) => Some(loc.keyword.clone()),
//             (LocName::Keyword, Node::Zsuper { loc, .. }) => Some(loc.keyword.clone()),
//             (LocName::Keyword, Node::Preexe { loc, .. }) => Some(loc.keyword.clone()),
//             (LocName::Keyword, Node::Postexe { loc, .. }) => Some(loc.keyword.clone()),

//             (LocName::End, Node::Array { loc, .. }) => loc.end.clone(),
//             (LocName::End, Node::ArrayPattern { loc, .. }) => loc.end.clone(),
//             (LocName::End, Node::ArrayPatternWithTail { loc, .. }) => loc.end.clone(),
//             (LocName::End, Node::Begin { loc, .. }) => loc.end.clone(),
//             (LocName::End, Node::Block { loc, .. }) => loc.end.clone(),
//             (LocName::End, Node::Case { loc, .. }) => loc.end.clone(),
//             (LocName::End, Node::CaseMatch { loc, .. }) => loc.end.clone(),
//             (LocName::End, Node::ConstPattern { loc, .. }) => loc.end.clone(),
//             (LocName::End, Node::Dstr { loc, .. }) => loc.end.clone(),
//             (LocName::End, Node::Dsym { loc, .. }) => loc.end.clone(),
//             (LocName::End, Node::FindPattern { loc, .. }) => loc.end.clone(),
//             (LocName::End, Node::For { loc, .. }) => Some(loc.end.clone()),
//             (LocName::End, Node::Hash { loc, .. }) => loc.end.clone(),
//             (LocName::End, Node::HashPattern { loc, .. }) => loc.end.clone(),
//             (LocName::End, Node::If { loc, .. }) => loc.end.clone(),
//             (LocName::End, Node::Index { loc, .. }) => Some(loc.end.clone()),
//             (LocName::End, Node::IndexAsgn { loc, .. }) => Some(loc.end.clone()),
//             (LocName::End, Node::KwBegin { loc, .. }) => loc.end.clone(),
//             (LocName::End, Node::Numblock { loc, .. }) => loc.end.clone(),
//             (LocName::End, Node::Postexe { loc, .. }) => loc.end.clone(),
//             (LocName::End, Node::Preexe { loc, .. }) => loc.end.clone(),
//             (LocName::End, Node::Regexp { loc, .. }) => loc.end.clone(),
//             (LocName::End, Node::Send { loc, .. }) => loc.end.clone(),
//             (LocName::End, Node::Str { loc, .. }) => loc.end.clone(),
//             (LocName::End, Node::Super { loc, .. }) => loc.end.clone(),
//             (LocName::End, Node::Sym { loc, .. }) => loc.end.clone(),
//             (LocName::End, Node::Until { loc, .. }) => loc.end.clone(),
//             (LocName::End, Node::UntilPost { loc, .. }) => loc.end.clone(),
//             (LocName::End, Node::While { loc, .. }) => loc.end.clone(),
//             (LocName::End, Node::WhilePost { loc, .. }) => loc.end.clone(),
//             (LocName::End, Node::Xstr { loc, .. }) => loc.end.clone(),
//             (LocName::End, Node::Yield { loc, .. }) => loc.end.clone(),
//             (LocName::End, Node::Class { loc, .. }) => Some(loc.end.clone()),
//             (LocName::End, Node::Def { loc, .. }) => loc.end.clone(),
//             (LocName::End, Node::Defs { loc, .. }) => loc.end.clone(),
//             (LocName::End, Node::Module { loc, .. }) => Some(loc.end.clone()),
//             (LocName::End, Node::Sclass { loc, .. }) => Some(loc.end.clone()),

//             (LocName::Name, Node::Casgn { loc, .. }) => Some(loc.name.clone()),
//             (LocName::Name, Node::Class { loc, .. }) => loc.name.clone(),
//             (LocName::Name, Node::Const { loc, .. }) => Some(loc.name.clone()),
//             (LocName::Name, Node::Cvar { loc, .. }) => loc.name.clone(),
//             (LocName::Name, Node::Def { loc, .. }) => Some(loc.name.clone()),
//             (LocName::Name, Node::Gvar { loc, .. }) => loc.name.clone(),
//             (LocName::Name, Node::Ivar { loc, .. }) => loc.name.clone(),
//             (LocName::Name, Node::Lvar { loc, .. }) => loc.name.clone(),
//             (LocName::Name, Node::MatchNilPattern { loc, .. }) => loc.name.clone(),
//             (LocName::Name, Node::MatchVar { loc, .. }) => loc.name.clone(),
//             (LocName::Name, Node::Module { loc, .. }) => loc.name.clone(),
//             (LocName::Name, Node::Cvasgn { loc, .. }) => loc.name.clone(),
//             (LocName::Name, Node::Gvasgn { loc, .. }) => loc.name.clone(),
//             (LocName::Name, Node::Ivasgn { loc, .. }) => loc.name.clone(),
//             (LocName::Name, Node::Lvasgn { loc, .. }) => loc.name.clone(),
//             (LocName::Name, Node::Defs { loc, .. }) => Some(loc.name.clone()),

//             (LocName::Begin, Node::InPattern { loc, .. }) => loc.begin.clone(),
//             (LocName::Begin, Node::For { loc, .. }) => Some(loc.begin.clone()),
//             (LocName::Begin, Node::If { loc, .. }) => loc.begin.clone(),
//             (LocName::Begin, Node::Str { loc, .. }) => loc.begin.clone(),
//             (LocName::Begin, Node::HashPattern { loc, .. }) => loc.begin.clone(),
//             (LocName::Begin, Node::Super { loc, .. }) => loc.begin.clone(),
//             (LocName::Begin, Node::KwBegin { loc, .. }) => loc.begin.clone(),
//             (LocName::Begin, Node::Regexp { loc, .. }) => loc.begin.clone(),
//             (LocName::Begin, Node::Block { loc, .. }) => loc.begin.clone(),
//             (LocName::Begin, Node::Array { loc, .. }) => loc.begin.clone(),
//             (LocName::Begin, Node::Hash { loc, .. }) => loc.begin.clone(),
//             (LocName::Begin, Node::Send { loc, .. }) => loc.begin.clone(),
//             (LocName::Begin, Node::While { loc, .. }) => loc.begin.clone(),
//             (LocName::Begin, Node::Postexe { loc, .. }) => loc.begin.clone(),
//             (LocName::Begin, Node::Preexe { loc, .. }) => loc.begin.clone(),
//             (LocName::Begin, Node::Until { loc, .. }) => loc.begin.clone(),
//             (LocName::Begin, Node::ConstPattern { loc, .. }) => loc.begin.clone(),
//             (LocName::Begin, Node::Xstr { loc, .. }) => loc.begin.clone(),
//             (LocName::Begin, Node::Dstr { loc, .. }) => loc.begin.clone(),
//             (LocName::Begin, Node::Index { loc, .. }) => Some(loc.begin.clone()),
//             (LocName::Begin, Node::Defined { loc, .. }) => loc.begin.clone(),
//             (LocName::Begin, Node::ArrayPattern { loc, .. }) => loc.begin.clone(),
//             (LocName::Begin, Node::ArrayPatternWithTail { loc, .. }) => loc.begin.clone(),
//             (LocName::Begin, Node::Numblock { loc, .. }) => loc.begin.clone(),
//             (LocName::Begin, Node::Sym { loc, .. }) => loc.begin.clone(),
//             (LocName::Begin, Node::Begin { loc, .. }) => loc.begin.clone(),
//             (LocName::Begin, Node::Yield { loc, .. }) => loc.begin.clone(),
//             (LocName::Begin, Node::FindPattern { loc, .. }) => loc.begin.clone(),
//             (LocName::Begin, Node::Dsym { loc, .. }) => loc.begin.clone(),
//             (LocName::Begin, Node::IndexAsgn { loc, .. }) => Some(loc.begin.clone()),

//             (LocName::Operator, Node::And { loc, .. }) => loc.operator.clone(),
//             (LocName::Operator, Node::AndAsgn { loc, .. }) => Some(loc.operator.clone()),
//             (LocName::Operator, Node::Class { loc, .. }) => loc.operator.clone(),
//             (LocName::Operator, Node::Cvasgn { loc, .. }) => loc.operator.clone(),
//             (LocName::Operator, Node::Defs { loc, .. }) => loc.operator.clone(),
//             (LocName::Operator, Node::Erange { loc, .. }) => loc.operator.clone(),
//             (LocName::Operator, Node::Float { loc, .. }) => loc.operator.clone(),
//             (LocName::Operator, Node::Gvasgn { loc, .. }) => loc.operator.clone(),
//             (LocName::Operator, Node::IndexAsgn { loc, .. }) => loc.operator.clone(),
//             (LocName::Operator, Node::Int { loc, .. }) => loc.operator.clone(),
//             (LocName::Operator, Node::Irange { loc, .. }) => loc.operator.clone(),
//             (LocName::Operator, Node::Ivasgn { loc, .. }) => loc.operator.clone(),
//             (LocName::Operator, Node::Lvasgn { loc, .. }) => loc.operator.clone(),
//             (LocName::Operator, Node::Masgn { loc, .. }) => loc.operator.clone(),
//             (LocName::Operator, Node::MatchAlt { loc, .. }) => loc.operator.clone(),
//             (LocName::Operator, Node::MatchAs { loc, .. }) => loc.operator.clone(),
//             (LocName::Operator, Node::MatchRest { loc, .. }) => loc.operator.clone(),
//             (LocName::Operator, Node::OpAsgn { loc, .. }) => Some(loc.operator.clone()),
//             (LocName::Operator, Node::Or { loc, .. }) => loc.operator.clone(),
//             (LocName::Operator, Node::OrAsgn { loc, .. }) => Some(loc.operator.clone()),
//             (LocName::Operator, Node::Pair { loc, .. }) => loc.operator.clone(),
//             (LocName::Operator, Node::Sclass { loc, .. }) => loc.operator.clone(),

//             (LocName::DoubleColon, Node::Casgn { loc, .. }) => loc.double_colon.clone(),
//             (LocName::DoubleColon, Node::Const { loc, .. }) => loc.double_colon.clone(),

//             (LocName::Else, Node::Case { loc, .. }) => loc.else_.clone(),
//             (LocName::Else, Node::CaseMatch { loc, .. }) => loc.else_.clone(),
//             (LocName::Else, Node::If { loc, .. }) => loc.else_.clone(),

//             (LocName::Colon, Node::IfTernary { loc, .. }) => Some(loc.colon.clone()),

//             (LocName::Selector, Node::CSend { loc, .. }) => loc.selector.clone(),
//             (LocName::Selector, Node::MatchWithLvasgn { loc, .. }) => loc.selector.clone(),
//             (LocName::Selector, Node::Pin { loc, .. }) => loc.selector.clone(),
//             (LocName::Selector, Node::Send { loc, .. }) => loc.selector.clone(),

//             (LocName::Assignment, Node::Def { loc, .. }) => loc.assignment.clone(),
//             (LocName::Assignment, Node::Defs { loc, .. }) => loc.assignment.clone(),

//             (LocName::HeredocBody, Node::Heredoc { loc, .. }) => Some(loc.heredoc_body.clone()),
//             (LocName::HeredocBody, Node::XHeredoc { loc, .. }) => Some(loc.heredoc_body.clone()),

//             (LocName::Expression, node) => Some(node.expression().clone()),

//             (name, node) => {
//                 panic!("node {} has no {:?} loc", node.str_type(), name);
//             }
//         };

//         match loc {
//             Some(loc) => {
//                 if self.begin != loc.begin_pos {
//                     return Err(format!(
//                         "{:?} has incorrect {:?} begin pos: expected {}, got {}",
//                         self.path, self.name, self.begin, loc.begin_pos
//                     ));
//                 }

//                 if self.end != loc.end_pos {
//                     return Err(format!(
//                         "{:?} has incorrect {:?} end pos: expected {}, got {}",
//                         self.path, self.name, self.begin, loc.begin_pos
//                     ));
//                 }
//             }
//             None => return Err(format!("{:?} has no {:?} loc", self.path, self.name)),
//         }

//         Ok(())
//     }
// }

// #[derive(Debug)]
// struct Path {
//     components: Vec<String>,
// }

// impl Path {
//     pub fn new(components: Vec<String>) -> Self {
//         Self { components }
//     }

//     pub fn go(&self, node: &Node) -> Option<Node> {
//         if self.components.is_empty() {
//             return Some(node.clone());
//         }

//         let first = self.components.first()?;
//         let rest = self.components[1..].to_owned();

//         let node: &Node = match (&first[..], node) {
//             ("args[0]", Node::Send { args, .. }) => &args[0],
//             ("args", Node::Def { args, .. }) => args.as_ref()?,
//             ("body", Node::Def { body, .. }) => body.as_ref()?,
//             ("body", Node::Postexe { body, .. }) => body.as_ref()?,
//             ("body", Node::Preexe { body, .. }) => body.as_ref()?,
//             ("body", Node::Rescue { body, .. }) => body.as_ref()?,
//             ("call", Node::Block { call, .. }) => call,
//             ("call", Node::Numblock { call, .. }) => call,
//             ("cond", Node::If { cond, .. }) => cond,
//             ("cond", Node::IfMod { cond, .. }) => cond,
//             ("expr", Node::Case { expr, .. }) => expr.as_ref()?,
//             ("expr", Node::CaseMatch { expr, .. }) => expr,
//             ("expr", Node::Until { cond, .. }) => cond,
//             ("expr", Node::UntilPost { cond, .. }) => cond,
//             ("expr", Node::While { cond, .. }) => cond,
//             ("expr", Node::WhilePost { cond, .. }) => cond,
//             ("item[0]", Node::Array { elements, .. }) => &elements[0],
//             ("item[0]", Node::Undef { names, .. }) => &names[0],
//             ("iterator", Node::For { iterator, .. }) => iterator,
//             ("left", Node::Erange { left, .. }) => left.as_ref()?,
//             ("right", Node::Erange { right, .. }) => right.as_ref()?,
//             ("left", Node::Irange { left, .. }) => left.as_ref()?,
//             ("right", Node::Irange { right, .. }) => right.as_ref()?,
//             ("lhs", Node::And { lhs, .. }) => lhs,
//             ("rhs", Node::And { rhs, .. }) => rhs,
//             ("lhs", Node::Or { lhs, .. }) => lhs,
//             ("rhs", Node::Or { rhs, .. }) => rhs,
//             ("name", Node::Class { name, .. }) => name,
//             ("body", Node::Class { body, .. }) => body.as_ref()?,
//             ("superclass", Node::Class { superclass, .. }) => superclass.as_ref()?,
//             ("name", Node::Module { name, .. }) => name,
//             ("body", Node::Module { body, .. }) => body.as_ref()?,
//             ("of", Node::Sclass { expr, .. }) => expr,
//             ("body", Node::Sclass { body, .. }) => body.as_ref()?,
//             ("pair[0]", Node::Hash { pairs, .. }) => &pairs[0],
//             ("part[0]", Node::Dstr { children, .. }) => &children[0],
//             ("part[0]", Node::Dsym { children, .. }) => &children[0],
//             ("part[0]", Node::Xstr { children, .. }) => &children[0],
//             ("re", Node::MatchWithLvasgn { receiver, .. }) => receiver,
//             ("recv", Node::AndAsgn { lhs, .. }) => lhs,
//             ("recv", Node::CSend { receiver, .. }) => receiver.as_ref()?,
//             ("recv", Node::Defs { definee, .. }) => definee,
//             ("recv", Node::Index { receiver, .. }) => receiver,
//             ("recv", Node::IndexAsgn { receiver, .. }) => receiver,
//             ("recv", Node::OpAsgn { lhs, .. }) => lhs,
//             ("recv", Node::OrAsgn { lhs, .. }) => lhs,
//             ("scope", Node::Casgn { scope, .. }) => scope.as_ref()?,
//             ("scope", Node::Const { scope, .. }) => scope.as_ref()?,
//             ("parts[0]", Node::Regexp { parts, .. }) => &parts[0],

//             ("args[0]", Node::Args { args, .. }) => &args[0],
//             ("args[0]", Node::CSend { args, .. }) => &args[0],
//             ("args[0]", Node::Send { args, .. }) => &args[0],
//             ("args", Node::Block { args, .. }) => args.as_ref()?,
//             ("args", Node::Defs { args, .. }) => args.as_ref()?,
//             ("body", Node::Block { body, .. }) => body.as_ref()?,
//             ("body", Node::Defs { body, .. }) => body.as_ref()?,
//             ("body", Node::Numblock { body, .. }) => body,
//             ("body", Node::Until { body, .. }) => body.as_ref()?,
//             ("body", Node::UntilPost { body, .. }) => body,
//             ("body", Node::While { body, .. }) => body.as_ref()?,
//             ("body", Node::WhilePost { body, .. }) => body,
//             ("falsey", Node::If { if_false, .. }) => if_false.as_ref()?,
//             ("falsey", Node::IfMod { if_false, .. }) => if_false.as_ref()?,
//             ("in_body[0]", Node::CaseMatch { in_bodies, .. }) => &in_bodies[0],
//             ("index[0]", Node::Index { indexes, .. }) => &indexes[0],
//             ("index[0]", Node::IndexAsgn { indexes, .. }) => &indexes[0],
//             ("item[0]", Node::Mlhs { items, .. }) => &items[0],
//             ("item[1]", Node::Array { elements, .. }) => &elements[0],
//             ("item[1]", Node::Undef { names, .. }) => &names[0],
//             ("iteratee", Node::For { iteratee, .. }) => iteratee,
//             ("key", Node::Pair { key, .. }) => key,
//             ("lhs", Node::Masgn { lhs, .. }) => lhs,
//             ("part[0]", Node::Regexp { parts, .. }) => &parts[0],
//             ("part[1]", Node::Dstr { children, .. }) => &children[1],
//             ("part[1]", Node::Dsym { children, .. }) => &children[1],
//             ("part[1]", Node::Xstr { children, .. }) => &children[1],
//             ("recv", Node::Send { receiver, .. }) => receiver.as_ref()?,
//             ("rescue_body[0]", Node::Rescue { rescue_bodies, .. }) => &rescue_bodies[0],
//             ("stmt[0]", Node::Begin { statements, .. }) => &statements[0],
//             ("stmt[0]", Node::KwBegin { statements, .. }) => &statements[0],
//             ("truthy", Node::If { if_true, .. }) => if_true.as_ref()?,
//             ("truthy", Node::IfMod { if_true, .. }) => if_true.as_ref()?,
//             ("value", Node::AndAsgn { rhs, .. }) => rhs,
//             ("value", Node::Break { .. }) => a,
//             ("value", Node::Casgn { .. }) => a,
//             ("value", Node::Cvasgn { .. }) => a,
//             ("value", Node::Defined { .. }) => a,
//             ("value", Node::Gvasgn { .. }) => a,
//             ("value", Node::IndexAsgn { .. }) => a,
//             ("value", Node::Ivasgn { .. }) => a,
//             ("value", Node::Lvasgn { .. }) => a,
//             ("value", Node::MatchCurrentLine { .. }) => a,
//             ("value", Node::Next { .. }) => a,
//             ("value", Node::OrAsgn { .. }) => a,
//             ("value", Node::Return { .. }) => a,
//             ("value", Node::Super { .. }) => a,
//             ("value", Node::Yield { .. }) => a,
//             ("when_body[0]", Node::Case { .. }) => a,

//             _ => panic!("can't go into {} on {:?}", first, node),
//         };

//         Path::new(rest).go(&node)
//     }
// }
