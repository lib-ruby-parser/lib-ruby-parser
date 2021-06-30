#include <stdlib.h>
#include "maybe_ptr.hpp"
#include "impl_blob.hpp"

// There's a conflict between PTR and MAYBE_PTR types
// as they are represented by the same data type
DECLARE_BLOB_PACK_FOR(MAYBE_PTR)
// PACK comes from unique_ptr
IMPL_BLOB_UNPACK(MAYBE_PTR)

extern "C"
{
    MAYBE_PTR_BLOB lib_ruby_parser__internal__containers__maybe_ptr__make(void *raw) noexcept
    {
        return PACK(MAYBE_PTR((DUMMY_MAYBE_PTR_VALUE *)raw));
    }

    void lib_ruby_parser__internal__containers__maybe_ptr__free(MAYBE_PTR_BLOB maybe_ptr_blob, DropPtrInPlace drop_ptr_in_place) noexcept
    {
        MAYBE_PTR ptr = UNPACK(maybe_ptr_blob);
        void *raw = ptr.release();
        if (raw)
        {
            drop_ptr_in_place(raw);
            free(raw);
        }
    }

    void *lib_ruby_parser__internal__containers__maybe_ptr__get_raw(MAYBE_PTR_BLOB maybe_ptr_blob) noexcept
    {
        return UNPACK(maybe_ptr_blob).release();
    }

    MAYBE_PTR_BLOB lib_ruby_parser__internal__containers__maybe_ptr__make_null() noexcept
    {
        return PACK(MAYBE_PTR(nullptr));
    }
}
