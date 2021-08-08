#ifndef LIB_RUBY_PARSER_EXTERNAL_C_SHARED_MAYBE_STRING_PTR_H
#define LIB_RUBY_PARSER_EXTERNAL_C_SHARED_MAYBE_STRING_PTR_H

#include <stddef.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>
#include "declare_blob.h"

// StringPtr
typedef struct
{
    char *ptr;
    uint64_t size;
} MAYBE_STRING_PTR;
_Static_assert(sizeof(MAYBE_STRING_PTR) == 16, "sizeof(MAYBE_STRING_PTR) != 16");
DECLARE_BLOB_FOR(MAYBE_STRING_PTR);

MAYBE_STRING_PTR_BLOB lib_ruby_parser__internal__containers__maybe_string_ptr__make_some(uint8_t *ptr, uint64_t size);
MAYBE_STRING_PTR_BLOB lib_ruby_parser__internal__containers__maybe_string_ptr__make_none();

bool lib_ruby_parser__internal__containers__maybe_string_ptr__is_some(const MAYBE_STRING_PTR_BLOB *blob);
bool lib_ruby_parser__internal__containers__maybe_string_ptr__is_none(const MAYBE_STRING_PTR_BLOB *blob);

void lib_ruby_parser__internal__containers__maybe_string_ptr__free(MAYBE_STRING_PTR_BLOB *blob);
const uint8_t *lib_ruby_parser__internal__containers__maybe_string_ptr__get_raw_const(const MAYBE_STRING_PTR_BLOB *blob);
uint8_t *lib_ruby_parser__internal__containers__maybe_string_ptr__into_raw(MAYBE_STRING_PTR_BLOB *blob);
uint64_t lib_ruby_parser__internal__containers__maybe_string_ptr__len(const MAYBE_STRING_PTR_BLOB *blob);

#endif // LIB_RUBY_PARSER_EXTERNAL_C_SHARED_MAYBE_STRING_PTR_H
