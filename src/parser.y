%expect 0

%define api.parser.struct { Parser }
%define api.location.type { Loc }
%define api.value.type { Value }

%define parse.error custom
%define parse.trace

%code parser_fields {
    result: Option<Node>,
    builder: Builder,
    current_arg_stack: CurrentArgStack,
    pub static_env: StaticEnvironment,
}

%code use {
    use crate::{Lexer, Builder, CurrentArgStack, StaticEnvironment};
    use crate::lexer::lex_states::*;
    use crate::lexer::{ContextItem};
    use crate::builder::{LoopType, KeywordCmd, LogicalOp, PKwLabel};
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
%type <node> literal numeric simple_numeric ssym dsym symbol cpath
%type <node> top_compstmt top_stmt rassign
%type <node> stmt_or_begin stmt expr arg primary command command_call method_call
%type <node> expr_value arg_value primary_value rel_expr
%type <node> block_arg var_ref
%type <node> command_rhs arg_rhs
%type <node> command_asgn mrhs_arg block_call block_command
%type <node> f_block_opt
%type <node> f_arg_item f_marg f_rest_marg
%type <node> assoc backref string_dvar
%type <node> f_opt
%type <node> f_kw f_block_kw
%type <node> bvar
%type <node> lambda
%type <node> fitem
%type <node> p_top_expr_body
%type <node> p_expr p_as p_alt p_expr_basic
%type <node> p_arg
%type <node> p_value p_primitive p_variable p_var_ref p_const
%type <node> p_kw
%type <node> f_block_arg keyword_variable program
%type <node> var_lhs lhs mlhs_node mlhs mlhs_item mlhs_inner for_var

%type <node_list> assocs assoc_list opt_f_block_arg f_rest_arg f_optarg f_args
%type <node_list> f_block_optarg f_kwrest f_no_kwarg f_kwarg f_block_kwarg f_arg
%type <node_list> opt_args_tail args_tail
%type <node_list> regexp_contents xstring_contents string_contents
%type <node_list> qsym_list qword_list symbol_list word word_list
%type <node_list> string exc_list opt_rescue
%type <node_list> p_kwnorest p_kwrest p_any_kwrest p_kwarg p_kwargs p_args_post
%type <node_list> p_find p_args_tail p_args_head p_args
%type <node_list> case_args bv_decls opt_bv_decl
%type <node_list> block_param opt_block_args_tail block_args_tail f_any_kwrest f_margs f_marg_list mrhs
%type <node_list> args opt_block_arg command_args call_args opt_call_args aref_args
%type <node_list> undef_list mlhs_post mlhs_head stmts top_stmts mlhs_basic

%type <expr_value_do> expr_value_do
%type <superclass> superclass
%type <opt_ensure> opt_ensure
%type <opt_else> opt_else
%type <exc_var> exc_var
%type <if_tail> if_tail
%type <brace_body> brace_body
%type <cmd_brace_block> cmd_brace_block
%type <brace_block> brace_block
%type <do_block> do_block
%type <begin_block> begin_block
%type <lambda_body> lambda_body
%type <paren_args> paren_args
%type <opt_paren_args> opt_paren_args
%type <defn_head> defn_head
%type <defs_head> defs_head
%type <cases> cases
%type <case_body> case_body
%type <p_cases> p_cases
%type <p_case_body> p_case_body
%type <user_variable> user_variable
%type <do_body> do_body
%type <p_top_expr> p_top_expr

%type <maybe_node> compstmt bodystmt f_arglist f_paren_args opt_block_param block_param_def f_larglist

%type <token>   sym operation operation2 operation3
%type <token>   cname fname op f_norm_arg f_bad_arg
%type <token>   f_label f_arg_asgn call_op call_op2 reswords relop dot_or_colon
%type <token>   p_rest p_kw_label
%type <token>   args_forward excessed_comma def_name
%type <token>   rbrace rparen rbracket p_lparen p_lbracket k_return then term fcall

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
%token <token> tSTRING_BEG      "string begin"
%token <token> tXSTRING_BEG     "backtick literal"
%token <token> tREGEXP_BEG      "regexp literal"
%token <token> tWORDS_BEG       "word list"
%token <token> tQWORDS_BEG      "verbatim word list"
%token <token> tSYMBOLS_BEG     "symbol list"
%token <token> tQSYMBOLS_BEG    "verbatim symbol list"
%token <token> tSTRING_END      "string end"
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
                        let _trigger_locs = @2;
                        self.result = $<MaybeNode>2;
                    }
                ;

    top_compstmt: top_stmts opt_terms
                    {
                        $$ = Value::MaybeNode(
                            self.builder.compstmt($<NodeList>1)
                        );
                    }
                ;

       top_stmts: none
                    {
                      $$ = Value::NodeList( vec![] );
                    }
                | top_stmt
                    {
                      $$ = Value::NodeList( vec![ $<Node>1 ] );
                    }
                | top_stmts terms top_stmt
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( $<Node>3 );
                        $$ = Value::NodeList(nodes);
                    }
                | error top_stmt
                    {
                      $$ = Value::NodeList( vec![ $<Node>2 ] );
                    }
                ;

        top_stmt: stmt
                | klBEGIN begin_block
                    {
                        let (begin, stmt, end) = $<BeginBlock>2;
                        $$ = Value::Node(
                            self.builder.preexe($<Token>1, begin, stmt, end)
                        );
                    }
                ;

     begin_block: tLCURLY top_compstmt tRCURLY
                    {
                        $$ = Value::BeginBlock(( $<Token>1, $<MaybeNode>2, $<Token>3 ));
                    }
                ;

        bodystmt: compstmt opt_rescue
                  k_else
                    {
                        let opt_rescue = $<Borrow:NodeList>2;
                        if opt_rescue.is_empty() {
                            self.yyerror(&@3, "else without rescue is useless");
                        }
                        $<none>$ = Value::None;
                    }
                  compstmt
                  opt_ensure
                    {
                        let compound_stmt = $<MaybeNode>1;
                        let rescue_bodies = $<NodeList>2;
                        let else_ = Some(( $<Token>3, $<Node>5 ));
                        let ensure = $<OptEnsure>6;

                        $$ = Value::MaybeNode(
                            self.builder.begin_body(
                                compound_stmt,
                                rescue_bodies,
                                else_,
                                ensure
                            )
                        );
                    }
                | compstmt
                  opt_rescue
                  opt_ensure
                    {
                        let compound_stmt = $<MaybeNode>1;
                        let rescue_bodies = $<NodeList>2;
                        let ensure = $<OptEnsure>3;

                        $$ = Value::MaybeNode(
                            self.builder.begin_body(
                                compound_stmt,
                                rescue_bodies,
                                None,
                                ensure
                            )
                        );
                    }
                ;

        compstmt: stmts opt_terms
                    {
                        $$ = Value::MaybeNode(
                            self.builder.compstmt($<NodeList>1)
                        );
                    }
                ;

           stmts: none
                    {
                        $$ = Value::NodeList( vec![] );
                    }
                | stmt_or_begin
                    {
                        $$ = Value::NodeList( vec![ $<Node>1 ] );
                    }
                | stmts terms stmt_or_begin
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( $<Node>3 );
                        $$ = Value::NodeList(nodes);
                    }
                | error stmt
                    {
                        $$ = Value::NodeList( vec![ $<Node>2 ] );
                    }
                ;

   stmt_or_begin: stmt
                | klBEGIN
                    {
                        self.yyerror(&@1, "BEGIN is permitted only at toplevel");
                    }
                  begin_block
                    {
                        $$ = $<RAW>3;
                    }
                ;

            stmt: kALIAS fitem
                    {
                        self.yylexer.set_lex_state(EXPR_FNAME);
                    }
                  fitem
                    {
                        $$ = Value::Node(
                            self.builder.alias($<Token>1, $<Node>2, $<Node>3)
                        );
                    }
                | kALIAS tGVAR tGVAR
                    {
                        $$ = Value::Node(
                            self.builder.alias(
                                $<Token>1,
                                self.builder.gvar($<Token>2),
                                self.builder.gvar($<Token>3),
                            )
                        )
                    }
                | kALIAS tGVAR tBACK_REF
                    {
                        $$ = Value::Node(
                            self.builder.alias(
                                $<Token>1,
                                self.builder.gvar($<Token>2),
                                self.builder.back_ref($<Token>3),
                            )
                        )
                    }
                | kALIAS tGVAR tNTH_REF
                    {
                        // FIXME: diagnostic :error, :nth_ref_alias, None, val[2]
                    }
                | kUNDEF undef_list
                    {
                        $$ = Value::Node(
                            self.builder.undef_method(
                                $<Token>1,
                                $<NodeList>2
                            )
                        )
                    }
                | stmt kIF_MOD expr_value
                    {
                        $$ = Value::Node(
                            self.builder.condition_mod(
                                Some($<Node>1),
                                None,
                                $<Token>2,
                                $<Node>3,
                            )
                        );
                    }
                | stmt kUNLESS_MOD expr_value
                    {
                        $$ = Value::Node(
                            self.builder.condition_mod(
                                None,
                                Some($<Node>1),
                                $<Token>2,
                                $<Node>3,
                            )
                        );
                    }
                | stmt kWHILE_MOD expr_value
                    {
                        $$ = Value::Node(
                            self.builder.loop_mod(
                                LoopType::While,
                                $<Node>1,
                                $<Token>2,
                                $<Node>3,
                            )
                        );
                    }
                | stmt kUNTIL_MOD expr_value
                    {
                        $$ = Value::Node(
                            self.builder.loop_mod(
                                LoopType::Until,
                                $<Node>1,
                                $<Token>2,
                                $<Node>3,
                            )
                        );
                    }
                | stmt kRESCUE_MOD stmt
                    {
                        let rescue_body = self.builder.rescue_body(
                            $<Token>2,
                            None,
                            None,
                            None,
                            None,
                            Some($<Node>3)
                        );

                        $$ = Value::Node(
                            self.builder.begin_body(
                                Some($<Node>1),
                                vec![rescue_body],
                                None,
                                None,
                            ).unwrap()
                        );
                    }
                | klEND tLCURLY compstmt tRCURLY
                    {
                        $$ = Value::Node(
                            self.builder.postexe(
                                $<Token>1,
                                $<Token>2,
                                $<MaybeNode>3,
                                $<Token>4,
                            )
                        );
                    }
                | command_asgn
                | mlhs tEQL command_call
                    {
                        $$ = Value::Node(
                            self.builder.multi_assign(
                                $<Node>1,
                                $<Token>2,
                                $<Node>3
                            )
                        );
                    }
                | lhs tEQL mrhs
                    {
                        $$ = Value::Node(
                            self.builder.assign(
                                $<Node>1,
                                $<Token>2,
                                self.builder.array(
                                    None,
                                    $<NodeList>3,
                                    None
                                )
                            )
                        );
                    }
                | mlhs tEQL mrhs_arg kRESCUE_MOD stmt
                    {
                        let rescue_body = self.builder.rescue_body(
                            $<Token>4,
                            None,
                            None,
                            None,
                            None,
                            Some($<Node>5)
                        );
                        let begin_body = self.builder.begin_body(
                            Some($<Node>3),
                            vec![ rescue_body ],
                            None,
                            None
                        ).unwrap();
                        $$ = Value::Node(
                            self.builder.multi_assign(
                                $<Node>1,
                                $<Token>2,
                                begin_body
                            )
                        );
                    }
                | mlhs tEQL mrhs_arg
                    {
                        $$ = Value::Node(
                            self.builder.multi_assign(
                                $<Node>1,
                                $<Token>2,
                                $<Node>3
                            )
                        );
                    }
                | rassign
                | expr
                ;

         rassign: arg_value tASSOC lhs
                    {
                        $$ = Value::Node(
                            self.builder.rassign(
                                $<Node>1,
                                $<Token>2,
                                $<Node>3
                            )
                        );
                    }
                | arg_value tASSOC mlhs
                    {
                        $$ = Value::Node(
                            self.builder.multi_rassign(
                                $<Node>1,
                                $<Token>2,
                                $<Node>3
                            )
                        );
                    }
                | rassign tASSOC lhs
                    {
                        $$ = Value::Node(
                            self.builder.rassign(
                                $<Node>1,
                                $<Token>2,
                                $<Node>3
                            )
                        );
                    }
                | rassign tASSOC mlhs
                    {
                        $$ = Value::Node(
                            self.builder.multi_rassign(
                                $<Node>1,
                                $<Token>2,
                                $<Node>3
                            )
                        );
                    }
                ;

    command_asgn: lhs tEQL command_rhs
                    {
                        $$ = Value::Node(
                            self.builder.assign(
                                $<Node>1,
                                $<Token>2,
                                $<Node>3
                            )
                        );
                    }
                | var_lhs tOP_ASGN command_rhs
                    {
                        $$ = Value::Node(
                            self.builder.op_assign(
                                $<Node>1,
                                $<Token>2,
                                $<Node>3
                            )
                        );
                    }
                | primary_value tLBRACK2 opt_call_args rbracket tOP_ASGN command_rhs
                    {
                        $$ = Value::Node(
                            self.builder.op_assign(
                                self.builder.index(
                                    $<Node>1,
                                    $<Token>2,
                                    $<NodeList>3,
                                    $<Token>4
                                ),
                                $<Token>5,
                                $<Node>6
                            )
                        );
                    }
                | primary_value call_op tIDENTIFIER tOP_ASGN command_rhs
                    {
                        $$ = Value::Node(
                            self.builder.op_assign(
                                self.builder.call_method(
                                    Some($<Node>1),
                                    Some($<Token>2),
                                    Some($<Token>3),
                                    None,
                                    vec![],
                                    None
                                ),
                                $<Token>4,
                                $<Node>5
                            )
                        );
                    }
                | primary_value call_op tCONSTANT tOP_ASGN command_rhs
                    {
                        $$ = Value::Node(
                            self.builder.op_assign(
                                self.builder.call_method(
                                    Some($<Node>1),
                                    Some($<Token>2),
                                    Some($<Token>3),
                                    None,
                                    vec![],
                                    None
                                ),
                                $<Token>4,
                                $<Node>5
                            )
                        );
                    }
                | primary_value tCOLON2 tCONSTANT tOP_ASGN command_rhs
                    {
                        let const_ = self.builder.const_op_assignable(
                            self.builder.const_fetch(
                                $<Node>1,
                                $<Token>2,
                                $<Token>3
                            )
                        );
                        $$ = Value::Node(
                            self.builder.op_assign(
                                const_,
                                $<Token>4,
                                $<Node>5
                            )
                        );
                    }
                | primary_value tCOLON2 tIDENTIFIER tOP_ASGN command_rhs
                    {
                        $$ = Value::Node(
                            self.builder.op_assign(
                                self.builder.call_method(
                                    Some($<Node>1),
                                    Some($<Token>2),
                                    Some($<Token>3),
                                    None,
                                    vec![],
                                    None
                                ),
                                $<Token>4,
                                $<Node>5
                            )
                        );
                    }
                | backref tOP_ASGN command_rhs
                    {
                        $$ = Value::Node(
                            self.builder.op_assign(
                                $<Node>1,
                                $<Token>2,
                                $<Node>3
                            )
                        );
                    }
                ;

     command_rhs: command_call   %prec tOP_ASGN
                | command_call kRESCUE_MOD stmt
                    {
                        let rescue_body = self.builder.rescue_body(
                            $<Token>2,
                            None,
                            None,
                            None,
                            None,
                            Some($<Node>3)
                        );
                        $$ = Value::Node(
                            self.builder.begin_body(
                                Some($<Node>1),
                                vec![ rescue_body ],
                                None,
                                None
                            ).unwrap()
                        );
                    }
                | command_asgn
                ;

            expr: command_call
                | expr kAND expr
                    {
                        $$ = Value::Node(
                            self.builder.logical_op(
                                LogicalOp::And,
                                $<Node>1,
                                $<Token>2,
                                $<Node>3
                            )
                        );
                    }
                | expr kOR expr
                    {
                        $$ = Value::Node(
                            self.builder.logical_op(
                                LogicalOp::Or,
                                $<Node>1,
                                $<Token>2,
                                $<Node>3
                            )
                        );
                    }
                | kNOT opt_nl expr
                    {
                        $$ = Value::Node(
                            self.builder.not_op(
                                $<Token>1,
                                None,
                                Some($<Node>3),
                                None
                            )
                        );
                    }
                | tBANG command_call
                    {
                        $$ = Value::Node(
                            self.builder.not_op(
                                $<Token>1,
                                None,
                                Some($<Node>2),
                                None
                            )
                        );
                    }
                | arg kIN
                    {
                        self.yylexer.set_lex_state(EXPR_BEG);
                        self.yylexer.p.command_start = true;
                        // self.pattern_variables.push();

                        $<Bool>$ = Value::Bool(self.yylexer.p.in_kwarg);
                        self.yylexer.p.in_kwarg = true;
                    }
                  p_expr
                    {
                        self.yylexer.p.in_kwarg = $<Bool>3;

                        $$ = Value::Node(
                            self.builder.in_match(
                                $<Node>1,
                                $<Token>2,
                                $<Node>4
                            )
                        );
                    }
                | arg %prec tLBRACE_ARG
                ;

        def_name: fname
                    {
                        self.static_env.extend_static();
                        self.yylexer.cmdarg_push(false);
                        self.yylexer.cond_push(false);
                        self.current_arg_stack.push(None);

                        $$ = $<RAW>1;
                    }
                ;

       defn_head: k_def def_name
                    {
                        self.yylexer.p.context.push(ContextItem::Def);

                        $$ = Value::DefnHead(( $<Token>1, $<Token>2 ));
                    }
                ;

       defs_head: k_def singleton dot_or_colon
                    {
                        self.yylexer.set_lex_state(EXPR_FNAME);
                    }
                  def_name
                    {
                        self.yylexer.p.context.push(ContextItem::Defs);

                        $$ = Value::DefsHead(( $<Token>1, $<Node>2, $<Token>3, $<Token>5 ));
                    }
                ;

      expr_value: expr
                ;

   expr_value_do:   {
                        self.yylexer.cond_push(true);
                    }
                  expr_value do
                    {
                        self.yylexer.cond_pop();

                        $$ = Value::ExprValueDo(( $<Node>2, $<Token>3 ));
                    }
                ;


    command_call: command
                | block_command
                ;

   block_command: block_call
                | block_call call_op2 operation2 command_args
                ;

 cmd_brace_block: tLBRACE_ARG
                    {
                        self.yylexer.p.context.push(ContextItem::Block);
                    }
                  brace_body tRCURLY
                    {
                        self.yylexer.p.context.pop();
                        let (args, body) = $<BraceBody>3;
                        $$ = Value::CmdBraceBlock(( $<Token>1, args, body, $<Token>4 ));
                    }
                ;

           fcall: operation
                ;

         command: fcall command_args       %prec tLOWEST
                    {
                        $$ = Value::Node(
                            self.builder.call_method(
                                None,
                                None,
                                Some($<Token>1),
                                None,
                                $<NodeList>2,
                                None
                            )
                        );
                    }
                | fcall command_args cmd_brace_block
                    {
                        let method_call = self.builder.call_method(
                            None,
                            None,
                            Some($<Token>1),
                            None,
                            $<NodeList>2,
                            None
                        );
                        let (begin_t, args, body, end_t) = $<CmdBraceBlock>3;

                        $$ = Value::Node(
                            self.builder.block(
                                method_call,
                                begin_t,
                                args,
                                body,
                                end_t
                            )
                        );
                    }
                | primary_value call_op operation2 command_args %prec tLOWEST
                    {
                        $$ = Value::Node(
                            self.builder.call_method(
                                Some($<Node>1),
                                Some($<Token>2),
                                Some($<Token>3),
                                None,
                                $<NodeList>4,
                                None
                            )
                        );
                    }
                | primary_value call_op operation2 command_args cmd_brace_block
                    {
                        let method_call = self.builder.call_method(
                            Some($<Node>1),
                            Some($<Token>2),
                            Some($<Token>3),
                            None,
                            $<NodeList>4,
                            None
                        );
                        let (begin_t, args, body, end_t) = $<CmdBraceBlock>5;

                        $$ = Value::Node(
                            self.builder.block(
                                method_call,
                                begin_t,
                                args,
                                body,
                                end_t
                            )
                        );
                    }
                | primary_value tCOLON2 operation2 command_args %prec tLOWEST
                    {
                        $$ = Value::Node(
                            self.builder.call_method(
                                Some($<Node>1),
                                Some($<Token>2),
                                Some($<Token>3),
                                None,
                                $<NodeList>4,
                                None
                            )
                        );
                    }
                | primary_value tCOLON2 operation2 command_args cmd_brace_block
                    {
                        let method_call = self.builder.call_method(
                            Some($<Node>1),
                            Some($<Token>2),
                            Some($<Token>3),
                            None,
                            $<NodeList>4,
                            None
                        );
                        let (begin_t, args, body, end_t) = $<CmdBraceBlock>5;

                        $$ = Value::Node(
                            self.builder.block(
                                method_call,
                                begin_t,
                                args,
                                body,
                                end_t
                            )
                        );
                    }
                | kSUPER command_args
                    {
                        $$ = Value::Node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Super,
                                $<Token>1,
                                None,
                                $<NodeList>2,
                                None
                            )
                        );
                    }
                | kYIELD command_args
                    {
                        $$ = Value::Node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Yield,
                                $<Token>1,
                                None,
                                $<NodeList>2,
                                None
                            )
                        );
                    }
                | k_return call_args
                    {
                        $$ = Value::Node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Return,
                                $<Token>1,
                                None,
                                $<NodeList>2,
                                None
                            )
                        );
                    }
                | kBREAK call_args
                    {
                        $$ = Value::Node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Break,
                                $<Token>1,
                                None,
                                $<NodeList>2,
                                None
                            )
                        );
                    }
                | kNEXT call_args
                    {
                        $$ = Value::Node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Next,
                                $<Token>1,
                                None,
                                $<NodeList>2,
                                None
                            )
                        );
                    }
                ;

            mlhs: mlhs_basic
                    {
                        $$ = Value::Node(
                            self.builder.multi_lhs(
                                None,
                                $<NodeList>1,
                                None
                            )
                        );
                    }
                | tLPAREN mlhs_inner rparen
                    {
                        $$ = Value::Node(
                            self.builder.multi_lhs(
                                Some($<Token>1),
                                vec![ $<Node>2 ],
                                Some($<Token>3)
                            )
                        );
                    }
                ;

      mlhs_inner: mlhs_basic
                    {
                        $$ = Value::Node(
                            self.builder.multi_lhs(
                                None,
                                $<NodeList>1,
                                None
                            )
                        );
                    }
                | tLPAREN mlhs_inner rparen
                    {
                        $$ = Value::Node(
                            self.builder.multi_lhs(
                                Some($<Token>1),
                                vec![ $<Node>2 ],
                                Some($<Token>3)
                            )
                        );
                    }
                ;

      mlhs_basic: mlhs_head
                | mlhs_head mlhs_item
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( $<Node>2 );
                        $$ = Value::NodeList(nodes);
                    }
                | mlhs_head tSTAR mlhs_node
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( self.builder.splat($<Token>2, Some($<Node>3)) );
                        $$ = Value::NodeList(nodes);
                    }
                | mlhs_head tSTAR mlhs_node tCOMMA mlhs_post
                    {
                        let nodes = [
                            $<NodeList>1,
                            vec![ self.builder.splat($<Token>2, Some($<Node>3)) ],
                            $<NodeList>5
                        ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | mlhs_head tSTAR
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( self.builder.splat($<Token>2, None) );
                        $$ = Value::NodeList(nodes);
                    }
                | mlhs_head tSTAR tCOMMA mlhs_post
                    {
                        let nodes = [
                            $<NodeList>1,
                            vec![ self.builder.splat($<Token>2, None) ],
                            $<NodeList>4
                        ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | tSTAR mlhs_node
                    {
                        $$ = Value::NodeList(
                            vec![
                                self.builder.splat($<Token>1, Some($<Node>2))
                            ]
                        );
                    }
                | tSTAR mlhs_node tCOMMA mlhs_post
                    {
                        let nodes = [
                            vec![ self.builder.splat($<Token>1, Some($<Node>2)) ],
                            $<NodeList>4
                        ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | tSTAR
                    {
                        $$ = Value::NodeList(
                            vec![
                                self.builder.splat($<Token>1, None)
                            ]
                        );
                    }
                | tSTAR tCOMMA mlhs_post
                    {
                        let nodes = [
                            vec![ self.builder.splat($<Token>1, None) ],
                            $<NodeList>3
                        ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                ;

       mlhs_item: mlhs_node
                | tLPAREN mlhs_inner rparen
                    {
                        $$ = Value::Node(
                            self.builder.begin(
                                $<Token>1,
                                Some($<Node>2),
                                $<Token>3
                            )
                        );
                    }
                ;

       mlhs_head: mlhs_item tCOMMA
                    {
                        $$ = Value::NodeList( vec![ $<Node>1 ]);
                    }
                | mlhs_head mlhs_item tCOMMA
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( $<Node>2 );
                        $$ = Value::NodeList(nodes);
                    }
                ;

       mlhs_post: mlhs_item
                    {
                        $$ = Value::NodeList( vec![ $<Node>1 ] );
                    }
                | mlhs_post tCOMMA mlhs_item
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( $<Node>3 );
                        $$ = Value::NodeList(nodes);
                    }
                ;

       mlhs_node: user_variable
                    {
                        $$ = Value::Node(
                            self.builder.assignable($<Node>1)
                        );
                    }
                | keyword_variable
                    {
                        $$ = Value::Node(
                            self.builder.assignable($<Node>1)
                        );
                    }
                | primary_value tLBRACK2 opt_call_args rbracket
                    {
                        $$ = Value::Node(
                            self.builder.index_asgn(
                                $<Node>1,
                                $<Token>2,
                                $<NodeList>3,
                                $<Token>4
                            )
                        );
                    }
                | primary_value call_op tIDENTIFIER
                    {
                        // if (val[1][0] == :anddot)
                        //     diagnostic :error, :csend_in_lhs_of_masgn, None, val[1]
                        // end

                        $$ = Value::Node(
                            self.builder.attr_asgn(
                                $<Node>1,
                                $<Token>2,
                                $<Token>3
                            )
                        );
                    }
                | primary_value tCOLON2 tIDENTIFIER
                    {
                        $$ = Value::Node(
                            self.builder.attr_asgn(
                                $<Node>1,
                                $<Token>2,
                                $<Token>3
                            )
                        );
                    }
                | primary_value call_op tCONSTANT
                    {
                        // if (val[1][0] == :anddot)
                        //     diagnostic :error, :csend_in_lhs_of_masgn, None, val[1]
                        // end

                        $$ = Value::Node(
                            self.builder.attr_asgn(
                                $<Node>1,
                                $<Token>2,
                                $<Token>3
                            )
                        );
                    }
                | primary_value tCOLON2 tCONSTANT
                    {
                        $$ = Value::Node(
                            self.builder.assignable(
                                self.builder.const_fetch(
                                    $<Node>1,
                                    $<Token>2,
                                    $<Token>3
                                )
                            )
                        );
                    }
                | tCOLON3 tCONSTANT
                    {
                        $$ = Value::Node(
                            self.builder.assignable(
                                self.builder.const_global(
                                    $<Token>1,
                                    $<Token>2
                                )
                            )
                        );
                    }
                | backref
                    {
                        $$ = Value::Node(
                            self.builder.assignable(
                                $<Node>1
                            )
                        );
                    }
                ;

             lhs: user_variable
                    {
                        $$ = Value::Node(
                            self.builder.assignable($<Node>1)
                        );
                    }
                | keyword_variable
                    {
                        $$ = Value::Node(
                            self.builder.assignable($<Node>1)
                        );
                    }
                | primary_value tLBRACK2 opt_call_args rbracket
                    {
                        $$ = Value::Node(
                            self.builder.index_asgn(
                                $<Node>1,
                                $<Token>2,
                                $<NodeList>3,
                                $<Token>4
                            )
                        )
                    }
                | primary_value call_op tIDENTIFIER
                    {
                        $$ = Value::Node(
                            self.builder.attr_asgn(
                                $<Node>1,
                                $<Token>2,
                                $<Token>3
                            )
                        );
                    }
                | primary_value tCOLON2 tIDENTIFIER
                    {
                        $$ = Value::Node(
                            self.builder.attr_asgn(
                                $<Node>1,
                                $<Token>2,
                                $<Token>3
                            )
                        );
                    }
                | primary_value call_op tCONSTANT
                    {
                        $$ = Value::Node(
                            self.builder.attr_asgn(
                                $<Node>1,
                                $<Token>2,
                                $<Token>3
                            )
                        );
                    }
                | primary_value tCOLON2 tCONSTANT
                    {
                        $$ = Value::Node(
                            self.builder.assignable(
                                self.builder.const_fetch(
                                    $<Node>1,
                                    $<Token>2,
                                    $<Token>3,
                                )
                            )
                        );
                    }
                | tCOLON3 tCONSTANT
                    {
                        $$ = Value::Node(
                            self.builder.assignable(
                                self.builder.const_global(
                                    $<Token>1,
                                    $<Token>2,
                                )
                            )
                        );
                    }
                | backref
                    {
                        $$ = Value::Node(
                            self.builder.assignable(
                                $<Node>1
                            )
                        );
                    }
                ;

           cname: tIDENTIFIER
                    {
                        // diagnostic :error, :module_name_const, None, val[0]
                        $$ = $<RAW>1;
                    }
                | tCONSTANT
                ;

           cpath: tCOLON3 cname
                    {
                        $$ = Value::Node(
                            self.builder.const_global($<Token>1, $<Token>2)
                        );
                    }
                | cname
                    {
                        $$ = Value::Node(
                            self.builder.const_($<Token>1)
                        );
                    }
                | primary_value tCOLON2 cname
                    {
                        $$ = Value::Node(
                            self.builder.const_fetch(
                                $<Node>1,
                                $<Token>2,
                                $<Token>3,
                            )
                        );
                    }
                ;

           fname: tIDENTIFIER
                | tCONSTANT
                | tFID
                | op
                    {
                        self.yylexer.set_lex_state(EXPR_ENDFN);
                        $$ = $<RAW>1;
                    }
                | reswords
                ;

           fitem: fname
                    {
                        $$ = Value::Node(
                            self.builder.symbol_internal($<Token>1)
                        );
                    }
                | symbol
                ;

      undef_list: fitem
                    {
                        $$ = Value::NodeList( vec![ $<Node>1 ] );
                    }
                | undef_list tCOMMA { self.yylexer.set_lex_state(EXPR_FNAME); } fitem
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( $<Node>4 );
                        $$ = Value::NodeList(nodes);
                    }
                ;

              op: tPIPE
                | tCARET
                | tAMPER2
                | tCMP
                | tEQ
                | tEQQ
                | tMATCH
                | tNMATCH
                | tGT
                | tGEQ
                | tLT
                | tLEQ
                | tNEQ
                | tLSHFT
                | tRSHFT
                | tPLUS
                | tMINUS
                | tSTAR2
                | tSTAR
                | tDIVIDE
                | tPERCENT
                | tPOW
                | tDSTAR
                | tBANG
                | tTILDE
                | tUPLUS
                | tUMINUS
                | tAREF
                | tASET
                | tBACK_REF2
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
                        $$ = Value::Node(
                            self.builder.assign(
                                $<Node>1,
                                $<Token>2,
                                $<Node>3
                            )
                        );
                    }
                | var_lhs tOP_ASGN arg_rhs
                    {
                        $$ = Value::Node(
                            self.builder.op_assign(
                                $<Node>1,
                                $<Token>2,
                                $<Node>3
                            )
                        );
                    }
                | primary_value tLBRACK2 opt_call_args rbracket tOP_ASGN arg_rhs
                    {
                        $$ = Value::Node(
                            self.builder.op_assign(
                                self.builder.index(
                                    $<Node>1,
                                    $<Token>2,
                                    $<NodeList>3,
                                    $<Token>4
                                ),
                                $<Token>5,
                                $<Node>6
                            )
                        );
                    }
                | primary_value call_op tIDENTIFIER tOP_ASGN arg_rhs
                    {
                        $$ = Value::Node(
                            self.builder.op_assign(
                                self.builder.call_method(
                                    Some($<Node>1),
                                    Some($<Token>2),
                                    Some($<Token>3),
                                    None,
                                    vec![],
                                    None
                                ),
                                $<Token>4,
                                $<Node>5
                            )
                        );
                    }
                | primary_value call_op tCONSTANT tOP_ASGN arg_rhs
                    {
                        $$ = Value::Node(
                            self.builder.op_assign(
                                self.builder.call_method(
                                    Some($<Node>1),
                                    Some($<Token>2),
                                    Some($<Token>3),
                                    None,
                                    vec![],
                                    None
                                ),
                                $<Token>4,
                                $<Node>5
                            )
                        );
                    }
                | primary_value tCOLON2 tIDENTIFIER tOP_ASGN arg_rhs
                    {
                        $$ = Value::Node(
                            self.builder.op_assign(
                                self.builder.call_method(
                                    Some($<Node>1),
                                    Some($<Token>2),
                                    Some($<Token>3),
                                    None,
                                    vec![],
                                    None
                                ),
                                $<Token>4,
                                $<Node>5
                            )
                        );
                    }
                | primary_value tCOLON2 tCONSTANT tOP_ASGN arg_rhs
                    {
                        let const_ = self.builder.const_op_assignable(
                            self.builder.const_fetch(
                                $<Node>1,
                                $<Token>2,
                                $<Token>3
                            )
                        );
                        $$ = Value::Node(
                            self.builder.op_assign(
                                const_,
                                $<Token>4,
                                $<Node>5
                            )
                        );
                    }
                | tCOLON3 tCONSTANT tOP_ASGN arg_rhs
                    {
                        let const_ = self.builder.const_op_assignable(
                            self.builder.const_global(
                                $<Token>1,
                                $<Token>2
                            )
                        );
                        $$ = Value::Node(
                            self.builder.op_assign(
                                const_,
                                $<Token>3,
                                $<Node>4
                            )
                        );
                    }
                | backref tOP_ASGN arg_rhs
                    {
                        $$ = Value::Node(
                            self.builder.op_assign(
                                $<Node>1,
                                $<Token>2,
                                $<Node>3
                            )
                        );
                    }
                | arg tDOT2 arg
                    {
                        $$ = Value::Node(
                            self.builder.range_inclusive(
                                Some($<Node>1),
                                $<Token>2,
                                Some($<Node>3)
                            )
                        );
                    }
                | arg tDOT3 arg
                    {
                        $$ = Value::Node(
                            self.builder.range_exclusive(
                                Some($<Node>1),
                                $<Token>2,
                                Some($<Node>3)
                            )
                        );
                    }
                | arg tDOT2
                    {
                        $$ = Value::Node(
                            self.builder.range_inclusive(
                                Some($<Node>1),
                                $<Token>2,
                                None
                            )
                        );
                    }
                | arg tDOT3
                    {
                        $$ = Value::Node(
                            self.builder.range_exclusive(
                                Some($<Node>1),
                                $<Token>2,
                                None
                            )
                        );
                    }
                | tBDOT2 arg
                    {
                        $$ = Value::Node(
                            self.builder.range_inclusive(
                                None,
                                $<Token>1,
                                Some($<Node>2)
                            )
                        );
                    }
                | tBDOT3 arg
                    {
                        $$ = Value::Node(
                            self.builder.range_exclusive(
                                None,
                                $<Token>1,
                                Some($<Node>2)
                            )
                        );
                    }
                | arg tPLUS arg
                    {
                        $$ = Value::Node(
                            self.builder.binary_op($<Node>1, $<Token>2, $<Node>3)
                        );
                    }
                | arg tMINUS arg
                    {
                        $$ = Value::Node(
                            self.builder.binary_op($<Node>1, $<Token>2, $<Node>3)
                        );
                    }
                | arg tSTAR2 arg
                    {
                        $$ = Value::Node(
                            self.builder.binary_op($<Node>1, $<Token>2, $<Node>3)
                        );
                    }
                | arg tDIVIDE arg
                    {
                        $$ = Value::Node(
                            self.builder.binary_op($<Node>1, $<Token>2, $<Node>3)
                        );
                    }
                | arg tPERCENT arg
                    {
                        $$ = Value::Node(
                            self.builder.binary_op($<Node>1, $<Token>2, $<Node>3)
                        );
                    }
                | arg tPOW arg
                    {
                        $$ = Value::Node(
                            self.builder.binary_op($<Node>1, $<Token>2, $<Node>3)
                        );
                    }
                | tUMINUS_NUM simple_numeric tPOW arg
                    {
                        $$ = Value::Node(
                            self.builder.unary_op(
                                $<Token>1,
                                self.builder.binary_op(
                                    $<Node>2,
                                    $<Token>3,
                                    $<Node>4
                                )
                            )
                        );
                    }
                | tUPLUS arg
                    {
                        $$ = Value::Node(
                            self.builder.unary_op(
                                $<Token>1,
                                $<Node>2
                            )
                        );
                    }
                | tUMINUS arg
                    {
                        $$ = Value::Node(
                            self.builder.unary_op(
                                $<Token>1,
                                $<Node>2
                            )
                        );
                    }
                | arg tPIPE arg
                    {
                        $$ = Value::Node(
                            self.builder.binary_op($<Node>1, $<Token>2, $<Node>3)
                        );
                    }
                | arg tCARET arg
                    {
                        $$ = Value::Node(
                            self.builder.binary_op($<Node>1, $<Token>2, $<Node>3)
                        );
                    }
                | arg tAMPER2 arg
                    {
                        $$ = Value::Node(
                            self.builder.binary_op($<Node>1, $<Token>2, $<Node>3)
                        );
                    }
                | arg tCMP arg
                    {
                        $$ = Value::Node(
                            self.builder.binary_op($<Node>1, $<Token>2, $<Node>3)
                        );
                    }
                | rel_expr   %prec tCMP
                | arg tEQ arg
                    {
                        $$ = Value::Node(
                            self.builder.binary_op($<Node>1, $<Token>2, $<Node>3)
                        );
                    }
                | arg tEQQ arg
                    {
                        $$ = Value::Node(
                            self.builder.binary_op($<Node>1, $<Token>2, $<Node>3)
                        );
                    }
                | arg tNEQ arg
                    {
                        $$ = Value::Node(
                            self.builder.binary_op($<Node>1, $<Token>2, $<Node>3)
                        );
                    }
                | arg tMATCH arg
                    {
                        $$ = Value::Node(
                            self.builder.match_op($<Node>1, $<Token>2, $<Node>3)
                        );
                    }
                | arg tNMATCH arg
                    {
                        $$ = Value::Node(
                            self.builder.match_op(
                                $<Node>1,
                                $<Token>2,
                                $<Node>3
                            )
                        );
                    }
                | tBANG arg
                    {
                        $$ = Value::Node(
                            self.builder.not_op(
                                $<Token>1,
                                None,
                                Some($<Node>2),
                                None
                            )
                        );
                    }
                | tTILDE arg
                    {
                        $$ = Value::Node(
                            self.builder.unary_op(
                                $<Token>1,
                                $<Node>2
                            )
                        );
                    }
                | arg tLSHFT arg
                    {
                        $$ = Value::Node(
                            self.builder.binary_op($<Node>1, $<Token>2, $<Node>3)
                        );
                    }
                | arg tRSHFT arg
                    {
                        $$ = Value::Node(
                            self.builder.binary_op($<Node>1, $<Token>2, $<Node>3)
                        );
                    }
                | arg tANDOP arg
                    {
                        $$ = Value::Node(
                            self.builder.logical_op(
                                LogicalOp::And,
                                $<Node>1,
                                $<Token>2,
                                $<Node>3
                            )
                        );
                    }
                | arg tOROP arg
                    {
                        $$ = Value::Node(
                            self.builder.logical_op(
                                LogicalOp::Or,
                                $<Node>1,
                                $<Token>2,
                                $<Node>3
                            )
                        );
                    }
                | kDEFINED opt_nl arg
                    {
                        $$ = Value::Node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Defined,
                                $<Token>1,
                                None,
                                vec![ $<Node>3 ],
                                None
                            )
                        );
                    }
                | arg tEH arg opt_nl tCOLON arg
                    {
                        $$ = Value::Node(
                            self.builder.ternary(
                                $<Node>1,
                                $<Token>2,
                                $<Node>3,
                                $<Token>4,
                                $<Node>5
                            )
                        );
                    }
                | defn_head f_paren_args tEQL arg
                    {
                        let (def_t, name_t) = $<DefnHead>1;

                        // if name_t[0].end_with?('=')
                        //     diagnostic :error, :endless_setter, None, name_t
                        // end

                        $$ = Value::Node(
                            self.builder.def_endless_method(
                                def_t,
                                name_t,
                                $<MaybeNode>2,
                                $<Token>3,
                                Some($<Node>4)
                            )
                        );

                        self.yylexer.cmdarg_pop();
                        self.yylexer.cond_pop();
                        self.static_env.unextend();
                        self.yylexer.p.context.pop();
                        self.current_arg_stack.pop();
                    }
                | defn_head f_paren_args tEQL arg kRESCUE_MOD arg
                    {
                        let (def_t, name_t) = $<DefnHead>1;

                        let rescue_body = self.builder.rescue_body(
                            $<Token>5,
                            None,
                            None,
                            None,
                            None,
                            Some($<Node>6)
                        );

                        let method_body = self.builder.begin_body(
                            Some($<Node>4),
                            vec![ rescue_body ],
                            None,
                            None
                        );

                        $$ = Value::Node(
                            self.builder.def_endless_method(
                                def_t,
                                name_t,
                                $<MaybeNode>2,
                                $<Token>3,
                                method_body
                            )
                        );

                        self.yylexer.cmdarg_pop();
                        self.yylexer.cond_pop();
                        self.static_env.unextend();
                        self.yylexer.p.context.pop();
                        self.current_arg_stack.pop();
                    }
                | defs_head f_paren_args tEQL arg
                    {
                        let (def_t, definee, dot_t, name_t) = $<DefsHead>1;

                        $$ = Value::Node(
                            self.builder.def_endless_singleton(
                                def_t,
                                definee,
                                dot_t,
                                name_t,
                                $<MaybeNode>2,
                                $<Token>3,
                                Some($<Node>4)
                            )
                        );

                        self.yylexer.cmdarg_pop();
                        self.yylexer.cond_pop();
                        self.static_env.unextend();
                        self.yylexer.p.context.pop();
                        self.current_arg_stack.pop();
                    }
                | defs_head f_paren_args tEQL arg kRESCUE_MOD arg
                    {
                        let (def_t, definee, dot_t, name_t) = $<DefsHead>1;

                        let rescue_body = self.builder.rescue_body(
                            $<Token>5,
                            None,
                            None,
                            None,
                            None,
                            Some($<Node>6)
                        );

                        let method_body = self.builder.begin_body(
                            Some($<Node>4),
                            vec![ rescue_body ],
                            None,
                            None
                        );

                        $$ = Value::Node(
                            self.builder.def_endless_singleton(
                                def_t,
                                definee,
                                dot_t,
                                name_t,
                                $<MaybeNode>2,
                                $<Token>3,
                                method_body
                            )
                        );

                        self.yylexer.cmdarg_pop();
                        self.yylexer.cond_pop();
                        self.static_env.unextend();
                        self.yylexer.p.context.pop();
                        self.current_arg_stack.pop();
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
                        $$ = Value::Node(
                            self.builder.binary_op(
                                $<Node>1,
                                $<Token>2,
                                $<Node>3
                            )
                        );
                    }
                | rel_expr relop arg   %prec tGT
                    {
                        $$ = Value::Node(
                            self.builder.binary_op(
                                $<Node>1,
                                $<Token>2,
                                $<Node>3
                            )
                        );
                    }
                ;

       arg_value: arg
                ;

       aref_args: none
                    {
                        $$ = Value::NodeList( vec![] );
                    }
                | args trailer
                | args tCOMMA assocs trailer
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push(
                            self.builder.associate(None, $<NodeList>3, None)
                        );
                        $$ = Value::NodeList( nodes );
                    }
                | assocs trailer
                    {
                        $$ = Value::NodeList(
                            vec![
                                self.builder.associate(None, $<NodeList>1, None)
                            ]
                        );
                    }
                ;

         arg_rhs: arg   %prec tOP_ASGN
                | arg kRESCUE_MOD arg
                    {
                        let rescue_body = self.builder.rescue_body(
                            $<Token>2,
                            None,
                            None,
                            None,
                            None,
                            Some($<Node>3)
                        );

                        $$ = Value::Node(
                            self.builder.begin_body(
                                Some($<Node>1),
                                vec![ rescue_body ],
                                None,
                                None
                            ).unwrap()
                        );
                    }
                ;

      paren_args: tLPAREN2 opt_call_args rparen
                    {
                        $$ = Value::ParenArgs(( $<Token>1, $<NodeList>2, $<Token>3 ));
                    }
                | tLPAREN2 args tCOMMA args_forward rparen
                    {
                        // unless self.static_env.declared_forward_args?
                        //     diagnostic :error, :unexpected_token, { :token => 'tBDOT3' } , val[3]
                        // end

                        let args = [
                            $<NodeList>2,
                            vec![ self.builder.forwarded_args($<Token>4) ]
                        ].concat();
                        $$ = Value::ParenArgs(( $<Token>1, args, $<Token>5 ));
                    }
                | tLPAREN2 args_forward rparen
                    {
                        // unless self.static_env.declared_forward_args?
                        //     diagnostic :error, :unexpected_token, { :token => 'tBDOT3' } , val[1]
                        // end

                        $$ = Value::ParenArgs(( $<Token>1, vec![ self.builder.forwarded_args($<Token>2) ], $<Token>3 ));
                    }
                ;

  opt_paren_args: none
                    {
                        $$ = Value::OptParenArgs(( None, vec![], None ));
                    }
                | paren_args
                    {
                        let (lparen, body, rparen) = $<ParenArgs>1;
                        $$ = Value::OptParenArgs(( Some(lparen), body, Some(rparen) ));
                    }
                ;

   opt_call_args: none
                    {
                        $$ = Value::NodeList( vec![] );
                    }
                | call_args
                | args tCOMMA
                | args tCOMMA assocs tCOMMA
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( self.builder.associate(None, $<NodeList>3, None) );
                        $$ = Value::NodeList( nodes );
                    }
                | assocs tCOMMA
                    {
                        $$ = Value::NodeList(
                            vec![
                                self.builder.associate(None, $<NodeList>1, None)
                            ]
                        );
                    }
                ;

       call_args: command
                    {
                        $$ = Value::NodeList( vec![ $<Node>1 ] );
                    }
                | args opt_block_arg
                    {
                        let nodes = [ $<NodeList>1, $<NodeList>2 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | assocs opt_block_arg
                    {
                        let nodes = [
                            vec![ self.builder.associate(None, $<NodeList>1, None) ],
                            $<NodeList>2
                        ].concat();
                        $$ = Value::NodeList( nodes );
                    }
                | args tCOMMA assocs opt_block_arg
                    {
                        let nodes = [
                            $<NodeList>1,
                            vec![ self.builder.associate(None, $<NodeList>3, None) ],
                            $<NodeList>4
                        ].concat();
                        $$ = Value::NodeList( nodes );
                    }
                | block_arg
                    {
                        $$ = Value::NodeList( vec![ $<Node>1 ] );
                    }
                ;

    command_args:   {
                        // TODO: crazy cmdarg manipulation
                    }
                  call_args
                    {
                        // TODO: crazy cmdarg manipulation
                        $$ = $<RAW>2;
                    }
                ;

       block_arg: tAMPER arg_value
                    {
                        $$ = Value::Node(
                            self.builder.block_pass(
                                $<Token>1,
                                $<Node>2
                            )
                        );
                    }
                ;

   opt_block_arg: tCOMMA block_arg
                    {
                        $$ = Value::NodeList( vec![ $<Node>2 ] );
                    }
                | none
                    {
                        $$ = Value::NodeList( vec![] );
                    }
                ;

            args: arg_value
                    {
                        $$ = Value::NodeList( vec![ $<Node>1 ] );
                    }
                | tSTAR arg_value
                    {
                        $$ = Value::NodeList(
                            vec![
                                self.builder.splat($<Token>1, Some($<Node>2))
                            ]
                        );
                    }
                | args tCOMMA arg_value
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( $<Node>3 );
                        $$ = Value::NodeList(nodes);
                    }
                | args tCOMMA tSTAR arg_value
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( self.builder.splat($<Token>3, Some($<Node>4)) );
                        $$ = Value::NodeList(nodes);
                    }
                ;

        mrhs_arg: mrhs
                    {
                        $$ = Value::Node(
                            self.builder.array(None, $<NodeList>1, None)
                        );
                    }
                | arg_value
                ;

            mrhs: args tCOMMA arg_value
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( $<Node>3 );
                        $$ = Value::NodeList(nodes);
                    }
                | args tCOMMA tSTAR arg_value
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push(
                            self.builder.splat($<Token>3, Some($<Node>4))
                        );
                        $$ = Value::NodeList(nodes);
                    }
                | tSTAR arg_value
                    {
                        $$ = Value::NodeList(
                            vec![
                                self.builder.splat($<Token>1, Some($<Node>2))
                            ]
                        );
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
                        $$ = Value::Node(
                            self.builder.call_method(
                                None,
                                None,
                                Some($<Token>1),
                                None,
                                vec![],
                                None
                            )
                        );
                    }
                | k_begin
                    {
                        self.yylexer.cmdarg_push(false);
                    }
                  bodystmt
                  k_end
                    {
                        self.yylexer.cmdarg_pop();

                        $$ = Value::Node(
                            self.builder.begin_keyword($<Token>1, $<MaybeNode>3, $<Token>4)
                        );
                    }
                | tLPAREN_ARG { self.yylexer.set_lex_state(EXPR_ENDARG); } rparen
                    {
                        $$ = Value::Node(
                            self.builder.begin(
                                $<Token>1,
                                None,
                                $<Token>3
                            )
                        );
                    }
                | tLPAREN_ARG stmt { self.yylexer.set_lex_state(EXPR_ENDARG); } rparen
                    {
                        $$ = Value::Node(
                            self.builder.begin(
                                $<Token>1,
                                Some($<Node>2),
                                $<Token>4
                            )
                        );
                    }
                | tLPAREN compstmt tRPAREN
                    {
                        $$ = Value::Node(
                            self.builder.begin(
                                $<Token>1,
                                Some($<Node>2),
                                $<Token>3
                            )
                        );
                    }
                | primary_value tCOLON2 tCONSTANT
                    {
                        $$ = Value::Node(
                            self.builder.const_fetch(
                                $<Node>1,
                                $<Token>2,
                                $<Token>3,
                            )
                        );
                    }
                | tCOLON3 tCONSTANT
                    {
                        $$ = Value::Node(
                            self.builder.const_global($<Token>1, $<Token>2)
                        );
                    }
                | tLBRACK aref_args tRBRACK
                    {
                        $$ = Value::Node(
                            self.builder.array(
                                Some($<Token>1),
                                $<NodeList>2,
                                Some($<Token>3)
                            )
                        );
                    }
                | tLBRACE assoc_list tRCURLY
                    {
                        $$ = Value::Node(
                            self.builder.associate(
                                Some($<Token>1),
                                $<NodeList>2,
                                Some($<Token>3)
                            )
                        );
                    }
                | k_return
                    {
                        $$ = Value::Node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Return,
                                $<Token>1,
                                None,
                                vec![],
                                None
                            )
                        );
                    }
                | kYIELD tLPAREN2 call_args rparen
                    {
                        $$ = Value::Node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Yield,
                                $<Token>1,
                                Some($<Token>2),
                                $<NodeList>3,
                                Some($<Token>4)
                            )
                        );
                    }
                | kYIELD tLPAREN2 rparen
                    {
                        $$ = Value::Node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Yield,
                                $<Token>1,
                                Some($<Token>2),
                                vec![],
                                Some($<Token>3)
                            )
                        );
                    }
                | kYIELD
                    {
                        $$ = Value::Node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Yield,
                                $<Token>1,
                                None,
                                vec![],
                                None
                            )
                        );
                    }
                | kDEFINED opt_nl tLPAREN2 expr rparen
                    {
                        $$ = Value::Node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Defined,
                                $<Token>1,
                                Some($<Token>3),
                                vec![ $<Node>4 ],
                                Some($<Token>4)
                            )
                        );
                    }
                | kNOT tLPAREN2 expr rparen
                    {
                        $$ = Value::Node(
                            self.builder.not_op(
                                $<Token>1,
                                Some($<Token>2),
                                Some($<Node>3),
                                Some($<Token>4)
                            )
                        );
                    }
                | kNOT tLPAREN2 rparen
                    {
                        $$ = Value::Node(
                            self.builder.not_op(
                                $<Token>1,
                                Some($<Token>2),
                                None,
                                Some($<Token>3)
                            )
                        );
                    }
                | fcall brace_block
                    {
                        let method_call = self.builder.call_method(
                            None,
                            None,
                            Some($<Token>1),
                            None,
                            vec![],
                            None
                        );
                        let (begin_t, args, body, end_t) = $<BraceBlock>2;

                        $$ = Value::Node(
                            self.builder.block(
                                method_call,
                                begin_t,
                                args,
                                body,
                                end_t
                            )
                        );
                    }
                | method_call
                | method_call brace_block
                    {
                        let (begin_t, args, body, end_t) = $<BraceBlock>2;
                        $$ = Value::Node(
                            self.builder.block(
                                $<Node>1,
                                begin_t,
                                args,
                                body,
                                end_t
                            )
                        );
                    }
                | lambda
                | k_if expr_value then
                  compstmt
                  if_tail
                  k_end
                    {
                        let (else_t, else_) = match $<IfTail>5 {
                            Some((else_t, else_)) => (Some(else_t), else_),
                            None => (None, None)
                        };

                        $$ = Value::Node(
                            self.builder.condition(
                                $<Token>1,
                                $<Node>2,
                                $<Token>3,
                                $<MaybeNode>4,
                                else_t,
                                else_,
                                Some($<Token>6)
                            )
                        );
                    }
                | k_unless expr_value then
                  compstmt
                  opt_else
                  k_end
                    {
                        let (else_t, else_) = match $<IfTail>5 {
                            Some((else_t, else_)) => (Some(else_t), else_),
                            None => (None, None)
                        };

                        $$ = Value::Node(
                            self.builder.condition(
                                $<Token>1,
                                $<Node>2,
                                $<Token>3,
                                else_,
                                else_t,
                                $<MaybeNode>4,
                                Some($<Token>6)
                            )
                        );
                    }
                | k_while expr_value_do
                  compstmt
                  k_end
                    {
                        let (cond, do_t) = $<ExprValueDo>2;
                        $$ = Value::Node(
                            self.builder.loop_(
                                LoopType::While,
                                $<Token>1,
                                cond,
                                do_t,
                                $<MaybeNode>3,
                                $<Token>4
                            )
                        );
                    }
                | k_until expr_value_do
                  compstmt
                  k_end
                    {
                        let (cond, do_t) = $<ExprValueDo>2;
                        $$ = Value::Node(
                            self.builder.loop_(
                                LoopType::While,
                                $<Token>1,
                                cond,
                                do_t,
                                $<MaybeNode>3,
                                $<Token>4
                            )
                        );
                    }
                | k_case expr_value opt_terms
                    {
                        // TODO: there's a warning that wq/parser doesn't trigger,
                        // search for `p->case_labels`
                    }
                  case_body
                  k_end
                    {
                        let (when_bodies, else_) = $<CaseBody>5;
                        let (else_t, else_body) = match else_ {
                            Some((else_t, else_body)) => (Some(else_t), else_body),
                            None => (None, None)
                        };

                        $$ = Value::Node(
                            self.builder.case(
                                $<Token>1,
                                Some($<Node>2),
                                when_bodies,
                                else_t,
                                else_body,
                                $<Token>6
                            )
                        );
                    }
                | k_case opt_terms
                    {
                        // TODO: there's a warning that wq/parser doesn't trigger,
                        // search for `p->case_labels`
                    }
                  case_body
                  k_end
                    {
                        let (when_bodies, else_) = $<CaseBody>4;
                        let (else_t, else_body) = match else_ {
                            Some((else_t, else_body)) => (Some(else_t), else_body),
                            None => (None, None)
                        };

                        $$ = Value::Node(
                            self.builder.case(
                                $<Token>1,
                                None,
                                when_bodies,
                                else_t,
                                else_body,
                                $<Token>5
                            )
                        );
                    }
                | k_case expr_value opt_terms
                  p_case_body
                  k_end
                    {
                        let (in_bodies, else_) = $<CaseBody>4;
                        let (else_t, else_body) = match else_ {
                            Some((else_t, else_body)) => (Some(else_t), else_body),
                            None => (None, None)
                        };

                        $$ = Value::Node(
                            self.builder.case_match(
                                $<Token>1,
                                $<Node>2,
                                in_bodies,
                                else_t,
                                else_body,
                                $<Token>5
                            )
                        );
                    }
                | k_for for_var kIN expr_value_do
                  compstmt
                  k_end
                    {
                        let (iteratee, do_t) = $<ExprValueDo>4;
                        $$ = Value::Node(
                            self.builder.for_(
                                $<Token>1,
                                $<Node>2,
                                $<Token>3,
                                iteratee,
                                do_t,
                                $<MaybeNode>5,
                                $<Token>6
                            )
                        );
                    }
                | k_class cpath superclass
                    {
                        self.static_env.extend_static();
                        self.yylexer.cmdarg_push(false);
                        self.yylexer.cond_push(false);
                        self.yylexer.p.context.push(ContextItem::Class);
                    }
                  bodystmt
                  k_end
                    {
                        // unless @context.class_definition_allowed?
                        //     diagnostic :error, :class_in_def, None, val[0]
                        // end

                        let (lt_t, superclass) = match $<Superclass>3 {
                            Some((lt_t, superclass)) => (Some(lt_t), Some(superclass)),
                            None => (None, None)
                        };

                        $$ = Value::Node(
                            self.builder.def_class(
                                $<Token>1,
                                $<Node>2,
                                lt_t,
                                superclass,
                                $<MaybeNode>5,
                                $<Token>6
                            )
                        );

                        self.yylexer.cmdarg_pop();
                        self.yylexer.cond_pop();
                        self.static_env.unextend();
                        self.yylexer.p.context.pop();
                    }
                | k_class tLSHFT expr
                    {
                        self.static_env.extend_static();
                        self.yylexer.cmdarg_push(false);
                        self.yylexer.cond_push(false);
                        self.yylexer.p.context.push(ContextItem::Sclass);
                    }
                  term
                  bodystmt
                  k_end
                    {
                        $$ = Value::Node(
                            self.builder.def_sclass(
                                $<Token>1,
                                $<Token>2,
                                $<Node>3,
                                $<MaybeNode>6,
                                $<Token>7
                            )
                        );

                        self.yylexer.cmdarg_pop();
                        self.yylexer.cond_pop();
                        self.static_env.unextend();
                        self.yylexer.p.context.pop();
                    }
                | k_module cpath
                    {
                        self.static_env.extend_static();
                        self.yylexer.cmdarg_push(false);
                        self.yylexer.p.context.push(ContextItem::Module);
                    }
                  bodystmt
                  k_end
                    {
                        // unless @context.module_definition_allowed?
                        //     diagnostic :error, :module_in_def, None, val[0]
                        // end

                        $$ = Value::Node(
                            self.builder.def_module(
                                $<Token>1,
                                $<Node>2,
                                $<MaybeNode>4,
                                $<Token>5
                            )
                        );

                        self.yylexer.cmdarg_pop();
                        self.static_env.unextend();
                        self.yylexer.p.context.pop();
                    }
                | defn_head
                  f_arglist
                  bodystmt
                  k_end
                    {
                        let (def_t, name_t) = $<DefnHead>1;

                        $$ = Value::Node(
                            self.builder.def_method(
                                def_t,
                                name_t,
                                $<MaybeNode>2,
                                $<MaybeNode>3,
                                $<Token>4
                            )
                        );

                        self.yylexer.cmdarg_pop();
                        self.yylexer.cond_pop();
                        self.static_env.unextend();
                        self.yylexer.p.context.pop();
                        self.current_arg_stack.pop();
                    }
                | defs_head
                  f_arglist
                  bodystmt
                  k_end
                    {
                        let (def_t, definee, dot_t, name_t) = $<DefsHead>1;

                        $$ = Value::Node(
                            self.builder.def_singleton(
                                def_t,
                                definee,
                                dot_t,
                                name_t,
                                $<MaybeNode>2,
                                $<MaybeNode>3,
                                $<Token>4
                            )
                        );

                        self.yylexer.cmdarg_pop();
                        self.yylexer.cond_pop();
                        self.static_env.unextend();
                        self.yylexer.p.context.pop();
                        self.current_arg_stack.pop();
                    }
                | kBREAK
                    {
                        $$ = Value::Node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Break,
                                $<Token>1,
                                None,
                                vec![],
                                None
                            )
                        );
                    }
                | kNEXT
                    {
                        $$ = Value::Node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Next,
                                $<Token>1,
                                None,
                                vec![],
                                None
                            )
                        );
                    }
                | kREDO
                    {
                        $$ = Value::Node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Redo,
                                $<Token>1,
                                None,
                                vec![],
                                None
                            )
                        );
                    }
                | kRETRY
                    {
                        $$ = Value::Node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Retry,
                                $<Token>1,
                                None,
                                vec![],
                                None
                            )
                        );
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
                        //     diagnostic :error, :invalid_return, None, val[0]
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
                    {
                        $$ = Value::IfTail($<OptElse>1);
                    }
                | k_elsif expr_value then
                  compstmt
                  if_tail
                    {
                        let (else_t, else_body) = match $<IfTail>5 {
                            Some((else_t, else_body)) => ( Some(else_t), else_body ),
                            None => (None, None)
                        };

                        let elsif_t = $<Token>1;

                        $$ = Value::IfTail(
                            Some((
                                elsif_t.clone(),
                                Some(
                                    self.builder.condition(
                                        elsif_t.clone(),
                                        $<Node>2,
                                        $<Token>3,
                                        $<MaybeNode>4,
                                        else_t,
                                        else_body,
                                        None
                                    )
                                )
                            ))
                        );
                    }
                ;

        opt_else: none
                    {
                        $$ = Value::OptElse(None);
                    }
                | k_else compstmt
                    {
                        let token = $<Token>1;
                        let node  = $<MaybeNode>2;
                        $$ = Value::OptElse( Some((token, node)) );
                    }
                ;

         for_var: lhs
                | mlhs
                ;

          f_marg: f_norm_arg
                    {
                        $$ = Value::Node(
                            self.builder.arg($<Token>1)
                        );
                    }
                | tLPAREN f_margs rparen
                    {
                        $$ = Value::Node(
                            self.builder.multi_lhs(
                                Some($<Token>1),
                                $<NodeList>2,
                                Some($<Token>3)
                            )
                        );
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
                        $$ = Value::Node(
                            self.builder.restarg($<Token>1, Some($<Token>2))
                        );
                    }
                | tSTAR
                    {
                        $$ = Value::Node(
                            self.builder.restarg($<Token>1, None)
                        );
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
                        let mut f_arg = $<NodeList>1;
                        let opt_block_args_tail = $<NodeList>2;
                        let nodes: Vec<Node>;

                        if opt_block_args_tail.is_empty() && f_arg.len() == 1 {
                            nodes = vec![ self.builder.procarg0(f_arg.pop().unwrap()) ];
                        } else {
                            nodes = [ f_arg, opt_block_args_tail ].concat();
                        }

                        $$ = Value::NodeList(nodes);
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
                        $$ = Value::MaybeNode(
                            self.builder.args(None, vec![], None)
                        );
                    }
                | block_param_def
                    {
                        self.yylexer.set_lex_state(EXPR_VALUE);
                        $$ = $<RAW>1;
                    }
                ;

 block_param_def: tPIPE opt_bv_decl tPIPE
                    {
                        // @max_numparam_stack.has_ordinary_params!
                        self.current_arg_stack.set(None);
                        $$ = Value::MaybeNode(
                            self.builder.args(
                                Some($<Token>1),
                                $<NodeList>2,
                                Some($<Token>3)
                            )
                        );
                    }
                | tPIPE block_param opt_bv_decl tPIPE
                    {
                        // @max_numparam_stack.has_ordinary_params!
                        self.current_arg_stack.set(None);

                        $$ = Value::MaybeNode(
                            self.builder.args(
                                Some($<Token>1),
                                [ $<NodeList>2, $<NodeList>3 ].concat(),
                                Some($<Token>4)
                            )
                        );
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
                        let ident_t = $<Token>1;
                        self.static_env.declare(&ident_t.1);
                        $$ = Value::Node(
                            self.builder.shadowarg(ident_t)
                        );
                    }
                | f_bad_arg
                    {
                        $$ = Value::None;
                    }
                ;

          lambda: tLAMBDA
                    {
                        self.static_env.extend_dynamic();
                        // @max_numparam_stack.push
                        self.yylexer.p.context.push(ContextItem::Lambda);
                    }
                  f_larglist
                    {
                        self.yylexer.p.context.pop();
                        self.yylexer.cmdarg_push(false);
                    }
                  lambda_body
                    {
                        let lambda_call = self.builder.call_lambda($<Token>1);
                        // args = @max_numparam_stack.has_numparams? ? self.builder.numargs(@max_numparam_stack.top) : val[2]
                        let args = $<MaybeNode>3;
                        let (begin_t, body, end_t) = $<LambdaBody>5;

                        // @max_numparam_stack.pop
                        self.static_env.unextend();
                        self.yylexer.cmdarg_pop();

                        $$ = Value::Node(
                            self.builder.block(
                                lambda_call,
                                begin_t,
                                args,
                                body,
                                end_t
                            )
                        );
                    }
                ;

      f_larglist: tLPAREN2 f_args opt_bv_decl tRPAREN
                    {
                        // @max_numparam_stack.has_ordinary_params!
                        $$ = Value::MaybeNode(
                            self.builder.args(
                                Some($<Token>1),
                                vec![ $<NodeList>2, $<NodeList>3 ].concat(),
                                Some($<Token>4)
                            )
                        );
                    }
                | f_args
                    {
                        // if val[0].any?
                        //     @max_numparam_stack.has_ordinary_params!
                        // end
                        $$ = Value::MaybeNode(
                            self.builder.args(None, $<NodeList>1, None)
                        );
                    }
                ;

     lambda_body: tLAMBEG
                    {
                        self.yylexer.p.context.push(ContextItem::Lambda);
                    }
                  compstmt tRCURLY
                    {
                        self.yylexer.p.context.pop();
                        $$ = Value::LambdaBody(( $<Token>1, $<MaybeNode>3, $<Token>4 ));
                    }
                | kDO_LAMBDA
                    {
                        self.yylexer.p.context.push(ContextItem::Lambda);
                    }
                  bodystmt k_end
                    {
                        self.yylexer.p.context.pop();
                        $$ = Value::LambdaBody(( $<Token>1, $<MaybeNode>3, $<Token>4 ));
                    }
                ;

        do_block: k_do_block
                    {
                        self.yylexer.p.context.push(ContextItem::Block);
                    }
                  do_body k_end
                    {
                        let (args, body) = $<DoBody>3;
                        self.yylexer.p.context.pop();
                        $$ = Value::DoBlock(( $<Token>1, args, body, $<Token>4 ));
                    }
                ;

      block_call: command do_block
                    {
                        let (begin_t, block_args, body, end_t) = $<DoBlock>2;
                        $$ = Value::Node(
                            self.builder.block(
                                $<Node>1,
                                begin_t,
                                block_args,
                                body,
                                end_t
                            )
                        );
                    }
                | block_call call_op2 operation2 opt_paren_args
                    {
                        let (lparen_t, args, rparen_t) = $<OptParenArgs>4;
                        $$ = Value::Node(
                            self.builder.call_method(
                                Some($<Node>1),
                                Some($<Token>2),
                                Some($<Token>3),
                                lparen_t,
                                args,
                                rparen_t
                            )
                        );
                    }
                | block_call call_op2 operation2 opt_paren_args brace_block
                    {
                        let (lparen_t, args, rparen_t) = $<OptParenArgs>4;
                        let method_call = self.builder.call_method(
                            Some($<Node>1),
                            Some($<Token>2),
                            Some($<Token>3),
                            lparen_t,
                            args,
                            rparen_t
                        );

                        let (begin_t, args, body, end_t) = $<BraceBlock>5;
                        $$ = Value::Node(
                            self.builder.block(
                                method_call,
                                begin_t,
                                args,
                                body,
                                end_t
                            )
                        );
                    }
                | block_call call_op2 operation2 command_args do_block
                    {
                        let method_call = self.builder.call_method(
                            Some($<Node>1),
                            Some($<Token>2),
                            Some($<Token>3),
                            None,
                            $<NodeList>4,
                            None
                        );

                        let (begin_t, args, body, end_t) = $<BraceBlock>5;
                        $$ = Value::Node(
                            self.builder.block(
                                method_call,
                                begin_t,
                                args,
                                body,
                                end_t
                            )
                        );
                    }
                ;

     method_call: fcall paren_args
                    {
                        let (lparen_t, args, rparen_t) = $<ParenArgs>2;

                        $$ = Value::Node(
                            self.builder.call_method(
                                None,
                                None,
                                Some($<Token>1),
                                Some(lparen_t),
                                args,
                                Some(rparen_t)
                            )
                        );
                    }
                | primary_value call_op operation2 opt_paren_args
                    {
                        let (lparen_t, args, rparen_t) = $<OptParenArgs>4;

                        $$ = Value::Node(
                            self.builder.call_method(
                                Some($<Node>1),
                                Some($<Token>2),
                                Some($<Token>3),
                                lparen_t,
                                args,
                                rparen_t
                            )
                        );
                    }
                | primary_value tCOLON2 operation2 paren_args
                    {
                        let (lparen_t, args, rparen_t) = $<ParenArgs>4;

                        $$ = Value::Node(
                            self.builder.call_method(
                                Some($<Node>1),
                                Some($<Token>2),
                                Some($<Token>3),
                                Some(lparen_t),
                                args,
                                Some(rparen_t)
                            )
                        );
                    }
                | primary_value tCOLON2 operation3
                    {
                        $$ = Value::Node(
                            self.builder.call_method(
                                Some($<Node>1),
                                Some($<Token>2),
                                Some($<Token>3),
                                None,
                                vec![],
                                None
                            )
                        );
                    }
                | primary_value call_op paren_args
                    {
                        let (lparen_t, args, rparen_t) = $<ParenArgs>3;

                        $$ = Value::Node(
                            self.builder.call_method(
                                Some($<Node>1),
                                Some($<Token>2),
                                None,
                                Some(lparen_t),
                                args,
                                Some(rparen_t)
                            )
                        );
                    }
                | primary_value tCOLON2 paren_args
                    {
                        let (lparen_t, args, rparen_t) = $<ParenArgs>3;

                        $$ = Value::Node(
                            self.builder.call_method(
                                Some($<Node>1),
                                Some($<Token>2),
                                None,
                                Some(lparen_t),
                                args,
                                Some(rparen_t)
                            )
                        );
                    }
                | kSUPER paren_args
                    {
                        let (lparen_t, args, rparen_t) = $<ParenArgs>2;

                        $$ = Value::Node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Super,
                                $<Token>1,
                                Some(lparen_t),
                                args,
                                Some(rparen_t)
                            )
                        );
                    }
                | kSUPER
                    {
                        $$ = Value::Node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Super,
                                $<Token>1,
                                None,
                                vec![],
                                None
                            )
                        );
                    }
                | primary_value tLBRACK2 opt_call_args rbracket
                    {
                        $$ = Value::Node(
                            self.builder.index(
                                $<Node>1,
                                $<Token>2,
                                $<NodeList>3,
                                $<Token>4
                            )
                        );
                    }
                ;

     brace_block: tLCURLY
                    {
                        self.yylexer.p.context.push(ContextItem::Block);
                    }
                  brace_body tRCURLY
                    {
                        let (args, body) = $<BraceBody>3;
                        self.yylexer.p.context.pop();

                        $$ = Value::BraceBlock(( $<Token>1, args, body, $<Token>4 ));
                    }
                | k_do
                    {
                        self.yylexer.p.context.push(ContextItem::Block);
                    }
                  do_body k_end
                    {
                        let (args, body) = $<DoBody>3;
                        self.yylexer.p.context.pop();

                        $$ = Value::BraceBlock(( $<Token>1, args, body, $<Token>4 ));
                    }
                ;

      brace_body:   {
                        self.static_env.extend_dynamic();
                        // @max_numparam_stack.push
                    }
                  opt_block_param compstmt
                    {
                        // args = @max_numparam_stack.has_numparams? ? self.builder.numargs(@max_numparam_stack.top) : val[1]

                        // @max_numparam_stack.pop
                        self.static_env.unextend();

                        $$ = Value::BraceBody(( $<MaybeNode>2, $<MaybeNode>3 ));
                    }
                ;

         do_body:   {
                        self.static_env.extend_dynamic();
                        // @max_numparam_stack.push
                        self.yylexer.cmdarg_push(false);
                    }
                  opt_block_param bodystmt
                    {
                        // args = @max_numparam_stack.has_numparams? ? self.builder.numargs(@max_numparam_stack.top) : val[2]

                        // @max_numparam_stack.pop
                        self.static_env.unextend();
                        self.yylexer.cmdarg_pop();

                        $$ = Value::DoBody(( $<MaybeNode>2, $<MaybeNode>3 ));
                    }
                ;

       case_args: arg_value
                    {
                        $$ = Value::NodeList( vec![ $<Node>1 ] );
                    }
                | tSTAR arg_value
                    {
                        $$ = Value::NodeList(
                            vec![
                                self.builder.splat($<Token>1, Some($<Node>2))
                            ]
                        );
                    }
                | case_args tCOMMA arg_value
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( $<Node>3 );
                        $$ = Value::NodeList(nodes);
                    }
                | case_args tCOMMA tSTAR arg_value
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( self.builder.splat($<Token>3, Some($<Node>4)) );
                        $$ = Value::NodeList( nodes );
                    }
                ;

       case_body: k_when case_args then
                  compstmt
                  cases
                    {
                        let when = self.builder.when($<Token>1, $<NodeList>2, $<Token>3, $<MaybeNode>4);
                        let (whens, else_) = $<Cases>5;
                        let whens = [ vec![when], whens ].concat();
                        $$ = Value::CaseBody(( whens, else_ ));
                    }
                ;

           cases: opt_else
                    {
                        $$ = Value::Cases(( vec![], $<OptElse>1 ));
                    }
                | case_body
                    {
                        let (whens, _) = $<CaseBody>1;
                        $$ = Value::Cases(( whens, None ));
                    }
                ;

     p_case_body: kIN
                    {
                        self.yylexer.set_lex_state(EXPR_BEG);
                        self.yylexer.p.command_start = false;
                        // @pattern_variables.push
                        // @pattern_hash_keys.push

                        $<Bool>$ = Value::Bool(self.yylexer.p.in_kwarg);
                        self.yylexer.p.in_kwarg = true
                    }
                  p_top_expr then
                    {
                        self.yylexer.p.in_kwarg = $<Bool>2;
                    }
                  compstmt
                  p_cases
                    {
                        let (whens, else_) = $<PCases>7;
                        let (pattern, guard) = $<PTopExpr>3;

                        let whens = [
                            vec![
                                self.builder.in_pattern(
                                    $<Token>1,
                                    pattern,
                                    guard,
                                    $<Token>4,
                                    $<MaybeNode>6
                                )
                            ],
                            whens
                        ].concat();
                        $$ = Value::PCaseBody(( whens, else_ ));
                    }
                ;

         p_cases: opt_else
                    {
                        $$ = Value::PCases(( vec![], $<OptElse>1 ));
                    }
                | p_case_body
                    {
                        let (whens, _) = $<PCaseBody>1;
                        $$ = Value::PCases(( whens, None ));
                    }
                ;

      p_top_expr: p_top_expr_body
                    {
                        $$ = Value::PTopExpr(( $<Node>1, None ));
                    }
                | p_top_expr_body kIF_MOD expr_value
                    {
                        let guard = self.builder.if_guard($<Token>2, $<Node>3);
                        $$ = Value::PTopExpr(( $<Node>1, Some(guard) ));
                    }
                | p_top_expr_body kUNLESS_MOD expr_value
                    {
                        let guard = self.builder.unless_guard($<Token>2, $<Node>3);
                        $$ = Value::PTopExpr(( $<Node>1, Some(guard) ));
                    }
                ;

 p_top_expr_body: p_expr
                | p_expr tCOMMA
                    {
                        // array patterns that end with comma
                        // like 1, 2,
                        // must be emitted as `array_pattern_with_tail`
                        let item = self.builder.match_with_trailing_comma($<Node>1, $<Token>2);
                        $$ = Value::Node(
                            self.builder.array_pattern(None, vec![ item ], None)
                        );
                    }
                | p_expr tCOMMA p_args
                    {
                        let items = [ $<NodeList>1, $<NodeList>3 ].concat();
                        $$ = Value::Node(
                            self.builder.array_pattern(None, items, None)
                        );
                    }
                | p_find
                    {
                        $$ = Value::Node(
                            self.builder.find_pattern(None, $<NodeList>1, None)
                        );
                    }
                | p_args_tail
                    {
                        $$ = Value::Node(
                            self.builder.array_pattern(None, $<NodeList>1, None)
                        );
                    }
                | p_kwargs
                    {
                        $$ = Value::Node(
                            self.builder.hash_pattern(None, $<NodeList>1, None)
                        );
                    }
                ;

          p_expr: p_as
                ;

            p_as: p_expr tASSOC p_variable
                    {
                        $$ = Value::Node(
                            self.builder.match_as(
                                $<Node>1,
                                $<Token>2,
                                $<Node>3
                            )
                        );
                    }
                | p_alt
                ;

           p_alt: p_alt tPIPE p_expr_basic
                    {
                        $$ = Value::Node(
                            self.builder.match_alt(
                                $<Node>1,
                                $<Token>2,
                                $<Node>3
                            )
                        );
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
                        let pattern = self.builder.array_pattern(None, $<NodeList>3, None);
                        $$ = Value::Node(
                            self.builder.const_pattern(
                                $<Node>1,
                                $<Token>2,
                                pattern,
                                $<Token>4
                            )
                        );
                    }
                | p_const p_lparen p_find rparen
                    {
                        // @pattern_hash_keys.pop
                        let pattern = self.builder.find_pattern(None, $<NodeList>3, None);
                        $$ = Value::Node(
                            self.builder.const_pattern(
                                $<Node>1,
                                $<Token>2,
                                pattern,
                                $<Token>4
                            )
                        );
                    }
                | p_const p_lparen p_kwargs rparen
                    {
                        // @pattern_hash_keys.pop
                        let pattern = self.builder.hash_pattern(None, $<NodeList>3, None);
                        $$ = Value::Node(
                            self.builder.const_pattern(
                                $<Node>1,
                                $<Token>2,
                                pattern,
                                $<Token>4
                            )
                        );
                    }
                | p_const tLPAREN2 rparen
                    {
                        let lparen = $<Token>2;
                        let rparen = $<Token>3;
                        let pattern = self.builder.array_pattern(Some(lparen.clone()), vec![], Some(rparen.clone()));
                        $$ = Value::Node(
                            self.builder.const_pattern(
                                $<Node>1,
                                lparen,
                                pattern,
                                rparen
                            )
                        );
                    }
                | p_const p_lbracket p_args rbracket
                    {
                        // @pattern_hash_keys.pop
                        let pattern = self.builder.array_pattern(None, $<NodeList>3, None);
                        $$ = Value::Node(
                            self.builder.const_pattern(
                                $<Node>1,
                                $<Token>2,
                                pattern,
                                $<Token>4
                            )
                        );
                    }
                | p_const p_lbracket p_find rbracket
                    {
                        // @pattern_hash_keys.pop
                        let pattern = self.builder.find_pattern(None, $<NodeList>3, None);
                        $$ = Value::Node(
                            self.builder.const_pattern(
                                $<Node>1,
                                $<Token>2,
                                pattern,
                                $<Token>4
                            )
                        );
                    }
                | p_const p_lbracket p_kwargs rbracket
                    {
                        // @pattern_hash_keys.pop
                        let pattern = self.builder.hash_pattern(None, $<NodeList>3, None);
                        $$ = Value::Node(
                            self.builder.const_pattern(
                                $<Node>1,
                                $<Token>2,
                                pattern,
                                $<Token>4
                            )
                        );
                    }
                | p_const tLBRACK2 rbracket
                    {
                        let lparen = $<Token>2;
                        let rparen = $<Token>3;
                        let pattern = self.builder.array_pattern(Some(lparen.clone()), vec![], Some(rparen.clone()));
                        $$ = Value::Node(
                            self.builder.const_pattern(
                                $<Node>1,
                                lparen,
                                pattern,
                                rparen
                            )
                        );
                    }
                | tLBRACK p_args rbracket
                    {
                        $$ = Value::Node(
                            self.builder.array_pattern(
                                Some($<Token>1),
                                $<NodeList>2,
                                Some($<Token>3)
                            )
                        );
                    }
                | tLBRACK p_find rbracket
                    {
                        $$ = Value::Node(
                            self.builder.find_pattern(
                                Some($<Token>1),
                                $<NodeList>2,
                                Some($<Token>3)
                            )
                        );
                    }
                | tLBRACK rbracket
                    {
                        $$ = Value::Node(
                            self.builder.array_pattern(
                                Some($<Token>1),
                                vec![],
                                Some($<Token>2)
                            )
                        );
                    }
                | tLBRACE
                    {
                        // @pattern_hash_keys.push
                        $<Bool>$ = Value::Bool(self.yylexer.p.in_kwarg);
                        self.yylexer.p.in_kwarg = false;
                    }
                  p_kwargs rbrace
                    {
                        // @pattern_hash_keys.pop
                        self.yylexer.p.in_kwarg = $<Bool>2;
                        $$ = Value::Node(
                            self.builder.hash_pattern(
                                Some($<Token>1),
                                $<NodeList>3,
                                Some($<Token>4)
                            )
                        );
                    }
                | tLBRACE rbrace
                    {
                        $$ = Value::Node(
                            self.builder.hash_pattern(
                                Some($<Token>1),
                                vec![],
                                Some($<Token>2),
                            )
                        );
                    }
                | tLPAREN
                    {
                        // @pattern_hash_keys.push
                    }
                  p_expr rparen
                    {
                        // @pattern_hash_keys.pop
                        $$ = Value::Node(
                            self.builder.begin(
                                $<Token>1,
                                Some($<Node>3),
                                $<Token>4
                            )
                        );
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
                        let match_rest = self.builder.match_rest($<Token>2, Some($<Token>3));
                        let nodes = [ $<NodeList>1, vec![ match_rest ] ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | p_args_head tSTAR tIDENTIFIER tCOMMA p_args_post
                    {
                        let match_rest = self.builder.match_rest($<Token>2, Some($<Token>3));
                        let nodes = [ $<NodeList>1, vec![ match_rest ], $<NodeList>5 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | p_args_head tSTAR
                    {
                        let match_rest = self.builder.match_rest($<Token>2, None);
                        let nodes = [ $<NodeList>1, vec![ match_rest ] ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | p_args_head tSTAR tCOMMA p_args_post
                    {
                        let match_rest = self.builder.match_rest($<Token>2, None);
                        let nodes = [ $<NodeList>1, vec![ match_rest ], $<NodeList>4 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | p_args_tail
                ;

     p_args_head: p_arg tCOMMA
                    {
                        // array patterns that end with comma
                        // like [1, 2,]
                        // must be emitted as `array_pattern_with_tail`
                        let item = self.builder.match_with_trailing_comma($<Node>1, $<Token>2);
                        $$ = Value::NodeList( vec![ item ] );
                    }
                | p_args_head p_arg tCOMMA
                    {
                        // array patterns that end with comma
                        // like [1, 2,]
                        // must be emitted as `array_pattern_with_tail`
                        let last_item = self.builder.match_with_trailing_comma($<Node>2, $<Token>3);
                        let nodes = [ $<NodeList>1, vec![ last_item ] ].concat();
                        $$ = Value::NodeList(nodes);
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
                        $$ = Value::Node(
                            self.builder.match_rest($<Token>1, Some($<Token>2))
                        );
                    }
                | tSTAR
                    {
                        $$ = Value::Node(
                            self.builder.match_rest($<Token>1, None)
                        );
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
                        $$ = Value::Node(
                            self.builder.match_pair(
                                $<PKwLabel>1,
                                $<Node>2
                            )
                        );
                    }
                | p_kw_label
                    {
                        $$ = Value::Node(
                            self.builder.match_label(
                                $<PKwLabel>1,
                            )
                        );
                    }
                ;

      p_kw_label: tLABEL
                    {
                        $$ = Value::PKwLabel(
                            PKwLabel::PlainLabel($<Token>1)
                        );
                    }
                | tSTRING_BEG string_contents tLABEL_END
                    {
                        $$ = Value::PKwLabel(
                            PKwLabel::QuotedLabel( ($<Token>1, $<NodeList>2, $<Token>3) )
                        );
                    }
                ;

        p_kwrest: kwrest_mark tIDENTIFIER
                    {
                        $$ = Value::NodeList(
                            vec![
                                self.builder.match_rest($<Token>1, Some($<Token>2))
                            ]
                        );
                    }
                | kwrest_mark
                    {
                        $$ = Value::NodeList(
                            vec![
                                self.builder.match_rest($<Token>1, None)
                            ]
                        );
                    }
                ;

      p_kwnorest: kwrest_mark kNIL
                    {
                        $$ = Value::NodeList(
                            vec![
                                self.builder.match_nil_pattern($<Token>1, $<Token>2)
                            ]
                        );
                    }
                ;

    p_any_kwrest: p_kwrest
                | p_kwnorest
                ;

         p_value: p_primitive
                | p_primitive tDOT2 p_primitive
                    {
                        $$ = Value::Node(
                            self.builder.range_inclusive(
                                Some($<Node>1),
                                $<Token>2,
                                Some($<Node>3)
                            )
                        );
                    }
                | p_primitive tDOT3 p_primitive
                    {
                        $$ = Value::Node(
                            self.builder.range_exclusive(
                                Some($<Node>1),
                                $<Token>2,
                                Some($<Node>3)
                            )
                        );
                    }
                | p_primitive tDOT2
                    {
                        $$ = Value::Node(
                            self.builder.range_inclusive(
                                Some($<Node>1),
                                $<Token>2,
                                None
                            )
                        );
                    }
                | p_primitive tDOT3
                    {
                        $$ = Value::Node(
                            self.builder.range_exclusive(
                                Some($<Node>1),
                                $<Token>2,
                                None
                            )
                        );
                    }
                | p_variable
                | p_var_ref
                | p_const
                | tBDOT2 p_primitive
                    {
                        $$ = Value::Node(
                            self.builder.range_inclusive(
                                None,
                                $<Token>1,
                                Some($<Node>2)
                            )
                        );
                    }
                | tBDOT3 p_primitive
                    {
                        $$ = Value::Node(
                            self.builder.range_exclusive(
                                None,
                                $<Token>1,
                                Some($<Node>2)
                            )
                        );
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
                        $$ = Value::Node(
                            self.builder.accessible($<Node>1)
                        );
                    }
                | lambda
                ;

      p_variable: tIDENTIFIER
                    {
                        $$ = Value::Node(
                            self.builder.match_var($<Token>1)
                        );
                    }
                ;

       p_var_ref: tCARET tIDENTIFIER
                    {
                        let ident_t = $<Token>2;

                        // name = val[1][0]
                        // unless static_env.declared?(name)
                        //     diagnostic :error, :undefined_lvar, { :name => name }, val[1]
                        // end

                        let lvar = self.builder.accessible(self.builder.lvar(ident_t.clone()));
                        $$ = Value::Node(
                            self.builder.pin($<Token>1, lvar)
                        );
                    }
                ;

         p_const: tCOLON3 cname
                    {
                        $$ = Value::Node(
                            self.builder.const_global($<Token>1, $<Token>2)
                        );
                    }
                | p_const tCOLON2 cname
                    {
                        $$ = Value::Node(
                            self.builder.const_fetch(
                                $<Node>1,
                                $<Token>2,
                                $<Token>3,
                            )
                        );
                    }
                | tCONSTANT
                    {
                        $$ = Value::Node(self.builder.const_($<Token>1));
                    }
                ;

      opt_rescue: k_rescue exc_list exc_var then
                  compstmt
                  opt_rescue
                    {
                        let (assoc_t, exc_var) = match $<ExcVar>3 {
                            Some((assoc_t, exc_var)) => ( Some(assoc_t), Some(exc_var) ),
                            None => (None, None)
                        };

                        let exc_list = $<NodeList>2;
                        let exc_list = if exc_list.is_empty() {
                            None
                        } else {
                            Some(self.builder.array(None, exc_list, None))
                        };

                        let rescue_body = self.builder.rescue_body(
                            $<Token>1,
                            exc_list,
                            assoc_t,
                            exc_var,
                            Some($<Token>4),
                            $<MaybeNode>5
                        );
                        let nodes = [ vec![rescue_body], $<NodeList>6 ].concat();

                        $$ = Value::NodeList(nodes);
                    }
                | none
                    {
                        $$ = Value::NodeList( vec![] );
                    }
                ;

        exc_list: arg_value
                    {
                        $$ = Value::NodeList(vec![ $<Node>1 ]);
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
                        $$ = Value::ExcVar( Some((token, node)) );
                    }
                | none
                    {
                        $$ = Value::ExcVar(None);
                    }
                ;

      opt_ensure: k_ensure compstmt
                    {
                        let token = $<Token>1;
                        let node = $<Node>2;
                        $$ = Value::OptEnsure( Some((token, node)) );
                    }
                | none
                    {
                        $$ = Value::OptEnsure(None);
                    }
                ;

         literal: numeric
                | symbol
                ;

         strings: string
                    {
                        $$ = Value::Node(
                            self.builder.string_compose(
                                None,
                                $<NodeList>1,
                                None
                            )
                        );
                    }
                ;

          string: tCHAR
                    {
                        $$ = Value::NodeList(
                            vec![
                                self.builder.character($<Token>1)
                            ]
                        );
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
                        let string = self.builder.string_compose(Some($<Token>1), $<NodeList>2, Some($<Token>3));
                        $$ = Value::Node(
                            self.builder.dedent_string(string, self.yylexer.p.heredoc_indent)
                        );
                    }
                ;

         xstring: tXSTRING_BEG xstring_contents tSTRING_END
                    {
                        let string = self.builder.xstring_compose($<Token>1, $<NodeList>2, $<Token>3);
                        $$ = Value::Node(
                            self.builder.dedent_string(string, self.yylexer.p.heredoc_indent)
                        );
                    }
                ;

          regexp: tREGEXP_BEG regexp_contents tREGEXP_END
                    {
                        let opts = self.builder.regexp_options($<Borrow:Token>3);
                        $$ = Value::Node(
                            self.builder.regexp_compose(
                                $<Token>1,
                                $<NodeList>2,
                                $<Token>3,
                                opts
                            )
                        );
                    }
                ;

           words: tWORDS_BEG tSPACE word_list tSTRING_END
                    {
                        $$ = Value::Node(
                            self.builder.words_compose(
                                $<Token>1,
                                $<NodeList>3,
                                $<Token>4
                            )
                        );
                    }
                ;

       word_list: /* none */
                    {
                        $$ = Value::NodeList( vec![] );

                    }
                | word_list word tSPACE
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push(
                            self.builder.word( $<NodeList>2 )
                        );
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
                        $$ = Value::Node(
                            self.builder.symbols_compose(
                                $<Token>1,
                                $<NodeList>3,
                                $<Token>4
                            )
                        );
                    }
                ;

     symbol_list: /* none */
                    {
                        $$ = Value::NodeList( vec![] );
                    }
                | symbol_list word tSPACE
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push(
                            self.builder.word( $<NodeList>2 )
                        );
                        $$ = Value::NodeList( nodes );
                    }
                ;

          qwords: tQWORDS_BEG tSPACE qword_list tSTRING_END
                    {
                        $$ = Value::Node(
                            self.builder.words_compose(
                                $<Token>1,
                                $<NodeList>3,
                                $<Token>4
                            )
                        );
                    }
                ;

        qsymbols: tQSYMBOLS_BEG tSPACE qsym_list tSTRING_END
                    {
                        $$ = Value::Node(
                            self.builder.symbols_compose(
                                $<Token>1,
                                $<NodeList>3,
                                $<Token>4
                            )
                        );
                    }
                ;

      qword_list: /* none */
                    {
                        $$ = Value::NodeList( vec![] );
                    }
                | qword_list tSTRING_CONTENT tSPACE
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push(
                            self.builder.string_internal( $<Token>2 )
                        );
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
                        nodes.push(
                            self.builder.symbol_internal( $<Token>2 )
                        );
                        $$ = Value::NodeList( nodes );
                    }
                ;

 string_contents: /* none */
                    {
                        $$ = Value::NodeList( vec![] );
                    }
                | string_contents string_content
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push($<Node>2);
                        $$ = Value::NodeList(nodes);
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
                        $$ = Value::Node(
                            self.builder.string_internal($<Token>1)
                        );
                    }
                | tSTRING_DVAR
                    {
                        // TODO: push terminal
                    }
                  string_dvar
                    {
                        $$ = $<RAW>3;
                    }
                | tSTRING_DBEG
                    {
                        // TODO: push terminal
                    }
                  compstmt tSTRING_DEND
                    {
                        $$ = Value::Node(
                            self.builder.begin(
                                $<Token>1,
                                $<MaybeNode>3,
                                $<Token>4
                            )
                        );
                    }
                ;

     string_dvar: tGVAR
                    {
                        $$ = Value::Node(self.builder.gvar($<Token>1));
                    }
                | tIVAR
                    {
                        $$ = Value::Node(self.builder.ivar($<Token>1));

                    }
                | tCVAR
                    {
                        $$ = Value::Node(self.builder.cvar($<Token>1));
                    }
                | backref
                ;

          symbol: ssym
                | dsym
                ;

            ssym: tSYMBEG sym
                    {
                        self.yylexer.set_lex_state(EXPR_END);
                        $$ = Value::Node(
                            self.builder.symbol($<Token>1, $<Token>2)
                        );
                    }
                ;

             sym: fname
                | tIVAR
                | tGVAR
                | tCVAR
                ;

            dsym: tSYMBEG string_contents tSTRING_END
                    {
                        self.yylexer.set_lex_state(EXPR_END);
                        $$ = Value::Node(
                            self.builder.symbol_compose($<Token>1, $<NodeList>2, $<Token>3)
                        );
                    }
                ;

         numeric: simple_numeric
                | tUMINUS_NUM simple_numeric   %prec tLOWEST
                    {
                        $$ = Value::Node(
                            self.builder.unary_num(
                                $<Token>1,
                                $<Node>2
                            )
                        );
                    }
                ;

  simple_numeric: tINTEGER
                    {
                        self.yylexer.set_lex_state(EXPR_END);
                        $$ = Value::Node(
                            self.builder.integer($<Token>1)
                        );
                    }
                | tFLOAT
                    {
                        self.yylexer.set_lex_state(EXPR_END);
                        $$ = Value::Node(
                            self.builder.float($<Token>1)
                        );
                    }
                | tRATIONAL
                    {
                        self.yylexer.set_lex_state(EXPR_END);
                        $$ = Value::Node(
                            self.builder.rational($<Token>1)
                        );
                    }
                | tIMAGINARY
                    {
                        self.yylexer.set_lex_state(EXPR_END);
                        $$ = Value::Node(
                            self.builder.complex($<Token>1)
                        );
                    }
                ;

   user_variable: tIDENTIFIER
                    {
                        $$ = Value::Node(
                            self.builder.lvar($<Token>1)
                        );
                    }
                | tIVAR
                    {
                        $$ = Value::Node(
                            self.builder.ivar($<Token>1)
                        );
                    }
                | tGVAR
                    {
                        $$ = Value::Node(
                            self.builder.gvar($<Token>1)
                        );
                    }
                | tCONSTANT
                    {
                        $$ = Value::Node(
                            self.builder.const_($<Token>1)
                        );
                    }
                | tCVAR
                    {
                        $$ = Value::Node(
                            self.builder.cvar($<Token>1)
                        );
                    }
                ;

keyword_variable: kNIL
                    {
                        $$ = Value::Node(
                            self.builder.nil($<Token>1)
                        );
                    }
                | kSELF
                    {
                        $$ = Value::Node(
                            self.builder.self_($<Token>1)
                        );
                    }
                | kTRUE
                    {
                        $$ = Value::Node(
                            self.builder.true_($<Token>1)
                        );
                    }
                | kFALSE
                    {
                        $$ = Value::Node(
                            self.builder.false_($<Token>1)
                        );
                    }
                | k__FILE__
                    {
                        $$ = Value::Node(
                            self.builder.__file__($<Token>1)
                        );
                    }
                | k__LINE__
                    {
                        $$ = Value::Node(
                            self.builder.__line__($<Token>1)
                        );
                    }
                | k__ENCODING__
                    {
                        $$ = Value::Node(
                            self.builder.__encoding__($<Token>1)
                        );
                    }
                ;

         var_ref: user_variable
                    {
                        // FIXME: error handling here is INSANE
                        $$ = Value::Node(
                            self.builder.accessible($<Node>1)
                        );
                    }
                | keyword_variable
                    {
                        $$ = Value::Node(
                            self.builder.accessible($<Node>1)
                        );
                    }
                ;

         var_lhs: user_variable
                    {
                        $$ = Value::Node(
                            self.builder.assignable($<Node>1)
                        );
                    }
                | keyword_variable
                    {
                        $$ = Value::Node(
                            self.builder.assignable($<Node>1)
                        );
                    }
                ;

         backref: tNTH_REF
                    {
                        $$ = Value::Node(
                            self.builder.nth_ref($<Token>1)
                        );
                    }
                | tBACK_REF
                    {
                        $$ = Value::Node(
                            self.builder.back_ref($<Token>1)
                        );
                    }
                ;

      superclass: tLT
                    {
                        self.yylexer.set_lex_state(EXPR_VALUE);
                    }
                  expr_value term
                    {
                        let token = $<Token>1;
                        let node  = $<Node>3;
                        $$ = Value::Superclass( Some((token, node)) );
                    }
                | /* none */
                    {
                        $$ = Value::Superclass(None);
                    }
                ;

    f_paren_args: tLPAREN2 f_args rparen
                    {
                        $$ = Value::MaybeNode(
                            self.builder.args(Some($<Token>1), $<NodeList>2, Some($<Token>3))
                        );

                        self.yylexer.set_lex_state(EXPR_VALUE);
                    }
                | tLPAREN2 f_arg tCOMMA args_forward rparen
                    {
                        let args = [
                            $<NodeList>2,
                            vec![ self.builder.forward_arg($<Token>4) ]
                        ].concat();
                        $$ = Value::MaybeNode(
                            self.builder.args(
                                Some($<Token>1),
                                args,
                                Some($<Token>5)
                            )
                        );
                        self.static_env.declare_forward_args();
                    }
                | tLPAREN2 args_forward rparen
                    {
                        $$ = Value::MaybeNode(
                            Some(
                                self.builder.forward_only_args($<Token>1, $<Token>2, $<Token>3)
                            )
                        );
                        self.static_env.declare_forward_args();
                        self.yylexer.set_lex_state(EXPR_VALUE);
                    }
                ;

       f_arglist: f_paren_args
                |    {
                        $<Bool>$ = Value::Bool(self.yylexer.p.in_kwarg);
                        self.yylexer.p.in_kwarg = true;
                    }
                  f_args term
                    {
                        self.yylexer.p.in_kwarg = $<Bool>1;
                        $$ = Value::MaybeNode(
                            self.builder.args(None, $<NodeList>2, None)
                        );
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
                        $$ = $<RAW>2;
                    }
                | /* none */
                    {
                        $$ = Value::NodeList( vec![] );
                    }
                ;

          f_args: f_arg tCOMMA f_optarg tCOMMA f_rest_arg opt_args_tail
                    {
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
                        // diagnostic :error, :argument_const, None, val[0]
                        $$ = $<RAW>1;
                    }
                | tIVAR
                    {
                        // diagnostic :error, :argument_const, None, val[0]
                        $$ = $<RAW>1;
                    }
                | tGVAR
                    {
                        // diagnostic :error, :argument_const, None, val[0]
                        $$ = $<RAW>1;
                    }
                | tCVAR
                    {
                        // diagnostic :error, :argument_const, None, val[0]
                        $$ = $<RAW>1;
                    }
                ;

      f_norm_arg: f_bad_arg
                | tIDENTIFIER
                    {
                        let ident_t = $<Borrow:Token>1;
                        self.static_env.declare(&ident_t.1);
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
                        $$ = Value::Node(
                            self.builder.arg($<Token>1)
                        );
                    }
                | tLPAREN f_margs rparen
                    {
                        $$ = Value::Node(
                            self.builder.multi_lhs(
                                Some($<Token>1),
                                $<NodeList>2,
                                Some($<Token>3)
                            )
                        );
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
                        let ident_t = $<Borrow:Token>1;
                        // check_kwarg_name(val[0])

                        self.static_env.declare(&ident_t.1);

                        // @max_numparam_stack.has_ordinary_params!

                        // @current_arg_stack.set(val[0][0])

                        $$ = $<RAW>1;
                    }
                ;

            f_kw: f_label arg_value
                    {
                        self.current_arg_stack.set(None);
                        $$ = Value::Node(
                            self.builder.kwoptarg($<Token>1, $<Node>2)
                        );
                    }
                | f_label
                    {
                        self.current_arg_stack.set(None);
                        $$ = Value::Node(
                            self.builder.kwarg($<Token>1)
                        );
                    }
                ;

      f_block_kw: f_label primary_value
                    {
                        $$ = Value::Node(
                            self.builder.kwoptarg($<Token>1, $<Node>2)
                        );
                    }
                | f_label
                    {
                        $$ = Value::Node(
                            self.builder.kwarg($<Token>1)
                        );
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
                        $$ = Value::NodeList(
                            vec![
                                self.builder.kwnilarg($<Token>1, $<Token>2)
                            ]
                        );
                    }
                ;

        f_kwrest: kwrest_mark tIDENTIFIER
                    {
                        let ident_t = $<Token>2;
                        self.static_env.declare(&ident_t.1);
                        $$ = Value::NodeList(
                            vec![
                                self.builder.kwrestarg($<Token>1, Some($<Token>2))
                            ]
                        );
                    }
                | kwrest_mark
                    {
                        $$ = Value::NodeList(
                            vec![
                                self.builder.kwrestarg($<Token>1, None)
                            ]
                        );
                    }
                ;

           f_opt: f_arg_asgn tEQL arg_value
                    {
                        // @current_arg_stack.set(0)
                        $$ = Value::Node(
                            self.builder.optarg(
                                $<Token>1,
                                $<Token>2,
                                $<Node>3
                            )
                        );
                    }
                ;

     f_block_opt: f_arg_asgn tEQL primary_value
                    {
                        // @current_arg_stack.set(0)
                        $$ = Value::Node(
                            self.builder.optarg(
                                $<Token>1,
                                $<Token>2,
                                $<Node>3
                            )
                        );
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
                        let ident_t = $<Token>1;
                        self.static_env.declare(&ident_t.1);

                        $$ = Value::NodeList(
                            vec![
                                self.builder.restarg($<Token>1, Some($<Token>2))
                            ]
                        );
                    }
                | restarg_mark
                    {
                        $$ = Value::NodeList(
                            vec![
                                self.builder.restarg($<Token>1, None)
                            ]
                        );
                    }
                ;

     blkarg_mark: tAMPER2
                | tAMPER
                ;

     f_block_arg: blkarg_mark tIDENTIFIER
                    {
                        let ident_t = $<Token>2;
                        self.static_env.declare(&ident_t.1);
                        $$ = Value::Node(
                            self.builder.blockarg($<Token>1, ident_t)
                        );
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
                        $$ = Value::Node(
                            self.builder.pair(
                                $<Node>1,
                                $<Token>2,
                                $<Node>3
                            )
                        );
                    }
                | tLABEL arg_value
                    {
                        $$ = Value::Node(
                            self.builder.pair_keyword(
                                $<Token>1,
                                $<Node>2
                            )
                        );
                    }
                | tSTRING_BEG string_contents tLABEL_END arg_value
                    {
                        $$ = Value::Node(
                            self.builder.pair_quoted(
                                $<Token>1,
                                $<NodeList>2,
                                $<Token>3,
                                $<Node>4
                            )
                        );
                    }
                | tDSTAR arg_value
                    {
                        $$ = Value::Node(
                            self.builder.kwsplat($<Token>1, $<Node>2)
                        );
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
                        $$ = $<RAW>2;
                    }
                ;

        rbracket: opt_nl tRBRACK
                    {
                        $$ = $<RAW>2;
                    }
                ;

          rbrace: opt_nl tRCURLY
                    {
                        $$ = $<RAW>2;
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

use crate::Node;

#[derive(Clone, PartialEq)]
pub enum Value {
    Stolen,
    None,
    Token(Token),
    TokenList(Vec<Token>),
    Node(Node),
    NodeList(Vec<Node>),
    Bool(bool),

    /* For custom superclass rule */
    Superclass(Option<(Token, Node)>),

    /* For custom opt_ensure rule */
    OptEnsure(Option<(Token, Node)>),

    /* For custom opt_else rule */
    OptElse(Option<(Token, Option<Node>)>),

    /* For custom exc_var rule */
    ExcVar(Option<(Token, Node)>),

    /* For custom if_tail rule */
    IfTail(Option<(Token, Option<Node>)>),

    /* For custom expr_value_do rule */
    ExprValueDo(( Node, Token )),

    /* For custom p_kw_label rule */
    PKwLabel( PKwLabel ),

    /* For custom brace_body rule */
    BraceBody(( Option<Node>, Option<Node> )),

    /* For custom cmd_brace_block rule */
    CmdBraceBlock(( Token, Option<Node>, Option<Node>, Token )),

    /* For custom paren_args rule  */
    ParenArgs(( Token, Vec<Node>, Token )),

    /* For custom opt_paren_args rule  */
    OptParenArgs(( Option<Token>, Vec<Node>, Option<Token> )),

    /* For custom lambda_body rule  */
    LambdaBody(( Token, Option<Node>, Token )),

    /* For custom do_block rule  */
    DoBlock(( Token, Option<Node>, Option<Node>, Token )),

    /* For custom brace_block rule  */
    BraceBlock(( Token, Option<Node>, Option<Node>, Token )),

    /* For custom defs_head rule */
    DefsHead(( Token, Node, Token, Token )),

    /* For custom defn_head rule */
    DefnHead(( Token, Token )),

    /* For custom begin_block rule  */
    BeginBlock(( Token, Option<Node>, Token )),

    /* For custom cases rule */
    Cases(( Vec<Node>, Option<(Token, Option<Node>)> )),

    /* For custom case_body rule */
    CaseBody(( Vec<Node>, Option<(Token, Option<Node>)> )),

    /* For custom p_cases rule */
    PCases(( Vec<Node>, Option<(Token, Option<Node>)> )),

    /* For custom p_case_body rule */
    PCaseBody(( Vec<Node>, Option<(Token, Option<Node>)> )),

    /* For custom compstmt rule */
    MaybeNode( Option<Node> ),

    /* For custom do_body rule */
    DoBody(( Option<Node>, Option<Node> )),

    /* For custom p_top_expr rule */
    PTopExpr(( Node, Option<Node> )),
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
            Value::Bool(value) => {
                f.write_fmt(format_args!("Bool({:?})", value))
            },
            Value::Superclass(data) => {
                f.write_fmt(format_args!("Superclass({:?})", data))
            },
            Value::OptEnsure(data) => {
                f.write_fmt(format_args!("OptEnsure({:?})", data))
            },
            Value::OptElse(data) => {
                f.write_fmt(format_args!("OptElse({:?})", data))
            },
            Value::ExcVar(data) => {
                f.write_fmt(format_args!("ExcVar({:?})", data))
            },
            Value::IfTail(data) => {
                f.write_fmt(format_args!("IfTail({:?})", data))
            },
            Value::ExprValueDo((node, token)) => {
                f.write_fmt(format_args!("ExprValueDo({:?}, {:?})", node, token))
            },
            Value::PKwLabel(label) => {
                f.write_fmt(format_args!("PKwLabel({:?})", label))
            },
            Value::BraceBody((args, body)) => {
                f.write_fmt(format_args!("BraceBody({:?}, {:?})", args, body))
            },
            Value::CmdBraceBlock((start, args, body, end)) => {
                f.write_fmt(format_args!("CmdBraceBlock({:?}, {:?}, {:?}, {:?})", start, args, body, end))
            },
            Value::ParenArgs((start, nodes, end)) => {
                f.write_fmt(format_args!("ParenArgs({:?}, {:?}, {:?})", start, nodes, end))
            },
            Value::OptParenArgs((start, nodes, end)) => {
                f.write_fmt(format_args!("OptParenArgs({:?}, {:?}, {:?})", start, nodes, end))
            },
            Value::LambdaBody((start, nodes, end)) => {
                f.write_fmt(format_args!("LambdaBody({:?}, {:?}, {:?})", start, nodes, end))
            },
            Value::DoBlock((start, args, body, end)) => {
                f.write_fmt(format_args!("DoBlock({:?}, {:?}, {:?}, {:?})", start, args, body, end))
            },
            Value::BraceBlock((start, args, body, end)) => {
                f.write_fmt(format_args!("BraceBlock({:?}, {:?}, {:?}, {:?})", start, args, body, end))
            },
            Value::DefsHead((def, singleton, dot, name)) => {
                f.write_fmt(format_args!("DefsHead({:?}, {:?}, {:?}, {:?})", def, singleton, dot, name))
            },
            Value::DefnHead((def, name)) => {
                f.write_fmt(format_args!("DefnHead({:?}, {:?})", def, name))
            },
            Value::BeginBlock((start, body, end)) => {
                f.write_fmt(format_args!("BeginBlock({:?}, {:?}, {:?})", start, body, end))
            },
            Value::Cases((whens, else_)) => {
                f.write_fmt(format_args!("Cases({:?}, {:?})", whens, else_))
            },
            Value::CaseBody((whens, else_)) => {
                f.write_fmt(format_args!("CaseBody({:?}, {:?})", whens, else_))
            },
            Value::PCases((whens, else_)) => {
                f.write_fmt(format_args!("PCases({:?}, {:?})", whens, else_))
            },
            Value::PCaseBody((whens, else_)) => {
                f.write_fmt(format_args!("PCaseBody({:?}, {:?})", whens, else_))
            },
            Value::MaybeNode(maybe_node) => {
                f.write_fmt(format_args!("MaybeNode({:?})", maybe_node))
            },
            Value::DoBody((args, body)) => {
                f.write_fmt(format_args!("DoBody({:?}, {:?})", args, body))
            },
            Value::PTopExpr((pattern, guard)) => {
                f.write_fmt(format_args!("PTopExpr({:?}, {:?})", pattern, guard))
            },
        }
    }
}

impl Parser {
  pub fn do_parse(mut self) -> Option<Node> {
      self.parse();
      self.result
  }
}

impl Lexer {
    fn report_syntax_error(&self, ctx: &Context) {
        if self.debug { eprintln!("{:#?}", ctx) }
    }

    fn yyerror(&mut self, loc: &Loc, msg: &str) {
        if self.debug { eprintln!("{:#?} {:#?}", loc, msg) }
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

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let static_env = lexer.p.static_env.clone();
        let context = lexer.p.context.clone();
        let current_arg_stack = CurrentArgStack::new();

        Self {
            yy_error_verbose: true,
            yynerrs: 0,
            yydebug: 0,
            yyerrstatus_: 0,
            result: None,
            builder: Builder::new(static_env.clone(), context.clone(), current_arg_stack.clone()),
            current_arg_stack: current_arg_stack,
            static_env: static_env,
            yylexer: lexer,
        }
    }

    pub fn set_debug(&mut self, debug: bool) {
        self.yydebug = if debug { 1 } else { 0 };
        self.yylexer.debug = debug;
    }
}
