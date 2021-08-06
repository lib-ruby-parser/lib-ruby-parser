#include "maybe_loc.h"
#include "impl_blob.h"

IMPL_BLOB(MaybeLoc);

MaybeLoc_BLOB lib_ruby_parser__internal__containers__maybe_loc__make_some(Loc_BLOB loc_blob)
{
    MaybeLoc maybe_loc = {.tag = MAYBE_LOC_SOME, .as = {.loc = UNPACK_Loc(loc_blob)}};
    return PACK_MaybeLoc(maybe_loc);
}
MaybeLoc_BLOB lib_ruby_parser__internal__containers__maybe_loc__make_none()
{
    MaybeLoc maybe_loc = {.tag = MAYBE_LOC_NONE, .as = {.nothing = {.dummy = 0}}};
    return PACK_MaybeLoc(maybe_loc);
}
bool lib_ruby_parser__internal__containers__maybe_loc__is_some(MaybeLoc_BLOB blob)
{
    return UNPACK_MaybeLoc(blob).tag == MAYBE_LOC_SOME;
}
bool lib_ruby_parser__internal__containers__maybe_loc__is_none(MaybeLoc_BLOB blob)
{
    return UNPACK_MaybeLoc(blob).tag == MAYBE_LOC_NONE;
}
Loc_BLOB *lib_ruby_parser__internal__containers__maybe_loc__borrow_loc(MaybeLoc_BLOB *blob)
{
    MaybeLoc *maybe_loc = (MaybeLoc *)blob;
    return (Loc_BLOB *)(&maybe_loc->as.loc);
}
Loc_BLOB lib_ruby_parser__internal__containers__maybe_loc__into_loc(MaybeLoc_BLOB blob)
{
    return PACK_Loc(UNPACK_MaybeLoc(blob).as.loc);
}
