#include "c/declare_blob.h"
#include "c/ptr.h"
#include "c/maybe_ptr.h"
#include "c/list.h"
#include "c/string_ptr.h"
#include "c/shared_byte_list.h"
#include "c/types/bytes.h"

// print-sizes

#ifdef PRINT_SIZES
#include <stdio.h>
int main()
{
    printf("LIB_RUBY_PARSER_PTR_SIZE = %lu\n", sizeof(PTR_BLOB_DATA));
    printf("LIB_RUBY_PARSER_MAYBE_PTR_SIZE = %lu\n", sizeof(MAYBE_PTR_BLOB_DATA));
    printf("LIB_RUBY_PARSER_LIST_SIZE = %lu\n", sizeof(LIST_OF_Node_BLOB_DATA));
    printf("LIB_RUBY_PARSER_STRING_PTR_SIZE = %lu\n", sizeof(STRING_PTR_BLOB_DATA));
    printf("LIB_RUBY_PARSER_SHARED_BYTE_LIST_SIZE = %lu\n", sizeof(SHARED_BYTE_LIST_BLOB_DATA));

    printf("LIB_RUBY_PARSER_BYTES_SIZE = %lu\n", sizeof(BYTES_BLOB_DATA));
}
#endif
