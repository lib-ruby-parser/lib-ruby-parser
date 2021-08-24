use super::DecoderResult;

#[cfg(feature = "compile-with-external-structures")]
use crate::containers::ExternalList;
#[cfg(feature = "compile-with-external-structures")]
type List<T> = ExternalList<T>;
#[cfg(not(feature = "compile-with-external-structures"))]
type List<T> = Vec<T>;

#[cfg(feature = "compile-with-external-structures")]
use crate::containers::ExternalStringPtr;
#[cfg(feature = "compile-with-external-structures")]
type StringPtr = ExternalStringPtr;
#[cfg(not(feature = "compile-with-external-structures"))]
type StringPtr = String;

use crate::source::InputError;

#[test]
fn test_ok() {
    let output = List::<u8>::from(vec![1, 2, 3]);
    let ok_result = DecoderResult::new_ok(output.clone());

    assert!(ok_result.is_ok());
    assert!(!ok_result.is_err());

    assert_eq!(ok_result.as_ok(), &output);
    assert_eq!(format!("{:?}", ok_result), "Ok([1, 2, 3])");
    assert_eq!(ok_result.into_result(), Ok(output));
}

#[test]
fn test_err() {
    let err = InputError::new_unsupported_encoding(StringPtr::from("foo"));
    let err_result = DecoderResult::new_err(err.clone());

    assert!(!err_result.is_ok());
    assert!(err_result.is_err());

    assert_eq!(err_result.as_err(), &err);
    assert_eq!(
        format!("{:?}", err_result),
        "Err(UnsupportedEncoding(\"foo\"))"
    );
    assert_eq!(err_result.into_result(), Err(err));
}
