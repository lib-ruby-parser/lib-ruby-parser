use crate::source::InputError;

/// Result that is returned from decoding function
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DecoderResult {
    /// Ok + decoded bytes
    Ok(Vec<u8>),

    /// Err + reason
    Err(InputError),
}

impl DecoderResult {
    /// Constructs `Ok` variant
    pub fn new_ok(output: Vec<u8>) -> Self {
        Self::Ok(output)
    }

    /// Constructs `Err` variant
    pub fn new_err(err: InputError) -> Self {
        Self::Err(err)
    }

    pub(crate) fn into_result(self) -> Result<Vec<u8>, InputError> {
        match self {
            Self::Ok(value) => Ok(value),
            Self::Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
impl DecoderResult {
    pub(crate) fn is_ok(&self) -> bool {
        matches!(self, Self::Ok(_))
    }

    pub(crate) fn is_err(&self) -> bool {
        matches!(self, Self::Err(_))
    }

    pub(crate) fn as_ok(&self) -> &Vec<u8> {
        match &self {
            Self::Ok(ok) => ok,
            Self::Err(_) => panic!("DecoderResult is Err"),
        }
    }

    pub(crate) fn as_err(&self) -> &InputError {
        match &self {
            Self::Err(err) => err,
            Self::Ok(_) => panic!("DecoderResult is Ok"),
        }
    }
}
