$VERBOSE = nil

require 'ripper';
keyword = {
    '__ENCODING__' => :k__ENCODING__,
    '__LINE__' => :k__LINE__,
    '__FILE__' => :k__FILE__,
    'BEGIN' => :klBEGIN,
    'END' => :klEND,
    'alias' => :kALIAS,
    'and' => :kAND,
    'begin' => :kBEGIN,
    'break' => :kBREAK,
    'case' => :kCASE,
    'class' => :kCLASS,
    'def' => :kDEF,
    'defined?' => :kDEFINED,
    'do' => :kDO,
    'else' => :kELSE,
    'elsif' => :kELSIF,
    'end' => :kEND,
    'ensure' => :kENSURE,
    'false' => :kFALSE,
    'for' => :kFOR,
    'if' => :kIF,
    'in' => :kIN,
    'module' => :kMODULE,
    'next' => :kNEXT,
    'nil' => :kNIL,
    'not' => :kNOT,
    'or' => :kOR,
    'redo' => :kREDO,
    'rescue' => :kRESCUE,
    'retry' => :kRETRY,
    'return' => :kRETURN,
    'self' => :kSELF,
    'super' => :kSUPER,
    'then' => :kTHEN,
    'true' => :kTRUE,
    'undef' => :kUNDEF,
    'unless' => :kUNLESS,
    'until' => :kUNTIL,
    'when' => :kWHEN,
    'while' => :kWHILE,
    'yield' => :kYIELD,
}

mapping = {
    on_int: :tINTEGER,
    on_ident: :tIDENTIFIER,
    on_tstring_beg: :tSTRING_BEG,
    on_tstring_content: :tSTRING_CONTENT,
    on_tstring_end: :tSTRING_END,
    on_const: :tCONSTANT,
    on_regexp_beg: :tREGEXP_BEG,
    on_regexp_end: :tREGEXP_END,
    on_comma: :tCOMMA,
    on_heredoc_beg: :tSTRING_BEG,
    on_heredoc_end: :tSTRING_END,
    on_embexpr_beg: :tSTRING_DBEG,
    on_embexpr_end: :tSTRING_DEND,
    on_symbeg: :tSYMBEG,
    on_period: :tDOT,
    on_lbracket: :tLBRACK,
    on_rbracket: :tRBRACK,
    on_semicolon: :tSEMI,
    on_lbrace: :tLBRACE,
    on_rbrace: :tRBRACE,
    on_lparen: :tLPAREN,
    on_rparen: :tRPAREN,
    on_CHAR: :tCHAR,
    on_ivar: :tIVAR,
    on_label: :tLABEL,
    on_backref: :tBACK_REF,
    on_qwords_beg: :tQWORDS_BEG,
    on_gvar: :tGVAR,
    on_tlambda: :tLAMBDA,
    on_tlambeg: :tLAMBEG,
    on_float: :tFLOAT,
    on_backtick: :tBACKTICK,
    on_cvar: :tCVAR,
    on_imaginary: :tIMAGINARY,
    on_rational: :tRATIONAL,
    on_qsymbols_beg: :tQSYMBOLS_BEG,
    on_embdoc_beg: :tCOMMENT,
    on_embdoc_end: :tCOMMENT,
    on_embdoc: :tCOMMENT,
    on_words_beg: :tWORDS_BEG,
    on___end__: :t__END__,
    on_embvar: :tSTRING_DVAR,
    on_symbols_beg: :tSYMBOLS_BEG,
}

ops = {
    '='   => :tEQL,     '&'   => :tAMPER,  '|'   => :tPIPE,
    '!'   => :tBANG,    '^'   => :tCARET,   '+'   => :tPLUS,
    '-'   => :tMINUS,   '*'   => :tSTAR,    '/'   => :tDIVIDE,
    '%'   => :tPERCENT, '~'   => :tTILDE,   ','   => :tCOMMA,
    ';'   => :tSEMI,    '.'   => :tDOT,     '..'  => :tDOT2,
    '...' => :tDOT3,    '['   => :tLBRACK,  ']'   => :tRBRACK,
    '('   => :tLPAREN,  ')'   => :tRPAREN,  '?'   => :tEH,
    ':'   => :tCOLON,   '&&'  => :tANDOP,   '||'  => :tOROP,
    '-@'  => :tMINUS,   '+@'  => :tPLUS,    '~@'  => :tTILDE,
    '**'  => :tDSTAR,   '->'  => :tLAMBDA,  '=~'  => :tMATCH,
    '!~'  => :tNMATCH,  '=='  => :tEQ,      '!='  => :tNEQ,
    '>'   => :tGT,      '>>'  => :tRSHFT,   '>='  => :tGEQ,
    '<'   => :tLT,      '<<'  => :tLSHFT,   '<='  => :tLEQ,
    '=>'  => :tASSOC,   '::'  => :tCOLON2,  '===' => :tEQQ,
    '<=>' => :tCMP,     '[]'  => :tAREF,    '[]=' => :tASET,
    '{'   => :tLCURLY,  '}'   => :tRCURLY,  '`'   => :tBACKTICK,
    '!@'  => :tBANG,    '&.'  => :tANDDOT,

    '+='  => :tOP_ASGN, '-='  => :tOP_ASGN, '||=' => :tOP_ASGN,
    '*='  => :tOP_ASGN, '|='  => :tOP_ASGN, '&='  => :tOP_ASGN,
    '>>=' => :tOP_ASGN, '<<=' => :tOP_ASGN, '&&=' => :tOP_ASGN,
    '%='  => :tOP_ASGN, '/='  => :tOP_ASGN, '^='  => :tOP_ASGN,
    '**=' => :tOP_ASGN,
}

strs = []

