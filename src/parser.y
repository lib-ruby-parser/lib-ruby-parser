%expect 0

%define api.parser.struct { Parser }
%define api.location.type { Loc }
%define api.value.type { Value }
%define api.parser.result_type { String }

%define parse.error custom
%define parse.trace


%code use {
    use crate::Lexer;
}

%code {
    // pre-code
}

/* Bison Declarations */
%token <token>
    kCLASS         "`class'"
    kMODULE        "`module'"
    kDEF           "`def'"
    kUNDEF         "`undef'"
    kBEGIN         "`begin'"
    kRESCUE        "`rescue'"
    kENSURE        "`ensure'"
    kEND           "`end'"
    kIF            "`if'"
    kUNLESS        "`unless'"
    kTHEN          "`then'"
    kELSIF         "`elsif'"
    kELSE          "`else'"
    kCASE          "`case'"
    kWHEN          "`when'"
    kWHILE         "`while'"
    kUNTIL         "`until'"
    kFOR           "`for'"
    kBREAK         "`break'"
    kNEXT          "`next'"
    kREDO          "`redo'"
    kRETRY         "`retry'"
    kIN            "`in'"
    kDO            "`do'"
    kDO_COND       "`do' for condition"
    kDO_BLOCK      "`do' for block"
    kDO_LAMBDA     "`do' for lambda"
    kRETURN        "`return'"
    kYIELD         "`yield'"
    kSUPER         "`super'"
    kSELF          "`self'"
    kNIL           "`nil'"
    kTRUE          "`true'"
    kFALSE         "`false'"
    kAND           "`and'"
    kOR            "`or'"
    kNOT           "`not'"
    kIF_MOD        "`if' modifier"
    kUNLESS_MOD    "`unless' modifier"
    kWHILE_MOD     "`while' modifier"
    kUNTIL_MOD     "`until' modifier"
    kRESCUE_MOD    "`rescue' modifier"
    kALIAS         "`alias'"
    kDEFINED       "`defined?'"
    klBEGIN        "`BEGIN'"
    klEND          "`END'"
    k__LINE__      "`__LINE__'"
    k__FILE__      "`__FILE__'"
    k__ENCODING__  "`__ENCODING__'"

%token <token>   tIDENTIFIER     "local variable or method"
%token <token>   tFID            "method"
%token <token>   tGVAR           "global variable"
%token <token>   tIVAR           "instance variable"
%token <token>   tCONSTANT       "constant"
%token <token>   tCVAR           "class variable"
%token <token>   tLABEL          "label"
%token <node> tINTEGER        "integer literal"
%token <node> tFLOAT          "float literal"
%token <node> tRATIONAL       "rational literal"
%token <node> tIMAGINARY      "imaginary literal"
%token <node> tCHAR           "char literal"
%token <node> tNTH_REF        "numbered reference"
%token <node> tBACK_REF       "back reference"
%token <node> tSTRING_CONTENT "literal content"
%token <num>  tREGEXP_END

%type <node> singleton strings string1 xstring regexp
%type <node> string_content
%type <node> words symbols qwords qsymbols
%type <node> literal numeric simple_numeric ssym dsym symbol cpath def_name defn_head defs_head
%type <node> top_compstmt top_stmts top_stmt begin_block rassign
%type <node> bodystmt compstmt stmts stmt_or_begin stmt expr arg primary command command_call method_call
%type <node> expr_value expr_value_do arg_value primary_value fcall rel_expr
%type <node> args call_args opt_call_args
%type <node> paren_args opt_paren_args
%type <node> command_args aref_args opt_block_arg block_arg var_ref var_lhs
%type <node> command_rhs arg_rhs
%type <node> command_asgn mrhs mrhs_arg block_call block_command
%type <node> f_block_opt
%type <node> f_arglist f_paren_args f_arg_item f_marg f_rest_marg
%type <node> assoc undef_list backref string_dvar for_var
%type <node> opt_block_param block_param_def f_opt
%type <node> f_kw f_block_kw
%type <node> bvar
%type <node> lambda f_larglist
%type <node> cmd_brace_block lhs fitem
%type <node> mlhs mlhs_head mlhs_basic mlhs_item mlhs_node mlhs_post mlhs_inner
%type <node> p_top_expr_body
%type <node> p_expr p_as p_alt p_expr_basic
%type <node> p_arg
%type <node> p_value p_primitive p_variable p_var_ref p_const
%type <node> p_kw
%type <node> f_block_arg keyword_variable user_variable

%type <node_list> assocs assoc_list opt_f_block_arg f_rest_arg f_optarg f_args
%type <node_list> f_block_optarg f_kwrest f_no_kwarg f_kwarg f_block_kwarg f_arg
%type <node_list> opt_args_tail args_tail
%type <node_list> regexp_contents xstring_contents string_contents
%type <node_list> qsym_list qword_list symbol_list word word_list
%type <node_list> string exc_list opt_rescue
%type <node_list> p_kwnorest p_kwrest p_any_kwrest p_kwarg p_kwargs p_args_post
%type <node_list> p_find p_args_tail p_args_head p_args p_top_expr p_case_body p_cases
%type <node_list> case_body cases case_args brace_body do_body bv_decls opt_bv_decl
%type <node_list> block_param opt_block_args_tail block_args_tail f_any_kwrest f_margs f_marg_list

%type <token_and_node> superclass opt_ensure exc_var opt_else if_tail

%type <token>   sym operation operation2 operation3
%type <token>   cname fname op f_norm_arg f_bad_arg
%type <token>   f_label f_arg_asgn call_op call_op2 reswords relop dot_or_colon
%type <token>   p_rest p_kw_label
%type <token>   args_forward excessed_comma
%type <token>   rbrace rparen rbracket p_lparen p_lbracket k_return then

%type <body> brace_block do_block lambda_body

%type <token_list> terms

%type <none> none

%token END_OF_INPUT 0   "end-of-input"
%token <token> tDOT
/* escaped chars, should be ignored otherwise */
%token <token> '\\'        "backslash"
%token tSP              "escaped space"
%token <token> '\t'        "escaped horizontal tab"
%token <token> '\f'        "escaped form feed"
%token <token> '\r'        "escaped carriage return"
%token <token> '\13'       "escaped vertical tab"
%token <token> tUPLUS           "unary+"
%token <token> tUMINUS          "unary-"
%token <token> tPOW             "**"
%token <token> tCMP        "<=>"
%token <token> tEQ         "=="
%token <token> tEQQ        "==="
%token <token> tNEQ        "!="
%token <token> tGEQ        ">="
%token <token> tLEQ        "<="
%token <token> tANDOP           "&&"
%token <token> tOROP            "||"
%token <token> tMATCH      "=~"
%token <token> tNMATCH     "!~"
%token <token> tDOT2            ".."
%token <token> tDOT3            "..."
%token <token> tBDOT2           "(.."
%token <token> tBDOT3           "(..."
%token <token> tAREF            "[]"
%token <token> tASET            "[]="
%token <token> tLSHFT      "<<"
%token <token> tRSHFT      ">>"
%token <token> tANDDOT     "&."
%token <token> tCOLON2     "::"
%token <token> tCOLON3          ":: at EXPR_BEG"
%token <token> tOP_ASGN    "operator-assignment" /* +=, -=  etc. */
%token <token> tASSOC           "=>"
%token <token> tLPAREN          "("
%token <token> tLPAREN_ARG      "( arg"
%token <token> tRPAREN          ")"
%token <token> tLBRACK          "["
%token <token> tLBRACE          "{"
%token <token> tLBRACE_ARG      "{ arg"
%token <token> tSTAR            "*"
%token <token> tDSTAR           "**arg"
%token <token> tAMPER           "&"
%token <token> tLAMBDA          "->"
%token <token> tSYMBEG          "symbol literal"
%token <token> tSTRING_BEG      "string literal"
%token <token> tXSTRING_BEG     "backtick literal"
%token <token> tREGEXP_BEG      "regexp literal"
%token <token> tWORDS_BEG       "word list"
%token <token> tQWORDS_BEG      "verbatim word list"
%token <token> tSYMBOLS_BEG     "symbol list"
%token <token> tQSYMBOLS_BEG    "verbatim symbol list"
%token <token> tSTRING_END      "terminator"
%token <token> tSTRING_DEND     "tRCURLY"
%token <token> tSTRING_DBEG
%token <token> tSTRING_DVAR
%token <token> tLAMBEG
%token <token> tLABEL_END

%token <token> tCOMMA           ","
%token <token> tLCURLY          "{ (tLCURLY)"
%token <token> tRCURLY          "}"
%token <token> tLBRACK2         "[ (tLBRACK2)"
%token <token> tEQL             "="
%token <token> tPIPE            "|"
%token <token> tAMPER2          "& (tAMPER2)"
%token <token> tGT              ">"
%token <token> tLT              "<"
%token <token> tBACK_REF2       "`"
%token <token> tCARET           "^"
%token <token> tLPAREN2         "( (tLPAREN2)"
%token <token> tRBRACK          "]"
%token <token> tSEMI            ";"
%token <token> tSPACE            " "
%token <token> tNL              "\n"
%token <token> tPLUS            "+"
%token <token> tMINUS           "-"
%token <token> tSTAR2           "* (tSTAR2)"
%token <token> tDIVIDE          "/"
%token <token> tPERCENT         "%"
%token <token> tTILDE           "~"
%token <token> tBANG            "!"

/*
 *	precedence table
 */

%nonassoc tLOWEST
%nonassoc tLBRACE_ARG

%nonassoc  kIF_MOD kUNLESS_MOD kWHILE_MOD kUNTIL_MOD kIN
%left  kOR kAND
%right kNOT
%nonassoc kDEFINED
%right tEQL tOP_ASGN
%left kRESCUE_MOD
%right tEH tCOLON
%nonassoc tDOT2 tDOT3 tBDOT2 tBDOT3
%left  tOROP
%left  tANDOP
%nonassoc  tCMP tEQ tEQQ tNEQ tMATCH tNMATCH
%left  tGT tGEQ tLT tLEQ
%left  tPIPE tCARET
%left  tAMPER2
%left  tLSHFT tRSHFT
%left  tPLUS tMINUS
%left  tSTAR2 tDIVIDE tPERCENT
%right tUMINUS_NUM tUMINUS
%right tPOW
%right tBANG tTILDE tUPLUS

%token tLAST_TOKEN

