#ifndef LIB_RUBY_PARSER_EXTERNAL_C_LOC_H
#define LIB_RUBY_PARSER_EXTERNAL_C_LOC_H

#include <stdint.h>
#include "declare_blob.h"

typedef struct
{
    uint64_t begin;
    uint64_t end;
} Loc;
DECLARE_BLOB_FOR(Loc);

_Static_assert(sizeof(Loc) == 16, "sizeof(Loc) == 16");

Loc_BLOB lib_ruby_parser__internal__containers__loc__make(uint64_t begin, uint64_t end);
uint64_t lib_ruby_parser__internal__containers__loc__begin(Loc_BLOB blob);
uint64_t lib_ruby_parser__internal__containers__loc__end(Loc_BLOB blob);

#endif // LIB_RUBY_PARSER_EXTERNAL_C_LOC_H
