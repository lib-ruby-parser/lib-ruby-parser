#ifndef LIB_RUBY_PARSER_EXTERNAL_CPP_TYPES_TOKEN_HPP
#define LIB_RUBY_PARSER_EXTERNAL_CPP_TYPES_TOKEN_HPP

#include "declare_dummy_struct.hpp"
#include "declare_blob.hpp"
#include "declare_list.hpp"
#include "bytes.hpp"
#include "loc.hpp"
#include <iostream>

class Token
{
public:
    uint32_t token_type;
    Bytes token_value;
    Loc loc;
    uint32_t lex_state_before;
    uint32_t lex_state_after;

    explicit Token(
        uint32_t token_type,
        Bytes token_value,
        Loc loc,
        uint32_t lex_state_before,
        uint32_t lex_state_after);

    Token(const Token &) = delete;
    Token(Token &&) = default;
    Token &operator=(const Token &other) = delete;
    Token &operator=(Token &&other) = default;
    ~Token() = default;
};
_Static_assert(sizeof(Token) == 56, "sizeof(Token) == 56");
DECLARE_BLOB_FOR(Token);

DECLARE_LIST_OF(Token_BLOB, LIST_OF_Token);
DECLARE_BLOB_FOR(LIST_OF_Token);
_Static_assert(sizeof(LIST_OF_Token) == 24, "sizeof(LIST_OF_Token) == 24");

extern "C"
{
    Token_BLOB lib_ruby_parser_token_blob_new(
        uint32_t token_type,
        Bytes_BLOB token_value,
        Loc loc,
        uint32_t lex_state_before,
        uint32_t lex_state_after);
    uint32_t lib_ruby_parser_token_blob_get_token_type(Token_BLOB token_blob);
    Bytes_BLOB *lib_ruby_parser_token_blob_borrow_token_value(Token_BLOB *token_blob);
    Token_BLOB lib_ruby_parser_token_set_token_value(Token_BLOB token_blob, Bytes_BLOB bytes_blob);
    Bytes_BLOB lib_ruby_parser_token_blob_into_token_value(Token_BLOB token_blob);
    Loc lib_ruby_parser_token_blob_borrow_loc(Token_BLOB token_blob);
    uint32_t lib_ruby_parser_token_blob_get_lex_state_before(Token_BLOB token_blob);
    uint32_t lib_ruby_parser_token_blob_get_lex_state_after(Token_BLOB token_blob);
    void lib_ruby_parser_token_blob_free(Token_BLOB token_blob);
}

#endif // LIB_RUBY_PARSER_EXTERNAL_CPP_TYPES_TOKEN_HPP
