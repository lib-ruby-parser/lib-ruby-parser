#ifndef LIB_RUBY_PARSER_EXTERNAL_CPP_LOC_HPP
#define LIB_RUBY_PARSER_EXTERNAL_CPP_LOC_HPP

#include <cstdint>
#include "declare_blob.hpp"

class Loc
{
public:
    Loc(uint64_t begin, uint64_t end);
    uint64_t begin;
    uint64_t end;
};
DECLARE_BLOB_FOR(Loc);
_Static_assert(sizeof(Loc) == 16, "sizeof(Loc) == 16");

extern "C"
{
    Loc_BLOB lib_ruby_parser__internal__containers__loc__make(uint64_t begin, uint64_t end);
    uint64_t lib_ruby_parser__internal__containers__loc__begin(Loc_BLOB blob);
    uint64_t lib_ruby_parser__internal__containers__loc__end(Loc_BLOB blob);
}

#endif // LIB_RUBY_PARSER_EXTERNAL_CPP_LOC_HPP
