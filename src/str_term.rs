use crate::source::SourceLine;

pub(crate) mod str_types {
    pub(crate) const STR_FUNC_ESCAPE: u32 = 0x01;
    pub(crate) const STR_FUNC_EXPAND: u32 = 0x02;
    pub(crate) const STR_FUNC_REGEXP: u32 = 0x04;
    pub(crate) const STR_FUNC_QWORDS: u32 = 0x08;
    pub(crate) const STR_FUNC_SYMBOL: u32 = 0x10;
    pub(crate) const STR_FUNC_INDENT: u32 = 0x20;
    pub(crate) const STR_FUNC_LABEL: u32 = 0x40;
    pub(crate) const STR_FUNC_LIST: u32 = 0x4000;
    pub(crate) const STR_FUNC_TERM: u32 = 0x8000;

    #[allow(non_upper_case_globals)]
    pub(crate) const str_label: u32 = STR_FUNC_LABEL;
    #[allow(non_upper_case_globals)]
    pub(crate) const str_squote: u32 = 0;
    #[allow(non_upper_case_globals)]
    pub(crate) const str_dquote: u32 = STR_FUNC_EXPAND;
    #[allow(non_upper_case_globals)]
    pub(crate) const str_xquote: u32 = STR_FUNC_EXPAND;
    #[allow(non_upper_case_globals)]
    pub(crate) const str_regexp: u32 = STR_FUNC_REGEXP | STR_FUNC_ESCAPE | STR_FUNC_EXPAND;
    #[allow(non_upper_case_globals)]
    pub(crate) const str_sword: u32 = STR_FUNC_QWORDS | STR_FUNC_LIST;
    #[allow(non_upper_case_globals)]
    pub(crate) const str_dword: u32 = STR_FUNC_QWORDS | STR_FUNC_EXPAND | STR_FUNC_LIST;
    #[allow(non_upper_case_globals)]
    pub(crate) const str_ssym: u32 = STR_FUNC_SYMBOL;
    #[allow(non_upper_case_globals)]
    pub(crate) const str_dsym: u32 = STR_FUNC_SYMBOL | STR_FUNC_EXPAND;
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct HeredocEnd<'b> {
    pub(crate) start: u32,
    pub(crate) end: u32,
    pub(crate) value: &'b [u8],
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct StringLiteral<'b> {
    // struct rb_strterm_literal_struct
    pub(crate) nest: u32,
    pub(crate) func: u32,
    pub(crate) paren: Option<u8>,
    pub(crate) term: u8,
    pub(crate) heredoc_end: Option<HeredocEnd<'b>>,
}

impl<'b> StringLiteral<'b> {
    pub(crate) fn new(
        nest: u32,
        func: u32,
        paren: Option<u8>,
        term: u8,
        heredoc_end: Option<HeredocEnd<'b>>,
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

#[derive(Debug, Clone, Copy)]
pub(crate) struct HeredocLiteral<'b> {
    pub(crate) lastline: &'b SourceLine, /* the string of line that contains `<<"END"` */
    pub(crate) offset: u32,              /* the column of END in `<<"END"` */
    pub(crate) sourceline: u32,          /* lineno of the line that contains `<<"END"` */
    pub(crate) length: u32,              /* the length of END in `<<"END"` */

    pub(crate) quote: u32,
    pub(crate) func: u32,
}

impl<'b> HeredocLiteral<'b> {
    pub(crate) fn new(
        lastline: &'b SourceLine,
        offset: u32,
        sourceline: u32,
        length: u32,
        quote: u32,
        func: u32,
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

#[derive(Debug, Clone, Copy)]
pub(crate) enum StrTerm<'b> {
    // struct rb_strterm_struct
    StringLiteral(StringLiteral<'b>),
    HeredocLiteral(HeredocLiteral<'b>),
}

impl<'b> StrTerm<'b> {
    pub(crate) fn new_literal(literal: StringLiteral<'b>) -> Self {
        Self::StringLiteral(literal)
    }

    pub(crate) fn new_heredoc(heredoc: HeredocLiteral<'b>) -> Self {
        Self::HeredocLiteral(heredoc)
    }
}