def eval_as(start_t, tok_value, end_t, kind)
    begin
        eval(start_t + tok_value.encode('utf-8') + end_t)
    rescue SyntaxError, EncodingError
        $stderr.puts("Can't dump #{kind} part #{tok_value.inspect}")
        nil
    end
end

def rotate_bracket(lbrack)
    case lbrack
    when '<' then '>'
    when '(' then ')'
    when '[' then ']'
    when '{' then '}'
    else
        # %r|foo|
        lbrack
    end
end

TAKE_SELF = ->(v) { v.to_s }
RE_SOURCE = ->(re) { re.source }
ARRAY_FIRST = ->(v) {
    raise 'expected array' unless v.is_a?(Array)
    raise 'expected 1 element' if v.length != 1
    v[0].to_s
}

def `(s)
    s
end

def str_for_str_beg(str_beg)
    case str_beg
    when /\A<<-?'\w+'\z/, /\A<<~'\w+'\z/
        ["<<'HERE'\n", "HERE", 'non-interp heredoc', TAKE_SELF]
    when /\A<<-?\w+\z/, /\A<<-?"\w+"\z/, /\A<<~\w+\z/, /\A<<~"\w+"\z/
        ["<<HERE\n", 'HERE', 'interp heredoc', TAKE_SELF]
    when '"'
        ['"', '"', '" str', TAKE_SELF]
    when "'"
        ["'", "'", '\' str', TAKE_SELF]
    when /\A%r(.)/
        [str_beg, rotate_bracket($1), 'regex', RE_SOURCE]
    when '/'
        ['/', '/', 'regex', RE_SOURCE]
    when /\A%w(.)/
        [str_beg, rotate_bracket($1), 'non-interp words', ARRAY_FIRST]
    when /\A%W(.)/
        [str_beg, rotate_bracket($1), 'interp words', ARRAY_FIRST]
    when /\A%i(.)/
        [str_beg, rotate_bracket($1), 'non-interp symbols', ARRAY_FIRST]
    when /\A%I(.)/
        [str_beg, rotate_bracket($1), 'interp symbols', ARRAY_FIRST]
    when /\A%q(.)/
        [str_beg, rotate_bracket($1), '%q() string', TAKE_SELF]
    when /\A%Q(.)/
        [str_beg, rotate_bracket($1), '%Q() string', TAKE_SELF]
    when /\A%x(.)/
        [str_beg, rotate_bracket($1), '%x() string', TAKE_SELF]
    when ':"'
        [':"', '"', 'dsymbol', TAKE_SELF]
    when ":'"
        [":'", "'", 'ssymbol', TAKE_SELF]
    when '`'
        ['`', '`', 'xstring', TAKE_SELF]
    when /\A%s(.)/
        [str_beg, rotate_bracket($1), '%s() string', TAKE_SELF]
    when /\A%(.)/
        [str_beg, rotate_bracket($1), '%() string', TAKE_SELF]
    else
        raise "unsupported str_beg #{str_beg.inspect}"
    end
end

$stderr.puts ARGV.first
Ripper.lex(File.read(ARGV.first)).each do |(start, tok_name, tok_value, state)|
    tok_name =
        case tok_name
        when :on_nl then next
        when :on_kw then keyword.fetch(tok_value) { raise 'unsupported keyword ' + tok_value  }
        when :on_sp, :on_ignored_sp, :on_ignored_nl, :on_comment, :on_words_sep then next
        when :on_op then ops.fetch(tok_value) { raise 'unsupported op ' + tok_value }
        when :on_heredoc_end
            tok_value = tok_value.strip
            mapping.fetch(tok_name)
        else
            mapping.fetch(tok_name) { raise 'unsupported ' + tok_name.to_s + ' ' + tok_value }
        end

    case tok_name
    when :tIDENTIFIER
        if state.to_int == Ripper::EXPR_ENDFN
            # :sym case, we need to pop :tSYMBEG
            strs.pop
        end
    when :tBACKTICK
        if (state.to_int & Ripper::EXPR_BEG) != 0
            # `string`
            strs.push(tok_value)
        else
            # :` - state is EXPR_ENDFN here
        end
    when :tSTRING_BEG, :tREGEXP_BEG, :tXSTRING_BEG, :tQWORDS_BEG, :tQSYMBOLS_BEG, :tSYMBEG, :tSYMBOLS_BEG, :tWORDS_BEG
        strs.push(tok_value)
    when :tSTRING_END, :tREGEXP_END
        strs.pop
    when :tSTRING_CONTENT
        (str_beg, str_end, kind, fn) = str_for_str_beg(strs.last)
        if kind.include?('heredoc') && !tok_value.end_with?("\n")
            tok_value += "\n"
        end
        begin
            enc = tok_value.encoding
            tok_value = fn.call(
                eval(str_beg.encode(enc) + tok_value + str_end.encode(enc))
            )
        rescue SyntaxError, EncodingError => e
            $stderr.puts(<<~MSG)
                Can't dump #{kind}:
                tok_value = #{tok_value.inspect}
                wrapped = #{(str_beg.encode(enc) + tok_value + str_end.encode(enc)).inspect}
                got error #{e.class}: #{e.message.inspect}
            MSG
            puts "<<UNKNOWN>>"
            next
        end
    when :tLABEL
        tok_value = tok_value.delete_suffix(':')
    when :tCHAR
        begin
            tok_value = eval('"' + tok_value.delete_prefix('?') + '"')
        rescue SyntaxError, EncodingError
            $stderr.puts("Can't dump char #{tok_value.inspect}")
            puts "<<UNKNOWN>>"
            next
        end
    when :tCOMMENT
        next
    end

    puts tok_name.to_s + ' ' + (tok_value || "").bytes.inspect + ' ' + start.join(':')
end
