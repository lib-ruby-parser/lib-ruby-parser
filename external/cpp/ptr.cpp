#include <stddef.h>
#include <stdlib.h>
#include "ptr.hpp"
#include "impl_blob.hpp"

IMPL_BLOB(PTR);

extern "C"
{
    PTR_BLOB lib_ruby_parser__internal__containers__ptr__make(void *raw) noexcept
    {
        return PACK(PTR((DUMMY_PTR_VALUE *)raw));
    }

    extern "C" void lib_ruby_parser__internal__containers__ptr__free(PTR_BLOB ptr_blob, DropPtrInPlace drop_ptr_in_place) noexcept
    {
        PTR ptr = UNPACK(ptr_blob);
        void *raw = ptr.release();
        if (raw)
        {
            drop_ptr_in_place(raw);
            free(raw);
        }
    }

    extern "C" void *lib_ruby_parser__internal__containers__ptr__get_raw(PTR_BLOB ptr_blob) noexcept
    {
        return UNPACK(ptr_blob).release();
    }

    extern "C" PTR_BLOB lib_ruby_parser__internal__containers__ptr__make_null() noexcept
    {
        return PACK(PTR(nullptr));
    }
}
