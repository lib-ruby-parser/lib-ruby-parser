#include <stddef.h>
#include <stdlib.h>
#include "ptr.hpp"
#include "impl_blob.hpp"

IMPL_BLOB(PTR);

extern "C"
{
    PTR_BLOB_DATA lib_ruby_parser_containers_make_ptr_blob(void *ptr) noexcept
    {
        PTR_BLOB_UNION u = {.as_value = std::unique_ptr<DUMMY_PTR_VALUE>((DUMMY_PTR_VALUE *)ptr)};
        PTR_BLOB_DATA result = u.as_blob;
        u.as_value.release(); // prevent running destructor
        return result;
    }

    extern "C" void lib_ruby_parser_containers_free_ptr_blob(PTR_BLOB_DATA blob, DropPtrInPlace drop_ptr_in_place) noexcept
    {
        PTR_BLOB_UNION u = {.as_blob = blob};
        void *raw = u.as_value.release();
        if (raw)
        {
            drop_ptr_in_place(raw);
            free(raw);
        }
    }

    extern "C" void *lib_ruby_parser_containers_raw_ptr_from_ptr_blob(PTR_BLOB_DATA blob) noexcept
    {
        PTR_BLOB_UNION u = {.as_blob = blob};
        return u.as_value.release();
    }

    extern "C" PTR_BLOB_DATA lib_ruby_parser_containers_null_ptr_blob() noexcept
    {
        PTR_BLOB_UNION u = {.as_value = std::unique_ptr<DUMMY_PTR_VALUE>(nullptr)};
        return u.as_blob;
    }
}
