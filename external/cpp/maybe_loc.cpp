#include "maybe_loc.hpp"
#include "impl_blob.hpp"

IMPL_BLOB(MaybeLoc);

extern "C"
{
    MaybeLoc_BLOB lib_ruby_parser__internal__containers__maybe_loc__make_some(Loc_BLOB loc_blob)
    {
        return PACK(MaybeLoc(UNPACK(loc_blob)));
    }
    MaybeLoc_BLOB lib_ruby_parser__internal__containers__maybe_loc__make_none()
    {
        return PACK(MaybeLoc());
    }
    bool lib_ruby_parser__internal__containers__maybe_loc__is_some(MaybeLoc_BLOB blob)
    {
        return UNPACK(blob).has_value();
    }
    bool lib_ruby_parser__internal__containers__maybe_loc__is_none(MaybeLoc_BLOB blob)
    {
        return !UNPACK(blob).has_value();
    }
    Loc_BLOB *lib_ruby_parser__internal__containers__maybe_loc__borrow_loc(MaybeLoc_BLOB *blob)
    {
        MaybeLoc *maybe_loc = (MaybeLoc *)blob;
        Loc *loc = &(*(*maybe_loc));
        return (Loc_BLOB *)loc;
    }
    Loc_BLOB lib_ruby_parser__internal__containers__maybe_loc__into_loc(MaybeLoc_BLOB blob)
    {
        return PACK(UNPACK(blob).value());
    }
}
