#ifndef LIB_RUBY_PARSER_EXTERNAL_CPP_SOURCE_LINE_HPP
#define LIB_RUBY_PARSER_EXTERNAL_CPP_SOURCE_LINE_HPP

#include "declare_dummy_struct.hpp"
#include "declare_blob.hpp"
#include "declare_list.hpp"

DECLARE_DUMMY_STRUCT(SourceLine, 24);
DECLARE_BLOB_FOR(SourceLine);

DECLARE_LIST_OF(SourceLine_BLOB, LIST_OF_SourceLine);
DECLARE_BLOB_FOR(LIST_OF_SourceLine);
_Static_assert(sizeof(LIST_OF_SourceLine) == 24, "sizeof(LIST_OF_SourceLine) == 24");

#endif // LIB_RUBY_PARSER_EXTERNAL_CPP_SOURCE_LINE_HPP
