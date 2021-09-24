#ifndef LIB_RUBY_PARSER_CPP_BINDINGS_STRUCTS_HPP
#define LIB_RUBY_PARSER_CPP_BINDINGS_STRUCTS_HPP

#include <string>
#include <cstdint>
#include <memory>
#include <string_view>
#include <optional>
#include <vector>
#include "list.hpp"

namespace lib_ruby_parser
{
    // Byte
    using Byte = uint8_t;
    using ByteList = List<Byte>;

    // Ptr<T>
    using Ptr = std::unique_ptr<int>;

    // MaybePtr<T>
    using MaybePtr = std::unique_ptr<int>;

    // StringPtr
    // Small strings have optimization that forces string content
    // to be stored INSIDE the string container.
    // Because of that moved small string has a different c_str()
    // which prevents us from sharing it with Rust
    using StringPtr = std::unique_ptr<std::string>;

    // MaybeStringPtr
    // Small strings have optimization that forces string content
    // to be stored INSIDE the string container.
    // Because of that moved small string has a different c_str()
    // which prevents us from sharing it with Rust
    using MaybeStringPtr = std::unique_ptr<std::string>;

    // SharedByteList
    using SharedByteList = std::string_view;

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
    using SourceLineList = List<SourceLine>;

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
    using MaybeLoc = std::optional<Loc>;

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
    using TokenList = List<Token>;

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
    using CommentList = List<Comment>;

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
    using MagicCommentList = List<MagicComment>;

    // ErrorLevel
    enum class ErrorLevel
    {
        WARNING,
        ERROR
    };
}

// DiagnosticMessage
#include "messages.hpp"

// Node
#include "nodes.hpp"

namespace lib_ruby_parser
{
    // InputError
    class InputError
    {
    public:
        class UnsupportedEncoding
        {
        public:
            StringPtr message;

            UnsupportedEncoding(StringPtr message);

            UnsupportedEncoding(const UnsupportedEncoding &) = delete;
            UnsupportedEncoding &operator=(const UnsupportedEncoding &other) = delete;

            UnsupportedEncoding(UnsupportedEncoding &&) = default;
            UnsupportedEncoding &operator=(UnsupportedEncoding &&other) = default;
        };

        class DecodingError
        {
        public:
            StringPtr message;

            DecodingError(StringPtr message);

            DecodingError(const DecodingError &) = delete;
            DecodingError &operator=(const DecodingError &other) = delete;

            DecodingError(DecodingError &&) = default;
            DecodingError &operator=(DecodingError &&other) = default;
        };

        using variant_t = std::variant<UnsupportedEncoding, DecodingError>;
        variant_t variant;

        InputError(variant_t variant);

        InputError(const InputError &) = delete;
        InputError &operator=(const InputError &other) = delete;

        InputError(InputError &&) = default;
        InputError &operator=(InputError &&other) = default;
    };

    // DecoderResult
    class DecoderResult
    {
    public:
        class Ok
        {
        public:
            ByteList output;

            Ok(ByteList output);

            Ok(const Ok &) = delete;
            Ok &operator=(const Ok &other) = delete;

            Ok(Ok &&) = default;
            Ok &operator=(Ok &&other) = default;
        };

        class Err
        {
        public:
            InputError error;

            Err(InputError error);

            Err(const Err &) = delete;
            Err &operator=(const Err &other) = delete;

            Err(Err &&) = default;
            Err &operator=(Err &&other) = default;
        };

        using variant_t = std::variant<Ok, Err>;
        variant_t variant;

        DecoderResult(variant_t variant);

        DecoderResult(const DecoderResult &) = delete;
        DecoderResult &operator=(const DecoderResult &other) = delete;

        DecoderResult(DecoderResult &&) = default;
        DecoderResult &operator=(DecoderResult &&other) = default;
    };

    // Decoder
    extern "C"
    {
        typedef DecoderResult (*dummy_decoder_t)(void);
    }
    class Decoder
    {
    public:
        // Here for tests we use a dummy fn that blindly returns what's configured when called
        dummy_decoder_t f;
    };

    // TokenRewriter
    enum class RewriteAction
    {
        DROP,
        KEEP
    };
    class LexStateAction
    {
    public:
        enum class Kind
        {
            SET,
            KEEP
        };
        Kind kind;
        int32_t next_state;

        LexStateAction(Kind kind, int32_t next_state);

        LexStateAction(const LexStateAction &) = delete;
        LexStateAction &operator=(const LexStateAction &other) = delete;

        LexStateAction(LexStateAction &&) = default;
        LexStateAction &operator=(LexStateAction &&other) = default;

