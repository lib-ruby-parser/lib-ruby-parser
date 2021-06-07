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

void lib_ruby_parser_containers_free_string_blob(STRING_PTR_BLOB blob);
STRING_PTR_BLOB lib_ruby_parser_containers_clone_string_blob(STRING_PTR_BLOB blob);
const uint8_t *lib_ruby_parser_containers_raw_ptr_from_string_blob(STRING_PTR_BLOB blob);
uint64_t lib_ruby_parser_containers_string_blob_len(STRING_PTR_BLOB blob);
STRING_PTR_BLOB lib_ruby_parser_containers_string_blob_from_raw_ptr(const char *ptr, uint64_t size);

#endif // LIB_RUBY_PARSER_EXTERNAL_C_SHARED_STRING_PTR_H
