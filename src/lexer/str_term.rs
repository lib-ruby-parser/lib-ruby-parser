#[derive(Debug, Clone)]
pub struct StringLiteral { // struct rb_strterm_literal_struct
    nest: Option<usize>,
    func: Option<usize>,
    paren: Option<usize>,
    term: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct HeredocLiteral {
    lastline: String,   /* the string of line that contains `<<"END"` */
    offset: usize,      /* the column of END in `<<"END"` */
    sourceline: usize,  /* lineno of the line that contains `<<"END"` */
    length: usize,      /* the length of END in `<<"END"` */

    quote: usize,
    func: usize,
}

pub enum StrTerm { // struct rb_strterm_struct
    Literal(StringLiteral),
    Heredoc(HeredocLiteral)
}
