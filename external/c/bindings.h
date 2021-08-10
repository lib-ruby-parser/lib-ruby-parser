#ifndef LIB_RUBY_PARSER_BINDINGS_H
#define LIB_RUBY_PARSER_BINDINGS_H

#include <stdint.h>
#include <stdbool.h>
#include "bindings_include.h"

/*
    Ptr
*/
Ptr_BLOB lib_ruby_parser__internal__containers__ptr__new(void *ptr);
Ptr_BLOB lib_ruby_parser__internal__containers__ptr__new_null();
void *lib_ruby_parser__internal__containers__ptr__get_raw(Ptr_BLOB *blob);
void lib_ruby_parser__internal__containers__ptr__of_node__free(Ptr_BLOB *blob);
void lib_ruby_parser__internal__containers__ptr__of_token__free(Ptr_BLOB *blob);

/*
    MaybePtr
*/
MaybePtr_BLOB lib_ruby_parser__internal__containers__maybe_ptr__new(void *ptr);
MaybePtr_BLOB lib_ruby_parser__internal__containers__maybe_ptr__new_null();
void *lib_ruby_parser__internal__containers__maybe_ptr__get_raw(MaybePtr_BLOB *blob);
void lib_ruby_parser__internal__containers__maybe_ptr__of_node__free(MaybePtr_BLOB *blob);
void lib_ruby_parser__internal__containers__maybe_ptr__of_token__free(MaybePtr_BLOB *blob);

/*
    StringPtr
*/
StringPtr_BLOB lib_ruby_parser__internal__containers__string_ptr__new(const uint8_t *ptr, uint64_t len);
void lib_ruby_parser__internal__containers__string_ptr__drop(StringPtr_BLOB *blob);
uint8_t *lib_ruby_parser__internal__containers__string_ptr__get_raw(StringPtr_BLOB *blob);
uint64_t lib_ruby_parser__internal__containers__string_ptr__get_len(const StringPtr_BLOB *blob);

/*
    MaybeStringPtr
*/
MaybeStringPtr_BLOB lib_ruby_parser__internal__containers__maybe_string_ptr__new_some(const uint8_t *ptr, uint64_t len);
MaybeStringPtr_BLOB lib_ruby_parser__internal__containers__maybe_string_ptr__new_none();
bool lib_ruby_parser__internal__containers__maybe_string_ptr__is_some(const MaybeStringPtr_BLOB *blob);
bool lib_ruby_parser__internal__containers__maybe_string_ptr__is_none(const MaybeStringPtr_BLOB *blob);
void lib_ruby_parser__internal__containers__maybe_string_ptr__drop(MaybeStringPtr_BLOB *blob);
uint8_t *lib_ruby_parser__internal__containers__maybe_string_ptr__get_raw(MaybeStringPtr_BLOB *blob);
uint8_t *lib_ruby_parser__internal__containers__maybe_string_ptr__into_raw(MaybeStringPtr_BLOB *blob);
uint64_t lib_ruby_parser__internal__containers__maybe_string_ptr__len(const MaybeStringPtr_BLOB *blob);

/*
    Lists
*/
#define LIST_DECL(ITEM, ITEM_BLOB, LIST, LIST_BLOB, NS)                                                     \
    LIST_BLOB lib_ruby_parser__internal__containers__list__##NS##__new();                                   \
    void lib_ruby_parser__internal__containers__list__##NS##__drop(LIST_BLOB *blob);                        \
    LIST_BLOB lib_ruby_parser__internal__containers__list__##NS##__with_capacity(uint64_t capacity);        \
    LIST_BLOB lib_ruby_parser__internal__containers__list__##NS##__from_raw(ITEM_BLOB *ptr, uint64_t len);  \
    void lib_ruby_parser__internal__containers__list__##NS##__push(LIST_BLOB *blob, ITEM_BLOB item_blob);   \
    ITEM_BLOB lib_ruby_parser__internal__containers__list__##NS##__remove(LIST_BLOB *blob, uint64_t index); \
    void lib_ruby_parser__internal__containers__list__##NS##__shrink_to_fit(LIST_BLOB *blob);               \
    ITEM_BLOB *lib_ruby_parser__internal__containers__list__##NS##__as_ptr(LIST_BLOB *blob);                \
    uint64_t lib_ruby_parser__internal__containers__list__##NS##__len(const LIST_BLOB *blob);               \
    uint64_t lib_ruby_parser__internal__containers__list__##NS##__capacity(const LIST_BLOB *blob);

