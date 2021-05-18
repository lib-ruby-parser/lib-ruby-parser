#include <stddef.h>
#include <stdlib.h>
#include <string.h>

typedef void(Drop)(void *);
typedef int DUMMY;
typedef uint8_t BYTE;

#define DECLARE_BLOB_FOR(VALUE)                                    \
    typedef struct                                                 \
    {                                                              \
        BYTE data[sizeof(VALUE)];                                  \
    } VALUE##_BLOB_DATA;                                           \
                                                                   \
    typedef union                                                  \
    {                                                              \
        _Static_assert(sizeof(VALUE) == sizeof(VALUE##_BLOB_DATA), \
                       "sizeof(VALUE) != sizeof(BLOB_DATA)");      \
                                                                   \
        VALUE as_value;                                            \
        VALUE##_BLOB_DATA as_blob;                                 \
    } VALUE##_BLOB_UNION;

// Ptr<T>
typedef DUMMY *PTR;
_Static_assert(sizeof(PTR) == 8, "wrong sizeof(PTR)");
DECLARE_BLOB_FOR(PTR);

PTR_BLOB_DATA lib_ruby_parser_containers_make_ptr_blob(void *ptr)
{
    PTR_BLOB_UNION u = {.as_value = ptr};
    return u.as_blob;
}

void lib_ruby_parser_containers_free_ptr_blob(PTR_BLOB_DATA blob, Drop drop)
{
    (void)drop;
    PTR_BLOB_UNION u = {.as_blob = blob};
    free(u.as_value);
}

void *lib_ruby_parser_containers_raw_ptr_from_ptr_blob(PTR_BLOB_DATA blob)
{
    PTR_BLOB_UNION u = {.as_blob = blob};
    return u.as_value;
}

PTR_BLOB_DATA lib_ruby_parser_containers_null_ptr_blob()
{
    PTR_BLOB_UNION u = {.as_value = NULL};
    return u.as_blob;
}

// MaybePtr<T>
typedef DUMMY *MAYBE_PTR;
_Static_assert(sizeof(MAYBE_PTR) == 8, "wrong sizeof(MAYBE_PTR)");
DECLARE_BLOB_FOR(MAYBE_PTR);

MAYBE_PTR_BLOB_DATA lib_ruby_parser_containers_make_maybe_ptr_blob(void *ptr)
{
    MAYBE_PTR_BLOB_UNION u = {.as_value = ptr};
    return u.as_blob;
}

void lib_ruby_parser_containers_free_maybe_ptr_blob(MAYBE_PTR_BLOB_DATA blob, Drop drop)
{
    (void)drop;
    MAYBE_PTR_BLOB_UNION u = {.as_blob = blob};
    free(u.as_value);
}

void *lib_ruby_parser_containers_raw_ptr_from_maybe_ptr_blob(MAYBE_PTR_BLOB_DATA blob)
{
    MAYBE_PTR_BLOB_UNION u = {.as_blob = blob};
    return u.as_value;
}

MAYBE_PTR_BLOB_DATA lib_ruby_parser_containers_null_maybe_ptr_blob()
{
    MAYBE_PTR_BLOB_UNION u = {.as_value = NULL};
    return u.as_blob;
}

// List<T>

#define DECLARE_BLOB_FOR_LIST_OF(VALUE, PREFIX)                                                                                    \
    typedef struct                                                                                                                 \
    {                                                                                                                              \
        VALUE##_BLOB_DATA *ptr;                                                                                                    \
        uint64_t size;                                                                                                             \
        uint64_t capacity;                                                                                                         \
    } VALUE##List;                                                                                                                 \
    DECLARE_BLOB_FOR(VALUE##List);                                                                                                 \
    _Static_assert(sizeof(VALUE##List) == 24, "sizeof(List) == 24");                                                               \
                                                                                                                                   \
    VALUE##List lib_ruby_parser_containers_##PREFIX##_unpack_blob(VALUE##List_BLOB_DATA blob)                                      \
    {                                                                                                                              \
        VALUE##List_BLOB_UNION u = {.as_blob = blob};                                                                              \
        return u.as_value;                                                                                                         \
    }                                                                                                                              \
                                                                                                                                   \
    VALUE##List_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_pack_blob(VALUE##List list)                                        \
    {                                                                                                                              \
        VALUE##List_BLOB_UNION u = {.as_value = list};                                                                             \
        return u.as_blob;                                                                                                          \
    }                                                                                                                              \
                                                                                                                                   \
    VALUE##List_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_list_blob_new()                                                    \
    {                                                                                                                              \
        VALUE##List list = {.ptr = NULL, .size = 0, .capacity = 0};                                                                \
        return lib_ruby_parser_containers_##PREFIX##_pack_blob(list);                                                              \
    }                                                                                                                              \
                                                                                                                                   \
    VALUE##List_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_list_blob_with_capacity(uint64_t capacity)                         \
    {                                                                                                                              \
        VALUE##List list = {.ptr = malloc(sizeof(VALUE) * capacity), .size = 0, .capacity = capacity};                             \
        return lib_ruby_parser_containers_##PREFIX##_pack_blob(list);                                                              \
    }                                                                                                                              \
                                                                                                                                   \
    VALUE##List_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_list_blob_from_raw(VALUE##_BLOB_DATA *ptr, uint64_t size)          \
    {                                                                                                                              \
        if (size > 0)                                                                                                              \
        {                                                                                                                          \
            VALUE##List list = {.ptr = ptr, .size = size, .capacity = size};                                                       \
            free(ptr);                                                                                                             \
            return lib_ruby_parser_containers_##PREFIX##_pack_blob(list);                                                          \
        }                                                                                                                          \
        else                                                                                                                       \
        {                                                                                                                          \
            return lib_ruby_parser_containers_##PREFIX##_list_blob_new();                                                          \
        }                                                                                                                          \
    }                                                                                                                              \
                                                                                                                                   \
    VALUE##List_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_list_blob_push(VALUE##List_BLOB_DATA blob, VALUE##_BLOB_DATA item) \
    {                                                                                                                              \
        VALUE##List list = lib_ruby_parser_containers_##PREFIX##_unpack_blob(blob);                                                \
        if (list.size + 1 > list.capacity)                                                                                         \
        {                                                                                                                          \
            list.capacity *= 2;                                                                                                    \
            list.ptr = malloc(sizeof(VALUE) * list.capacity);                                                                      \
        }                                                                                                                          \
        list.ptr[list.size] = item;                                                                                                \
        list.size++;                                                                                                               \
        return lib_ruby_parser_containers_##PREFIX##_pack_blob(list);                                                              \
    }                                                                                                                              \
                                                                                                                                   \
    typedef struct                                                                                                                 \
    {                                                                                                                              \
        VALUE##List_BLOB_DATA new_blob;                                                                                            \
        VALUE##_BLOB_DATA removed_item;                                                                                            \
    } VALUE##List_REMOVE_RESULT;                                                                                                   \
                                                                                                                                   \
    VALUE##List_REMOVE_RESULT lib_ruby_parser_containers_##PREFIX##_list_blob_remove(VALUE##List_BLOB_DATA blob, uint64_t index)   \
    {                                                                                                                              \
        VALUE##List list = lib_ruby_parser_containers_##PREFIX##_unpack_blob(blob);                                                \
        VALUE##_BLOB_DATA item = list.ptr[index];                                                                                  \
        memcpy(list.ptr + index, list.ptr + index + 1, list.size - index - 1);                                                     \
        list.size--;                                                                                                               \
        VALUE##List_REMOVE_RESULT result = {                                                                                       \
            .new_blob = lib_ruby_parser_containers_##PREFIX##_pack_blob(list),                                                     \
            .removed_item = item};                                                                                                 \
                                                                                                                                   \
        return result;                                                                                                             \
    }                                                                                                                              \
                                                                                                                                   \
    VALUE##List_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_list_blob_shrink_to_fit(VALUE##List_BLOB_DATA blob)                \
    {                                                                                                                              \
        VALUE##List list = lib_ruby_parser_containers_##PREFIX##_unpack_blob(blob);                                                \
        uint64_t new_len = list.capacity;                                                                                          \
        VALUE##List new_list = {.ptr = malloc(sizeof(VALUE) * new_len), .size = new_len, .capacity = new_len};                     \
        memcpy(new_list.ptr, list.ptr, new_len);                                                                                   \
        free(list.ptr);                                                                                                            \
        return lib_ruby_parser_containers_##PREFIX##_pack_blob(new_list);                                                          \
    }                                                                                                                              \
                                                                                                                                   \
    VALUE##_BLOB_DATA *lib_ruby_parser_containers_##PREFIX##_list_blob_as_ptr(VALUE##List_BLOB_DATA blob)                          \
    {                                                                                                                              \
        VALUE##List list = lib_ruby_parser_containers_##PREFIX##_unpack_blob(blob);                                                \
        return list.ptr;                                                                                                           \
    }                                                                                                                              \
                                                                                                                                   \
    uint64_t lib_ruby_parser_containers_##PREFIX##_list_blob_len(VALUE##List_BLOB_DATA blob)                                       \
    {                                                                                                                              \
        VALUE##List list = lib_ruby_parser_containers_##PREFIX##_unpack_blob(blob);                                                \
        return list.size;                                                                                                          \
    }                                                                                                                              \
                                                                                                                                   \
    uint64_t lib_ruby_parser_containers_##PREFIX##_list_blob_capacity(VALUE##List_BLOB_DATA blob)                                  \
    {                                                                                                                              \
        VALUE##List list = lib_ruby_parser_containers_##PREFIX##_unpack_blob(blob);                                                \
        return list.capacity;                                                                                                      \
    }                                                                                                                              \
                                                                                                                                   \
    void lib_ruby_parser_containers_##PREFIX##_list_blob_free(                                                                     \
        VALUE##List_BLOB_DATA blob, Drop drop_ptr_in_place)                                                                        \
    {                                                                                                                              \
        VALUE##List list = lib_ruby_parser_containers_##PREFIX##_unpack_blob(blob);                                                \
        for (size_t i = 0; i < list.size; i++)                                                                                     \
        {                                                                                                                          \
            drop_ptr_in_place(&list.ptr[i]);                                                                                       \
        }                                                                                                                          \
    }

typedef struct
{
    BYTE data[184];
} NodeStruct;
DECLARE_BLOB_FOR(NodeStruct);
DECLARE_BLOB_FOR_LIST_OF(NodeStruct, node);

typedef struct
{
    BYTE data[40];
} DiagnosticStruct;
DECLARE_BLOB_FOR(DiagnosticStruct);
DECLARE_BLOB_FOR_LIST_OF(DiagnosticStruct, diagnostic);

typedef struct
{
    BYTE data[24];
} ComentStruct;
DECLARE_BLOB_FOR(ComentStruct);
DECLARE_BLOB_FOR_LIST_OF(ComentStruct, comment);

typedef struct
{
    BYTE data[40];
} MagicCommentStruct;
DECLARE_BLOB_FOR(MagicCommentStruct);
DECLARE_BLOB_FOR_LIST_OF(MagicCommentStruct, magic_comment);

typedef struct
{
    BYTE data[56];
} TokenStruct;
DECLARE_BLOB_FOR(TokenStruct);
DECLARE_BLOB_FOR_LIST_OF(TokenStruct, token);

typedef struct
{
    BYTE data[24];
} SourceLineStruct;
DECLARE_BLOB_FOR(SourceLineStruct);
DECLARE_BLOB_FOR_LIST_OF(SourceLineStruct, source_line);

typedef struct
{
    BYTE data[1];
} ByteStruct;
DECLARE_BLOB_FOR(ByteStruct);
DECLARE_BLOB_FOR_LIST_OF(ByteStruct, byte);

// print-sizes

#ifdef PRINT_SIZES
#include <stdio.h>
int main()
{
    printf("LIB_RUBY_PARSER_PTR_SIZE = %lu\n", sizeof(PTR_BLOB));
    printf("LIB_RUBY_PARSER_MAYBE_PTR_SIZE = %lu\n", sizeof(MAYBE_PTR_BLOB));
    printf("LIB_RUBY_PARSER_LIST_SIZE = TBD\n");
    printf("LIB_RUBY_PARSER_STRING_PTR_SIZE = TBD\n");
}
#endif
