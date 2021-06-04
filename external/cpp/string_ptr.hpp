#ifndef LIB_RUBY_PARSER_EXTERNAL_CPP_STRING_PTR_HPP
#define LIB_RUBY_PARSER_EXTERNAL_CPP_STRING_PTR_HPP

#include <string>
#include <memory>
#include "declare_blob.hpp"

// StringPtr<T>
// Small strings have optimization that forces string content
// to be stored INSIDE the string container.
// Because of that moved small string has a different c_str()
// which prevents us from sharing it with Rust
typedef std::unique_ptr<std::string> STRING_PTR;
_Static_assert(sizeof(STRING_PTR) == 8);
DECLARE_BLOB_FOR(STRING_PTR);

extern "C"
{
    void lib_ruby_parser_containers_free_string_blob(STRING_PTR_BLOB_DATA blob) noexcept;
    STRING_PTR_BLOB_DATA lib_ruby_parser_containers_clone_string_blob(STRING_PTR_BLOB_DATA blob) noexcept;
    const uint8_t *lib_ruby_parser_containers_raw_ptr_from_string_blob(STRING_PTR_BLOB_DATA blob) noexcept;
    uint64_t lib_ruby_parser_containers_string_blob_len(STRING_PTR_BLOB_DATA blob) noexcept;
    STRING_PTR_BLOB_DATA lib_ruby_parser_containers_string_blob_from_raw_ptr(const char *ptr, uint64_t len) noexcept;
}

#endif // LIB_RUBY_PARSER_EXTERNAL_CPP_STRING_PTR_HPP
