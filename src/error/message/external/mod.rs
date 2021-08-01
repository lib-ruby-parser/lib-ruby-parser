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

mod constructors;
mod getters;
mod predicates;

mod impl_clone;
mod impl_debug;
mod impl_drop;
mod impl_partial_eq;

impl Eq for DiagnosticMessage {}