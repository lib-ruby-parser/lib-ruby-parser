#ifndef LIB_RUBY_PARSER_EXTERNAL_C_TOKEN_H
#define LIB_RUBY_PARSER_EXTERNAL_C_TOKEN_H

#include "declare_blob.h"
#include "declare_list.h"
#include "bytes.h"
#include "loc.h"

typedef struct
{
    uint32_t token_type;
    BYTES_BLOB_DATA token_value;
    Loc loc;
    uint32_t lex_state_before;
    uint32_t lex_state_after;
} Token;
_Static_assert(sizeof(Token) == 56, "sizeof(Token) != 56");
DECLARE_BLOB_FOR(Token);

DECLARE_LIST_OF(Token_BLOB_DATA, LIST_OF_Token);
DECLARE_BLOB_FOR(LIST_OF_Token);
_Static_assert(sizeof(LIST_OF_Token) == 24, "sizeof(LIST_OF_Token) == 24");

#endif // LIB_RUBY_PARSER_EXTERNAL_C_TOKEN_H
