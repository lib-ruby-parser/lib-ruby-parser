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
    // struct rb_strterm_literal_struct
    pub(crate) nest: Rc<RefCell<usize>>,
    pub(crate) func: Rc<RefCell<usize>>,
    pub(crate) paren: Rc<RefCell<Option<u8>>>,
    pub(crate) term: Rc<RefCell<u8>>,
    pub(crate) heredoc_end: Rc<RefCell<Option<HeredocEnd>>>,
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
            nest: Rc::new(RefCell::new(nest)),
            func: Rc::new(RefCell::new(func)),
            paren: Rc::new(RefCell::new(paren)),
            term: Rc::new(RefCell::new(term)),
            heredoc_end: Rc::new(RefCell::new(heredoc_end)),
        }
    }

    pub(crate) fn nest(&self) -> usize {
        *self.nest.borrow()
    }
    pub(crate) fn func(&self) -> usize {
        *self.func.borrow()
    }
    pub(crate) fn paren(&self) -> Option<u8> {
        *self.paren.borrow()
    }
    pub(crate) fn term(&self) -> u8 {
        *self.term.borrow()
    }

    pub(crate) fn set_nest(&self, nest: usize) {
        *self.nest.borrow_mut() = nest;
    }
    pub(crate) fn set_func(&self, func: usize) {
        *self.func.borrow_mut() = func;
    }
    #[allow(dead_code)]
    pub(crate) fn set_paren(&self, paren: Option<u8>) {
        *self.paren.borrow_mut() = paren;
    }
    #[allow(dead_code)]
    pub(crate) fn set_term(&self, term: u8) {
        *self.term.borrow_mut() = term;
    }

    pub(crate) fn heredoc_end(&self) -> Option<HeredocEnd> {
        self.heredoc_end.borrow().clone()
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct HeredocLiteral {
    lastline: Rc<RefCell<usize>>, /* the string of line that contains `<<"END"` */
    offset: Rc<RefCell<usize>>,   /* the column of END in `<<"END"` */
    sourceline: Rc<RefCell<usize>>, /* lineno of the line that contains `<<"END"` */
    length: Rc<RefCell<usize>>,   /* the length of END in `<<"END"` */

    quote: Rc<RefCell<usize>>,
    func: Rc<RefCell<usize>>,
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
            lastline: Rc::new(RefCell::new(lastline)),
            offset: Rc::new(RefCell::new(offset)),
            sourceline: Rc::new(RefCell::new(sourceline)),
            length: Rc::new(RefCell::new(length)),
            quote: Rc::new(RefCell::new(quote)),
            func: Rc::new(RefCell::new(func)),
        }
    }

    pub(crate) fn lastline(&self) -> usize {
        *self.lastline.borrow()
    }
    pub(crate) fn offset(&self) -> usize {
        *self.offset.borrow()
    }
    pub(crate) fn sourceline(&self) -> usize {
        *self.sourceline.borrow()
    }
    pub(crate) fn length(&self) -> usize {
        *self.length.borrow()
    }
    pub(crate) fn quote(&self) -> usize {
        *self.quote.borrow()
    }
    pub(crate) fn func(&self) -> usize {
        *self.func.borrow()
    }

    #[allow(dead_code)]
    pub(crate) fn set_lastline(&self, lastline: usize) {
        *self.lastline.borrow_mut() = lastline;
    }
    #[allow(dead_code)]
    pub(crate) fn set_offset(&self, offset: usize) {
        *self.offset.borrow_mut() = offset;
    }
    #[allow(dead_code)]
    pub(crate) fn set_sourceline(&self, sourceline: usize) {
        *self.sourceline.borrow_mut() = sourceline;
    }
    #[allow(dead_code)]
    pub(crate) fn set_length(&self, length: usize) {
        *self.length.borrow_mut() = length;
    }
    #[allow(dead_code)]
    pub(crate) fn set_quote(&self, quote: usize) {
        *self.quote.borrow_mut() = quote;
    }
    #[allow(dead_code)]
    pub(crate) fn set_func(&self, func: usize) {
        *self.func.borrow_mut() = func;
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
