use crate::blobs::{Blob, HasBlob};
use crate::error::level::ErrorLevel;
use crate::error::message::DiagnosticMessage;
use crate::loc::Loc;

/// Diagnostic message that comes from the parser when there's an error or warning
#[repr(C)]
pub struct Diagnostic {
    pub(crate) blob: Blob<Diagnostic>,
}

extern "C" {
    fn lib_ruby_parser__external__diagnostic__new(
        level: Blob<ErrorLevel>,
        message: Blob<DiagnosticMessage>,
        loc: Blob<Loc>,
    ) -> Blob<Diagnostic>;
    fn lib_ruby_parser__external__diagnostic__drop(blob: *mut Blob<Diagnostic>);
    fn lib_ruby_parser__external__diagnostic__get_level(
        blob: *const Blob<Diagnostic>,
    ) -> *const Blob<ErrorLevel>;
    fn lib_ruby_parser__external__diagnostic__get_message(
        blob: *const Blob<Diagnostic>,
    ) -> *const Blob<DiagnosticMessage>;
    fn lib_ruby_parser__external__diagnostic__get_loc(
        blob: *const Blob<Diagnostic>,
    ) -> *const Blob<Loc>;
}

impl Diagnostic {
    /// Construncts an instance of `Diagnostic`
    pub fn new(level: ErrorLevel, message: DiagnosticMessage, loc: Loc) -> Self {
        let blob = unsafe {
            lib_ruby_parser__external__diagnostic__new(
                level.into_blob(),
                message.into_blob(),
                loc.into_blob(),
            )
        };
        Self { blob }
    }

    /// Returns `level` field
    pub fn level(&self) -> &ErrorLevel {
        unsafe {
            (lib_ruby_parser__external__diagnostic__get_level(&self.blob) as *const ErrorLevel)
                .as_ref()
                .unwrap()
        }
    }

    /// Returns `message` field
    pub fn message(&self) -> &DiagnosticMessage {
        unsafe {
            (lib_ruby_parser__external__diagnostic__get_message(&self.blob)
                as *const DiagnosticMessage)
                .as_ref()
                .unwrap()
        }
    }

    /// Returns `loc` field
    pub fn loc(&self) -> &Loc {
        unsafe {
            (lib_ruby_parser__external__diagnostic__get_loc(&self.blob) as *const Loc)
                .as_ref()
                .unwrap()
        }
    }
}

impl Clone for Diagnostic {
    fn clone(&self) -> Self {
        Self::new(
            self.level().clone(),
            self.message().clone(),
            self.loc().clone(),
        )
    }
}

impl std::fmt::Debug for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Diagnostic")
            .field("level", self.level())
            .field("message", self.message())
            .field("loc", self.loc())
            .finish()
    }
}

impl PartialEq for Diagnostic {
    fn eq(&self, other: &Self) -> bool {
        self.level() == other.level()
            && self.message() == other.message()
            && self.loc() == other.loc()
    }
}

impl Drop for Diagnostic {
    fn drop(&mut self) {
        unsafe { lib_ruby_parser__external__diagnostic__drop(&mut self.blob) }
    }
}
