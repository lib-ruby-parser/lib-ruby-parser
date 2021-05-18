#include <iostream>
#include <sstream>
#include <memory>
#include <string>
#include <vector>
#include <cstdint>

extern "C" typedef void(Drop)(void *);
typedef int DUMMY;
typedef uint8_t BYTE;

#define DECLARE_BLOB_FOR(VALUE)                                \
    extern "C"                                                 \
    {                                                          \
        struct VALUE##_BLOB_DATA                               \
        {                                                      \
            BYTE data[sizeof(VALUE)];                          \
        };                                                     \
    }                                                          \
                                                               \
    extern "C"                                                 \
    {                                                          \
        union VALUE##_BLOB_UNION                               \
        {                                                      \
            typedef VALUE value_t;                             \
            typedef VALUE##_BLOB_DATA blob_t;                  \
                                                               \
            _Static_assert(sizeof(value_t) == sizeof(blob_t)); \
                                                               \
            value_t as_value;                                  \
            blob_t as_blob;                                    \
                                                               \
            ~VALUE##_BLOB_UNION() noexcept                     \
            {                                                  \
            }                                                  \
        };                                                     \
    }

// Ptr<T>
typedef std::unique_ptr<DUMMY> PTR;
_Static_assert(sizeof(PTR) == 8);
DECLARE_BLOB_FOR(PTR);

extern "C" PTR_BLOB_DATA lib_ruby_parser_containers_make_ptr_blob(void *ptr) noexcept
{
    PTR_BLOB_UNION u = {.as_value = std::unique_ptr<DUMMY>((DUMMY *)ptr)};
    PTR_BLOB_DATA result = u.as_blob;
    u.as_value.release(); // prevent running destructor
    return result;
}

extern "C" void lib_ruby_parser_containers_free_ptr_blob(PTR_BLOB_DATA blob, Drop drop_ptr_in_place) noexcept
{
    PTR_BLOB_UNION u = {.as_blob = blob};
    void *raw = u.as_value.release();
    if (raw)
    {
        drop_ptr_in_place(raw);
        free(raw);
    }
}

extern "C" void *lib_ruby_parser_containers_raw_ptr_from_ptr_blob(PTR_BLOB_DATA blob) noexcept
{
    PTR_BLOB_UNION u = {.as_blob = blob};
    return u.as_value.release();
}

extern "C" PTR_BLOB_DATA lib_ruby_parser_containers_null_ptr_blob() noexcept
{
    PTR_BLOB_UNION u = {.as_value = std::unique_ptr<DUMMY>(nullptr)};
    return u.as_blob;
}

// MaybePtr<T>
typedef std::unique_ptr<DUMMY> MAYBE_PTR;
_Static_assert(sizeof(MAYBE_PTR) == 8);
DECLARE_BLOB_FOR(MAYBE_PTR);

extern "C" MAYBE_PTR_BLOB_DATA lib_ruby_parser_containers_make_maybe_ptr_blob(void *ptr) noexcept
{
    MAYBE_PTR_BLOB_UNION u = {.as_value = std::unique_ptr<DUMMY>((DUMMY *)ptr)};
    MAYBE_PTR_BLOB_DATA result = u.as_blob;
    u.as_value.release(); // prevent running destructor
    return result;
}

extern "C" void lib_ruby_parser_containers_free_maybe_ptr_blob(MAYBE_PTR_BLOB_DATA blob, Drop drop_ptr_in_place) noexcept
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
    MAYBE_PTR_BLOB_UNION u = {.as_value = std::unique_ptr<DUMMY>(nullptr)};
    return u.as_blob;
}

// List<T>

