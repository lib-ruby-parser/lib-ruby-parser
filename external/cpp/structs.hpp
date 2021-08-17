#ifndef LIB_RUBY_PARSER_CPP_BINDINGS_STRUCTS_HPP
#define LIB_RUBY_PARSER_CPP_BINDINGS_STRUCTS_HPP

#include <string>
#include <cstdint>
#include <memory>
#include <string_view>
#include <optional>
#include <vector>
#include "declare_list.hpp"

// Byte
typedef uint8_t Byte;
DECLARE_LIST_OF(uint8_t, ByteList);

// Ptr<T>
typedef std::unique_ptr<int> Ptr;

// MaybePtr<T>
typedef std::unique_ptr<int> MaybePtr;

// StringPtr
// Small strings have optimization that forces string content
// to be stored INSIDE the string container.
// Because of that moved small string has a different c_str()
// which prevents us from sharing it with Rust
typedef std::unique_ptr<std::string> StringPtr;

// MaybeStringPtr
// Small strings have optimization that forces string content
// to be stored INSIDE the string container.
// Because of that moved small string has a different c_str()
// which prevents us from sharing it with Rust
typedef std::unique_ptr<std::string> MaybeStringPtr;

// SharedByteList
typedef std::string_view SharedByteList;

// SourceLine
class SourceLine
{
public:
    uint64_t start;
    uint64_t end;
    bool ends_with_eof;

    explicit SourceLine(uint64_t start,
                        uint64_t end,
                        bool ends_with_eof);

    SourceLine(const SourceLine &) = delete;
    SourceLine &operator=(SourceLine const &) = delete;

    SourceLine(SourceLine &&) = default;
    SourceLine &operator=(SourceLine &&other) = default;
};
struct SourceLine_BLOB;
DECLARE_LIST_OF(SourceLine, SourceLineList);

// Loc
class Loc
{
public:
    uint64_t begin;
    uint64_t end;

    explicit Loc(uint64_t begin, uint64_t end);

    Loc(const Loc &) = delete;
    Loc &operator=(Loc const &) = delete;

    Loc(Loc &&) = default;
    Loc &operator=(Loc &&other) = default;
};

// MaybeLoc
typedef std::optional<Loc> MaybeLoc;

// Bytes
class Bytes
{
public:
    ByteList raw;

    explicit Bytes(ByteList raw);

    Bytes(const Bytes &) = delete;
    Bytes &operator=(const Bytes &other) = delete;

    Bytes(Bytes &&) = default;
    Bytes &operator=(Bytes &&other) = default;
};

// Token
class Token
{
public:
    uint32_t token_type;
    Bytes token_value;
    Loc loc;
    uint32_t lex_state_before;
    uint32_t lex_state_after;

    explicit Token(
        uint32_t token_type,
        Bytes token_value,
        Loc loc,
        uint32_t lex_state_before,
        uint32_t lex_state_after);

    Token(const Token &) = delete;
    Token &operator=(const Token &other) = delete;

    Token(Token &&) = default;
    Token &operator=(Token &&other) = default;
};
struct Token_BLOB;
DECLARE_LIST_OF(Token, TokenList);

// CommentType
enum class CommentType
{
    DOCUMENT,
    INLINE,
    UNKNOWN,
};

// Comment
class Comment
{
public:
    Loc location;
    CommentType kind;

    Comment(Loc location, CommentType kind);

    Comment(const Comment &) = delete;
    Comment &operator=(const Comment &other) = delete;

    Comment(Comment &&) = default;
    Comment &operator=(Comment &&other) = default;
};
struct Comment_BLOB;
DECLARE_LIST_OF(Comment, CommentList);

// MagicCommentKind
enum class MagicCommentKind
{
    ENCODING,
    FROZEN_STRING_LITERAL,
    WARN_INDENT,
    SHAREABLE_CONSTANT_VALUE,
};

// MagicComment
class MagicComment
{
public:
    MagicCommentKind kind;
    Loc key_l;
    Loc value_l;

    MagicComment(MagicCommentKind kind, Loc key_l, Loc value_l);

    MagicComment(const MagicComment &) = delete;
    MagicComment &operator=(const MagicComment &other) = delete;

    MagicComment(MagicComment &&) = default;
    MagicComment &operator=(MagicComment &&other) = default;
};
struct MagicComment_BLOB;
DECLARE_LIST_OF(MagicComment, MagicCommentList);

// ErrorLevel
enum class ErrorLevel
{
    WARNING,
    ERROR
};

// DiagnosticMessage
#include "messages.hpp"

#endif // LIB_RUBY_PARSER_CPP_BINDINGS_STRUCTS_HPP
