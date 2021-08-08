#include "maybe_string_ptr.h"
#include "impl_blob.h"

IMPL_BLOB(MAYBE_STRING_PTR);

MAYBE_STRING_PTR_BLOB lib_ruby_parser__internal__containers__maybe_string_ptr__make_some(uint8_t *ptr, uint64_t size)
{
    MAYBE_STRING_PTR maybe_string_ptr = {.ptr = malloc(size), .size = size};
    memcpy(maybe_string_ptr.ptr, ptr, size);
    return PACK_MAYBE_STRING_PTR(maybe_string_ptr);
}
MAYBE_STRING_PTR_BLOB lib_ruby_parser__internal__containers__maybe_string_ptr__make_none()
{
    MAYBE_STRING_PTR maybe_string_ptr = {.ptr = NULL, .size = 0};
    return PACK_MAYBE_STRING_PTR(maybe_string_ptr);
}

bool lib_ruby_parser__internal__containers__maybe_string_ptr__is_some(const MAYBE_STRING_PTR_BLOB *blob)
{
    const MAYBE_STRING_PTR *maybe_string_ptr = (const MAYBE_STRING_PTR *)blob;
    return maybe_string_ptr->ptr != NULL;
}
bool lib_ruby_parser__internal__containers__maybe_string_ptr__is_none(const MAYBE_STRING_PTR_BLOB *blob)
{
    const MAYBE_STRING_PTR *maybe_string_ptr = (const MAYBE_STRING_PTR *)blob;
    return maybe_string_ptr->ptr == NULL;
}

void lib_ruby_parser__internal__containers__maybe_string_ptr__free(MAYBE_STRING_PTR_BLOB *blob)
{
    MAYBE_STRING_PTR *maybe_string_ptr = (MAYBE_STRING_PTR *)blob;
    if (maybe_string_ptr->ptr == NULL)
    {
        return;
    }
    free(maybe_string_ptr->ptr);
    maybe_string_ptr->size = 0;
    maybe_string_ptr->ptr = NULL;
}
const uint8_t *lib_ruby_parser__internal__containers__maybe_string_ptr__get_raw_const(const MAYBE_STRING_PTR_BLOB *blob)
{
    const MAYBE_STRING_PTR *maybe_string_ptr = (const MAYBE_STRING_PTR *)blob;
    return (const uint8_t *)(maybe_string_ptr->ptr);
}
uint8_t *lib_ruby_parser__internal__containers__maybe_string_ptr__into_raw(MAYBE_STRING_PTR_BLOB *blob)
{
    MAYBE_STRING_PTR *maybe_string_ptr = (MAYBE_STRING_PTR *)blob;
    if (maybe_string_ptr->ptr == NULL)
    {
        return NULL;
    }
    uint8_t *result = (uint8_t *)(maybe_string_ptr->ptr);
    maybe_string_ptr->ptr = NULL;
    maybe_string_ptr->size = 0;
    return result;
}
uint64_t lib_ruby_parser__internal__containers__maybe_string_ptr__len(const MAYBE_STRING_PTR_BLOB *blob)
{
    const MAYBE_STRING_PTR *maybe_string_ptr = (const MAYBE_STRING_PTR *)blob;
    return maybe_string_ptr->size;
}
