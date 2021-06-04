#ifndef LIB_RUBY_PARSER_EXTERNAL_CPP_TYPES_TOKEN_HPP
#define LIB_RUBY_PARSER_EXTERNAL_CPP_TYPES_TOKEN_HPP

#include "declare_dummy_struct.hpp"
#include "declare_blob.hpp"
#include "declare_list.hpp"

DECLARE_DUMMY_STRUCT(Token, 56);
DECLARE_BLOB_FOR(Token);

DECLARE_LIST_OF(Token_BLOB_DATA, LIST_OF_Token);
DECLARE_BLOB_FOR(LIST_OF_Token);
_Static_assert(sizeof(LIST_OF_Token) == 24, "sizeof(LIST_OF_Token) == 24");

#endif // LIB_RUBY_PARSER_EXTERNAL_CPP_TYPES_TOKEN_HPP
