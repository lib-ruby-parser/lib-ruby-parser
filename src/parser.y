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

%type <node> singleton strings string string1 xstring regexp
%type <node> string_contents xstring_contents regexp_contents string_content
%type <node> words symbols symbol_list qwords qsymbols word_list qword_list qsym_list word
%type <node> literal numeric simple_numeric ssym dsym symbol cpath def_name defn_head defs_head
%type <node> top_compstmt top_stmts top_stmt begin_block rassign
%type <node> bodystmt compstmt stmts stmt_or_begin stmt expr arg primary command command_call method_call
%type <node> expr_value expr_value_do arg_value primary_value fcall rel_expr
%type <node> if_tail opt_else case_body case_args cases opt_rescue exc_list exc_var opt_ensure
%type <node> args call_args opt_call_args
%type <node> paren_args opt_paren_args block_args_tail opt_block_args_tail
%type <node> command_args aref_args opt_block_arg block_arg var_ref var_lhs
%type <node> command_rhs arg_rhs
%type <node> command_asgn mrhs mrhs_arg block_call block_command
%type <node> f_block_opt
%type <node> f_arglist f_paren_args f_arg_item f_marg f_marg_list f_margs f_rest_marg
%type <node> assoc undef_list backref string_dvar for_var
%type <node> block_param opt_block_param block_param_def f_opt
%type <node> f_kw f_block_kw
%type <node> bv_decls opt_bv_decl bvar
%type <node> lambda f_larglist lambda_body brace_body do_body
%type <node> brace_block cmd_brace_block do_block lhs none fitem
%type <node> mlhs mlhs_head mlhs_basic mlhs_item mlhs_node mlhs_post mlhs_inner
%type <node> p_case_body p_cases p_top_expr p_top_expr_body
%type <node> p_expr p_as p_alt p_expr_basic p_find
%type <node> p_args p_args_head p_args_tail p_args_post p_arg
%type <node> p_value p_primitive p_variable p_var_ref p_const
%type <node> p_kwargs p_kwarg p_kw
%type <node> f_block_arg keyword_variable user_variable

%type <node_list> assocs assoc_list opt_f_block_arg f_rest_arg f_optarg f_args
%type <node_list> f_block_optarg f_kwrest f_no_kwarg f_kwarg f_block_kwarg f_arg
%type <node_list> opt_args_tail args_tail

%type <token_and_node> superclass

%type <token>   sym operation operation2 operation3
%type <token>   cname fname op f_norm_arg f_bad_arg
%type <token>   f_label f_arg_asgn call_op call_op2 reswords relop dot_or_colon
%type <token>   p_rest p_kwrest p_kwnorest p_any_kwrest p_kw_label
%type <token>   f_any_kwrest args_forward excessed_comma
%type <token>   rbrace rparen rbracket

