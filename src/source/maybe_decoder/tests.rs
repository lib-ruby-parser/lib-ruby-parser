use super::{MaybeDecoder, MaybeDecoderAPI};
use crate::source::Decoder;

#[cfg(feature = "compile-with-external-structures")]
fn decoder() -> Decoder {
    use crate::blobs::{Blob, HasBlob};
    Decoder::from_blob(Blob::<Decoder>::zeroed())
}

#[cfg(not(feature = "compile-with-external-structures"))]
fn decoder() -> Decoder {
    use crate::source::DecoderResult;
    Decoder::new(Box::new(|_encoding: String, _input: Vec<u8>| {
        DecoderResult::Ok(b"# encoding: us-ascii\ndecoded".to_vec().into())
    }))
}

#[test]
fn test_some() {
    let decoder = decoder();
    let maybe_decoder = MaybeDecoder::new_some(decoder);

    assert!(maybe_decoder.is_some());
    assert!(!maybe_decoder.is_none());
}

#[test]
fn test_none() {
    let maybe_decoder = MaybeDecoder::new_none();

    assert!(maybe_decoder.is_none());
    assert!(!maybe_decoder.is_some());
}
