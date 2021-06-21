#include "source_line.hpp"
#include "impl_blob.hpp"

IMPL_BLOB(SourceLine);
IMPL_BLOB(LIST_OF_SourceLine);

SourceLine::SourceLine(uint64_t start,
                       uint64_t end,
                       bool ends_with_eof) : start(start),
                                             end(end),
                                             ends_with_eof(ends_with_eof)
{
}

extern "C"
{
    SourceLine_BLOB lib_ruby_parser__internal__containers__source_line__new(uint64_t start, uint64_t end, bool ends_with_eof)
    {
        return PACK(SourceLine(start, end, ends_with_eof));
    }

    uint64_t lib_ruby_parser__internal__containers__source_line__get_start(SourceLine_BLOB blob)
    {
        return UNPACK(blob).start;
    }
    uint64_t lib_ruby_parser__internal__containers__source_line__get_end(SourceLine_BLOB blob)
    {
        return UNPACK(blob).end;
    }
    bool lib_ruby_parser__internal__containers__source_line__get_ends_with_eof(SourceLine_BLOB blob)
    {
        return UNPACK(blob).ends_with_eof;
    }

    SourceLine_BLOB lib_ruby_parser__internal__containers__source_line__set_start(SourceLine_BLOB blob, uint64_t start)
    {
        SourceLine source_line = UNPACK(blob);
        source_line.start = start;
        return PACK(source_line);
    }
    SourceLine_BLOB lib_ruby_parser__internal__containers__source_line__set_end(SourceLine_BLOB blob, uint64_t end)
    {
        SourceLine source_line = UNPACK(blob);
        source_line.end = end;
        return PACK(source_line);
    }
    SourceLine_BLOB lib_ruby_parser__internal__containers__source_line__set_ends_with_eof(SourceLine_BLOB blob, bool ends_with_eof)
    {
        SourceLine source_line = UNPACK(blob);
        source_line.ends_with_eof = ends_with_eof;
        return PACK(source_line);
    }
}
