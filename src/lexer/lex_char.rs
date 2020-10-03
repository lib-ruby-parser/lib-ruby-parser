#[derive(Debug, Clone)]
pub enum LexChar {
    Some(char),
    EOF
}

const SPACE: char = ' ';
const TAB: char = '\t';
const VTAB: char = '\x0b';
const PUNCT: [char; 21] = ['!', '"', '$', '&', '\'', '*', '+', ',', '.', '/', '0', ':', ';', '<', '=', '>', '?', '@', '\\', '`', '~'];

impl LexChar {
    pub fn is_eof(&self) -> bool {
        self == &LexChar::EOF
    }

    pub fn unwrap(&self) -> char {
        match self {
            LexChar::Some(c) => *c,
            _ => panic!("LexChar is empty, can't unwrap()")
        }
    }

    pub fn is_ascii(&self) -> bool {
        if let LexChar::Some(c) = self { c.is_ascii() } else { false }
    }

    pub fn is_upper(&self) -> bool {
        if let LexChar::Some(c) = self { c.is_ascii_uppercase() } else { false }
    }

    pub fn is_lower(&self) -> bool {
        if let LexChar::Some(c) = self { c.is_ascii_lowercase() } else { false }
    }

    pub fn is_alpha(&self) -> bool {
        if let LexChar::Some(c) = self { c.is_ascii_alphabetic() } else { false }
    }

    pub fn is_digit(&self) -> bool {
        if let LexChar::Some(c) = self { c.is_ascii_digit() } else { false }
    }

    pub fn is_alnum(&self) -> bool {
        if let LexChar::Some(c) = self { c.is_ascii_alphanumeric() } else { false }
    }

    pub fn is_hexdigit(&self) -> bool {
        if let LexChar::Some(c) = self { c.is_ascii_hexdigit() } else { false }
    }

    pub fn is_blank(&self) -> bool {
        if let LexChar::Some(c) = self { *c == SPACE || *c == TAB } else { false }
    }

    pub fn is_space(&self) -> bool {
        if let LexChar::Some(c) = self { c.is_ascii_whitespace() || *c == VTAB } else { false }
    }

    pub fn is_global_name_punct(&self) -> bool {
        if let LexChar::Some(c) = self { PUNCT.contains(c) } else { false }
    }

    pub fn to_option(&self) -> Option<char> {
        if let LexChar::Some(c) = self { Some(*c) } else { None }
    }
}

impl PartialEq<char> for LexChar {
    fn eq(&self, other: &char) -> bool {
        match self {
            LexChar::Some(charcode) => charcode == other,
            LexChar::EOF => false
        }
    }
}

impl PartialEq<Option<char>> for LexChar {
    fn eq(&self, other: &Option<char>) -> bool {
        match other {
            Some(c) => self == c,
            _ => false
        }
    }
}

impl PartialEq<LexChar> for char {
    fn eq(&self, other: &LexChar) -> bool {
        match other {
            LexChar::Some(charcode) => charcode == self,
            LexChar::EOF => false
        }
    }
}

impl PartialEq for LexChar {
    fn eq(&self, other: &LexChar) -> bool {
        match (self, other) {
            (LexChar::Some(lhs), LexChar::Some(rhs)) => lhs == rhs,
            (LexChar::EOF, LexChar::EOF) => true,
            _ => false
        }
    }
}

impl PartialOrd<char> for LexChar {
    fn partial_cmp(&self, other: &char) -> Option<std::cmp::Ordering> {
        match self {
            LexChar::Some(charcode) => Some(charcode.cmp(other)),
            LexChar::EOF => Some(std::cmp::Ordering::Less)
        }
    }
}
