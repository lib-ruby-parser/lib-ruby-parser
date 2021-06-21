#include "string_ptr.h"
#include "impl_blob.h"

IMPL_BLOB(STRING_PTR);

void lib_ruby_parser__internal__containers__string_ptr__free(STRING_PTR_BLOB blob)
{
    free(UNPACK_STRING_PTR(blob).ptr);
}
STRING_PTR_BLOB lib_ruby_parser__internal__containers__string_ptr__clone(STRING_PTR_BLOB blob)
{
    STRING_PTR string_ptr = UNPACK_STRING_PTR(blob);
    STRING_PTR string_ptr_copy = {.ptr = malloc(string_ptr.size), .size = string_ptr.size};
    memcpy(string_ptr_copy.ptr, string_ptr.ptr, string_ptr.size);
    return PACK_STRING_PTR(string_ptr_copy);
}
const uint8_t *lib_ruby_parser__internal__containers__string_ptr__get_raw(STRING_PTR_BLOB blob)
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
uint64_t lib_ruby_parser__internal__containers__string_ptr__len(STRING_PTR_BLOB blob)
{
    return UNPACK_STRING_PTR(blob).size;
}
STRING_PTR_BLOB lib_ruby_parser__internal__containers__string_ptr__make(const char *ptr, uint64_t size)
{
    STRING_PTR string_ptr = {.ptr = malloc(size), .size = size};
    memcpy(string_ptr.ptr, ptr, size);
    return PACK_STRING_PTR(string_ptr);
}
