#[cfg(feature = "compile-with-external-structures")]
pub(crate) mod external {
    use std::ops::Deref;

    use crate::containers::size::SHARED_BYTE_LIST_SIZE;

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct SharedByteListBlob {
        blob: [u8; SHARED_BYTE_LIST_SIZE],
    }

    /// C-compatible shared list
    #[repr(C)]
    pub struct SharedByteList {
        pub(crate) blob: SharedByteListBlob,
    }

    extern "C" {
        fn lib_ruby_parser__external__shared_byte_list__new(
            ptr: *const u8,
            len: u64,
        ) -> SharedByteListBlob;
        fn lib_ruby_parser__external__shared_byte_list__drop(blob: *mut SharedByteListBlob);
        fn lib_ruby_parser__external__shared_byte_list__get_raw(
            blob: *const SharedByteListBlob,
        ) -> *const u8;
        fn lib_ruby_parser__external__shared_byte_list__get_len(
            blob: *const SharedByteListBlob,
        ) -> u64;
    }

    impl Drop for SharedByteList {
        fn drop(&mut self) {
            unsafe { lib_ruby_parser__external__shared_byte_list__drop(&mut self.blob) }
        }
    }

    impl std::fmt::Debug for SharedByteList {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Debug::fmt(&**self, f)
        }
    }

    impl Deref for SharedByteList {
        type Target = [u8];

        fn deref(&self) -> &[u8] {
            let len = self.len();
            let ptr = if len == 0 {
                std::ptr::null()
            } else {
                self.as_ptr()
            };
            unsafe { std::slice::from_raw_parts(ptr, len) }
        }
    }

    impl SharedByteList {
        pub(crate) fn from_raw(ptr: *const u8, len: usize) -> Self {
            let blob = unsafe { lib_ruby_parser__external__shared_byte_list__new(ptr, len as u64) };
            Self { blob }
        }

        pub(crate) fn as_ptr(&self) -> *const u8 {
            unsafe { lib_ruby_parser__external__shared_byte_list__get_raw(&self.blob) }
        }

        /// Equivalent of std::slice::len
        pub fn len(&self) -> usize {
            unsafe { lib_ruby_parser__external__shared_byte_list__get_len(&self.blob) as usize }
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::containers::ExternalList;
        use std::ops::Deref;

        type ByteList = ExternalList<u8>;

        #[test]
        fn test_from_raw() {
            let mut list = ByteList::new();

            list.push(1);
            list.push(2);
            list.push(3);

            let shared = list.as_slice();

            assert_eq!(shared.len(), 3);
            assert_eq!(shared.as_ptr(), list.as_ptr());
            assert_eq!(shared.deref(), &[1, 2, 3]);
        }

        #[test]
        fn test_from_raw_empty() {
            let list = ByteList::new();
            let shared = list.as_slice();

            assert_eq!(shared.len(), 0);
            assert_eq!(shared.as_ptr(), std::ptr::null());
            assert_eq!(shared.deref(), &[])
        }
    }
}
