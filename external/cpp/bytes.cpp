#include "bytes.hpp"
#include "impl_blob.hpp"
#include "list.hpp"

IMPL_BLOB(Bytes);

Bytes::Bytes(LIST_OF_Byte raw)
{
    this->raw = raw;
}

extern "C"
{
    Bytes_BLOB_DATA lib_ruby_parser_bytes_blob_from_list_blob(LIST_OF_Byte_BLOB_DATA list_blob)
    {
        return PACK_Bytes(Bytes(UNPACK_LIST_OF_Byte(list_blob)));
    }

    extern void drop_u8(void *p) { (void)p; }

    void lib_ruby_parser_bytes_blob_free(Bytes_BLOB_DATA bytes_blob)
    {
        Bytes bytes = UNPACK_Bytes(bytes_blob);
        lib_ruby_parser_containers_byte_list_blob_free(PACK_LIST_OF_Byte(std::move(bytes.raw)), drop_u8);
    }
    Bytes_BLOB_DATA lib_ruby_parser_bytes_blob_new()
    {
        return PACK_Bytes(Bytes(LIST_OF_Byte()));
    }
    LIST_OF_Byte_BLOB_DATA lib_ruby_parser_list_blob_from_bytes_blob(Bytes_BLOB_DATA bytes_blob)
    {
        return PACK_LIST_OF_Byte(UNPACK_Bytes(bytes_blob).raw);
    }
}
