crate::use_native_or_external!(List);
use super::Bytes;

#[test]
fn test_new() {
    let bytes = Bytes::new(list![1, 2, 3]);
    drop(bytes);
}

#[test]
fn test_as_raw() {
    let bytes = Bytes::new(list![1, 2, 3]);

    assert_eq!(bytes.as_raw(), &list![1, 2, 3])
}

#[test]
fn test_into_raw() {
    let bytes = Bytes::new(list![1, 2, 3]);

    assert_eq!(bytes.into_raw(), list![1, 2, 3])
}

#[test]
fn test_set_raw() {
    let mut bytes = Bytes::new(list![1, 2, 3]);
    bytes.set_raw(list![4, 5, 6]);

    assert_eq!(bytes.as_raw(), &list![4, 5, 6])
}

#[test]
fn test_push() {
    let mut bytes = Bytes::default();
    for i in 0..10 {
        bytes.push(i);
    }
    assert_eq!(bytes.as_raw(), &list![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
}
