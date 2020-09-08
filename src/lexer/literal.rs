use crate::lexer::Lexer;
use std::collections::HashMap;
use regex::Regex;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum StartToken {
    tSTRING_BEG,
    tQWORDS_BEG,
    tWORDS_BEG,
    tQSYMBOLS_BEG,
    tSYMBOLS_BEG,
    tSYMBEG,
    tREGEXP_BEG,
    tXSTRING_BEG
}

lazy_static! {
    static ref DELIMITERS: HashMap<&'static str, &'static str> =
        [
            ("(", ")"),
            ("[", "]"),
            ("{", "}"),
            ("<", ">"),
        ].iter().cloned().collect();

    static ref TYPES: HashMap<&'static str, (StartToken, bool)> =
        [
            ("'",    (StartToken::tSTRING_BEG,   false) ),
            ("<<'",  (StartToken::tSTRING_BEG,   false) ),
            ("%q",   (StartToken::tSTRING_BEG,   false) ),
            ("\"",   (StartToken::tSTRING_BEG,   true ) ),
            ("<<\"", (StartToken::tSTRING_BEG,   true ) ),
            ("%",    (StartToken::tSTRING_BEG,   true ) ),
            ("%Q",   (StartToken::tSTRING_BEG,   true ) ),

            ("%w",   (StartToken::tQWORDS_BEG,   false) ),
            ("%W",   (StartToken::tWORDS_BEG,    true ) ),

            ("%i",   (StartToken::tQSYMBOLS_BEG, false) ),
            ("%I",   (StartToken::tSYMBOLS_BEG,  true ) ),

            (":'",   (StartToken::tSYMBEG,       false) ),
            ("%s",   (StartToken::tSYMBEG,       false) ),
            (":\"",  (StartToken::tSYMBEG,       true ) ),

            ("/",    (StartToken::tREGEXP_BEG,   true ) ),
            ("%r",   (StartToken::tREGEXP_BEG,   true ) ),

            ("%x",   (StartToken::tXSTRING_BEG,  true ) ),
            ("`",    (StartToken::tXSTRING_BEG,  true ) ),
            ("<<`",  (StartToken::tXSTRING_BEG,  true ) ),
        ].iter().cloned().collect();

    static ref MUNGE_ESCAPE_REGEX: Regex = Regex::new(r"/[ \t\v\r\f\n]/").unwrap();
}

pub struct Literal<'a> {
    lexer: &'a mut Lexer,
    nesting: usize,
    str_type: String,
    str_s: usize,
    start_tok: StartToken,
    interpolate: bool,
    start_delim: Option<&'static str>,
    end_delim: &'a str,
    heredoc_e: Option<usize>,
    indent: bool,
    label_allowed: bool,
    dedent_body: bool,
    dedent_level: Option<usize>,
    interp_braces: usize,
    space_emitted: bool,
    monolithic: bool,

    buffer_s: Option<usize>,
    buffer_e: Option<usize>,
    buffer: String
}


impl<'a> Literal<'a> {
    pub fn new(lexer: &'a mut Lexer, str_type: &str, delimiter: &'a str, str_s: usize, heredoc_e: Option<usize>, indent: bool, dedent_body: bool, label_allowed: bool) -> Option<Self> {
        let (start_tok, interpolate) = TYPES.get(str_type).map(&|entry: &(StartToken, bool)| entry.clone() )?;
        let start_delim: Option<&'static str> = DELIMITERS.get(delimiter).map(|v| *v);
        let end_delim: &'a str = DELIMITERS.get(delimiter).map(|v| *v).unwrap_or(delimiter);
        let monolithic = (start_tok == StartToken::tSTRING_BEG) && (str_type == "'" || str_type == "\"") && heredoc_e.is_none();

        let str_type =
            if str_type.starts_with("%") {
                format!("{}{}", str_type, delimiter)
            } else {
                str_type.into()
            };

        let mut literal = Self {
            lexer,
            nesting: 1,
            str_type: str_type.into(),
            str_s,
            start_tok,
            interpolate,
            start_delim,
            end_delim,
            heredoc_e,
            indent,
            label_allowed,
            dedent_body,
            dedent_level: None,
            interp_braces: 0,
            space_emitted: true,
            monolithic,

            buffer_s: None,
            buffer_e: None,
            buffer: "".into()
        };

        literal.clear_buffer();

        if !monolithic {
            literal.emit_start_tok();
        }

        Some(literal)
    }

    pub fn is_interpolate(&self) -> bool {
        self.interpolate
    }

    pub fn is_words(&self) -> bool {
        self.type_() == &StartToken::tWORDS_BEG ||
            self.type_() == &StartToken::tQWORDS_BEG ||
            self.type_() == &StartToken::tSYMBOLS_BEG ||
            self.type_() == &StartToken::tQSYMBOLS_BEG
    }

