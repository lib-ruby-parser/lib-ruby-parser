%expect 0

%define api.parser.struct { Parser }
%define api.location.type Loc
%define api.value.type { Value }
%define api.parser.result_type { String }

%define parse.error custom
%define parse.trace


%code use {
  use crate::Lexer;
}

%code {
  // code
}


/* Bison Declarations */
%token <id>
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

%token <id>   tIDENTIFIER     "local variable or method"
%token <id>   tFID            "method"
%token <id>   tGVAR           "global variable"
%token <id>   tIVAR           "instance variable"
%token <id>   tCONSTANT       "constant"
%token <id>   tCVAR           "class variable"
%token <id>   tLABEL          "label"
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
%type <node> paren_args opt_paren_args args_tail opt_args_tail block_args_tail opt_block_args_tail
%type <node> command_args aref_args opt_block_arg block_arg var_ref var_lhs
%type <node> command_rhs arg_rhs
%type <node> command_asgn mrhs mrhs_arg superclass block_call block_command
%type <node> f_block_optarg f_block_opt
%type <node> f_arglist f_paren_args f_args f_arg f_arg_item f_optarg f_marg f_marg_list f_margs f_rest_marg
%type <node> assoc_list assocs assoc undef_list backref string_dvar for_var
%type <node> block_param opt_block_param block_param_def f_opt
%type <node> f_kwarg f_kw f_block_kwarg f_block_kw
%type <node> bv_decls opt_bv_decl bvar
%type <node> lambda f_larglist lambda_body brace_body do_body
%type <node> brace_block cmd_brace_block do_block lhs none fitem
%type <node> mlhs mlhs_head mlhs_basic mlhs_item mlhs_node mlhs_post mlhs_inner
%type <node> p_case_body p_cases p_top_expr p_top_expr_body
%type <node> p_expr p_as p_alt p_expr_basic p_find
%type <node> p_args p_args_head p_args_tail p_args_post p_arg
%type <node> p_value p_primitive p_variable p_var_ref p_const
%type <node> p_kwargs p_kwarg p_kw
%type <id>   keyword_variable user_variable sym operation operation2 operation3
%type <id>   cname fname op f_rest_arg f_block_arg opt_f_block_arg f_norm_arg f_bad_arg
%type <id>   f_kwrest f_label f_arg_asgn call_op call_op2 reswords relop dot_or_colon
%type <id>   p_rest p_kwrest p_kwnorest p_any_kwrest p_kw_label
%type <id>   f_no_kwarg f_any_kwrest args_forward excessed_comma
%token END_OF_INPUT 0   "end-of-input"
%token <id> tDOT
/* escaped chars, should be ignored otherwise */
%token <id> '\\'        "backslash"
%token tSP              "escaped space"
%token <id> '\t'        "escaped horizontal tab"
%token <id> '\f'        "escaped form feed"
%token <id> '\r'        "escaped carriage return"
%token <id> '\13'       "escaped vertical tab"
%token <id> tUPLUS           "unary+"
%token <id> tUMINUS          "unary-"
%token <id> tPOW             "**"
%token <id> tCMP        "<=>"
%token <id> tEQ         "=="
%token <id> tEQQ        "==="
%token <id> tNEQ        "!="
%token <id> tGEQ        ">="
%token <id> tLEQ        "<="
%token <id> tANDOP           "&&"
%token <id> tOROP            "||"
%token <id> tMATCH      "=~"
%token <id> tNMATCH     "!~"
%token <id> tDOT2            ".."
%token <id> tDOT3            "..."
%token <id> tBDOT2           "(.."
%token <id> tBDOT3           "(..."
%token <id> tAREF            "[]"
%token <id> tASET            "[]="
%token <id> tLSHFT      "<<"
%token <id> tRSHFT      ">>"
%token <id> tANDDOT     "&."
%token <id> tCOLON2     "::"
%token <id> tCOLON3          ":: at EXPR_BEG"
%token <id> tOP_ASGN    "operator-assignment" /* +=, -=  etc. */
%token <id> tASSOC           "=>"
%token <id> tLPAREN          "("
%token <id> tLPAREN_ARG      "( arg"
%token <id> tRPAREN          ")"
%token <id> tLBRACK          "["
%token <id> tLBRACE          "{"
%token <id> tLBRACE_ARG      "{ arg"
%token <id> tSTAR            "*"
%token <id> tDSTAR           "**arg"
%token <id> tAMPER           "&"
%token <id> tLAMBDA          "->"
%token <id> tSYMBEG          "symbol literal"
%token <id> tSTRING_BEG      "string literal"
%token <id> tXSTRING_BEG     "backtick literal"
%token <id> tREGEXP_BEG      "regexp literal"
%token <id> tWORDS_BEG       "word list"
%token <id> tQWORDS_BEG      "verbatim word list"
%token <id> tSYMBOLS_BEG     "symbol list"
%token <id> tQSYMBOLS_BEG    "verbatim symbol list"
%token <id> tSTRING_END      "terminator"
%token <id> tSTRING_DEND     "tRCURLY"
%token <id> tSTRING_DBEG
%token <id> tSTRING_DVAR
%token <id> tLAMBEG
%token <id> tLABEL_END