#define DECLARE_BLOB_FOR_LIST_OF(VALUE, PREFIX)                                                                                                 \
    typedef std::vector<VALUE##_BLOB_DATA> VALUE##List;                                                                                         \
    DECLARE_BLOB_FOR(VALUE##List);                                                                                                              \
    _Static_assert(sizeof(VALUE##List) == 24);                                                                                                  \
                                                                                                                                                \
    VALUE##List lib_ruby_parser_containers_##PREFIX##_unpack_blob(VALUE##List_BLOB_DATA blob) noexcept                                          \
    {                                                                                                                                           \
        VALUE##List_BLOB_UNION u = {.as_blob = blob};                                                                                           \
        return std::move(u.as_value);                                                                                                           \
    }                                                                                                                                           \
                                                                                                                                                \
    VALUE##List_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_pack_blob(VALUE##List vec) noexcept                                             \
    {                                                                                                                                           \
        VALUE##List_BLOB_UNION u = {.as_value = std::move(vec)};                                                                                \
        return u.as_blob;                                                                                                                       \
    }                                                                                                                                           \
                                                                                                                                                \
    extern "C"                                                                                                                                  \
    {                                                                                                                                           \
        VALUE##List_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_list_blob_new() noexcept                                                    \
        {                                                                                                                                       \
            return lib_ruby_parser_containers_##PREFIX##_pack_blob(VALUE##List());                                                              \
        }                                                                                                                                       \
                                                                                                                                                \
        VALUE##List_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_list_blob_with_capacity(uint64_t capacity) noexcept                         \
        {                                                                                                                                       \
            VALUE##List vec;                                                                                                                    \
            vec.reserve(capacity);                                                                                                              \
            return lib_ruby_parser_containers_##PREFIX##_pack_blob(std::move(vec));                                                             \
        }                                                                                                                                       \
                                                                                                                                                \
        VALUE##List_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_list_blob_from_raw(VALUE##_BLOB_DATA *ptr, uint64_t size) noexcept          \
        {                                                                                                                                       \
            if (size > 0)                                                                                                                       \
            {                                                                                                                                   \
                auto vec = VALUE##List(ptr, ptr + size);                                                                                        \
                free(ptr);                                                                                                                      \
                return lib_ruby_parser_containers_##PREFIX##_pack_blob(std::move(vec));                                                         \
            }                                                                                                                                   \
            else                                                                                                                                \
            {                                                                                                                                   \
                return lib_ruby_parser_containers_##PREFIX##_list_blob_new();                                                                   \
            }                                                                                                                                   \
        }                                                                                                                                       \
                                                                                                                                                \
        VALUE##List_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_list_blob_push(VALUE##List_BLOB_DATA blob, VALUE##_BLOB_DATA item) noexcept \
        {                                                                                                                                       \
            VALUE##List vec = lib_ruby_parser_containers_##PREFIX##_unpack_blob(blob);                                                          \
            vec.push_back(item);                                                                                                                \
            return lib_ruby_parser_containers_##PREFIX##_pack_blob(std::move(vec));                                                             \
        }                                                                                                                                       \
                                                                                                                                                \
        typedef struct                                                                                                                          \
        {                                                                                                                                       \
            VALUE##List_BLOB_DATA new_blob;                                                                                                     \
            VALUE##_BLOB_DATA removed_item;                                                                                                     \
        } VALUE##List_REMOVE_RESULT;                                                                                                            \
                                                                                                                                                \
        VALUE##List_REMOVE_RESULT lib_ruby_parser_containers_##PREFIX##_list_blob_remove(VALUE##List_BLOB_DATA blob, uint64_t index) noexcept   \
        {                                                                                                                                       \
            VALUE##List vec = lib_ruby_parser_containers_##PREFIX##_unpack_blob(blob);                                                          \
            VALUE##_BLOB_DATA item = std::move(vec[index]);                                                                                     \
            vec.erase(vec.begin() + index);                                                                                                     \
            VALUE##List_REMOVE_RESULT result = {                                                                                                \
                .new_blob = lib_ruby_parser_containers_##PREFIX##_pack_blob(std::move(vec)),                                                    \
                .removed_item = item};                                                                                                          \
                                                                                                                                                \
            return result;                                                                                                                      \
        }                                                                                                                                       \
                                                                                                                                                \
        VALUE##List_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_list_blob_shrink_to_fit(VALUE##List_BLOB_DATA blob) noexcept                \
        {                                                                                                                                       \
            VALUE##List vec = lib_ruby_parser_containers_##PREFIX##_unpack_blob(blob);                                                          \
            vec.shrink_to_fit();                                                                                                                \
            return lib_ruby_parser_containers_##PREFIX##_pack_blob(std::move(vec));                                                             \
        }                                                                                                                                       \
                                                                                                                                                \
        VALUE##_BLOB_DATA *lib_ruby_parser_containers_##PREFIX##_list_blob_as_ptr(VALUE##List_BLOB_DATA blob) noexcept                          \
        {                                                                                                                                       \
            VALUE##List vec = lib_ruby_parser_containers_##PREFIX##_unpack_blob(blob);                                                          \
            auto result = vec.data();                                                                                                           \
            lib_ruby_parser_containers_##PREFIX##_pack_blob(std::move(vec));                                                                    \
            return result;                                                                                                                      \
        }                                                                                                                                       \
                                                                                                                                                \
        uint64_t lib_ruby_parser_containers_##PREFIX##_list_blob_len(VALUE##List_BLOB_DATA blob) noexcept                                       \
        {                                                                                                                                       \
            VALUE##List vec = lib_ruby_parser_containers_##PREFIX##_unpack_blob(blob);                                                          \
            auto result = vec.size();                                                                                                           \
            lib_ruby_parser_containers_##PREFIX##_pack_blob(std::move(vec));                                                                    \
            return result;                                                                                                                      \
        }                                                                                                                                       \
                                                                                                                                                \
        uint64_t lib_ruby_parser_containers_##PREFIX##_list_blob_capacity(VALUE##List_BLOB_DATA blob) noexcept                                  \
        {                                                                                                                                       \
            VALUE##List vec = lib_ruby_parser_containers_##PREFIX##_unpack_blob(blob);                                                          \
            auto result = vec.capacity();                                                                                                       \
            lib_ruby_parser_containers_##PREFIX##_pack_blob(std::move(vec));                                                                    \
            return result;                                                                                                                      \
        }                                                                                                                                       \
                                                                                                                                                \
        void lib_ruby_parser_containers_##PREFIX##_list_blob_free(                                                                              \
            VALUE##List_BLOB_DATA blob, Drop drop_ptr_in_place) noexcept                                                                        \
        {                                                                                                                                       \
            VALUE##List vec = lib_ruby_parser_containers_##PREFIX##_unpack_blob(blob);                                                          \
            for (size_t i = 0; i < vec.size(); i++)                                                                                             \
            {                                                                                                                                   \
                drop_ptr_in_place(&vec[i]);                                                                                                     \
            }                                                                                                                                   \
        }                                                                                                                                       \
    }

struct NodeStruct
{
    BYTE data[184];
};
DECLARE_BLOB_FOR(NodeStruct);
DECLARE_BLOB_FOR_LIST_OF(NodeStruct, node);

struct DiagnosticStruct
{
    BYTE data[40];
};
DECLARE_BLOB_FOR(DiagnosticStruct);
DECLARE_BLOB_FOR_LIST_OF(DiagnosticStruct, diagnostic);

struct ComentStruct
{
    BYTE data[24];
};
DECLARE_BLOB_FOR(ComentStruct);
DECLARE_BLOB_FOR_LIST_OF(ComentStruct, comment);

struct MagicCommentStruct
{
    BYTE data[40];
};
DECLARE_BLOB_FOR(MagicCommentStruct);
DECLARE_BLOB_FOR_LIST_OF(MagicCommentStruct, magic_comment);

struct TokenStruct
{
    BYTE data[56];
};
DECLARE_BLOB_FOR(TokenStruct);
DECLARE_BLOB_FOR_LIST_OF(TokenStruct, token);

struct SourceLineStruct
{
    BYTE data[24];
};
DECLARE_BLOB_FOR(SourceLineStruct);
DECLARE_BLOB_FOR_LIST_OF(SourceLineStruct, source_line);

struct ByteStruct
{
    BYTE data[1];
};
DECLARE_BLOB_FOR(ByteStruct);
DECLARE_BLOB_FOR_LIST_OF(ByteStruct, byte);

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

// SharedList<T>
typedef std::string_view SHARED_BYTE_LIST;
_Static_assert(sizeof(SHARED_BYTE_LIST) == 16);
DECLARE_BLOB_FOR(SHARED_BYTE_LIST);

extern "C"
{
    SHARED_BYTE_LIST_BLOB_DATA lib_ruby_parser_containers_shared_byte_list_blob_from_raw(const char *ptr, uint64_t len) noexcept
    {
        SHARED_BYTE_LIST byte_list(ptr, len);
        SHARED_BYTE_LIST_BLOB_UNION u = {.as_value = byte_list};
        return u.as_blob;
    }

    const char *lib_ruby_parser_containers_shared_byte_list_blob_as_ptr(SHARED_BYTE_LIST_BLOB_DATA blob) noexcept
    {
        SHARED_BYTE_LIST_BLOB_UNION u = {.as_blob = blob};
        if (u.as_value.length() == 0)
        {
            return nullptr;
        }
        else
        {

            return u.as_value.begin();
        }
    }
    uint64_t lib_ruby_parser_containers_shared_byte_list_blob_len(SHARED_BYTE_LIST_BLOB_DATA blob) noexcept
    {
        SHARED_BYTE_LIST_BLOB_UNION u = {.as_blob = blob};
        return u.as_value.length();
    }
}

// print-sizes
#ifdef PRINT_SIZES
int main()
{
    std::cout << "LIB_RUBY_PARSER_PTR_SIZE = " << sizeof(PTR_BLOB_DATA) << "\n";
    std::cout << "LIB_RUBY_PARSER_MAYBE_PTR_SIZE = " << sizeof(MAYBE_PTR_BLOB_DATA) << "\n";
    std::cout << "LIB_RUBY_PARSER_LIST_SIZE = " << sizeof(LIST_OF_U64_BLOB_DATA) << "\n";
    std::cout << "LIB_RUBY_PARSER_STRING_PTR_SIZE = " << sizeof(STRING_PTR_BLOB_DATA) << "\n";
    std::cout << "LIB_RUBY_PARSER_SHARED_BYTE_LIST_SIZE = " << sizeof(SHARED_BYTE_LIST_BLOB_DATA) << "\n";
}
#endif
