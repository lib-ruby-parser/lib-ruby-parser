#include <stdlib.h>
#include "maybe_ptr.hpp"
#include "impl_blob.hpp"

// PACK comes from unique_ptr
IMPL_BLOB_UNPACK(MAYBE_PTR)

extern "C"
{
    MAYBE_PTR_BLOB lib_ruby_parser_containers_make_maybe_ptr_blob(void *raw) noexcept
    {
        return PACK(MAYBE_PTR((DUMMY_MAYBE_PTR_VALUE *)raw));
    }

    void lib_ruby_parser_containers_free_maybe_ptr_blob(MAYBE_PTR_BLOB maybe_ptr_blob, DropPtrInPlace drop_ptr_in_place) noexcept
    {
        MAYBE_PTR ptr = UNPACK(maybe_ptr_blob);
        void *raw = ptr.release();
        if (raw)
        {
            drop_ptr_in_place(raw);
            free(raw);
        }
    }

    void *lib_ruby_parser_containers_raw_ptr_from_maybe_ptr_blob(MAYBE_PTR_BLOB maybe_ptr_blob) noexcept
    {
        return UNPACK(maybe_ptr_blob).release();
    }

    MAYBE_PTR_BLOB lib_ruby_parser_containers_null_maybe_ptr_blob() noexcept
    {
        return PACK(MAYBE_PTR(nullptr));
    }
}
