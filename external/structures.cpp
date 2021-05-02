#include <iostream>
#include <memory>
#include <string>
#include <vector>
#include <cstdint>

extern "C" typedef void(Drop)(void *);

template <int N>
struct BlobData
{
    // N bytes
    uint8_t data[N];
};

template <typename Value>
union Blob
{
    typedef Value value_t;
    typedef BlobData<sizeof(Value)> blob_t;

    _Static_assert(sizeof(value_t) == sizeof(blob_t));

    value_t as_value;
    blob_t as_blob;

    ~Blob() {}
};

// Ptr<T>
typedef std::unique_ptr<int> Ptr;
typedef Blob<Ptr> PTR_BLOB;

extern "C" PTR_BLOB::blob_t lib_ruby_parser_containers_make_ptr_blob(void *ptr)
{
    PTR_BLOB u = {.as_value = std::unique_ptr<int>((int *)ptr)};
    PTR_BLOB::blob_t result = u.as_blob;
    u.as_value.release(); // prevent running destructor
    return result;
}

extern "C" void lib_ruby_parser_containers_free_ptr_blob(PTR_BLOB::blob_t blob, Drop drop)
{
    PTR_BLOB u = {.as_blob = blob};
    void *raw = u.as_value.release();
    drop(raw);
}

extern "C" void *lib_ruby_parser_containers_raw_ptr_from_ptr_blob(PTR_BLOB::blob_t blob)
{
    PTR_BLOB u = {.as_blob = blob};
    return u.as_value.release();
}

extern "C" PTR_BLOB::blob_t lib_ruby_parser_containers_null_ptr_blob()
{
    PTR_BLOB u = {.as_value = std::unique_ptr<int>(nullptr)};
    return u.as_blob;
}

// MaybePtr<T>
typedef std::unique_ptr<int> MaybePtr;
typedef Blob<MaybePtr> MAYBE_PTR_BLOB;

extern "C" MAYBE_PTR_BLOB::blob_t lib_ruby_parser_containers_make_maybe_ptr_blob(void *ptr)
{
    MAYBE_PTR_BLOB u = {.as_value = std::unique_ptr<int>((int *)ptr)};
    MAYBE_PTR_BLOB::blob_t result = u.as_blob;
    u.as_value.release(); // prevent running destructor
    return result;
}

extern "C" void lib_ruby_parser_containers_free_maybe_ptr_blob(MAYBE_PTR_BLOB::blob_t blob, Drop drop)
{
    MAYBE_PTR_BLOB u = {.as_blob = blob};
    void *raw = u.as_value.release();
    drop(raw);
}

extern "C" void *lib_ruby_parser_containers_raw_ptr_from_maybe_ptr_blob(MAYBE_PTR_BLOB::blob_t blob)
{
    MAYBE_PTR_BLOB u = {.as_blob = blob};
    return u.as_value.get();
}

extern "C" MAYBE_PTR_BLOB::blob_t lib_ruby_parser_containers_null_maybe_ptr_blob()
{
    MAYBE_PTR_BLOB u = {.as_value = std::unique_ptr<int>(nullptr)};
    return u.as_blob;
}

// List<T>

