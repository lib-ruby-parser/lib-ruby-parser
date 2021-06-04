#include "shared_byte_list.hpp"
#include "impl_blob.hpp"

IMPL_BLOB(SHARED_BYTE_LIST);

extern "C"
{
    SHARED_BYTE_LIST_BLOB_DATA lib_ruby_parser_containers_shared_byte_list_blob_from_raw(const char *ptr, uint64_t len) noexcept
    {
        SHARED_BYTE_LIST byte_list(ptr, len);
        SHARED_BYTE_LIST_BLOB_UNION u = {.as_value = byte_list};
        return u.as_blob;
    }

    const char *lib_ruby_parser_containers_shared_byte_list_blob_as_ptr(SHARED_BYTE_LIST_BLOB_DATA blob) noexcept
    {
        SHARED_BYTE_LIST_BLOB_UNION u = {.as_blob = blob};
        if (u.as_value.length() == 0)
        {
            return nullptr;
        }
        else
        {

            return u.as_value.begin();
        }
    }
    uint64_t lib_ruby_parser_containers_shared_byte_list_blob_len(SHARED_BYTE_LIST_BLOB_DATA blob) noexcept
    {
        SHARED_BYTE_LIST_BLOB_UNION u = {.as_blob = blob};
        return u.as_value.length();
    }
}
