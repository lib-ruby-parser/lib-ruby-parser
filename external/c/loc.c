#include "loc.h"
#include "impl_blob.h"

IMPL_BLOB(Loc);

Loc_BLOB lib_ruby_parser__internal__containers__loc__make(uint64_t begin, uint64_t end)
{
    Loc loc = {.begin = begin, .end = end};
    return PACK_Loc(loc);
}
uint64_t lib_ruby_parser__internal__containers__loc__begin(Loc_BLOB blob)
{
    return UNPACK_Loc(blob).begin;
}
uint64_t lib_ruby_parser__internal__containers__loc__end(Loc_BLOB blob)
{
    return UNPACK_Loc(blob).end;
}
