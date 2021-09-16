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

    // MaybePtr<T>
    (MaybePtr) => {
        #[cfg(feature = "compile-with-external-structures")]
        use $crate::containers::ExternalMaybePtr;
        #[cfg(feature = "compile-with-external-structures")]
        type MaybePtr<T> = ExternalMaybePtr<T>;
        #[cfg(not(feature = "compile-with-external-structures"))]
        type MaybePtr<T> = Option<Box<T>>;

        #[allow(unused_imports)]
        use crate::containers::helpers::MaybePtrAPI;
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

    // MaybeStringPtr
    (MaybeStringPtr) => {
        #[cfg(feature = "compile-with-external-structures")]
        use $crate::containers::ExternalMaybeStringPtr;
        #[cfg(feature = "compile-with-external-structures")]
        type MaybeStringPtr = ExternalMaybeStringPtr;
        #[cfg(not(feature = "compile-with-external-structures"))]
        type MaybeStringPtr = Option<String>;

        #[allow(unused_imports)]
        use crate::containers::helpers::MaybeStringPtrAPI;
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

    // MaybeLoc
    (MaybeLoc) => {
        #[cfg(feature = "compile-with-external-structures")]
        use $crate::containers::ExternalMaybeLoc;
        #[cfg(feature = "compile-with-external-structures")]
        type MaybeLoc = ExternalMaybeLoc;
        #[cfg(not(feature = "compile-with-external-structures"))]
        #[allow(unused_qualifications)]
        type MaybeLoc = Option<crate::Loc>;

        #[allow(unused_imports)]
        use crate::containers::helpers::MaybeLocAPI;
    };
}

pub(crate) use use_native_or_external;