%token <id> tCOMMA           ","
%token <id> tLCURLY          "{ (tLCURLY)"
%token <id> tRCURLY          "}"
%token <id> tLBRACK2         "[ (tLBRACK2)"
%token <id> tEQL             "="
%token <id> tPIPE            "|"
%token <id> tAMPER2          "& (tAMPER2)"
%token <id> tGT              ">"
%token <id> tLT              "<"
%token <id> tBACK_REF2       "`"
%token <id> tCARET           "^"
%token <id> tLPAREN2         "( (tLPAREN2)"
%token <id> tRBRACK          "]"
%token <id> tSEMI            ";"
%token <id> tSPACE            " "
%token <id> tNL              "\n"
%token <id> tPLUS            "+"
%token <id> tMINUS           "-"
%token <id> tSTAR2           "* (tSTAR2)"
%token <id> tDIVIDE          "/"
%token <id> tPERCENT         "%"
%token <id> tTILDE           "~"
%token <id> tBANG            "!"

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

              op: tPIPE      { $$ = $1.clone(); }
                | tCARET     { $$ = $1.clone(); }
                | tAMPER2    { $$ = $1.clone(); }
                | tCMP       { $$ = $1.clone(); }
                | tEQ        { $$ = $1.clone(); }
                | tEQQ       { $$ = $1.clone(); }
                | tMATCH     { $$ = $1.clone(); }
                | tNMATCH    { $$ = $1.clone(); }
                | tGT        { $$ = $1.clone(); }
                | tGEQ       { $$ = $1.clone(); }
                | tLT        { $$ = $1.clone(); }
                | tLEQ       { $$ = $1.clone(); }
                | tNEQ       { $$ = $1.clone(); }
                | tLSHFT     { $$ = $1.clone(); }
                | tRSHFT     { $$ = $1.clone(); }
                | tPLUS      { $$ = $1.clone(); }
                | tMINUS     { $$ = $1.clone(); }
                | tSTAR2     { $$ = $1.clone(); }
                | tSTAR      { $$ = $1.clone(); }
                | tDIVIDE    { $$ = $1.clone(); }
                | tPERCENT   { $$ = $1.clone(); }
                | tPOW       { $$ = $1.clone(); }
                | tDSTAR     { $$ = $1.clone(); }
                | tBANG      { $$ = $1.clone(); }
                | tTILDE     { $$ = $1.clone(); }
                | tUPLUS     { $$ = $1.clone(); }
                | tUMINUS    { $$ = $1.clone(); }
                | tAREF      { $$ = $1.clone(); }
                | tASET      { $$ = $1.clone(); }
                | tBACK_REF2 { $$ = $1.clone(); }
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

                    }
                  bodystmt
                  k_end
                    {

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

                    }
                ;

             sym: fname
                | tIVAR
                | tGVAR
                | tCVAR
                ;

            dsym: tSYMBEG string_contents tSTRING_END
                    {

                    }
                ;

         numeric: simple_numeric
                | tUMINUS_NUM simple_numeric   %prec tLOWEST
                    {

                    }
                ;

  simple_numeric: tINTEGER
                | tFLOAT
                | tRATIONAL
                | tIMAGINARY
                ;

   user_variable: tIDENTIFIER
                | tIVAR
                | tGVAR
                | tCONSTANT
                | tCVAR
                ;

