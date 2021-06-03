#ifndef LIB_RUBY_PARSER_EXTERNAL_CPP_MAYBE_PTR_H
#define LIB_RUBY_PARSER_EXTERNAL_CPP_MAYBE_PTR_H

#include <stdint.h>

extern "C" typedef void(DropPtrInPlace)(void *);
typedef int DUMMY_MAYBE_PTR_VALUE;

// MaybePtr<T>
typedef std::unique_ptr<DUMMY_MAYBE_PTR_VALUE> MAYBE_PTR;
_Static_assert(sizeof(MAYBE_PTR) == 8);
DECLARE_BLOB_FOR(MAYBE_PTR);

extern "C" MAYBE_PTR_BLOB_DATA lib_ruby_parser_containers_make_maybe_ptr_blob(void *ptr) noexcept
{
    MAYBE_PTR_BLOB_UNION u = {.as_value = std::unique_ptr<DUMMY_MAYBE_PTR_VALUE>((DUMMY_MAYBE_PTR_VALUE *)ptr)};
    MAYBE_PTR_BLOB_DATA result = u.as_blob;
    u.as_value.release(); // prevent running destructor
    return result;
}

extern "C" void lib_ruby_parser_containers_free_maybe_ptr_blob(MAYBE_PTR_BLOB_DATA blob, DropPtrInPlace drop_ptr_in_place) noexcept
{
    MAYBE_PTR_BLOB_UNION u = {.as_blob = blob};
    void *raw = u.as_value.release();
    if (raw)
    {
        drop_ptr_in_place(raw);
        free(raw);
    }
}

extern "C" void *lib_ruby_parser_containers_raw_ptr_from_maybe_ptr_blob(MAYBE_PTR_BLOB_DATA blob) noexcept
{
    MAYBE_PTR_BLOB_UNION u = {.as_blob = blob};
    return u.as_value.get();
}

extern "C" MAYBE_PTR_BLOB_DATA lib_ruby_parser_containers_null_maybe_ptr_blob() noexcept
{
    MAYBE_PTR_BLOB_UNION u = {.as_value = std::unique_ptr<DUMMY_MAYBE_PTR_VALUE>(nullptr)};
    return u.as_blob;
}

#endif // LIB_RUBY_PARSER_EXTERNAL_CPP_MAYBE_PTR_H
