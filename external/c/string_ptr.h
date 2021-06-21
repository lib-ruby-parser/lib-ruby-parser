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

void lib_ruby_parser__internal__containers__string_ptr__free(STRING_PTR_BLOB blob);
STRING_PTR_BLOB lib_ruby_parser__internal__containers__string_ptr__clone(STRING_PTR_BLOB blob);
const uint8_t *lib_ruby_parser__internal__containers__string_ptr__get_raw(STRING_PTR_BLOB blob);
uint64_t lib_ruby_parser__internal__containers__string_ptr__len(STRING_PTR_BLOB blob);
STRING_PTR_BLOB lib_ruby_parser__internal__containers__string_ptr__make(const char *ptr, uint64_t size);

#endif // LIB_RUBY_PARSER_EXTERNAL_C_SHARED_STRING_PTR_H
