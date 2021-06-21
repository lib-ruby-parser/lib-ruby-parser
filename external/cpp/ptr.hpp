#ifndef LIB_RUBY_PARSER_EXTERNAL_CPP_PTR_HPP
#define LIB_RUBY_PARSER_EXTERNAL_CPP_PTR_HPP

#include <stdint.h>
#include <memory>
#include "declare_blob.hpp"

typedef int DUMMY_PTR_VALUE;
extern "C" typedef void(DropPtrInPlace)(void *);

// Ptr<T>
typedef std::unique_ptr<DUMMY_PTR_VALUE> PTR;
_Static_assert(sizeof(PTR) == 8);
DECLARE_BLOB_FOR(PTR);

extern "C"
{
    PTR_BLOB lib_ruby_parser__internal__containers__ptr__make(void *ptr) noexcept;
    extern "C" void lib_ruby_parser__internal__containers__ptr__free(PTR_BLOB blob, DropPtrInPlace drop_ptr_in_place) noexcept;
    extern "C" void *lib_ruby_parser__internal__containers__ptr__get_raw(PTR_BLOB blob) noexcept;
    extern "C" PTR_BLOB lib_ruby_parser__internal__containers__ptr__make_null() noexcept;
}

#endif // LIB_RUBY_PARSER_EXTERNAL_CPP_PTR_HPP
