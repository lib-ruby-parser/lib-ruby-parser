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
    Bytes_BLOB lib_ruby_parser__internal__containers__bytes__make_from_list_blob(LIST_OF_Byte_BLOB list_blob);
    void lib_ruby_parser__internal__containers__bytes__free(Bytes_BLOB bytes_blob);
    Bytes_BLOB lib_ruby_parser__internal__containers__bytes__make();
    LIST_OF_Byte_BLOB lib_ruby_parser__internal__containers__bytes__to_list_blob(Bytes_BLOB bytes_blob);
}

#endif // LIB_RUBY_PARSER_EXTERNAL_CPP_TYPES_BYTES_H
