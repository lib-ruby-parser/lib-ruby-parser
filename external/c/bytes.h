#ifndef LIB_RUBY_PARSER_EXTERNAL_C_BYTES_H
#define LIB_RUBY_PARSER_EXTERNAL_C_BYTES_H

#include "declare_blob.h"
#include "byte.h"

// Bytes
typedef struct
{
    LIST_OF_Byte_BLOB raw;
} Bytes;
_Static_assert(sizeof(Bytes) == 24, "sizeof(Bytes) != 24");
DECLARE_BLOB_FOR(Bytes);

Bytes_BLOB lib_ruby_parser_bytes_blob_from_list_blob(LIST_OF_Byte_BLOB list_blob);
void lib_ruby_parser_bytes_blob_free(Bytes_BLOB bytes_blob);
Bytes_BLOB lib_ruby_parser_bytes_blob_new();
LIST_OF_Byte_BLOB lib_ruby_parser_list_blob_from_bytes_blob(Bytes_BLOB bytes_blob);

#endif // LIB_RUBY_PARSER_EXTERNAL_C_BYTES_H