keyword_variable: kNIL
                | kSELF
                | kTRUE
                | kFALSE
                | k__FILE__
                | k__LINE__
                | k__ENCODING__
                ;

         var_ref: user_variable
                    {

                    }
                | keyword_variable
                    {

                    }
                ;

         var_lhs: user_variable
                    {

                    }
                | keyword_variable
                    {

                    }
                ;

         backref: tNTH_REF
                | tBACK_REF
                ;

      superclass: tLT
                    {

                    }
                  expr_value term
                    {

                    }
                | /* none */
                    {

                    }
                ;

    f_paren_args: tLPAREN2 f_args rparen
                    {

                    }
                | tLPAREN2 f_arg tCOMMA args_forward rparen
                    {

                    }
                | tLPAREN2 args_forward rparen
                    {

                    }
                ;

       f_arglist: f_paren_args
                | f_args term
                ;

       args_tail: f_kwarg tCOMMA f_kwrest opt_f_block_arg
                | f_kwarg opt_f_block_arg
                | f_any_kwrest opt_f_block_arg
                    {

                    }
                | f_block_arg
                    {

                    }
                ;

   opt_args_tail: tCOMMA args_tail
                    {

                    }
                | /* none */
                    {

                    }
                ;

          f_args: f_arg tCOMMA f_optarg tCOMMA f_rest_arg opt_args_tail
                | f_arg tCOMMA f_optarg tCOMMA f_rest_arg tCOMMA f_arg opt_args_tail
                | f_arg tCOMMA f_optarg opt_args_tail
                | f_arg tCOMMA f_optarg tCOMMA f_arg opt_args_tail
                | f_arg tCOMMA f_rest_arg opt_args_tail
                | f_arg tCOMMA f_rest_arg tCOMMA f_arg opt_args_tail
                | f_arg opt_args_tail
                | f_optarg tCOMMA f_rest_arg opt_args_tail
                | f_optarg tCOMMA f_rest_arg tCOMMA f_arg opt_args_tail
                | f_optarg opt_args_tail
                | f_optarg tCOMMA f_arg opt_args_tail
                | f_rest_arg opt_args_tail
                    {

                    }
                | f_rest_arg tCOMMA f_arg opt_args_tail
                    {

                    }
                | args_tail
                | /* none */
                    {

                    }
                ;

    args_forward: tBDOT3
                ;

       f_bad_arg: tCONSTANT
                | tIVAR
                | tGVAR
                | tCVAR
                ;

      f_norm_arg: f_bad_arg
                | tIDENTIFIER
                ;

      f_arg_asgn: f_norm_arg
                ;

      f_arg_item: f_arg_asgn
                    {

                    }
                | tLPAREN f_margs rparen
                    {

                    }
                ;

           f_arg: f_arg_item
                | f_arg tCOMMA f_arg_item
                ;


         f_label: tLABEL
                ;

            f_kw: f_label arg_value
                    {

                    }
                | f_label
                    {

                    }
                ;

      f_block_kw: f_label primary_value
                    {

                    }
                | f_label
                    {

                    }
                ;

   f_block_kwarg: f_block_kw
                | f_block_kwarg tCOMMA f_block_kw
                ;


         f_kwarg: f_kw
                | f_kwarg tCOMMA f_kw
                ;

     kwrest_mark: tPOW
                | tDSTAR
                ;

      f_no_kwarg: kwrest_mark kNIL
                    {

                    }
                ;

        f_kwrest: kwrest_mark tIDENTIFIER
                    {

                    }
                | kwrest_mark
                    {

                    }
                ;

           f_opt: f_arg_asgn tEQL arg_value
                    {

                    }
                ;

     f_block_opt: f_arg_asgn tEQL primary_value
                    {

                    }
                ;

  f_block_optarg: f_block_opt
                | f_block_optarg tCOMMA f_block_opt
                ;

        f_optarg: f_opt
                | f_optarg tCOMMA f_opt
                ;

    restarg_mark: tSTAR2
                | tSTAR
                ;

      f_rest_arg: restarg_mark tIDENTIFIER
                    {

                    }
                | restarg_mark
                    {

                    }
                ;

     blkarg_mark: tAMPER2
                | tAMPER
                ;

     f_block_arg: blkarg_mark tIDENTIFIER
                    {

                    }
                ;

 opt_f_block_arg: tCOMMA f_block_arg
                    {

                    }
                | none
                    {

                    }
                ;

       singleton: var_ref
                | tLPAREN2 expr rparen
                    {

                    }
                ;

      assoc_list: none
                | assocs trailer
                ;

          assocs: assoc
                | assocs tCOMMA assoc
                ;

           assoc: arg_value tASSOC arg_value
                | tLABEL arg_value
                    {

                    }
                | tSTRING_BEG string_contents tLABEL_END arg_value
                    {

                    }
                | tDSTAR arg_value
                    {

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
                ;

        rbracket: opt_nl tRBRACK
                ;

          rbrace: opt_nl tRCURLY
                ;

         trailer: /* none */
                | tNL
                | tCOMMA
                ;

            term: tSEMI
                | tNL
                ;

           terms: term
                | terms tSEMI
                ;

            none: /* empty */
                ;

%%


#[derive(Clone, PartialEq)]
pub enum Value {
    None,
    Token(Token),
    Expr(String)
}

impl Value {
    pub fn from_token(token: Token) -> Self {
        Self::Token(token)
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { //'
        match self {
            Value::None => f.write_str("<No Token>"),
            Value::Token((token_type, token_value, loc)) => {
              f.write_fmt(format_args!("Token({}, {:?}, {:?})", token_type, token_value, loc))
            },
            Value::Expr(expr) => f.write_fmt(format_args!("Expr({})", expr))
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
