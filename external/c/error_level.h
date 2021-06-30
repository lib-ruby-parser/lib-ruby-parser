#ifndef LIB_RUBY_PARSER_EXTERNAL_C_ERROR_LEVEL_H
#define LIB_RUBY_PARSER_EXTERNAL_C_ERROR_LEVEL_H

#include "declare_blob.h"
#include <stdbool.h>

typedef enum
{
    WARNING,
    ERROR
} ErrorLevel;
DECLARE_BLOB_FOR(ErrorLevel);

_Static_assert(sizeof(ErrorLevel) == 4, "sizeof(ErrorLevel) == 4");

ErrorLevel_BLOB lib_ruby_parser__internal__containers__error_level__make_warning();
ErrorLevel_BLOB lib_ruby_parser__internal__containers__error_level__make_error();
bool lib_ruby_parser__internal__containers__error_level__is_warning(ErrorLevel_BLOB blob);
bool lib_ruby_parser__internal__containers__error_level__is_error(ErrorLevel_BLOB blob);

#endif // LIB_RUBY_PARSER_EXTERNAL_C_ERROR_LEVEL_H
