#include <iostream>
#include <sstream>
#include <memory>
#include <string>
#include <vector>
#include <cstdint>

extern "C" typedef void(Drop)(void *);
typedef int DUMMY;
typedef uint8_t BYTE;

#define DECLARE_BLOB(BLOB_DATA, BLOB_UNION, VALUE)                      \
    extern "C"                                                          \
    {                                                                   \
        struct BLOB_DATA                                                \
        {                                                               \
            BYTE data[sizeof(VALUE)];                                   \
        };                                                              \
    }                                                                   \
                                                                        \
    BLOB_DATA make_##BLOB_DATA(BYTE start) noexcept                     \
    {                                                                   \
        BLOB_DATA blob_data;                                            \
        BYTE value = start;                                             \
        for (size_t i = 0; i < sizeof(VALUE); i++)                      \
        {                                                               \
            blob_data.data[i] = value;                                  \
            value++;                                                    \
        }                                                               \
        return blob_data;                                               \
    }                                                                   \
                                                                        \
    std::string BLOB_DATA##_to_string(BLOB_DATA blob_data) noexcept     \
    {                                                                   \
        std::stringstream ss;                                           \
        std::string output;                                             \
        ss << "BlobData<" << sizeof(VALUE) << ">(";                     \
        for (size_t i = 0; i < sizeof(VALUE); i++)                      \
        {                                                               \
            if (i != 0)                                                 \
            {                                                           \
                ss << ",";                                              \
            }                                                           \
            ss << std::hex << (int)(blob_data.data[i]) << std::dec;     \
        }                                                               \
        ss << ";; 0 = " << std::hex << 0 << ", 1 = " << 1 << std::dec;  \
        ss << ")";                                                      \
        return ss.str();                                                \
    }                                                                   \
                                                                        \
    extern "C"                                                          \
    {                                                                   \
        union BLOB_UNION                                                \
        {                                                               \
            typedef VALUE value_t;                                      \
            typedef BLOB_DATA blob_t;                                   \
                                                                        \
            _Static_assert(sizeof(value_t) == sizeof(blob_t));          \
                                                                        \
            value_t as_value;                                           \
            blob_t as_blob;                                             \
                                                                        \
            ~BLOB_UNION() noexcept                                      \
            {                                                           \
            }                                                           \
                                                                        \
            std::string to_string() noexcept                            \
            {                                                           \
                std::stringstream ss;                                   \
                ss << "Blob(" << BLOB_DATA##_to_string(as_blob) << ")"; \
                return ss.str();                                        \
            }                                                           \
        };                                                              \
    }

// Ptr<T>
typedef std::unique_ptr<DUMMY> Ptr;
_Static_assert(sizeof(Ptr) == 8);
DECLARE_BLOB(PTR_BLOB_DATA, PTR_BLOB_UNION, Ptr);

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
typedef std::unique_ptr<DUMMY> MaybePtr;
_Static_assert(sizeof(MaybePtr) == 8);
DECLARE_BLOB(MAYBE_PTR_BLOB_DATA, MAYBE_PTR_BLOB_UNION, MaybePtr);

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

