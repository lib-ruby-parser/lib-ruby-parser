#ifndef LIB_RUBY_PARSER_EXTERNAL_CPP_MAYBE_LOC_HPP
#define LIB_RUBY_PARSER_EXTERNAL_CPP_MAYBE_LOC_HPP

#include <stdint.h>
#include <optional>
#include "declare_blob.hpp"
#include "loc.hpp"

typedef std::optional<Loc> MaybeLoc;
DECLARE_BLOB_FOR(MaybeLoc);

_Static_assert(sizeof(MaybeLoc) == 24);

extern "C"
{
    MaybeLoc_BLOB lib_ruby_parser__internal__containers__maybe_loc__make_some(Loc_BLOB loc_blob);
    MaybeLoc_BLOB lib_ruby_parser__internal__containers__maybe_loc__make_none();
    bool lib_ruby_parser__internal__containers__maybe_loc__is_some(MaybeLoc_BLOB blob);
    bool lib_ruby_parser__internal__containers__maybe_loc__is_none(MaybeLoc_BLOB blob);
    Loc_BLOB *lib_ruby_parser__internal__containers__maybe_loc__borrow_loc(MaybeLoc_BLOB *blob);
    Loc_BLOB lib_ruby_parser__internal__containers__maybe_loc__into_loc(MaybeLoc_BLOB blob);
}

#endif // LIB_RUBY_PARSER_EXTERNAL_CPP_MAYBE_LOC_HPP
