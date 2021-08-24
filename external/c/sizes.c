#include <stdio.h>

#include "structs.h"
#include "nodes.h"
#include "messages.h"

int main()
{
    printf("LIB_RUBY_PARSER_PTR_SIZE=%lu\n", sizeof(Ptr));
    printf("LIB_RUBY_PARSER_MAYBE_PTR_SIZE=%lu\n", sizeof(MaybePtr));
    printf("LIB_RUBY_PARSER_LIST_SIZE=%lu\n", sizeof(NodeList));
    printf("LIB_RUBY_PARSER_STRING_PTR_SIZE=%lu\n", sizeof(StringPtr));
    printf("LIB_RUBY_PARSER_MAYBE_STRING_PTR_SIZE=%lu\n", sizeof(MaybeStringPtr));
    printf("LIB_RUBY_PARSER_SHARED_BYTE_LIST_SIZE=%lu\n", sizeof(SharedByteList));

    printf("LIB_RUBY_PARSER_BYTES_SIZE=%lu\n", sizeof(Bytes));
    printf("LIB_RUBY_PARSER_TOKEN_SIZE=%lu\n", sizeof(Token));
    printf("LIB_RUBY_PARSER_ERROR_LEVEL_SIZE=%lu\n", sizeof(ErrorLevel));
    printf("LIB_RUBY_PARSER_LOC_SIZE=%lu\n", sizeof(Loc));
    printf("LIB_RUBY_PARSER_MAYBE_LOC_SIZE=%lu\n", sizeof(MaybeLoc));
    printf("LIB_RUBY_PARSER_SOURCE_LINE_SIZE=%lu\n", sizeof(SourceLine));
    printf("LIB_RUBY_PARSER_COMMENT_TYPE_SIZE=%lu\n", sizeof(CommentType));
    printf("LIB_RUBY_PARSER_COMMENT_SIZE=%lu\n", sizeof(Comment));
    printf("LIB_RUBY_PARSER_MAGIC_COMMENT_KIND_SIZE=%lu\n", sizeof(MagicCommentKind));
    printf("LIB_RUBY_PARSER_MAGIC_COMMENT_SIZE=%lu\n", sizeof(MagicComment));
    printf("LIB_RUBY_PARSER_DIAGNOSTIC_MESSAGE_SIZE=%lu\n", sizeof(DiagnosticMessage));

    NODE_PRINT_SIZES

    MESSAGE_PRINT_SIZES

    printf("LIB_RUBY_PARSER_DIAGNOSTIC_SIZE=%lu\n", sizeof(Diagnostic));
    printf("LIB_RUBY_PARSER_NODE_SIZE=%lu\n", sizeof(Node));

    printf("LIB_RUBY_PARSER_INPUT_ERROR_SIZE=%lu\n", sizeof(InputError));
    printf("LIB_RUBY_PARSER_DECODER_RESULT_SIZE=%lu\n", sizeof(DecoderResult));
    printf("LIB_RUBY_PARSER_DECODER_SIZE=%lu\n", sizeof(Decoder));

    printf("LIB_RUBY_PARSER_REWRITE_ACTION_SIZE=%lu\n", sizeof(RewriteAction));
    printf("LIB_RUBY_PARSER_LEX_STATE_ACTION_SIZE=%lu\n", sizeof(LexStateAction));
    printf("LIB_RUBY_PARSER_TOKEN_REWRITER_RESULT_SIZE=%lu\n", sizeof(TokenRewriterResult));
}
