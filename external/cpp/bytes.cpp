#include "bytes.hpp"
#include "impl_blob.hpp"
#include "list.hpp"

IMPL_BLOB(BYTES);

BYTES::BYTES(LIST_OF_Byte_BLOB_DATA raw)
{
    this->raw = raw;
}

extern "C"
{
    BYTES_BLOB_DATA lib_ruby_parser_bytes_blob_from_list_blob(LIST_OF_Byte_BLOB_DATA list_blob)
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
        LIST_OF_Byte_BLOB_DATA raw = lib_ruby_parser_containers_byte_list_blob_new();
        BYTES bytes(raw);
        BYTES_BLOB_UNION u = {.as_value = bytes};
        return u.as_blob;
    }
    LIST_OF_Byte_BLOB_DATA lib_ruby_parser_list_blob_from_bytes_blob(BYTES_BLOB_DATA bytes_blob)
    {
        BYTES_BLOB_UNION u = {.as_blob = bytes_blob};
        BYTES bytes = u.as_value;
        return bytes.raw;
    }
}
