#include <stddef.h>
#include <stdlib.h>
#include "ptr.hpp"
#include "impl_blob.hpp"

IMPL_BLOB(PTR);

extern "C"
{
    PTR_BLOB_DATA lib_ruby_parser_containers_make_ptr_blob(void *raw) noexcept
    {
        return PACK(PTR((DUMMY_PTR_VALUE *)raw));
    }

    extern "C" void lib_ruby_parser_containers_free_ptr_blob(PTR_BLOB_DATA ptr_blob, DropPtrInPlace drop_ptr_in_place) noexcept
    {
        PTR ptr = UNPACK(ptr_blob);
        void *raw = ptr.release();
        if (raw)
        {
            drop_ptr_in_place(raw);
            free(raw);
        }
    }

    extern "C" void *lib_ruby_parser_containers_raw_ptr_from_ptr_blob(PTR_BLOB_DATA ptr_blob) noexcept
    {
        return UNPACK(ptr_blob).release();
    }

    extern "C" PTR_BLOB_DATA lib_ruby_parser_containers_null_ptr_blob() noexcept
    {
        return PACK(PTR(nullptr));
    }
}
