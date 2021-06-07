#ifndef LIB_RUBY_PARSER_EXTERNAL_C_BYTE_HPP
#define LIB_RUBY_PARSER_EXTERNAL_C_BYTE_HPP

#include "declare_blob.hpp"
#include "declare_list.hpp"

typedef uint8_t Byte;
_Static_assert(sizeof(Byte) == 1, "sizeof(Byte) == 1");
DECLARE_BLOB_FOR(Byte);

DECLARE_LIST_OF(Byte_BLOB, LIST_OF_Byte);
DECLARE_BLOB_FOR(LIST_OF_Byte);
_Static_assert(sizeof(LIST_OF_Byte) == 24, "sizeof(LIST_OF_Byte) == 24");

#endif // LIB_RUBY_PARSER_EXTERNAL_C_BYTE_HPP
