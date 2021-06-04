#ifndef LIB_RUBY_PARSER_EXTERNAL_CPP_TYPES_BYTES_H
#define LIB_RUBY_PARSER_EXTERNAL_CPP_TYPES_BYTES_H

#include "declare_blob.hpp"
#include "byte.hpp"

// Bytes
class BYTES
{
public:
    LIST_OF_Byte_BLOB_DATA raw;
    explicit BYTES(LIST_OF_Byte_BLOB_DATA raw);
};
_Static_assert(sizeof(BYTES) == 24, "sizeof(BYTES) != 24");
DECLARE_BLOB_FOR(BYTES);

extern "C"
{
    BYTES_BLOB_DATA lib_ruby_parser_bytes_blob_from_list_blob(LIST_OF_Byte_BLOB_DATA list_blob);
    void lib_ruby_parser_bytes_blob_free(BYTES_BLOB_DATA bytes_blob);
    BYTES_BLOB_DATA lib_ruby_parser_bytes_blob_new();
    LIST_OF_Byte_BLOB_DATA lib_ruby_parser_list_blob_from_bytes_blob(BYTES_BLOB_DATA bytes_blob);
}

#endif // LIB_RUBY_PARSER_EXTERNAL_CPP_TYPES_BYTES_H
