#[cfg(test)]
pub(crate) mod dummy_decoder {
    use crate::source::{Decoder, DecoderResult, InputError};
    crate::use_native_or_external!(List);
    crate::use_native_or_external!(StringPtr);

    pub(crate) fn decoded_output() -> List<u8> {
        crate::use_native_or_external!(List);

        list![b'3', b'+', b'3']
    }

    pub(crate) fn decoding_error() -> InputError {
        InputError::new_decoding_error(StringPtr::from("foo"))
    }

    #[cfg(feature = "compile-with-external-structures")]
    mod implementation {
        use super::{decoded_output, decoding_error};
        use crate::blobs::{Blob, HasBlob};
        use crate::source::{Decoder, DecoderResult};

        type ExternDecodeFn = extern "C" fn() -> Blob<DecoderResult>;

        extern "C" {
            fn lib_ruby_parser__external__decoder__new(f: ExternDecodeFn) -> Blob<Decoder>;
        }

        extern "C" fn decode_ok() -> Blob<DecoderResult> {
            DecoderResult::new_ok(decoded_output()).into_blob()
        }

        extern "C" fn decode_err() -> Blob<DecoderResult> {
            DecoderResult::new_err(decoding_error()).into_blob()
        }

        pub(crate) fn ok_decoder() -> Decoder {
            Decoder::from_blob(unsafe { lib_ruby_parser__external__decoder__new(decode_ok) })
        }

        pub(crate) fn err_decoder() -> Decoder {
            Decoder::from_blob(unsafe { lib_ruby_parser__external__decoder__new(decode_err) })
        }
    }

    #[cfg(not(feature = "compile-with-external-structures"))]
    mod implementation {
        use super::{decoded_output, decoding_error};
        use crate::source::{Decoder, DecoderResult};

        fn decode_ok(_encoding: String, _input: Vec<u8>) -> DecoderResult {
            DecoderResult::Ok(decoded_output())
        }

        fn decode_err(_encoding: String, _input: Vec<u8>) -> DecoderResult {
            DecoderResult::Err(decoding_error())
        }

        pub(crate) fn ok_decoder() -> Decoder {
            Decoder::new(Box::new(decode_ok))
        }

        pub(crate) fn err_decoder() -> Decoder {
            Decoder::new(Box::new(decode_err))
        }
    }

    pub(crate) use implementation::{err_decoder, ok_decoder};

    pub(crate) fn call_dummy_decoder(decoder: Decoder) -> DecoderResult {
        // it's dummy, so encoding/input doesn't matter
        let encoding = StringPtr::from("UTF-8");
        let input = list![b'2', b'+', b'2'];

        decoder.call(encoding, input)
    }
}
