pub mod str_types {
    pub const STR_FUNC_ESCAPE: usize = 0x01;
    pub const STR_FUNC_EXPAND: usize = 0x02;
    pub const STR_FUNC_REGEXP: usize = 0x04;
    pub const STR_FUNC_QWORDS: usize = 0x08;
    pub const STR_FUNC_SYMBOL: usize = 0x10;
    pub const STR_FUNC_INDENT: usize = 0x20;
    pub const STR_FUNC_LABEL : usize = 0x40;
    pub const STR_FUNC_LIST  : usize = 0x4000;
    pub const STR_FUNC_TERM  : usize = 0x8000;

    pub const str_label : usize = STR_FUNC_LABEL;
    pub const str_squote: usize = 0;
    pub const str_dquote: usize = STR_FUNC_EXPAND;
    pub const str_xquote: usize = STR_FUNC_EXPAND;
    pub const str_regexp: usize = STR_FUNC_REGEXP|STR_FUNC_ESCAPE|STR_FUNC_EXPAND;
    pub const str_sword : usize = STR_FUNC_QWORDS|STR_FUNC_LIST;
    pub const str_dword : usize = STR_FUNC_QWORDS|STR_FUNC_EXPAND|STR_FUNC_LIST;
    pub const str_ssym  : usize = STR_FUNC_SYMBOL;
    pub const str_dsym  : usize = STR_FUNC_SYMBOL|STR_FUNC_EXPAND;
}

#[derive(Debug, Clone, Default)]
pub struct StringLiteral { // struct rb_strterm_literal_struct
    pub nest: usize,
    pub func: usize,
    pub paren: Option<char>,
    pub term: char,
}

#[derive(Debug, Clone, Default)]
pub struct HeredocLiteral {
    lastline: String,   /* the string of line that contains `<<"END"` */
    offset: usize,      /* the column of END in `<<"END"` */
    sourceline: usize,  /* lineno of the line that contains `<<"END"` */
    length: usize,      /* the length of END in `<<"END"` */

    quote: usize,
    func: usize,
}

#[derive(Debug, Clone)]
pub enum StrTerm { // struct rb_strterm_struct
    Literal(StringLiteral),
    Heredoc(HeredocLiteral)
}
