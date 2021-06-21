#ifndef LIB_RUBY_PARSER_EXTERNAL_CPP_SHARED_LIST_HPP
#define LIB_RUBY_PARSER_EXTERNAL_CPP_SHARED_LIST_HPP

#include <string_view>
#include "declare_blob.hpp"

// SharedList<T>
typedef std::string_view SHARED_BYTE_LIST;
_Static_assert(sizeof(SHARED_BYTE_LIST) == 16);
DECLARE_BLOB_FOR(SHARED_BYTE_LIST);

extern "C"
{
    SHARED_BYTE_LIST_BLOB lib_ruby_parser__internal__containers__shared_byte_list__from_raw(const char *ptr, uint64_t len) noexcept;
    const char *lib_ruby_parser__internal__containers__shared_byte_list__as_ptr(SHARED_BYTE_LIST_BLOB blob) noexcept;
    uint64_t lib_ruby_parser__internal__containers__shared_byte_list__len(SHARED_BYTE_LIST_BLOB blob) noexcept;
}

#endif // LIB_RUBY_PARSER_EXTERNAL_CPP_SHARED_LIST_HPP
