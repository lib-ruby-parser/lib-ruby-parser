#include <iostream>
#include <memory>
#include <string>
#include <vector>
#include <cstdint>

extern "C" typedef void(Deleter)(void *);

// Ptr<T>
typedef std::unique_ptr<int> RustUniquePtr;
typedef uint64_t PTR_BLOB;
_Static_assert(sizeof(RustUniquePtr) == sizeof(PTR_BLOB), "can't use PTR_BLOB(uint64_t) instead of std::unique_ptr<T>");

extern "C"
{
    PTR_BLOB lib_ruby_parser_containers_make_ptr_blob(void *ptr)
    {
        RustUniquePtr unique_ptr = std::unique_ptr<int>((int *)ptr);
        PTR_BLOB result = *(PTR_BLOB *)(&unique_ptr);
        unique_ptr.release();
        std::cout << "make_ptr_blob: created " << result << "\n";
        return result;
    }

    void lib_ruby_parser_containers_free_ptr_blob(PTR_BLOB blob, Deleter deleter)
    {
        std::cout << "free_ptr_blob: freeing " << blob << "\n";
        RustUniquePtr *unique_ptr_ptr = (RustUniquePtr *)(&blob);
        void *raw = unique_ptr_ptr->release();
        deleter(raw);
    }

    void *lib_ruby_parser_containers_raw_ptr_from_ptr_blob(PTR_BLOB blob)
    {
        return ((RustUniquePtr *)(&blob))->get();
    }

    PTR_BLOB lib_ruby_parser_containers_null_ptr_blob()
    {
        RustUniquePtr unique_ptr = std::unique_ptr<int>(nullptr);
        PTR_BLOB result = *(PTR_BLOB *)(&unique_ptr);
        return result;
    }
}

// MaybePtr<T>
typedef std::unique_ptr<int> MaybeRustUniquePtr;
typedef uint64_t MAYBE_PTR_BLOB;
_Static_assert(sizeof(MaybeRustUniquePtr) == sizeof(MAYBE_PTR_BLOB), "can't use MAYBE_PTR_BLOB(uint64_t) instead of std::unique_ptr<T>");

extern "C"
{
    PTR_BLOB lib_ruby_parser_containers_make_maybe_ptr_blob(void *ptr)
    {
        RustUniquePtr unique_ptr = std::unique_ptr<int>((int *)ptr);
        PTR_BLOB result = *(PTR_BLOB *)(&unique_ptr);
        unique_ptr.release();
        std::cout << "make_maybe_ptr_blob: created " << result << "\n";
        return result;
    }

    void lib_ruby_parser_containers_free_maybe_ptr_blob(PTR_BLOB blob, Deleter deleter)
    {
        std::cout << "free_maybe_ptr_blob: freeing " << blob << "\n";
        RustUniquePtr *unique_ptr_ptr = (RustUniquePtr *)(&blob);
        void *raw = unique_ptr_ptr->release();
        deleter(raw);
    }

    void *lib_ruby_parser_containers_raw_ptr_from_maybe_ptr_blob(PTR_BLOB blob)
    {
        return ((RustUniquePtr *)(&blob))->get();
    }

    PTR_BLOB lib_ruby_parser_containers_null_maybe_ptr_blob()
    {
        RustUniquePtr unique_ptr = std::unique_ptr<int>(nullptr);
        PTR_BLOB result = *(PTR_BLOB *)(&unique_ptr);
        return result;
    }
}
