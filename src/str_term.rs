pub(crate) mod str_types {
    pub(crate) const STR_FUNC_ESCAPE: usize = 0x01;
    pub(crate) const STR_FUNC_EXPAND: usize = 0x02;
    pub(crate) const STR_FUNC_REGEXP: usize = 0x04;
    pub(crate) const STR_FUNC_QWORDS: usize = 0x08;
    pub(crate) const STR_FUNC_SYMBOL: usize = 0x10;
    pub(crate) const STR_FUNC_INDENT: usize = 0x20;
    pub(crate) const STR_FUNC_LABEL: usize = 0x40;
    pub(crate) const STR_FUNC_LIST: usize = 0x4000;
    pub(crate) const STR_FUNC_TERM: usize = 0x8000;

    #[allow(non_upper_case_globals)]
    pub(crate) const str_label: usize = STR_FUNC_LABEL;
    #[allow(non_upper_case_globals)]
    pub(crate) const str_squote: usize = 0;
    #[allow(non_upper_case_globals)]
    pub(crate) const str_dquote: usize = STR_FUNC_EXPAND;
    #[allow(non_upper_case_globals)]
    pub(crate) const str_xquote: usize = STR_FUNC_EXPAND;
    #[allow(non_upper_case_globals)]
    pub(crate) const str_regexp: usize = STR_FUNC_REGEXP | STR_FUNC_ESCAPE | STR_FUNC_EXPAND;
    #[allow(non_upper_case_globals)]
    pub(crate) const str_sword: usize = STR_FUNC_QWORDS | STR_FUNC_LIST;
    #[allow(non_upper_case_globals)]
    pub(crate) const str_dword: usize = STR_FUNC_QWORDS | STR_FUNC_EXPAND | STR_FUNC_LIST;
    #[allow(non_upper_case_globals)]
    pub(crate) const str_ssym: usize = STR_FUNC_SYMBOL;
    #[allow(non_upper_case_globals)]
    pub(crate) const str_dsym: usize = STR_FUNC_SYMBOL | STR_FUNC_EXPAND;
}

#[derive(Debug, Clone, Default)]
pub(crate) struct HeredocEnd {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) value: Vec<u8>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct StringLiteral {
    // struct rb_strterm_literal_struct
    pub(crate) nest: usize,
    pub(crate) func: usize,
    pub(crate) paren: Option<u8>,
    pub(crate) term: u8,
    pub(crate) heredoc_end: Option<HeredocEnd>,
}

impl StringLiteral {
    pub(crate) fn new(
        nest: usize,
        func: usize,
        paren: Option<u8>,
        term: u8,
        heredoc_end: Option<HeredocEnd>,
    ) -> Self {
        Self {
            nest,
            func,
            paren,
            term,
            heredoc_end,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct HeredocLiteral {
    pub(crate) lastline: usize, /* the string of line that contains `<<"END"` */
    pub(crate) offset: usize,   /* the column of END in `<<"END"` */
    pub(crate) sourceline: usize, /* lineno of the line that contains `<<"END"` */
    pub(crate) length: usize,   /* the length of END in `<<"END"` */

    pub(crate) quote: usize,
    pub(crate) func: usize,
}

impl HeredocLiteral {
    pub(crate) fn new(
        lastline: usize,
        offset: usize,
        sourceline: usize,
        length: usize,
        quote: usize,
        func: usize,
    ) -> Self {
        Self {
            lastline,
            offset,
            sourceline,
            length,
            quote,
            func,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum StrTerm {
    // struct rb_strterm_struct
    StringLiteral(StringLiteral),
    HeredocLiteral(HeredocLiteral),
}

impl StrTerm {
    pub(crate) fn new_literal(literal: StringLiteral) -> Self {
        Self::StringLiteral(literal)
    }

    pub(crate) fn new_heredoc(heredoc: HeredocLiteral) -> Self {
        Self::HeredocLiteral(heredoc)
    }
}
