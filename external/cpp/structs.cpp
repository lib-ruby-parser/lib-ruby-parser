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
