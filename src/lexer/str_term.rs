use std::rc::Rc;
use std::cell::RefCell;

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
struct InnerStringLiteral { // struct rb_strterm_literal_struct
    pub nest: usize,
    pub func: usize,
    pub paren: Option<u8>,
    pub term: u8,
}

#[derive(Debug, Clone, Default)]
pub struct StringLiteral {
    inner: Rc<RefCell<InnerStringLiteral>>
}

impl StringLiteral {
    pub fn new(nest: usize, func: usize, paren: Option<u8>, term: u8) -> Self {
        Self { inner: Rc::new(RefCell::new(InnerStringLiteral { nest, func, paren, term })) }
    }

    pub fn nest(&self) -> usize { self.inner.borrow().nest }
    pub fn func(&self) -> usize { self.inner.borrow().func }
    pub fn paren(&self) -> Option<u8> { self.inner.borrow().paren }
    pub fn term(&self) -> u8 { self.inner.borrow().term }

    pub fn set_nest(&self, nest: usize) { self.inner.borrow_mut().nest = nest; }
    pub fn set_func(&self, func: usize) { self.inner.borrow_mut().func = func; }
    pub fn set_paren(&self, paren: Option<u8>) { self.inner.borrow_mut().paren = paren; }
    pub fn set_term(&self, term: u8) { self.inner.borrow_mut().term = term; }
}

#[derive(Debug, Clone, Default)]
struct InnerHeredocLiteral {
    lastline: Vec<u8>,   /* the string of line that contains `<<"END"` */
    offset: usize,      /* the column of END in `<<"END"` */
    sourceline: usize,  /* lineno of the line that contains `<<"END"` */
    length: usize,      /* the length of END in `<<"END"` */

    quote: usize,
    func: usize,
}

#[derive(Debug, Clone, Default)]
pub struct HeredocLiteral {
    inner: Rc<RefCell<InnerHeredocLiteral>>
}

impl HeredocLiteral {
    pub fn new(lastline: Vec<u8>, offset: usize, sourceline: usize, length: usize, quote: usize, func: usize) -> Self {
        Self {
            inner: Rc::new(
                RefCell::new(
                    InnerHeredocLiteral {
                        lastline,
                        offset,
                        sourceline,
                        length,
                        quote,
                        func
                    }
                )
            )
        }
    }

    pub fn lastline(&self) -> Vec<u8> { self.inner.borrow().lastline.clone() }
    pub fn offset(&self) -> usize { self.inner.borrow().offset }
    pub fn sourceline(&self) -> usize { self.inner.borrow().sourceline }
    pub fn length(&self) -> usize { self.inner.borrow().length }
    pub fn quote(&self) -> usize { self.inner.borrow().quote }
    pub fn func(&self) -> usize { self.inner.borrow().func }

    pub fn set_lastline(&self, lastline: Vec<u8>) { self.inner.borrow_mut().lastline = lastline; }
    pub fn set_offset(&self, offset: usize) { self.inner.borrow_mut().offset = offset; }
    pub fn set_sourceline(&self, sourceline: usize) { self.inner.borrow_mut().sourceline = sourceline; }
    pub fn set_length(&self, length: usize) { self.inner.borrow_mut().length = length; }
    pub fn set_quote(&self, quote: usize) { self.inner.borrow_mut().quote = quote; }
    pub fn set_func(&self, func: usize) { self.inner.borrow_mut().func = func; }
}

#[derive(Debug, Clone)]
pub enum StrTerm { // struct rb_strterm_struct
    StringLiteral(StringLiteral),
    HeredocLiteral(HeredocLiteral)
}

impl StrTerm {
    pub fn new_literal(literal: StringLiteral) -> Self {
        Self::StringLiteral(literal)
    }

    pub fn new_heredoc(heredoc: HeredocLiteral) -> Self {
        Self::HeredocLiteral(heredoc)
    }
}