        static LexStateAction NewKeep();
        static LexStateAction NewSet(int32_t next_state);
    };
    class TokenRewriterResult
    {
    public:
        std::unique_ptr<Token> rewritten_token;
        RewriteAction token_action;
        LexStateAction lex_state_action;

        TokenRewriterResult(std::unique_ptr<Token> rewritten_token,
                            RewriteAction token_action,
                            LexStateAction lex_state_action);

        TokenRewriterResult(const TokenRewriterResult &) = delete;
        TokenRewriterResult &operator=(const TokenRewriterResult &other) = delete;

        TokenRewriterResult(TokenRewriterResult &&) = default;
        TokenRewriterResult &operator=(TokenRewriterResult &&other) = default;
    };
    extern "C"
    {
        typedef Token *(*build_new_token_t)(void);
    }
    typedef TokenRewriterResult (*rewrite_token_t)(std::unique_ptr<Token>, build_new_token_t);
    class TokenRewriter
    {
    public:
        // Here for tests we use a dummy fn that (when called) blindly returns what's configured
        rewrite_token_t rewrite_f;
        build_new_token_t build_new_token_f;

        TokenRewriter(rewrite_token_t rewrite_f, build_new_token_t build_new_token_f);

        TokenRewriter(const TokenRewriter &) = delete;
        TokenRewriter &operator=(const TokenRewriter &other) = delete;

        TokenRewriter(TokenRewriter &&) = default;
        TokenRewriter &operator=(TokenRewriter &&other) = default;

        static TokenRewriter NewKeepRewriter(build_new_token_t build_new_token_f);
        static TokenRewriter NewDropRewriter(build_new_token_t build_new_token_f);
        static TokenRewriter NewRewriteRewriter(build_new_token_t build_new_token_f);
    };

    // ParserOptions
    class MaybeDecoder
    {
    public:
        std::optional<Decoder> decoder;

        MaybeDecoder();
        MaybeDecoder(Decoder decoder);

        MaybeDecoder(const MaybeDecoder &) = delete;
        MaybeDecoder &operator=(const MaybeDecoder &other) = delete;

        MaybeDecoder(MaybeDecoder &&) = default;
        MaybeDecoder &operator=(MaybeDecoder &&other) = default;
    };

    class MaybeTokenRewriter
    {
    public:
        std::optional<TokenRewriter> token_rewriter;

        MaybeTokenRewriter();
        MaybeTokenRewriter(TokenRewriter decoder);

        MaybeTokenRewriter(const MaybeTokenRewriter &) = delete;
        MaybeTokenRewriter &operator=(const MaybeTokenRewriter &other) = delete;

        MaybeTokenRewriter(MaybeTokenRewriter &&) = default;
        MaybeTokenRewriter &operator=(MaybeTokenRewriter &&other) = default;
    };

    class ParserOptions
    {
    public:
        StringPtr buffer_name;
        MaybeDecoder decoder;
        MaybeTokenRewriter token_rewriter;
        bool record_tokens;

        ParserOptions(
            StringPtr buffer_name,
            MaybeDecoder decoder,
            MaybeTokenRewriter token_rewriter,
            bool record_tokens);

        ParserOptions(const ParserOptions &) = delete;
        ParserOptions &operator=(const ParserOptions &other) = delete;

        ParserOptions(ParserOptions &&) = default;
        ParserOptions &operator=(ParserOptions &&other) = default;
    };

    // DecodedInput
    class DecodedInput
    {
    public:
        StringPtr name;
        SourceLineList lines;
        ByteList bytes;

        DecodedInput(
            StringPtr name,
            SourceLineList lines,
            ByteList bytes);

        DecodedInput(const DecodedInput &) = delete;
        DecodedInput &operator=(const DecodedInput &other) = delete;

        DecodedInput(DecodedInput &&) = default;
        DecodedInput &operator=(DecodedInput &&other) = default;
    };

    // ParserResult
    class ParserResult
    {
    public:
        std::unique_ptr<Node> ast;
        TokenList tokens;
        DiagnosticList diagnostics;
        CommentList comments;
        MagicCommentList magic_comments;
        DecodedInput input;

        ParserResult(
            std::unique_ptr<Node> ast,
            TokenList tokens,
            DiagnosticList diagnostics,
            CommentList comments,
            MagicCommentList magic_comments,
            DecodedInput input);

        ParserResult(const ParserResult &) = delete;
        ParserResult &operator=(const ParserResult &other) = delete;

        ParserResult(ParserResult &&) = default;
        ParserResult &operator=(ParserResult &&other) = default;
    };
}

#endif // LIB_RUBY_PARSER_CPP_BINDINGS_STRUCTS_HPP
