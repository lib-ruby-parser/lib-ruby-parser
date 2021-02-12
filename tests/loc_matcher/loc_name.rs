#[derive(Debug)]
pub enum LocName {
    Begin,
    End,
    Expression,
    Keyword,
    Name,
    Assignment,
    Colon,
    DoubleColon,
    Else,
    HeredocBody,
    Operator,
    Selector,
    Assoc,
    Question,
    HeredocEnd,
}

impl LocName {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        match &name[..] {
            "begin" => LocName::Begin,
            "end" => LocName::End,
            "expression" => LocName::Expression,
            "keyword" => LocName::Keyword,
            "name" => LocName::Name,
            "assignment" => LocName::Assignment,
            "colon" => LocName::Colon,
            "double_colon" => LocName::DoubleColon,
            "else" => LocName::Else,
            "heredoc_body" => LocName::HeredocBody,
            "operator" => LocName::Operator,
            "selector" => LocName::Selector,
            "assoc" => LocName::Assoc,
            "question" => LocName::Question,
            "heredoc_end" => LocName::HeredocEnd,
            _ => panic!("unsupported loc name {}", name),
        }
    }
}
