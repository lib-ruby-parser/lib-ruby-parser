#[derive(Debug, Clone)]
pub(crate) enum MaybeByte {
    Some(u8),
    EndOfInput,
}

const SPACE: u8 = b' ';
const TAB: u8 = b'\t';
const VTAB: u8 = b'\x0b';
const PUNCT: [u8; 21] = [
    b'!', b'"', b'$', b'&', b'\'', b'*', b'+', b',', b'.', b'/', b'0', b':', b';', b'<', b'=',
    b'>', b'?', b'@', b'\\', b'`', b'~',
];

impl MaybeByte {
    pub(crate) fn is_eof(&self) -> bool {
        self == &MaybeByte::EndOfInput
    }

    pub(crate) fn unwrap(&self) -> u8 {
        match self {
            MaybeByte::Some(byte) => *byte,
            _ => panic!("MaybeByte::EndOfInput has no bytes"),
        }
    }

    pub(crate) fn to_option(&self) -> Option<u8> {
        match self {
            MaybeByte::Some(c) => Some(*c),
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
            c == b' ' || (b'\t' <= c && c <= b'\r')
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
            (c as char).is_control()
        } else {
            false
        }
    }

    #[allow(dead_code)]
    pub(crate) fn map<F: FnOnce(u8) -> MaybeByte>(&self, f: F) -> MaybeByte {
        match self.to_option() {
            Some(c) => f(c),
            _ => MaybeByte::EndOfInput,
        }
    }
}

pub(crate) trait MaybeByteNew<T> {
    fn new(c: T) -> Self;
}

impl MaybeByteNew<char> for MaybeByte {
    fn new(c: char) -> Self {
        if c.len_utf8() > 1 {
            unreachable!("Can't construct MaybeByte from a multibyte char {:?}", c)
        }
        MaybeByte::Some(c as u8)
    }
}

impl MaybeByteNew<u8> for MaybeByte {
    fn new(byte: u8) -> Self {
        MaybeByte::Some(byte)
    }
}

impl PartialEq<u8> for MaybeByte {
    fn eq(&self, other: &u8) -> bool {
        match self.to_option() {
            Some(c) => c == *other,
            _ => false,
        }
    }
}

impl PartialEq<char> for MaybeByte {
    fn eq(&self, other: &char) -> bool {
        if other.len_utf8() > 1 {
            return false;
        }
        match self.to_option() {
            Some(c) => c == *other as u8,
            _ => false,
        }
    }
}

impl PartialEq<Option<u8>> for MaybeByte {
    fn eq(&self, other: &Option<u8>) -> bool {
        &self.to_option() == other
    }
}

impl PartialEq for MaybeByte {
    fn eq(&self, other: &MaybeByte) -> bool {
        self.to_option() == other.to_option()
    }
}

impl PartialOrd<u8> for MaybeByte {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        match self.to_option() {
            Some(c) => Some(c.cmp(other)),
            _ => Some(std::cmp::Ordering::Less),
        }
    }
}

impl PartialOrd<char> for MaybeByte {
    fn partial_cmp(&self, other: &char) -> Option<std::cmp::Ordering> {
        if other.len_utf8() > 1 {
            unreachable!("can't compare byte and multibyte char");
        }

        match self.to_option() {
            Some(c) => Some(c.cmp(&(*other as u8))),
            _ => Some(std::cmp::Ordering::Less),
        }
    }
}
