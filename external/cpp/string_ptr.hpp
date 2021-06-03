#ifndef LIB_RUBY_PARSER_EXTERNAL_CPP_STRING_PTR_H
#define LIB_RUBY_PARSER_EXTERNAL_CPP_STRING_PTR_H

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
    void lib_ruby_parser_containers_free_string_blob(STRING_PTR_BLOB_DATA blob) noexcept
    {
        STRING_PTR_BLOB_UNION u = {.as_blob = blob};
        STRING_PTR s = std::move(u.as_value);
        // unique_ptr<string> destructor does the cleanup
    }
    STRING_PTR_BLOB_DATA lib_ruby_parser_containers_clone_string_blob(STRING_PTR_BLOB_DATA blob) noexcept
    {
        STRING_PTR_BLOB_UNION u = {.as_blob = blob};
        STRING_PTR string_ptr_copy = std::make_unique<std::string>(*(u.as_value.get()));
        STRING_PTR_BLOB_UNION u_result = {.as_value = std::move(string_ptr_copy)};
        return u_result.as_blob;
    }
    const uint8_t *lib_ruby_parser_containers_raw_ptr_from_string_blob(STRING_PTR_BLOB_DATA blob) noexcept
    {
        STRING_PTR_BLOB_UNION u = {.as_blob = blob};
        if (u.as_value->length() == 0)
        {
            return nullptr;
        }
        else
        {
            return (const uint8_t *)(u.as_value->c_str());
        }
    }
    uint64_t lib_ruby_parser_containers_string_blob_len(STRING_PTR_BLOB_DATA blob) noexcept
    {
        STRING_PTR_BLOB_UNION u = {.as_blob = blob};
        return u.as_value->length();
    }
    STRING_PTR_BLOB_DATA lib_ruby_parser_containers_string_blob_from_raw_ptr(const char *ptr, uint64_t len) noexcept
    {
        STRING_PTR string_ptr = std::make_unique<std::string>(ptr, len);
        STRING_PTR_BLOB_UNION u = {.as_value = std::move(string_ptr)};
        return u.as_blob;
    }
}

#endif // LIB_RUBY_PARSER_EXTERNAL_CPP_STRING_PTR_H
