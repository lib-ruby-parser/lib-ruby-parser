#include <stdlib.h>
#include "structs.h"
#include "impl_blob.h"
#include "messages.h"

// Byte
void drop_byte(Byte *byte)
{
    (void)byte;
    // noop
}
void drop_byte_list(ByteList *byte_list)
{
    free(byte_list->ptr);
}

// Ptr

// MaybePtr

// StringPtr
void drop_string_ptr(StringPtr *string_ptr)
{
    free(string_ptr->ptr);
}

// MaybeStringPtr
void drop_maybe_string_ptr(MaybeStringPtr *maybe_string_ptr)
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

// Loc
void drop_loc(Loc *loc)
{
    (void)loc;
}

// MaybeLoc
void drop_maybe_loc(MaybeLoc *maybe_loc)
{
    (void)maybe_loc;
}

// Bytes
void drop_bytes(Bytes *bytes)
{
    drop_byte_list(&(bytes->raw));
}

// Token
void drop_token(Token *token)
{
    drop_bytes(&(token->token_value));
}

// CommentType

// Comment

// MagicCommentKind

// MagicComment

// ErrorLevel

// Diagnostic
void drop_diagnostic(Diagnostic *diagnostic)
{
    drop_diagnostic_message(&(diagnostic->message));
}
