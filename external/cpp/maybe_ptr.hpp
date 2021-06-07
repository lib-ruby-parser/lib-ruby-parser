#ifndef LIB_RUBY_PARSER_EXTERNAL_CPP_MAYBE_PTR_HPP
#define LIB_RUBY_PARSER_EXTERNAL_CPP_MAYBE_PTR_HPP

#include <stdint.h>
#include <memory>
#include "declare_blob.hpp"

typedef int DUMMY_MAYBE_PTR_VALUE;
extern "C" typedef void(DropPtrInPlace)(void *);

// MaybePtr<T>
typedef std::unique_ptr<DUMMY_MAYBE_PTR_VALUE> MAYBE_PTR;
_Static_assert(sizeof(MAYBE_PTR) == 8);
DECLARE_BLOB_STRUCTS(MAYBE_PTR)
DECLARE_BLOB_UNPACK_FOR(MAYBE_PTR)

extern "C"
{
    MAYBE_PTR_BLOB lib_ruby_parser_containers_make_maybe_ptr_blob(void *ptr) noexcept;
    void lib_ruby_parser_containers_free_maybe_ptr_blob(MAYBE_PTR_BLOB blob, DropPtrInPlace drop_ptr_in_place) noexcept;
    void *lib_ruby_parser_containers_raw_ptr_from_maybe_ptr_blob(MAYBE_PTR_BLOB blob) noexcept;
    MAYBE_PTR_BLOB lib_ruby_parser_containers_null_maybe_ptr_blob() noexcept;
}

#endif // LIB_RUBY_PARSER_EXTERNAL_CPP_MAYBE_PTR_HPP
