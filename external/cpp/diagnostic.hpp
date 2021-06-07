#ifndef LIB_RUBY_PARSER_EXTERNAL_CPP_TYPES_DIAGNOSTIC_H
#define LIB_RUBY_PARSER_EXTERNAL_CPP_TYPES_DIAGNOSTIC_H

#include "declare_dummy_struct.hpp"
#include "declare_blob.hpp"
#include "declare_list.hpp"

DECLARE_DUMMY_STRUCT(Diagnostic, 40);
DECLARE_BLOB_FOR(Diagnostic);

DECLARE_LIST_OF(Diagnostic_BLOB, LIST_OF_Diagnostic);
DECLARE_BLOB_FOR(LIST_OF_Diagnostic);
_Static_assert(sizeof(LIST_OF_Diagnostic) == 24, "sizeof(LIST_OF_Diagnostic) == 24");

#endif // LIB_RUBY_PARSER_EXTERNAL_CPP_TYPES_DIAGNOSTIC_H
