#ifndef LIB_RUBY_PARSER_EXTERNAL_C_MAYBE_LOC_H
#define LIB_RUBY_PARSER_EXTERNAL_C_MAYBE_LOC_H

#include <stdint.h>
#include <stdbool.h>
#include "declare_blob.h"
#include "loc.h"

typedef struct
{
    enum
    {
        MAYBE_LOC_SOME,
        MAYBE_LOC_NONE,
    } tag;

    union
    {
        struct
        {
            uint8_t dummy;
        } nothing;
        Loc loc;
    } as;
} MaybeLoc;

DECLARE_BLOB_FOR(MaybeLoc);

_Static_assert(sizeof(MaybeLoc) == 24, "sizeof(MaybeLoc) == 24");

MaybeLoc_BLOB lib_ruby_parser__internal__containers__maybe_loc__make_some(Loc_BLOB loc_blob);
MaybeLoc_BLOB lib_ruby_parser__internal__containers__maybe_loc__make_none();
bool lib_ruby_parser__internal__containers__maybe_loc__is_some(MaybeLoc_BLOB blob);
bool lib_ruby_parser__internal__containers__maybe_loc__is_none(MaybeLoc_BLOB blob);
Loc_BLOB *lib_ruby_parser__internal__containers__maybe_loc__borrow_loc(MaybeLoc_BLOB *blob);
Loc_BLOB lib_ruby_parser__internal__containers__maybe_loc__into_loc(MaybeLoc_BLOB blob);

#endif // LIB_RUBY_PARSER_EXTERNAL_C_MAYBE_LOC_H
