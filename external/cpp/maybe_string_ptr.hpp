#ifndef LIB_RUBY_PARSER_EXTERNAL_CPP_MAYBE_STRING_PTR_HPP
#define LIB_RUBY_PARSER_EXTERNAL_CPP_MAYBE_STRING_PTR_HPP

#include <string>
#include <memory>
#include "declare_blob.hpp"

// StringPtr<T>
// Small strings have optimization that forces string content
// to be stored INSIDE the string container.
// Because of that moved small string has a different c_str()
// which prevents us from sharing it with Rust
typedef std::unique_ptr<std::string> MAYBE_STRING_PTR;
_Static_assert(sizeof(MAYBE_STRING_PTR) == 8);
DECLARE_BLOB_STRUCTS(MAYBE_STRING_PTR);
MAYBE_STRING_PTR_BLOB PACK_MAYBE_STRING_PTR(MAYBE_STRING_PTR maybe_string_ptr);
MAYBE_STRING_PTR UNPACK_MAYBE_STRING_PTR(MAYBE_STRING_PTR_BLOB blob);

extern "C"
{
    MAYBE_STRING_PTR_BLOB lib_ruby_parser__internal__containers__maybe_string_ptr__make_some(uint8_t *ptr, uint64_t size) noexcept;
    MAYBE_STRING_PTR_BLOB lib_ruby_parser__internal__containers__maybe_string_ptr__make_none() noexcept;

    bool lib_ruby_parser__internal__containers__maybe_string_ptr__is_some(const MAYBE_STRING_PTR_BLOB *blob) noexcept;
    bool lib_ruby_parser__internal__containers__maybe_string_ptr__is_none(const MAYBE_STRING_PTR_BLOB *blob) noexcept;

    void lib_ruby_parser__internal__containers__maybe_string_ptr__free(MAYBE_STRING_PTR_BLOB *blob) noexcept;
    const uint8_t *lib_ruby_parser__internal__containers__maybe_string_ptr__get_raw_const(const MAYBE_STRING_PTR_BLOB *blob) noexcept;
    uint8_t *lib_ruby_parser__internal__containers__maybe_string_ptr__into_raw(MAYBE_STRING_PTR_BLOB *blob) noexcept;
    uint64_t lib_ruby_parser__internal__containers__maybe_string_ptr__len(const MAYBE_STRING_PTR_BLOB *blob) noexcept;
}

#endif // LIB_RUBY_PARSER_EXTERNAL_CPP_MAYBE_STRING_PTR_HPP