#define DECLARE_BLOB_FOR_LIST_OF(ITEM_BLOB_DATA, ITEM_BLOB_UNION, PREFIX)                                                                                                 \
    DECLARE_BLOB(LIST_OF_##ITEM_BLOB_DATA, LIST_OF_##ITEM_BLOB_UNION, std::vector<ITEM_BLOB_DATA>);                                                                       \
    _Static_assert(sizeof(std::vector<ITEM_BLOB_DATA>) == 24);                                                                                                            \
                                                                                                                                                                          \
    std::vector<ITEM_BLOB_DATA> lib_ruby_parser_containers_##PREFIX##_unpack_blob(LIST_OF_##ITEM_BLOB_DATA blob) noexcept                                                 \
    {                                                                                                                                                                     \
        LIST_OF_##ITEM_BLOB_UNION u = {.as_blob = blob};                                                                                                                  \
        return std::move(u.as_value);                                                                                                                                     \
    }                                                                                                                                                                     \
                                                                                                                                                                          \
    LIST_OF_##ITEM_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_pack_blob(std::vector<ITEM_BLOB_DATA> vec) noexcept                                                    \
    {                                                                                                                                                                     \
        LIST_OF_##ITEM_BLOB_UNION u = {.as_value = std::move(vec)};                                                                                                       \
        return u.as_blob;                                                                                                                                                 \
    }                                                                                                                                                                     \
                                                                                                                                                                          \
    extern "C"                                                                                                                                                            \
    {                                                                                                                                                                     \
        LIST_OF_##ITEM_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_list_blob_new() noexcept                                                                           \
        {                                                                                                                                                                 \
            return lib_ruby_parser_containers_##PREFIX##_pack_blob(std::vector<ITEM_BLOB_DATA>());                                                                        \
        }                                                                                                                                                                 \
                                                                                                                                                                          \
        LIST_OF_##ITEM_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_list_blob_with_capacity(uint64_t capacity) noexcept                                                \
        {                                                                                                                                                                 \
            std::vector<ITEM_BLOB_DATA> vec;                                                                                                                              \
            vec.reserve(capacity);                                                                                                                                        \
            return lib_ruby_parser_containers_##PREFIX##_pack_blob(std::move(vec));                                                                                       \
        }                                                                                                                                                                 \
                                                                                                                                                                          \
        LIST_OF_##ITEM_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_list_blob_from_raw(ITEM_BLOB_DATA *ptr, uint64_t size) noexcept                                    \
        {                                                                                                                                                                 \
            if (size > 0)                                                                                                                                                 \
            {                                                                                                                                                             \
                auto vec = std::vector<ITEM_BLOB_DATA>(ptr, ptr + size);                                                                                                  \
                free(ptr);                                                                                                                                                \
                return lib_ruby_parser_containers_##PREFIX##_pack_blob(std::move(vec));                                                                                   \
            }                                                                                                                                                             \
            else                                                                                                                                                          \
            {                                                                                                                                                             \
                return lib_ruby_parser_containers_##PREFIX##_list_blob_new();                                                                                             \
            }                                                                                                                                                             \
        }                                                                                                                                                                 \
                                                                                                                                                                          \
        LIST_OF_##ITEM_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_list_blob_push(LIST_OF_##ITEM_BLOB_DATA blob, ITEM_BLOB_DATA item) noexcept                        \
        {                                                                                                                                                                 \
            std::vector<ITEM_BLOB_DATA> vec = lib_ruby_parser_containers_##PREFIX##_unpack_blob(blob);                                                                    \
            vec.push_back(item);                                                                                                                                          \
                                                                                                                                                                          \
            {                                                                                                                                                             \
                auto x = vec;                                                                                                                                             \
            }                                                                                                                                                             \
            return lib_ruby_parser_containers_##PREFIX##_pack_blob(std::move(vec));                                                                                       \
        }                                                                                                                                                                 \
                                                                                                                                                                          \
        typedef struct                                                                                                                                                    \
        {                                                                                                                                                                 \
            LIST_OF_##ITEM_BLOB_DATA new_blob;                                                                                                                            \
            ITEM_BLOB_DATA removed_item;                                                                                                                                  \
        } LIB_RUBY_PARSER_LIST_BLOB_##PREFIX##_REMOVE_RESULT;                                                                                                             \
                                                                                                                                                                          \
        LIB_RUBY_PARSER_LIST_BLOB_##PREFIX##_REMOVE_RESULT lib_ruby_parser_containers_##PREFIX##_list_blob_remove(LIST_OF_##ITEM_BLOB_DATA blob, uint64_t index) noexcept \
        {                                                                                                                                                                 \
            std::vector<ITEM_BLOB_DATA> vec = lib_ruby_parser_containers_##PREFIX##_unpack_blob(blob);                                                                    \
            ITEM_BLOB_DATA item = std::move(vec[index]);                                                                                                                  \
            vec.erase(vec.begin() + index);                                                                                                                               \
            LIB_RUBY_PARSER_LIST_BLOB_##PREFIX##_REMOVE_RESULT result = {                                                                                                 \
                .new_blob = lib_ruby_parser_containers_##PREFIX##_pack_blob(std::move(vec)),                                                                              \
                .removed_item = item};                                                                                                                                    \
                                                                                                                                                                          \
            return result;                                                                                                                                                \
        }                                                                                                                                                                 \
                                                                                                                                                                          \
        LIST_OF_##ITEM_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_list_blob_shrink_to_fit(LIST_OF_##ITEM_BLOB_DATA blob) noexcept                                    \
        {                                                                                                                                                                 \
            std::vector<ITEM_BLOB_DATA> vec = lib_ruby_parser_containers_##PREFIX##_unpack_blob(blob);                                                                    \
            vec.shrink_to_fit();                                                                                                                                          \
            return lib_ruby_parser_containers_##PREFIX##_pack_blob(std::move(vec));                                                                                       \
        }                                                                                                                                                                 \
                                                                                                                                                                          \
        ITEM_BLOB_DATA *lib_ruby_parser_containers_##PREFIX##_list_blob_as_ptr(LIST_OF_##ITEM_BLOB_DATA blob) noexcept                                                    \
        {                                                                                                                                                                 \
            std::vector<ITEM_BLOB_DATA> vec = lib_ruby_parser_containers_##PREFIX##_unpack_blob(blob);                                                                    \
            auto result = vec.data();                                                                                                                                     \
            lib_ruby_parser_containers_##PREFIX##_pack_blob(std::move(vec));                                                                                              \
            return result;                                                                                                                                                \
        }                                                                                                                                                                 \
                                                                                                                                                                          \
        uint64_t lib_ruby_parser_containers_##PREFIX##_list_blob_len(LIST_OF_##ITEM_BLOB_DATA blob) noexcept                                                              \
        {                                                                                                                                                                 \
            std::vector<ITEM_BLOB_DATA> vec = lib_ruby_parser_containers_##PREFIX##_unpack_blob(blob);                                                                    \
            auto result = vec.size();                                                                                                                                     \
            lib_ruby_parser_containers_##PREFIX##_pack_blob(std::move(vec));                                                                                              \
            return result;                                                                                                                                                \
        }                                                                                                                                                                 \
                                                                                                                                                                          \
        uint64_t lib_ruby_parser_containers_##PREFIX##_list_blob_capacity(LIST_OF_##ITEM_BLOB_DATA blob) noexcept                                                         \
        {                                                                                                                                                                 \
            std::vector<ITEM_BLOB_DATA> vec = lib_ruby_parser_containers_##PREFIX##_unpack_blob(blob);                                                                    \
            auto result = vec.capacity();                                                                                                                                 \
            lib_ruby_parser_containers_##PREFIX##_pack_blob(std::move(vec));                                                                                              \
            return result;                                                                                                                                                \
        }                                                                                                                                                                 \
                                                                                                                                                                          \
        void lib_ruby_parser_containers_##PREFIX##_list_blob_free(LIST_OF_##ITEM_BLOB_DATA blob, Drop drop_ptr_in_place) noexcept                                         \
        {                                                                                                                                                                 \
            std::vector<ITEM_BLOB_DATA> vec = lib_ruby_parser_containers_##PREFIX##_unpack_blob(blob);                                                                    \
            for (size_t i = 0; i < vec.size(); i++)                                                                                                                       \
            {                                                                                                                                                             \
                drop_ptr_in_place(&vec[i]);                                                                                                                               \
            }                                                                                                                                                             \
        }                                                                                                                                                                 \
    }

