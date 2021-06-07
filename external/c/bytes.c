#include "bytes.h"
#include "list.h"
#include "impl_blob.h"

IMPL_BLOB(Bytes);

Bytes_BLOB_DATA lib_ruby_parser_bytes_blob_from_list_blob(LIST_OF_Byte_BLOB_DATA list_blob)
{
    Bytes bytes = {.raw = list_blob};
    return PACK_Bytes(bytes);
}

extern void drop_u8(void *p) { (void)p; }

void lib_ruby_parser_bytes_blob_free(Bytes_BLOB_DATA bytes_blob)
{
    lib_ruby_parser_containers_byte_list_blob_free(UNPACK_Bytes(bytes_blob).raw, drop_u8);
}
Bytes_BLOB_DATA lib_ruby_parser_bytes_blob_new()
{
    Bytes bytes = {.raw = lib_ruby_parser_containers_byte_list_blob_new()};
    return PACK_Bytes(bytes);
}
LIST_OF_Byte_BLOB_DATA lib_ruby_parser_list_blob_from_bytes_blob(Bytes_BLOB_DATA bytes_blob)
{
    return UNPACK_Bytes(bytes_blob).raw;
}
