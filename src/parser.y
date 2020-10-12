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
    context: ParserContext,
    last_token: Token,
    max_numparam_stack: MaxNumparamStack,
    pattern_variables: VariablesStack,
    pattern_hash_keys: VariablesStack,
    tokens: Vec<Token>,
}

%code use {
    use crate::{Lexer, Builder, CurrentArgStack, StaticEnvironment, MaxNumparamStack, VariablesStack};
    use crate::lex_states::*;
    use crate::{Context as ParserContext, ContextItem};
    use crate::builder::{LoopType, KeywordCmd, LogicalOp, PKwLabel, ArgsType};
    use crate::str_term::StrTerm;
    use crate::map_builder::value;
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
%type <token>   args_forward excessed_comma def_name k_if k_elsif
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
                        self.yylexer.set_lex_state(EXPR_BEG);
                    }
                  top_compstmt
                    {
                        let _trigger_locs = @2;
                        self.result = $<MaybeNode>2;
                    }
                ;

    top_compstmt: top_stmts opt_terms
                    {
                        // TODO: run void_stmts
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
                        let else_ = Some(( $<Token>3, $<MaybeNode>5 ));
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
                        // TODO: run void_stmts
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
                        return Self::YYERROR;
                    }
                  begin_block
                    {
                    }
                ;

            stmt: kALIAS fitem
                    {
                        self.yylexer.set_lex_state(EXPR_FNAME|EXPR_FITEM);
                    }
                  fitem
                    {
                        $$ = Value::Node(
                            self.builder.alias($<Token>1, $<Node>2, $<Node>4)
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
                        self.yyerror(&@3, "can't make alias for the number variables");
                        return Self::YYERROR;
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
                        if self.context.is_in_def() {
                            self.warn(&@1, "END in method; use at_exit")
                        }

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
                        // TODO: backref_error
                        $$ = Value::Node( Node::empty_begin(&@$) );
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
                        self.yylexer.set_lex_state(EXPR_BEG|EXPR_LABEL);
                        self.yylexer.command_start = true;
                        self.pattern_variables.push();

                        $<Bool>$ = Value::Bool(self.yylexer.in_kwarg);
                        self.yylexer.in_kwarg = true;
                    }
                  p_expr
                    {
                        self.pattern_variables.pop();
                        self.yylexer.in_kwarg = $<Bool>3;

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

                        $$ = $1;
                    }
                ;

       defn_head: k_def def_name
                    {
                        self.context.push_def();

                        $$ = Value::DefnHead(( $<Token>1, $<Token>2 ));
                    }
                ;

       defs_head: k_def singleton dot_or_colon
                    {
                        self.yylexer.set_lex_state(EXPR_FNAME);
                    }
                  def_name
                    {
                        self.yylexer.set_lex_state(EXPR_ENDFN|EXPR_LABEL);
                        self.context.push_defs();

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
                ;

 cmd_brace_block: tLBRACE_ARG
                    {
                        self.context.push_block();
                    }
                  brace_body tRCURLY
                    {
                        self.context.pop();
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
                            self.builder.begin(
                                $<Token>1,
                                Some($<Node>2),
                                $<Token>3
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
                        let mlhs_items: Vec<Node> = match $<Node>2 {
                            Node::Mlhs { items, .. } => items,
                            other => panic!("unsupported mlhs item {:?}", other)
                        };

                        $$ = Value::Node(
                            self.builder.multi_lhs(
                                Some($<Token>1),
                                mlhs_items,
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
                        let op_t = $<Token>2;
                        if op_t.0 == Lexer::tANDDOT {
                            self.yyerror(&@2, "&. inside multiple assignment destination");
                            return Self::YYERROR;
                        }

                        $$ = Value::Node(
                            self.builder.attr_asgn(
                                $<Node>1,
                                op_t,
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
                        let op_t = $<Token>2;
                        if op_t.0 == Lexer::tANDDOT {
                            self.yyerror(&@2, "&. inside multiple assignment destination");
                            return Self::YYERROR;
                        }

                        $$ = Value::Node(
                            self.builder.attr_asgn(
                                $<Node>1,
                                op_t,
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
                        self.yyerror(&@1, "class/module name must be CONSTANT");
                        $$ = $1;
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
                | undef_list tCOMMA
                    {
                        self.yylexer.set_lex_state(EXPR_FNAME|EXPR_FITEM);
                    }
                  fitem
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
                            self.builder.binary_op(
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
                                $<Token>5,
                                $<Node>6
                            )
                        );
                    }
                | defn_head f_paren_args tEQL arg
                    {
                        let (def_t, name_t) = $<DefnHead>1;

                        let name = value(&name_t);
                        if name.ends_with('=') {
                            self.yyerror(&@1, "setter method cannot be defined in an endless method definition");
                        }

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
                        self.context.pop();
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
                        self.context.pop();
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
                        self.context.pop();
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
                        self.context.pop();
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
                        self.warn(&@2, &format!("comparison after comparison"));
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
                        if !self.static_env.is_forward_args_declared() {
                            self.yyerror(&@4, "unexpected ...");
                            return Self::YYERROR;
                        }

                        let args = [
                            $<NodeList>2,
                            vec![ self.builder.forwarded_args($<Token>4) ]
                        ].concat();
                        $$ = Value::ParenArgs(( $<Token>1, args, $<Token>5 ));
                    }
                | tLPAREN2 args_forward rparen
                    {
                        if !self.static_env.is_forward_args_declared() {
                            self.yyerror(&@2, "unexpected ...");
                            return Self::YYERROR;
                        }

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
                        let lookahead =
                            match self.last_token.0 {
                                Lexer::tLPAREN2
                                | Lexer::tLPAREN
                                | Lexer:: tLPAREN_ARG
                                | Lexer::tLBRACK2
                                | Lexer::tLBRACK => true,
                                _ => false
                            };

                        if lookahead { self.yylexer.cmdarg_pop() }
                        self.yylexer.cmdarg_push(true);
                        if lookahead { self.yylexer.cmdarg_push(false) }
                    }
                  call_args
                    {
                        let lookahead =
                            match self.last_token.0 {
                                Lexer::tLBRACE_ARG => true,
                                _ => false
                            };

                        if lookahead { self.yylexer.cmdarg_pop() }
                        self.yylexer.cmdarg_pop();
                        if lookahead { self.yylexer.cmdarg_push(false) }

                        $$ = $2;
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
                                $<MaybeNode>2,
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
                                Some($<Token>5)
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
                        let (else_t, else_) = match $<OptElse>5 {
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
                                LoopType::Until,
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
                        let (in_bodies, else_) = $<PCaseBody>4;
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
                        self.context.push_class();
                    }
                  bodystmt
                  k_end
                    {
                        if !self.context.is_class_definition_allowed() {
                            self.yyerror(&@1, "class definition in method body");
                            return Self::YYERROR;
                        }

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
                        self.context.pop();
                    }
                | k_class tLSHFT expr
                    {
                        self.static_env.extend_static();
                        self.yylexer.cmdarg_push(false);
                        self.yylexer.cond_push(false);
                        self.context.push_sclass();
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
                        self.context.pop();
                    }
                | k_module cpath
                    {
                        self.static_env.extend_static();
                        self.yylexer.cmdarg_push(false);
                        self.context.push_module();
                    }
                  bodystmt
                  k_end
                    {
                        if !self.context.is_module_definition_allowed() {
                            self.yyerror(&@1, "module definition in method body");
                            return Self::YYERROR;
                        }

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
                        self.context.pop();
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
                        self.context.pop();
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
                        self.context.pop();
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
                    {
                        // TODO: check for WARN_EOL("if")
                        $$ = $1;
                    }
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
                    {
                        // TODO: check for WARN_EOL("elsif")
                        $$ = $1;
                    }
                ;

           k_end: kEND
                ;

        k_return: kRETURN
                    {
                        if self.context.is_in_class() {
                            self.yyerror(&@1, "Invalid return in class/module body");
                            return Self::YYERROR;
                        }
                        $$ = $1;
                    }
                ;

            then: term
                | kTHEN
                | term kTHEN
                    {
                        $$ = $2;
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
                        let nodes = [ $<NodeList>1, $<NodeList>3, $<NodeList>5, $<NodeList>6 ].concat();
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
                        let nodes = [ $<NodeList>1, $<NodeList>3, $<NodeList>4 ].concat();
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
                        self.yylexer.command_start = true;
                        $$ = $1;
                    }
                ;

 block_param_def: tPIPE opt_bv_decl tPIPE
                    {
                        self.max_numparam_stack.set_has_ordinary_params();
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
                        self.max_numparam_stack.set_has_ordinary_params();
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
                        self.static_env.declare(&value(&ident_t));
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
                        self.max_numparam_stack.push();
                        self.context.push_lambda();
                        $<Num>$ = Value::Num(self.yylexer.lpar_beg);
                        self.yylexer.lpar_beg = self.yylexer.paren_nest;
                    }
                  f_larglist
                    {
                        self.context.pop();
                        self.yylexer.cmdarg_push(false);
                    }
                  lambda_body
                    {
                        self.yylexer.lpar_beg = $<Num>2;

                        let lambda_call = self.builder.call_lambda($<Token>1);
                        let args = if self.max_numparam_stack.has_numparams() {
                            ArgsType::Numargs(self.max_numparam_stack.top() as u8)
                        } else {
                            ArgsType::Args($<MaybeNode>3)
                        };
                        let (begin_t, body, end_t) = $<LambdaBody>5;

                        self.max_numparam_stack.pop();
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
                        self.max_numparam_stack.set_has_ordinary_params();
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
                        let args = $<NodeList>1;
                        if !args.is_empty() {
                            self.max_numparam_stack.set_has_ordinary_params();
                        }
                        $$ = Value::MaybeNode(
                            self.builder.args(None, args, None)
                        );
                    }
                ;

     lambda_body: tLAMBEG
                    {
                        self.context.push_lambda();
                    }
                  compstmt tRCURLY
                    {
                        self.context.pop();
                        $$ = Value::LambdaBody(( $<Token>1, $<MaybeNode>3, $<Token>4 ));
                    }
                | kDO_LAMBDA
                    {
                        self.context.push_lambda();
                    }
                  bodystmt k_end
                    {
                        self.context.pop();
                        $$ = Value::LambdaBody(( $<Token>1, $<MaybeNode>3, $<Token>4 ));
                    }
                ;

        do_block: k_do_block
                    {
                        self.context.push_block();
                    }
                  do_body k_end
                    {
                        let (args, body) = $<DoBody>3;
                        self.context.pop();
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

                        let (begin_t, args, body, end_t) = $<DoBlock>5;
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
                                KeywordCmd::Zsuper,
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
                        self.context.push_block();
                    }
                  brace_body tRCURLY
                    {
                        let (args, body) = $<BraceBody>3;
                        self.context.pop();

                        $$ = Value::BraceBlock(( $<Token>1, args, body, $<Token>4 ));
                    }
                | k_do
                    {
                        self.context.push_block();
                    }
                  do_body k_end
                    {
                        let (args, body) = $<DoBody>3;
                        self.context.pop();

                        $$ = Value::BraceBlock(( $<Token>1, args, body, $<Token>4 ));
                    }
                ;

      brace_body:   {
                        self.static_env.extend_dynamic();
                        self.max_numparam_stack.push();
                    }
                  opt_block_param compstmt
                    {
                        let args = if self.max_numparam_stack.has_numparams() {
                            ArgsType::Numargs(self.max_numparam_stack.top() as u8)
                        } else {
                            ArgsType::Args($<MaybeNode>2)
                        };

                        self.max_numparam_stack.pop();
                        self.static_env.unextend();

                        $$ = Value::BraceBody(( args, $<MaybeNode>3 ));
                    }
                ;

         do_body:   {
                        self.static_env.extend_dynamic();
                        self.max_numparam_stack.push();
                        self.yylexer.cmdarg_push(false);
                    }
                  opt_block_param bodystmt
                    {
                        let args = if self.max_numparam_stack.has_numparams() {
                            ArgsType::Numargs(self.max_numparam_stack.top() as u8)
                        } else {
                            ArgsType::Args($<MaybeNode>2)
                        };

                        self.max_numparam_stack.pop();
                        self.static_env.unextend();
                        self.yylexer.cmdarg_pop();

                        $$ = Value::DoBody(( args, $<MaybeNode>3 ));
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
                        self.yylexer.set_lex_state(EXPR_BEG|EXPR_LABEL);
                        self.yylexer.command_start = false;
                        self.pattern_variables.push();
                        self.pattern_hash_keys.push();

                        $<Bool>$ = Value::Bool(self.yylexer.in_kwarg);
                        self.yylexer.in_kwarg = true;
                    }
                  p_top_expr then
                    {
                        self.yylexer.in_kwarg = $<Bool>2;
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
                        let items = [ vec![$<Node>1], $<NodeList>3 ].concat();
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
                        $$ = $1;
                        self.pattern_hash_keys.push();
                    }
                ;

      p_lbracket: tLBRACK2
                    {
                        $$ = $1;
                        self.pattern_hash_keys.push();
                    }
                ;

    p_expr_basic: p_value
                | p_const p_lparen p_args rparen
                    {
                        self.pattern_hash_keys.pop();
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
                        self.pattern_hash_keys.pop();
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
                        self.pattern_hash_keys.pop();
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
                        self.pattern_hash_keys.pop();
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
                        self.pattern_hash_keys.pop();
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
                        self.pattern_hash_keys.pop();
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
                        self.pattern_hash_keys.push();
                        $<Bool>$ = Value::Bool(self.yylexer.in_kwarg);
                        self.yylexer.in_kwarg = false;
                    }
                  p_kwargs rbrace
                    {
                        self.pattern_hash_keys.pop();
                        self.yylexer.in_kwarg = $<Bool>2;
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
                        self.pattern_hash_keys.push();
                    }
                  p_expr rparen
                    {
                        self.pattern_hash_keys.pop();
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
                        let name = value(&ident_t);

                        if !self.static_env.is_declared(&name) {
                            self.yyerror(&@2, &format!("{}: no such local variable", name));
                        }

                        let lvar = self.builder.accessible(self.builder.lvar(ident_t));
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
                        let node = $<MaybeNode>2;
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
                            self.builder.dedent_string(string, self.yylexer.buffer.heredoc_indent)
                        );
                    }
                ;

         xstring: tXSTRING_BEG xstring_contents tSTRING_END
                    {
                        let string = self.builder.xstring_compose($<Token>1, $<NodeList>2, $<Token>3);
                        $$ = Value::Node(
                            self.builder.dedent_string(string, self.yylexer.buffer.heredoc_indent)
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
                        let mut nodes = $<NodeList>1;
                        nodes.push($<Node>2);
                        $$ = Value::NodeList(nodes);
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
                        let mut strterm: Option<StrTerm> = None;
                        std::mem::swap(&mut strterm, &mut self.yylexer.strterm);
                        $<StrTerm>$ = Value::StrTerm(strterm);
                        self.yylexer.set_lex_state(EXPR_BEG);
                    }
                  string_dvar
                    {
                        self.yylexer.strterm = $<StrTerm>2;
                        $$ = $3;
                    }
                | tSTRING_DBEG
                    {
                        self.yylexer.cmdarg_push(false);
                        self.yylexer.cond_push(false);
                    }
                    {
                        let mut strterm: Option<StrTerm> = None;
                        std::mem::swap(&mut strterm, &mut self.yylexer.strterm);
                        $<StrTerm>$ = Value::StrTerm(strterm);
                    }
                    {
                        $<Num>$ = Value::Num( self.yylexer.state.get() );
                        self.yylexer.set_lex_state(EXPR_BEG);
                    }
                    {
                        $<Num>$ = Value::Num( self.yylexer.brace_nest );
                        self.yylexer.brace_nest = 0;
                    }
                    {
                        $<Num>$ = Value::Num( self.yylexer.buffer.heredoc_indent );
                        self.yylexer.buffer.heredoc_indent = 0;
                    }
                  compstmt tSTRING_DEND
                    {
                        self.yylexer.cond_pop();
                        self.yylexer.cmdarg_pop();
                        self.yylexer.strterm = $<StrTerm>3;
                        self.yylexer.set_lex_state($<Num>4);
                        self.yylexer.brace_nest = $<Num>5;
                        self.yylexer.buffer.heredoc_indent = $<Num>6;
                        self.yylexer.buffer.heredoc_line_indent = -1;

                        $$ = Value::Node(
                            self.builder.begin(
                                $<Token>1,
                                $<MaybeNode>7,
                                $<Token>8
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
                        $$ = Value::Node(
                            self.builder.integer($<Token>1)
                        );
                    }
                | tFLOAT
                    {
                        $$ = Value::Node(
                            self.builder.float($<Token>1)
                        );
                    }
                | tRATIONAL
                    {
                        $$ = Value::Node(
                            self.builder.rational($<Token>1)
                        );
                    }
                | tIMAGINARY
                    {
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
                        let node =  cast_to_variant!(Node, yystack, yystack.owned_value_at(0));
                        match &node {
                            Node::Lvar { name, loc } => {
                                match name.chars().collect::<Vec<_>>()[..] {
                                    ['_', n] if n >= '1' && n <= '9' => {
                                        if !self.static_env.is_declared(name) && self.context.is_in_dynamic_block() {
                                            /* definitely an implicit param */
                                            let location = loc.expression.clone();

                                            if self.max_numparam_stack.has_ordinary_params() {
                                                // diagnostic :error, :ordinary_param_defined, nil, [nil, location]
                                            }

                                            let mut raw_context = self.context.inner_clone();
                                            let mut raw_max_numparam_stack = self.max_numparam_stack.inner_clone();

                                            /* ignore current block scope */
                                            raw_context.pop();
                                            raw_max_numparam_stack.pop();

                                            for outer_scope in raw_context.iter().rev() {
                                                if *outer_scope == ContextItem::Block || *outer_scope == ContextItem::Lambda {
                                                    let outer_scope_has_numparams = raw_max_numparam_stack.pop().unwrap() > 0;

                                                    if outer_scope_has_numparams {
                                                        // diagnostic :error, :numparam_used_in_outer_scope, nil, [nil, location]
                                                    } else {
                                                        /* for now it's ok, but an outer scope can also be a block
                                                           with numparams, so we need to continue */
                                                    }
                                                } else {
                                                    /* found an outer scope that can't have numparams
                                                       like def/class/etc */
                                                    break;
                                                }
                                            }

                                            self.static_env.declare(name);
                                            self.max_numparam_stack.register(n.to_digit(10).unwrap() as i32)
                                        }
                                    },
                                    _ => {}
                                }
                            },
                            _ => {},
                        }

                        $$ = Value::Node(
                            self.builder.accessible(node)
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
                        self.yylexer.set_lex_state(EXPR_BEG);
                        self.yylexer.command_start = true;
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

                        self.yylexer.set_lex_state(EXPR_BEG);
                        self.yylexer.command_start = true;
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
                        self.yylexer.set_lex_state(EXPR_BEG);
                        self.yylexer.command_start = true;
                    }
                | tLPAREN2 args_forward rparen
                    {
                        $$ = Value::MaybeNode(
                            Some(
                                self.builder.forward_only_args($<Token>1, $<Token>2, $<Token>3)
                            )
                        );

                        self.static_env.declare_forward_args();
                        self.yylexer.set_lex_state(EXPR_BEG);
                        self.yylexer.command_start = true;
                    }
                ;

       f_arglist: f_paren_args
                |    {
                        $<Bool>$ = Value::Bool(self.yylexer.in_kwarg);
                        self.yylexer.in_kwarg = true;
                        self.yylexer.set_lex_state(self.yylexer.state.get()|EXPR_LABEL);
                    }
                  f_args term
                    {
                        self.yylexer.in_kwarg = $<Bool>1;
                        $$ = Value::MaybeNode(
                            self.builder.args(None, $<NodeList>2, None)
                        );
                        self.yylexer.set_lex_state(EXPR_BEG);
                        self.yylexer.command_start = true;
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
                        let nodes = [ $<NodeList>1, $<NodeList>3, $<NodeList>5, $<NodeList>7, $<NodeList>8 ].concat();
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
                        let nodes = [ $<NodeList>1, $<NodeList>3, $<NodeList>4 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | f_optarg tCOMMA f_rest_arg tCOMMA f_arg opt_args_tail
                    {
                        let nodes = [ $<NodeList>1, $<NodeList>3, $<NodeList>5, $<NodeList>6 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | f_optarg opt_args_tail
                    {
                        let nodes = [ $<NodeList>1, $<NodeList>2 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | f_optarg tCOMMA f_arg opt_args_tail
                    {
                        let nodes = [ $<NodeList>1, $<NodeList>3, $<NodeList>4 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | f_rest_arg opt_args_tail
                    {
                        let nodes = [ $<NodeList>1, $<NodeList>2 ].concat();
                        $$ = Value::NodeList(nodes);
                    }
                | f_rest_arg tCOMMA f_arg opt_args_tail
                    {
                        let nodes = [ $<NodeList>1, $<NodeList>3, $<NodeList>4 ].concat();
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
                        self.yyerror(&@1, "formal argument cannot be a constant");
                        $$ = $<RAW>1;
                    }
                | tIVAR
                    {
                        self.yyerror(&@1, "formal argument cannot be an instance variable");
                        $$ = $<RAW>1;
                    }
                | tGVAR
                    {
                        self.yyerror(&@1, "formal argument cannot be a global variable");
                        $$ = $<RAW>1;
                    }
                | tCVAR
                    {
                        self.yyerror(&@1, "formal argument cannot be a class variable");
                        $$ = $<RAW>1;
                    }
                ;

      f_norm_arg: f_bad_arg
                | tIDENTIFIER
                    {
                        let ident_t = $<Token>1;
                        let name = value(&ident_t);
                        self.static_env.declare(&name);
                        self.max_numparam_stack.set_has_ordinary_params();
                        $$ = Value::Token(ident_t);
                    }
                ;

      f_arg_asgn: f_norm_arg
                    {
                        let arg_t = $<Token>1;
                        let arg_name = value(&arg_t);
                        self.current_arg_stack.set(Some(arg_name));
                        $$ = Value::Token(arg_t);
                    }
                ;

      f_arg_item: f_arg_asgn
                    {
                        self.current_arg_stack.set(None);
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
                        let ident_t = $<Token>1;
                        if let Err(e) = self.check_kwarg_name(&ident_t) {
                            self.yyerror(&@1, &e);
                        }

                        let ident = value(&ident_t);
                        self.static_env.declare(&ident);

                        self.max_numparam_stack.set_has_ordinary_params();

                        self.current_arg_stack.set(Some(ident));

                        $$ = Value::Token(ident_t);
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
                        self.static_env.declare(&value(&ident_t));
                        $$ = Value::NodeList(
                            vec![
                                self.builder.kwrestarg($<Token>1, Some(ident_t))
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
                        self.current_arg_stack.set(None);
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
                        self.current_arg_stack.set(None);
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
                        let ident_t = $<Token>2;
                        self.static_env.declare(&value(&ident_t));

                        $$ = Value::NodeList(
                            vec![
                                self.builder.restarg($<Token>1, Some(ident_t))
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
                        self.static_env.declare(&value(&ident_t));
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
                | tLPAREN2 { self.yylexer.set_lex_state(EXPR_BEG); } expr rparen
                    {
                        $$ = $3;
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

#[derive(Clone)]
pub enum Value {
    Stolen,
    None,
    Token(Token),
    TokenList(Vec<Token>),
    Node(Node),
    NodeList(Vec<Node>),
    Bool(bool),
    StrTerm(Option<StrTerm>),
    Num(i32),

    /* For custom superclass rule */
    Superclass(Option<(Token, Node)>),

    /* For custom opt_ensure rule */
    OptEnsure(Option<(Token, Option<Node>)>),

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
    BraceBody(( ArgsType, Option<Node> )),

    /* For custom cmd_brace_block rule */
    CmdBraceBlock(( Token, ArgsType, Option<Node>, Token )),

    /* For custom paren_args rule  */
    ParenArgs(( Token, Vec<Node>, Token )),

    /* For custom opt_paren_args rule  */
    OptParenArgs(( Option<Token>, Vec<Node>, Option<Token> )),

    /* For custom lambda_body rule  */
    LambdaBody(( Token, Option<Node>, Token )),

    /* For custom do_block rule  */
    DoBlock(( Token, ArgsType, Option<Node>, Token )),

    /* For custom brace_block rule  */
    BraceBlock(( Token, ArgsType, Option<Node>, Token )),

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
    DoBody(( ArgsType, Option<Node> )),

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
            Value::StrTerm(value) => {
                f.write_fmt(format_args!("StrTerm({:?})", value))
            },
            Value::Num(value) => {
                f.write_fmt(format_args!("Num({:?})", value))
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

#[allow(non_upper_case_globals)]
impl Lexer {
    // Dummy tokens to satisfy tests for now
    pub const tSTRING: i32 = 1_000;
    pub const tSYMBOL: i32 = 1_001;
    pub const tUNARY_NUM: i32 = 1_002;
    pub const tREGEXP_OPT: i32 = 1_003;
    pub const tCHARACTER: i32 = 1_004;

    fn report_syntax_error(&self, ctx: &Context) {
        if self.debug { eprintln!("syntax error: {:?}", ctx) }
    }

    fn yyerror(&mut self, loc: &Loc, msg: &str) {
        if self.debug { eprintln!("yyerror: {:?} {:?}", loc, msg) }
        panic!("{:?} {}", loc, msg)
    }
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let static_env = lexer.static_env.clone();
        let context = lexer.context.clone();
        let current_arg_stack = CurrentArgStack::new();
        let max_numparam_stack = MaxNumparamStack::new();
        let pattern_variables = VariablesStack::new();
        let pattern_hash_keys = VariablesStack::new();

        Self {
            yy_error_verbose: true,
            yynerrs: 0,
            yydebug: 0,
            yyerrstatus_: 0,
            result: None,
            builder: Builder::new(
                static_env.clone(),
                context.clone(),
                current_arg_stack.clone(),
                max_numparam_stack.clone()
            ),
            context,
            current_arg_stack,
            max_numparam_stack,
            pattern_variables,
            pattern_hash_keys,
            static_env,
            yylexer: lexer,
            last_token: (0, TokenValue::String("".to_owned()), Loc { begin: 0, end: 0 }),
            tokens: vec![],
        }
    }

    pub fn do_parse(&mut self) -> Option<Node> {
        self.parse();
        std::mem::replace(&mut self.result, None)
    }

    pub fn lex(&mut self) -> Option<Vec<Token>> {
        self.do_parse()?;
        Some(
            std::mem::replace(&mut self.tokens, vec![])
        )
    }

    pub fn set_debug(&mut self, debug: bool) {
        self.yydebug = if debug { 1 } else { 0 };
        self.yylexer.set_debug(debug);
    }

    pub fn warn(&self, loc: &Loc, message: &str) {
        eprintln!("warn: {:?} {:?}", loc, message)
    }

    fn next_token(&mut self) -> Token {
        let token = self.yylexer.yylex();
        self.last_token = token.clone();
        self.tokens.push(token.clone());
        token
    }

    fn check_kwarg_name(&self, ident_t: &Token) -> Result<(), String> {
        let name = value(ident_t);
        let first_char = name.chars().nth(0).unwrap();
        if first_char.is_lowercase() {
            Ok(())
        } else {
            Err("formal argument cannot be a constant".to_owned())
        }
    }
}