LIST_DECL(Byte, Byte_BLOB, ByteList, ByteList_BLOB, of_bytes)
LIST_DECL(Token, Token_BLOB, TokenList, TokenList_BLOB, of_tokens)
LIST_DECL(Node, Node_BLOB, NodeList, NodeList_BLOB, of_nodes)
LIST_DECL(Diagnostic, Diagnostic_BLOB, DiagnosticList, DiagnosticList_BLOB, of_diagnostics)
LIST_DECL(Comment, Comment_BLOB, CommentList, CommentList_BLOB, of_comments)
LIST_DECL(MagicComment, MagicComment_BLOB, MagicCommentList, MagicCommentList_BLOB, of_magic_comments)
LIST_DECL(SourceLine, SourceLine_BLOB, SourceLineList, SourceLineList_BLOB, of_source_lines)

/*
    SourceLine
*/
SourceLine_BLOB lib_ruby_parser__internal__containers__source_line__new(uint64_t start, uint64_t end, bool ends_with_eof);
uint64_t lib_ruby_parser__internal__containers__source_line__get_start(const SourceLine_BLOB *blob);
uint64_t lib_ruby_parser__internal__containers__source_line__get_end(const SourceLine_BLOB *blob);
bool lib_ruby_parser__internal__containers__source_line__get_ends_with_eof(const SourceLine_BLOB *blob);
void lib_ruby_parser__internal__containers__source_line__set_start(SourceLine_BLOB *blob, uint64_t start);
void lib_ruby_parser__internal__containers__source_line__set_end(SourceLine_BLOB *blob, uint64_t end);
void lib_ruby_parser__internal__containers__source_line__set_ends_with_eof(SourceLine_BLOB *blob, bool ends_with_eof);
void lib_ruby_parser__internal__containers__source_line__drop(SourceLine_BLOB *blob);

/*
    Bytes
*/
Bytes_BLOB lib_ruby_parser__internal__containers__bytes__new_from_byte_list(ByteList_BLOB list_blob);
void lib_ruby_parser__internal__containers__bytes__drop(Bytes_BLOB *blob);
const ByteList_BLOB *lib_ruby_parser__internal__containers__bytes__get_byte_list(const Bytes_BLOB *blob);
void lib_ruby_parser__internal__containers__bytes__set_byte_list(Bytes_BLOB *blob, ByteList_BLOB list_blob);
ByteList_BLOB lib_ruby_parser__internal__containers__bytes__into_byte_list(Bytes_BLOB blob);
void lib_ruby_parser__internal__containers__bytes__push(Bytes_BLOB *blob, Byte byte);

/*
    Token
*/
Token_BLOB lib_ruby_parser__internal__containers__token__new(uint32_t token_type,
                                                             Bytes_BLOB token_value,
                                                             Loc_BLOB loc,
                                                             uint32_t lex_state_before,
                                                             uint32_t lex_state_after);
uint32_t lib_ruby_parser__internal__containers__token__get_token_type(const Token_BLOB *blob);
const Bytes_BLOB *lib_ruby_parser__internal__containers__token__get_token_value(const Token_BLOB *blob);
void lib_ruby_parser__internal__containers__token__set_token_value(Token_BLOB *blob, Bytes_BLOB token_value_blob);
Bytes_BLOB lib_ruby_parser__internal__containers__token__into_token_value(Token_BLOB blob);
const Loc_BLOB *lib_ruby_parser__internal__containers__token__get_loc(const Token_BLOB *blob);
uint32_t lib_ruby_parser__internal__containers__token__get_lex_state_before(const Token_BLOB *blob);
uint32_t lib_ruby_parser__internal__containers__token__get_lex_state_after(const Token_BLOB *blob);
void lib_ruby_parser__internal__containers__token__drop(Token_BLOB *blob);

/*
    CommentType
*/
CommentType_BLOB lib_ruby_parser__internal__containers__comment_type__new_inline();
CommentType_BLOB lib_ruby_parser__internal__containers__comment_type__new_document();
CommentType_BLOB lib_ruby_parser__internal__containers__comment_type__new_unknown();
bool lib_ruby_parser__internal__containers__comment_type__is_inline(CommentType_BLOB blob);
bool lib_ruby_parser__internal__containers__comment_type__is_document(CommentType_BLOB blob);
bool lib_ruby_parser__internal__containers__comment_type__is_unknown(CommentType_BLOB blob);

/*
    Comment
*/
Comment_BLOB lib_ruby_parser__internal__containers__comment__new(Loc_BLOB location, CommentType_BLOB kind);
Loc_BLOB *lib_ruby_parser__internal__containers__comment__get_location(Comment_BLOB *blob);
CommentType_BLOB *lib_ruby_parser__internal__containers__comment__get_kind(Comment_BLOB *blob);
void lib_ruby_parser__internal__containers__comment__drop(Comment_BLOB *blob);

