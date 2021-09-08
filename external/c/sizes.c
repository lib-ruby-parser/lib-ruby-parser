#include <stdio.h>

#include "structs.h"
#include "sizes_gen.h"

int main()
{
    printf("LIB_RUBY_PARSER_PTR_SIZE=%lu\n", sizeof(LIB_RUBY_PARSER_Ptr));
    printf("LIB_RUBY_PARSER_MAYBE_PTR_SIZE=%lu\n", sizeof(LIB_RUBY_PARSER_MaybePtr));
    printf("LIB_RUBY_PARSER_LIST_SIZE=%lu\n", sizeof(LIB_RUBY_PARSER_NodeList));
    printf("LIB_RUBY_PARSER_STRING_PTR_SIZE=%lu\n", sizeof(LIB_RUBY_PARSER_StringPtr));
    printf("LIB_RUBY_PARSER_MAYBE_STRING_PTR_SIZE=%lu\n", sizeof(LIB_RUBY_PARSER_MaybeStringPtr));
    printf("LIB_RUBY_PARSER_SHARED_BYTE_LIST_SIZE=%lu\n", sizeof(LIB_RUBY_PARSER_SharedByteList));

    printf("LIB_RUBY_PARSER_BYTES_SIZE=%lu\n", sizeof(LIB_RUBY_PARSER_Bytes));
    printf("LIB_RUBY_PARSER_TOKEN_SIZE=%lu\n", sizeof(LIB_RUBY_PARSER_Token));
    printf("LIB_RUBY_PARSER_ERROR_LEVEL_SIZE=%lu\n", sizeof(LIB_RUBY_PARSER_ErrorLevel));
    printf("LIB_RUBY_PARSER_LOC_SIZE=%lu\n", sizeof(LIB_RUBY_PARSER_Loc));
    printf("LIB_RUBY_PARSER_MAYBE_LOC_SIZE=%lu\n", sizeof(LIB_RUBY_PARSER_MaybeLoc));
    printf("LIB_RUBY_PARSER_SOURCE_LINE_SIZE=%lu\n", sizeof(LIB_RUBY_PARSER_SourceLine));
    printf("LIB_RUBY_PARSER_COMMENT_TYPE_SIZE=%lu\n", sizeof(LIB_RUBY_PARSER_CommentType));
    printf("LIB_RUBY_PARSER_COMMENT_SIZE=%lu\n", sizeof(LIB_RUBY_PARSER_Comment));
    printf("LIB_RUBY_PARSER_MAGIC_COMMENT_KIND_SIZE=%lu\n", sizeof(LIB_RUBY_PARSER_MagicCommentKind));
    printf("LIB_RUBY_PARSER_MAGIC_COMMENT_SIZE=%lu\n", sizeof(LIB_RUBY_PARSER_MagicComment));
    printf("LIB_RUBY_PARSER_DIAGNOSTIC_MESSAGE_SIZE=%lu\n", sizeof(LIB_RUBY_PARSER_DiagnosticMessage));

    print_node_sizes();

    print_messages_sizes();

    printf("LIB_RUBY_PARSER_DIAGNOSTIC_SIZE=%lu\n", sizeof(LIB_RUBY_PARSER_Diagnostic));
    printf("LIB_RUBY_PARSER_NODE_SIZE=%lu\n", sizeof(LIB_RUBY_PARSER_Node));

    printf("LIB_RUBY_PARSER_INPUT_ERROR_SIZE=%lu\n", sizeof(LIB_RUBY_PARSER_InputError));
    printf("LIB_RUBY_PARSER_DECODER_RESULT_SIZE=%lu\n", sizeof(LIB_RUBY_PARSER_DecoderResult));
    printf("LIB_RUBY_PARSER_DECODER_SIZE=%lu\n", sizeof(LIB_RUBY_PARSER_Decoder));

    printf("LIB_RUBY_PARSER_REWRITE_ACTION_SIZE=%lu\n", sizeof(LIB_RUBY_PARSER_RewriteAction));
    printf("LIB_RUBY_PARSER_LEX_STATE_ACTION_SIZE=%lu\n", sizeof(LIB_RUBY_PARSER_LexStateAction));
    printf("LIB_RUBY_PARSER_TOKEN_REWRITER_RESULT_SIZE=%lu\n", sizeof(LIB_RUBY_PARSER_TokenRewriterResult));
    printf("LIB_RUBY_PARSER_TOKEN_REWRITER_SIZE=%lu\n", sizeof(LIB_RUBY_PARSER_TokenRewriter));

    printf("LIB_RUBY_PARSER_MAYBE_DECODER_SIZE=%lu\n", sizeof(LIB_RUBY_PARSER_MaybeDecoder));
    printf("LIB_RUBY_PARSER_MAYBE_TOKEN_REWRITER_SIZE=%lu\n", sizeof(LIB_RUBY_PARSER_MaybeTokenRewriter));
    printf("LIB_RUBY_PARSER_PARSER_OPTIONS_SIZE=%lu\n", sizeof(LIB_RUBY_PARSER_ParserOptions));

    printf("LIB_RUBY_PARSER_DECODED_INPUT_SIZE=%lu\n", sizeof(LIB_RUBY_PARSER_DecodedInput));
    printf("LIB_RUBY_PARSER_PARSER_RESULT_SIZE=%lu\n", sizeof(LIB_RUBY_PARSER_ParserResult));
}
