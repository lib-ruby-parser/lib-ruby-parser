#[derive(Debug, Clone)]
pub enum LexChar {
    Some(u8),
    EOF
}

const SPACE: u8 = ' ' as u8;
const TAB: u8 = '\t' as u8;
const VTAB: u8 = '\x0b' as u8;
const PUNCT: [u8; 21] = ['!' as u8, '"' as u8, '$' as u8, '&' as u8, '\'' as u8, '*' as u8, '+' as u8, ',' as u8, '.' as u8, '/' as u8, '0' as u8, ':' as u8, ';' as u8, '<' as u8, '=' as u8, '>' as u8, '?' as u8, '@' as u8, '\\' as u8, '`' as u8, '~' as u8];

impl LexChar {
    pub fn is_eof(&self) -> bool {
        self == &LexChar::EOF
    }

    pub fn unwrap(&self) -> u8 {
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

    pub fn to_option(&self) -> Option<u8> {
        if let LexChar::Some(c) = self { Some(c.clone()) } else { None }
    }
}

impl PartialEq<u8> for LexChar {
    fn eq(&self, other: &u8) -> bool {
        match self {
            LexChar::Some(charcode) => charcode == other,
            LexChar::EOF => false
        }
    }
}

impl PartialEq<Option<u8>> for LexChar {
    fn eq(&self, other: &Option<u8>) -> bool {
        match other {
            Some(c) => self == c,
            _ => false
        }
    }
}

impl PartialEq<LexChar> for u8 {
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

impl PartialOrd<u8> for LexChar {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        match self {
            LexChar::Some(charcode) => Some(charcode.cmp(other)),
            LexChar::EOF => Some(std::cmp::Ordering::Less)
        }
    }
}
