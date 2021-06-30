#ifndef LIB_RUBY_PARSER_EXTERNAL_CPP_TYPES_ERROR_LEVEL_HPP
#define LIB_RUBY_PARSER_EXTERNAL_CPP_TYPES_ERROR_LEVEL_HPP

#include "declare_blob.hpp"

enum class ErrorLevel
{
    WARNING,
    ERROR
};

DECLARE_BLOB_FOR(ErrorLevel);
_Static_assert(sizeof(ErrorLevel) == 4, "sizeof(ErrorLevel) == 4");

extern "C"
{
    ErrorLevel_BLOB lib_ruby_parser__internal__containers__error_level__make_warning();
    ErrorLevel_BLOB lib_ruby_parser__internal__containers__error_level__make_error();
    bool lib_ruby_parser__internal__containers__error_level__is_warning(ErrorLevel_BLOB blob);
    bool lib_ruby_parser__internal__containers__error_level__is_error(ErrorLevel_BLOB blob);
}

#endif // LIB_RUBY_PARSER_EXTERNAL_CPP_TYPES_ERROR_LEVEL_HPP
