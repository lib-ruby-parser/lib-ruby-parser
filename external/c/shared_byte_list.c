#include "shared_byte_list.h"
#include "impl_blob.h"

IMPL_BLOB(SHARED_BYTE_LIST);

SHARED_BYTE_LIST_BLOB lib_ruby_parser_containers_shared_byte_list_blob_from_raw(const char *ptr, uint64_t size)
{
    SHARED_BYTE_LIST shared_byte_list = {.ptr = (char *)ptr, .size = size};
    return PACK_SHARED_BYTE_LIST(shared_byte_list);
}

const char *lib_ruby_parser_containers_shared_byte_list_blob_as_ptr(SHARED_BYTE_LIST_BLOB blob)
{
    SHARED_BYTE_LIST shared_byte_list = UNPACK_SHARED_BYTE_LIST(blob);
    if (shared_byte_list.size == 0)
    {
        return NULL;
    }
    else
    {

        return shared_byte_list.ptr;
    }
}
uint64_t lib_ruby_parser_containers_shared_byte_list_blob_len(SHARED_BYTE_LIST_BLOB blob)
{
    return UNPACK_SHARED_BYTE_LIST(blob).size;
}
