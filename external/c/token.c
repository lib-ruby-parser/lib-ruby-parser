#include "token.h"
#include "impl_blob.h"

IMPL_BLOB(Token);
IMPL_BLOB(LIST_OF_Token);

Token_BLOB_DATA lib_ruby_parser_token_blob_new(
    uint32_t token_type,
    BYTES_BLOB_DATA token_value,
    Loc loc,
    uint32_t lex_state_before,
    uint32_t lex_state_after)
{
    Token token = {
        .token_type = token_type,
        .token_value = token_value,
        .loc = loc,
        .lex_state_before = lex_state_before,
        .lex_state_after = lex_state_after};
    return PACK_Token(token);
}

uint32_t lib_ruby_parser_token_blob_get_token_type(Token_BLOB_DATA token_blob)
{
    return UNPACK_Token(token_blob).token_type;
}

BYTES_BLOB_DATA *lib_ruby_parser_token_blob_borrow_token_value(Token_BLOB_DATA *token_blob)
{
    Token *token = (Token *)token_blob;
    return &(token->token_value);
}

Token_BLOB_DATA lib_ruby_parser_token_set_token_value(Token_BLOB_DATA token_blob, BYTES_BLOB_DATA bytes_blob)
{
    Token token = UNPACK_Token(token_blob);
    lib_ruby_parser_bytes_blob_free(token.token_value);
    token.token_value = bytes_blob;
    return PACK_Token(token);
}

BYTES_BLOB_DATA lib_ruby_parser_token_blob_into_token_value(Token_BLOB_DATA token_blob)
{
    return UNPACK_Token(token_blob).token_value;
}

Loc lib_ruby_parser_token_blob_borrow_loc(Token_BLOB_DATA token_blob)
{
    return UNPACK_Token(token_blob).loc;
}
uint32_t lib_ruby_parser_token_blob_get_lex_state_before(Token_BLOB_DATA token_blob)
{
    return UNPACK_Token(token_blob).lex_state_before;
}

uint32_t lib_ruby_parser_token_blob_get_lex_state_after(Token_BLOB_DATA token_blob)
{
    return UNPACK_Token(token_blob).lex_state_after;
}

void lib_ruby_parser_token_blob_free(Token_BLOB_DATA token_blob)
{
    Token token = UNPACK_Token(token_blob);
    lib_ruby_parser_bytes_blob_free(token.token_value);
}
