#ifndef LIB_RUBY_PARSER_EXTERNAL_C_SOURCE_LINE_H
#define LIB_RUBY_PARSER_EXTERNAL_C_SOURCE_LINE_H

#include "declare_dummy_struct.h"
#include "declare_blob.h"
#include "declare_list.h"
#include <stdbool.h>

typedef struct
{
    uint64_t start;
    uint64_t end;
    bool ends_with_eof;
} SourceLine;
_Static_assert(sizeof(SourceLine) == 24, "sizeof(SourceLine) == 24");
DECLARE_BLOB_FOR(SourceLine);

DECLARE_LIST_OF(SourceLine_BLOB, LIST_OF_SourceLine);
DECLARE_BLOB_FOR(LIST_OF_SourceLine);
_Static_assert(sizeof(LIST_OF_SourceLine) == 24, "sizeof(LIST_OF_SourceLine) == 24");

// API

SourceLine_BLOB lib_ruby_parser_source_line_new(uint64_t start, uint64_t end, bool ends_with_eof);

uint64_t lib_ruby_parser_source_line_get_start(SourceLine_BLOB blob);
uint64_t lib_ruby_parser_source_line_get_end(SourceLine_BLOB blob);
bool lib_ruby_parser_source_line_get_ends_with_eof(SourceLine_BLOB blob);

SourceLine_BLOB lib_ruby_parser_source_line_set_start(SourceLine_BLOB blob, uint64_t start);
SourceLine_BLOB lib_ruby_parser_source_line_set_end(SourceLine_BLOB blob, uint64_t end);
SourceLine_BLOB lib_ruby_parser_source_line_set_ends_with_eof(SourceLine_BLOB blob, bool ends_with_eof);

#endif // LIB_RUBY_PARSER_EXTERNAL_C_SOURCE_LINE_H
