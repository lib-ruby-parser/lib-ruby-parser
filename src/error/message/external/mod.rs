use crate::containers::size::DIAGNOSTIC_MESSAGE_SIZE;

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct DiagnosticMessageBlob {
    blob: [u8; DIAGNOSTIC_MESSAGE_SIZE],
}

/// Enum of all possible diagnostic message (both warnings and errors)
#[repr(C)]
pub struct DiagnosticMessage {
    pub(crate) blob: DiagnosticMessageBlob,
}

extern "C" {
    fn lib_ruby_parser__external__diagnostic_message__drop(blob: *mut DiagnosticMessageBlob);
}

impl Drop for DiagnosticMessage {
    fn drop(&mut self) {
        unsafe { lib_ruby_parser__external__diagnostic_message__drop(&mut self.blob) }
    }
}

mod constructors;
mod predicates;

mod impl_clone;
mod impl_debug;
mod impl_partial_eq;

impl Eq for DiagnosticMessage {}

mod variants;