    pub fn is_regexp(&self) -> bool {
        self.type_() == &StartToken::tREGEXP_BEG
    }

    pub fn is_heredoc(&self) -> bool {
        self.heredoc_e.is_some()
    }

    pub fn is_plain_heredoc(&self) -> bool {
        self.is_heredoc() && !self.dedent_body
    }

    pub fn is_squiggly_heredoc(&self) -> bool {
        self.is_heredoc() && self.dedent_body
    }

    pub fn is_backslash_delimited(&self) -> bool {
        self.end_delim == "\\"
    }

    pub fn type_(&self) -> &StartToken {
        &self.start_tok
    }

    pub fn is_munge_escape(&self, character: &str) -> bool {
        if self.is_words() && MUNGE_ESCAPE_REGEX.is_match(character) {
            true
        } else {
            character == "\\" ||
                Some(character) == self.start_delim ||
                character == self.end_delim
        }
    }

    pub fn nest_and_try_closing(&mut self, delimiter: &str, ts: usize, te: usize, lookahead: Option<(char, char)>) {
        if let Some(start_delim) = self.start_delim {
            if start_delim == delimiter {
                self.nesting += 1;
            }
        } else if self.is_delimiter(delimiter) {
            self.nesting -= 1;
        }

        // Finalize if last matching delimiter is closed.
        if self.nesting == 0 {
            if self.is_words() {
                self.extend_space(ts, te);
            }

            if let Some((l1, l2)) = lookahead {
                if self.label_allowed && l1 == ':' && l2 == ':' && self.start_tok == StartToken::tSTRING_BEG {
                    // This is a quoted label.
                    self.flush_string();
                    self.emit("tLABEL_END", self.end_delim, ts, te + 1);
                }
            } else if self.monolithic {
                // Emit the string as a single token.
                self.emit("tSTRING", self.end_delim, self.str_s, te);
            } else {
                // If this is a heredoc, self.buffer contains the sentinel now.
                // Just throw it out. Lexer flushes the heredoc after each
                // non-heredoc-terminating \n anyway, so no data will be lost.
                if !self.is_heredoc() {
                    self.flush_string()
                }

                self.emit("tSTRING_END", self.end_delim, ts, te)
            }
        }
    }

    pub fn infer_indent_level(&mut self, line: &str) {
        unimplemented!("infer_indent_level {:#?} {}", self.dedent_level, line)
    }

    pub fn start_interp_brace(&mut self) {
        self.interp_braces += 1;
    }

    pub fn end_interp_brace_and_try_closing(&mut self) -> bool {
        self.interp_braces -= 1;

        self.interp_braces == 0
    }

    pub fn extend_string(&mut self, string: &str, ts: usize, te: usize) {
        if self.buffer_s.is_none() {
            self.buffer_s = Some(ts);
        }

        self.buffer_e = Some(te);
        self.buffer.push_str(string);
    }

    pub fn flush_string(&mut self) {
        if self.monolithic {
            self.emit_start_tok();
            self.monolithic = false;
        }

        match (self.buffer_s, self.buffer_e) {
            (Some(buffer_s), Some(buffer_e)) => {
                self.emit("tSTRING_CONTENT", &self.buffer.clone(), buffer_s, buffer_e);

                self.clear_buffer();
                self.extend_content();
            },
            (_, _) => {}
        }
    }

    pub fn extend_content(&mut self) {
        self.space_emitted = false;
    }

    pub fn extend_space(&mut self, ts: usize, te: usize) {
        self.flush_string();

        if !self.space_emitted {
            self.emit("tSPACE", "nil", ts, te);

            self.space_emitted = true;
        }
    }

    pub fn supports_line_continuation_via_slash(&self) -> bool {
        !self.is_words() && self.interpolate
    }

    fn is_delimiter(&self, delimiter: &str) -> bool {
        if self.indent {
            self.end_delim == delimiter.chars().skip_while(|c| c.is_whitespace()).collect::<String>()
        } else {
            self.end_delim == delimiter
        }
    }

    fn clear_buffer(&mut self) {
        self.buffer = "".into();

        self.buffer_s = None;
        self.buffer_e = None;
    }

    fn emit_start_tok(&mut self) {
        let str_e = self.heredoc_e.unwrap_or(self.str_s + self.str_type.len());
        self.emit(&format!("{:#?}", self.start_tok), &self.str_type.clone(), self.str_s, str_e)
    }

    fn emit(&mut self, token: &str, token_type: &str, s: usize, e: usize) {
        self.lexer.emit(token, token_type, s, e)
    }
}
