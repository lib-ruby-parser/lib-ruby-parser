#include <stdlib.h>
#include "structs.h"
#include "messages.h"

// Byte
void LIB_RUBY_PARSER_drop_byte(LIB_RUBY_PARSER_Byte *byte)
{
    (void)byte;
    // noop
}
void LIB_RUBY_PARSER_drop_byte_list(LIB_RUBY_PARSER_ByteList *byte_list)
{
    free(byte_list->ptr);
}

// Ptr

// MaybePtr

// StringPtr
void LIB_RUBY_PARSER_drop_string_ptr(LIB_RUBY_PARSER_StringPtr *string_ptr)
{
    free(string_ptr->ptr);
}

// MaybeStringPtr
void LIB_RUBY_PARSER_drop_maybe_string_ptr(LIB_RUBY_PARSER_MaybeStringPtr *maybe_string_ptr)
{
    if (maybe_string_ptr->ptr == NULL)
    {
        return;
    }
    free(maybe_string_ptr->ptr);
    maybe_string_ptr->len = 0;
    maybe_string_ptr->ptr = NULL;
}

// SharedByteList

// SourceLine
void LIB_RUBY_PARSER_drop_source_line_list(LIB_RUBY_PARSER_SourceLineList *source_line_list)
{
    free(source_line_list->ptr);
}

// Loc
void LIB_RUBY_PARSER_drop_loc(LIB_RUBY_PARSER_Loc *loc)
{
    (void)loc;
}

// MaybeLoc
void LIB_RUBY_PARSER_drop_maybe_loc(LIB_RUBY_PARSER_MaybeLoc *maybe_loc)
{
    (void)maybe_loc;
}

// Bytes
void LIB_RUBY_PARSER_drop_bytes(LIB_RUBY_PARSER_Bytes *bytes)
{
    LIB_RUBY_PARSER_drop_byte_list(&(bytes->raw));
}

// Token
void LIB_RUBY_PARSER_drop_token(LIB_RUBY_PARSER_Token *token)
{
    LIB_RUBY_PARSER_drop_bytes(&(token->token_value));
}
void LIB_RUBY_PARSER_drop_token_list(LIB_RUBY_PARSER_TokenList *token_list)
{
    for (uint64_t i = 0; i < token_list->len; i++)
    {
        LIB_RUBY_PARSER_drop_token(&(token_list->ptr[i]));
    }
    free(token_list->ptr);
}

// CommentType

// Comment
void LIB_RUBY_PARSER_drop_comment_list(LIB_RUBY_PARSER_CommentList *comment_list)
{
    free(comment_list->ptr);
}

// MagicCommentKind

// MagicComment
void LIB_RUBY_PARSER_drop_magic_comment_list(LIB_RUBY_PARSER_MagicCommentList *magic_comment_list)
{
    free(magic_comment_list->ptr);
}

// ErrorLevel

// Diagnostic
void LIB_RUBY_PARSER_drop_diagnostic(LIB_RUBY_PARSER_Diagnostic *diagnostic)
{
    LIB_RUBY_PARSER_drop_diagnostic_message(&(diagnostic->message));
}
void LIB_RUBY_PARSER_drop_diagnostic_list(LIB_RUBY_PARSER_DiagnosticList *diagnostic_list)
{
    for (uint64_t i = 0; i < diagnostic_list->len; i++)
    {
        LIB_RUBY_PARSER_drop_diagnostic(&(diagnostic_list->ptr[i]));
    }
    free(diagnostic_list->ptr);
}

// InputError
void LIB_RUBY_PARSER_drop_input_error(LIB_RUBY_PARSER_InputError *input_error)
{
    switch (input_error->tag)
    {
    case LIB_RUBY_PARSER_INPUT_ERROR_UNSUPPORTED_ENCODING:
        LIB_RUBY_PARSER_drop_string_ptr(&(input_error->as.unsupported_encoding));
        break;
    case LIB_RUBY_PARSER_INPUT_ERROR_DECODING_ERROR:
        LIB_RUBY_PARSER_drop_string_ptr(&(input_error->as.decoding_error));
        break;
    }
}

// DecoderResult
void LIB_RUBY_PARSER_drop_decoder_result(LIB_RUBY_PARSER_DecoderResult *decoder_result)
{
    switch (decoder_result->tag)
    {
    case LIB_RUBY_PARSER_DECODER_RESULT_OK:
        LIB_RUBY_PARSER_drop_byte_list(&(decoder_result->as.ok));
        break;
    case LIB_RUBY_PARSER_DECODER_RESULT_ERR:
        LIB_RUBY_PARSER_drop_input_error(&(decoder_result->as.err));
        break;
    }
}

