use super::shared::dummy_decoder::*;
use crate::source::DecoderResult;

#[test]
fn test_decoder_ok() {
    assert_eq!(
        call_dummy_decoder(ok_decoder()),
        DecoderResult::new_ok(decoded_output())
    );
}

#[test]
fn test_decoder_err() {
    assert_eq!(
        call_dummy_decoder(err_decoder()),
        DecoderResult::new_err(decoding_error())
    );
}
