#include "token.h"
#include "impl_blob.h"

IMPL_BLOB(Token);
IMPL_BLOB(LIST_OF_Token);

Token_BLOB lib_ruby_parser__internal__containers__token__new(
    uint32_t token_type,
    Bytes_BLOB token_value,
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

uint32_t lib_ruby_parser__internal__containers__token__get_token_type(Token_BLOB token_blob)
{
    return UNPACK_Token(token_blob).token_type;
}

Bytes_BLOB *lib_ruby_parser__internal__containers__token__get_token_value_ptr(Token_BLOB *token_blob)
{
    Token *token = (Token *)token_blob;
    return &(token->token_value);
}

Token_BLOB lib_ruby_parser__internal__containers__token__set_token_value(Token_BLOB token_blob, Bytes_BLOB bytes_blob)
{
    Token token = UNPACK_Token(token_blob);
    lib_ruby_parser__internal__containers__bytes__free(token.token_value);
    token.token_value = bytes_blob;
    return PACK_Token(token);
}

Bytes_BLOB lib_ruby_parser__internal__containers__token__into_token_value(Token_BLOB token_blob)
{
    return UNPACK_Token(token_blob).token_value;
}

Loc_BLOB lib_ruby_parser__internal__containers__token__get_loc(Token_BLOB token_blob)
{
    return PACK_Loc(UNPACK_Token(token_blob).loc);
}
uint32_t lib_ruby_parser__internal__containers__token__get_lex_state_before(Token_BLOB token_blob)
{
    return UNPACK_Token(token_blob).lex_state_before;
}

uint32_t lib_ruby_parser__internal__containers__token__get_lex_state_after(Token_BLOB token_blob)
{
    return UNPACK_Token(token_blob).lex_state_after;
}

void lib_ruby_parser__internal__containers__token__free(Token_BLOB token_blob)
{
    Token token = UNPACK_Token(token_blob);
    lib_ruby_parser__internal__containers__bytes__free(token.token_value);
}
