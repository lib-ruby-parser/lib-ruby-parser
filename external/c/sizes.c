#include <stdio.h>

#include "ptr.h"
#include "maybe_ptr.h"
#include "list.h"
#include "string_ptr.h"
#include "shared_byte_list.h"
#include "bytes.h"

int main()
{
    printf("LIB_RUBY_PARSER_PTR_SIZE = %lu\n", sizeof(PTR_BLOB_DATA));
    printf("LIB_RUBY_PARSER_MAYBE_PTR_SIZE = %lu\n", sizeof(MAYBE_PTR_BLOB_DATA));
    printf("LIB_RUBY_PARSER_LIST_SIZE = %lu\n", sizeof(LIST_OF_Node_BLOB_DATA));
    printf("LIB_RUBY_PARSER_STRING_PTR_SIZE = %lu\n", sizeof(STRING_PTR_BLOB_DATA));
    printf("LIB_RUBY_PARSER_SHARED_BYTE_LIST_SIZE = %lu\n", sizeof(SHARED_BYTE_LIST_BLOB_DATA));

    printf("LIB_RUBY_PARSER_BYTES_SIZE = %lu\n", sizeof(BYTES_BLOB_DATA));
}
