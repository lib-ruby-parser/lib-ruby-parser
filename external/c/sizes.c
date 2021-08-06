#include <stdio.h>

#include "ptr.h"
#include "maybe_ptr.h"
#include "list.h"
#include "string_ptr.h"
#include "shared_byte_list.h"
#include "bytes.h"
#include "token.h"
#include "error_level.h"
#include "loc.h"
#include "comment.h"
#include "messages.h"
#include "maybe_loc.h"

int main()
{
    printf("LIB_RUBY_PARSER_PTR_SIZE = %lu\n", sizeof(PTR));
    printf("LIB_RUBY_PARSER_MAYBE_PTR_SIZE = %lu\n", sizeof(MAYBE_PTR));
    printf("LIB_RUBY_PARSER_LIST_SIZE = %lu\n", sizeof(LIST_OF_Node));
    printf("LIB_RUBY_PARSER_STRING_PTR_SIZE = %lu\n", sizeof(STRING_PTR));
    printf("LIB_RUBY_PARSER_SHARED_BYTE_LIST_SIZE = %lu\n", sizeof(SHARED_BYTE_LIST));

    printf("LIB_RUBY_PARSER_BYTES_SIZE = %lu\n", sizeof(Bytes));
    printf("LIB_RUBY_PARSER_TOKEN_SIZE = %lu\n", sizeof(Token));
    printf("LIB_RUBY_PARSER_ERROR_LEVEL_SIZE = %lu\n", sizeof(ErrorLevel));
    printf("LIB_RUBY_PARSER_LOC_SIZE = %lu\n", sizeof(Loc));
    printf("LIB_RUBY_PARSER_MAYBE_LOC_SIZE = %lu\n", sizeof(MaybeLoc));
    printf("LIB_RUBY_PARSER_COMMENT_TYPE_SIZE = %lu\n", sizeof(CommentType));
    printf("LIB_RUBY_PARSER_COMMENT_SIZE = %lu\n", sizeof(Comment));
    printf("LIB_RUBY_PARSER_MAGIC_COMMENT_KIND_SIZE = %lu\n", sizeof(MagicCommentKind));
    printf("LIB_RUBY_PARSER_MAGIC_COMMENT_SIZE = %lu\n", sizeof(MagicComment));
    printf("LIB_RUBY_PARSER_DIAGNOSTIC_MESSAGE_SIZE = %lu\n", sizeof(DiagnosticMessage));
}
