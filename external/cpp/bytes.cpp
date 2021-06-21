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
    Bytes_BLOB lib_ruby_parser__internal__containers__bytes__make_from_list_blob(LIST_OF_Byte_BLOB list_blob)
    {
        return PACK(Bytes(UNPACK(list_blob)));
    }

    extern void drop_u8(void *p) { (void)p; }

    void lib_ruby_parser__internal__containers__bytes__free(Bytes_BLOB bytes_blob)
    {
        Bytes bytes = UNPACK(bytes_blob);
        lib_ruby_parser__internal__containers__list__of_bytes__free(PACK(std::move(bytes.raw)), drop_u8);
    }
    Bytes_BLOB lib_ruby_parser__internal__containers__bytes__make()
    {
        return PACK(Bytes(LIST_OF_Byte()));
    }
    LIST_OF_Byte_BLOB lib_ruby_parser__internal__containers__bytes__to_list_blob(Bytes_BLOB bytes_blob)
    {
        return PACK(UNPACK(bytes_blob).raw);
    }
}
