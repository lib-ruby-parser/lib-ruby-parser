#ifndef LIB_RUBY_PARSER_EXTERNAL_CPP_TYPES_BYTES_H
#define LIB_RUBY_PARSER_EXTERNAL_CPP_TYPES_BYTES_H

#include "declare_blob.hpp"
#include "byte.hpp"

// Bytes
class Bytes
{
public:
    LIST_OF_Byte raw;
    explicit Bytes(LIST_OF_Byte raw);
    Bytes(const Bytes &) = delete;
    Bytes(Bytes &&) = default;
    Bytes &operator=(const Bytes &other) = delete;
    Bytes &operator=(Bytes &&other) = default;
    ~Bytes() = default;
};
_Static_assert(sizeof(Bytes) == 24, "sizeof(BYTES) != 24");
DECLARE_BLOB_FOR(Bytes);

extern "C"
{
    Bytes_BLOB_DATA lib_ruby_parser_bytes_blob_from_list_blob(LIST_OF_Byte_BLOB_DATA list_blob);
    void lib_ruby_parser_bytes_blob_free(Bytes_BLOB_DATA bytes_blob);
    Bytes_BLOB_DATA lib_ruby_parser_bytes_blob_new();
    LIST_OF_Byte_BLOB_DATA lib_ruby_parser_list_blob_from_bytes_blob(Bytes_BLOB_DATA bytes_blob);
}

#endif // LIB_RUBY_PARSER_EXTERNAL_CPP_TYPES_BYTES_H
