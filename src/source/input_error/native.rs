/// An enum with all possible kinds of errors that can be returned
/// from a decoder
#[derive(Debug)]
#[repr(C)]
pub enum InputError {
    /// Emitted when no custom decoder provided but input has custom encoding.
    ///
    /// You can return this error from your custom decoder if you don't support given encoding.
    UnsupportedEncoding(String),

    /// Generic error that can be emitted from a custom decoder
    DecodingError(String),
}
