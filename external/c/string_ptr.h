#ifndef LIB_RUBY_PARSER_EXTERNAL_C_SHARED_STRING_PTR_H
#define LIB_RUBY_PARSER_EXTERNAL_C_SHARED_STRING_PTR_H

#include <stddef.h>
#include <stdlib.h>
#include <string.h>
#include "declare_blob.h"

// StringPtr
typedef struct
{
    char *ptr;
    uint64_t size;
} STRING_PTR;
_Static_assert(sizeof(STRING_PTR) == 16, "sizeof(STRING_PTR) != 16");
DECLARE_BLOB_FOR(STRING_PTR);

void lib_ruby_parser_containers_free_string_blob(STRING_PTR_BLOB_DATA blob)
{
    free(UNPACK_STRING_PTR(blob).ptr);
}
STRING_PTR_BLOB_DATA lib_ruby_parser_containers_clone_string_blob(STRING_PTR_BLOB_DATA blob)
{
    STRING_PTR string_ptr = UNPACK_STRING_PTR(blob);
    STRING_PTR string_ptr_copy = {.ptr = malloc(string_ptr.size), .size = string_ptr.size};
    memcpy(string_ptr_copy.ptr, string_ptr.ptr, string_ptr.size);
    return PACK_STRING_PTR(string_ptr_copy);
}
const uint8_t *lib_ruby_parser_containers_raw_ptr_from_string_blob(STRING_PTR_BLOB_DATA blob)
{
    STRING_PTR string_ptr = UNPACK_STRING_PTR(blob);
    if (string_ptr.size == 0)
    {
        return NULL;
    }
    else
    {
        return (const uint8_t *)string_ptr.ptr;
    }
}
uint64_t lib_ruby_parser_containers_string_blob_len(STRING_PTR_BLOB_DATA blob)
{
    return UNPACK_STRING_PTR(blob).size;
}
STRING_PTR_BLOB_DATA lib_ruby_parser_containers_string_blob_from_raw_ptr(const char *ptr, uint64_t size)
{
    STRING_PTR string_ptr = {.ptr = malloc(size), .size = size};
    memcpy(string_ptr.ptr, ptr, size);
    return PACK_STRING_PTR(string_ptr);
}

#endif // LIB_RUBY_PARSER_EXTERNAL_C_SHARED_STRING_PTR_H
