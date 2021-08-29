#include "structs.hpp"

// Byte

// Ptr

// MaybePtr

// StringPtr

// MaybeStringPtr

// SharedByteList

// SourceLine
SourceLine::SourceLine(uint64_t start,
                       uint64_t end,
                       bool ends_with_eof) : start(start), end(end), ends_with_eof(ends_with_eof) {}

// Loc
Loc::Loc(uint64_t begin, uint64_t end) : begin(begin), end(end) {}

// MaybeLoc

// Bytes
Bytes::Bytes(ByteList raw) : raw(std::move(raw)) {}

// Token
Token::Token(
    uint32_t token_type,
    Bytes token_value,
    Loc loc,
    uint32_t lex_state_before,
    uint32_t lex_state_after) : token_type(token_type),
                                token_value(std::move(token_value)),
                                loc(std::move(loc)),
                                lex_state_before(lex_state_before),
                                lex_state_after(lex_state_after) {}

// CommentType

// Comment
Comment::Comment(Loc location, CommentType kind) : location(std::move(location)), kind(kind) {}

// MagicCommentKind

// MagicComment
MagicComment::MagicComment(
    MagicCommentKind kind,
    Loc key_l,
    Loc value_l) : kind(kind),
                   key_l(std::move(key_l)),
                   value_l(std::move(value_l)) {}

// ErrorLevel

// Diagnostic
void drop_diagnostic(Diagnostic *diagnostic)
{
    diagnostic->~Diagnostic();
}

// InputError
InputError::UnsupportedEncoding::UnsupportedEncoding(StringPtr message) : message(std::move(message)) {}
InputError::DecodingError::DecodingError(StringPtr message) : message(std::move(message)) {}
InputError::InputError(InputError::variant_t variant) : variant(std::move(variant)) {}

// DecoderResult
DecoderResult::Ok::Ok(ByteList output) : output(std::move(output)) {}
DecoderResult::Err::Err(InputError error) : error(std::move(error)) {}
DecoderResult::DecoderResult(DecoderResult::variant_t variant) : variant(std::move(variant)) {}

// TokenRewriter
LexStateAction::LexStateAction(Kind kind, int32_t next_state) : kind(kind), next_state(next_state) {}

LexStateAction LexStateAction::NewKeep()
{
    return LexStateAction(LexStateAction::Kind::KEEP, 0);
}
LexStateAction LexStateAction::NewSet(int32_t next_state)
{
    return LexStateAction(LexStateAction::Kind::SET, next_state);
}

TokenRewriterResult::TokenRewriterResult(std::unique_ptr<Token> rewritten_token,
                                         RewriteAction token_action,
                                         LexStateAction lex_state_action) : rewritten_token(std::move(rewritten_token)),
                                                                            token_action(token_action),
                                                                            lex_state_action(std::move(lex_state_action)) {}

TokenRewriter::TokenRewriter(rewrite_token_t rewrite_f,
                             build_new_token_t build_new_token_f) : rewrite_f(rewrite_f),
                                                                    build_new_token_f(build_new_token_f) {}

TokenRewriterResult __keep_token(std::unique_ptr<Token> token_to_rewrite, build_new_token_t build_new_token)
{
    (void)build_new_token;
    return TokenRewriterResult(
        std::move(token_to_rewrite),
        RewriteAction::KEEP,
        LexStateAction::NewKeep());
}
TokenRewriter TokenRewriter::NewKeepRewriter(build_new_token_t build_new_token_f)
{
    return TokenRewriter(__keep_token, build_new_token_f);
}
TokenRewriterResult __drop_token(std::unique_ptr<Token> token_to_rewrite, build_new_token_t build_new_token)
{
    (void)build_new_token;
    return TokenRewriterResult(
        std::move(token_to_rewrite),
        RewriteAction::DROP,
        LexStateAction::NewKeep());
}
TokenRewriter TokenRewriter::NewDropRewriter(build_new_token_t build_new_token_f)
{
    return TokenRewriter(__drop_token, build_new_token_f);
}
TokenRewriterResult __rewrite_token(std::unique_ptr<Token> token_to_rewrite, build_new_token_t build_new_token)
{
    (void)token_to_rewrite;
    return TokenRewriterResult(
        std::unique_ptr<Token>(build_new_token()),
        RewriteAction::KEEP,
        LexStateAction::NewKeep());
}
TokenRewriter TokenRewriter::NewRewriteRewriter(build_new_token_t build_new_token_f)
{
    return TokenRewriter(__rewrite_token, build_new_token_f);
}

// ParserOptions
MaybeDecoder::MaybeDecoder() {}
MaybeDecoder::MaybeDecoder(Decoder decoder) : decoder(std::move(decoder)) {}

MaybeTokenRewriter::MaybeTokenRewriter() {}
MaybeTokenRewriter::MaybeTokenRewriter(TokenRewriter token_rewriter) : token_rewriter(std::move(token_rewriter)) {}

ParserOptions::ParserOptions(StringPtr buffer_name,
                             uint8_t debug,
                             MaybeDecoder decoder,
                             MaybeTokenRewriter token_rewriter,
                             bool record_tokens) : buffer_name(std::move(buffer_name)),
                                                   debug(debug),
                                                   decoder(std::move(decoder)),
                                                   token_rewriter(std::move(token_rewriter)),
                                                   record_tokens(record_tokens) {}

// DecodedInput
DecodedInput::DecodedInput(StringPtr name,
                           SourceLineList lines,
                           ByteList bytes) : name(std::move(name)),
                                             lines(std::move(lines)),
                                             bytes(std::move(bytes)) {}

// ParserResult
ParserResult::ParserResult(std::unique_ptr<Node> ast,
                           TokenList tokens,
                           DiagnosticList diagnostics,
                           CommentList comments,
                           MagicCommentList magic_comments,
                           DecodedInput input) : ast(std::move(ast)),
                                                 tokens(std::move(tokens)),
                                                 diagnostics(std::move(diagnostics)),
                                                 comments(std::move(comments)),
                                                 magic_comments(std::move(magic_comments)),
                                                 input(std::move(input)) {}
