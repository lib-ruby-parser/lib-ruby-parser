#include <stddef.h>
#include <stdlib.h>

typedef void(Drop)(void *);

// Ptr<T>
typedef struct
{
    uint8_t data[8];
} PTR_BLOB;
_Static_assert(sizeof(void *) == sizeof(PTR_BLOB), "sizeof(T*) must be 8 bytes");

typedef union
{
    PTR_BLOB as_blob;
    void *as_ptr;
} PTR_BLOB_UNION;

PTR_BLOB lib_ruby_parser_containers_make_ptr_blob(void *ptr)
{
    PTR_BLOB_UNION u = {.as_ptr = ptr};
    return u.as_blob;
}

void lib_ruby_parser_containers_free_ptr_blob(PTR_BLOB blob, Drop drop)
{
    (void)drop;
    PTR_BLOB_UNION u = {.as_blob = blob};
    free(u.as_ptr);
}

void *lib_ruby_parser_containers_raw_ptr_from_ptr_blob(PTR_BLOB blob)
{
    PTR_BLOB_UNION u = {.as_blob = blob};
    return u.as_ptr;
}

PTR_BLOB lib_ruby_parser_containers_null_ptr_blob()
{
    PTR_BLOB_UNION u = {.as_ptr = NULL};
    return u.as_blob;
}

// MaybePtr<T>
typedef struct
{
    uint8_t data[8];
} MAYBE_PTR_BLOB;
_Static_assert(sizeof(void *) == sizeof(MAYBE_PTR_BLOB), "sizeof(T*) must be 8 bytes");

typedef union
{
    MAYBE_PTR_BLOB as_blob;
    void *as_ptr;
} MAYBE_PTR_BLOB_UNION;

MAYBE_PTR_BLOB lib_ruby_parser_containers_make_maybe_ptr_blob(void *ptr)
{
    MAYBE_PTR_BLOB_UNION u = {.as_ptr = ptr};
    return u.as_blob;
}

void lib_ruby_parser_containers_free_maybe_ptr_blob(MAYBE_PTR_BLOB blob, Drop drop)
{
    (void)drop;
    MAYBE_PTR_BLOB_UNION u = {.as_blob = blob};
    free(u.as_ptr);
}

void *lib_ruby_parser_containers_raw_ptr_from_maybe_ptr_blob(MAYBE_PTR_BLOB blob)
{
    MAYBE_PTR_BLOB_UNION u = {.as_blob = blob};
    return u.as_ptr;
}

MAYBE_PTR_BLOB lib_ruby_parser_containers_null_maybe_ptr_blob()
{
    MAYBE_PTR_BLOB_UNION u = {.as_ptr = NULL};
    return u.as_blob;
}

// List<T>
typedef struct
{
    uint8_t data[64];
} LIST_BLOB;

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
