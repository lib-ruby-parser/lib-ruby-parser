#include <iostream>

#include "structs.hpp"
#include "nodes.hpp"
#include "messages.hpp"

int main()
{
    std::cout << "LIB_RUBY_PARSER_PTR_SIZE=" << sizeof(Ptr) << '\n';
    std::cout << "LIB_RUBY_PARSER_MAYBE_PTR_SIZE=" << sizeof(MaybePtr) << '\n';
    std::cout << "LIB_RUBY_PARSER_LIST_SIZE=" << sizeof(NodeList) << '\n';
    std::cout << "LIB_RUBY_PARSER_STRING_PTR_SIZE=" << sizeof(StringPtr) << '\n';
    std::cout << "LIB_RUBY_PARSER_MAYBE_STRING_PTR_SIZE=" << sizeof(MaybeStringPtr) << '\n';
    std::cout << "LIB_RUBY_PARSER_SHARED_BYTE_LIST_SIZE=" << sizeof(SharedByteList) << '\n';

    std::cout << "LIB_RUBY_PARSER_BYTES_SIZE=" << sizeof(Bytes) << '\n';
    std::cout << "LIB_RUBY_PARSER_TOKEN_SIZE=" << sizeof(Token) << '\n';
    std::cout << "LIB_RUBY_PARSER_ERROR_LEVEL_SIZE=" << sizeof(ErrorLevel) << '\n';
    std::cout << "LIB_RUBY_PARSER_LOC_SIZE=" << sizeof(Loc) << '\n';
    std::cout << "LIB_RUBY_PARSER_MAYBE_LOC_SIZE=" << sizeof(MaybeLoc) << '\n';
    std::cout << "LIB_RUBY_PARSER_SOURCE_LINE_SIZE=" << sizeof(SourceLine) << '\n';
    std::cout << "LIB_RUBY_PARSER_COMMENT_TYPE_SIZE=" << sizeof(CommentType) << '\n';
    std::cout << "LIB_RUBY_PARSER_COMMENT_SIZE=" << sizeof(Comment) << '\n';
    std::cout << "LIB_RUBY_PARSER_MAGIC_COMMENT_KIND_SIZE=" << sizeof(MagicCommentKind) << '\n';
    std::cout << "LIB_RUBY_PARSER_MAGIC_COMMENT_SIZE=" << sizeof(MagicComment) << '\n';
    std::cout << "LIB_RUBY_PARSER_DIAGNOSTIC_MESSAGE_SIZE=" << sizeof(DiagnosticMessage) << '\n';

    NODE_PRINT_SIZES

    MESSAGE_PRINT_SIZES

    std::cout << "LIB_RUBY_PARSER_DIAGNOSTIC_SIZE=" << sizeof(Diagnostic) << '\n';
    std::cout << "LIB_RUBY_PARSER_NODE_SIZE=" << sizeof(Node) << '\n';

    std::cout << "LIB_RUBY_PARSER_INPUT_ERROR_SIZE=" << sizeof(InputError) << '\n';
    std::cout << "LIB_RUBY_PARSER_DECODER_RESULT_SIZE=" << sizeof(DecoderResult) << '\n';
    std::cout << "LIB_RUBY_PARSER_DECODER_SIZE=" << sizeof(Decoder) << '\n';

    std::cout << "LIB_RUBY_PARSER_REWRITE_ACTION_SIZE=" << sizeof(RewriteAction) << '\n';
    std::cout << "LIB_RUBY_PARSER_LEX_STATE_ACTION_SIZE=" << sizeof(LexStateAction) << '\n';
    std::cout << "LIB_RUBY_PARSER_TOKEN_REWRITER_RESULT_SIZE=" << sizeof(TokenRewriterResult) << '\n';
    std::cout << "LIB_RUBY_PARSER_TOKEN_REWRITER_SIZE=" << sizeof(TokenRewriter) << '\n';

    std::cout << "LIB_RUBY_PARSER_MAYBE_DECODER_SIZE=" << sizeof(MaybeDecoder) << '\n';
    std::cout << "LIB_RUBY_PARSER_MAYBE_TOKEN_REWRITER_SIZE=" << sizeof(MaybeTokenRewriter) << '\n';
    std::cout << "LIB_RUBY_PARSER_PARSER_OPTIONS_SIZE=" << sizeof(ParserOptions) << '\n';

    std::cout << "LIB_RUBY_PARSER_DECODED_INPUT_SIZE=" << sizeof(DecodedInput) << '\n';
    std::cout << "LIB_RUBY_PARSER_PARSER_RESULT_SIZE=" << sizeof(ParserResult) << '\n';
}
