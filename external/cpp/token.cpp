#include "token.hpp"
#include "impl_blob.hpp"
#include "forget.hpp"

IMPL_BLOB(Token);
IMPL_BLOB(LIST_OF_Token);

Token::Token(
    uint32_t token_type,
    Bytes token_value,
    Loc loc,
    uint32_t lex_state_before,
    uint32_t lex_state_after) : token_type(token_type),
                                token_value(std::move(token_value)),
                                loc(loc),
                                lex_state_before(lex_state_before),
                                lex_state_after(lex_state_after)
{
}

extern "C"
{
    Token_BLOB_DATA lib_ruby_parser_token_blob_new(
        uint32_t token_type,
        Bytes_BLOB_DATA token_value,
        Loc loc,
        uint32_t lex_state_before,
        uint32_t lex_state_after)
    {
        return PACK(Token(token_type, UNPACK(token_value), loc, lex_state_before, lex_state_after));
    }

    uint32_t lib_ruby_parser_token_blob_get_token_type(Token_BLOB_DATA token_blob)
    {
        Token token = UNPACK(token_blob);
        uint32_t token_type = token.token_type;
        forget(std::move(token));
        return token_type;
    }

    Bytes_BLOB_DATA *lib_ruby_parser_token_blob_borrow_token_value(Token_BLOB_DATA *token_blob)
    {
        Token *token = (Token *)token_blob;
        Bytes *bytes = &(token->token_value);
        return (Bytes_BLOB_DATA *)bytes;
    }

    Token_BLOB_DATA lib_ruby_parser_token_set_token_value(Token_BLOB_DATA token_blob, Bytes_BLOB_DATA bytes_blob)
    {
        Token token = UNPACK(token_blob);
        lib_ruby_parser_bytes_blob_free(PACK(std::move(token.token_value)));
        token.token_value = UNPACK(bytes_blob);
        return PACK(std::move(token));
    }

    Bytes_BLOB_DATA lib_ruby_parser_token_blob_into_token_value(Token_BLOB_DATA token_blob)
    {
        Token token = UNPACK(token_blob);
        Bytes token_value = std::move(token.token_value);
        return PACK(std::move(token_value));
    }

    Loc lib_ruby_parser_token_blob_borrow_loc(Token_BLOB_DATA token_blob)
    {
        Token token = UNPACK(token_blob);
        Loc loc = token.loc;
        forget(std::move(token));
        return loc;
    }
    uint32_t lib_ruby_parser_token_blob_get_lex_state_before(Token_BLOB_DATA token_blob)
    {
        Token token = UNPACK(token_blob);
        uint32_t lex_state_before = token.lex_state_before;
        forget(std::move(token));
        return lex_state_before;
    }
    uint32_t lib_ruby_parser_token_blob_get_lex_state_after(Token_BLOB_DATA token_blob)
    {
        Token token = UNPACK(token_blob);
        uint32_t lex_state_after = token.lex_state_after;
        forget(std::move(token));
        return lex_state_after;
    }
    void lib_ruby_parser_token_blob_free(Token_BLOB_DATA token_blob)
    {
        Token token = UNPACK(token_blob);
        // ~Token
    }
}
