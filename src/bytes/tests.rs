#[cfg(feature = "compile-with-external-structures")]
use crate::containers::ExternalList;
#[cfg(feature = "compile-with-external-structures")]
type List<T> = ExternalList<T>;
#[cfg(not(feature = "compile-with-external-structures"))]
type List<T> = Vec<T>;

use super::Bytes;

#[test]
fn test_new() {
    let bytes = Bytes::new(vec![1, 2, 3]);
    drop(bytes);
}

#[test]
fn test_as_raw() {
    let bytes = Bytes::new(vec![1, 2, 3]);

    assert_eq!(bytes.as_raw(), &[1, 2, 3])
}

#[test]
fn test_into_raw() {
    let bytes = Bytes::new(vec![1, 2, 3]);

    assert_eq!(bytes.into_raw(), List::<u8>::from(vec![1, 2, 3]))
}

#[test]
fn test_set_raw() {
    let mut bytes = Bytes::new(vec![1, 2, 3]);
    bytes.set_raw(vec![4, 5, 6].into());

    assert_eq!(bytes.as_raw(), &[4, 5, 6])
}

#[test]
fn test_push() {
    let mut bytes = Bytes::default();
    for i in 0..10 {
        bytes.push(i);
    }
    assert_eq!(bytes.as_raw(), &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
}
