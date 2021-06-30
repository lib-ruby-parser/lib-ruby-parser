#include "loc.hpp"
#include "impl_blob.hpp"

IMPL_BLOB(Loc);

Loc::Loc(uint64_t begin, uint64_t end) : begin(begin), end(end) {}

extern "C"
{
    Loc_BLOB lib_ruby_parser__internal__containers__loc__make(uint64_t begin, uint64_t end)
    {
        return PACK(Loc(begin, end));
    }
    uint64_t lib_ruby_parser__internal__containers__loc__begin(Loc_BLOB blob)
    {
        return UNPACK(blob).begin;
    }
    uint64_t lib_ruby_parser__internal__containers__loc__end(Loc_BLOB blob)
    {
        return UNPACK(blob).end;
    }
}