/* Grammar follows */
%%

         program:   {

                    }
                  top_compstmt
                    {

                    }
                ;

    top_compstmt: top_stmts opt_terms
                    {

                    }
                ;

       top_stmts: none
                    {
                      let _trigger_locs = @1;
                    }
                | top_stmt
                    {

                    }
                | top_stmts terms top_stmt
                    {

                    }
                | error top_stmt
                    {

                    }
                ;

        top_stmt: stmt
                | klBEGIN begin_block
                    {

                    }
                ;

     begin_block: tLCURLY top_compstmt tRCURLY
                    {

                    }
                ;

        bodystmt: compstmt opt_rescue
                  k_else {}
                  compstmt
                  opt_ensure
                    {
                        let opt_ensure = $<RAW>2;
                        let (ensure_t, ensure) = match opt_ensure {
                            Value::TokenAndNode((token, node)) => ( Some(token), Some(node) ),
                            Value::None => (None, None),
                            _ => panic!("Expected TokenAndNode/None, got {:#?}", opt_ensure)
                        };
                    }
                | compstmt
                  opt_rescue
                  opt_ensure
                    {
                        let opt_ensure = $<RAW>2;
                        let (ensure_t, ensure) = match opt_ensure {
                            Value::TokenAndNode((token, node)) => ( Some(token), Some(node) ),
                            Value::None => (None, None),
                            _ => panic!("Expected TokenAndNode/None, got {:#?}", opt_ensure)
                        };
                    }
                ;

        compstmt: stmts opt_terms
                    {

                    }
                ;

           stmts: none
                    {

                    }
                | stmt_or_begin
                    {

                    }
                | stmts terms stmt_or_begin
                    {

                    }
                | error stmt
                    {

                    }
                ;

   stmt_or_begin: stmt
                    {

                    }
                | klBEGIN
                    {

                    }
                  begin_block
                    {

                    }
                ;

            stmt: kALIAS fitem {} fitem
                    {

                    }
                | kALIAS tGVAR tGVAR
                    {

                    }
                | kALIAS tGVAR tBACK_REF
                    {

                    }
                | kALIAS tGVAR tNTH_REF
                    {

                    }
                | kUNDEF undef_list
                    {

                    }
                | stmt kIF_MOD expr_value
                    {

                    }
                | stmt kUNLESS_MOD expr_value
                    {

                    }
                | stmt kWHILE_MOD expr_value
                    {

                    }
                | stmt kUNTIL_MOD expr_value
                    {

                    }
                | stmt kRESCUE_MOD stmt
                    {

                    }
                | klEND tLCURLY compstmt tRCURLY
                    {

                    }
                | command_asgn
                | mlhs tEQL command_call
                    {

                    }
                | lhs tEQL mrhs
                    {

                    }
                | mlhs tEQL mrhs_arg kRESCUE_MOD stmt
                    {

                    }
                | mlhs tEQL mrhs_arg
                    {

                    }
                | rassign
                | expr
                ;

         rassign: arg_value tASSOC lhs
                    {

                    }
                | arg_value tASSOC mlhs
                    {

                    }
                | rassign tASSOC lhs
                    {

                    }
                | rassign tASSOC mlhs
                    {

                    }
                ;

    command_asgn: lhs tEQL command_rhs
                    {

                    }
                | var_lhs tOP_ASGN command_rhs
                    {

                    }
                | primary_value tLBRACK2 opt_call_args rbracket tOP_ASGN command_rhs
                    {

                    }
                | primary_value call_op tIDENTIFIER tOP_ASGN command_rhs
                    {

                    }
                | primary_value call_op tCONSTANT tOP_ASGN command_rhs
                    {

                    }
                | primary_value tCOLON2 tCONSTANT tOP_ASGN command_rhs
                    {

                    }
                | primary_value tCOLON2 tIDENTIFIER tOP_ASGN command_rhs
                    {

                    }
                | backref tOP_ASGN command_rhs
                    {

                    }
                ;

     command_rhs: command_call   %prec tOP_ASGN
                    {

                    }
                | command_call kRESCUE_MOD stmt
                    {

                    }
                | command_asgn
                ;

            expr: command_call
                | expr kAND expr
                    {

                    }
                | expr kOR expr
                    {

                    }
                | kNOT opt_nl expr
                    {

                    }
                | tBANG command_call
                    {

                    }
                | arg kIN
                    {

                    }
                  p_expr
                    {

                    }
                | arg %prec tLBRACE_ARG
                ;

        def_name: fname
                    {

                    }
                ;

       defn_head: k_def def_name
                    {

                    }
                ;

       defs_head: k_def singleton dot_or_colon {} def_name
                    {

                    }
                ;

      expr_value: expr
                ;

   expr_value_do: {} expr_value do {}
                    {

                    }
                ;


    command_call: command
                | block_command
                ;

   block_command: block_call
                | block_call call_op2 operation2 command_args
                ;

 cmd_brace_block: tLBRACE_ARG brace_body tRCURLY
                    {

                    }
                ;

           fcall: operation
                    {

                    }
                ;

         command: fcall command_args       %prec tLOWEST
                    {

                    }
                | fcall command_args cmd_brace_block
                    {

                    }
                | primary_value call_op operation2 command_args %prec tLOWEST
                    {

                    }
                | primary_value call_op operation2 command_args cmd_brace_block
                    {

                    }
                | primary_value tCOLON2 operation2 command_args %prec tLOWEST
                    {

                    }
                | primary_value tCOLON2 operation2 command_args cmd_brace_block
                    {

                    }
                | kSUPER command_args
                    {

                    }
                | kYIELD command_args
                    {

                    }
                | k_return call_args
                    {

                    }
                | kBREAK call_args
                    {

                    }
                | kNEXT call_args
                    {

                    }
                ;

            mlhs: mlhs_basic
                | tLPAREN mlhs_inner rparen
                    {

                    }
                ;

      mlhs_inner: mlhs_basic
                | tLPAREN mlhs_inner rparen
                    {

                    }
                ;

      mlhs_basic: mlhs_head
                | mlhs_head mlhs_item
                    {

                    }
                | mlhs_head tSTAR mlhs_node
                    {

                    }
                | mlhs_head tSTAR mlhs_node tCOMMA mlhs_post
                    {

                    }
                | mlhs_head tSTAR
                    {

                    }
                | mlhs_head tSTAR tCOMMA mlhs_post
                    {

                    }
                | tSTAR mlhs_node
                    {

                    }
                | tSTAR mlhs_node tCOMMA mlhs_post
                    {

                    }
                | tSTAR
                    {

                    }
                | tSTAR tCOMMA mlhs_post
                    {

                    }
                ;

       mlhs_item: mlhs_node
                | tLPAREN mlhs_inner rparen
                    {

                    }
                ;

       mlhs_head: mlhs_item tCOMMA
                    {

                    }
                | mlhs_head mlhs_item tCOMMA
                    {

                    }
                ;

       mlhs_post: mlhs_item
                    {

                    }
                | mlhs_post tCOMMA mlhs_item
                    {

                    }
                ;

       mlhs_node: user_variable
                    {
                        // NOTE: user_variable can be Value::IDENT
                    }
                | keyword_variable
                    {

                    }
                | primary_value tLBRACK2 opt_call_args rbracket
                    {

                    }
                | primary_value call_op tIDENTIFIER
                    {

                    }
                | primary_value tCOLON2 tIDENTIFIER
                    {

                    }
                | primary_value call_op tCONSTANT
                    {

                    }
                | primary_value tCOLON2 tCONSTANT
                    {

                    }
                | tCOLON3 tCONSTANT
                    {

                    }
                | backref
                    {

                    }
                ;

             lhs: user_variable
                    {
                        // NOTE: user_variable can be Value::IDENT
                    }
                | keyword_variable
                    {

                    }
                | primary_value tLBRACK2 opt_call_args rbracket
                    {

                    }
                | primary_value call_op tIDENTIFIER
                    {

                    }
                | primary_value tCOLON2 tIDENTIFIER
                    {

                    }
                | primary_value call_op tCONSTANT
                    {

                    }
                | primary_value tCOLON2 tCONSTANT
                    {

                    }
                | tCOLON3 tCONSTANT
                    {

                    }
                | backref
                    {

                    }
                ;

           cname: tIDENTIFIER
                    {

                    }
                | tCONSTANT
                ;

           cpath: tCOLON3 cname
                    {

                    }
                | cname
                    {

                    }
                | primary_value tCOLON2 cname
                    {

                    }
                ;

           fname: tIDENTIFIER
                | tCONSTANT
                | tFID
                | op
                    {

                    }
                | reswords
                ;

           fitem: fname
                    {

                    }
                | symbol
                ;

      undef_list: fitem
                    {

                    }
                | undef_list tCOMMA {} fitem
                    {

                    }
                ;

              op: tPIPE      { $$ = $1; }
                | tCARET     { $$ = $1; }
                | tAMPER2    { $$ = $1; }
                | tCMP       { $$ = $1; }
                | tEQ        { $$ = $1; }
                | tEQQ       { $$ = $1; }
                | tMATCH     { $$ = $1; }
                | tNMATCH    { $$ = $1; }
                | tGT        { $$ = $1; }
                | tGEQ       { $$ = $1; }
                | tLT        { $$ = $1; }
                | tLEQ       { $$ = $1; }
                | tNEQ       { $$ = $1; }
                | tLSHFT     { $$ = $1; }
                | tRSHFT     { $$ = $1; }
                | tPLUS      { $$ = $1; }
                | tMINUS     { $$ = $1; }
                | tSTAR2     { $$ = $1; }
                | tSTAR      { $$ = $1; }
                | tDIVIDE    { $$ = $1; }
                | tPERCENT   { $$ = $1; }
                | tPOW       { $$ = $1; }
                | tDSTAR     { $$ = $1; }
                | tBANG      { $$ = $1; }
                | tTILDE     { $$ = $1; }
                | tUPLUS     { $$ = $1; }
                | tUMINUS    { $$ = $1; }
                | tAREF      { $$ = $1; }
                | tASET      { $$ = $1; }
                | tBACK_REF2 { $$ = $1; }
                ;

        reswords: k__LINE__ | k__FILE__ | k__ENCODING__ | klBEGIN | klEND
                | kALIAS    | kAND      | kBEGIN        | kBREAK  | kCASE
                | kCLASS    | kDEF      | kDEFINED      | kDO     | kELSE
                | kELSIF    | kEND      | kENSURE       | kFALSE  | kFOR
                | kIN       | kMODULE   | kNEXT         | kNIL    | kNOT
                | kOR       | kREDO     | kRESCUE       | kRETRY  | kRETURN
                | kSELF     | kSUPER    | kTHEN         | kTRUE   | kUNDEF
                | kWHEN     | kYIELD    | kIF           | kUNLESS | kWHILE
                | kUNTIL
                ;

             arg: lhs tEQL arg_rhs
                    {

                    }
                | var_lhs tOP_ASGN arg_rhs
                    {

                    }
                | primary_value tLBRACK2 opt_call_args rbracket tOP_ASGN arg_rhs
                    {

                    }
                | primary_value call_op tIDENTIFIER tOP_ASGN arg_rhs
                    {

                    }
                | primary_value call_op tCONSTANT tOP_ASGN arg_rhs
                    {

                    }
                | primary_value tCOLON2 tIDENTIFIER tOP_ASGN arg_rhs
                    {

                    }
                | primary_value tCOLON2 tCONSTANT tOP_ASGN arg_rhs
                    {

                    }
                | tCOLON3 tCONSTANT tOP_ASGN arg_rhs
                    {

                    }
                | backref tOP_ASGN arg_rhs
                    {

                    }
                | arg tDOT2 arg
                    {

                    }
                | arg tDOT3 arg
                    {

                    }
                | arg tDOT2
                    {

                    }
                | arg tDOT3
                    {

                    }
                | tBDOT2 arg
                    {

                    }
                | tBDOT3 arg
                    {

                    }
                | arg tPLUS arg
                    {

                    }
                | arg tMINUS arg
                    {

                    }
                | arg tSTAR2 arg
                    {

                    }
                | arg tDIVIDE arg
                    {

                    }
                | arg tPERCENT arg
                    {

                    }
                | arg tPOW arg
                    {

                    }
                | tUMINUS_NUM simple_numeric tPOW arg
                    {

                    }
                | tUPLUS arg
                    {

                    }
                | tUMINUS arg
                    {

                    }
                | arg tPIPE arg
                    {

                    }
                | arg tCARET arg
                    {

                    }
                | arg tAMPER2 arg
                    {

                    }
                | arg tCMP arg
                    {

                    }
                | rel_expr   %prec tCMP
                    {

                    }
                | arg tEQ arg
                    {

                    }
                | arg tEQQ arg
                    {

                    }
                | arg tNEQ arg
                    {

                    }
                | arg tMATCH arg
                    {

                    }
                | arg tNMATCH arg
                    {

                    }
                | tBANG arg
                    {

                    }
                | tTILDE arg
                    {

                    }
                | arg tLSHFT arg
                    {

                    }
                | arg tRSHFT arg
                    {

                    }
                | arg tANDOP arg
                    {

                    }
                | arg tOROP arg
                    {

                    }
                | kDEFINED opt_nl arg
                    {

                    }
                | arg tEH arg opt_nl tCOLON arg
                    {

                    }
                | defn_head f_paren_args tEQL arg
                    {

                    }
                | defn_head f_paren_args tEQL arg kRESCUE_MOD arg
                    {

                    }
                | defs_head f_paren_args tEQL arg
                    {

                    }
                | defs_head f_paren_args tEQL arg kRESCUE_MOD arg
                    {

                    }
                | primary
                ;

           relop: tGT
                | tLT
                | tGEQ
                | tLEQ
                ;

        rel_expr: arg relop arg   %prec tGT
                    {

                    }
                | rel_expr relop arg   %prec tGT
                    {

                    }
                ;

       arg_value: arg
                ;

       aref_args: none
                | args trailer
                | args tCOMMA assocs trailer
                    {

                    }
                | assocs trailer
                    {

                    }
                ;

         arg_rhs: arg   %prec tOP_ASGN
                | arg kRESCUE_MOD arg
                ;

      paren_args: tLPAREN2 opt_call_args rparen
                    {

                    }
                | tLPAREN2 args tCOMMA args_forward rparen
                    {

                    }
                | tLPAREN2 args_forward rparen
                    {

                    }
                ;

  opt_paren_args: none
                    {

                    }
                | paren_args
                ;

   opt_call_args: none
                    {

                    }
                | call_args
                | args tCOMMA
                    {

                    }
                | args tCOMMA assocs tCOMMA
                    {

                    }
                | assocs tCOMMA
                    {

                    }
                ;

       call_args: command
                    {

                    }
                | args opt_block_arg
                    {

                    }
                | assocs opt_block_arg
                    {

                    }
                | args tCOMMA assocs opt_block_arg
                    {

                    }
                | block_arg
                    {

                    }
                ;

    command_args:   {

                    }
                  call_args
                    {

                    }
                ;

       block_arg: tAMPER arg_value
                    {

                    }
                ;

   opt_block_arg: tCOMMA block_arg
                    {

                    }
                | none
                    {

                    }
                ;

            args: arg_value
                    {

                    }
                | tSTAR arg_value
                    {

                    }
                | args tCOMMA arg_value
                    {

                    }
                | args tCOMMA tSTAR arg_value
                    {

                    }
                ;

        mrhs_arg: mrhs
                    {

                    }
                | arg_value
                ;

            mrhs: args tCOMMA arg_value
                    {

                    }
                | args tCOMMA tSTAR arg_value
                    {

                    }
                | tSTAR arg_value
                    {

                    }
                ;

         primary: literal
                | strings
                | xstring
                | regexp
                | words
                | qwords
                | symbols
                | qsymbols
                | var_ref
                | backref
                | tFID
                    {
                        // result = @builder.call_method(nil, nil, val[0])
                        $$ = Value::Node(Node::None);
                    }
                | k_begin
                    {
                        // @lexer.cmdarg.push(false)
                    }
                  bodystmt
                  k_end
                    {
                        // @lexer.cmdarg.pop

                        // result = @builder.begin_keyword(val[0], val[2], val[3])
                        $$ = Value::Node(Node::None);
                    }
                | tLPAREN_ARG { /* @lexer.state = :expr_endarg */ } rparen
                    {
                        // result = @builder.begin(val[0], val[1], val[3])
                        $$ = Value::Node(Node::None);
                    }
                | tLPAREN_ARG stmt { /* @lexer.state = :expr_endarg */ } rparen
                    {
                        // result = @builder.begin(val[0], val[1], val[3])
                        $$ = Value::Node(Node::None);
                    }
                | tLPAREN compstmt tRPAREN
                    {
                        // result = @builder.begin(val[0], val[1], val[2])
                        $$ = Value::Node(Node::None);
                    }
                | primary_value tCOLON2 tCONSTANT
                    {
                        // result = @builder.const_fetch(val[0], val[1], val[2])
                        $$ = Value::Node(Node::None);
                    }
                | tCOLON3 tCONSTANT
                    {
                        // result = @builder.const_global(val[0], val[1])
                        $$ = Value::Node(Node::None);
                    }
                | tLBRACK aref_args tRBRACK
                    {
                        // result = @builder.array(val[0], val[1], val[2])
                        $$ = Value::Node(Node::None);
                    }
                | tLBRACE assoc_list tRCURLY
                    {
                        // result = @builder.associate(val[0], val[1], val[2])
                        $$ = Value::Node(Node::None);
                    }
                | k_return
                    {
                        // result = @builder.keyword_cmd(:return, val[0])
                        $$ = Value::Node(Node::None);
                    }
                | kYIELD tLPAREN2 call_args rparen
                    {
                        // result = @builder.keyword_cmd(:yield, val[0], val[1], val[2], val[3])
                        $$ = Value::Node(Node::None);
                    }
                | kYIELD tLPAREN2 rparen
                    {
                        // result = @builder.keyword_cmd(:yield, val[0], val[1], [], val[2])
                        $$ = Value::Node(Node::None);
                    }
                | kYIELD
                    {
                        // result = @builder.keyword_cmd(:yield, val[0])
                        $$ = Value::Node(Node::None);
                    }
                | kDEFINED opt_nl tLPAREN2 {} expr rparen
                    {
                        // result = @builder.keyword_cmd(:defined?, val[0],
                        //                             val[2], [ val[3] ], val[4])
                        $$ = Value::Node(Node::None);
                    }
                | kNOT tLPAREN2 expr rparen
                    {
                        // result = @builder.not_op(val[0], val[1], val[2], val[3])
                        $$ = Value::Node(Node::None);
                    }
                | kNOT tLPAREN2 rparen
                    {
                        // result = @builder.not_op(val[0], val[1], nil, val[2])
                        $$ = Value::Node(Node::None);
                    }
                | fcall brace_block
                    {
                        // method_call = @builder.call_method(nil, nil, val[0])

                        // begin_t, args, body, end_t = val[1]
                        // result      = @builder.block(method_call,
                        //                 begin_t, args, body, end_t)
                        $$ = Value::Node(Node::None);
                    }
                | method_call
                | method_call brace_block
                    {
                        // begin_t, args, body, end_t = val[1]
                        // result      = @builder.block(val[0],
                        //                 begin_t, args, body, end_t)
                        $$ = Value::Node(Node::None);
                    }
                | lambda
                | k_if expr_value then
                  compstmt
                  if_tail
                  k_end
                    {
                        // else_t, else_ = val[4]
                        // result = @builder.condition(val[0], val[1], val[2],
                        //                             val[3], else_t,
                        //                             else_,  val[5])
                        $$ = Value::Node(Node::None);
                    }
                | k_unless expr_value then
                  compstmt
                  opt_else
                  k_end
                    {
                        // else_t, else_ = val[4]
                        // result = @builder.condition(val[0], val[1], val[2],
                        //                             else_,  else_t,
                        //                             val[3], val[5])
                        $$ = Value::Node(Node::None);
                    }
                | k_while expr_value_do
                  compstmt
                  k_end
                    {
                        // result = @builder.loop(:while, val[0], *val[1], val[2], val[3])
                        $$ = Value::Node(Node::None);
                    }
                | k_until expr_value_do
                  compstmt
                  k_end
                    {
                        // result = @builder.loop(:until, val[0], *val[1], val[2], val[3])
                        $$ = Value::Node(Node::None);
                    }
                | k_case expr_value opt_terms
                    {
                        // TODO: there's a warning that wq/parser doesn't trigger,
                        // search for `p->case_labels`
                    }
                  case_body
                  k_end
                    {
                        // *when_bodies, (else_t, else_body) = *val[3]

                        // result = @builder.case(val[0], val[1],
                        //                         when_bodies, else_t, else_body,
                        //                         val[4])
                        $$ = Value::Node(Node::None);
                    }
                | k_case opt_terms
                    {
                        // TODO: there's a warning that wq/parser doesn't trigger,
                        // search for `p->case_labels`
                    }
                  case_body
                  k_end
                    {
                        // *when_bodies, (else_t, else_body) = *val[2]

                        // result = @builder.case(val[0], nil,
                        //                         when_bodies, else_t, else_body,
                        //                         val[3])
                        $$ = Value::Node(Node::None);
                    }
                | k_case expr_value opt_terms
                  p_case_body
                  k_end
                    {
                        // *in_bodies, (else_t, else_body) = *val[3]

                        // result = @builder.case_match(val[0], val[1],
                        //                         in_bodies, else_t, else_body,
                        //                         val[4])
                        $$ = Value::Node(Node::None);
                    }
                | k_for for_var kIN expr_value_do
                  compstmt
                  k_end
                    {
                        // result = @builder.for(val[0], val[1], val[2], *val[3], val[4], val[5])
                        $$ = Value::Node(Node::None);
                    }
                | k_class cpath superclass
                    {
                        // @static_env.extend_static
                        // @lexer.cmdarg.push(false)
                        // @lexer.cond.push(false)
                        // @context.push(:class)
                    }
                  bodystmt
                  k_end
                    {
                        // unless @context.class_definition_allowed?
                        //     diagnostic :error, :class_in_def, nil, val[0]
                        // end

                        let superclass = $<RAW>3;
                        let (lt_t, superclass) = match superclass {
                            Value::TokenAndNode((token, node)) => (Some(token), Some(node)),
                            Value::None => (None, None),
                            _ => panic!("Expected TokenAndNode, got {:#?}", superclass)
                        };
                        // result = @builder.def_class(val[0], val[1],
                        //                             lt_t, superclass,
                        //                             val[4], val[5])

                        // @lexer.cmdarg.pop
                        // @lexer.cond.pop
                        // @static_env.unextend
                        // @context.pop
                        $$ = Value::Node(Node::None);
                    }
                | k_class tLSHFT expr
                    {
                        // @static_env.extend_static
                        // @lexer.cmdarg.push(false)
                        // @lexer.cond.push(false)
                        // @context.push(:sclass)
                    }
                  term
                  bodystmt
                  k_end
                    {
                        // result = @builder.def_sclass(val[0], val[1], val[2],
                        //                            val[5], val[6])

                        // @lexer.cmdarg.pop
                        // @lexer.cond.pop
                        // @static_env.unextend
                        // @context.pop
                        $$ = Value::Node(Node::None);
                    }
                | k_module cpath
                    {
                        // @static_env.extend_static
                        // @lexer.cmdarg.push(false)
                        // @context.push(:module)
                    }
                  bodystmt
                  k_end
                    {
                        // unless @context.module_definition_allowed?
                        //     diagnostic :error, :module_in_def, nil, val[0]
                        // end

                        // result = @builder.def_module(val[0], val[1],
                        //                             val[3], val[4])

                        // @lexer.cmdarg.pop
                        // @static_env.unextend
                        // @context.pop
                        $$ = Value::Node(Node::None);
                    }
                | defn_head
                  f_arglist
                  bodystmt
                  k_end
                    {
                        // result = @builder.def_method(*val[0], val[1],
                        //           val[2], val[3])

                        // @lexer.cmdarg.pop
                        // @lexer.cond.pop
                        // @static_env.unextend
                        // @context.pop
                        // @current_arg_stack.pop
                        $$ = Value::Node(Node::None);
                    }
                | defs_head
                  f_arglist
                  bodystmt
                  k_end
                    {
                        // result = @builder.def_singleton(*val[0], val[1],
                        //           val[2], val[3])

                        // @lexer.cmdarg.pop
                        // @lexer.cond.pop
                        // @static_env.unextend
                        // @context.pop
                        // @current_arg_stack.pop
                        $$ = Value::Node(Node::None);
                    }
                | kBREAK
                    {
                        // result = @builder.keyword_cmd(:break, val[0])
                        $$ = Value::Node(Node::None);
                    }
                | kNEXT
                    {
                        // result = @builder.keyword_cmd(:next, val[0])
                        $$ = Value::Node(Node::None);
                    }
                | kREDO
                    {
                        // result = @builder.keyword_cmd(:redo, val[0])
                        $$ = Value::Node(Node::None);
                    }
                | kRETRY
                    {
                        // result = @builder.keyword_cmd(:retry, val[0])
                        $$ = Value::Node(Node::None);
                    }
                ;

   primary_value: primary
                ;

         k_begin: kBEGIN
                ;

            k_if: kIF
                ;

        k_unless: kUNLESS
                ;

         k_while: kWHILE
                ;

         k_until: kUNTIL
                ;

          k_case: kCASE
                ;

           k_for: kFOR
                ;

         k_class: kCLASS
                ;

        k_module: kMODULE
                ;

           k_def: kDEF
                ;

            k_do: kDO
                ;

      k_do_block: kDO_BLOCK
                ;

        k_rescue: kRESCUE
                ;

        k_ensure: kENSURE
                ;

          k_when: kWHEN
                ;

          k_else: kELSE
                ;

         k_elsif: kELSIF
                ;

           k_end: kEND
                ;

        k_return: kRETURN
                    {
                        // if @context.in_class?
                        //     diagnostic :error, :invalid_return, nil, val[0]
                        // end
                        $$ = $<RAW>1;
                    }
                ;

            then: term
                | kTHEN
                | term kTHEN
                    {
                        $$ = $<RAW>2;
                    }
                ;

              do: term
                | kDO_COND
                ;

         if_tail: opt_else
                | k_elsif expr_value then
                  compstmt
                  if_tail
                    {
                        let opt_else = $<RAW>5;
                        let (else_t, else_) = match opt_else {
                            Value::TokenAndNode((token, node)) => (Some(token), Some(node)),
                            Value::None => (None, None),
                            _ => panic!("Expected TokenAndNode/None, got {:#?}", opt_else)
                        };
                        // result = [ val[0],
                        //             @builder.condition(val[0], val[1], val[2],
                        //                                 val[3], else_t,
                        //                                 else_,  nil),
                        //         ]
                        $$ = Value::Node(Node::None);
                    }
                ;

        opt_else: none
                    {
                        $$ = Value::None;
                    }
                | k_else compstmt
                    {
                        let token = $<Token>1;
                        let node  = $<Node>2;
                        $$ = Value::TokenAndNode((token, node));
                    }
                ;

         for_var: lhs
                | mlhs
                ;

          f_marg: f_norm_arg
                    {
                        // result = @builder.arg(val[0])
                        $$ = Value::Node(Node::None);
                    }
                | tLPAREN f_margs rparen
                    {
                        // result = @builder.multi_lhs(val[0], val[1], val[2])
                        $$ = Value::Node(Node::None);
                    }
                ;

     f_marg_list: f_marg
                    {
                        $$ = Value::NodeList( vec![ $<Node>1 ] );
                    }
                | f_marg_list tCOMMA f_marg
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( $<Node>3 );
                        $$ = Value::NodeList(nodes);
                    }
                ;

         f_margs: f_marg_list
                | f_marg_list tCOMMA f_rest_marg
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( $<Node>3 );
                        $$ = Value::NodeList(nodes);
                    }
                | f_marg_list tCOMMA f_rest_marg tCOMMA f_marg_list
                    {
                        let nodes = [ $<NodeList>1, vec![ $<Node>3 ], $<NodeList>5 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | f_rest_marg
                    {
                        $$ = Value::NodeList( vec![ $<Node>1 ] );
                    }
                | f_rest_marg tCOMMA f_marg_list
                    {
                        let nodes = [ vec![ $<Node>1 ], $<NodeList>3 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                ;

     f_rest_marg: tSTAR f_norm_arg
                    {
                        // result = @builder.restarg(val[0], val[1])
                        $$ = Value::Node(Node::None);
                    }
                | tSTAR
                    {
                        // result = @builder.restarg(val[0])
                        $$ = Value::Node(Node::None);
                    }
                ;

    f_any_kwrest: f_kwrest
                | f_no_kwarg
                ;

 block_args_tail: f_block_kwarg tCOMMA f_kwrest opt_f_block_arg
                    {
                        let nodes = [ $<NodeList>1, $<NodeList>3, $<NodeList>4 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | f_block_kwarg opt_f_block_arg
                    {
                        let nodes = [ $<NodeList>1, $<NodeList>2 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | f_any_kwrest opt_f_block_arg
                    {
                        let nodes = [ $<NodeList>1, $<NodeList>2 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | f_block_arg
                    {
                        $$ = Value::NodeList( vec![ $<Node>1 ] );
                    }
                ;

opt_block_args_tail:
                  tCOMMA block_args_tail
                    {
                        $$ = $<RAW>2;
                    }
                | /* none */
                    {
                        $$ = Value::NodeList( vec![] );
                    }
                ;

  excessed_comma: tCOMMA
                ;

     block_param: f_arg tCOMMA f_block_optarg tCOMMA f_rest_arg opt_block_args_tail
                    {
                        let nodes = [ $<NodeList>1, $<NodeList>3, $<NodeList>5, $<NodeList>6 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | f_arg tCOMMA f_block_optarg tCOMMA f_rest_arg tCOMMA f_arg opt_block_args_tail
                    {
                        let nodes = [ $<NodeList>1, $<NodeList>3, $<NodeList>5, $<NodeList>7, $<NodeList>8 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | f_arg tCOMMA f_block_optarg opt_block_args_tail
                    {
                        let nodes = [ $<NodeList>1, $<NodeList>3, $<NodeList>4 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | f_arg tCOMMA f_block_optarg tCOMMA f_arg opt_block_args_tail
                    {
                        let nodes = [ $<NodeList>1, $<NodeList>3, $<NodeList>4, $<NodeList>5 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | f_arg tCOMMA f_rest_arg opt_block_args_tail
                    {
                        let nodes = [ $<NodeList>1, $<NodeList>3, $<NodeList>4 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | f_arg excessed_comma
                | f_arg tCOMMA f_rest_arg tCOMMA f_arg opt_block_args_tail
                    {
                        let nodes = [ $<NodeList>1, $<NodeList>3, $<NodeList>5, $<NodeList>6 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | f_arg opt_block_args_tail
                    {
                        // if val[1].empty? && val[0].size == 1
                        //     result = [@builder.procarg0(val[0][0])]
                        // else
                        //     result = val[0].concat(val[1])
                        // end
                        $$ = Value::NodeList(vec![]);
                    }
                | f_block_optarg tCOMMA f_rest_arg opt_block_args_tail
                    {
                        let nodes = [ $<NodeList>1, $<NodeList>2, $<NodeList>3 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | f_block_optarg tCOMMA f_rest_arg tCOMMA f_arg opt_block_args_tail
                    {
                        let nodes = [ $<NodeList>1, $<NodeList>3, $<NodeList>5, $<NodeList>6 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | f_block_optarg opt_block_args_tail
                    {
                        let nodes = [ $<NodeList>1, $<NodeList>2 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | f_block_optarg tCOMMA f_arg opt_block_args_tail
                    {
                        let nodes = [ $<NodeList>1, $<NodeList>3, $<NodeList>4 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | f_rest_arg opt_block_args_tail
                    {
                        let nodes = [ $<NodeList>1, $<NodeList>2 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | f_rest_arg tCOMMA f_arg opt_block_args_tail
                    {
                        let nodes = [ $<NodeList>1, $<NodeList>3, $<NodeList>4 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | block_args_tail
                ;

 opt_block_param: none
                    {
                        // result = @builder.args(nil, [], nil)
                        $$ = Value::Node(Node::None);
                    }
                | block_param_def
                    {
                        // @lexer.state = :expr_value
                        $$ = $<RAW>1;
                    }
                ;

 block_param_def: tPIPE opt_bv_decl tPIPE
                    {
                        // @max_numparam_stack.has_ordinary_params!
                        // @current_arg_stack.set(nil)
                        // result = @builder.args(val[0], val[1], val[2])
                        $$ = Value::Node(Node::None);
                    }
                | tPIPE block_param opt_bv_decl tPIPE
                    {
                        // @max_numparam_stack.has_ordinary_params!
                        // @current_arg_stack.set(nil)
                        // result = @builder.args(val[0], val[1].concat(val[2]), val[3])
                        $$ = Value::Node(Node::None);
                    }
                ;


     opt_bv_decl: opt_nl
                    {
                        $$ = Value::NodeList( vec![] );
                    }
                | opt_nl tSEMI bv_decls opt_nl
                    {
                        $$ = $<RAW>3;
                    }
                ;

        bv_decls: bvar
                    {
                        $$ = Value::NodeList( vec![ $<Node>1 ] );
                    }
                | bv_decls tCOMMA bvar
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( $<Node>3 );
                        $$ = Value::NodeList(nodes);
                    }
                ;

            bvar: tIDENTIFIER
                    {
                        // @static_env.declare val[0][0]
                        // result = @builder.shadowarg(val[0])
                        $$ = Value::Node(Node::None);
                    }
                | f_bad_arg
                    {
                        // FIXME;
                        $$ = Value::None;
                    }
                ;

          lambda: tLAMBDA
                    {
                        // @static_env.extend_dynamic
                        // @max_numparam_stack.push
                        // @context.push(:lambda)
                    }
                  f_larglist
                    {
                        // @context.pop
                        // @lexer.cmdarg.push(false)
                    }
                  lambda_body
                    {
                        // lambda_call = @builder.call_lambda(val[0])
                        // args = @max_numparam_stack.has_numparams? ? @builder.numargs(@max_numparam_stack.top) : val[2]
                        // begin_t, body, end_t = val[4]

                        // @max_numparam_stack.pop
                        // @static_env.unextend
                        // @lexer.cmdarg.pop

                        // result      = @builder.block(lambda_call,
                        //                 begin_t, args, body, end_t)
                        $$ = Value::Node(Node::None);
                    }
                ;

      f_larglist: tLPAREN2 f_args opt_bv_decl tRPAREN
                    {
                        // @max_numparam_stack.has_ordinary_params!
                        // result = @builder.args(val[0], val[1].concat(val[2]), val[3])
                        $$ = Value::Node(Node::None);
                    }
                | f_args
                    {
                        // if val[0].any?
                        //     @max_numparam_stack.has_ordinary_params!
                        // end
                        // result = @builder.args(nil, val[0], nil)
                        $$ = Value::Node(Node::None);
                    }
                ;

     lambda_body: tLAMBEG
                    {
                        // @context.push(:lambda)
                    }
                  compstmt tRCURLY
                    {
                        // result = [ val[0], val[2], val[3] ]
                        // @context.pop
                        $$ = Value::Body(( $<Token>1, $<NodeList>3, $<Token>4 ));
                    }
                | kDO_LAMBDA
                    {
                        // @context.push(:lambda)
                    }
                  bodystmt k_end
                    {
                        // result = [ val[0], val[2], val[3] ]
                        // @context.pop
                        $$ = Value::Body(( $<Token>1, $<NodeList>3, $<Token>4 ));
                    }
                ;

        do_block: k_do_block
                    {
                        // @context.push(:block)
                    }
                  do_body k_end
                    {
                        // result = [ val[0], *val[2], val[3] ]
                        // @context.pop
                        $$ = Value::Body(( $<Token>1, $<NodeList>3, $<Token>4 ));
                    }
                ;

      block_call: command do_block
                    {
                        // begin_t, block_args, body, end_t = val[1]
                        // result      = @builder.block(val[0],
                        //                 begin_t, block_args, body, end_t)
                        $$ = Value::Node(Node::None);
                    }
                | block_call call_op2 operation2 opt_paren_args
                    {
                        // lparen_t, args, rparen_t = val[3]
                        // result = @builder.call_method(val[0], val[1], val[2],
                        //             lparen_t, args, rparen_t)
                        $$ = Value::Node(Node::None);
                    }
                | block_call call_op2 operation2 opt_paren_args brace_block
                    {
                        // lparen_t, args, rparen_t = val[3]
                        // method_call = @builder.call_method(val[0], val[1], val[2],
                        //                 lparen_t, args, rparen_t)

                        // begin_t, args, body, end_t = val[4]
                        // result      = @builder.block(method_call,
                        //                 begin_t, args, body, end_t)
                        $$ = Value::Node(Node::None);
                    }
                | block_call call_op2 operation2 command_args do_block
                    {
                        // method_call = @builder.call_method(val[0], val[1], val[2],
                        //                 nil, val[3], nil)

                        // begin_t, args, body, end_t = val[4]
                        // result      = @builder.block(method_call,
                        //                 begin_t, args, body, end_t)
                        $$ = Value::Node(Node::None);
                    }
                ;

     method_call: fcall paren_args
                    {
                        // lparen_t, args, rparen_t = val[1]
                        // result = @builder.call_method(nil, nil, val[0],
                        //             lparen_t, args, rparen_t)
                        $$ = Value::Node(Node::None);
                    }
                | primary_value call_op operation2 opt_paren_args
                    {
                        // lparen_t, args, rparen_t = val[3]
                        // result = @builder.call_method(val[0], val[1], val[2],
                        //             lparen_t, args, rparen_t)
                        $$ = Value::Node(Node::None);
                    }
                | primary_value tCOLON2 operation2 paren_args
                    {
                        // lparen_t, args, rparen_t = val[3]
                        // result = @builder.call_method(val[0], val[1], val[2],
                        //           lparen_t, args, rparen_t)
                        $$ = Value::Node(Node::None);
                    }
                | primary_value tCOLON2 operation3
                    {
                        // result = @builder.call_method(val[0], val[1], val[2])
                        $$ = Value::Node(Node::None);
                    }
                | primary_value call_op paren_args
                    {
                        // lparen_t, args, rparen_t = val[2]
                        // result = @builder.call_method(val[0], val[1], nil,
                        //             lparen_t, args, rparen_t)
                        $$ = Value::Node(Node::None);
                    }
                | primary_value tCOLON2 paren_args
                    {
                        // lparen_t, args, rparen_t = val[2]
                        // result = @builder.call_method(val[0], val[1], nil,
                        //             lparen_t, args, rparen_t)
                        $$ = Value::Node(Node::None);
                    }
                | kSUPER paren_args
                    {
                        // lparen_t, args, rparen_t = val[1]
                        // result = @builder.keyword_cmd(:super, val[0],
                        //             lparen_t, args, rparen_t)
                        $$ = Value::Node(Node::None);
                    }
                | kSUPER
                    {
                        // result = @builder.keyword_cmd(:zsuper, val[0])
                        $$ = Value::Node(Node::None);
                    }
                | primary_value tLBRACK2 opt_call_args rbracket
                    {
                        // result = @builder.index(val[0], val[1], val[2], val[3])
                        $$ = Value::Node(Node::None);
                    }
                ;

     brace_block: tLCURLY
                    {
                        // @context.push(:block)
                    }
                  brace_body tRCURLY
                    {
                        // result = [ val[0], *val[2], val[3] ]
                        // @context.pop

                        $$ = Value::Body(( $<Token>1, $<NodeList>3, $<Token>4 ));
                    }
                | k_do
                    {
                        // @context.push(:block)
                    }
                  do_body k_end
                    {
                        // result = [ val[0], *val[2], val[3] ]
                        // @context.pop

                        $$ = Value::Body(( $<Token>1, $<NodeList>3, $<Token>4 ));
                    }
                ;

      brace_body:   {
                        // @static_env.extend_dynamic
                        // @max_numparam_stack.push
                    }
                  opt_block_param compstmt
                    {
                        // args = @max_numparam_stack.has_numparams? ? @builder.numargs(@max_numparam_stack.top) : val[1]
                        // result = [ args, val[2] ]

                        // @max_numparam_stack.pop
                        // @static_env.unextend

                        $$ = Value::NodeList(vec![]);
                    }
                ;

         do_body:   {
                        // @static_env.extend_dynamic
                        // @max_numparam_stack.push
                        // @lexer.cmdarg.push(false)
                    }
                  opt_block_param bodystmt
                    {
                        // args = @max_numparam_stack.has_numparams? ? @builder.numargs(@max_numparam_stack.top) : val[2]
                        // result = [ args, val[3] ]

                        // @max_numparam_stack.pop
                        // @static_env.unextend
                        // @lexer.cmdarg.pop

                        $$ = Value::NodeList(vec![]);
                    }
                ;

       case_args: arg_value
                    {
                        $$ = Value::NodeList( vec![ $<Node>1 ] );
                    }
                | tSTAR arg_value
                    {
                        // result = [ @builder.splat(val[0], val[1]) ]
                        $$ = Value::NodeList( vec![] );
                    }
                | case_args tCOMMA arg_value
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( $<Node>3 );
                        $$ = Value::NodeList(nodes);
                    }
                | case_args tCOMMA tSTAR arg_value
                    {
                        // result = val[0] << @builder.splat(val[2], val[3])
                        $$ = Value::NodeList( vec![] );
                    }
                ;

       case_body: k_when case_args then
                  compstmt
                  cases
                    {
                        // @builder.when(val[0], val[1], val[2], val[3])
                        let when = Node::None;
                        let nodes = [ vec![when], $<NodeList>5 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                ;

           cases: opt_else
                    {
                        $$ = Value::NodeList( vec![ $<Node>1 ] );
                    }
                | case_body
                ;

     p_case_body: kIN
                    {
                        // @lexer.state = :expr_beg
                        // @lexer.command_start = false
                        // @pattern_variables.push
                        // @pattern_hash_keys.push

                        // result = @lexer.in_kwarg
                        // @lexer.in_kwarg = true
                    }
                  p_top_expr then
                    {
                        // @lexer.in_kwarg = val[1]
                    }
                  compstmt
                  p_cases
                    {
                        // @builder.in_pattern(val[0], *val[2], val[3], val[5])
                        let pattern = Node::None;
                        let nodes = [ vec![pattern], $<NodeList>7 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                ;

         p_cases: opt_else
                    {
                        $$ = Value::NodeList( vec![ $<Node>1 ] );
                    }
                | p_case_body
                ;

      p_top_expr: p_top_expr_body
                    {
                        $$ = Value::NodeList( vec![ $<Node>1 ] );
                    }
                | p_top_expr_body kIF_MOD expr_value
                    {
                        // @builder.if_guard(val[1], val[2])
                        let guard = Node::None;
                        $$ = Value::NodeList( vec![ $<Node>1, guard ] );
                    }
                | p_top_expr_body kUNLESS_MOD expr_value
                    {
                        // @builder.unless_guard(val[1], val[2])
                        let guard = Node::None;
                        $$ = Value::NodeList( vec![ $<Node>1, guard ] );
                    }
                ;

 p_top_expr_body: p_expr
                | p_expr tCOMMA
                    {
                        // array patterns that end with comma
                        // like 1, 2,
                        // must be emitted as `array_pattern_with_tail`
                        // item = @builder.match_with_trailing_comma(val[0], val[1])
                        // result = @builder.array_pattern(nil, [ item ], nil)
                        $$ = Value::Node(Node::None);
                    }
                | p_expr tCOMMA p_args
                    {
                        // result = @builder.array_pattern(nil, [val[0]].concat(val[2]), nil)
                        $$ = Value::Node(Node::None);
                    }
                | p_find
                    {
                        // result = @builder.find_pattern(nil, val[0], nil)
                        $$ = Value::Node(Node::None);
                    }
                | p_args_tail
                    {
                        // result = @builder.array_pattern(nil, val[0], nil)
                        $$ = Value::Node(Node::None);
                    }
                | p_kwargs
                    {
                        // result = @builder.hash_pattern(nil, val[0], nil)
                        $$ = Value::Node(Node::None);
                    }
                ;

          p_expr: p_as
                ;

            p_as: p_expr tASSOC p_variable
                    {
                        // result = @builder.match_as(val[0], val[1], val[2])
                        $$ = Value::Node(Node::None);
                    }
                | p_alt
                ;

           p_alt: p_alt tPIPE p_expr_basic
                    {
                        // result = @builder.match_alt(val[0], val[1], val[2])
                        $$ = Value::Node(Node::None);
                    }
                | p_expr_basic
                ;

        p_lparen: tLPAREN2
                    {
                        $$ = $<RAW>1;
                        // @pattern_hash_keys.push
                    }
                ;

      p_lbracket: tLBRACK2
                    {
                        $$ = $<RAW>1;
                        // @pattern_hash_keys.push
                    }
                ;

    p_expr_basic: p_value
                | p_const p_lparen p_args rparen
                    {
                        // @pattern_hash_keys.pop
                        // pattern = @builder.array_pattern(nil, val[2], nil)
                        // result = @builder.const_pattern(val[0], val[1], pattern, val[3])
                        $$ = Value::Node(Node::None);
                    }
                | p_const p_lparen p_find rparen
                    {
                        // @pattern_hash_keys.pop
                        // pattern = @builder.find_pattern(nil, val[2], nil)
                        // result = @builder.const_pattern(val[0], val[1], pattern, val[3])
                        $$ = Value::Node(Node::None);
                    }
                | p_const p_lparen p_kwargs rparen
                    {
                        // @pattern_hash_keys.pop
                        // pattern = @builder.hash_pattern(nil, val[2], nil)
                        // result = @builder.const_pattern(val[0], val[1], pattern, val[3])
                        $$ = Value::Node(Node::None);
                    }
                | p_const tLPAREN2 rparen
                    {
                        // pattern = @builder.array_pattern(val[1], nil, val[2])
                        // result = @builder.const_pattern(val[0], val[1], pattern, val[2])
                        $$ = Value::Node(Node::None);
                    }
                | p_const p_lbracket p_args rbracket
                    {
                        // @pattern_hash_keys.pop
                        // pattern = @builder.array_pattern(nil, val[2], nil)
                        // result = @builder.const_pattern(val[0], val[1], pattern, val[3])
                        $$ = Value::Node(Node::None);
                    }
                | p_const p_lbracket p_find rbracket
                    {
                        // @pattern_hash_keys.pop
                        // pattern = @builder.find_pattern(nil, val[2], nil)
                        // result = @builder.const_pattern(val[0], val[1], pattern, val[3])
                        $$ = Value::Node(Node::None);
                    }
                | p_const p_lbracket p_kwargs rbracket
                    {
                        // @pattern_hash_keys.pop
                        // pattern = @builder.hash_pattern(nil, val[2], nil)
                        // result = @builder.const_pattern(val[0], val[1], pattern, val[3])
                        $$ = Value::Node(Node::None);
                    }
                | p_const tLBRACK2 rbracket
                    {
                        // pattern = @builder.array_pattern(val[1], nil, val[2])
                        // result = @builder.const_pattern(val[0], val[1], pattern, val[2])
                        $$ = Value::Node(Node::None);
                    }
                | tLBRACK p_args rbracket
                    {
                        // result = @builder.array_pattern(val[0], val[1], val[2])
                        $$ = Value::Node(Node::None);
                    }
                | tLBRACK p_find rbracket
                    {
                        // result = @builder.find_pattern(val[0], val[1], val[2])
                        $$ = Value::Node(Node::None);
                    }
                | tLBRACK rbracket
                    {
                        // result = @builder.array_pattern(val[0], [], val[1])
                        $$ = Value::Node(Node::None);
                    }
                | tLBRACE
                    {
                        // @pattern_hash_keys.push
                        // result = @lexer.in_kwarg
                        // @lexer.in_kwarg = false
                    }
                  p_kwargs rbrace
                    {
                        // @pattern_hash_keys.pop
                        // @lexer.in_kwarg = val[1]
                        // result = @builder.hash_pattern(val[0], val[2], val[3])
                        $$ = Value::Node(Node::None);
                    }
                | tLBRACE rbrace
                    {
                        // result = @builder.hash_pattern(val[0], [], val[1])
                        $$ = Value::Node(Node::None);
                    }
                | tLPAREN
                    {
                        // @pattern_hash_keys.push
                    }
                  p_expr rparen
                    {
                        // @pattern_hash_keys.pop
                        // result = @builder.begin(val[0], val[2], val[3])
                        $$ = Value::Node(Node::None);
                    }
                ;

          p_args: p_expr
                    {
                        $$ = Value::NodeList( vec![ $<Node>1 ] );
                    }
                | p_args_head
                | p_args_head p_arg
                    {
                        let nodes = [ $<NodeList>1, vec![ $<Node>2 ] ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | p_args_head tSTAR tIDENTIFIER
                    {
                        // match_rest = @builder.match_rest(val[1], val[2])
                        // result = [ *val[0], match_rest ]
                        $$ = Value::NodeList( vec![] );
                    }
                | p_args_head tSTAR tIDENTIFIER tCOMMA p_args_post
                    {
                        // match_rest = @builder.match_rest(val[1], val[2])
                        // result = [ *val[0], match_rest, *val[4] ]
                        $$ = Value::NodeList( vec![] );
                    }
                | p_args_head tSTAR
                    {
                        // result = [ *val[0], @builder.match_rest(val[1]) ]
                        $$ = Value::NodeList( vec![] );
                    }
                | p_args_head tSTAR tCOMMA p_args_post
                    {
                        // result = [ *val[0], @builder.match_rest(val[1]), *val[3] ]
                        $$ = Value::NodeList( vec![] );
                    }
                | p_args_tail
                ;

     p_args_head: p_arg tCOMMA
                    {
                        // array patterns that end with comma
                        // like [1, 2,]
                        // must be emitted as `array_pattern_with_tail`
                        // item = @builder.match_with_trailing_comma(val[0], val[1])
                        // result = [ item ]
                        $$ = Value::NodeList( vec![] );
                    }
                | p_args_head p_arg tCOMMA
                    {
                        // array patterns that end with comma
                        // like [1, 2,]
                        // must be emitted as `array_pattern_with_tail`
                        // last_item = @builder.match_with_trailing_comma(val[1], val[2])
                        // result = [ *val[0], last_item ]
                        $$ = Value::NodeList( vec![] );
                    }
                ;

     p_args_tail: p_rest
                    {
                        $$ = Value::NodeList( vec![ $<Node>1 ] );
                    }
                | p_rest tCOMMA p_args_post
                    {
                        let nodes = [ vec![ $<Node>1 ], $<NodeList>3 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                ;

          p_find: p_rest tCOMMA p_args_post tCOMMA p_rest
                    {
                        let nodes = [ vec![ $<Node>1 ], $<NodeList>3, vec![ $<Node>5 ] ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                ;


          p_rest: tSTAR tIDENTIFIER
                    {
                        // result = @builder.match_rest(val[0], val[1])
                        $$ = Value::Node(Node::None);
                    }
                | tSTAR
                    {
                        // result = @builder.match_rest(val[0])
                        $$ = Value::Node(Node::None);
                    }
                ;

     p_args_post: p_arg
                    {
                        $$ = Value::NodeList( vec![ $<Node>1 ] );
                    }
                | p_args_post tCOMMA p_arg
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( $<Node>3 );
                        $$ = Value::NodeList(nodes);
                    }
                ;

           p_arg: p_expr
                ;

        p_kwargs: p_kwarg tCOMMA p_any_kwrest
                    {
                        let nodes = [ $<NodeList>1, $<NodeList>3 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | p_kwarg
                | p_kwarg tCOMMA
                | p_any_kwrest
                ;

         p_kwarg: p_kw
                    {
                        $$ = Value::NodeList( vec![ $<Node>1 ] );
                    }
                | p_kwarg tCOMMA p_kw
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( $<Node>3 );
                        $$ = Value::NodeList(nodes);
                    }
                ;

            p_kw: p_kw_label p_expr
                    {
                        let p_kw_label = $<RAW>1;
                        let p_expr = $<Node>2;
                        match p_kw_label {
                            Value::PlainLabel(label_t) => {
                                // result = @builder.match_plain_label_pair(label_t, p_expr);
                                $$ = Value::Node(Node::None);
                            },
                            Value::QuotedLabel((begin_t, parts, end_t)) => {
                                // result = @builder.match_quoted_label_pair(begin_t, parts, end_t, p_expr);
                                $$ = Value::Node(Node::None);
                            },
                            _ => panic!("Expected PlainLabel/QuotedLabel, got {:#?}", p_kw_label)
                        }
                    }
                | p_kw_label
                    {
                        let p_kw_label = $<RAW>1;
                        match p_kw_label {
                            Value::PlainLabel(label_t) => {
                                // result = @builder.match_plain_label(label_t);
                                $$ = Value::Node(Node::None);
                            },
                            Value::QuotedLabel((begin_t, parts, end_t)) => {
                                // result = @builder.match_quoted_label(begin_t, parts, end_t);
                                $$ = Value::Node(Node::None);
                            },
                            _ => panic!("Expected PlainLabel/QuotedLabel, got {:#?}", p_kw_label)
                        }
                    }
                ;

      p_kw_label: tLABEL
                    {
                        $$ = Value::PlainLabel($<Token>1);
                    }
                | tSTRING_BEG string_contents tLABEL_END
                    {
                        $$ = Value::QuotedLabel( ($<Token>1, $<NodeList>2, $<Token>3) );
                    }
                ;

        p_kwrest: kwrest_mark tIDENTIFIER
                    {
                        // result = [ @builder.match_rest(val[0], val[1]) ]
                        $$ = Value::NodeList( vec![] );
                    }
                | kwrest_mark
                    {
                        // result = [ @builder.match_rest(val[0], nil) ]
                        $$ = Value::NodeList( vec![] );
                    }
                ;

      p_kwnorest: kwrest_mark kNIL
                    {
                        // result = [ @builder.match_nil_pattern(val[0], val[1]) ]
                        $$ = Value::NodeList( vec![] );
                    }
                ;

    p_any_kwrest: p_kwrest
                | p_kwnorest
                ;

         p_value: p_primitive
                | p_primitive tDOT2 p_primitive
                    {
                        // result = @builder.range_inclusive(val[0], val[1], val[2])
                        $$ = Value::Node(Node::None);
                    }
                | p_primitive tDOT3 p_primitive
                    {
                        // result = @builder.range_exclusive(val[0], val[1], val[2])
                        $$ = Value::Node(Node::None);
                    }
                | p_primitive tDOT2
                    {
                        // result = @builder.range_inclusive(val[0], val[1], nil)
                        $$ = Value::Node(Node::None);
                    }
                | p_primitive tDOT3
                    {
                        // result = @builder.range_exclusive(val[0], val[1], nil)
                        $$ = Value::Node(Node::None);
                    }
                | p_variable
                | p_var_ref
                | p_const
                | tBDOT2 p_primitive
                    {
                        // result = @builder.range_inclusive(nil, val[0], val[1])
                        $$ = Value::Node(Node::None);
                    }
                | tBDOT3 p_primitive
                    {
                        // result = @builder.range_exclusive(nil, val[0], val[1])
                        $$ = Value::Node(Node::None);
                    }
                ;

     p_primitive: literal
                | strings
                | xstring
                | regexp
                | words
                | qwords
                | symbols
                | qsymbols
                | keyword_variable
                    {
                        // result = @builder.accessible(val[0])
                        $$ = Value::Node(Node::None);
                    }
                | lambda
                ;

      p_variable: tIDENTIFIER
                    {
                        // result = @builder.match_var(val[0])
                        $$ = Value::Node(Node::None);
                    }
                ;

       p_var_ref: tCARET tIDENTIFIER
                    {
                        // name = val[1][0]
                        // unless static_env.declared?(name)
                        //     diagnostic :error, :undefined_lvar, { :name => name }, val[1]
                        // end

                        // lvar = @builder.accessible(@builder.ident(val[1]))
                        // result = @builder.pin(val[0], lvar)
                        $$ = Value::Node(Node::None);
                    }
                ;

         p_const: tCOLON3 cname
                    {
                        // result = @builder.const_global(val[0], val[1])
                        $$ = Value::Node(Node::None);
                    }
                | p_const tCOLON2 cname
                    {
                        // result = @builder.const_fetch(val[0], val[1], val[2])
                        $$ = Value::Node(Node::None);
                    }
                | tCONSTANT
                    {
                        // result = @builder.const(val[0])
                        $$ = Value::Node(Node::None);
                    }
                ;

      opt_rescue: k_rescue exc_list exc_var then
                  compstmt
                  opt_rescue
                    {
                        let exc_var = $<RAW>3;
                        let (assoc_t, exc_var) = match exc_var {
                            Value::TokenAndNode((token, node)) => ( Some(token), Some(node) ),
                            Value::None => (None, None),
                            _ => panic!("Expected TokenAndNode/None, got {:#?}", exc_var)
                        };

                        let exc_list = $<NodeList>2;
                        let rescue_body = Node::None;
                        let opt_rescue = $<NodeList>6;

                        let rescues: Vec<Node> = [ vec![rescue_body], opt_rescue ].concat();
                        $$ = Value::NodeList(rescues);
                    }
                | none
                    {
                        $$ = Value::NodeList( vec![] );
                    }
                ;

        exc_list: arg_value
                    {
                        $$ = Value::NodeList( vec![ $<Node>1 ] );
                    }
                | mrhs
                | none
                    {
                        $$ = Value::NodeList( vec![] );
                    }
                ;

         exc_var: tASSOC lhs
                    {
                        let token = $<Token>1;
                        let node = $<Node>2;
                        $$ = Value::TokenAndNode( (token, node) );
                    }
                | none
                    {
                        $$ = Value::None;
                    }
                ;

      opt_ensure: k_ensure compstmt
                    {
                        let token = $<Token>1;
                        let node = $<Node>2;
                        $$ = Value::TokenAndNode((token, node));
                    }
                | none
                    {
                        $$ = Value::None;
                    }
                ;

         literal: numeric
                | symbol
                ;

         strings: string
                    {
                        // result = @builder.string_compose(nil, val[0], nil)
                        $$ = Value::Node(Node::None);
                    }
                ;

          string: tCHAR
                    {
                        // result = [ @builder.character(val[0]) ]
                        $$ = Value::NodeList( vec![] );
                    }
                | string1
                    {
                        $$ = Value::NodeList( vec![ $<Node>1 ] );
                    }
                | string string1
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( $<Node>2 );
                        $$ = Value::NodeList(nodes);
                    }
                ;

         string1: tSTRING_BEG string_contents tSTRING_END
                    {
                        // string = @builder.string_compose(val[0], val[1], val[2])
                        // result = @builder.dedent_string(string, @lexer.dedent_level)
                        $$ = Value::Node(Node::None);
                    }
                ;

         xstring: tXSTRING_BEG xstring_contents tSTRING_END
                    {
                        // string = @builder.xstring_compose(val[0], val[1], val[2])
                        // result = @builder.dedent_string(string, @lexer.dedent_level)
                        $$ = Value::Node(Node::None);
                    }
                ;

          regexp: tREGEXP_BEG regexp_contents tREGEXP_END
                    {
                        // opts   = @builder.regexp_options(val[3])
                        // result = @builder.regexp_compose(val[0], val[1], val[2], opts)
                        // TODO: handle regexp options
                        $$ = Value::Node(Node::None);
                    }
                ;

           words: tWORDS_BEG tSPACE word_list tSTRING_END
                    {
                        // result = @builder.words_compose(val[0], val[1], val[2])
                        $$ = Value::Node(Node::None);
                    }
                ;

       word_list: /* none */
                    {
                        $$ = Value::NodeList( vec![] );

                    }
                | word_list word tSPACE
                    {
                        let mut nodes = $<NodeList>1;
                        // nodes.push( builder.word( $<Node>2 ) );
                        $$ = Value::NodeList(nodes);
                    }
                ;

            word: string_content
                    {
                        $$ = Value::NodeList( vec![ $<Node>1 ] );
                    }
                | word string_content
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( $<Node>2 );
                        $$ = Value::NodeList(nodes);
                    }
                ;

         symbols: tSYMBOLS_BEG tSPACE symbol_list tSTRING_END
                    {
                        // result = @builder.symbols_compose(val[0], val[1], val[2])
                        $$ = Value::Node(Node::None);
                    }
                ;

     symbol_list: /* none */
                    {
                        $$ = Value::NodeList( vec![] );
                    }
                | symbol_list word tSPACE
                    {
                        let mut nodes = $<NodeList>1;
                        // nodes.push(builder.word( $<Token>2 ));
                        $$ = Value::NodeList( nodes );
                    }
                ;

          qwords: tQWORDS_BEG tSPACE qword_list tSTRING_END
                    {
                        // result = @builder.words_compose(val[0], val[1], val[2])
                        $$ = Value::Node(Node::None);
                    }
                ;

        qsymbols: tQSYMBOLS_BEG tSPACE qsym_list tSTRING_END
                    {
                        // result = @builder.symbols_compose(val[0], val[1], val[2])
                        $$ = Value::Node(Node::None);
                    }
                ;

      qword_list: /* none */
                    {
                        $$ = Value::NodeList( vec![] );
                    }
                | qword_list tSTRING_CONTENT tSPACE
                    {
                        let mut nodes = $<NodeList>1;
                        // nodes.push(builder.string_internal( $<Token>2 ));
                        $$ = Value::NodeList( nodes );
                    }
                ;

       qsym_list: /* none */
                    {
                        $$ = Value::NodeList( vec![] );
                    }
                | qsym_list tSTRING_CONTENT tSPACE
                    {
                        let mut nodes = $<NodeList>1;
                        // nodes.push(builder.symbol_internal( $<Token>2 ));
                        $$ = Value::NodeList( nodes );
                    }
                ;

 string_contents: /* none */
                    {
                        $$ = Value::NodeList( vec![] );
                    }
                | string_contents string_content
                    {
                        $$ = Value::NodeList( vec![] );
                    }
                ;

xstring_contents: /* none */
                    {
                        $$ = Value::NodeList( vec![] );
                    }
                | xstring_contents string_content
                    {
                        $$ = Value::NodeList( vec![] );
                    }
                ;

 regexp_contents: /* none */
                    {
                        $$ = Value::NodeList( vec![] );
                    }
                | regexp_contents string_content
                    {
                        let mut  nodes = $<NodeList>1;
                        nodes.push( $<Node>2 );
                        $$ = Value::NodeList( nodes );
                    }
                ;

  string_content: tSTRING_CONTENT
                    {
                        // result = @builder.string_internal(val[0])
                        $$ = Value::Node(Node::None);
                    }
                | tSTRING_DVAR
                    {
                        // TODO: push terminal
                    }
                  string_dvar
                    {
                        // result = val[1]
                        $$ = Value::Node(Node::None);
                    }
                | tSTRING_DBEG
                    {
                        // TODO: push terminal
                    }
                  compstmt tSTRING_DEND
                    {
                        $$ = Value::Node(Node::None);
                    }
                ;

     string_dvar: tGVAR
                    {
                        // result = @builder.gvar(val[0])
                        $$ = Value::Node(Node::None);
                    }
                | tIVAR
                    {
                        // result = @builder.ivar(val[0])
                        $$ = Value::Node(Node::None);

                    }
                | tCVAR
                    {
                        // result = @builder.cvar(val[0])
                        $$ = Value::Node(Node::None);
                    }
                | backref
                ;

          symbol: ssym
                | dsym
                ;

            ssym: tSYMBEG sym
                    {
                        // @lexer.state = :expr_end
                        // result = @builder.symbol(val[0])
                        $$ = Value::Node(Node::None);
                    }
                ;

             sym: fname
                | tIVAR
                | tGVAR
                | tCVAR
                ;

            dsym: tSYMBEG string_contents tSTRING_END
                    {
                        // @lexer.state = :expr_end
                        // result = @builder.symbol_compose(val[0], val[1], val[2])
                        $$ = Value::Node(Node::None);
                    }
                ;

         numeric: simple_numeric
                | tUMINUS_NUM simple_numeric   %prec tLOWEST
                    {
                        // result = @builder.unary_num(val[0], val[1])
                        $$ = Value::Node(Node::None);
                    }
                ;

  simple_numeric: tINTEGER
                    {
                        // @lexer.state = :expr_end
                        // result = @builder.integer(val[0])
                        $$ = Value::Node(Node::None);
                    }
                | tFLOAT
                    {
                        // @lexer.state = :expr_end
                        // result = @builder.float(val[0])
                        $$ = Value::Node(Node::None);
                    }
                | tRATIONAL
                    {
                        // @lexer.state = :expr_end
                        // result = @builder.rational(val[0])
                        $$ = Value::Node(Node::None);
                    }
                | tIMAGINARY
                    {
                        // @lexer.state = :expr_end
                        // result = @builder.complex(val[0])
                        $$ = Value::Node(Node::None);
                    }
                ;

   user_variable: tIDENTIFIER
                    {
                        // !! this rule returns different value types
                        // ugly, but this way we don't have Node::Ident
                        // result = @builder.ident(val[0])
                        $$ = Value::Ident($<Token>1);
                    }
                | tIVAR
                    {
                        // result = @builder.ivar(val[0])
                        $$ = Value::Node(Node::None);
                    }
                | tGVAR
                    {
                        // result = @builder.gvar(val[0])
                        $$ = Value::Node(Node::None);
                    }
                | tCONSTANT
                    {
                        // result = @builder.const(val[0])
                        $$ = Value::Node(Node::None);
                    }
                | tCVAR
                    {
                        // result = @builder.cvar(val[0])
                        $$ = Value::Node(Node::None);
                    }
                ;

keyword_variable: kNIL
                    {
                        // result = @builder.nil($<Token>1)
                        $$ = Value::Node(Node::None);
                    }
                | kSELF
                    {
                        // result = @builder.self($<Token>1)
                        $$ = Value::Node(Node::None);
                    }
                | kTRUE
                    {
                        // result = @builder.true($<Token>1)
                        $$ = Value::Node(Node::None);
                    }
                | kFALSE
                    {
                        // result = @builder.false($<Token>1)
                        $$ = Value::Node(Node::None);
                    }
                | k__FILE__
                    {
                        // result = @builder.__file__($<Token>1)
                        $$ = Value::Node(Node::None);
                    }
                | k__LINE__
                    {
                        // result = @builder.__line__($<Token>1)
                        $$ = Value::Node(Node::None);
                    }
                | k__ENCODING__
                    {
                        // result = @builder.__encoding__($<Token>1)
                        $$ = Value::Node(Node::None);
                    }
                ;

         var_ref: user_variable
                    {
                        // FIXME: error handling here is INSANE
                        // NOTE: user_variable can be Value::IDENT
                        // result = @builder.assignable(val[0])
                        $$ = Value::Node(Node::None);
                    }
                | keyword_variable
                    {
                        // result = @builder.assignable(val[0])
                        $$ = Value::Node(Node::None);
                    }
                ;

         var_lhs: user_variable
                    {
                        // NOTE: user_variable can be Value::IDENT
                        // result = @builder.assignable(val[0])
                        $$ = Value::Node(Node::None);
                    }
                | keyword_variable
                    {
                        // result = @builder.assignable(val[0])
                        $$ = Value::Node(Node::None);
                    }
                ;

         backref: tNTH_REF
                    {
                        // result = @builder.nth_ref(val[0])
                        $$ = Value::Node(Node::None);
                    }
                | tBACK_REF
                    {
                        // result = @builder.back_ref(val[0])
                        $$ = Value::Node(Node::None);
                    }
                ;

      superclass: tLT
                    {
                        // @lexer.state = :expr_value
                    }
                  expr_value term
                    {
                        let token = $<Token>1;
                        let node  = $<Node>3;
                        $$ = Value::TokenAndNode( (token, node) );
                    }
                | /* none */
                    {
                        $$ = Value::None;
                    }
                ;

    f_paren_args: tLPAREN2 f_args rparen
                    {
                        // result = @builder.args(val[0], val[1], val[2])
                        // @lexer.state = :expr_value

                        $$ = Value::Node(Node::None);
                    }
                | tLPAREN2 f_arg tCOMMA args_forward rparen
                    {
                        // args = [ *val[1], @builder.forward_arg(val[3]) ]
                        // result = @builder.args(val[0], args, val[4])
                        // @static_env.declare_forward_args

                        $$ = Value::Node(Node::None);
                    }
                | tLPAREN2 args_forward rparen
                    {
                        // result = @builder.forward_only_args(val[0], val[1], val[2])
                        // @static_env.declare_forward_args
                        // @lexer.state = :expr_value

                        $$ = Value::Node(Node::None);
                    }
                ;

       f_arglist: f_paren_args
                    {
                        // result = @lexer.in_kwarg
                        // @lexer.in_kwarg = true
                    }
                | f_args term
                    {
                        // @lexer.in_kwarg = val[0]
                        // result = @builder.args(nil, val[1], nil)
                        $$ = Value::Node(Node::None);
                    }
                ;

       args_tail: f_kwarg tCOMMA f_kwrest opt_f_block_arg
                    {
                        let nodes = [ $<NodeList>1, $<NodeList>3, $<NodeList>4 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | f_kwarg opt_f_block_arg
                    {
                        let nodes = [ $<NodeList>1, $<NodeList>2 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | f_any_kwrest opt_f_block_arg
                    {
                        let nodes = [ $<NodeList>1, $<NodeList>2 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | f_block_arg
                    {
                        $$ = Value::NodeList(vec![ $<Node>1 ]);
                    }
                ;

   opt_args_tail: tCOMMA args_tail
                    {
                        // $$ = $<RAW>2;
                        $$ = Value::NodeList( vec![] );
                    }
                | /* none */
                    {
                        $$ = Value::NodeList( vec![] );
                    }
                ;

          f_args: f_arg tCOMMA f_optarg tCOMMA f_rest_arg opt_args_tail
                    {
                        // println!("YYSTACK: {:#?}", yystack.value_stack);
                        let nodes = [ $<NodeList>1, $<NodeList>3, $<NodeList>5, $<NodeList>6 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | f_arg tCOMMA f_optarg tCOMMA f_rest_arg tCOMMA f_arg opt_args_tail
                    {
                        let nodes = [ $<NodeList>1, $<NodeList>3, $<NodeList>4, $<NodeList>7, $<NodeList>8 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | f_arg tCOMMA f_optarg opt_args_tail
                    {
                        let nodes = [ $<NodeList>1, $<NodeList>3, $<NodeList>4 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | f_arg tCOMMA f_optarg tCOMMA f_arg opt_args_tail
                    {
                        let nodes = [ $<NodeList>1, $<NodeList>3, $<NodeList>5, $<NodeList>6 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | f_arg tCOMMA f_rest_arg opt_args_tail
                    {
                        let nodes = [ $<NodeList>1, $<NodeList>3, $<NodeList>4 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | f_arg tCOMMA f_rest_arg tCOMMA f_arg opt_args_tail
                    {
                        let nodes = [ $<NodeList>1, $<NodeList>3, $<NodeList>5, $<NodeList>6 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | f_arg opt_args_tail
                    {
                        let nodes = [ $<NodeList>1, $<NodeList>2 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | f_optarg tCOMMA f_rest_arg opt_args_tail
                    {
                        let nodes = [ $<NodeList>1, $<NodeList>2, $<NodeList>3 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | f_optarg tCOMMA f_rest_arg tCOMMA f_arg opt_args_tail
                    {
                        let nodes = [ $<NodeList>1, $<NodeList>2, $<NodeList>3, $<NodeList>4 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | f_optarg opt_args_tail
                    {
                        let nodes = [ $<NodeList>1, $<NodeList>2 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | f_optarg tCOMMA f_arg opt_args_tail
                    {
                        let nodes = [ $<NodeList>1, $<NodeList>2, $<NodeList>3 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | f_rest_arg opt_args_tail
                    {
                        let nodes = [ $<NodeList>1, $<NodeList>2 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | f_rest_arg tCOMMA f_arg opt_args_tail
                    {
                        let nodes = [ $<NodeList>1, $<NodeList>2, $<NodeList>3 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | args_tail
                    {
                        let nodes = [ $<NodeList>1 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | /* none */
                    {
                        $$ = Value::NodeList( vec![] );
                    }
                ;

    args_forward: tBDOT3
                ;

       f_bad_arg: tCONSTANT
                    {
                        // diagnostic :error, :argument_const, nil, val[0]
                        $$ = $<RAW>1;
                    }
                | tIVAR
                    {
                        // diagnostic :error, :argument_const, nil, val[0]
                        $$ = $<RAW>1;
                    }
                | tGVAR
                    {
                        // diagnostic :error, :argument_const, nil, val[0]
                        $$ = $<RAW>1;
                    }
                | tCVAR
                    {
                        // diagnostic :error, :argument_const, nil, val[0]
                        $$ = $<RAW>1;
                    }
                ;

      f_norm_arg: f_bad_arg
                | tIDENTIFIER
                    {
                        // @static_env.declare val[0][0]
                        // @max_numparam_stack.has_ordinary_params!
                        $$ = $<RAW>1;
                    }
                ;

      f_arg_asgn: f_norm_arg
                    {
                        // @current_arg_stack.set(val[0][0])
                        $$ = $<RAW>1;
                    }
                ;

      f_arg_item: f_arg_asgn
                    {
                        // @current_arg_stack.set(0)
                        // result = @builder.arg(val[0])
                        $$ = Value::Node(Node::None);
                    }
                | tLPAREN f_margs rparen
                    {
                        // result = @builder.multi_lhs(val[0], val[1], val[2])
                        $$ = Value::Node(Node::None);
                    }
                ;

           f_arg: f_arg_item
                    {
                        $$ = Value::NodeList( vec![ $<Node>1 ] );
                    }
                | f_arg tCOMMA f_arg_item
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( $<Node>3 );
                        $$ = Value::NodeList(nodes);
                    }
                ;


         f_label: tLABEL
                    {
                        // check_kwarg_name(val[0])

                        // @static_env.declare val[0][0]

                        // @max_numparam_stack.has_ordinary_params!

                        // @current_arg_stack.set(val[0][0])

                        // result = val[0]

                        $$ = $<RAW>1;
                    }
                ;

            f_kw: f_label arg_value
                    {
                        // @current_arg_stack.set(nil)
                        // result = @builder.kwoptarg(val[0], val[1])
                        $$ = Value::Node(Node::None);
                    }
                | f_label
                    {
                        // @current_arg_stack.set(nil)
                        // result = @builder.kwarg(val[0])
                        $$ = Value::Node(Node::None);
                    }
                ;

      f_block_kw: f_label primary_value
                    {
                        // result = @builder.kwoptarg(val[0], val[1])
                        $$ = Value::Node(Node::None);
                    }
                | f_label
                    {
                        // result = @builder.kwarg(val[0])
                        $$ = Value::Node(Node::None);
                    }
                ;

   f_block_kwarg: f_block_kw
                    {
                        $$ = Value::NodeList( vec![ $<Node>1 ] );
                    }
                | f_block_kwarg tCOMMA f_block_kw
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( $<Node>3 );
                        $$ = Value::NodeList(nodes);
                    }
                ;


         f_kwarg: f_kw
                    {
                        $$ = Value::NodeList( vec![ $<Node>1 ] );
                    }
                | f_kwarg tCOMMA f_kw
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( $<Node>3 );
                        $$ = Value::NodeList(nodes);
                    }
                ;

     kwrest_mark: tPOW
                | tDSTAR
                ;

      f_no_kwarg: kwrest_mark kNIL
                    {
                        // result = [ @builder.kwnilarg(val[0], val[1]) ]
                        $$ = Value::NodeList( vec![ $<Node>1 ] );
                    }
                ;

        f_kwrest: kwrest_mark tIDENTIFIER
                    {
                        // @static_env.declare val[1][0]
                        // result = [ @builder.kwrestarg(val[0], val[1]) ]
                        $$ = Value::NodeList( vec![ $<Node>1 ] );
                    }
                | kwrest_mark
                    {
                        // result = [ @builder.kwrestarg(val[0]) ]
                        $$ = Value::NodeList( vec![ $<Node>1 ] );
                    }
                ;

           f_opt: f_arg_asgn tEQL arg_value
                    {
                        // @current_arg_stack.set(0)
                        // result = @builder.optarg(val[0], val[1], val[2])
                        $$ = Value::Node(Node::None);
                    }
                ;

     f_block_opt: f_arg_asgn tEQL primary_value
                    {
                        // @current_arg_stack.set(0)
                        // result = @builder.optarg(val[0], val[1], val[2])
                        $$ = Value::Node(Node::None);
                    }
                ;

  f_block_optarg: f_block_opt
                    {
                        $$ = Value::NodeList( vec![ $<Node>1 ] );
                    }
                | f_block_optarg tCOMMA f_block_opt
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( $<Node>3 );
                        $$ = Value::NodeList(nodes);
                    }
                ;

        f_optarg: f_opt
                    {
                        $$ = Value::NodeList( vec![ $<Node>1 ] );
                    }
                | f_optarg tCOMMA f_opt
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( $<Node>3 );
                        $$ = Value::NodeList(nodes);
                    }
                ;

    restarg_mark: tSTAR2
                | tSTAR
                ;

      f_rest_arg: restarg_mark tIDENTIFIER
                    {
                        // @static_env.declare val[1][0]
                        // result = [ @builder.restarg(val[0], val[1]) ]

                        $$ = Value::NodeList( vec![] );
                    }
                | restarg_mark
                    {
                        // result = [ @builder.restarg(val[0]) ]
                        $$ = Value::NodeList( vec![] );
                    }
                ;

     blkarg_mark: tAMPER2
                | tAMPER
                ;

     f_block_arg: blkarg_mark tIDENTIFIER
                    {
                        // @static_env.declare val[1][0]
                        // result = @builder.blockarg(val[0], val[1])
                        $$ = Value::Node(Node::None);
                    }
                ;

 opt_f_block_arg: tCOMMA f_block_arg
                    {
                        $$ = Value::NodeList( vec![ $<Node>2 ] )
                    }
                | none
                    {
                        $$ = Value::NodeList( vec![] );
                    }
                ;

       singleton: var_ref
                | tLPAREN2 expr rparen
                    {
                        $$ = $<RAW>1;
                    }
                ;

      assoc_list: none
                    {
                        $$ = Value::NodeList(vec![]);
                    }
                | assocs trailer
                ;

          assocs: assoc
                    {
                        $$ = Value::NodeList( vec![ $<Node>1 ] );
                    }
                | assocs tCOMMA assoc
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push($<Node>3);
                        $$ = Value::NodeList( nodes );
                    }
                ;

           assoc: arg_value tASSOC arg_value
                    {
                        // result = @builder.pair(val[0], val[1], val[2])
                        $$ = Value::Node(Node::None);
                    }
                | tLABEL arg_value
                    {
                        // result = @builder.pair_keyword(val[0], val[1])
                        $$ = Value::Node(Node::None);
                    }
                | tSTRING_BEG string_contents tLABEL_END arg_value
                    {
                        println!("self.builder.pair_quoted({:#?} {:#?} {:#?} {:#?})", $<Token>1, $<TokenList>2, $<Token>3, $<Node>4);
                        $$ = Value::Node(Node::None);
                    }
                | tDSTAR arg_value
                    {
                        println!("self.builder.kwsplat({:#?} {:#?})", $<RAW>1, $<RAW>2);
                        $$ = Value::Node(Node::None);
                    }
                ;

       operation: tIDENTIFIER
                | tCONSTANT
                | tFID
                ;

      operation2: tIDENTIFIER
                | tCONSTANT
                | tFID
                | op
                ;

      operation3: tIDENTIFIER
                | tFID
                | op
                ;

    dot_or_colon: tDOT
                | tCOLON2
                ;

         call_op: tDOT
                | tANDDOT
                ;

        call_op2: call_op
                | tCOLON2
                ;

       opt_terms: /* none */
                | terms
                ;

          opt_nl: /* none */
                | tNL
                ;

          rparen: opt_nl tRPAREN
                    {
                        $$ = $<RAW>1;
                    }
                ;

        rbracket: opt_nl tRBRACK
                    {
                        $$ = $<RAW>1;
                    }
                ;

          rbrace: opt_nl tRCURLY
                    {
                        $$ = $<RAW>1;
                    }
                ;

         trailer: /* none */
                | tNL
                | tCOMMA
                ;

            term: tSEMI
                | tNL
                ;

           terms: term
                    {
                        $$ = Value::TokenList(vec![]);
                    }
                | terms tSEMI
                    {
                        $$ = Value::TokenList(vec![]);
                    }
                ;

            none: /* empty */
                  {
                        $$ = Value::None;
                  }
                ;

%%

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    None,
}

#[derive(Clone, PartialEq)]
pub enum Value {
    Stolen,
    None,
    Token(Token),
    TokenList(Vec<Token>),
    Node(Node),
    NodeList(Vec<Node>),
    /* For superclass/opt_ensure/exc_var rule */
    TokenAndNode((Token, Node)),
    /* For user_variable rule */
    Ident(Token),
    /* For p_kw_label rule */
    PlainLabel(Token),
    /* For p_kw_label rule */
    QuotedLabel((Token, Vec<Node>, Token)),

    /* */
    Body((Token, Vec<Node>, Token)),
}

impl Value {
    pub fn from_token(token: Token) -> Self {
        Self::Token(token)
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { //'
        match self {
            Value::None => {
                f.write_str("Token::None")
            },
            Value::Stolen => {
                f.write_str("Token::Stolen")
            },
            Value::Token((token_type, token_value, loc)) => {
                f.write_fmt(format_args!("Token({}, {:?}, {:?})", token_type, token_value, loc))
            },
            Value::TokenList(tokens) => {
                f.write_fmt(format_args!("TokenList({:?})", tokens))
            },
            Value::Node(node) => {
                f.write_fmt(format_args!("Node({:?})", node))
            },
            Value::NodeList(nodes) => {
                f.write_fmt(format_args!("NodeList({:?})", nodes))
            },
            Value::TokenAndNode((token, node)) => {
                f.write_fmt(format_args!("TokenAndNode({:?}, {:?})", token, node))
            },
            Value::Ident(token) => {
                f.write_fmt(format_args!("Ident({:?})", token))
            },
            Value::PlainLabel(token) => {
                f.write_fmt(format_args!("PlainLabel({:?})", token))
            },
            Value::QuotedLabel((start, tokens, end)) => {
                f.write_fmt(format_args!("QuotedLabel({:?}, {:?}, {:?})", start, tokens, end))
            },
            Value::Body((start, nodes, end)) => {
                f.write_fmt(format_args!("Body({:?}, {:?}, {:?})", start, nodes, end))
            },
        }
    }
}

impl Parser {
  pub fn do_parse(mut self) -> Option<String> {
      self.parse();
      self.result
  }
}

impl Lexer {
    fn report_syntax_error(&self, ctx: &Context) {
        eprintln!("{:#?}", ctx)
    }

    fn yyerror(&mut self, loc: &Loc, msg: &str) {
        eprintln!("{:#?} {:#?}", loc, msg)
    }
}

#[allow(non_upper_case_globals)]
impl Lexer {
    // Dummy tokens to satisfy tests for now
    pub const tSTRING: i32 = 1_000;
    pub const tSYMBOL: i32 = 1_001;
    pub const tUNARY_NUM: i32 = 1_002;
    pub const tREGEXP_OPT: i32 = 1_003;
    pub const tCHARACTER: i32 = 1_004;
}
