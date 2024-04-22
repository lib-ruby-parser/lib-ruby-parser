#[derive(Debug, Clone, Copy)]
pub(crate) enum MaybeByte {
    Some(u8),
    EndOfInput,
}

const PUNCT: [u8; 21] = [
    b'!', b'"', b'$', b'&', b'\'', b'*', b'+', b',', b'.', b'/', b'0', b':', b';', b'<', b'=',
    b'>', b'?', b'@', b'\\', b'`', b'~',
];

impl MaybeByte {
    pub(crate) fn is_eof(&self) -> bool {
        self == &MaybeByte::EndOfInput
    }

    pub(crate) fn is_some(&self) -> bool {
        self != &MaybeByte::EndOfInput
    }

    pub(crate) fn expect(&self, msg: &str) -> u8 {
        self.as_option().expect(msg)
    }

    pub(crate) fn as_option(&self) -> Option<u8> {
        match self {
            MaybeByte::Some(c) => Some(*c),
            _ => None,
        }
    }

    pub(crate) fn is_ascii(&self) -> bool {
        if let Some(c) = self.as_option() {
            c.is_ascii()
        } else {
            false
        }
    }

    pub(crate) fn is_alpha(&self) -> bool {
        if let Some(c) = self.as_option() {
            c.is_ascii_alphabetic()
        } else {
            false
        }
    }

    pub(crate) fn is_digit(&self) -> bool {
        if let Some(c) = self.as_option() {
            c.is_ascii_digit()
        } else {
            false
        }
    }

    pub(crate) fn is_alnum(&self) -> bool {
        if let Some(c) = self.as_option() {
            c.is_ascii_alphanumeric()
        } else {
            false
        }
    }

    pub(crate) fn is_hexdigit(&self) -> bool {
        if let Some(c) = self.as_option() {
            c.is_ascii_hexdigit()
        } else {
            false
        }
    }

    pub(crate) fn is_space(&self) -> bool {
        if let Some(c) = self.as_option() {
            c == b' ' || (b'\t'..=b'\r').contains(&c)
        } else {
            false
        }
    }

    pub(crate) fn is_global_name_punct(&self) -> bool {
        if let Some(c) = self.as_option() {
            PUNCT.contains(&c)
        } else {
            false
        }
    }

    pub(crate) fn is_control(&self) -> bool {
        if let Some(c) = self.as_option() {
            (c as char).is_control()
        } else {
            false
        }
    }

    pub(crate) fn map<F: FnOnce(u8) -> MaybeByte>(&self, f: F) -> MaybeByte {
        match self.as_option() {
            Some(c) => f(c),
            _ => MaybeByte::EndOfInput,
        }
    }

    pub(crate) fn escaped_control_code(&self) -> Option<u8> {
        if *self == b' ' {
            return Some(b's');
        }
        if *self == b'\n' {
            return Some(b'n');
        }
        if *self == b'\t' {
            return Some(b't');
        }
        if *self == 0x0b {
            return Some(b'v');
        }
        if *self == b'\r' {
            return Some(b'r');
        }
        if *self == 0x0c {
            return Some(b'f');
        }
        None
    }
}

pub(crate) trait MaybeByteNew<T> {
    fn new(c: T) -> Self;
}

impl MaybeByteNew<u8> for MaybeByte {
    fn new(byte: u8) -> Self {
        MaybeByte::Some(byte)
    }
}

impl PartialEq<u8> for MaybeByte {
    fn eq(&self, other: &u8) -> bool {
        match self {
            MaybeByte::Some(c) => c == other,
            MaybeByte::EndOfInput => false,
        }
    }
}

impl PartialEq<Option<u8>> for MaybeByte {
    fn eq(&self, other: &Option<u8>) -> bool {
        &self.as_option() == other
    }
}

impl PartialEq for MaybeByte {
    fn eq(&self, other: &MaybeByte) -> bool {
        self.as_option() == other.as_option()
    }
}

impl PartialOrd<u8> for MaybeByte {
    fn partial_cmp(&self, other: &u8) -> Option<core::cmp::Ordering> {
        match self.as_option() {
            Some(c) => Some(c.cmp(other)),
            _ => Some(core::cmp::Ordering::Less),
        }
    }
}