#define generate_list_impl(Item, prefix)                                                                                               \
    typedef Blob<std::vector<Item>> LIST_BLOB_##Item;                                                                                  \
                                                                                                                                       \
    std::vector<Item> lib_ruby_parser_containers_##prefix##_unpack_blob(LIST_BLOB_##Item::blob_t blob)                                 \
    {                                                                                                                                  \
        LIST_BLOB_##Item u = {.as_blob = blob};                                                                                        \
        return std::move(u.as_value);                                                                                                  \
    }                                                                                                                                  \
                                                                                                                                       \
    LIST_BLOB_##Item::blob_t lib_ruby_parser_containers_##prefix##_pack_blob(std::vector<Item> vec)                                    \
    {                                                                                                                                  \
        LIST_BLOB_##Item u = {.as_value = std::move(vec)};                                                                             \
        return u.as_blob;                                                                                                              \
    }                                                                                                                                  \
                                                                                                                                       \
    extern "C" LIST_BLOB_##Item::blob_t lib_ruby_parser_containers_##prefix##_list_blob_new()                                          \
    {                                                                                                                                  \
        return lib_ruby_parser_containers_##prefix##_pack_blob(std::vector<Item>());                                                   \
    }                                                                                                                                  \
                                                                                                                                       \
    extern "C" LIST_BLOB_##Item::blob_t lib_ruby_parser_containers_##prefix##_list_blob_with_capacity(uint64_t capacity)               \
    {                                                                                                                                  \
        std::vector<Item> vec;                                                                                                         \
        vec.reserve(capacity);                                                                                                         \
        return lib_ruby_parser_containers_##prefix##_pack_blob(std::move(vec));                                                        \
    }                                                                                                                                  \
                                                                                                                                       \
    extern "C" LIST_BLOB_##Item::blob_t lib_ruby_parser_containers_##prefix##_list_blob_from_raw(Item *ptr, uint64_t size)             \
    {                                                                                                                                  \
        auto vec = std::vector<Item>(ptr, ptr + size);                                                                                 \
        free(ptr);                                                                                                                     \
        return lib_ruby_parser_containers_##prefix##_pack_blob(std::move(vec));                                                        \
    }                                                                                                                                  \
                                                                                                                                       \
    extern "C" LIST_BLOB_##Item::blob_t lib_ruby_parser_containers_##prefix##_list_blob_push(LIST_BLOB_##Item::blob_t blob, Item item) \
    {                                                                                                                                  \
        std::vector<Item> vec = lib_ruby_parser_containers_##prefix##_unpack_blob(blob);                                               \
        vec.push_back(item);                                                                                                           \
        return lib_ruby_parser_containers_##prefix##_pack_blob(std::move(vec));                                                        \
    }                                                                                                                                  \
                                                                                                                                       \
    extern "C" Item lib_ruby_parser_containers_##prefix##_list_blob_remove(LIST_BLOB_##Item::blob_t blob, uint64_t index)              \
    {                                                                                                                                  \
        std::vector<Item> vec = lib_ruby_parser_containers_##prefix##_unpack_blob(blob);                                               \
        Item item = std::move(vec[index]);                                                                                             \
        vec.erase(vec.begin() + index);                                                                                                \
        lib_ruby_parser_containers_##prefix##_pack_blob(std::move(vec));                                                               \
        return item;                                                                                                                   \
    }                                                                                                                                  \
                                                                                                                                       \
    extern "C" void lib_ruby_parser_containers_##prefix##_list_blob_shrink_to_fit(LIST_BLOB_##Item::blob_t blob)                       \
    {                                                                                                                                  \
        std::vector<Item> vec = lib_ruby_parser_containers_##prefix##_unpack_blob(blob);                                               \
        vec.shrink_to_fit();                                                                                                           \
        lib_ruby_parser_containers_##prefix##_pack_blob(std::move(vec));                                                               \
    }                                                                                                                                  \
                                                                                                                                       \
    extern "C" Item *lib_ruby_parser_containers_##prefix##_list_blob_as_ptr(LIST_BLOB_##Item::blob_t blob)                             \
    {                                                                                                                                  \
        std::vector<Item> vec = lib_ruby_parser_containers_##prefix##_unpack_blob(blob);                                               \
        auto result = vec.data();                                                                                                      \
        lib_ruby_parser_containers_##prefix##_pack_blob(std::move(vec));                                                               \
        return result;                                                                                                                 \
    }                                                                                                                                  \
                                                                                                                                       \
    extern "C" uint64_t lib_ruby_parser_containers_##prefix##_list_blob_len(LIST_BLOB_##Item::blob_t blob)                             \
    {                                                                                                                                  \
        std::vector<Item> vec = lib_ruby_parser_containers_##prefix##_unpack_blob(blob);                                               \
        auto result = vec.size();                                                                                                      \
        lib_ruby_parser_containers_##prefix##_pack_blob(std::move(vec));                                                               \
        return result;                                                                                                                 \
    }                                                                                                                                  \
                                                                                                                                       \
    extern "C" uint64_t lib_ruby_parser_containers_##prefix##_list_blob_capacity(LIST_BLOB_##Item::blob_t blob)                        \
    {                                                                                                                                  \
        std::vector<Item> vec = lib_ruby_parser_containers_##prefix##_unpack_blob(blob);                                               \
        auto result = vec.capacity();                                                                                                  \
        lib_ruby_parser_containers_##prefix##_pack_blob(std::move(vec));                                                               \
        return result;                                                                                                                 \
    }                                                                                                                                  \
                                                                                                                                       \
    extern "C" void lib_ruby_parser_containers_##prefix##_list_blob_free(LIST_BLOB_##Item::blob_t blob, Drop drop_item_in_place)       \
    {                                                                                                                                  \
        std::vector<Item> vec = lib_ruby_parser_containers_##prefix##_unpack_blob(blob);                                               \
        for (size_t i = 0; i < vec.size(); i++)                                                                                        \
        {                                                                                                                              \
            drop_item_in_place(&vec.data()[i]);                                                                                        \
        }                                                                                                                              \
    }

typedef BlobData<192> Node;
generate_list_impl(Node, node);

typedef BlobData<64> Diagnostic;
generate_list_impl(Diagnostic, diagnostic);

typedef BlobData<24> Comment;
generate_list_impl(Comment, comment);

typedef BlobData<40> MagicComment;
generate_list_impl(MagicComment, magic_comment);

typedef BlobData<56> Token;
generate_list_impl(Token, token);

typedef BlobData<24> SourceLine;
generate_list_impl(SourceLine, source_line);

typedef BlobData<1> Byte;
generate_list_impl(Byte, byte);

typedef BlobData<8> U64;
generate_list_impl(U64, u64);
