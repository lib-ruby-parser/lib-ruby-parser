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
    on_backtick: :tXSTRING_BEG,
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
    '{'   => :tLCURLY,  '}'   => :tRCURLY,  '`'   => :tBACK_REF2,
    '!@'  => :tBANG,    '&.'  => :tANDDOT,

    '+='  => :tOP_ASGN, '-='  => :tOP_ASGN, '||=' => :tOP_ASGN,
    '*='  => :tOP_ASGN, '|='  => :tOP_ASGN, '&='  => :tOP_ASGN,
    '>>=' => :tOP_ASGN, '<<=' => :tOP_ASGN, '&&=' => :tOP_ASGN,
    '%='  => :tOP_ASGN, '/='  => :tOP_ASGN, '^='  => :tOP_ASGN,
    '**=' => :tOP_ASGN,
}

strs = []

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
    when :tSTRING_BEG, :tREGEXP_BEG, :tXSTRING_BEG, :tQWORDS_BEG, :tQSYMBOLS_BEG, :tSYMBEG, :tSYMBOLS_BEG
        strs.push(tok_value)
    when :tSTRING_END, :tREGEXP_END
        strs.pop
    when :tSTRING_CONTENT
        case strs.last
        when /\A<<-?'\w+'\z/ # no escaping
        when '"', '`', /\A%W/, /\A%I/, ':"',
            '%<', '%{', '%(', '%[', '%!', '%@', '%#', '%%', '%^', '%&', '%*', '%-', '%_', '%=', '%+', '%~', '%:', '%;', '%\\', '%"', '%|', '%?', '%/', '%,', '%.', '%\'', '%`', '%$'
            begin
                tok_value = eval('"' + tok_value.encode('utf-8') + '"')
            rescue SyntaxError, EncodingError
                $stderr.puts("Can't dump squote str part #{tok_value.inspect}")
                puts "<<UNKNOWN>>"
                next
            end
        when "'", /\A%w/, /\A%i/, ":'", '%q('
            begin
                tok_value = eval("'" + tok_value.encode('utf-8') + "'")
            rescue SyntaxError, EncodingError
                $stderr.puts("Can't dump dquote str part #{tok_value.inspect}")
                puts "<<UNKNOWN>>"
                next
            end
        when "/", /\A%r/
            begin
                tok_value = eval("/" + tok_value.encode('utf-8') + "/").source
            rescue SyntaxError, EncodingError
                $stderr.puts("Can't dump regex part #{tok_value.inspect}")
                puts "<<UNKNOWN>>"
                next
            end
        when /\A<</ # ignore
        when /%r\[/ # ignore
        else
            raise "unknown str type #{strs.last.inspect}"
        end
    when :tINTEGER
        tok_value = tok_value.gsub('_', '')
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
