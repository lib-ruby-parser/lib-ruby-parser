use crate::containers::size::DECODED_INPUT_SIZE;
use crate::containers::{
    ExternalList as List, ExternalStringPtr as StringPtr, IntoBlob, ListBlob, StringPtrBlob,
};
use crate::source::SourceLine;

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct DecodedInputBlob {
    blob: [u8; DECODED_INPUT_SIZE],
}

/// Decoded input
#[repr(C)]
pub struct DecodedInput {
    pub(crate) blob: DecodedInputBlob,
}

extern "C" {
    fn lib_ruby_parser__internal__containers__decoded_input__new(
        name: StringPtrBlob,
        lines: ListBlob,
        bytes: ListBlob,
    ) -> DecodedInputBlob;
    fn lib_ruby_parser__internal__containers__decoded_input__drop(blob: *mut DecodedInputBlob);
    fn lib_ruby_parser__internal__containers__decoded_input__get_name(
        blob: *const DecodedInputBlob,
    ) -> *const StringPtrBlob;
    fn lib_ruby_parser__internal__containers__decoded_input__get_lines(
        blob: *const DecodedInputBlob,
    ) -> *const ListBlob;
    fn lib_ruby_parser__internal__containers__decoded_input__get_bytes(
        blob: *const DecodedInputBlob,
    ) -> *const ListBlob;
    fn lib_ruby_parser__internal__containers__decoded_input__set_name(
        blob: *mut DecodedInputBlob,
        name: StringPtrBlob,
    );
    fn lib_ruby_parser__internal__containers__decoded_input__set_lines(
        blob: *mut DecodedInputBlob,
        lines: ListBlob,
    );
    fn lib_ruby_parser__internal__containers__decoded_input__set_bytes(
        blob: *mut DecodedInputBlob,
        bytes: ListBlob,
    );
    fn lib_ruby_parser__internal__containers__decoded_input__into_bytes(
        blob: DecodedInputBlob,
    ) -> ListBlob;
    fn lib_ruby_parser__internal__containers__decoded_input__take_bytes(
        blob: *const DecodedInputBlob,
    ) -> ListBlob;
}

impl Drop for DecodedInput {
    fn drop(&mut self) {
        unsafe { lib_ruby_parser__internal__containers__decoded_input__drop(&mut self.blob) }
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
            lib_ruby_parser__internal__containers__decoded_input__new(
                name.into_blob(),
                List::<SourceLine>::new().into_blob(),
                List::<u8>::new().into_blob(),
            )
        };
        Self { blob }
    }

    pub(crate) fn name(&self) -> &StringPtr {
        unsafe {
            (lib_ruby_parser__internal__containers__decoded_input__get_name(&self.blob)
                as *const StringPtr)
                .as_ref()
                .unwrap()
        }
    }

    pub(crate) fn lines(&self) -> &List<SourceLine> {
        unsafe {
            (lib_ruby_parser__internal__containers__decoded_input__get_lines(&self.blob)
                as *const List<SourceLine>)
                .as_ref()
                .unwrap()
        }
    }

    pub(crate) fn bytes(&self) -> &List<u8> {
        unsafe {
            (lib_ruby_parser__internal__containers__decoded_input__get_bytes(&self.blob)
                as *const List<u8>)
                .as_ref()
                .unwrap()
        }
    }

    #[allow(dead_code)]
    pub(crate) fn set_name(&mut self, name: StringPtr) {
        unsafe {
            lib_ruby_parser__internal__containers__decoded_input__set_name(
                &mut self.blob,
                name.into_blob(),
            )
        }
    }

    pub(crate) fn set_lines(&mut self, lines: List<SourceLine>) {
        unsafe {
            lib_ruby_parser__internal__containers__decoded_input__set_lines(
                &mut self.blob,
                lines.into_blob(),
            )
        }
    }

    pub(crate) fn set_bytes(&mut self, bytes: List<u8>) {
        unsafe {
            lib_ruby_parser__internal__containers__decoded_input__set_bytes(
                &mut self.blob,
                bytes.into_blob(),
            )
        }
    }

    pub(crate) fn take_bytes(&mut self) -> List<u8> {
        unsafe {
            List::<u8>::from_blob(
                lib_ruby_parser__internal__containers__decoded_input__take_bytes(&mut self.blob),
            )
        }
    }

    /// Converts itself into owned vector of bytes
    pub fn into_bytes(self) -> List<u8> {
        let bytes = unsafe {
            List::<u8>::from_blob(
                lib_ruby_parser__internal__containers__decoded_input__into_bytes(self.blob),
            )
        };
        std::mem::forget(self);
        bytes
    }
}
