#include "shared_byte_list.hpp"
#include "impl_blob.hpp"

IMPL_BLOB(SHARED_BYTE_LIST);

extern "C"
{
    SHARED_BYTE_LIST_BLOB lib_ruby_parser_containers_shared_byte_list_blob_from_raw(const char *ptr, uint64_t len) noexcept
    {
        return PACK(SHARED_BYTE_LIST(ptr, len));
    }

    const char *lib_ruby_parser_containers_shared_byte_list_blob_as_ptr(SHARED_BYTE_LIST_BLOB blob) noexcept
    {
        SHARED_BYTE_LIST shared_byte_list = UNPACK(blob);
        if (shared_byte_list.length() == 0)
        {
            return nullptr;
        }
        else
        {

            return shared_byte_list.begin();
        }
    }
    uint64_t lib_ruby_parser_containers_shared_byte_list_blob_len(SHARED_BYTE_LIST_BLOB blob) noexcept
    {
        return UNPACK(blob).length();
    }
}