/*
    ErrorLevel
*/
ErrorLevel_BLOB lib_ruby_parser__internal__containers__error_level__new_warning();
ErrorLevel_BLOB lib_ruby_parser__internal__containers__error_level__new_error();
bool lib_ruby_parser__internal__containers__error_level__is_warning(ErrorLevel_BLOB blob);
bool lib_ruby_parser__internal__containers__error_level__is_error(ErrorLevel_BLOB blob);

/*
    Loc
*/
Loc_BLOB lib_ruby_parser__internal__containers__loc__new(uint64_t begin, uint64_t end);
uint64_t lib_ruby_parser__internal__containers__loc__begin(const Loc_BLOB *blob);
uint64_t lib_ruby_parser__internal__containers__loc__end(const Loc_BLOB *blob);
void lib_ruby_parser__internal__containers__loc__drop(Loc_BLOB *blob);

/*
    MaybeLoc
*/
MaybeLoc_BLOB lib_ruby_parser__internal__containers__maybe_loc__new_some(Loc_BLOB loc_blob);
MaybeLoc_BLOB lib_ruby_parser__internal__containers__maybe_loc__new_none();
bool lib_ruby_parser__internal__containers__maybe_loc__is_some(const MaybeLoc_BLOB *blob);
bool lib_ruby_parser__internal__containers__maybe_loc__is_none(const MaybeLoc_BLOB *blob);
Loc_BLOB *lib_ruby_parser__internal__containers__maybe_loc__get_loc(MaybeLoc_BLOB *blob);
Loc_BLOB lib_ruby_parser__internal__containers__maybe_loc__into_loc(MaybeLoc_BLOB blob);
void lib_ruby_parser__internal__containers__maybe_loc__drop(MaybeLoc_BLOB *blob);

/*
    MagicCommentKind
*/
MagicCommentKind_BLOB lib_ruby_parser__internal__containers__magic_comment_kind__new_encoding();
MagicCommentKind_BLOB lib_ruby_parser__internal__containers__magic_comment_kind__new_frozen_string_literal();
MagicCommentKind_BLOB lib_ruby_parser__internal__containers__magic_comment_kind__new_warn_indent();
MagicCommentKind_BLOB lib_ruby_parser__internal__containers__magic_comment_kind__new_shareable_constant_value();

bool lib_ruby_parser__internal__containers__magic_comment_kind__is_encoding(MagicCommentKind_BLOB blob);
bool lib_ruby_parser__internal__containers__magic_comment_kind__is_frozen_string_literal(MagicCommentKind_BLOB blob);
bool lib_ruby_parser__internal__containers__magic_comment_kind__is_warn_indent(MagicCommentKind_BLOB blob);
bool lib_ruby_parser__internal__containers__magic_comment_kind__is_shareable_constant_value(MagicCommentKind_BLOB blob);

/*
    MagicComment
*/

MagicComment_BLOB lib_ruby_parser__internal__containers__magic_comment__new(MagicCommentKind_BLOB kind, Loc_BLOB key_l, Loc_BLOB value_l);
const MagicCommentKind_BLOB *lib_ruby_parser__internal__containers__magic_comment__get_kind(const MagicComment_BLOB *blob);
const Loc_BLOB *lib_ruby_parser__internal__containers__magic_comment__get_key_l(const MagicComment_BLOB *blob);
const Loc_BLOB *lib_ruby_parser__internal__containers__magic_comment__get_value_l(const MagicComment_BLOB *blob);
void lib_ruby_parser__internal__containers__magic_comment__drop(MagicComment_BLOB *blob);

/*
    SharedByteList
*/
SharedByteList_BLOB lib_ruby_parser__internal__containers__shared_byte_list__new(const uint8_t *ptr, uint64_t len);
const uint8_t *lib_ruby_parser__internal__containers__shared_byte_list__get_raw(const SharedByteList_BLOB *blob);
uint64_t lib_ruby_parser__internal__containers__shared_byte_list__get_len(const SharedByteList_BLOB *blob);

#include "bindings_nodes.h"
#include "bindings_messages.h"

/* Diagnostic */
Diagnostic_BLOB lib_ruby_parser__internal__containers__diagnostic__new(ErrorLevel_BLOB level, DiagnosticMessage_BLOB message, Loc_BLOB loc);
const ErrorLevel_BLOB *lib_ruby_parser__internal__containers__diagnostic__get_level(const Diagnostic_BLOB *blob);
const DiagnosticMessage_BLOB *lib_ruby_parser__internal__containers__diagnostic__get_message(const Diagnostic_BLOB *blob);
const Loc_BLOB *lib_ruby_parser__internal__containers__diagnostic__get_loc(const Diagnostic_BLOB *blob);
void lib_ruby_parser__internal__containers__diagnostic__drop(Diagnostic_BLOB *blob);

#endif // LIB_RUBY_PARSER_BINDINGS_H
