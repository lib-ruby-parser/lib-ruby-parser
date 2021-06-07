#ifndef LIB_RUBY_PARSER_EXTERNAL_C_SOURCE_LINE_H
#define LIB_RUBY_PARSER_EXTERNAL_C_SOURCE_LINE_H

#include "declare_dummy_struct.h"
#include "declare_blob.h"
#include "declare_list.h"

DECLARE_DUMMY_STRUCT(SourceLine, 24);
DECLARE_BLOB_FOR(SourceLine);

DECLARE_LIST_OF(SourceLine_BLOB, LIST_OF_SourceLine);
DECLARE_BLOB_FOR(LIST_OF_SourceLine);
_Static_assert(sizeof(LIST_OF_SourceLine) == 24, "sizeof(LIST_OF_SourceLine) == 24");

#endif // LIB_RUBY_PARSER_EXTERNAL_C_SOURCE_LINE_H
