macro_rules! use_native_or_external {
    // Ptr<T>
    (Ptr) => {
        #[cfg(feature = "compile-with-external-structures")]
        use $crate::containers::ExternalPtr;
        #[cfg(feature = "compile-with-external-structures")]
        type Ptr<T> = ExternalPtr<T>;
        #[cfg(not(feature = "compile-with-external-structures"))]
        type Ptr<T> = Box<T>;

        #[allow(unused_imports)]
        use crate::containers::helpers::PtrAPI;
    };

    // Maybe<T>
    (Maybe) => {
        #[cfg(feature = "compile-with-external-structures")]
        use $crate::containers::ExternalMaybe;
        #[cfg(feature = "compile-with-external-structures")]
        type Maybe<T> = ExternalMaybe<T>;
        #[cfg(not(feature = "compile-with-external-structures"))]
        type Maybe<T> = Option<T>;

        #[allow(unused_imports)]
        use crate::containers::helpers::MaybeAPI;
    };

    // StringPtr
    (StringPtr) => {
        #[cfg(feature = "compile-with-external-structures")]
        use $crate::containers::ExternalStringPtr;
        #[cfg(feature = "compile-with-external-structures")]
        type StringPtr = ExternalStringPtr;
        #[cfg(not(feature = "compile-with-external-structures"))]
        type StringPtr = String;
    };

    // List<T>
    (List) => {
        // Sometimes List type is used, sometimes macro is better.
        // Because of that one of them can be unused depending on a usage pattern.
        // Thus, both are marked as "potentially unused"
        #[cfg(feature = "compile-with-external-structures")]
        use $crate::containers::ExternalList;
        #[cfg(feature = "compile-with-external-structures")]
        #[allow(dead_code)]
        type List<T> = ExternalList<T>;
        #[cfg(feature = "compile-with-external-structures")]
        #[allow(unused_imports)]
        use crate::containers::{list, list_count};

        #[cfg(not(feature = "compile-with-external-structures"))]
        #[allow(dead_code)]
        type List<T> = Vec<T>;
        #[cfg(not(feature = "compile-with-external-structures"))]
        #[allow(unused_imports)]
        use std::vec as list;

        #[allow(unused_imports)]
        use crate::containers::helpers::ListAPI;
    };

    // SharedByteList
    (SharedByteList) => {
        #[cfg(feature = "compile-with-external-structures")]
        use $crate::containers::ExternalSharedByteList;
        #[cfg(feature = "compile-with-external-structures")]
        type SharedByteList = ExternalSharedByteList;
        #[cfg(not(feature = "compile-with-external-structures"))]
        type SharedByteList<'a> = &'a [u8];
    };
}

pub(crate) use use_native_or_external;
