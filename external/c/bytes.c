#include "bytes.h"
#include "list.h"
#include "impl_blob.h"

IMPL_BLOB(Bytes);

Bytes_BLOB lib_ruby_parser__internal__containers__bytes__make_from_list_blob(LIST_OF_Byte_BLOB list_blob)
{
    Bytes bytes = {.raw = list_blob};
    return PACK_Bytes(bytes);
}

extern void drop_u8(void *p) { (void)p; }

void lib_ruby_parser__internal__containers__bytes__free(Bytes_BLOB bytes_blob)
{
    lib_ruby_parser__internal__containers__list__of_bytes__free(UNPACK_Bytes(bytes_blob).raw, drop_u8);
}
Bytes_BLOB lib_ruby_parser__internal__containers__bytes__make()
{
    Bytes bytes = {.raw = lib_ruby_parser__internal__containers__list__of_bytes__new()};
    return PACK_Bytes(bytes);
}
LIST_OF_Byte_BLOB lib_ruby_parser__internal__containers__bytes__to_list_blob(Bytes_BLOB bytes_blob)
{
    return UNPACK_Bytes(bytes_blob).raw;
}
