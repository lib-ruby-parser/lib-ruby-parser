#ifndef LIB_RUBY_PARSER_EXTERNAL_C_SHARED_BYTE_LIST_H
#define LIB_RUBY_PARSER_EXTERNAL_C_SHARED_BYTE_LIST_H

#include <stddef.h>
#include "declare_blob.h"

// SharedByteList
typedef struct
{
    char *ptr;
    uint64_t size;
} SHARED_BYTE_LIST;
_Static_assert(sizeof(SHARED_BYTE_LIST) == 16, "sizeof(SHARED_BYTE_LIST) != 16");
DECLARE_BLOB_FOR(SHARED_BYTE_LIST);

SHARED_BYTE_LIST_BLOB_DATA lib_ruby_parser_containers_shared_byte_list_blob_from_raw(const char *ptr, uint64_t size)
{
    SHARED_BYTE_LIST shared_byte_list = {.ptr = (char *)ptr, .size = size};
    return PACK_SHARED_BYTE_LIST(shared_byte_list);
}

const char *lib_ruby_parser_containers_shared_byte_list_blob_as_ptr(SHARED_BYTE_LIST_BLOB_DATA blob)
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
uint64_t lib_ruby_parser_containers_shared_byte_list_blob_len(SHARED_BYTE_LIST_BLOB_DATA blob)
{
    return UNPACK_SHARED_BYTE_LIST(blob).size;
}

#endif // LIB_RUBY_PARSER_EXTERNAL_C_SHARED_BYTE_LIST_H
