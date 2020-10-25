use std::cell::RefCell;
use std::rc::Rc;

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
    pub(crate) value: String,
}

#[derive(Debug, Clone, Default)]
struct InnerStringLiteral {
    // struct rb_strterm_literal_struct
    pub(crate) nest: usize,
    pub(crate) func: usize,
    pub(crate) paren: Option<u8>,
    pub(crate) term: u8,
    pub(crate) heredoc_end: Option<HeredocEnd>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct StringLiteral {
    inner: Rc<RefCell<InnerStringLiteral>>,
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
            inner: Rc::new(RefCell::new(InnerStringLiteral {
                nest,
                func,
                paren,
                term,
                heredoc_end,
            })),
        }
    }

    pub(crate) fn nest(&self) -> usize {
        self.inner.borrow().nest
    }
    pub(crate) fn func(&self) -> usize {
        self.inner.borrow().func
    }
    pub(crate) fn paren(&self) -> Option<u8> {
        self.inner.borrow().paren
    }
    pub(crate) fn term(&self) -> u8 {
        self.inner.borrow().term
    }

    pub(crate) fn set_nest(&self, nest: usize) {
        self.inner.borrow_mut().nest = nest;
    }
    pub(crate) fn set_func(&self, func: usize) {
        self.inner.borrow_mut().func = func;
    }
    #[allow(dead_code)]
    pub(crate) fn set_paren(&self, paren: Option<u8>) {
        self.inner.borrow_mut().paren = paren;
    }
    #[allow(dead_code)]
    pub(crate) fn set_term(&self, term: u8) {
        self.inner.borrow_mut().term = term;
    }

    pub(crate) fn heredoc_end(&self) -> Option<HeredocEnd> {
        self.inner.borrow().heredoc_end.clone()
    }
}

#[derive(Debug, Clone, Default)]
struct InnerHeredocLiteral {
    lastline: usize,   /* the string of line that contains `<<"END"` */
    offset: usize,     /* the column of END in `<<"END"` */
    sourceline: usize, /* lineno of the line that contains `<<"END"` */
    length: usize,     /* the length of END in `<<"END"` */

    quote: usize,
    func: usize,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct HeredocLiteral {
    inner: Rc<RefCell<InnerHeredocLiteral>>,
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
            inner: Rc::new(RefCell::new(InnerHeredocLiteral {
                lastline,
                offset,
                sourceline,
                length,
                quote,
                func,
            })),
        }
    }

    pub(crate) fn lastline(&self) -> usize {
        self.inner.borrow().lastline.clone()
    }
    pub(crate) fn offset(&self) -> usize {
        self.inner.borrow().offset
    }
    pub(crate) fn sourceline(&self) -> usize {
        self.inner.borrow().sourceline
    }
    pub(crate) fn length(&self) -> usize {
        self.inner.borrow().length
    }
    pub(crate) fn quote(&self) -> usize {
        self.inner.borrow().quote
    }
    pub(crate) fn func(&self) -> usize {
        self.inner.borrow().func
    }

    #[allow(dead_code)]
    pub(crate) fn set_lastline(&self, lastline: usize) {
        self.inner.borrow_mut().lastline = lastline;
    }
    #[allow(dead_code)]
    pub(crate) fn set_offset(&self, offset: usize) {
        self.inner.borrow_mut().offset = offset;
    }
    #[allow(dead_code)]
    pub(crate) fn set_sourceline(&self, sourceline: usize) {
        self.inner.borrow_mut().sourceline = sourceline;
    }
    #[allow(dead_code)]
    pub(crate) fn set_length(&self, length: usize) {
        self.inner.borrow_mut().length = length;
    }
    #[allow(dead_code)]
    pub(crate) fn set_quote(&self, quote: usize) {
        self.inner.borrow_mut().quote = quote;
    }
    #[allow(dead_code)]
    pub(crate) fn set_func(&self, func: usize) {
        self.inner.borrow_mut().func = func;
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
