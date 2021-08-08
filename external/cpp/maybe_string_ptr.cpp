#include "maybe_string_ptr.hpp"
#include "impl_blob.hpp"
#include "forget.hpp"

MAYBE_STRING_PTR_BLOB PACK_MAYBE_STRING_PTR(MAYBE_STRING_PTR maybe_string_ptr)
{
    MAYBE_STRING_PTR_BLOB_UNION u = {.as_value = std::move(maybe_string_ptr)};
    return u.as_blob;
}
MAYBE_STRING_PTR UNPACK_MAYBE_STRING_PTR(MAYBE_STRING_PTR_BLOB blob)
{
    MAYBE_STRING_PTR_BLOB_UNION u = {.as_blob = blob};
    return std::move(u.as_value);
}

extern "C"
{
    MAYBE_STRING_PTR_BLOB lib_ruby_parser__internal__containers__maybe_string_ptr__make_some(uint8_t *ptr, uint64_t size) noexcept
    {
        return PACK_MAYBE_STRING_PTR(std::make_unique<std::string>((char *)ptr, (size_t)size));
    }
    MAYBE_STRING_PTR_BLOB lib_ruby_parser__internal__containers__maybe_string_ptr__make_none() noexcept
    {
        return PACK_MAYBE_STRING_PTR(std::unique_ptr<std::string>(nullptr));
    }

    bool lib_ruby_parser__internal__containers__maybe_string_ptr__is_some(const MAYBE_STRING_PTR_BLOB *blob) noexcept
    {
        const MAYBE_STRING_PTR *maybe_string_ptr = (const MAYBE_STRING_PTR *)blob;
        return maybe_string_ptr->get() != nullptr;
    }
    bool lib_ruby_parser__internal__containers__maybe_string_ptr__is_none(const MAYBE_STRING_PTR_BLOB *blob) noexcept
    {
        const MAYBE_STRING_PTR *maybe_string_ptr = (const MAYBE_STRING_PTR *)blob;
        return maybe_string_ptr->get() == nullptr;
    }

    void lib_ruby_parser__internal__containers__maybe_string_ptr__free(MAYBE_STRING_PTR_BLOB *blob) noexcept
    {
        MAYBE_STRING_PTR *maybe_string_ptr = (MAYBE_STRING_PTR *)blob;
        std::string *inner = maybe_string_ptr->release();
        if (inner)
        {
            delete inner;
        }
    }
    const uint8_t *lib_ruby_parser__internal__containers__maybe_string_ptr__get_raw_const(const MAYBE_STRING_PTR_BLOB *blob) noexcept
    {
        const MAYBE_STRING_PTR *maybe_string_ptr = (const MAYBE_STRING_PTR *)blob;
        if (maybe_string_ptr->get() == nullptr)
        {
            return nullptr;
        }
        const char *ptr = maybe_string_ptr->get()->data();
        return (const uint8_t *)ptr;
    }
    uint8_t *lib_ruby_parser__internal__containers__maybe_string_ptr__into_raw(MAYBE_STRING_PTR_BLOB *blob) noexcept
    {
        MAYBE_STRING_PTR *maybe_string_ptr = (MAYBE_STRING_PTR *)blob;
        if (maybe_string_ptr->get() == nullptr)
        {
            return nullptr;
        }
        std::string *inner = maybe_string_ptr->release();
        const char *ptr = inner->data();
        return (uint8_t *)ptr;
    }
    uint64_t lib_ruby_parser__internal__containers__maybe_string_ptr__len(const MAYBE_STRING_PTR_BLOB *blob) noexcept
    {
        const MAYBE_STRING_PTR *maybe_string_ptr = (const MAYBE_STRING_PTR *)blob;
        return maybe_string_ptr->get()->length();
    }
}
