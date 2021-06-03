#ifndef LIB_RUBY_PARSER_EXTERNAL_CPP_TYPES_BYTES_H
#define LIB_RUBY_PARSER_EXTERNAL_CPP_TYPES_BYTES_H

#include "../declare_blob.hpp"
#include "../list.hpp"

// Bytes
class BYTES
{
public:
    ByteList_BLOB_DATA raw;
    explicit BYTES(ByteList_BLOB_DATA raw) : raw(raw) {}
};
_Static_assert(sizeof(BYTES) == 24, "sizeof(BYTES) != 24");
DECLARE_BLOB_FOR(BYTES);

extern "C"
{
    BYTES_BLOB_DATA lib_ruby_parser_bytes_blob_from_list_blob(ByteList_BLOB_DATA list_blob)
    {
        BYTES bytes(list_blob);
        BYTES_BLOB_UNION u = {.as_value = bytes};
        return u.as_blob;
    }

    extern void drop_u8(void *p) { (void)p; }

    void lib_ruby_parser_bytes_blob_free(BYTES_BLOB_DATA bytes_blob)
    {
        BYTES_BLOB_UNION u = {.as_blob = bytes_blob};
        BYTES bytes = u.as_value;
        lib_ruby_parser_containers_byte_list_blob_free(bytes.raw, drop_u8);
    }
    BYTES_BLOB_DATA lib_ruby_parser_bytes_blob_new()
    {
        ByteList_BLOB_DATA raw = lib_ruby_parser_containers_byte_list_blob_new();
        BYTES bytes(raw);
        BYTES_BLOB_UNION u = {.as_value = bytes};
        return u.as_blob;
    }
    ByteList_BLOB_DATA lib_ruby_parser_list_blob_from_bytes_blob(BYTES_BLOB_DATA bytes_blob)
    {
        BYTES_BLOB_UNION u = {.as_blob = bytes_blob};
        BYTES bytes = u.as_value;
        return bytes.raw;
    }
}

#endif // LIB_RUBY_PARSER_EXTERNAL_CPP_TYPES_BYTES_H
