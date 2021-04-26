#include <stddef.h>
#include <stdlib.h>

// Ptr<T>
_Static_assert(sizeof(void *) == sizeof(uint64_t), "sizeof(T*) must be 8 bytes");

typedef void *PTR_BLOB;
typedef void(Deleter)(void *);

PTR_BLOB lib_ruby_parser_containers_make_ptr_blob(void *ptr)
{
    return ptr;
}

void lib_ruby_parser_containers_free_ptr_blob(PTR_BLOB blob, Deleter deleter)
{
    (void)deleter;
    free(blob);
}

void *lib_ruby_parser_containers_raw_ptr_from_ptr_blob(PTR_BLOB blob)
{
    return blob;
}

PTR_BLOB lib_ruby_parser_containers_null_ptr_blob()
{
    return NULL;
}
