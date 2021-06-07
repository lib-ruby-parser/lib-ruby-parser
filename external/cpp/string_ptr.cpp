#include "string_ptr.hpp"
#include "impl_blob.hpp"
#include "forget.hpp"

IMPL_BLOB(STRING_PTR);

extern "C"
{
    void lib_ruby_parser_containers_free_string_blob(STRING_PTR_BLOB blob) noexcept
    {
        STRING_PTR string_ptr = UNPACK(blob);
        // unique_ptr<string> destructor does the cleanup
    }
    STRING_PTR_BLOB lib_ruby_parser_containers_clone_string_blob(STRING_PTR_BLOB blob) noexcept
    {
        STRING_PTR original = UNPACK(blob);
        STRING_PTR copy(new std::string(original->c_str()));
        forget(std::move(original));
        return PACK(std::move(copy));
    }
    const uint8_t *lib_ruby_parser_containers_raw_ptr_from_string_blob(STRING_PTR_BLOB blob) noexcept
    {
        STRING_PTR string_ptr = UNPACK(blob);
        const uint8_t *raw_ptr;
        if (string_ptr->length() == 0)
        {
            raw_ptr = nullptr;
        }
        else
        {
            raw_ptr = (const uint8_t *)(string_ptr->c_str());
        }
        forget(std::move(string_ptr));
        return raw_ptr;
    }
    uint64_t lib_ruby_parser_containers_string_blob_len(STRING_PTR_BLOB blob) noexcept
    {
        STRING_PTR string_ptr = UNPACK(blob);
        uint64_t length = string_ptr->length();
        forget(std::move(string_ptr));
        return length;
    }
    STRING_PTR_BLOB lib_ruby_parser_containers_string_blob_from_raw_ptr(const char *ptr, uint64_t len) noexcept
    {
        return PACK(std::make_unique<std::string>(ptr, len));
    }
}
