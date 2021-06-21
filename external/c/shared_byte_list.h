#ifndef LIB_RUBY_PARSER_EXTERNAL_C_SHARED_BYTE_LIST_H
#define LIB_RUBY_PARSER_EXTERNAL_C_SHARED_BYTE_LIST_H

#include <stddef.h>
#include <stdint.h>
#include "declare_blob.h"

// SharedByteList
typedef struct
{
    char *ptr;
    uint64_t size;
} SHARED_BYTE_LIST;
_Static_assert(sizeof(SHARED_BYTE_LIST) == 16, "sizeof(SHARED_BYTE_LIST) != 16");
DECLARE_BLOB_FOR(SHARED_BYTE_LIST);

SHARED_BYTE_LIST_BLOB lib_ruby_parser__internal__containers__shared_byte_list__from_raw(const char *ptr, uint64_t size);
const char *lib_ruby_parser__internal__containers__shared_byte_list__as_ptr(SHARED_BYTE_LIST_BLOB blob);
uint64_t lib_ruby_parser__internal__containers__shared_byte_list__len(SHARED_BYTE_LIST_BLOB blob);

#endif // LIB_RUBY_PARSER_EXTERNAL_C_SHARED_BYTE_LIST_H
