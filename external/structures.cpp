#include <iostream>
#include <memory>
#include <string>
#include <vector>
#include <cstdint>

extern "C" typedef void(Drop)(void *);

// Ptr<T>
extern "C"
{
    typedef std::unique_ptr<int> RustUniquePtr;
    typedef struct
    {
        // 8 bytes
        uint8_t data[8];
    } PTR_BLOB;
    _Static_assert(sizeof(RustUniquePtr) == sizeof(PTR_BLOB), "can't use PTR_BLOB(8 bytes) instead of std::unique_ptr<T>");

    typedef union PTR_BLOB_UNION
    {
        RustUniquePtr as_ptr;
        PTR_BLOB as_blob;

        ~PTR_BLOB_UNION() {}
    } PTR_BLOB_UNION;

    PTR_BLOB lib_ruby_parser_containers_make_ptr_blob(void *ptr)
    {
        PTR_BLOB_UNION u = {.as_ptr = std::unique_ptr<int>((int *)ptr)};
        PTR_BLOB result = u.as_blob;
        u.as_ptr.release(); // prevent running destructor
        return result;
    }

    void lib_ruby_parser_containers_free_ptr_blob(PTR_BLOB blob, Drop drop)
    {
        PTR_BLOB_UNION u = {.as_blob = blob};
        void *raw = u.as_ptr.release();
        drop(raw);
    }

    void *lib_ruby_parser_containers_raw_ptr_from_ptr_blob(PTR_BLOB blob)
    {
        PTR_BLOB_UNION u = {.as_blob = blob};
        return u.as_ptr.release();
    }

    PTR_BLOB lib_ruby_parser_containers_null_ptr_blob()
    {
        PTR_BLOB_UNION u = {.as_ptr = std::unique_ptr<int>(nullptr)};
        return u.as_blob;
    }
}

// MaybePtr<T>
extern "C"
{
    typedef std::unique_ptr<int> MaybeRustUniquePtr;
    typedef struct
    {
        // 8 bytes
        uint8_t data[8];
    } MAYBE_PTR_BLOB;
    _Static_assert(sizeof(MaybeRustUniquePtr) == sizeof(MAYBE_PTR_BLOB), "can't use MAYBE_PTR_BLOB(8 bytes) instead of std::unique_ptr<T>");

    typedef union MAYBE_PTR_BLOB_UNION
    {
        RustUniquePtr as_ptr;
        MAYBE_PTR_BLOB as_blob;

        ~MAYBE_PTR_BLOB_UNION() {}
    } MAYBE_PTR_BLOB_UNION;

    MAYBE_PTR_BLOB lib_ruby_parser_containers_make_maybe_ptr_blob(void *ptr)
    {
        MAYBE_PTR_BLOB_UNION u = {.as_ptr = std::unique_ptr<int>((int *)ptr)};
        MAYBE_PTR_BLOB result = u.as_blob;
        u.as_ptr.release(); // prevent running destructor
        return result;
    }

    void lib_ruby_parser_containers_free_maybe_ptr_blob(MAYBE_PTR_BLOB blob, Drop drop)
    {
        MAYBE_PTR_BLOB_UNION u = {.as_blob = blob};
        void *raw = u.as_ptr.release();
        drop(raw);
    }

    void *lib_ruby_parser_containers_raw_ptr_from_maybe_ptr_blob(MAYBE_PTR_BLOB blob)
    {
        MAYBE_PTR_BLOB_UNION u = {.as_blob = blob};
        return u.as_ptr.get();
    }

    MAYBE_PTR_BLOB lib_ruby_parser_containers_null_maybe_ptr_blob()
    {
        MAYBE_PTR_BLOB_UNION u = {.as_ptr = std::unique_ptr<int>(nullptr)};
        return u.as_blob;
    }
}

// List<T>

extern "C"
{
    typedef std::vector<int> RustList;
    struct LIST_BLOB
    {
        // 24 bytes
        uint8_t data[24];
    };

    _Static_assert(sizeof(RustList) == sizeof(LIST_BLOB), "can't use LIST_BLOB(24 bytes) instead of std::vector<T>");

    LIST_BLOB lib_ruby_parser_containers_maybe_list_blob(void *ptr, uint64_t size);
}
