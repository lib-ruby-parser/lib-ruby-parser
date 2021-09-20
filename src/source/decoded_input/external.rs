use crate::blobs::{Blob, HasBlob};
use crate::containers::{ExternalList as List, ExternalStringPtr as StringPtr};
use crate::source::SourceLine;

/// Decoded input
#[repr(C)]
pub struct DecodedInput {
    pub(crate) blob: Blob<DecodedInput>,
}

extern "C" {
    fn lib_ruby_parser__external__decoded_input__new(
        name: Blob<StringPtr>,
        lines: Blob<List<SourceLine>>,
        bytes: Blob<List<u8>>,
    ) -> Blob<DecodedInput>;
    fn lib_ruby_parser__external__decoded_input__drop(blob: *mut Blob<DecodedInput>);
    fn lib_ruby_parser__external__decoded_input__get_name(
        blob: *const Blob<DecodedInput>,
    ) -> *const Blob<StringPtr>;
    fn lib_ruby_parser__external__decoded_input__get_lines(
        blob: *const Blob<DecodedInput>,
    ) -> *const Blob<List<SourceLine>>;
    fn lib_ruby_parser__external__decoded_input__get_bytes(
        blob: *const Blob<DecodedInput>,
    ) -> *const Blob<List<u8>>;
    fn lib_ruby_parser__external__decoded_input__set_name(
        blob: *mut Blob<DecodedInput>,
        name: Blob<StringPtr>,
    );
    fn lib_ruby_parser__external__decoded_input__set_lines(
        blob: *mut Blob<DecodedInput>,
        lines: Blob<List<SourceLine>>,
    );
    fn lib_ruby_parser__external__decoded_input__set_bytes(
        blob: *mut Blob<DecodedInput>,
        bytes: Blob<List<u8>>,
    );
    fn lib_ruby_parser__external__decoded_input__into_bytes(
        blob: Blob<DecodedInput>,
    ) -> Blob<List<u8>>;
    fn lib_ruby_parser__external__decoded_input__take_bytes(
        blob: *mut Blob<DecodedInput>,
    ) -> Blob<List<u8>>;
}

impl Drop for DecodedInput {
    fn drop(&mut self) {
        unsafe { lib_ruby_parser__external__decoded_input__drop(&mut self.blob) }
    }
}

impl Default for DecodedInput {
    fn default() -> Self {
        Self::named(StringPtr::default())
    }
}

impl std::fmt::Debug for DecodedInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DecodedInput")
            .field("name", self.name())
            .field("lines", self.lines())
            .field("bytes", self.bytes())
            .finish()
    }
}

impl DecodedInput {
    /// Constructs empty `DecodedInput` with given name
    pub fn named<T: Into<StringPtr>>(name: T) -> Self {
        let name: StringPtr = name.into();

        let blob = unsafe {
            lib_ruby_parser__external__decoded_input__new(
                name.into_blob(),
                List::<SourceLine>::new().into_blob(),
                List::<u8>::new().into_blob(),
            )
        };
        Self { blob }
    }

    pub(crate) fn name(&self) -> &StringPtr {
        unsafe {
            (lib_ruby_parser__external__decoded_input__get_name(&self.blob) as *const StringPtr)
                .as_ref()
                .unwrap()
        }
    }

    pub(crate) fn lines(&self) -> &List<SourceLine> {
        unsafe {
            (lib_ruby_parser__external__decoded_input__get_lines(&self.blob)
                as *const List<SourceLine>)
                .as_ref()
                .unwrap()
        }
    }

    pub(crate) fn bytes(&self) -> &List<u8> {
        unsafe {
            (lib_ruby_parser__external__decoded_input__get_bytes(&self.blob) as *const List<u8>)
                .as_ref()
                .unwrap()
        }
    }

    #[allow(dead_code)]
    pub(crate) fn set_name(&mut self, name: StringPtr) {
        unsafe {
            lib_ruby_parser__external__decoded_input__set_name(&mut self.blob, name.into_blob())
        }
    }

    pub(crate) fn set_lines(&mut self, lines: List<SourceLine>) {
        unsafe {
            lib_ruby_parser__external__decoded_input__set_lines(&mut self.blob, lines.into_blob())
        }
    }

    pub(crate) fn set_bytes(&mut self, bytes: List<u8>) {
        unsafe {
            lib_ruby_parser__external__decoded_input__set_bytes(&mut self.blob, bytes.into_blob())
        }
    }

    pub(crate) fn take_bytes(&mut self) -> List<u8> {
        unsafe {
            List::from_blob(lib_ruby_parser__external__decoded_input__take_bytes(
                &mut self.blob,
            ))
        }
    }

    /// Converts itself into owned vector of bytes
    pub fn into_bytes(self) -> List<u8> {
        unsafe {
            List::from_blob(lib_ruby_parser__external__decoded_input__into_bytes(
                self.into_blob(),
            ))
        }
    }
}
