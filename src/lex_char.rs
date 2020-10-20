#[derive(Debug, Clone)]
pub(crate) enum LexChar {
    Multibyte(char),
    AsciiByte(char),
    NonUtf8Byte(u8),
    EOF,
}

const SPACE: char = ' ';
const TAB: char = '\t';
const VTAB: char = '\x0b';
const PUNCT: [char; 21] = [
    '!', '"', '$', '&', '\'', '*', '+', ',', '.', '/', '0', ':', ';', '<', '=', '>', '?', '@',
    '\\', '`', '~',
];

impl LexChar {
    pub(crate) fn is_eof(&self) -> bool {
        self == &LexChar::EOF
    }

    pub(crate) fn unwrap(&self) -> char {
        match self {
            LexChar::Multibyte(c) | LexChar::AsciiByte(c) => *c,
            LexChar::NonUtf8Byte(_) => panic!("LexChar is non-utf8, can't turn it into a char"),
            _ => panic!("LexChar is empty, can't unwrap()"),
        }
    }

    pub(crate) fn to_option(&self) -> Option<char> {
        match self {
            LexChar::Multibyte(c) | LexChar::AsciiByte(c) => Some(*c),
            _ => None,
        }
    }

    pub(crate) fn is_ascii(&self) -> bool {
        if let Some(c) = self.to_option() {
            c.is_ascii()
        } else {
            false
        }
    }

    #[allow(dead_code)]
    pub(crate) fn is_upper(&self) -> bool {
        if let Some(c) = self.to_option() {
            c.is_ascii_uppercase()
        } else {
            false
        }
    }

    #[allow(dead_code)]
    pub(crate) fn is_lower(&self) -> bool {
        if let Some(c) = self.to_option() {
            c.is_ascii_lowercase()
        } else {
            false
        }
    }

    pub(crate) fn is_alpha(&self) -> bool {
        if let Some(c) = self.to_option() {
            c.is_ascii_alphabetic()
        } else {
            false
        }
    }

    pub(crate) fn is_digit(&self) -> bool {
        if let Some(c) = self.to_option() {
            c.is_ascii_digit()
        } else {
            false
        }
    }

    pub(crate) fn is_alnum(&self) -> bool {
        if let Some(c) = self.to_option() {
            c.is_ascii_alphanumeric()
        } else {
            false
        }
    }

    pub(crate) fn is_hexdigit(&self) -> bool {
        if let Some(c) = self.to_option() {
            c.is_ascii_hexdigit()
        } else {
            false
        }
    }

    #[allow(dead_code)]
    pub(crate) fn is_blank(&self) -> bool {
        if let Some(c) = self.to_option() {
            c == SPACE || c == TAB
        } else {
            false
        }
    }

    pub(crate) fn is_space(&self) -> bool {
        if let Some(c) = self.to_option() {
            c.is_ascii_whitespace() || c == VTAB
        } else {
            false
        }
    }

    pub(crate) fn is_global_name_punct(&self) -> bool {
        if let Some(c) = self.to_option() {
            PUNCT.contains(&c)
        } else {
            false
        }
    }

    pub(crate) fn is_control(&self) -> bool {
        if let Some(c) = self.to_option() {
            c.is_control()
        } else {
            false
        }
    }

    #[allow(dead_code)]
    pub(crate) fn map<F: FnOnce(char) -> LexChar>(&self, f: F) -> LexChar {
        match self.to_option() {
            Some(c) => f(c),
            _ => LexChar::EOF,
        }
    }

    pub(crate) fn map_as_u8<F: FnOnce(u8) -> u8>(&self, f: F) -> LexChar {
        match &self {
            LexChar::Multibyte(_) => {
                unreachable!("applying bitmask to multibyte char");
            }
            LexChar::NonUtf8Byte(c) => LexChar::new(f(*c)),
            LexChar::AsciiByte(c) => LexChar::new(f(*c as u8)),
            LexChar::EOF => LexChar::EOF,
        }
    }
}

pub(crate) trait LexCharNew<T> {
    fn new(c: T) -> Self;
}

impl LexCharNew<char> for LexChar {
    fn new(c: char) -> Self {
        match c.len_utf8() {
            1 => {
                let byte = c as u8;
                if byte <= 127 {
                    LexChar::AsciiByte(c)
                } else {
                    LexChar::NonUtf8Byte(byte)
                }
            }
            _ => LexChar::Multibyte(c),
        }
    }
}

impl LexCharNew<u8> for LexChar {
    fn new(byte: u8) -> Self {
        if byte <= 127 {
            LexChar::AsciiByte(byte as char)
        } else {
            LexChar::NonUtf8Byte(byte)
        }
    }
}

impl PartialEq<char> for LexChar {
    fn eq(&self, other: &char) -> bool {
        match self.to_option() {
            Some(c) => c == *other,
            _ => false,
        }
    }
}

impl PartialEq<Option<char>> for LexChar {
    fn eq(&self, other: &Option<char>) -> bool {
        &self.to_option() == other
    }
}

impl PartialEq for LexChar {
    fn eq(&self, other: &LexChar) -> bool {
        self.to_option() == other.to_option()
    }
}

impl PartialOrd<char> for LexChar {
    fn partial_cmp(&self, other: &char) -> Option<std::cmp::Ordering> {
        match self.to_option() {
            Some(c) => Some(c.cmp(other)),
            _ => Some(std::cmp::Ordering::Less),
        }
    }
}