struct NodeStruct
{
    BYTE data[184];
};
DECLARE_BLOB(NODE_BLOB_DATA, NODE_BLOB_UNION, NodeStruct);
DECLARE_BLOB_FOR_LIST_OF(NODE_BLOB_DATA, NODE_BLOB_UNION, node);

struct DiagnosticStruct
{
    BYTE data[40];
};
DECLARE_BLOB(DIAGNOSTIC_BLOB_DATA, DIAGNOSTIC_BLOB_UNION, DiagnosticStruct);
DECLARE_BLOB_FOR_LIST_OF(DIAGNOSTIC_BLOB_DATA, DIAGNOSTIC_BLOB_UNION, diagnostic);

struct ComentStruct
{
    BYTE data[24];
};
DECLARE_BLOB(COMMENT_BLOB_DATA, COMMENT_BLOB_UNION, ComentStruct);
DECLARE_BLOB_FOR_LIST_OF(COMMENT_BLOB_DATA, COMMENT_BLOB_UNION, comment);

struct MagicCommentStruct
{
    BYTE data[40];
};
DECLARE_BLOB(MAGIC_COMMENT_BLOB_DATA, MAGIC_COMMENT_BLOB_UNION, MagicCommentStruct);
DECLARE_BLOB_FOR_LIST_OF(MAGIC_COMMENT_BLOB_DATA, MAGIC_COMMENT_BLOB_UNION, magic_comment);