// TokenRewriter
void LIB_RUBY_PARSER_drop_token_rewriter_result(LIB_RUBY_PARSER_TokenRewriterResult *token_rewriter_result)
{
    LIB_RUBY_PARSER_drop_token(token_rewriter_result->rewritten_token);
}
LIB_RUBY_PARSER_TokenRewriterResult __keep_token(LIB_RUBY_PARSER_Token *token_to_rewrite, LIB_RUBY_PARSER_build_new_token_t build_new_token)
{
    (void)build_new_token;
    LIB_RUBY_PARSER_TokenRewriterResult result = {
        .rewritten_token = token_to_rewrite,
        .lex_state_action = {.tag = LIB_RUBY_PARSER_LEX_STATE_KEEP},
        .token_action = LIB_RUBY_PARSER_REWRITE_ACTION_KEEP};
    return result;
}
LIB_RUBY_PARSER_TokenRewriter __keep_token_rewriter(LIB_RUBY_PARSER_build_new_token_t build_new_token_f)
{
    LIB_RUBY_PARSER_TokenRewriter rewriter = {.build_new_token_f = build_new_token_f, .rewrite_f = __keep_token};
    return rewriter;
}
LIB_RUBY_PARSER_TokenRewriterResult __drop_token(LIB_RUBY_PARSER_Token *token_to_rewrite, LIB_RUBY_PARSER_build_new_token_t build_new_token)
{
    LIB_RUBY_PARSER_drop_token(token_to_rewrite);
    free(token_to_rewrite);
    LIB_RUBY_PARSER_TokenRewriterResult result = {
        .rewritten_token = build_new_token(),
        .lex_state_action = {.tag = LIB_RUBY_PARSER_LEX_STATE_KEEP},
        .token_action = LIB_RUBY_PARSER_REWRITE_ACTION_DROP};
    return result;
}
LIB_RUBY_PARSER_TokenRewriter __drop_token_rewriter(LIB_RUBY_PARSER_build_new_token_t build_new_token_f)
{
    LIB_RUBY_PARSER_TokenRewriter rewriter = {.build_new_token_f = build_new_token_f, .rewrite_f = __drop_token};
    return rewriter;
}
LIB_RUBY_PARSER_TokenRewriterResult __rewrite_token(LIB_RUBY_PARSER_Token *token_to_rewrite, LIB_RUBY_PARSER_build_new_token_t build_new_token)
{
    LIB_RUBY_PARSER_drop_token(token_to_rewrite);
    free(token_to_rewrite);
    LIB_RUBY_PARSER_TokenRewriterResult result = {
        .rewritten_token = build_new_token(),
        .lex_state_action = {.tag = LIB_RUBY_PARSER_LEX_STATE_KEEP},
        .token_action = LIB_RUBY_PARSER_REWRITE_ACTION_KEEP};
    return result;
}
LIB_RUBY_PARSER_TokenRewriter __rewriter_token_rewriter(LIB_RUBY_PARSER_build_new_token_t build_new_token_f)
{
    LIB_RUBY_PARSER_TokenRewriter rewriter = {.build_new_token_f = build_new_token_f, .rewrite_f = __rewrite_token};
    return rewriter;
}

// ParserOptions
void LIB_RUBY_PARSER_drop_parser_options(LIB_RUBY_PARSER_ParserOptions *parser_options)
{
    LIB_RUBY_PARSER_drop_string_ptr(&(parser_options->buffer_name));
}

// DecodedInput
void LIB_RUBY_PARSER_drop_decoded_input(LIB_RUBY_PARSER_DecodedInput *decoded_input)
{
    LIB_RUBY_PARSER_drop_string_ptr(&(decoded_input->name));
    LIB_RUBY_PARSER_drop_source_line_list(&(decoded_input->lines));
    LIB_RUBY_PARSER_drop_byte_list(&(decoded_input->bytes));
}

// ParserResult
void LIB_RUBY_PARSER_drop_parser_result(LIB_RUBY_PARSER_ParserResult *parser_result)
{
    LIB_RUBY_PARSER_drop_maybe_node_ptr(&(parser_result->ast));
    LIB_RUBY_PARSER_drop_token_list(&(parser_result->tokens));
    LIB_RUBY_PARSER_drop_diagnostic_list(&(parser_result->diagnostics));
    LIB_RUBY_PARSER_drop_comment_list(&(parser_result->comments));
    LIB_RUBY_PARSER_drop_magic_comment_list(&(parser_result->magic_comments));
    LIB_RUBY_PARSER_drop_decoded_input(&(parser_result->input));
}
