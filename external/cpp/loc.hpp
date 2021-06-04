#ifndef LIB_RUBY_PARSER_EXTERNAL_CPP_LOC_HPP
#define LIB_RUBY_PARSER_EXTERNAL_CPP_LOC_HPP

#include <cstdint>

extern "C"
{
    struct Loc
    {
        uint64_t begin;
        uint64_t end;
    };
}

#endif // LIB_RUBY_PARSER_EXTERNAL_CPP_LOC_HPP