%type <token_list> terms

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

                    }
                | compstmt
                  opt_rescue
                  opt_ensure
                    {

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

                    }
                | k_begin
                    {

                    }
                  bodystmt
                  k_end
                    {

                    }
                | tLPAREN_ARG {} rparen
                    {

                    }
                | tLPAREN_ARG stmt {} rparen
                    {

                    }
                | tLPAREN compstmt tRPAREN
                    {

                    }
                | primary_value tCOLON2 tCONSTANT
                    {

                    }
                | tCOLON3 tCONSTANT
                    {

                    }
                | tLBRACK aref_args tRBRACK
                    {

                    }
                | tLBRACE assoc_list tRCURLY
                    {

                    }
                | k_return
                    {

                    }
                | kYIELD tLPAREN2 call_args rparen
                    {

                    }
                | kYIELD tLPAREN2 rparen
                    {

                    }
                | kYIELD
                    {

                    }
                | kDEFINED opt_nl tLPAREN2 {} expr rparen
                    {

                    }
                | kNOT tLPAREN2 expr rparen
                    {

                    }
                | kNOT tLPAREN2 rparen
                    {

                    }
                | fcall brace_block
                    {

                    }
                | method_call
                | method_call brace_block
                    {

                    }
                | lambda
                    {

                    }
                | k_if expr_value then
                  compstmt
                  if_tail
                  k_end
                    {

                    }
                | k_unless expr_value then
                  compstmt
                  opt_else
                  k_end
                    {

                    }
                | k_while expr_value_do
                  compstmt
                  k_end
                    {

                    }
                | k_until expr_value_do
                  compstmt
                  k_end
                    {

                    }
                | k_case expr_value opt_terms
                    {

                    }
                  case_body
                  k_end
                    {

                    }
                | k_case opt_terms
                    {

                    }
                  case_body
                  k_end
                    {

                    }
                | k_case expr_value opt_terms
                  p_case_body
                  k_end
                    {

                    }
                | k_for for_var kIN expr_value_do
                  compstmt
                  k_end
                    {

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
                    }
                | k_class tLSHFT expr
                    {

                    }
                  term
                  bodystmt
                  k_end
                    {

                    }
                | k_module cpath
                    {

                    }
                  bodystmt
                  k_end
                    {

                    }
                | defn_head
                  f_arglist
                  bodystmt
                  k_end
                    {

                    }
                | defs_head
                  f_arglist
                  bodystmt
                  k_end
                    {

                    }
                | kBREAK
                    {

                    }
                | kNEXT
                    {

                    }
                | kREDO
                    {

                    }
                | kRETRY
                    {

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
                ;

            then: term
                | kTHEN
                | term kTHEN
                ;

              do: term
                | kDO_COND
                ;

         if_tail: opt_else
                | k_elsif expr_value then
                  compstmt
                  if_tail
                    {

                    }
                ;

        opt_else: none
                | k_else compstmt
                    {

                    }
                ;

         for_var: lhs
                | mlhs
                ;

          f_marg: f_norm_arg
                    {

                    }
                | tLPAREN f_margs rparen
                    {

                    }
                ;

     f_marg_list: f_marg
                | f_marg_list tCOMMA f_marg
                ;

         f_margs: f_marg_list
                | f_marg_list tCOMMA f_rest_marg
                | f_marg_list tCOMMA f_rest_marg tCOMMA f_marg_list
                | f_rest_marg
                | f_rest_marg tCOMMA f_marg_list
                ;

     f_rest_marg: tSTAR f_norm_arg
                    {

                    }
                | tSTAR
                    {

                    }
                ;

    f_any_kwrest: f_kwrest
                | f_no_kwarg
                ;

 block_args_tail: f_block_kwarg tCOMMA f_kwrest opt_f_block_arg
                | f_block_kwarg opt_f_block_arg
                | f_any_kwrest opt_f_block_arg
                    {

                    }
                | f_block_arg
                    {

                    }
                ;

opt_block_args_tail:
                  tCOMMA block_args_tail
                    {

                    }
                | /* none */
                    {

                    }
                ;

  excessed_comma: tCOMMA
                ;

     block_param: f_arg tCOMMA f_block_optarg tCOMMA f_rest_arg opt_block_args_tail
                | f_arg tCOMMA f_block_optarg tCOMMA f_rest_arg tCOMMA f_arg opt_block_args_tail
                | f_arg tCOMMA f_block_optarg opt_block_args_tail
                | f_arg tCOMMA f_block_optarg tCOMMA f_arg opt_block_args_tail
                | f_arg tCOMMA f_rest_arg opt_block_args_tail
                | f_arg excessed_comma
                | f_arg tCOMMA f_rest_arg tCOMMA f_arg opt_block_args_tail
                | f_arg opt_block_args_tail
                | f_block_optarg tCOMMA f_rest_arg opt_block_args_tail
                | f_block_optarg tCOMMA f_rest_arg tCOMMA f_arg opt_block_args_tail
                | f_block_optarg opt_block_args_tail
                | f_block_optarg tCOMMA f_arg opt_block_args_tail
                | f_rest_arg opt_block_args_tail
                    {

                    }
                | f_rest_arg tCOMMA f_arg opt_block_args_tail
                    {

                    }
                | block_args_tail
                ;

 opt_block_param: none
                | block_param_def
                ;

 block_param_def: tPIPE opt_bv_decl tPIPE
                    {

                    }
                | tPIPE block_param opt_bv_decl tPIPE
                    {

                    }
                ;


     opt_bv_decl: opt_nl
                    {

                    }
                | opt_nl tSEMI bv_decls opt_nl
                    {

                    }
                ;

        bv_decls: bvar
                | bv_decls tCOMMA bvar
                ;

            bvar: tIDENTIFIER
                    {

                    }
                | f_bad_arg
                    {

                    }
                ;

          lambda: tLAMBDA
                  f_larglist
                  lambda_body
                    {

                    }
                ;

      f_larglist: tLPAREN2 f_args opt_bv_decl tRPAREN
                    {

                    }
                | f_args
                    {

                    }
                ;

     lambda_body: tLAMBEG compstmt tRCURLY
                    {

                    }
                | kDO_LAMBDA bodystmt k_end
                    {

                    }
                ;

        do_block: k_do_block do_body k_end
                    {

                    }
                ;

      block_call: command do_block
                | block_call call_op2 operation2 opt_paren_args
                | block_call call_op2 operation2 opt_paren_args brace_block
                | block_call call_op2 operation2 command_args do_block
                ;

     method_call: fcall paren_args
                | primary_value call_op operation2 opt_paren_args
                | primary_value tCOLON2 operation2 paren_args
                | primary_value tCOLON2 operation3
                | primary_value call_op paren_args
                | primary_value tCOLON2 paren_args
                | kSUPER paren_args
                    {

                    }
                | kSUPER
                    {

                    }
                | primary_value tLBRACK2 opt_call_args rbracket
                ;

     brace_block: tLCURLY brace_body tRCURLY
                    {

                    }
                | k_do do_body k_end
                    {

                    }
                ;

      brace_body: opt_block_param compstmt
                ;

         do_body: opt_block_param bodystmt
                ;

       case_args: arg_value
                | tSTAR arg_value
                    {

                    }
                | case_args tCOMMA arg_value
                | case_args tCOMMA tSTAR arg_value
                ;

       case_body: k_when case_args then
                  compstmt
                  cases
                    {

                    }
                ;

           cases: opt_else
                | case_body
                ;

     p_case_body: kIN
                  p_top_expr then
                  compstmt
                  p_cases
                    {

                    }
                ;

         p_cases: opt_else
                | p_case_body
                ;

      p_top_expr: p_top_expr_body
                | p_top_expr_body kIF_MOD expr_value
                | p_top_expr_body kUNLESS_MOD expr_value
                ;

 p_top_expr_body: p_expr
                | p_expr tCOMMA
                | p_expr tCOMMA p_args
                | p_find
                | p_args_tail
                | p_kwargs
                ;

          p_expr: p_as
                ;

            p_as: p_expr tASSOC p_variable
                | p_alt
                ;

           p_alt: p_alt tPIPE p_expr_basic
                | p_expr_basic
                ;

        p_lparen: tLPAREN2
                ;

      p_lbracket: tLBRACK2
                ;

    p_expr_basic: p_value
                | p_const p_lparen p_args rparen
                | p_const p_lparen p_find rparen
                | p_const p_lparen p_kwargs rparen
                | p_const tLPAREN2 rparen
                | p_const p_lbracket p_args rbracket
                | p_const p_lbracket p_find rbracket
                | p_const p_lbracket p_kwargs rbracket
                | p_const tLBRACK2 rbracket
                | tLBRACK p_args rbracket
                    {

                    }
                | tLBRACK p_find rbracket
                    {

                    }
                | tLBRACK rbracket
                    {

                    }
                | tLBRACE
                  p_kwargs rbrace
                    {

                    }
                | tLBRACE rbrace
                    {

                    }
                | tLPAREN p_expr rparen
                    {

                    }
                ;

          p_args: p_expr
                | p_args_head
                | p_args_head p_arg
                | p_args_head tSTAR tIDENTIFIER
                | p_args_head tSTAR tIDENTIFIER tCOMMA p_args_post
                | p_args_head tSTAR
                | p_args_head tSTAR tCOMMA p_args_post
                | p_args_tail
                ;

     p_args_head: p_arg tCOMMA
                | p_args_head p_arg tCOMMA
                ;

     p_args_tail: p_rest
                    {

                    }
                | p_rest tCOMMA p_args_post
                    {

                    }
                ;

          p_find: p_rest tCOMMA p_args_post tCOMMA p_rest
                    {

                    }
                ;


          p_rest: tSTAR tIDENTIFIER
                | tSTAR
                ;

     p_args_post: p_arg
                | p_args_post tCOMMA p_arg
                ;

           p_arg: p_expr
                ;

        p_kwargs: p_kwarg tCOMMA p_any_kwrest
                | p_kwarg
                | p_kwarg tCOMMA
                | p_any_kwrest
                    {

                    }
                ;

         p_kwarg: p_kw
                | p_kwarg tCOMMA p_kw
                ;

            p_kw: p_kw_label p_expr
                    {

                    }
                | p_kw_label
                    {

                    }
                ;

      p_kw_label: tLABEL
                | tSTRING_BEG string_contents tLABEL_END
                ;

        p_kwrest: kwrest_mark tIDENTIFIER
                    {

                    }
                | kwrest_mark
                    {

                    }
                ;

      p_kwnorest: kwrest_mark kNIL
                    {

                    }
                ;

    p_any_kwrest: p_kwrest
                | p_kwnorest
                ;

         p_value: p_primitive
                | p_primitive tDOT2 p_primitive
                | p_primitive tDOT3 p_primitive
                | p_primitive tDOT2
                | p_primitive tDOT3
                | p_variable
                | p_var_ref
                | p_const
                | tBDOT2 p_primitive
                    {

                    }
                | tBDOT3 p_primitive
                    {

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

                    }
                | lambda
                ;

      p_variable: tIDENTIFIER
                    {

                    }
                ;

       p_var_ref: tCARET tIDENTIFIER
                    {

                    }
                ;

         p_const: tCOLON3 cname
                    {

                    }
                | p_const tCOLON2 cname
                | tCONSTANT
                    {

                    }
                ;

      opt_rescue: k_rescue exc_list exc_var then
                    {

                    }
                  compstmt
                  opt_rescue
                    {

                    }
                | none
                ;

        exc_list: arg_value
                | mrhs
                | none
                ;

         exc_var: tASSOC lhs
                    {

                    }
                | none
                ;

      opt_ensure: k_ensure compstmt
                    {

                    }
                | none
                ;

         literal: numeric
                | symbol
                ;

         strings: string
                ;

          string: tCHAR
                | string1
                | string string1
                ;

         string1: tSTRING_BEG string_contents tSTRING_END
                    {

                    }
                ;

         xstring: tXSTRING_BEG xstring_contents tSTRING_END
                    {

                    }
                ;

          regexp: tREGEXP_BEG regexp_contents tREGEXP_END
                    {

                    }
                ;

           words: tWORDS_BEG tSPACE word_list tSTRING_END
                    {

                    }
                ;

       word_list: /* none */
                    {

                    }
                | word_list word tSPACE
                ;

            word: string_content
                | word string_content
                ;

         symbols: tSYMBOLS_BEG tSPACE symbol_list tSTRING_END
                    {

                    }
                ;

     symbol_list: /* none */
                    {

                    }
                | symbol_list word tSPACE
                ;

          qwords: tQWORDS_BEG tSPACE qword_list tSTRING_END
                    {

                    }
                ;

        qsymbols: tQSYMBOLS_BEG tSPACE qsym_list tSTRING_END
                    {

                    }
                ;

      qword_list: /* none */
                    {

                    }
                | qword_list tSTRING_CONTENT tSPACE
                ;

       qsym_list: /* none */
                    {

                    }
                | qsym_list tSTRING_CONTENT tSPACE
                ;

 string_contents: /* none */
                    {

                    }
                | string_contents string_content
                ;

xstring_contents: /* none */
                    {

                    }
                | xstring_contents string_content
                ;

 regexp_contents: /* none */
                    {

                    }
                | regexp_contents string_content
                ;

  string_content: tSTRING_CONTENT
                | tSTRING_DVAR
                  string_dvar
                    {

                    }
                | tSTRING_DBEG
                    {

                    }
                  compstmt tSTRING_DEND
                    {

                    }
                ;

     string_dvar: tGVAR
                    {

                    }
                | tIVAR
                    {

                    }
                | tCVAR
                    {

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
    /* For superclass rule */
    TokenAndNode((Token, Node)),
    /* For user_variable rule */
    Ident(Token),
}

impl Value {
    pub fn from_token(token: Token) -> Self {
        Self::Token(token)
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { //'
        match self {
            Value::None => f.write_str("Token::None"),
            Value::Stolen => f.write_str("Token::Stolen"),
            Value::Token((token_type, token_value, loc)) => {
                f.write_fmt(format_args!("Token({}, {:?}, {:?})", token_type, token_value, loc))
            },
            Value::TokenList(tokens) => {
                f.write_fmt(format_args!("TokenList({:?})", tokens))
            },
            Value::Node(node) => f.write_fmt(format_args!("Node({:?})", node)),
            Value::NodeList(nodes) => {
                f.write_fmt(format_args!("NodeList({:?})", nodes))
            },
            Value::TokenAndNode((token, node)) => {
                f.write_fmt(format_args!("TokenAndNode({:?}, {:?})", token, node))
            },
            Value::Ident(token) => {
                f.write_fmt(format_args!("Ident({:?})", token))
            }
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
