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
LexStateAction LexStateAction::NewKeep()
{
    LexStateAction result;
    result.kind = LexStateAction::Kind::KEEP;
    result.next_state = 0;
    return result;
}
LexStateAction LexStateAction::NewSet(int32_t next_state)
{
    LexStateAction result;
    result.kind = LexStateAction::Kind::SET;
    result.next_state = next_state;
    return result;
}

TokenRewriterResult::TokenRewriterResult(std::unique_ptr<Token> rewritten_token,
                                         RewriteAction token_action,
                                         LexStateAction lex_state_action) : rewritten_token(std::move(rewritten_token)),
                                                                            token_action(token_action),
                                                                            lex_state_action(lex_state_action) {}

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