struct TokenStruct
{
    BYTE data[56];
};
DECLARE_BLOB(TOKEN_BLOB_DATA, TOKEN_BLOB_UNION, TokenStruct);
DECLARE_BLOB_FOR_LIST_OF(TOKEN_BLOB_DATA, TOKEN_BLOB_UNION, token);

struct SourceLineStruct
{
    BYTE data[24];
};
DECLARE_BLOB(SOURCE_LINE_BLOB_DATA, SOURCE_LINE_BLOB_UNION, SourceLineStruct);
DECLARE_BLOB_FOR_LIST_OF(SOURCE_LINE_BLOB_DATA, SOURCE_LINE_BLOB_UNION, source_line);

struct ByteStruct
{
    BYTE data[1];
};
DECLARE_BLOB(BYTE_BLOB_DATA, BYTE_BLOB_UNION, ByteStruct);
DECLARE_BLOB_FOR_LIST_OF(BYTE_BLOB_DATA, BYTE_BLOB_UNION, byte);

struct U64Struct
{
    uint64_t data;
    friend std::ostream &operator<<(std::ostream &os, const U64Struct &u64) noexcept
    {
        os << u64.data;
        return os;
    }
};
DECLARE_BLOB(U64_BLOB_DATA, U64_BLOB_UNION, U64Struct);
DECLARE_BLOB_FOR_LIST_OF(U64_BLOB_DATA, U64_BLOB_UNION, u64);

// StringPtr<T>
// Small strings have optimization that forces string content
// to be stored INSIDE the string container.
// Because of that moved small string has a different c_str()
// which prevents us from sharing it with Rust
typedef std::unique_ptr<std::string> STRING_PTR;
_Static_assert(sizeof(STRING_PTR) == 8);
DECLARE_BLOB(STRING_PTR_BLOB_DATA, STRING_PTR_BLOB_UNION, STRING_PTR);

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

#ifdef PRINT_SIZES
int main()
{
    std::cout << "LIB_RUBY_PARSER_PTR_SIZE = " << sizeof(PTR_BLOB_DATA) << "\n";
    std::cout << "LIB_RUBY_PARSER_MAYBE_PTR_SIZE = " << sizeof(MAYBE_PTR_BLOB_DATA) << "\n";
    std::cout << "LIB_RUBY_PARSER_LIST_SIZE = " << sizeof(LIST_OF_U64_BLOB_DATA) << "\n";
    std::cout << "LIB_RUBY_PARSER_STRING_PTR_SIZE = " << sizeof(STRING_PTR_BLOB_DATA) << "\n";
}
#endif
