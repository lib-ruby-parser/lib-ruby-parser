/* A Bison parser, made by GNU Bison 3.8.2.  */

/* Skeleton implementation for Bison LALR(1) parsers in Rust

   Copyright (C) 2007-2015, 2018-2020 Free Software Foundation, Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

/* As a special exception, you may create a larger work that contains
   part or all of the Bison parser skeleton and distribute that work
   under terms of your choice, so long as that work isn't itself a
   parser generator using the skeleton or a modified version thereof
   as a parser skeleton.  Alternatively, if you modify or redistribute
   the parser skeleton itself, you may (at your option) remove this
   special exception, which will cause the skeleton and the resulting
   Bison output files to be licensed under the GNU General Public
   License without this special exception.

   This special exception was added by the Free Software Foundation in
   version 2.2 of Bison.  */

/* DO NOT RELY ON FEATURES THAT ARE NOT DOCUMENTED in the manual,
   especially those whose name start with YY_ or yy_.  They are
   private implementation details that can be changed or removed.  */




use std::convert::TryInto;


/* "%code use" blocks.  */
/* "src/parser/parse.y":52  */


use alloc_from_pool::{Pool, PoolValue};
use crate::{ParserOptions, ParserResult};
use crate::{Token};
use crate::{Lexer, Builder, CurrentArgStack, StaticEnvironment, MaxNumparamStack, VariablesStack};
use crate::lex_states::*;
use crate::{SharedContext as ParserContext, context::Context};
use crate::builder::{LoopType, KeywordCmd, LogicalOp, PKwLabel, ArgsType};
use crate::builder::clone_value;
use crate::parse_value::ParseValue as Value;
use crate::parse_value::*;
use crate::Node;
use crate::nodes;
use crate::{Diagnostic, DiagnosticMessage, ErrorLevel};
use crate::error::Diagnostics;
use crate::source::token_rewriter::{TokenRewriter, TokenRewriterResult, LexStateAction, RewriteAction};
use crate::Loc;


/* "src/parser/parse.rs":66  */


/// A Bison parser, automatically generated from src/parser/parse.y.
#[derive(Debug)]
pub struct  Parser  {
    /// Lexer that is used to get tokens
    pub yylexer: Lexer,
    // true if verbose error messages are enabled.
    #[allow(dead_code)]
    yy_error_verbose: bool,
    // number of errors so far
    yynerrs: i32,

    yyerrstatus_: i32,

    /* "%code parser_fields" blocks.  */
/* "src/parser/parse.y":10  */

    result: Option<Box<Node>>,
    builder: Builder,
    current_arg_stack: CurrentArgStack,
    /// Stack of sets of variables in current scopes.
    /// Each stack item represents locals in the scope.
    ///
    /// You can use it to pre-define some locals and parse
    /// your input as if these locals exist.
    ///
    /// For example, you can parse the following code
    ///
    /// ```text
    /// a = b + c
    /// ```
    ///
    /// as
    ///
    /// ```text
    /// Send(LocalVar(a), "+", LocalVar(b))
    /// ```
    ///
    /// by declaring `a` and `b` as locals using
    ///
    /// ```text
    /// parser.static_env.declare("a")
    /// parser.static_env.declare("b")
    /// parser.parse()
    /// ```
    pub static_env: StaticEnvironment,
    context: ParserContext,
    last_token_type: i32,
    max_numparam_stack: MaxNumparamStack,
    pattern_variables: VariablesStack,
    pattern_hash_keys: VariablesStack,
    tokens: Vec<Token>,
    diagnostics: Diagnostics,
    token_rewriter: Option<TokenRewriter>,
    record_tokens: bool,
    tokens_pool: Pool<Token>,

/* "src/parser/parse.rs":125  */

}

#[inline]
fn i32_to_usize(v: i32) -> usize {
    v as usize
}

/// Maps token ID into human-readable name
pub fn token_name(id: i32) -> &'static str { /* ' */
    let first_token = Lexer::YYerror;
    if id > first_token + 1 {
        let pos: usize = (id - first_token + 1)
            .try_into()
            .expect("failed to cast token id into usize, is it negative?");
        Lexer::TOKEN_NAMES[pos]
    } else if id == 0 {
        "EOF"
    } else {
        panic!("token_name fails, {} (first token = {})", id, first_token)
    }
}

/// Local alias
type YYLoc = Loc;

impl  Parser  {
    // Version number for the Bison executable that generated this parser.
    #[allow(dead_code)]
    const BISON_VERSION: &'static str = "30802";

}


fn make_yylloc(rhs: &YYStack, n: usize) -> YYLoc {
    if 0 < n {
        YYLoc {
            begin: rhs.location_at(n - 1).begin,
            end: rhs.location_at(0).end
        }
    } else {
        YYLoc {
            begin: rhs.location_at(0).end,
            end: rhs.location_at(0).end
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SymbolKind { value: i32 }

impl SymbolKind {



    #[allow(non_upper_case_globals)]
    const S_YYEOF: i32 = 0;        /* "end-of-input"  */

    #[allow(non_upper_case_globals)]
    const S_YYerror: i32 = 1;      /* error  */

    #[allow(non_upper_case_globals)]
    const S_YYUNDEF: i32 = 2;      /* "invalid token"  */

    #[allow(non_upper_case_globals)]
    const S_kCLASS: i32 = 3;       /* "`class'"  */

    #[allow(non_upper_case_globals)]
    const S_kMODULE: i32 = 4;      /* "`module'"  */

    #[allow(non_upper_case_globals)]
    const S_kDEF: i32 = 5;         /* "`def'"  */

    #[allow(non_upper_case_globals)]
    const S_kUNDEF: i32 = 6;       /* "`undef'"  */

    #[allow(non_upper_case_globals)]
    const S_kBEGIN: i32 = 7;       /* "`begin'"  */

    #[allow(non_upper_case_globals)]
    const S_kRESCUE: i32 = 8;      /* "`rescue'"  */

    #[allow(non_upper_case_globals)]
    const S_kENSURE: i32 = 9;      /* "`ensure'"  */

    #[allow(non_upper_case_globals)]
    const S_kEND: i32 = 10;        /* "`end'"  */

    #[allow(non_upper_case_globals)]
    const S_kIF: i32 = 11;         /* "`if'"  */

    #[allow(non_upper_case_globals)]
    const S_kUNLESS: i32 = 12;     /* "`unless'"  */

    #[allow(non_upper_case_globals)]
    const S_kTHEN: i32 = 13;       /* "`then'"  */

    #[allow(non_upper_case_globals)]
    const S_kELSIF: i32 = 14;      /* "`elsif'"  */

    #[allow(non_upper_case_globals)]
    const S_kELSE: i32 = 15;       /* "`else'"  */

    #[allow(non_upper_case_globals)]
    const S_kCASE: i32 = 16;       /* "`case'"  */

    #[allow(non_upper_case_globals)]
    const S_kWHEN: i32 = 17;       /* "`when'"  */

    #[allow(non_upper_case_globals)]
    const S_kWHILE: i32 = 18;      /* "`while'"  */

    #[allow(non_upper_case_globals)]
    const S_kUNTIL: i32 = 19;      /* "`until'"  */

    #[allow(non_upper_case_globals)]
    const S_kFOR: i32 = 20;        /* "`for'"  */

    #[allow(non_upper_case_globals)]
    const S_kBREAK: i32 = 21;      /* "`break'"  */

    #[allow(non_upper_case_globals)]
    const S_kNEXT: i32 = 22;       /* "`next'"  */

    #[allow(non_upper_case_globals)]
    const S_kREDO: i32 = 23;       /* "`redo'"  */

    #[allow(non_upper_case_globals)]
    const S_kRETRY: i32 = 24;      /* "`retry'"  */

    #[allow(non_upper_case_globals)]
    const S_kIN: i32 = 25;         /* "`in'"  */

    #[allow(non_upper_case_globals)]
    const S_kDO: i32 = 26;         /* "`do'"  */

    #[allow(non_upper_case_globals)]
    const S_kDO_COND: i32 = 27;    /* "`do' for condition"  */

    #[allow(non_upper_case_globals)]
    const S_kDO_BLOCK: i32 = 28;   /* "`do' for block"  */

    #[allow(non_upper_case_globals)]
    const S_kDO_LAMBDA: i32 = 29;  /* "`do' for lambda"  */

    #[allow(non_upper_case_globals)]
    const S_kRETURN: i32 = 30;     /* "`return'"  */

    #[allow(non_upper_case_globals)]
    const S_kYIELD: i32 = 31;      /* "`yield'"  */

    #[allow(non_upper_case_globals)]
    const S_kSUPER: i32 = 32;      /* "`super'"  */

    #[allow(non_upper_case_globals)]
    const S_kSELF: i32 = 33;       /* "`self'"  */

    #[allow(non_upper_case_globals)]
    const S_kNIL: i32 = 34;        /* "`nil'"  */

    #[allow(non_upper_case_globals)]
    const S_kTRUE: i32 = 35;       /* "`true'"  */

    #[allow(non_upper_case_globals)]
    const S_kFALSE: i32 = 36;      /* "`false'"  */

    #[allow(non_upper_case_globals)]
    const S_kAND: i32 = 37;        /* "`and'"  */

    #[allow(non_upper_case_globals)]
    const S_kOR: i32 = 38;         /* "`or'"  */

    #[allow(non_upper_case_globals)]
    const S_kNOT: i32 = 39;        /* "`not'"  */

    #[allow(non_upper_case_globals)]
    const S_kIF_MOD: i32 = 40;     /* "`if' modifier"  */

    #[allow(non_upper_case_globals)]
    const S_kUNLESS_MOD: i32 = 41; /* "`unless' modifier"  */

    #[allow(non_upper_case_globals)]
    const S_kWHILE_MOD: i32 = 42;  /* "`while' modifier"  */

    #[allow(non_upper_case_globals)]
    const S_kUNTIL_MOD: i32 = 43;  /* "`until' modifier"  */

    #[allow(non_upper_case_globals)]
    const S_kRESCUE_MOD: i32 = 44; /* "`rescue' modifier"  */

    #[allow(non_upper_case_globals)]
    const S_kALIAS: i32 = 45;      /* "`alias'"  */

    #[allow(non_upper_case_globals)]
    const S_kDEFINED: i32 = 46;    /* "`defined?'"  */

    #[allow(non_upper_case_globals)]
    const S_klBEGIN: i32 = 47;     /* "`BEGIN'"  */

    #[allow(non_upper_case_globals)]
    const S_klEND: i32 = 48;       /* "`END'"  */

    #[allow(non_upper_case_globals)]
    const S_k__LINE__: i32 = 49;   /* "`__LINE__'"  */

    #[allow(non_upper_case_globals)]
    const S_k__FILE__: i32 = 50;   /* "`__FILE__'"  */

    #[allow(non_upper_case_globals)]
    const S_k__ENCODING__: i32 = 51; /* "`__ENCODING__'"  */

    #[allow(non_upper_case_globals)]
    const S_tIDENTIFIER: i32 = 52; /* "local variable or method"  */

    #[allow(non_upper_case_globals)]
    const S_tFID: i32 = 53;        /* "method"  */

    #[allow(non_upper_case_globals)]
    const S_tGVAR: i32 = 54;       /* "global variable"  */

    #[allow(non_upper_case_globals)]
    const S_tIVAR: i32 = 55;       /* "instance variable"  */

    #[allow(non_upper_case_globals)]
    const S_tCONSTANT: i32 = 56;   /* "constant"  */

    #[allow(non_upper_case_globals)]
    const S_tCVAR: i32 = 57;       /* "class variable"  */

    #[allow(non_upper_case_globals)]
    const S_tLABEL: i32 = 58;      /* "label"  */

    #[allow(non_upper_case_globals)]
    const S_tINTEGER: i32 = 59;    /* "integer literal"  */

    #[allow(non_upper_case_globals)]
    const S_tFLOAT: i32 = 60;      /* "float literal"  */

    #[allow(non_upper_case_globals)]
    const S_tRATIONAL: i32 = 61;   /* "rational literal"  */

    #[allow(non_upper_case_globals)]
    const S_tIMAGINARY: i32 = 62;  /* "imaginary literal"  */

    #[allow(non_upper_case_globals)]
    const S_tCHAR: i32 = 63;       /* "char literal"  */

    #[allow(non_upper_case_globals)]
    const S_tNTH_REF: i32 = 64;    /* "numbered reference"  */

    #[allow(non_upper_case_globals)]
    const S_tBACK_REF: i32 = 65;   /* "back reference"  */

    #[allow(non_upper_case_globals)]
    const S_tSTRING_CONTENT: i32 = 66; /* "literal content"  */

    #[allow(non_upper_case_globals)]
    const S_tREGEXP_END: i32 = 67; /* tREGEXP_END  */

    #[allow(non_upper_case_globals)]
    const S_tDOT: i32 = 68;        /* tDOT  */

    #[allow(non_upper_case_globals)]
    const S_tBACKSLASH: i32 = 69;  /* "backslash"  */

    #[allow(non_upper_case_globals)]
    const S_tSP: i32 = 70;         /* "escaped space"  */

    #[allow(non_upper_case_globals)]
    const S_tSLASH_T: i32 = 71;    /* "escaped horizontal tab"  */

    #[allow(non_upper_case_globals)]
    const S_tSLASH_F: i32 = 72;    /* "escaped form feed"  */

    #[allow(non_upper_case_globals)]
    const S_tSLASH_R: i32 = 73;    /* "escaped carriage return"  */

    #[allow(non_upper_case_globals)]
    const S_tVTAB: i32 = 74;       /* "escaped vertical tab"  */

    #[allow(non_upper_case_globals)]
    const S_tUPLUS: i32 = 75;      /* "unary+"  */

    #[allow(non_upper_case_globals)]
    const S_tUMINUS: i32 = 76;     /* "unary-"  */

    #[allow(non_upper_case_globals)]
    const S_tPOW: i32 = 77;        /* "**"  */

    #[allow(non_upper_case_globals)]
    const S_tCMP: i32 = 78;        /* "<=>"  */

    #[allow(non_upper_case_globals)]
    const S_tEQ: i32 = 79;         /* "=="  */

    #[allow(non_upper_case_globals)]
    const S_tEQQ: i32 = 80;        /* "==="  */

    #[allow(non_upper_case_globals)]
    const S_tNEQ: i32 = 81;        /* "!="  */

    #[allow(non_upper_case_globals)]
    const S_tGEQ: i32 = 82;        /* ">="  */

    #[allow(non_upper_case_globals)]
    const S_tLEQ: i32 = 83;        /* "<="  */

    #[allow(non_upper_case_globals)]
    const S_tANDOP: i32 = 84;      /* "&&"  */

    #[allow(non_upper_case_globals)]
    const S_tOROP: i32 = 85;       /* "||"  */

    #[allow(non_upper_case_globals)]
    const S_tMATCH: i32 = 86;      /* "=~"  */

    #[allow(non_upper_case_globals)]
    const S_tNMATCH: i32 = 87;     /* "!~"  */

    #[allow(non_upper_case_globals)]
    const S_tDOT2: i32 = 88;       /* ".."  */

    #[allow(non_upper_case_globals)]
    const S_tDOT3: i32 = 89;       /* "..."  */

    #[allow(non_upper_case_globals)]
    const S_tBDOT2: i32 = 90;      /* "(.."  */

    #[allow(non_upper_case_globals)]
    const S_tBDOT3: i32 = 91;      /* "(..."  */

    #[allow(non_upper_case_globals)]
    const S_tAREF: i32 = 92;       /* "[]"  */

    #[allow(non_upper_case_globals)]
    const S_tASET: i32 = 93;       /* "[]="  */

    #[allow(non_upper_case_globals)]
    const S_tLSHFT: i32 = 94;      /* "<<"  */

    #[allow(non_upper_case_globals)]
    const S_tRSHFT: i32 = 95;      /* ">>"  */

    #[allow(non_upper_case_globals)]
    const S_tANDDOT: i32 = 96;     /* "&."  */

    #[allow(non_upper_case_globals)]
    const S_tCOLON2: i32 = 97;     /* "::"  */

    #[allow(non_upper_case_globals)]
    const S_tCOLON3: i32 = 98;     /* ":: at EXPR_BEG"  */

    #[allow(non_upper_case_globals)]
    const S_tOP_ASGN: i32 = 99;    /* "operator-assignment"  */

    #[allow(non_upper_case_globals)]
    const S_tASSOC: i32 = 100;     /* "=>"  */

    #[allow(non_upper_case_globals)]
    const S_tLPAREN: i32 = 101;    /* "("  */

    #[allow(non_upper_case_globals)]
    const S_tLPAREN_ARG: i32 = 102; /* "( arg"  */

    #[allow(non_upper_case_globals)]
    const S_tRPAREN: i32 = 103;    /* ")"  */

    #[allow(non_upper_case_globals)]
    const S_tLBRACK: i32 = 104;    /* "["  */

    #[allow(non_upper_case_globals)]
    const S_tLBRACE: i32 = 105;    /* "{"  */

    #[allow(non_upper_case_globals)]
    const S_tLBRACE_ARG: i32 = 106; /* "{ arg"  */

    #[allow(non_upper_case_globals)]
    const S_tSTAR: i32 = 107;      /* "*"  */

    #[allow(non_upper_case_globals)]
    const S_tDSTAR: i32 = 108;     /* "**arg"  */

    #[allow(non_upper_case_globals)]
    const S_tAMPER: i32 = 109;     /* "&"  */

    #[allow(non_upper_case_globals)]
    const S_tLAMBDA: i32 = 110;    /* "->"  */

    #[allow(non_upper_case_globals)]
    const S_tSYMBEG: i32 = 111;    /* "symbol literal"  */

    #[allow(non_upper_case_globals)]
    const S_tSTRING_BEG: i32 = 112; /* "string begin"  */

    #[allow(non_upper_case_globals)]
    const S_tXSTRING_BEG: i32 = 113; /* "backtick literal"  */

    #[allow(non_upper_case_globals)]
    const S_tREGEXP_BEG: i32 = 114; /* "regexp literal"  */

    #[allow(non_upper_case_globals)]
    const S_tWORDS_BEG: i32 = 115; /* "word list"  */

    #[allow(non_upper_case_globals)]
    const S_tQWORDS_BEG: i32 = 116; /* "verbatim word list"  */

    #[allow(non_upper_case_globals)]
    const S_tSYMBOLS_BEG: i32 = 117; /* "symbol list"  */

    #[allow(non_upper_case_globals)]
    const S_tQSYMBOLS_BEG: i32 = 118; /* "verbatim symbol list"  */

    #[allow(non_upper_case_globals)]
    const S_tSTRING_END: i32 = 119; /* "string end"  */

    #[allow(non_upper_case_globals)]
    const S_tSTRING_DEND: i32 = 120; /* "tRCURLY"  */

    #[allow(non_upper_case_globals)]
    const S_tSTRING_DBEG: i32 = 121; /* tSTRING_DBEG  */

    #[allow(non_upper_case_globals)]
    const S_tSTRING_DVAR: i32 = 122; /* tSTRING_DVAR  */

    #[allow(non_upper_case_globals)]
    const S_tLAMBEG: i32 = 123;    /* tLAMBEG  */

    #[allow(non_upper_case_globals)]
    const S_tLABEL_END: i32 = 124; /* tLABEL_END  */

    #[allow(non_upper_case_globals)]
    const S_tCOMMA: i32 = 125;     /* ","  */

    #[allow(non_upper_case_globals)]
    const S_tLCURLY: i32 = 126;    /* "{ (tLCURLY)"  */

    #[allow(non_upper_case_globals)]
    const S_tRCURLY: i32 = 127;    /* "}"  */

    #[allow(non_upper_case_globals)]
    const S_tLBRACK2: i32 = 128;   /* "[ (tLBRACK2)"  */

    #[allow(non_upper_case_globals)]
    const S_tEQL: i32 = 129;       /* "="  */

    #[allow(non_upper_case_globals)]
    const S_tPIPE: i32 = 130;      /* "|"  */

    #[allow(non_upper_case_globals)]
    const S_tAMPER2: i32 = 131;    /* "& (tAMPER2)"  */

    #[allow(non_upper_case_globals)]
    const S_tGT: i32 = 132;        /* ">"  */

    #[allow(non_upper_case_globals)]
    const S_tLT: i32 = 133;        /* "<"  */

    #[allow(non_upper_case_globals)]
    const S_tBACK_REF2: i32 = 134; /* "`"  */

    #[allow(non_upper_case_globals)]
    const S_tCARET: i32 = 135;     /* "^"  */

    #[allow(non_upper_case_globals)]
    const S_tLPAREN2: i32 = 136;   /* "( (tLPAREN2)"  */

    #[allow(non_upper_case_globals)]
    const S_tRBRACK: i32 = 137;    /* "]"  */

    #[allow(non_upper_case_globals)]
    const S_tSEMI: i32 = 138;      /* ";"  */

    #[allow(non_upper_case_globals)]
    const S_tSPACE: i32 = 139;     /* " "  */

    #[allow(non_upper_case_globals)]
    const S_tNL: i32 = 140;        /* "\n"  */

    #[allow(non_upper_case_globals)]
    const S_tPLUS: i32 = 141;      /* "+"  */

    #[allow(non_upper_case_globals)]
    const S_tMINUS: i32 = 142;     /* "-"  */

    #[allow(non_upper_case_globals)]
    const S_tSTAR2: i32 = 143;     /* "* (tSTAR2)"  */

    #[allow(non_upper_case_globals)]
    const S_tDIVIDE: i32 = 144;    /* "/"  */

    #[allow(non_upper_case_globals)]
    const S_tPERCENT: i32 = 145;   /* "%"  */

    #[allow(non_upper_case_globals)]
    const S_tTILDE: i32 = 146;     /* "~"  */

    #[allow(non_upper_case_globals)]
    const S_tBANG: i32 = 147;      /* "!"  */

    #[allow(non_upper_case_globals)]
    const S_tLOWEST: i32 = 148;    /* tLOWEST  */

    #[allow(non_upper_case_globals)]
    const S_tEH: i32 = 149;        /* tEH  */

    #[allow(non_upper_case_globals)]
    const S_tCOLON: i32 = 150;     /* tCOLON  */

    #[allow(non_upper_case_globals)]
    const S_tUMINUS_NUM: i32 = 151; /* tUMINUS_NUM  */

    #[allow(non_upper_case_globals)]
    const S_tLAST_TOKEN: i32 = 152; /* tLAST_TOKEN  */

    #[allow(non_upper_case_globals)]
    const S_YYACCEPT: i32 = 153;   /* $accept  */

    #[allow(non_upper_case_globals)]
    const S_program: i32 = 154;    /* program  */

    #[allow(non_upper_case_globals)]
    const S_155_1: i32 = 155;      /* @1  */

    #[allow(non_upper_case_globals)]
    const S_top_compstmt: i32 = 156; /* top_compstmt  */

    #[allow(non_upper_case_globals)]
    const S_top_stmts: i32 = 157;  /* top_stmts  */

    #[allow(non_upper_case_globals)]
    const S_top_stmt: i32 = 158;   /* top_stmt  */

    #[allow(non_upper_case_globals)]
    const S_begin_block: i32 = 159; /* begin_block  */

    #[allow(non_upper_case_globals)]
    const S_bodystmt: i32 = 160;   /* bodystmt  */

    #[allow(non_upper_case_globals)]
    const S_compstmt: i32 = 161;   /* compstmt  */

    #[allow(non_upper_case_globals)]
    const S_stmts: i32 = 162;      /* stmts  */

    #[allow(non_upper_case_globals)]
    const S_stmt_or_begin: i32 = 163; /* stmt_or_begin  */

    #[allow(non_upper_case_globals)]
    const S_164_2: i32 = 164;      /* $@2  */

    #[allow(non_upper_case_globals)]
    const S_stmt: i32 = 165;       /* stmt  */

    #[allow(non_upper_case_globals)]
    const S_166_3: i32 = 166;      /* @3  */

    #[allow(non_upper_case_globals)]
    const S_command_asgn: i32 = 167; /* command_asgn  */

    #[allow(non_upper_case_globals)]
    const S_command_rhs: i32 = 168; /* command_rhs  */

    #[allow(non_upper_case_globals)]
    const S_expr: i32 = 169;       /* expr  */

    #[allow(non_upper_case_globals)]
    const S_170_4: i32 = 170;      /* @4  */

    #[allow(non_upper_case_globals)]
    const S_171_5: i32 = 171;      /* @5  */

    #[allow(non_upper_case_globals)]
    const S_def_name: i32 = 172;   /* def_name  */

    #[allow(non_upper_case_globals)]
    const S_defn_head: i32 = 173;  /* defn_head  */

    #[allow(non_upper_case_globals)]
    const S_defs_head: i32 = 174;  /* defs_head  */

    #[allow(non_upper_case_globals)]
    const S_175_6: i32 = 175;      /* @6  */

    #[allow(non_upper_case_globals)]
    const S_expr_value: i32 = 176; /* expr_value  */

    #[allow(non_upper_case_globals)]
    const S_expr_value_do: i32 = 177; /* expr_value_do  */

    #[allow(non_upper_case_globals)]
    const S_178_7: i32 = 178;      /* @7  */

    #[allow(non_upper_case_globals)]
    const S_command_call: i32 = 179; /* command_call  */

    #[allow(non_upper_case_globals)]
    const S_block_command: i32 = 180; /* block_command  */

    #[allow(non_upper_case_globals)]
    const S_cmd_brace_block: i32 = 181; /* cmd_brace_block  */

    #[allow(non_upper_case_globals)]
    const S_182_8: i32 = 182;      /* @8  */

    #[allow(non_upper_case_globals)]
    const S_fcall: i32 = 183;      /* fcall  */

    #[allow(non_upper_case_globals)]
    const S_command: i32 = 184;    /* command  */

    #[allow(non_upper_case_globals)]
    const S_mlhs: i32 = 185;       /* mlhs  */

    #[allow(non_upper_case_globals)]
    const S_mlhs_inner: i32 = 186; /* mlhs_inner  */

    #[allow(non_upper_case_globals)]
    const S_mlhs_basic: i32 = 187; /* mlhs_basic  */

    #[allow(non_upper_case_globals)]
    const S_mlhs_item: i32 = 188;  /* mlhs_item  */

    #[allow(non_upper_case_globals)]
    const S_mlhs_head: i32 = 189;  /* mlhs_head  */

    #[allow(non_upper_case_globals)]
    const S_mlhs_post: i32 = 190;  /* mlhs_post  */

    #[allow(non_upper_case_globals)]
    const S_mlhs_node: i32 = 191;  /* mlhs_node  */

    #[allow(non_upper_case_globals)]
    const S_lhs: i32 = 192;        /* lhs  */

    #[allow(non_upper_case_globals)]
    const S_cname: i32 = 193;      /* cname  */

    #[allow(non_upper_case_globals)]
    const S_cpath: i32 = 194;      /* cpath  */

    #[allow(non_upper_case_globals)]
    const S_fname: i32 = 195;      /* fname  */

    #[allow(non_upper_case_globals)]
    const S_fitem: i32 = 196;      /* fitem  */

    #[allow(non_upper_case_globals)]
    const S_undef_list: i32 = 197; /* undef_list  */

    #[allow(non_upper_case_globals)]
    const S_198_9: i32 = 198;      /* @9  */

    #[allow(non_upper_case_globals)]
    const S_op: i32 = 199;         /* op  */

    #[allow(non_upper_case_globals)]
    const S_reswords: i32 = 200;   /* reswords  */

    #[allow(non_upper_case_globals)]
    const S_arg: i32 = 201;        /* arg  */

    #[allow(non_upper_case_globals)]
    const S_202_10: i32 = 202;     /* @10  */

    #[allow(non_upper_case_globals)]
    const S_relop: i32 = 203;      /* relop  */

    #[allow(non_upper_case_globals)]
    const S_rel_expr: i32 = 204;   /* rel_expr  */

    #[allow(non_upper_case_globals)]
    const S_arg_value: i32 = 205;  /* arg_value  */

    #[allow(non_upper_case_globals)]
    const S_aref_args: i32 = 206;  /* aref_args  */

    #[allow(non_upper_case_globals)]
    const S_arg_rhs: i32 = 207;    /* arg_rhs  */

    #[allow(non_upper_case_globals)]
    const S_paren_args: i32 = 208; /* paren_args  */

    #[allow(non_upper_case_globals)]
    const S_opt_paren_args: i32 = 209; /* opt_paren_args  */

    #[allow(non_upper_case_globals)]
    const S_opt_call_args: i32 = 210; /* opt_call_args  */

    #[allow(non_upper_case_globals)]
    const S_call_args: i32 = 211;  /* call_args  */

    #[allow(non_upper_case_globals)]
    const S_command_args: i32 = 212; /* command_args  */

    #[allow(non_upper_case_globals)]
    const S_213_11: i32 = 213;     /* @11  */

    #[allow(non_upper_case_globals)]
    const S_block_arg: i32 = 214;  /* block_arg  */

    #[allow(non_upper_case_globals)]
    const S_opt_block_arg: i32 = 215; /* opt_block_arg  */

    #[allow(non_upper_case_globals)]
    const S_args: i32 = 216;       /* args  */

    #[allow(non_upper_case_globals)]
    const S_mrhs_arg: i32 = 217;   /* mrhs_arg  */

    #[allow(non_upper_case_globals)]
    const S_mrhs: i32 = 218;       /* mrhs  */

    #[allow(non_upper_case_globals)]
    const S_primary: i32 = 219;    /* primary  */

    #[allow(non_upper_case_globals)]
    const S_220_12: i32 = 220;     /* @12  */

    #[allow(non_upper_case_globals)]
    const S_221_13: i32 = 221;     /* @13  */

    #[allow(non_upper_case_globals)]
    const S_222_14: i32 = 222;     /* @14  */

    #[allow(non_upper_case_globals)]
    const S_223_15: i32 = 223;     /* @15  */

    #[allow(non_upper_case_globals)]
    const S_224_16: i32 = 224;     /* @16  */

    #[allow(non_upper_case_globals)]
    const S_225_17: i32 = 225;     /* @17  */

    #[allow(non_upper_case_globals)]
    const S_226_18: i32 = 226;     /* @18  */

    #[allow(non_upper_case_globals)]
    const S_227_19: i32 = 227;     /* @19  */

    #[allow(non_upper_case_globals)]
    const S_228_20: i32 = 228;     /* @20  */

    #[allow(non_upper_case_globals)]
    const S_primary_value: i32 = 229; /* primary_value  */

    #[allow(non_upper_case_globals)]
    const S_k_begin: i32 = 230;    /* k_begin  */

    #[allow(non_upper_case_globals)]
    const S_k_if: i32 = 231;       /* k_if  */

    #[allow(non_upper_case_globals)]
    const S_k_unless: i32 = 232;   /* k_unless  */

    #[allow(non_upper_case_globals)]
    const S_k_while: i32 = 233;    /* k_while  */

    #[allow(non_upper_case_globals)]
    const S_k_until: i32 = 234;    /* k_until  */

    #[allow(non_upper_case_globals)]
    const S_k_case: i32 = 235;     /* k_case  */

    #[allow(non_upper_case_globals)]
    const S_k_for: i32 = 236;      /* k_for  */

    #[allow(non_upper_case_globals)]
    const S_k_class: i32 = 237;    /* k_class  */

    #[allow(non_upper_case_globals)]
    const S_k_module: i32 = 238;   /* k_module  */

    #[allow(non_upper_case_globals)]
    const S_k_def: i32 = 239;      /* k_def  */

    #[allow(non_upper_case_globals)]
    const S_k_do: i32 = 240;       /* k_do  */

    #[allow(non_upper_case_globals)]
    const S_k_do_block: i32 = 241; /* k_do_block  */

    #[allow(non_upper_case_globals)]
    const S_k_rescue: i32 = 242;   /* k_rescue  */

    #[allow(non_upper_case_globals)]
    const S_k_ensure: i32 = 243;   /* k_ensure  */

    #[allow(non_upper_case_globals)]
    const S_k_when: i32 = 244;     /* k_when  */

    #[allow(non_upper_case_globals)]
    const S_k_else: i32 = 245;     /* k_else  */

    #[allow(non_upper_case_globals)]
    const S_k_elsif: i32 = 246;    /* k_elsif  */

    #[allow(non_upper_case_globals)]
    const S_k_end: i32 = 247;      /* k_end  */

    #[allow(non_upper_case_globals)]
    const S_k_return: i32 = 248;   /* k_return  */

    #[allow(non_upper_case_globals)]
    const S_then: i32 = 249;       /* then  */

    #[allow(non_upper_case_globals)]
    const S_do: i32 = 250;         /* do  */

    #[allow(non_upper_case_globals)]
    const S_if_tail: i32 = 251;    /* if_tail  */

    #[allow(non_upper_case_globals)]
    const S_opt_else: i32 = 252;   /* opt_else  */

    #[allow(non_upper_case_globals)]
    const S_for_var: i32 = 253;    /* for_var  */

    #[allow(non_upper_case_globals)]
    const S_f_marg: i32 = 254;     /* f_marg  */

    #[allow(non_upper_case_globals)]
    const S_f_marg_list: i32 = 255; /* f_marg_list  */

    #[allow(non_upper_case_globals)]
    const S_f_margs: i32 = 256;    /* f_margs  */

    #[allow(non_upper_case_globals)]
    const S_f_rest_marg: i32 = 257; /* f_rest_marg  */

    #[allow(non_upper_case_globals)]
    const S_f_any_kwrest: i32 = 258; /* f_any_kwrest  */

    #[allow(non_upper_case_globals)]
    const S_f_eq: i32 = 259;       /* f_eq  */

    #[allow(non_upper_case_globals)]
    const S_260_21: i32 = 260;     /* @21  */

    #[allow(non_upper_case_globals)]
    const S_block_args_tail: i32 = 261; /* block_args_tail  */

    #[allow(non_upper_case_globals)]
    const S_opt_block_args_tail: i32 = 262; /* opt_block_args_tail  */

    #[allow(non_upper_case_globals)]
    const S_excessed_comma: i32 = 263; /* excessed_comma  */

    #[allow(non_upper_case_globals)]
    const S_block_param: i32 = 264; /* block_param  */

    #[allow(non_upper_case_globals)]
    const S_opt_block_param: i32 = 265; /* opt_block_param  */

    #[allow(non_upper_case_globals)]
    const S_block_param_def: i32 = 266; /* block_param_def  */

    #[allow(non_upper_case_globals)]
    const S_opt_bv_decl: i32 = 267; /* opt_bv_decl  */

    #[allow(non_upper_case_globals)]
    const S_bv_decls: i32 = 268;   /* bv_decls  */

    #[allow(non_upper_case_globals)]
    const S_bvar: i32 = 269;       /* bvar  */

    #[allow(non_upper_case_globals)]
    const S_lambda: i32 = 270;     /* lambda  */

    #[allow(non_upper_case_globals)]
    const S_271_22: i32 = 271;     /* @22  */

    #[allow(non_upper_case_globals)]
    const S_272_23: i32 = 272;     /* @23  */

    #[allow(non_upper_case_globals)]
    const S_273_24: i32 = 273;     /* @24  */

    #[allow(non_upper_case_globals)]
    const S_f_larglist: i32 = 274; /* f_larglist  */

    #[allow(non_upper_case_globals)]
    const S_lambda_body: i32 = 275; /* lambda_body  */

    #[allow(non_upper_case_globals)]
    const S_276_25: i32 = 276;     /* @25  */

    #[allow(non_upper_case_globals)]
    const S_277_26: i32 = 277;     /* @26  */

    #[allow(non_upper_case_globals)]
    const S_do_block: i32 = 278;   /* do_block  */

    #[allow(non_upper_case_globals)]
    const S_279_27: i32 = 279;     /* @27  */

    #[allow(non_upper_case_globals)]
    const S_block_call: i32 = 280; /* block_call  */

    #[allow(non_upper_case_globals)]
    const S_method_call: i32 = 281; /* method_call  */

    #[allow(non_upper_case_globals)]
    const S_brace_block: i32 = 282; /* brace_block  */

    #[allow(non_upper_case_globals)]
    const S_283_28: i32 = 283;     /* @28  */

    #[allow(non_upper_case_globals)]
    const S_284_29: i32 = 284;     /* @29  */

    #[allow(non_upper_case_globals)]
    const S_brace_body: i32 = 285; /* brace_body  */

    #[allow(non_upper_case_globals)]
    const S_286_30: i32 = 286;     /* @30  */

    #[allow(non_upper_case_globals)]
    const S_do_body: i32 = 287;    /* do_body  */

    #[allow(non_upper_case_globals)]
    const S_288_31: i32 = 288;     /* @31  */

    #[allow(non_upper_case_globals)]
    const S_case_args: i32 = 289;  /* case_args  */

    #[allow(non_upper_case_globals)]
    const S_case_body: i32 = 290;  /* case_body  */

    #[allow(non_upper_case_globals)]
    const S_cases: i32 = 291;      /* cases  */

    #[allow(non_upper_case_globals)]
    const S_p_case_body: i32 = 292; /* p_case_body  */

    #[allow(non_upper_case_globals)]
    const S_293_32: i32 = 293;     /* @32  */

    #[allow(non_upper_case_globals)]
    const S_294_33: i32 = 294;     /* @33  */

    #[allow(non_upper_case_globals)]
    const S_p_cases: i32 = 295;    /* p_cases  */

    #[allow(non_upper_case_globals)]
    const S_p_top_expr: i32 = 296; /* p_top_expr  */

    #[allow(non_upper_case_globals)]
    const S_p_top_expr_body: i32 = 297; /* p_top_expr_body  */

    #[allow(non_upper_case_globals)]
    const S_p_expr: i32 = 298;     /* p_expr  */

    #[allow(non_upper_case_globals)]
    const S_p_as: i32 = 299;       /* p_as  */

    #[allow(non_upper_case_globals)]
    const S_p_alt: i32 = 300;      /* p_alt  */

    #[allow(non_upper_case_globals)]
    const S_p_lparen: i32 = 301;   /* p_lparen  */

    #[allow(non_upper_case_globals)]
    const S_p_lbracket: i32 = 302; /* p_lbracket  */

    #[allow(non_upper_case_globals)]
    const S_p_expr_basic: i32 = 303; /* p_expr_basic  */

    #[allow(non_upper_case_globals)]
    const S_304_34: i32 = 304;     /* @34  */

    #[allow(non_upper_case_globals)]
    const S_305_35: i32 = 305;     /* @35  */

    #[allow(non_upper_case_globals)]
    const S_p_args: i32 = 306;     /* p_args  */

    #[allow(non_upper_case_globals)]
    const S_p_args_head: i32 = 307; /* p_args_head  */

    #[allow(non_upper_case_globals)]
    const S_p_args_tail: i32 = 308; /* p_args_tail  */

    #[allow(non_upper_case_globals)]
    const S_p_find: i32 = 309;     /* p_find  */

    #[allow(non_upper_case_globals)]
    const S_p_rest: i32 = 310;     /* p_rest  */

    #[allow(non_upper_case_globals)]
    const S_p_args_post: i32 = 311; /* p_args_post  */

    #[allow(non_upper_case_globals)]
    const S_p_arg: i32 = 312;      /* p_arg  */

    #[allow(non_upper_case_globals)]
    const S_p_kwargs: i32 = 313;   /* p_kwargs  */

    #[allow(non_upper_case_globals)]
    const S_p_kwarg: i32 = 314;    /* p_kwarg  */

    #[allow(non_upper_case_globals)]
    const S_p_kw: i32 = 315;       /* p_kw  */

    #[allow(non_upper_case_globals)]
    const S_p_kw_label: i32 = 316; /* p_kw_label  */

    #[allow(non_upper_case_globals)]
    const S_p_kwrest: i32 = 317;   /* p_kwrest  */

    #[allow(non_upper_case_globals)]
    const S_p_kwnorest: i32 = 318; /* p_kwnorest  */

    #[allow(non_upper_case_globals)]
    const S_p_any_kwrest: i32 = 319; /* p_any_kwrest  */

    #[allow(non_upper_case_globals)]
    const S_p_value: i32 = 320;    /* p_value  */

    #[allow(non_upper_case_globals)]
    const S_p_primitive: i32 = 321; /* p_primitive  */

    #[allow(non_upper_case_globals)]
    const S_p_variable: i32 = 322; /* p_variable  */

    #[allow(non_upper_case_globals)]
    const S_p_var_ref: i32 = 323;  /* p_var_ref  */

    #[allow(non_upper_case_globals)]
    const S_p_expr_ref: i32 = 324; /* p_expr_ref  */

    #[allow(non_upper_case_globals)]
    const S_p_const: i32 = 325;    /* p_const  */

    #[allow(non_upper_case_globals)]
    const S_opt_rescue: i32 = 326; /* opt_rescue  */

    #[allow(non_upper_case_globals)]
    const S_exc_list: i32 = 327;   /* exc_list  */

    #[allow(non_upper_case_globals)]
    const S_exc_var: i32 = 328;    /* exc_var  */

    #[allow(non_upper_case_globals)]
    const S_opt_ensure: i32 = 329; /* opt_ensure  */

    #[allow(non_upper_case_globals)]
    const S_literal: i32 = 330;    /* literal  */

    #[allow(non_upper_case_globals)]
    const S_strings: i32 = 331;    /* strings  */

    #[allow(non_upper_case_globals)]
    const S_string: i32 = 332;     /* string  */

    #[allow(non_upper_case_globals)]
    const S_string1: i32 = 333;    /* string1  */

    #[allow(non_upper_case_globals)]
    const S_xstring: i32 = 334;    /* xstring  */

    #[allow(non_upper_case_globals)]
    const S_regexp: i32 = 335;     /* regexp  */

    #[allow(non_upper_case_globals)]
    const S_words: i32 = 336;      /* words  */

    #[allow(non_upper_case_globals)]
    const S_word_list: i32 = 337;  /* word_list  */

    #[allow(non_upper_case_globals)]
    const S_word: i32 = 338;       /* word  */

    #[allow(non_upper_case_globals)]
    const S_symbols: i32 = 339;    /* symbols  */

    #[allow(non_upper_case_globals)]
    const S_symbol_list: i32 = 340; /* symbol_list  */

    #[allow(non_upper_case_globals)]
    const S_qwords: i32 = 341;     /* qwords  */

    #[allow(non_upper_case_globals)]
    const S_qsymbols: i32 = 342;   /* qsymbols  */

    #[allow(non_upper_case_globals)]
    const S_qword_list: i32 = 343; /* qword_list  */

    #[allow(non_upper_case_globals)]
    const S_qsym_list: i32 = 344;  /* qsym_list  */

    #[allow(non_upper_case_globals)]
    const S_string_contents: i32 = 345; /* string_contents  */

    #[allow(non_upper_case_globals)]
    const S_xstring_contents: i32 = 346; /* xstring_contents  */

    #[allow(non_upper_case_globals)]
    const S_regexp_contents: i32 = 347; /* regexp_contents  */

    #[allow(non_upper_case_globals)]
    const S_string_content: i32 = 348; /* string_content  */

    #[allow(non_upper_case_globals)]
    const S_349_36: i32 = 349;     /* @36  */

    #[allow(non_upper_case_globals)]
    const S_350_37: i32 = 350;     /* @37  */

    #[allow(non_upper_case_globals)]
    const S_351_38: i32 = 351;     /* @38  */

    #[allow(non_upper_case_globals)]
    const S_352_39: i32 = 352;     /* @39  */

    #[allow(non_upper_case_globals)]
    const S_353_40: i32 = 353;     /* @40  */

    #[allow(non_upper_case_globals)]
    const S_354_41: i32 = 354;     /* @41  */

    #[allow(non_upper_case_globals)]
    const S_string_dvar: i32 = 355; /* string_dvar  */

    #[allow(non_upper_case_globals)]
    const S_symbol: i32 = 356;     /* symbol  */

    #[allow(non_upper_case_globals)]
    const S_ssym: i32 = 357;       /* ssym  */

    #[allow(non_upper_case_globals)]
    const S_sym: i32 = 358;        /* sym  */

    #[allow(non_upper_case_globals)]
    const S_dsym: i32 = 359;       /* dsym  */

    #[allow(non_upper_case_globals)]
    const S_numeric: i32 = 360;    /* numeric  */

    #[allow(non_upper_case_globals)]
    const S_simple_numeric: i32 = 361; /* simple_numeric  */

    #[allow(non_upper_case_globals)]
    const S_nonlocal_var: i32 = 362; /* nonlocal_var  */

    #[allow(non_upper_case_globals)]
    const S_user_variable: i32 = 363; /* user_variable  */

    #[allow(non_upper_case_globals)]
    const S_keyword_variable: i32 = 364; /* keyword_variable  */

    #[allow(non_upper_case_globals)]
    const S_var_ref: i32 = 365;    /* var_ref  */

    #[allow(non_upper_case_globals)]
    const S_var_lhs: i32 = 366;    /* var_lhs  */

    #[allow(non_upper_case_globals)]
    const S_backref: i32 = 367;    /* backref  */

    #[allow(non_upper_case_globals)]
    const S_superclass: i32 = 368; /* superclass  */

    #[allow(non_upper_case_globals)]
    const S_369_42: i32 = 369;     /* @42  */

    #[allow(non_upper_case_globals)]
    const S_f_opt_paren_args: i32 = 370; /* f_opt_paren_args  */

    #[allow(non_upper_case_globals)]
    const S_f_paren_args: i32 = 371; /* f_paren_args  */

    #[allow(non_upper_case_globals)]
    const S_f_arglist: i32 = 372;  /* f_arglist  */

    #[allow(non_upper_case_globals)]
    const S_373_43: i32 = 373;     /* @43  */

    #[allow(non_upper_case_globals)]
    const S_args_tail: i32 = 374;  /* args_tail  */

    #[allow(non_upper_case_globals)]
    const S_opt_args_tail: i32 = 375; /* opt_args_tail  */

    #[allow(non_upper_case_globals)]
    const S_f_args: i32 = 376;     /* f_args  */

    #[allow(non_upper_case_globals)]
    const S_args_forward: i32 = 377; /* args_forward  */

    #[allow(non_upper_case_globals)]
    const S_f_bad_arg: i32 = 378;  /* f_bad_arg  */

    #[allow(non_upper_case_globals)]
    const S_f_norm_arg: i32 = 379; /* f_norm_arg  */

    #[allow(non_upper_case_globals)]
    const S_f_arg_asgn: i32 = 380; /* f_arg_asgn  */

    #[allow(non_upper_case_globals)]
    const S_f_arg_item: i32 = 381; /* f_arg_item  */

    #[allow(non_upper_case_globals)]
    const S_f_arg: i32 = 382;      /* f_arg  */

    #[allow(non_upper_case_globals)]
    const S_f_label: i32 = 383;    /* f_label  */

    #[allow(non_upper_case_globals)]
    const S_f_kw: i32 = 384;       /* f_kw  */

    #[allow(non_upper_case_globals)]
    const S_f_block_kw: i32 = 385; /* f_block_kw  */

    #[allow(non_upper_case_globals)]
    const S_f_block_kwarg: i32 = 386; /* f_block_kwarg  */

    #[allow(non_upper_case_globals)]
    const S_f_kwarg: i32 = 387;    /* f_kwarg  */

    #[allow(non_upper_case_globals)]
    const S_kwrest_mark: i32 = 388; /* kwrest_mark  */

    #[allow(non_upper_case_globals)]
    const S_f_no_kwarg: i32 = 389; /* f_no_kwarg  */

    #[allow(non_upper_case_globals)]
    const S_f_kwrest: i32 = 390;   /* f_kwrest  */

    #[allow(non_upper_case_globals)]
    const S_f_opt: i32 = 391;      /* f_opt  */

    #[allow(non_upper_case_globals)]
    const S_f_block_opt: i32 = 392; /* f_block_opt  */

    #[allow(non_upper_case_globals)]
    const S_f_block_optarg: i32 = 393; /* f_block_optarg  */

    #[allow(non_upper_case_globals)]
    const S_f_optarg: i32 = 394;   /* f_optarg  */

    #[allow(non_upper_case_globals)]
    const S_restarg_mark: i32 = 395; /* restarg_mark  */

    #[allow(non_upper_case_globals)]
    const S_f_rest_arg: i32 = 396; /* f_rest_arg  */

    #[allow(non_upper_case_globals)]
    const S_blkarg_mark: i32 = 397; /* blkarg_mark  */

    #[allow(non_upper_case_globals)]
    const S_f_block_arg: i32 = 398; /* f_block_arg  */

    #[allow(non_upper_case_globals)]
    const S_opt_f_block_arg: i32 = 399; /* opt_f_block_arg  */

    #[allow(non_upper_case_globals)]
    const S_singleton: i32 = 400;  /* singleton  */

    #[allow(non_upper_case_globals)]
    const S_401_44: i32 = 401;     /* @44  */

    #[allow(non_upper_case_globals)]
    const S_assoc_list: i32 = 402; /* assoc_list  */

    #[allow(non_upper_case_globals)]
    const S_assocs: i32 = 403;     /* assocs  */

    #[allow(non_upper_case_globals)]
    const S_assoc: i32 = 404;      /* assoc  */

    #[allow(non_upper_case_globals)]
    const S_operation: i32 = 405;  /* operation  */

    #[allow(non_upper_case_globals)]
    const S_operation2: i32 = 406; /* operation2  */

    #[allow(non_upper_case_globals)]
    const S_operation3: i32 = 407; /* operation3  */

    #[allow(non_upper_case_globals)]
    const S_dot_or_colon: i32 = 408; /* dot_or_colon  */

    #[allow(non_upper_case_globals)]
    const S_call_op: i32 = 409;    /* call_op  */

    #[allow(non_upper_case_globals)]
    const S_call_op2: i32 = 410;   /* call_op2  */

    #[allow(non_upper_case_globals)]
    const S_opt_terms: i32 = 411;  /* opt_terms  */

    #[allow(non_upper_case_globals)]
    const S_opt_nl: i32 = 412;     /* opt_nl  */

    #[allow(non_upper_case_globals)]
    const S_rparen: i32 = 413;     /* rparen  */

    #[allow(non_upper_case_globals)]
    const S_rbracket: i32 = 414;   /* rbracket  */

    #[allow(non_upper_case_globals)]
    const S_rbrace: i32 = 415;     /* rbrace  */

    #[allow(non_upper_case_globals)]
    const S_trailer: i32 = 416;    /* trailer  */

    #[allow(non_upper_case_globals)]
    const S_term: i32 = 417;       /* term  */

    #[allow(non_upper_case_globals)]
    const S_terms: i32 = 418;      /* terms  */

    #[allow(non_upper_case_globals)]
    const S_none: i32 = 419;       /* none  */


    const VALUES_: &'static [SymbolKind] = &[ 
        SymbolKind { value: SymbolKind::S_YYEOF },
        SymbolKind { value: SymbolKind::S_YYerror },
        SymbolKind { value: SymbolKind::S_YYUNDEF },
        SymbolKind { value: SymbolKind::S_kCLASS },
        SymbolKind { value: SymbolKind::S_kMODULE },
        SymbolKind { value: SymbolKind::S_kDEF },
        SymbolKind { value: SymbolKind::S_kUNDEF },
        SymbolKind { value: SymbolKind::S_kBEGIN },
        SymbolKind { value: SymbolKind::S_kRESCUE },
        SymbolKind { value: SymbolKind::S_kENSURE },
        SymbolKind { value: SymbolKind::S_kEND },
        SymbolKind { value: SymbolKind::S_kIF },
        SymbolKind { value: SymbolKind::S_kUNLESS },
        SymbolKind { value: SymbolKind::S_kTHEN },
        SymbolKind { value: SymbolKind::S_kELSIF },
        SymbolKind { value: SymbolKind::S_kELSE },
        SymbolKind { value: SymbolKind::S_kCASE },
        SymbolKind { value: SymbolKind::S_kWHEN },
        SymbolKind { value: SymbolKind::S_kWHILE },
        SymbolKind { value: SymbolKind::S_kUNTIL },
        SymbolKind { value: SymbolKind::S_kFOR },
        SymbolKind { value: SymbolKind::S_kBREAK },
        SymbolKind { value: SymbolKind::S_kNEXT },
        SymbolKind { value: SymbolKind::S_kREDO },
        SymbolKind { value: SymbolKind::S_kRETRY },
        SymbolKind { value: SymbolKind::S_kIN },
        SymbolKind { value: SymbolKind::S_kDO },
        SymbolKind { value: SymbolKind::S_kDO_COND },
        SymbolKind { value: SymbolKind::S_kDO_BLOCK },
        SymbolKind { value: SymbolKind::S_kDO_LAMBDA },
        SymbolKind { value: SymbolKind::S_kRETURN },
        SymbolKind { value: SymbolKind::S_kYIELD },
        SymbolKind { value: SymbolKind::S_kSUPER },
        SymbolKind { value: SymbolKind::S_kSELF },
        SymbolKind { value: SymbolKind::S_kNIL },
        SymbolKind { value: SymbolKind::S_kTRUE },
        SymbolKind { value: SymbolKind::S_kFALSE },
        SymbolKind { value: SymbolKind::S_kAND },
        SymbolKind { value: SymbolKind::S_kOR },
        SymbolKind { value: SymbolKind::S_kNOT },
        SymbolKind { value: SymbolKind::S_kIF_MOD },
        SymbolKind { value: SymbolKind::S_kUNLESS_MOD },
        SymbolKind { value: SymbolKind::S_kWHILE_MOD },
        SymbolKind { value: SymbolKind::S_kUNTIL_MOD },
        SymbolKind { value: SymbolKind::S_kRESCUE_MOD },
        SymbolKind { value: SymbolKind::S_kALIAS },
        SymbolKind { value: SymbolKind::S_kDEFINED },
        SymbolKind { value: SymbolKind::S_klBEGIN },
        SymbolKind { value: SymbolKind::S_klEND },
        SymbolKind { value: SymbolKind::S_k__LINE__ },
        SymbolKind { value: SymbolKind::S_k__FILE__ },
        SymbolKind { value: SymbolKind::S_k__ENCODING__ },
        SymbolKind { value: SymbolKind::S_tIDENTIFIER },
        SymbolKind { value: SymbolKind::S_tFID },
        SymbolKind { value: SymbolKind::S_tGVAR },
        SymbolKind { value: SymbolKind::S_tIVAR },
        SymbolKind { value: SymbolKind::S_tCONSTANT },
        SymbolKind { value: SymbolKind::S_tCVAR },
        SymbolKind { value: SymbolKind::S_tLABEL },
        SymbolKind { value: SymbolKind::S_tINTEGER },
        SymbolKind { value: SymbolKind::S_tFLOAT },
        SymbolKind { value: SymbolKind::S_tRATIONAL },
        SymbolKind { value: SymbolKind::S_tIMAGINARY },
        SymbolKind { value: SymbolKind::S_tCHAR },
        SymbolKind { value: SymbolKind::S_tNTH_REF },
        SymbolKind { value: SymbolKind::S_tBACK_REF },
        SymbolKind { value: SymbolKind::S_tSTRING_CONTENT },
        SymbolKind { value: SymbolKind::S_tREGEXP_END },
        SymbolKind { value: SymbolKind::S_tDOT },
        SymbolKind { value: SymbolKind::S_tBACKSLASH },
        SymbolKind { value: SymbolKind::S_tSP },
        SymbolKind { value: SymbolKind::S_tSLASH_T },
        SymbolKind { value: SymbolKind::S_tSLASH_F },
        SymbolKind { value: SymbolKind::S_tSLASH_R },
        SymbolKind { value: SymbolKind::S_tVTAB },
        SymbolKind { value: SymbolKind::S_tUPLUS },
        SymbolKind { value: SymbolKind::S_tUMINUS },
        SymbolKind { value: SymbolKind::S_tPOW },
        SymbolKind { value: SymbolKind::S_tCMP },
        SymbolKind { value: SymbolKind::S_tEQ },
        SymbolKind { value: SymbolKind::S_tEQQ },
        SymbolKind { value: SymbolKind::S_tNEQ },
        SymbolKind { value: SymbolKind::S_tGEQ },
        SymbolKind { value: SymbolKind::S_tLEQ },
        SymbolKind { value: SymbolKind::S_tANDOP },
        SymbolKind { value: SymbolKind::S_tOROP },
        SymbolKind { value: SymbolKind::S_tMATCH },
        SymbolKind { value: SymbolKind::S_tNMATCH },
        SymbolKind { value: SymbolKind::S_tDOT2 },
        SymbolKind { value: SymbolKind::S_tDOT3 },
        SymbolKind { value: SymbolKind::S_tBDOT2 },
        SymbolKind { value: SymbolKind::S_tBDOT3 },
        SymbolKind { value: SymbolKind::S_tAREF },
        SymbolKind { value: SymbolKind::S_tASET },
        SymbolKind { value: SymbolKind::S_tLSHFT },
        SymbolKind { value: SymbolKind::S_tRSHFT },
        SymbolKind { value: SymbolKind::S_tANDDOT },
        SymbolKind { value: SymbolKind::S_tCOLON2 },
        SymbolKind { value: SymbolKind::S_tCOLON3 },
        SymbolKind { value: SymbolKind::S_tOP_ASGN },
        SymbolKind { value: SymbolKind::S_tASSOC },
        SymbolKind { value: SymbolKind::S_tLPAREN },
        SymbolKind { value: SymbolKind::S_tLPAREN_ARG },
        SymbolKind { value: SymbolKind::S_tRPAREN },
        SymbolKind { value: SymbolKind::S_tLBRACK },
        SymbolKind { value: SymbolKind::S_tLBRACE },
        SymbolKind { value: SymbolKind::S_tLBRACE_ARG },
        SymbolKind { value: SymbolKind::S_tSTAR },
        SymbolKind { value: SymbolKind::S_tDSTAR },
        SymbolKind { value: SymbolKind::S_tAMPER },
        SymbolKind { value: SymbolKind::S_tLAMBDA },
        SymbolKind { value: SymbolKind::S_tSYMBEG },
        SymbolKind { value: SymbolKind::S_tSTRING_BEG },
        SymbolKind { value: SymbolKind::S_tXSTRING_BEG },
        SymbolKind { value: SymbolKind::S_tREGEXP_BEG },
        SymbolKind { value: SymbolKind::S_tWORDS_BEG },
        SymbolKind { value: SymbolKind::S_tQWORDS_BEG },
        SymbolKind { value: SymbolKind::S_tSYMBOLS_BEG },
        SymbolKind { value: SymbolKind::S_tQSYMBOLS_BEG },
        SymbolKind { value: SymbolKind::S_tSTRING_END },
        SymbolKind { value: SymbolKind::S_tSTRING_DEND },
        SymbolKind { value: SymbolKind::S_tSTRING_DBEG },
        SymbolKind { value: SymbolKind::S_tSTRING_DVAR },
        SymbolKind { value: SymbolKind::S_tLAMBEG },
        SymbolKind { value: SymbolKind::S_tLABEL_END },
        SymbolKind { value: SymbolKind::S_tCOMMA },
        SymbolKind { value: SymbolKind::S_tLCURLY },
        SymbolKind { value: SymbolKind::S_tRCURLY },
        SymbolKind { value: SymbolKind::S_tLBRACK2 },
        SymbolKind { value: SymbolKind::S_tEQL },
        SymbolKind { value: SymbolKind::S_tPIPE },
        SymbolKind { value: SymbolKind::S_tAMPER2 },
        SymbolKind { value: SymbolKind::S_tGT },
        SymbolKind { value: SymbolKind::S_tLT },
        SymbolKind { value: SymbolKind::S_tBACK_REF2 },
        SymbolKind { value: SymbolKind::S_tCARET },
        SymbolKind { value: SymbolKind::S_tLPAREN2 },
        SymbolKind { value: SymbolKind::S_tRBRACK },
        SymbolKind { value: SymbolKind::S_tSEMI },
        SymbolKind { value: SymbolKind::S_tSPACE },
        SymbolKind { value: SymbolKind::S_tNL },
        SymbolKind { value: SymbolKind::S_tPLUS },
        SymbolKind { value: SymbolKind::S_tMINUS },
        SymbolKind { value: SymbolKind::S_tSTAR2 },
        SymbolKind { value: SymbolKind::S_tDIVIDE },
        SymbolKind { value: SymbolKind::S_tPERCENT },
        SymbolKind { value: SymbolKind::S_tTILDE },
        SymbolKind { value: SymbolKind::S_tBANG },
        SymbolKind { value: SymbolKind::S_tLOWEST },
        SymbolKind { value: SymbolKind::S_tEH },
        SymbolKind { value: SymbolKind::S_tCOLON },
        SymbolKind { value: SymbolKind::S_tUMINUS_NUM },
        SymbolKind { value: SymbolKind::S_tLAST_TOKEN },
        SymbolKind { value: SymbolKind::S_YYACCEPT },
        SymbolKind { value: SymbolKind::S_program },
        SymbolKind { value: SymbolKind::S_155_1 },
        SymbolKind { value: SymbolKind::S_top_compstmt },
        SymbolKind { value: SymbolKind::S_top_stmts },
        SymbolKind { value: SymbolKind::S_top_stmt },
        SymbolKind { value: SymbolKind::S_begin_block },
        SymbolKind { value: SymbolKind::S_bodystmt },
        SymbolKind { value: SymbolKind::S_compstmt },
        SymbolKind { value: SymbolKind::S_stmts },
        SymbolKind { value: SymbolKind::S_stmt_or_begin },
        SymbolKind { value: SymbolKind::S_164_2 },
        SymbolKind { value: SymbolKind::S_stmt },
        SymbolKind { value: SymbolKind::S_166_3 },
        SymbolKind { value: SymbolKind::S_command_asgn },
        SymbolKind { value: SymbolKind::S_command_rhs },
        SymbolKind { value: SymbolKind::S_expr },
        SymbolKind { value: SymbolKind::S_170_4 },
        SymbolKind { value: SymbolKind::S_171_5 },
        SymbolKind { value: SymbolKind::S_def_name },
        SymbolKind { value: SymbolKind::S_defn_head },
        SymbolKind { value: SymbolKind::S_defs_head },
        SymbolKind { value: SymbolKind::S_175_6 },
        SymbolKind { value: SymbolKind::S_expr_value },
        SymbolKind { value: SymbolKind::S_expr_value_do },
        SymbolKind { value: SymbolKind::S_178_7 },
        SymbolKind { value: SymbolKind::S_command_call },
        SymbolKind { value: SymbolKind::S_block_command },
        SymbolKind { value: SymbolKind::S_cmd_brace_block },
        SymbolKind { value: SymbolKind::S_182_8 },
        SymbolKind { value: SymbolKind::S_fcall },
        SymbolKind { value: SymbolKind::S_command },
        SymbolKind { value: SymbolKind::S_mlhs },
        SymbolKind { value: SymbolKind::S_mlhs_inner },
        SymbolKind { value: SymbolKind::S_mlhs_basic },
        SymbolKind { value: SymbolKind::S_mlhs_item },
        SymbolKind { value: SymbolKind::S_mlhs_head },
        SymbolKind { value: SymbolKind::S_mlhs_post },
        SymbolKind { value: SymbolKind::S_mlhs_node },
        SymbolKind { value: SymbolKind::S_lhs },
        SymbolKind { value: SymbolKind::S_cname },
        SymbolKind { value: SymbolKind::S_cpath },
        SymbolKind { value: SymbolKind::S_fname },
        SymbolKind { value: SymbolKind::S_fitem },
        SymbolKind { value: SymbolKind::S_undef_list },
        SymbolKind { value: SymbolKind::S_198_9 },
        SymbolKind { value: SymbolKind::S_op },
        SymbolKind { value: SymbolKind::S_reswords },
        SymbolKind { value: SymbolKind::S_arg },
        SymbolKind { value: SymbolKind::S_202_10 },
        SymbolKind { value: SymbolKind::S_relop },
        SymbolKind { value: SymbolKind::S_rel_expr },
        SymbolKind { value: SymbolKind::S_arg_value },
        SymbolKind { value: SymbolKind::S_aref_args },
        SymbolKind { value: SymbolKind::S_arg_rhs },
        SymbolKind { value: SymbolKind::S_paren_args },
        SymbolKind { value: SymbolKind::S_opt_paren_args },
        SymbolKind { value: SymbolKind::S_opt_call_args },
        SymbolKind { value: SymbolKind::S_call_args },
        SymbolKind { value: SymbolKind::S_command_args },
        SymbolKind { value: SymbolKind::S_213_11 },
        SymbolKind { value: SymbolKind::S_block_arg },
        SymbolKind { value: SymbolKind::S_opt_block_arg },
        SymbolKind { value: SymbolKind::S_args },
        SymbolKind { value: SymbolKind::S_mrhs_arg },
        SymbolKind { value: SymbolKind::S_mrhs },
        SymbolKind { value: SymbolKind::S_primary },
        SymbolKind { value: SymbolKind::S_220_12 },
        SymbolKind { value: SymbolKind::S_221_13 },
        SymbolKind { value: SymbolKind::S_222_14 },
        SymbolKind { value: SymbolKind::S_223_15 },
        SymbolKind { value: SymbolKind::S_224_16 },
        SymbolKind { value: SymbolKind::S_225_17 },
        SymbolKind { value: SymbolKind::S_226_18 },
        SymbolKind { value: SymbolKind::S_227_19 },
        SymbolKind { value: SymbolKind::S_228_20 },
        SymbolKind { value: SymbolKind::S_primary_value },
        SymbolKind { value: SymbolKind::S_k_begin },
        SymbolKind { value: SymbolKind::S_k_if },
        SymbolKind { value: SymbolKind::S_k_unless },
        SymbolKind { value: SymbolKind::S_k_while },
        SymbolKind { value: SymbolKind::S_k_until },
        SymbolKind { value: SymbolKind::S_k_case },
        SymbolKind { value: SymbolKind::S_k_for },
        SymbolKind { value: SymbolKind::S_k_class },
        SymbolKind { value: SymbolKind::S_k_module },
        SymbolKind { value: SymbolKind::S_k_def },
        SymbolKind { value: SymbolKind::S_k_do },
        SymbolKind { value: SymbolKind::S_k_do_block },
        SymbolKind { value: SymbolKind::S_k_rescue },
        SymbolKind { value: SymbolKind::S_k_ensure },
        SymbolKind { value: SymbolKind::S_k_when },
        SymbolKind { value: SymbolKind::S_k_else },
        SymbolKind { value: SymbolKind::S_k_elsif },
        SymbolKind { value: SymbolKind::S_k_end },
        SymbolKind { value: SymbolKind::S_k_return },
        SymbolKind { value: SymbolKind::S_then },
        SymbolKind { value: SymbolKind::S_do },
        SymbolKind { value: SymbolKind::S_if_tail },
        SymbolKind { value: SymbolKind::S_opt_else },
        SymbolKind { value: SymbolKind::S_for_var },
        SymbolKind { value: SymbolKind::S_f_marg },
        SymbolKind { value: SymbolKind::S_f_marg_list },
        SymbolKind { value: SymbolKind::S_f_margs },
        SymbolKind { value: SymbolKind::S_f_rest_marg },
        SymbolKind { value: SymbolKind::S_f_any_kwrest },
        SymbolKind { value: SymbolKind::S_f_eq },
        SymbolKind { value: SymbolKind::S_260_21 },
        SymbolKind { value: SymbolKind::S_block_args_tail },
        SymbolKind { value: SymbolKind::S_opt_block_args_tail },
        SymbolKind { value: SymbolKind::S_excessed_comma },
        SymbolKind { value: SymbolKind::S_block_param },
        SymbolKind { value: SymbolKind::S_opt_block_param },
        SymbolKind { value: SymbolKind::S_block_param_def },
        SymbolKind { value: SymbolKind::S_opt_bv_decl },
        SymbolKind { value: SymbolKind::S_bv_decls },
        SymbolKind { value: SymbolKind::S_bvar },
        SymbolKind { value: SymbolKind::S_lambda },
        SymbolKind { value: SymbolKind::S_271_22 },
        SymbolKind { value: SymbolKind::S_272_23 },
        SymbolKind { value: SymbolKind::S_273_24 },
        SymbolKind { value: SymbolKind::S_f_larglist },
        SymbolKind { value: SymbolKind::S_lambda_body },
        SymbolKind { value: SymbolKind::S_276_25 },
        SymbolKind { value: SymbolKind::S_277_26 },
        SymbolKind { value: SymbolKind::S_do_block },
        SymbolKind { value: SymbolKind::S_279_27 },
        SymbolKind { value: SymbolKind::S_block_call },
        SymbolKind { value: SymbolKind::S_method_call },
        SymbolKind { value: SymbolKind::S_brace_block },
        SymbolKind { value: SymbolKind::S_283_28 },
        SymbolKind { value: SymbolKind::S_284_29 },
        SymbolKind { value: SymbolKind::S_brace_body },
        SymbolKind { value: SymbolKind::S_286_30 },
        SymbolKind { value: SymbolKind::S_do_body },
        SymbolKind { value: SymbolKind::S_288_31 },
        SymbolKind { value: SymbolKind::S_case_args },
        SymbolKind { value: SymbolKind::S_case_body },
        SymbolKind { value: SymbolKind::S_cases },
        SymbolKind { value: SymbolKind::S_p_case_body },
        SymbolKind { value: SymbolKind::S_293_32 },
        SymbolKind { value: SymbolKind::S_294_33 },
        SymbolKind { value: SymbolKind::S_p_cases },
        SymbolKind { value: SymbolKind::S_p_top_expr },
        SymbolKind { value: SymbolKind::S_p_top_expr_body },
        SymbolKind { value: SymbolKind::S_p_expr },
        SymbolKind { value: SymbolKind::S_p_as },
        SymbolKind { value: SymbolKind::S_p_alt },
        SymbolKind { value: SymbolKind::S_p_lparen },
        SymbolKind { value: SymbolKind::S_p_lbracket },
        SymbolKind { value: SymbolKind::S_p_expr_basic },
        SymbolKind { value: SymbolKind::S_304_34 },
        SymbolKind { value: SymbolKind::S_305_35 },
        SymbolKind { value: SymbolKind::S_p_args },
        SymbolKind { value: SymbolKind::S_p_args_head },
        SymbolKind { value: SymbolKind::S_p_args_tail },
        SymbolKind { value: SymbolKind::S_p_find },
        SymbolKind { value: SymbolKind::S_p_rest },
        SymbolKind { value: SymbolKind::S_p_args_post },
        SymbolKind { value: SymbolKind::S_p_arg },
        SymbolKind { value: SymbolKind::S_p_kwargs },
        SymbolKind { value: SymbolKind::S_p_kwarg },
        SymbolKind { value: SymbolKind::S_p_kw },
        SymbolKind { value: SymbolKind::S_p_kw_label },
        SymbolKind { value: SymbolKind::S_p_kwrest },
        SymbolKind { value: SymbolKind::S_p_kwnorest },
        SymbolKind { value: SymbolKind::S_p_any_kwrest },
        SymbolKind { value: SymbolKind::S_p_value },
        SymbolKind { value: SymbolKind::S_p_primitive },
        SymbolKind { value: SymbolKind::S_p_variable },
        SymbolKind { value: SymbolKind::S_p_var_ref },
        SymbolKind { value: SymbolKind::S_p_expr_ref },
        SymbolKind { value: SymbolKind::S_p_const },
        SymbolKind { value: SymbolKind::S_opt_rescue },
        SymbolKind { value: SymbolKind::S_exc_list },
        SymbolKind { value: SymbolKind::S_exc_var },
        SymbolKind { value: SymbolKind::S_opt_ensure },
        SymbolKind { value: SymbolKind::S_literal },
        SymbolKind { value: SymbolKind::S_strings },
        SymbolKind { value: SymbolKind::S_string },
        SymbolKind { value: SymbolKind::S_string1 },
        SymbolKind { value: SymbolKind::S_xstring },
        SymbolKind { value: SymbolKind::S_regexp },
        SymbolKind { value: SymbolKind::S_words },
        SymbolKind { value: SymbolKind::S_word_list },
        SymbolKind { value: SymbolKind::S_word },
        SymbolKind { value: SymbolKind::S_symbols },
        SymbolKind { value: SymbolKind::S_symbol_list },
        SymbolKind { value: SymbolKind::S_qwords },
        SymbolKind { value: SymbolKind::S_qsymbols },
        SymbolKind { value: SymbolKind::S_qword_list },
        SymbolKind { value: SymbolKind::S_qsym_list },
        SymbolKind { value: SymbolKind::S_string_contents },
        SymbolKind { value: SymbolKind::S_xstring_contents },
        SymbolKind { value: SymbolKind::S_regexp_contents },
        SymbolKind { value: SymbolKind::S_string_content },
        SymbolKind { value: SymbolKind::S_349_36 },
        SymbolKind { value: SymbolKind::S_350_37 },
        SymbolKind { value: SymbolKind::S_351_38 },
        SymbolKind { value: SymbolKind::S_352_39 },
        SymbolKind { value: SymbolKind::S_353_40 },
        SymbolKind { value: SymbolKind::S_354_41 },
        SymbolKind { value: SymbolKind::S_string_dvar },
        SymbolKind { value: SymbolKind::S_symbol },
        SymbolKind { value: SymbolKind::S_ssym },
        SymbolKind { value: SymbolKind::S_sym },
        SymbolKind { value: SymbolKind::S_dsym },
        SymbolKind { value: SymbolKind::S_numeric },
        SymbolKind { value: SymbolKind::S_simple_numeric },
        SymbolKind { value: SymbolKind::S_nonlocal_var },
        SymbolKind { value: SymbolKind::S_user_variable },
        SymbolKind { value: SymbolKind::S_keyword_variable },
        SymbolKind { value: SymbolKind::S_var_ref },
        SymbolKind { value: SymbolKind::S_var_lhs },
        SymbolKind { value: SymbolKind::S_backref },
        SymbolKind { value: SymbolKind::S_superclass },
        SymbolKind { value: SymbolKind::S_369_42 },
        SymbolKind { value: SymbolKind::S_f_opt_paren_args },
        SymbolKind { value: SymbolKind::S_f_paren_args },
        SymbolKind { value: SymbolKind::S_f_arglist },
        SymbolKind { value: SymbolKind::S_373_43 },
        SymbolKind { value: SymbolKind::S_args_tail },
        SymbolKind { value: SymbolKind::S_opt_args_tail },
        SymbolKind { value: SymbolKind::S_f_args },
        SymbolKind { value: SymbolKind::S_args_forward },
        SymbolKind { value: SymbolKind::S_f_bad_arg },
        SymbolKind { value: SymbolKind::S_f_norm_arg },
        SymbolKind { value: SymbolKind::S_f_arg_asgn },
        SymbolKind { value: SymbolKind::S_f_arg_item },
        SymbolKind { value: SymbolKind::S_f_arg },
        SymbolKind { value: SymbolKind::S_f_label },
        SymbolKind { value: SymbolKind::S_f_kw },
        SymbolKind { value: SymbolKind::S_f_block_kw },
        SymbolKind { value: SymbolKind::S_f_block_kwarg },
        SymbolKind { value: SymbolKind::S_f_kwarg },
        SymbolKind { value: SymbolKind::S_kwrest_mark },
        SymbolKind { value: SymbolKind::S_f_no_kwarg },
        SymbolKind { value: SymbolKind::S_f_kwrest },
        SymbolKind { value: SymbolKind::S_f_opt },
        SymbolKind { value: SymbolKind::S_f_block_opt },
        SymbolKind { value: SymbolKind::S_f_block_optarg },
        SymbolKind { value: SymbolKind::S_f_optarg },
        SymbolKind { value: SymbolKind::S_restarg_mark },
        SymbolKind { value: SymbolKind::S_f_rest_arg },
        SymbolKind { value: SymbolKind::S_blkarg_mark },
        SymbolKind { value: SymbolKind::S_f_block_arg },
        SymbolKind { value: SymbolKind::S_opt_f_block_arg },
        SymbolKind { value: SymbolKind::S_singleton },
        SymbolKind { value: SymbolKind::S_401_44 },
        SymbolKind { value: SymbolKind::S_assoc_list },
        SymbolKind { value: SymbolKind::S_assocs },
        SymbolKind { value: SymbolKind::S_assoc },
        SymbolKind { value: SymbolKind::S_operation },
        SymbolKind { value: SymbolKind::S_operation2 },
        SymbolKind { value: SymbolKind::S_operation3 },
        SymbolKind { value: SymbolKind::S_dot_or_colon },
        SymbolKind { value: SymbolKind::S_call_op },
        SymbolKind { value: SymbolKind::S_call_op2 },
        SymbolKind { value: SymbolKind::S_opt_terms },
        SymbolKind { value: SymbolKind::S_opt_nl },
        SymbolKind { value: SymbolKind::S_rparen },
        SymbolKind { value: SymbolKind::S_rbracket },
        SymbolKind { value: SymbolKind::S_rbrace },
        SymbolKind { value: SymbolKind::S_trailer },
        SymbolKind { value: SymbolKind::S_term },
        SymbolKind { value: SymbolKind::S_terms },
        SymbolKind { value: SymbolKind::S_none }
    ];

    pub(crate) fn get(n: i32) -> &'static SymbolKind {
        &Self::VALUES_[i32_to_usize(n)]
    }

    pub(crate) fn code(&self) -> i32 {
        self.value
    }

    /* YYNAMES_[SYMBOL-NUM] -- String name of the symbol SYMBOL-NUM.
    First, the terminals, then, starting at \a YYNTOKENS_, nonterminals.  */
    #[allow(non_upper_case_globals)]
const yynames_: &'static [&'static str] = &[ "end-of-input", "error", "invalid token", "`class'", "`module'",
  "`def'", "`undef'", "`begin'", "`rescue'", "`ensure'", "`end'", "`if'",
  "`unless'", "`then'", "`elsif'", "`else'", "`case'", "`when'", "`while'",
  "`until'", "`for'", "`break'", "`next'", "`redo'", "`retry'", "`in'",
  "`do'", "`do' for condition", "`do' for block", "`do' for lambda",
  "`return'", "`yield'", "`super'", "`self'", "`nil'", "`true'", "`false'",
  "`and'", "`or'", "`not'", "`if' modifier", "`unless' modifier",
  "`while' modifier", "`until' modifier", "`rescue' modifier", "`alias'",
  "`defined?'", "`BEGIN'", "`END'", "`__LINE__'", "`__FILE__'",
  "`__ENCODING__'", "local variable or method", "method",
  "global variable", "instance variable", "constant", "class variable",
  "label", "integer literal", "float literal", "rational literal",
  "imaginary literal", "char literal", "numbered reference",
  "back reference", "literal content", "tREGEXP_END", "tDOT", "backslash",
  "escaped space", "escaped horizontal tab", "escaped form feed",
  "escaped carriage return", "escaped vertical tab", "unary+", "unary-",
  "**", "<=>", "==", "===", "!=", ">=", "<=", "&&", "||", "=~", "!~", "..",
  "...", "(..", "(...", "[]", "[]=", "<<", ">>", "&.", "::",
  ":: at EXPR_BEG", "operator-assignment", "=>", "(", "( arg", ")", "[",
  "{", "{ arg", "*", "**arg", "&", "->", "symbol literal", "string begin",
  "backtick literal", "regexp literal", "word list", "verbatim word list",
  "symbol list", "verbatim symbol list", "string end", "tRCURLY",
  "tSTRING_DBEG", "tSTRING_DVAR", "tLAMBEG", "tLABEL_END", ",",
  "{ (tLCURLY)", "}", "[ (tLBRACK2)", "=", "|", "& (tAMPER2)", ">", "<",
  "`", "^", "( (tLPAREN2)", "]", ";", " ", "\n", "+", "-", "* (tSTAR2)",
  "/", "%", "~", "!", "tLOWEST", "tEH", "tCOLON", "tUMINUS_NUM",
  "tLAST_TOKEN", "$accept", "program", "@1", "top_compstmt", "top_stmts",
  "top_stmt", "begin_block", "bodystmt", "compstmt", "stmts",
  "stmt_or_begin", "$@2", "stmt", "@3", "command_asgn", "command_rhs",
  "expr", "@4", "@5", "def_name", "defn_head", "defs_head", "@6",
  "expr_value", "expr_value_do", "@7", "command_call", "block_command",
  "cmd_brace_block", "@8", "fcall", "command", "mlhs", "mlhs_inner",
  "mlhs_basic", "mlhs_item", "mlhs_head", "mlhs_post", "mlhs_node", "lhs",
  "cname", "cpath", "fname", "fitem", "undef_list", "@9", "op", "reswords",
  "arg", "@10", "relop", "rel_expr", "arg_value", "aref_args", "arg_rhs",
  "paren_args", "opt_paren_args", "opt_call_args", "call_args",
  "command_args", "@11", "block_arg", "opt_block_arg", "args", "mrhs_arg",
  "mrhs", "primary", "@12", "@13", "@14", "@15", "@16", "@17", "@18",
  "@19", "@20", "primary_value", "k_begin", "k_if", "k_unless", "k_while",
  "k_until", "k_case", "k_for", "k_class", "k_module", "k_def", "k_do",
  "k_do_block", "k_rescue", "k_ensure", "k_when", "k_else", "k_elsif",
  "k_end", "k_return", "then", "do", "if_tail", "opt_else", "for_var",
  "f_marg", "f_marg_list", "f_margs", "f_rest_marg", "f_any_kwrest",
  "f_eq", "@21", "block_args_tail", "opt_block_args_tail",
  "excessed_comma", "block_param", "opt_block_param", "block_param_def",
  "opt_bv_decl", "bv_decls", "bvar", "lambda", "@22", "@23", "@24",
  "f_larglist", "lambda_body", "@25", "@26", "do_block", "@27",
  "block_call", "method_call", "brace_block", "@28", "@29", "brace_body",
  "@30", "do_body", "@31", "case_args", "case_body", "cases",
  "p_case_body", "@32", "@33", "p_cases", "p_top_expr", "p_top_expr_body",
  "p_expr", "p_as", "p_alt", "p_lparen", "p_lbracket", "p_expr_basic",
  "@34", "@35", "p_args", "p_args_head", "p_args_tail", "p_find", "p_rest",
  "p_args_post", "p_arg", "p_kwargs", "p_kwarg", "p_kw", "p_kw_label",
  "p_kwrest", "p_kwnorest", "p_any_kwrest", "p_value", "p_primitive",
  "p_variable", "p_var_ref", "p_expr_ref", "p_const", "opt_rescue",
  "exc_list", "exc_var", "opt_ensure", "literal", "strings", "string",
  "string1", "xstring", "regexp", "words", "word_list", "word", "symbols",
  "symbol_list", "qwords", "qsymbols", "qword_list", "qsym_list",
  "string_contents", "xstring_contents", "regexp_contents",
  "string_content", "@36", "@37", "@38", "@39", "@40", "@41",
  "string_dvar", "symbol", "ssym", "sym", "dsym", "numeric",
  "simple_numeric", "nonlocal_var", "user_variable", "keyword_variable",
  "var_ref", "var_lhs", "backref", "superclass", "@42", "f_opt_paren_args",
  "f_paren_args", "f_arglist", "@43", "args_tail", "opt_args_tail",
  "f_args", "args_forward", "f_bad_arg", "f_norm_arg", "f_arg_asgn",
  "f_arg_item", "f_arg", "f_label", "f_kw", "f_block_kw", "f_block_kwarg",
  "f_kwarg", "kwrest_mark", "f_no_kwarg", "f_kwrest", "f_opt",
  "f_block_opt", "f_block_optarg", "f_optarg", "restarg_mark",
  "f_rest_arg", "blkarg_mark", "f_block_arg", "opt_f_block_arg",
  "singleton", "@44", "assoc_list", "assocs", "assoc", "operation",
  "operation2", "operation3", "dot_or_colon", "call_op", "call_op2",
  "opt_terms", "opt_nl", "rparen", "rbracket", "rbrace", "trailer", "term",
  "terms", "none", "<<NULL>>" ] ;

    /* The user-facing name of this symbol.  */
    pub(crate) fn name(&self) -> String {
        let code: usize = self.code().try_into().unwrap();
        Self::yynames_[code].to_owned()
    }
}


const DYMMY_SYMBOL_KIND: SymbolKind = SymbolKind { value: 0 };

impl Lexer {
        /* Token kinds.  */
    /// Token `` "end-of-input" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const END_OF_INPUT: i32 = 0;
    /// Token `` error ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const YYerror: i32 = 256;
    /// Token `` "invalid token" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const YYUNDEF: i32 = 257;
    /// Token `` "`class'" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kCLASS: i32 = 258;
    /// Token `` "`module'" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kMODULE: i32 = 259;
    /// Token `` "`def'" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kDEF: i32 = 260;
    /// Token `` "`undef'" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kUNDEF: i32 = 261;
    /// Token `` "`begin'" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kBEGIN: i32 = 262;
    /// Token `` "`rescue'" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kRESCUE: i32 = 263;
    /// Token `` "`ensure'" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kENSURE: i32 = 264;
    /// Token `` "`end'" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kEND: i32 = 265;
    /// Token `` "`if'" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kIF: i32 = 266;
    /// Token `` "`unless'" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kUNLESS: i32 = 267;
    /// Token `` "`then'" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kTHEN: i32 = 268;
    /// Token `` "`elsif'" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kELSIF: i32 = 269;
    /// Token `` "`else'" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kELSE: i32 = 270;
    /// Token `` "`case'" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kCASE: i32 = 271;
    /// Token `` "`when'" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kWHEN: i32 = 272;
    /// Token `` "`while'" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kWHILE: i32 = 273;
    /// Token `` "`until'" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kUNTIL: i32 = 274;
    /// Token `` "`for'" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kFOR: i32 = 275;
    /// Token `` "`break'" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kBREAK: i32 = 276;
    /// Token `` "`next'" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kNEXT: i32 = 277;
    /// Token `` "`redo'" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kREDO: i32 = 278;
    /// Token `` "`retry'" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kRETRY: i32 = 279;
    /// Token `` "`in'" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kIN: i32 = 280;
    /// Token `` "`do'" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kDO: i32 = 281;
    /// Token `` "`do' for condition" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kDO_COND: i32 = 282;
    /// Token `` "`do' for block" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kDO_BLOCK: i32 = 283;
    /// Token `` "`do' for lambda" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kDO_LAMBDA: i32 = 284;
    /// Token `` "`return'" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kRETURN: i32 = 285;
    /// Token `` "`yield'" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kYIELD: i32 = 286;
    /// Token `` "`super'" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kSUPER: i32 = 287;
    /// Token `` "`self'" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kSELF: i32 = 288;
    /// Token `` "`nil'" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kNIL: i32 = 289;
    /// Token `` "`true'" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kTRUE: i32 = 290;
    /// Token `` "`false'" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kFALSE: i32 = 291;
    /// Token `` "`and'" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kAND: i32 = 292;
    /// Token `` "`or'" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kOR: i32 = 293;
    /// Token `` "`not'" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kNOT: i32 = 294;
    /// Token `` "`if' modifier" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kIF_MOD: i32 = 295;
    /// Token `` "`unless' modifier" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kUNLESS_MOD: i32 = 296;
    /// Token `` "`while' modifier" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kWHILE_MOD: i32 = 297;
    /// Token `` "`until' modifier" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kUNTIL_MOD: i32 = 298;
    /// Token `` "`rescue' modifier" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kRESCUE_MOD: i32 = 299;
    /// Token `` "`alias'" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kALIAS: i32 = 300;
    /// Token `` "`defined?'" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kDEFINED: i32 = 301;
    /// Token `` "`BEGIN'" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const klBEGIN: i32 = 302;
    /// Token `` "`END'" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const klEND: i32 = 303;
    /// Token `` "`__LINE__'" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const k__LINE__: i32 = 304;
    /// Token `` "`__FILE__'" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const k__FILE__: i32 = 305;
    /// Token `` "`__ENCODING__'" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const k__ENCODING__: i32 = 306;
    /// Token `` "local variable or method" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tIDENTIFIER: i32 = 307;
    /// Token `` "method" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tFID: i32 = 308;
    /// Token `` "global variable" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tGVAR: i32 = 309;
    /// Token `` "instance variable" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tIVAR: i32 = 310;
    /// Token `` "constant" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tCONSTANT: i32 = 311;
    /// Token `` "class variable" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tCVAR: i32 = 312;
    /// Token `` "label" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tLABEL: i32 = 313;
    /// Token `` "integer literal" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tINTEGER: i32 = 314;
    /// Token `` "float literal" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tFLOAT: i32 = 315;
    /// Token `` "rational literal" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tRATIONAL: i32 = 316;
    /// Token `` "imaginary literal" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tIMAGINARY: i32 = 317;
    /// Token `` "char literal" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tCHAR: i32 = 318;
    /// Token `` "numbered reference" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tNTH_REF: i32 = 319;
    /// Token `` "back reference" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tBACK_REF: i32 = 320;
    /// Token `` "literal content" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tSTRING_CONTENT: i32 = 321;
    /// Token `` tREGEXP_END ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tREGEXP_END: i32 = 322;
    /// Token `` tDOT ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tDOT: i32 = 323;
    /// Token `` "backslash" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tBACKSLASH: i32 = 324;
    /// Token `` "escaped space" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tSP: i32 = 325;
    /// Token `` "escaped horizontal tab" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tSLASH_T: i32 = 326;
    /// Token `` "escaped form feed" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tSLASH_F: i32 = 327;
    /// Token `` "escaped carriage return" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tSLASH_R: i32 = 328;
    /// Token `` "escaped vertical tab" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tVTAB: i32 = 329;
    /// Token `` "unary+" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tUPLUS: i32 = 330;
    /// Token `` "unary-" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tUMINUS: i32 = 331;
    /// Token `` "**" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tPOW: i32 = 332;
    /// Token `` "<=>" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tCMP: i32 = 333;
    /// Token `` "==" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tEQ: i32 = 334;
    /// Token `` "===" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tEQQ: i32 = 335;
    /// Token `` "!=" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tNEQ: i32 = 336;
    /// Token `` ">=" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tGEQ: i32 = 337;
    /// Token `` "<=" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tLEQ: i32 = 338;
    /// Token `` "&&" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tANDOP: i32 = 339;
    /// Token `` "||" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tOROP: i32 = 340;
    /// Token `` "=~" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tMATCH: i32 = 341;
    /// Token `` "!~" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tNMATCH: i32 = 342;
    /// Token `` ".." ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tDOT2: i32 = 343;
    /// Token `` "..." ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tDOT3: i32 = 344;
    /// Token `` "(.." ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tBDOT2: i32 = 345;
    /// Token `` "(..." ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tBDOT3: i32 = 346;
    /// Token `` "[]" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tAREF: i32 = 347;
    /// Token `` "[]=" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tASET: i32 = 348;
    /// Token `` "<<" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tLSHFT: i32 = 349;
    /// Token `` ">>" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tRSHFT: i32 = 350;
    /// Token `` "&." ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tANDDOT: i32 = 351;
    /// Token `` "::" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tCOLON2: i32 = 352;
    /// Token `` ":: at EXPR_BEG" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tCOLON3: i32 = 353;
    /// Token `` "operator-assignment" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tOP_ASGN: i32 = 354;
    /// Token `` "=>" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tASSOC: i32 = 355;
    /// Token `` "(" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tLPAREN: i32 = 356;
    /// Token `` "( arg" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tLPAREN_ARG: i32 = 357;
    /// Token `` ")" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tRPAREN: i32 = 358;
    /// Token `` "[" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tLBRACK: i32 = 359;
    /// Token `` "{" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tLBRACE: i32 = 360;
    /// Token `` "{ arg" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tLBRACE_ARG: i32 = 361;
    /// Token `` "*" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tSTAR: i32 = 362;
    /// Token `` "**arg" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tDSTAR: i32 = 363;
    /// Token `` "&" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tAMPER: i32 = 364;
    /// Token `` "->" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tLAMBDA: i32 = 365;
    /// Token `` "symbol literal" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tSYMBEG: i32 = 366;
    /// Token `` "string begin" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tSTRING_BEG: i32 = 367;
    /// Token `` "backtick literal" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tXSTRING_BEG: i32 = 368;
    /// Token `` "regexp literal" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tREGEXP_BEG: i32 = 369;
    /// Token `` "word list" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tWORDS_BEG: i32 = 370;
    /// Token `` "verbatim word list" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tQWORDS_BEG: i32 = 371;
    /// Token `` "symbol list" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tSYMBOLS_BEG: i32 = 372;
    /// Token `` "verbatim symbol list" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tQSYMBOLS_BEG: i32 = 373;
    /// Token `` "string end" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tSTRING_END: i32 = 374;
    /// Token `` "tRCURLY" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tSTRING_DEND: i32 = 375;
    /// Token `` tSTRING_DBEG ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tSTRING_DBEG: i32 = 376;
    /// Token `` tSTRING_DVAR ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tSTRING_DVAR: i32 = 377;
    /// Token `` tLAMBEG ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tLAMBEG: i32 = 378;
    /// Token `` tLABEL_END ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tLABEL_END: i32 = 379;
    /// Token `` "," ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tCOMMA: i32 = 380;
    /// Token `` "{ (tLCURLY)" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tLCURLY: i32 = 381;
    /// Token `` "}" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tRCURLY: i32 = 382;
    /// Token `` "[ (tLBRACK2)" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tLBRACK2: i32 = 383;
    /// Token `` "=" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tEQL: i32 = 384;
    /// Token `` "|" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tPIPE: i32 = 385;
    /// Token `` "& (tAMPER2)" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tAMPER2: i32 = 386;
    /// Token `` ">" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tGT: i32 = 387;
    /// Token `` "<" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tLT: i32 = 388;
    /// Token `` "`" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tBACK_REF2: i32 = 389;
    /// Token `` "^" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tCARET: i32 = 390;
    /// Token `` "( (tLPAREN2)" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tLPAREN2: i32 = 391;
    /// Token `` "]" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tRBRACK: i32 = 392;
    /// Token `` ";" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tSEMI: i32 = 393;
    /// Token `` " " ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tSPACE: i32 = 394;
    /// Token `` "\n" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tNL: i32 = 395;
    /// Token `` "+" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tPLUS: i32 = 396;
    /// Token `` "-" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tMINUS: i32 = 397;
    /// Token `` "* (tSTAR2)" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tSTAR2: i32 = 398;
    /// Token `` "/" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tDIVIDE: i32 = 399;
    /// Token `` "%" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tPERCENT: i32 = 400;
    /// Token `` "~" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tTILDE: i32 = 401;
    /// Token `` "!" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tBANG: i32 = 402;
    /// Token `` tLOWEST ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tLOWEST: i32 = 403;
    /// Token `` tEH ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tEH: i32 = 404;
    /// Token `` tCOLON ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tCOLON: i32 = 405;
    /// Token `` tUMINUS_NUM ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tUMINUS_NUM: i32 = 406;
    /// Token `` tLAST_TOKEN ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tLAST_TOKEN: i32 = 407;


    // Deprecated, use END_OF_INPUT instead.
    #[allow(dead_code)]
    const EOF: i32 = Self::END_OF_INPUT;

    // Token values
    #[allow(dead_code)]
    pub(crate) const TOKEN_NAMES: &'static [&'static str] = &    [

    "END_OF_INPUT",

    "YYerror",

    "YYUNDEF",

    "kCLASS",

    "kMODULE",

    "kDEF",

    "kUNDEF",

    "kBEGIN",

    "kRESCUE",

    "kENSURE",

    "kEND",

    "kIF",

    "kUNLESS",

    "kTHEN",

    "kELSIF",

    "kELSE",

    "kCASE",

    "kWHEN",

    "kWHILE",

    "kUNTIL",

    "kFOR",

    "kBREAK",

    "kNEXT",

    "kREDO",

    "kRETRY",

    "kIN",

    "kDO",

    "kDO_COND",

    "kDO_BLOCK",

    "kDO_LAMBDA",

    "kRETURN",

    "kYIELD",

    "kSUPER",

    "kSELF",

    "kNIL",

    "kTRUE",

    "kFALSE",

    "kAND",

    "kOR",

    "kNOT",

    "kIF_MOD",

    "kUNLESS_MOD",

    "kWHILE_MOD",

    "kUNTIL_MOD",

    "kRESCUE_MOD",

    "kALIAS",

    "kDEFINED",

    "klBEGIN",

    "klEND",

    "k__LINE__",

    "k__FILE__",

    "k__ENCODING__",

    "tIDENTIFIER",

    "tFID",

    "tGVAR",

    "tIVAR",

    "tCONSTANT",

    "tCVAR",

    "tLABEL",

    "tINTEGER",

    "tFLOAT",

    "tRATIONAL",

    "tIMAGINARY",

    "tCHAR",

    "tNTH_REF",

    "tBACK_REF",

    "tSTRING_CONTENT",

    "tREGEXP_END",

    "tDOT",

    "tBACKSLASH",

    "tSP",

    "tSLASH_T",

    "tSLASH_F",

    "tSLASH_R",

    "tVTAB",

    "tUPLUS",

    "tUMINUS",

    "tPOW",

    "tCMP",

    "tEQ",

    "tEQQ",

    "tNEQ",

    "tGEQ",

    "tLEQ",

    "tANDOP",

    "tOROP",

    "tMATCH",

    "tNMATCH",

    "tDOT2",

    "tDOT3",

    "tBDOT2",

    "tBDOT3",

    "tAREF",

    "tASET",

    "tLSHFT",

    "tRSHFT",

    "tANDDOT",

    "tCOLON2",

    "tCOLON3",

    "tOP_ASGN",

    "tASSOC",

    "tLPAREN",

    "tLPAREN_ARG",

    "tRPAREN",

    "tLBRACK",

    "tLBRACE",

    "tLBRACE_ARG",

    "tSTAR",

    "tDSTAR",

    "tAMPER",

    "tLAMBDA",

    "tSYMBEG",

    "tSTRING_BEG",

    "tXSTRING_BEG",

    "tREGEXP_BEG",

    "tWORDS_BEG",

    "tQWORDS_BEG",

    "tSYMBOLS_BEG",

    "tQSYMBOLS_BEG",

    "tSTRING_END",

    "tSTRING_DEND",

    "tSTRING_DBEG",

    "tSTRING_DVAR",

    "tLAMBEG",

    "tLABEL_END",

    "tCOMMA",

    "tLCURLY",

    "tRCURLY",

    "tLBRACK2",

    "tEQL",

    "tPIPE",

    "tAMPER2",

    "tGT",

    "tLT",

    "tBACK_REF2",

    "tCARET",

    "tLPAREN2",

    "tRBRACK",

    "tSEMI",

    "tSPACE",

    "tNL",

    "tPLUS",

    "tMINUS",

    "tSTAR2",

    "tDIVIDE",

    "tPERCENT",

    "tTILDE",

    "tBANG",

    "tLOWEST",

    "tEH",

    "tCOLON",

    "tUMINUS_NUM",

    "tLAST_TOKEN",

]
;
}


impl  Parser  {

    fn yycdebug(&self, s: &str) {
        if  self.is_debug()  {
            eprintln!("{}", s);
        }
    }

}

/// Local alias
type YYValue =  Value ;

#[derive(Debug)]
struct YYStackItem {
    state: i32,
    value: YYValue,
    loc: YYLoc,
}

#[derive(Debug)]
pub struct YYStack {
    stack: Vec<YYStackItem>,
}

impl YYStack {
    pub(crate) fn new() -> Self {
        Self {
          stack: Vec::with_capacity(20),
        }
    }

    pub(crate) fn push(&mut self, state: i32, value: YYValue, loc: YYLoc) {
        self.stack.push(YYStackItem { state, value, loc });
    }

    pub(crate) fn pop(&mut self) {
        self.stack.pop();
    }

    pub(crate) fn pop_n(&mut self, num: usize) {
        let len = self.stack.len() - num;
        self.stack.truncate(len);
    }

    pub(crate) fn state_at(&self, i: usize) -> i32 {
        self.stack[self.len() - 1 - i].state
    }

    pub(crate) fn location_at(&self, i: usize) -> &YYLoc {
        &self.stack[self.len() - 1 - i].loc
    }

    pub(crate) fn borrow_value_at(&self, i: usize) -> &YYValue {
        &self.stack[self.len() - 1 - i].value
    }

    pub(crate) fn owned_value_at(&mut self, i: usize) -> YYValue {
        let len = self.len();
        std::mem::take(&mut self.stack[len - 1 - i].value)
    }

    pub(crate) fn len(&self) -> usize {
      self.stack.len()
    }
}

impl std::fmt::Display for YYStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let states = self.stack.iter().map(|e| e.state.to_string()).collect::<Vec<String>>().join(" ");
        let values = self.stack.iter().map(|e| format!("{:?}", e.value)).collect::<Vec<String>>().join(" ");
        f.write_fmt(format_args!("Stack now states = {} / values = {:?} ", states, values))
    }
}

impl  Parser  {
  /// Returned by a Bison action in order to stop the parsing process and
  /// return success (true).
  pub(crate) const YYACCEPT: i32 = 0;

  /// Returned by a Bison action in order to stop the parsing process and
  /// return failure (false).
  pub(crate) const YYABORT: i32 = 1;

  /// Returned by a Bison action in order to start error recovery without
  /// printing an error message.
  pub(crate) const YYERROR: i32 = 2;

  /// Internal return codes that are not supported for user semantic
  /// actions.
  pub(crate) const YYERRLAB: i32 = 3;
  pub(crate) const YYNEWSTATE: i32 = 4;
  pub(crate) const YYDEFAULT: i32 = 5;
  pub(crate) const YYREDUCE: i32 = 6;
  pub(crate) const YYERRLAB1: i32 = 7;
  #[allow(dead_code)]
  pub(crate) const YYRETURN: i32 = 8;

  /// Whether error recovery is being done.  In this state, the parser
  /// reads token until it reaches a known state, and then restarts normal
  /// operation.
  #[allow(dead_code)]
  pub(crate) fn recovering(&self) -> bool {
      self.yyerrstatus_ == 0
  }

    // Compute post-reduction state.
    // yystate:   the current state
    // yysym:     the nonterminal to push on the stack
    fn yy_lr_goto_state(&self, yystate: i32, yysym: i32) -> i32 {
        let idx = i32_to_usize(yysym - Self::YYNTOKENS_);
        let yyr = Self::yypgoto_[idx] + yystate;
        if (0..=Self::YYLAST_).contains(&yyr) {
            let yyr = i32_to_usize(yyr);
            if Self::yycheck_[yyr] == yystate {
                return Self::yytable_[yyr];
            }
        }
        Self::yydefgoto_[idx]
    }

    fn yyaction(&mut self, yyn: i32, yystack: &mut YYStack, yylen: &mut usize) -> Result<i32, ()> {
        // If YYLEN is nonzero, implement the default value of the action:
        // '$$ = $1'.  Otherwise, use the top of the stack.
        //
        // Otherwise, the following line sets YYVAL to garbage.
        // This behavior is undocumented and Bison
        // users should not rely upon it.
        #[allow(unused_assignments)]
        let mut yyval: YYValue = YYValue::new_uninitialized();
        let yyloc: YYLoc = make_yylloc(yystack, *yylen);

        self.yy_reduce_print(yyn, yystack);

        match yyn {
              2 =>  /* @1: %empty  */
  /* "src/parser/parse.y":346  */
                    {
                        self.yylexer.lex_state.set(EXPR_BEG);
                        self.current_arg_stack.push(None);
                        self.max_numparam_stack.push(true);

                        yyval = Value::None;
                    },


  3 =>  /* program: @1 top_compstmt  */
  /* "src/parser/parse.y":354  */
                    {
                        let top_compstmt =  MaybeNode::from(yystack.owned_value_at(0));
                        self.result = top_compstmt.map(Box::new);
                        yyval = Value::None;

                        self.current_arg_stack.pop();
                        self.max_numparam_stack.pop();
                    },


  4 =>  /* top_compstmt: top_stmts opt_terms  */
  /* "src/parser/parse.y":365  */
                    {
                        // TODO: run void_stmts
                        yyval = Value::MaybeNode(
                            self.builder.compstmt( NodeList::from(yystack.owned_value_at(1)))
                        );
                    },


  5 =>  /* top_stmts: none  */
  /* "src/parser/parse.y":374  */
                    {
                      yyval = Value::NodeList( Box::new(vec![]) );
                    },


  6 =>  /* top_stmts: top_stmt  */
  /* "src/parser/parse.y":378  */
                    {
                      yyval = Value::NodeList( Box::new(vec![  Node::from(yystack.owned_value_at(0)) ]) );
                    },


  7 =>  /* top_stmts: top_stmts terms top_stmt  */
  /* "src/parser/parse.y":382  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(2));
                        nodes.push(  Node::from(yystack.owned_value_at(0)) );
                        yyval = Value::NodeList( nodes );
                    },


  8 =>  /* top_stmts: error top_stmt  */
  /* "src/parser/parse.y":388  */
                    {
                      yyval = Value::NodeList( Box::new(vec![  Node::from(yystack.owned_value_at(0)) ]) );
                    },


  9 =>  /* top_stmt: stmt  */
  /* "src/parser/parse.y":394  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  10 =>  /* top_stmt: "`BEGIN'" begin_block  */
  /* "src/parser/parse.y":398  */
                    {
                        let BeginBlock { begin_t, body, end_t } =  BeginBlock::from(yystack.owned_value_at(0));
                        yyval = Value::Node(
                            self.builder.preexe( Token::from(yystack.owned_value_at(1)), begin_t, body, end_t)
                        );
                    },


  11 =>  /* begin_block: "{ (tLCURLY)" top_compstmt "}"  */
  /* "src/parser/parse.y":407  */
                    {
                        yyval = Value::new_begin_block(
                            BeginBlock {
                                begin_t:  Token::from(yystack.owned_value_at(2)),
                                body:  MaybeBoxedNode::from(yystack.owned_value_at(1)),
                                end_t:  Token::from(yystack.owned_value_at(0))
                            }
                        );
                    },


  12 =>  /* bodystmt: compstmt opt_rescue k_else compstmt opt_ensure  */
  /* "src/parser/parse.y":422  */
                    {
                        let compound_stmt =  MaybeBoxedNode::from(yystack.owned_value_at(4));
                        let rescue_bodies =  NodeList::from(yystack.owned_value_at(3));
                        if rescue_bodies.is_empty() {
                            return self.yyerror(yystack.location_at (2), DiagnosticMessage::ElseWithoutRescue {});
                        }

                        let else_ = Some((  Token::from(yystack.owned_value_at(2)),  MaybeBoxedNode::from(yystack.owned_value_at(1)) ));
                        let ensure =  OptEnsure::from(yystack.owned_value_at(0)).map(|ensure| (ensure.ensure_t, ensure.body));

                        yyval = Value::MaybeNode(
                            self.builder.begin_body(
                                compound_stmt,
                                rescue_bodies,
                                else_,
                                ensure
                            )
                        );
                    },


  13 =>  /* bodystmt: compstmt opt_rescue opt_ensure  */
  /* "src/parser/parse.y":444  */
                    {
                        let compound_stmt =  MaybeBoxedNode::from(yystack.owned_value_at(2));
                        let rescue_bodies =  NodeList::from(yystack.owned_value_at(1));
                        let ensure =  OptEnsure::from(yystack.owned_value_at(0)).map(|ensure| (ensure.ensure_t, ensure.body));

                        yyval = Value::MaybeNode(
                            self.builder.begin_body(
                                compound_stmt,
                                rescue_bodies,
                                None,
                                ensure
                            )
                        );
                    },


  14 =>  /* compstmt: stmts opt_terms  */
  /* "src/parser/parse.y":461  */
                    {
                        // TODO: run void_stmts
                        yyval = Value::MaybeNode(
                            self.builder.compstmt( NodeList::from(yystack.owned_value_at(1)))
                        );
                    },


  15 =>  /* stmts: none  */
  /* "src/parser/parse.y":470  */
                    {
                        yyval = Value::NodeList( Box::new(vec![]) );
                    },


  16 =>  /* stmts: stmt_or_begin  */
  /* "src/parser/parse.y":474  */
                    {
                        yyval = Value::NodeList( Box::new(vec![  Node::from(yystack.owned_value_at(0)) ]) );
                    },


  17 =>  /* stmts: stmts terms stmt_or_begin  */
  /* "src/parser/parse.y":478  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(2));
                        nodes.push(  Node::from(yystack.owned_value_at(0)) );
                        yyval = Value::NodeList(nodes);
                    },


  18 =>  /* stmts: error  */
  /* "src/parser/parse.y":484  */
                    {
                        yyval = Value::NodeList( Box::new(vec![]) );
                    },


  19 =>  /* stmt_or_begin: stmt  */
  /* "src/parser/parse.y":490  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  20 =>  /* $@2: %empty  */
  /* "src/parser/parse.y":494  */
                    {
                        return self.yyerror(yystack.location_at (0), DiagnosticMessage::BeginNotAtTopLevel {});
                    },


  21 =>  /* stmt_or_begin: "`BEGIN'" $@2 begin_block  */
  /* "src/parser/parse.y":498  */
                    {
                        yyval = Value::None;
                    },


  22 =>  /* @3: %empty  */
  /* "src/parser/parse.y":504  */
                    {
                        self.yylexer.lex_state.set(EXPR_FNAME|EXPR_FITEM);
                        yyval = Value::None;
                    },


  23 =>  /* stmt: "`alias'" fitem @3 fitem  */
  /* "src/parser/parse.y":509  */
                    {
                        yyval = Value::Node(
                            self.builder.alias( Token::from(yystack.owned_value_at(3)),  BoxedNode::from(yystack.owned_value_at(2)),  BoxedNode::from(yystack.owned_value_at(0)))
                        );
                    },


  24 =>  /* stmt: "`alias'" "global variable" "global variable"  */
  /* "src/parser/parse.y":515  */
                    {
                        yyval = Value::Node(
                            self.builder.alias(
                                 Token::from(yystack.owned_value_at(2)),
                                self.builder.gvar( Token::from(yystack.owned_value_at(1))),
                                self.builder.gvar( Token::from(yystack.owned_value_at(0))),
                            )
                        )
                    },


  25 =>  /* stmt: "`alias'" "global variable" "back reference"  */
  /* "src/parser/parse.y":525  */
                    {
                        yyval = Value::Node(
                            self.builder.alias(
                                 Token::from(yystack.owned_value_at(2)),
                                self.builder.gvar( Token::from(yystack.owned_value_at(1))),
                                self.builder.back_ref( Token::from(yystack.owned_value_at(0))),
                            )
                        )
                    },


  26 =>  /* stmt: "`alias'" "global variable" "numbered reference"  */
  /* "src/parser/parse.y":535  */
                    {
                        return self.yyerror(yystack.location_at (0), DiagnosticMessage::AliasNthRef {});
                    },


  27 =>  /* stmt: "`undef'" undef_list  */
  /* "src/parser/parse.y":539  */
                    {
                        yyval = Value::Node(
                            self.builder.undef_method(
                                 Token::from(yystack.owned_value_at(1)),
                                 NodeList::from(yystack.owned_value_at(0))
                            )
                        )
                    },


  28 =>  /* stmt: stmt "`if' modifier" expr_value  */
  /* "src/parser/parse.y":548  */
                    {
                        yyval = Value::Node(
                            self.builder.condition_mod(
                                Some( BoxedNode::from(yystack.owned_value_at(2))),
                                None,
                                 Token::from(yystack.owned_value_at(1)),
                                 BoxedNode::from(yystack.owned_value_at(0)),
                            )
                        );
                    },


  29 =>  /* stmt: stmt "`unless' modifier" expr_value  */
  /* "src/parser/parse.y":559  */
                    {
                        yyval = Value::Node(
                            self.builder.condition_mod(
                                None,
                                Some( BoxedNode::from(yystack.owned_value_at(2))),
                                 Token::from(yystack.owned_value_at(1)),
                                 BoxedNode::from(yystack.owned_value_at(0)),
                            )
                        );
                    },


  30 =>  /* stmt: stmt "`while' modifier" expr_value  */
  /* "src/parser/parse.y":570  */
                    {
                        yyval = Value::Node(
                            self.builder.loop_mod(
                                LoopType::While,
                                 BoxedNode::from(yystack.owned_value_at(2)),
                                 Token::from(yystack.owned_value_at(1)),
                                 BoxedNode::from(yystack.owned_value_at(0)),
                            )
                        );
                    },


  31 =>  /* stmt: stmt "`until' modifier" expr_value  */
  /* "src/parser/parse.y":581  */
                    {
                        yyval = Value::Node(
                            self.builder.loop_mod(
                                LoopType::Until,
                                 BoxedNode::from(yystack.owned_value_at(2)),
                                 Token::from(yystack.owned_value_at(1)),
                                 BoxedNode::from(yystack.owned_value_at(0)),
                            )
                        );
                    },


  32 =>  /* stmt: stmt "`rescue' modifier" stmt  */
  /* "src/parser/parse.y":592  */
                    {
                        let rescue_body = self.builder.rescue_body(
                             Token::from(yystack.owned_value_at(1)),
                            None,
                            None,
                            None,
                            None,
                            Some( BoxedNode::from(yystack.owned_value_at(0)))
                        );

                        yyval = Value::Node(
                            self.builder.begin_body(
                                Some( BoxedNode::from(yystack.owned_value_at(2))),
                                vec![*rescue_body],
                                None,
                                None,
                            ).expect("expected begin_body to return Some (compound_stmt was given)")
                        );
                    },


  33 =>  /* stmt: "`END'" "{ (tLCURLY)" compstmt "}"  */
  /* "src/parser/parse.y":612  */
                    {
                        if self.context.in_def() {
                            self.warn(yystack.location_at (3), DiagnosticMessage::EndInMethod {});
                        }

                        yyval = Value::Node(
                            self.builder.postexe(
                                 Token::from(yystack.owned_value_at(3)),
                                 Token::from(yystack.owned_value_at(2)),
                                 MaybeBoxedNode::from(yystack.owned_value_at(1)),
                                 Token::from(yystack.owned_value_at(0)),
                            )
                        );
                    },


  34 =>  /* stmt: command_asgn  */
  /* "src/parser/parse.y":627  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  35 =>  /* stmt: mlhs "=" command_call  */
  /* "src/parser/parse.y":631  */
                    {
                        let command_call =  BoxedNode::from(yystack.owned_value_at(0));
                        self.value_expr(&command_call)?;

                        yyval = Value::Node(
                            self.builder.multi_assign(
                                 BoxedNode::from(yystack.owned_value_at(2)),
                                 Token::from(yystack.owned_value_at(1)),
                                command_call
                            )
                        );
                    },


  36 =>  /* stmt: lhs "=" mrhs  */
  /* "src/parser/parse.y":644  */
                    {
                        let mrhs = self.builder.array(
                            None,
                             NodeList::from(yystack.owned_value_at(0)),
                            None
                        );
                        self.value_expr(&mrhs)?;

                        yyval = Value::Node(
                            self.builder.assign(
                                 BoxedNode::from(yystack.owned_value_at(2)),
                                 Token::from(yystack.owned_value_at(1)),
                                mrhs
                            )
                        );
                    },


  37 =>  /* stmt: mlhs "=" mrhs_arg "`rescue' modifier" stmt  */
  /* "src/parser/parse.y":661  */
                    {
                        let rescue_body = self.builder.rescue_body(
                             Token::from(yystack.owned_value_at(1)),
                            None,
                            None,
                            None,
                            None,
                            Some( BoxedNode::from(yystack.owned_value_at(0)))
                        );

                        let mrhs_arg =  BoxedNode::from(yystack.owned_value_at(2));
                        self.value_expr(&mrhs_arg)?;

                        let begin_body = self.builder.begin_body(
                            Some(mrhs_arg),
                            vec![ *rescue_body ],
                            None,
                            None,
                        ).expect("expected begin_body to return Some (compound_stmt was given)");

                        yyval = Value::Node(
                            self.builder.multi_assign(
                                 BoxedNode::from(yystack.owned_value_at(4)),
                                 Token::from(yystack.owned_value_at(3)),
                                begin_body
                            )
                        );
                    },


  38 =>  /* stmt: mlhs "=" mrhs_arg  */
  /* "src/parser/parse.y":690  */
                    {
                        yyval = Value::Node(
                            self.builder.multi_assign(
                                 BoxedNode::from(yystack.owned_value_at(2)),
                                 Token::from(yystack.owned_value_at(1)),
                                 BoxedNode::from(yystack.owned_value_at(0))
                            )
                        );
                    },


  39 =>  /* stmt: expr  */
  /* "src/parser/parse.y":700  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  40 =>  /* command_asgn: lhs "=" command_rhs  */
  /* "src/parser/parse.y":706  */
                    {
                        yyval = Value::Node(
                            self.builder.assign(
                                 BoxedNode::from(yystack.owned_value_at(2)),
                                 Token::from(yystack.owned_value_at(1)),
                                 BoxedNode::from(yystack.owned_value_at(0))
                            )
                        );
                    },


  41 =>  /* command_asgn: var_lhs "operator-assignment" command_rhs  */
  /* "src/parser/parse.y":716  */
                    {
                        yyval = Value::Node(
                            self.builder.op_assign(
                                 BoxedNode::from(yystack.owned_value_at(2)),
                                 Token::from(yystack.owned_value_at(1)),
                                 BoxedNode::from(yystack.owned_value_at(0))
                            )?
                        );
                    },


  42 =>  /* command_asgn: primary_value "[ (tLBRACK2)" opt_call_args rbracket "operator-assignment" command_rhs  */
  /* "src/parser/parse.y":726  */
                    {
                        yyval = Value::Node(
                            self.builder.op_assign(
                                self.builder.index(
                                     BoxedNode::from(yystack.owned_value_at(5)),
                                     Token::from(yystack.owned_value_at(4)),
                                     NodeList::from(yystack.owned_value_at(3)),
                                     Token::from(yystack.owned_value_at(2))
                                ),
                                 Token::from(yystack.owned_value_at(1)),
                                 BoxedNode::from(yystack.owned_value_at(0))
                            )?
                        );
                    },


  43 =>  /* command_asgn: primary_value call_op "local variable or method" "operator-assignment" command_rhs  */
  /* "src/parser/parse.y":741  */
                    {
                        yyval = Value::Node(
                            self.builder.op_assign(
                                self.builder.call_method(
                                    Some( BoxedNode::from(yystack.owned_value_at(4))),
                                    Some( Token::from(yystack.owned_value_at(3))),
                                    Some( Token::from(yystack.owned_value_at(2))),
                                    None,
                                    vec![],
                                    None
                                ),
                                 Token::from(yystack.owned_value_at(1)),
                                 BoxedNode::from(yystack.owned_value_at(0))
                            )?
                        );
                    },


  44 =>  /* command_asgn: primary_value call_op "constant" "operator-assignment" command_rhs  */
  /* "src/parser/parse.y":758  */
                    {
                        yyval = Value::Node(
                            self.builder.op_assign(
                                self.builder.call_method(
                                    Some( BoxedNode::from(yystack.owned_value_at(4))),
                                    Some( Token::from(yystack.owned_value_at(3))),
                                    Some( Token::from(yystack.owned_value_at(2))),
                                    None,
                                    vec![],
                                    None
                                ),
                                 Token::from(yystack.owned_value_at(1)),
                                 BoxedNode::from(yystack.owned_value_at(0))
                            )?
                        );
                    },


  45 =>  /* command_asgn: primary_value "::" "constant" "operator-assignment" command_rhs  */
  /* "src/parser/parse.y":775  */
                    {
                        let const_ = self.builder.const_op_assignable(
                            self.builder.const_fetch(
                                 BoxedNode::from(yystack.owned_value_at(4)),
                                 Token::from(yystack.owned_value_at(3)),
                                 Token::from(yystack.owned_value_at(2))
                            )
                        );
                        yyval = Value::Node(
                            self.builder.op_assign(
                                const_,
                                 Token::from(yystack.owned_value_at(1)),
                                 BoxedNode::from(yystack.owned_value_at(0))
                            )?
                        );
                    },


  46 =>  /* command_asgn: primary_value "::" "local variable or method" "operator-assignment" command_rhs  */
  /* "src/parser/parse.y":792  */
                    {
                        yyval = Value::Node(
                            self.builder.op_assign(
                                self.builder.call_method(
                                    Some( BoxedNode::from(yystack.owned_value_at(4))),
                                    Some( Token::from(yystack.owned_value_at(3))),
                                    Some( Token::from(yystack.owned_value_at(2))),
                                    None,
                                    vec![],
                                    None
                                ),
                                 Token::from(yystack.owned_value_at(1)),
                                 BoxedNode::from(yystack.owned_value_at(0))
                            )?
                        );
                    },


  47 =>  /* command_asgn: defn_head f_opt_paren_args "=" command  */
  /* "src/parser/parse.y":809  */
                    {
                        let DefnHead { def_t, name_t } =  DefnHead::from(yystack.owned_value_at(3));
                        let TokenWithContext { token: name_t, ctx } = name_t;
                        self.validate_endless_method_name(&name_t)?;

                        yyval = Value::Node(
                            self.builder.def_endless_method(
                                def_t,
                                name_t,
                                 MaybeBoxedNode::from(yystack.owned_value_at(2)),
                                 Token::from(yystack.owned_value_at(1)),
                                Some( BoxedNode::from(yystack.owned_value_at(0))),
                            )?
                        );

                        self.local_pop();
                        self.current_arg_stack.pop();
                        self.context.set_in_def(ctx.in_def());
                    },


  48 =>  /* command_asgn: defn_head f_opt_paren_args "=" command "`rescue' modifier" arg  */
  /* "src/parser/parse.y":829  */
                    {
                        let DefnHead { def_t, name_t } =  DefnHead::from(yystack.owned_value_at(5));
                        let TokenWithContext { token: name_t, ctx } = name_t;
                        self.validate_endless_method_name(&name_t)?;

                        let rescue_body = self.builder.rescue_body(
                             Token::from(yystack.owned_value_at(1)),
                            None,
                            None,
                            None,
                            None,
                            Some( BoxedNode::from(yystack.owned_value_at(0))),
                        );

                        let method_body = self.builder.begin_body(
                            Some( BoxedNode::from(yystack.owned_value_at(2))),
                            vec![ *rescue_body ],
                            None,
                            None,
                        );

                        yyval = Value::Node(
                            self.builder.def_endless_method(
                                def_t,
                                name_t,
                                 MaybeBoxedNode::from(yystack.owned_value_at(4)),
                                 Token::from(yystack.owned_value_at(3)),
                                method_body,
                            )?
                        );

                        self.local_pop();
                        self.current_arg_stack.pop();
                        self.context.set_in_def(ctx.in_def());
                    },


  49 =>  /* command_asgn: defs_head f_opt_paren_args "=" command  */
  /* "src/parser/parse.y":865  */
                    {
                        let DefsHead { def_t, definee, dot_t, name_t } =  DefsHead::from(yystack.owned_value_at(3));
                        let TokenWithContext { token: name_t, ctx } = name_t;
                        self.validate_endless_method_name(&name_t)?;

                        yyval = Value::Node(
                            self.builder.def_endless_singleton(
                                def_t,
                                definee,
                                dot_t,
                                name_t,
                                 MaybeBoxedNode::from(yystack.owned_value_at(2)),
                                 Token::from(yystack.owned_value_at(1)),
                                Some( BoxedNode::from(yystack.owned_value_at(0))),
                            )?
                        );

                        self.local_pop();
                        self.current_arg_stack.pop();
                        self.context.set_in_def(ctx.in_def());
                    },


  50 =>  /* command_asgn: defs_head f_opt_paren_args "=" command "`rescue' modifier" arg  */
  /* "src/parser/parse.y":887  */
                    {
                        let DefsHead { def_t, definee, dot_t, name_t } =  DefsHead::from(yystack.owned_value_at(5));
                        let TokenWithContext { token: name_t, ctx } = name_t;
                        self.validate_endless_method_name(&name_t)?;

                        let rescue_body = self.builder.rescue_body(
                             Token::from(yystack.owned_value_at(1)),
                            None,
                            None,
                            None,
                            None,
                            Some( BoxedNode::from(yystack.owned_value_at(0))),
                        );

                        let method_body = self.builder.begin_body(
                            Some( BoxedNode::from(yystack.owned_value_at(2))),
                            vec![ *rescue_body ],
                            None,
                            None,
                        );

                        yyval = Value::Node(
                            self.builder.def_endless_singleton(
                                def_t,
                                definee,
                                dot_t,
                                name_t,
                                 MaybeBoxedNode::from(yystack.owned_value_at(4)),
                                 Token::from(yystack.owned_value_at(3)),
                                method_body,
                            )?
                        );

                        self.local_pop();
                        self.current_arg_stack.pop();
                        self.context.set_in_def(ctx.in_def());
                    },


  51 =>  /* command_asgn: backref "operator-assignment" command_rhs  */
  /* "src/parser/parse.y":925  */
                    {
                        yyval = Value::Node(
                            self.builder.op_assign(
                                 BoxedNode::from(yystack.owned_value_at(2)),
                                 Token::from(yystack.owned_value_at(1)),
                                 BoxedNode::from(yystack.owned_value_at(0))
                            )?
                        );
                    },


  52 =>  /* command_rhs: command_call  */
  /* "src/parser/parse.y":937  */
                    {
                        let command_call =  BoxedNode::from(yystack.owned_value_at(0));
                        self.value_expr(&command_call)?;
                        yyval = Value::Node(command_call);
                    },


  53 =>  /* command_rhs: command_call "`rescue' modifier" stmt  */
  /* "src/parser/parse.y":943  */
                    {
                        let command_call =  BoxedNode::from(yystack.owned_value_at(2));
                        self.value_expr(&command_call)?;

                        let rescue_body = self.builder.rescue_body(
                             Token::from(yystack.owned_value_at(1)),
                            None,
                            None,
                            None,
                            None,
                            Some( BoxedNode::from(yystack.owned_value_at(0)))
                        );

                        yyval = Value::Node(
                            self.builder.begin_body(
                                Some(command_call),
                                vec![ *rescue_body ],
                                None,
                                None,
                            ).expect("expected begin_body to return Some (compound_stmt was given)")
                        );
                    },


  54 =>  /* command_rhs: command_asgn  */
  /* "src/parser/parse.y":966  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  55 =>  /* expr: command_call  */
  /* "src/parser/parse.y":972  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  56 =>  /* expr: expr "`and'" expr  */
  /* "src/parser/parse.y":976  */
                    {
                        yyval = Value::Node(
                            self.builder.logical_op(
                                LogicalOp::And,
                                 BoxedNode::from(yystack.owned_value_at(2)),
                                 Token::from(yystack.owned_value_at(1)),
                                 BoxedNode::from(yystack.owned_value_at(0))
                            )?
                        );
                    },


  57 =>  /* expr: expr "`or'" expr  */
  /* "src/parser/parse.y":987  */
                    {
                        yyval = Value::Node(
                            self.builder.logical_op(
                                LogicalOp::Or,
                                 BoxedNode::from(yystack.owned_value_at(2)),
                                 Token::from(yystack.owned_value_at(1)),
                                 BoxedNode::from(yystack.owned_value_at(0))
                            )?
                        );
                    },


  58 =>  /* expr: "`not'" opt_nl expr  */
  /* "src/parser/parse.y":998  */
                    {
                        yyval = Value::Node(
                            self.builder.not_op(
                                 Token::from(yystack.owned_value_at(2)),
                                None,
                                Some( BoxedNode::from(yystack.owned_value_at(0))),
                                None
                            )?
                        );
                    },


  59 =>  /* expr: "!" command_call  */
  /* "src/parser/parse.y":1009  */
                    {
                        yyval = Value::Node(
                            self.builder.not_op(
                                 Token::from(yystack.owned_value_at(1)),
                                None,
                                Some( BoxedNode::from(yystack.owned_value_at(0))),
                                None
                            )?
                        );
                    },


  60 =>  /* @4: %empty  */
  /* "src/parser/parse.y":1020  */
                    {
                        let arg = match yystack.borrow_value_at(1) {
                            Value::Node(node) => node,
                            other => unreachable!("expected Node, got {:?}", other)
                        };
                        self.value_expr(arg)?;

                        self.yylexer.lex_state.set(EXPR_BEG|EXPR_LABEL);
                        self.yylexer.command_start = false;
                        self.pattern_variables.push();

                        yyval = Value::Bool(self.context.in_kwarg());
                        self.context.set_in_kwarg(true);
                    },


  61 =>  /* expr: arg "=>" @4 p_top_expr_body  */
  /* "src/parser/parse.y":1035  */
                    {
                        self.pattern_variables.pop();
                        self.context.set_in_kwarg( Bool::from(yystack.owned_value_at(1)));

                        yyval = Value::Node(
                            self.builder.match_pattern(
                                 BoxedNode::from(yystack.owned_value_at(3)),
                                 Token::from(yystack.owned_value_at(2)),
                                 BoxedNode::from(yystack.owned_value_at(0))
                            )
                        );
                    },


  62 =>  /* @5: %empty  */
  /* "src/parser/parse.y":1048  */
                    {
                        let arg = match yystack.borrow_value_at(1) {
                            Value::Node(node) => node,
                            other => unreachable!("expected Node, got {:?}", other)
                        };
                        self.value_expr(arg)?;

                        self.yylexer.lex_state.set(EXPR_BEG|EXPR_LABEL);
                        self.yylexer.command_start = false;
                        self.pattern_variables.push();

                        yyval = Value::Bool(self.context.in_kwarg());
                        self.context.set_in_kwarg(true);
                    },


  63 =>  /* expr: arg "`in'" @5 p_top_expr_body  */
  /* "src/parser/parse.y":1063  */
                    {
                        self.pattern_variables.pop();
                        self.context.set_in_kwarg( Bool::from(yystack.owned_value_at(1)));

                        yyval = Value::Node(
                            self.builder.match_pattern_p(
                                 BoxedNode::from(yystack.owned_value_at(3)),
                                 Token::from(yystack.owned_value_at(2)),
                                 BoxedNode::from(yystack.owned_value_at(0))
                            )
                        );
                    },


  64 =>  /* expr: arg  */
  /* "src/parser/parse.y":1076  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  65 =>  /* def_name: fname  */
  /* "src/parser/parse.y":1082  */
                    {
                        self.local_push();
                        self.current_arg_stack.push(None);

                        yyval = Value::TokenWithContext(
                            Box::new(
                                TokenWithContext {
                                    token:  Token::from(yystack.owned_value_at(0)),
                                    ctx: self.context.dump()
                                }
                            )
                        );

                        self.context.set_in_def(true);
                    },


  66 =>  /* defn_head: k_def def_name  */
  /* "src/parser/parse.y":1100  */
                    {
                        yyval = Value::new_defn_head(
                            DefnHead {
                                def_t:  Token::from(yystack.owned_value_at(1)),
                                name_t:  TokenWithContext::from(yystack.owned_value_at(0))
                            }
                        );
                    },


  67 =>  /* @6: %empty  */
  /* "src/parser/parse.y":1111  */
                    {
                        self.yylexer.lex_state.set(EXPR_FNAME);
                        self.context.set_in_argdef(true);
                        yyval = Value::None;
                    },


  68 =>  /* defs_head: k_def singleton dot_or_colon @6 def_name  */
  /* "src/parser/parse.y":1117  */
                    {
                        self.yylexer.lex_state.set(EXPR_ENDFN|EXPR_LABEL);

                        yyval = Value::new_defs_head(
                            DefsHead {
                                def_t:  Token::from(yystack.owned_value_at(4)),
                                definee:  BoxedNode::from(yystack.owned_value_at(3)),
                                dot_t:  Token::from(yystack.owned_value_at(2)),
                                name_t:  TokenWithContext::from(yystack.owned_value_at(0))
                            }
                        );
                    },


  69 =>  /* expr_value: expr  */
  /* "src/parser/parse.y":1132  */
                    {
                        let expr =  BoxedNode::from(yystack.owned_value_at(0));
                        self.value_expr(&expr)?;
                        yyval = Value::Node(expr);
                    },


  70 =>  /* @7: %empty  */
  /* "src/parser/parse.y":1139  */
                    {
                        self.yylexer.cond.push(true);
                        yyval = Value::None;
                    },


  71 =>  /* expr_value_do: @7 expr_value do  */
  /* "src/parser/parse.y":1144  */
                    {
                        self.yylexer.cond.pop();

                        yyval = Value::new_expr_value_do(
                            ExprValueDo {
                                value:  BoxedNode::from(yystack.owned_value_at(1)),
                                do_t:  Token::from(yystack.owned_value_at(0))
                            }
                        );
                    },


  72 =>  /* command_call: command  */
  /* "src/parser/parse.y":1158  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  73 =>  /* command_call: block_command  */
  /* "src/parser/parse.y":1162  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  74 =>  /* block_command: block_call  */
  /* "src/parser/parse.y":1168  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  75 =>  /* block_command: block_call call_op2 operation2 command_args  */
  /* "src/parser/parse.y":1172  */
                    {
                        yyval = Value::Node(
                            self.builder.call_method(
                                Some( BoxedNode::from(yystack.owned_value_at(3))),
                                Some( Token::from(yystack.owned_value_at(2))),
                                Some( Token::from(yystack.owned_value_at(1))),
                                None,
                                 NodeList::from(yystack.owned_value_at(0)),
                                None
                            )
                        );
                    },


  76 =>  /* @8: %empty  */
  /* "src/parser/parse.y":1187  */
                    {
                        yyval = Value::Context(self.context.dump());
                        self.context.set_in_block(true);
                    },


  77 =>  /* cmd_brace_block: "{ arg" @8 brace_body "}"  */
  /* "src/parser/parse.y":1192  */
                    {
                        self.context.set_in_block( Context::from(yystack.owned_value_at(2)).in_block());
                        let BraceBody { args_type, body } =  BraceBody::from(yystack.owned_value_at(1));
                        yyval = Value::new_cmd_brace_block(
                            CmdBraceBlock {
                                begin_t:  Token::from(yystack.owned_value_at(3)),
                                args_type,
                                body,
                                end_t:  Token::from(yystack.owned_value_at(0))
                            }
                        );
                    },


  78 =>  /* fcall: operation  */
  /* "src/parser/parse.y":1207  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  79 =>  /* command: fcall command_args  */
  /* "src/parser/parse.y":1213  */
                    {
                        yyval = Value::Node(
                            self.builder.call_method(
                                None,
                                None,
                                Some( Token::from(yystack.owned_value_at(1))),
                                None,
                                 NodeList::from(yystack.owned_value_at(0)),
                                None
                            )
                        );
                    },


  80 =>  /* command: fcall command_args cmd_brace_block  */
  /* "src/parser/parse.y":1226  */
                    {
                        let method_call = self.builder.call_method(
                            None,
                            None,
                            Some( Token::from(yystack.owned_value_at(2))),
                            None,
                             NodeList::from(yystack.owned_value_at(1)),
                            None
                        );
                        let CmdBraceBlock { begin_t, args_type, body, end_t } =  CmdBraceBlock::from(yystack.owned_value_at(0));

                        yyval = Value::Node(
                            self.builder.block(
                                method_call,
                                begin_t,
                                args_type,
                                body,
                                end_t
                            )?
                        );
                    },


  81 =>  /* command: primary_value call_op operation2 command_args  */
  /* "src/parser/parse.y":1248  */
                    {
                        yyval = Value::Node(
                            self.builder.call_method(
                                Some( BoxedNode::from(yystack.owned_value_at(3))),
                                Some( Token::from(yystack.owned_value_at(2))),
                                Some( Token::from(yystack.owned_value_at(1))),
                                None,
                                 NodeList::from(yystack.owned_value_at(0)),
                                None
                            )
                        );
                    },


  82 =>  /* command: primary_value call_op operation2 command_args cmd_brace_block  */
  /* "src/parser/parse.y":1261  */
                    {
                        let method_call = self.builder.call_method(
                            Some( BoxedNode::from(yystack.owned_value_at(4))),
                            Some( Token::from(yystack.owned_value_at(3))),
                            Some( Token::from(yystack.owned_value_at(2))),
                            None,
                             NodeList::from(yystack.owned_value_at(1)),
                            None
                        );
                        let CmdBraceBlock { begin_t, args_type, body, end_t } =  CmdBraceBlock::from(yystack.owned_value_at(0));

                        yyval = Value::Node(
                            self.builder.block(
                                method_call,
                                begin_t,
                                args_type,
                                body,
                                end_t
                            )?
                        );
                    },


  83 =>  /* command: primary_value "::" operation2 command_args  */
  /* "src/parser/parse.y":1283  */
                    {
                        yyval = Value::Node(
                            self.builder.call_method(
                                Some( BoxedNode::from(yystack.owned_value_at(3))),
                                Some( Token::from(yystack.owned_value_at(2))),
                                Some( Token::from(yystack.owned_value_at(1))),
                                None,
                                 NodeList::from(yystack.owned_value_at(0)),
                                None
                            )
                        );
                    },


  84 =>  /* command: primary_value "::" operation2 command_args cmd_brace_block  */
  /* "src/parser/parse.y":1296  */
                    {
                        let method_call = self.builder.call_method(
                            Some( BoxedNode::from(yystack.owned_value_at(4))),
                            Some( Token::from(yystack.owned_value_at(3))),
                            Some( Token::from(yystack.owned_value_at(2))),
                            None,
                             NodeList::from(yystack.owned_value_at(1)),
                            None
                        );
                        let CmdBraceBlock { begin_t, args_type, body, end_t } =  CmdBraceBlock::from(yystack.owned_value_at(0));

                        yyval = Value::Node(
                            self.builder.block(
                                method_call,
                                begin_t,
                                args_type,
                                body,
                                end_t
                            )?
                        );
                    },


  85 =>  /* command: "`super'" command_args  */
  /* "src/parser/parse.y":1318  */
                    {
                        yyval = Value::Node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Super,
                                 Token::from(yystack.owned_value_at(1)),
                                None,
                                 NodeList::from(yystack.owned_value_at(0)),
                                None
                            )?
                        );
                    },


  86 =>  /* command: "`yield'" command_args  */
  /* "src/parser/parse.y":1330  */
                    {
                        yyval = Value::Node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Yield,
                                 Token::from(yystack.owned_value_at(1)),
                                None,
                                 NodeList::from(yystack.owned_value_at(0)),
                                None
                            )?
                        );
                    },


  87 =>  /* command: k_return call_args  */
  /* "src/parser/parse.y":1342  */
                    {
                        yyval = Value::Node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Return,
                                 Token::from(yystack.owned_value_at(1)),
                                None,
                                 NodeList::from(yystack.owned_value_at(0)),
                                None
                            )?
                        );
                    },


  88 =>  /* command: "`break'" call_args  */
  /* "src/parser/parse.y":1354  */
                    {
                        yyval = Value::Node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Break,
                                 Token::from(yystack.owned_value_at(1)),
                                None,
                                 NodeList::from(yystack.owned_value_at(0)),
                                None
                            )?
                        );
                    },


  89 =>  /* command: "`next'" call_args  */
  /* "src/parser/parse.y":1366  */
                    {
                        yyval = Value::Node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Next,
                                 Token::from(yystack.owned_value_at(1)),
                                None,
                                 NodeList::from(yystack.owned_value_at(0)),
                                None
                            )?
                        );
                    },


  90 =>  /* mlhs: mlhs_basic  */
  /* "src/parser/parse.y":1380  */
                    {
                        yyval = Value::Node(
                            self.builder.multi_lhs(
                                None,
                                 NodeList::from(yystack.owned_value_at(0)),
                                None
                            )
                        );
                    },


  91 =>  /* mlhs: "(" mlhs_inner rparen  */
  /* "src/parser/parse.y":1390  */
                    {
                        yyval = Value::Node(
                            self.builder.begin(
                                 Token::from(yystack.owned_value_at(2)),
                                Some( BoxedNode::from(yystack.owned_value_at(1))),
                                 Token::from(yystack.owned_value_at(0))
                            )
                        );
                    },


  92 =>  /* mlhs_inner: mlhs_basic  */
  /* "src/parser/parse.y":1402  */
                    {
                        yyval = Value::Node(
                            self.builder.multi_lhs(
                                None,
                                 NodeList::from(yystack.owned_value_at(0)),
                                None
                            )
                        );
                    },


  93 =>  /* mlhs_inner: "(" mlhs_inner rparen  */
  /* "src/parser/parse.y":1412  */
                    {
                        let mlhs_inner =  Node::from(yystack.owned_value_at(1));
                        let mlhs_items = match mlhs_inner {
                            Node::Mlhs(nodes::Mlhs { items, .. }) => {
                                items
                            }
                            other => {
                                unreachable!("unsupported mlhs item {:?}", other)
                            }
                        };

                        yyval = Value::Node(
                            self.builder.multi_lhs(
                                Some( Token::from(yystack.owned_value_at(2))),
                                mlhs_items,
                                Some( Token::from(yystack.owned_value_at(0)))
                            )
                        );
                    },


  94 =>  /* mlhs_basic: mlhs_head  */
  /* "src/parser/parse.y":1434  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  95 =>  /* mlhs_basic: mlhs_head mlhs_item  */
  /* "src/parser/parse.y":1438  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(1));
                        nodes.push(  Node::from(yystack.owned_value_at(0)) );
                        yyval = Value::NodeList(nodes);
                    },


  96 =>  /* mlhs_basic: mlhs_head "*" mlhs_node  */
  /* "src/parser/parse.y":1444  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(2));
                        let mlhs_node = *self.builder.splat( Token::from(yystack.owned_value_at(1)), Some( BoxedNode::from(yystack.owned_value_at(0))));
                        nodes.push(mlhs_node);
                        yyval = Value::NodeList(nodes);
                    },


  97 =>  /* mlhs_basic: mlhs_head "*" mlhs_node "," mlhs_post  */
  /* "src/parser/parse.y":1451  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(4));
                        let mlhs_node = *self.builder.splat( Token::from(yystack.owned_value_at(3)), Some( BoxedNode::from(yystack.owned_value_at(2))));
                        let mut mlhs_post =  NodeList::from(yystack.owned_value_at(0));

                        nodes.reserve(1 + mlhs_post.len());
                        nodes.push(mlhs_node);
                        nodes.append(&mut mlhs_post);

                        yyval = Value::NodeList(nodes);
                    },


  98 =>  /* mlhs_basic: mlhs_head "*"  */
  /* "src/parser/parse.y":1463  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(1));
                        let splat = *self.builder.splat( Token::from(yystack.owned_value_at(0)), None);
                        nodes.push(splat);
                        yyval = Value::NodeList(nodes);
                    },


  99 =>  /* mlhs_basic: mlhs_head "*" "," mlhs_post  */
  /* "src/parser/parse.y":1470  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(3));
                        let splat = *self.builder.splat( Token::from(yystack.owned_value_at(2)), None);
                        let mut mlhs_post =  NodeList::from(yystack.owned_value_at(0));

                        nodes.reserve(1 + mlhs_post.len());
                        nodes.push(splat);
                        nodes.append(&mut mlhs_post);

                        yyval = Value::NodeList(nodes);
                    },


  100 =>  /* mlhs_basic: "*" mlhs_node  */
  /* "src/parser/parse.y":1482  */
                    {
                        yyval = Value::NodeList(
                            Box::new(
                                vec![
                                    *self.builder.splat(
                                         Token::from(yystack.owned_value_at(1)),
                                        Some( BoxedNode::from(yystack.owned_value_at(0)))
                                    )
                                ]
                            )
                        );
                    },


  101 =>  /* mlhs_basic: "*" mlhs_node "," mlhs_post  */
  /* "src/parser/parse.y":1495  */
                    {
                        let splat = *self.builder.splat( Token::from(yystack.owned_value_at(3)), Some( BoxedNode::from(yystack.owned_value_at(2))));
                        let mut mlhs_post =  NodeList::from(yystack.owned_value_at(0));

                        let mut nodes = Box::new(Vec::with_capacity(1 + mlhs_post.len()));
                        nodes.push(splat);
                        nodes.append(&mut mlhs_post);

                        yyval = Value::NodeList(nodes);
                    },


  102 =>  /* mlhs_basic: "*"  */
  /* "src/parser/parse.y":1506  */
                    {
                        yyval = Value::NodeList(
                            Box::new(
                                vec![
                                    *self.builder.splat(
                                         Token::from(yystack.owned_value_at(0)),
                                        None
                                    )
                                ]
                            )
                        );
                    },


  103 =>  /* mlhs_basic: "*" "," mlhs_post  */
  /* "src/parser/parse.y":1519  */
                    {
                        let splat = *self.builder.splat( Token::from(yystack.owned_value_at(2)), None);
                        let mut mlhs_post =  NodeList::from(yystack.owned_value_at(0));

                        let mut nodes = Box::new(Vec::with_capacity(1 + mlhs_post.len()));
                        nodes.push(splat);
                        nodes.append(&mut mlhs_post);

                        yyval = Value::NodeList(nodes);
                    },


  104 =>  /* mlhs_item: mlhs_node  */
  /* "src/parser/parse.y":1532  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  105 =>  /* mlhs_item: "(" mlhs_inner rparen  */
  /* "src/parser/parse.y":1536  */
                    {
                        yyval = Value::Node(
                            self.builder.begin(
                                 Token::from(yystack.owned_value_at(2)),
                                Some( BoxedNode::from(yystack.owned_value_at(1))),
                                 Token::from(yystack.owned_value_at(0))
                            )
                        );
                    },


  106 =>  /* mlhs_head: mlhs_item ","  */
  /* "src/parser/parse.y":1548  */
                    {
                        yyval = Value::NodeList( Box::new(vec![  Node::from(yystack.owned_value_at(1)) ]) );
                    },


  107 =>  /* mlhs_head: mlhs_head mlhs_item ","  */
  /* "src/parser/parse.y":1552  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(2));
                        nodes.push(  Node::from(yystack.owned_value_at(1)) );
                        yyval = Value::NodeList(nodes);
                    },


  108 =>  /* mlhs_post: mlhs_item  */
  /* "src/parser/parse.y":1560  */
                    {
                        yyval = Value::NodeList( Box::new(vec![  Node::from(yystack.owned_value_at(0)) ]) );
                    },


  109 =>  /* mlhs_post: mlhs_post "," mlhs_item  */
  /* "src/parser/parse.y":1564  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(2));
                        nodes.push(  Node::from(yystack.owned_value_at(0)) );
                        yyval = Value::NodeList(nodes);
                    },


  110 =>  /* mlhs_node: user_variable  */
  /* "src/parser/parse.y":1572  */
                    {
                        yyval = Value::Node(
                            self.builder.assignable( BoxedNode::from(yystack.owned_value_at(0)))?
                        );
                    },


  111 =>  /* mlhs_node: keyword_variable  */
  /* "src/parser/parse.y":1578  */
                    {
                        yyval = Value::Node(
                            self.builder.assignable( BoxedNode::from(yystack.owned_value_at(0)))?
                        );
                    },


  112 =>  /* mlhs_node: primary_value "[ (tLBRACK2)" opt_call_args rbracket  */
  /* "src/parser/parse.y":1584  */
                    {
                        yyval = Value::Node(
                            self.builder.index_asgn(
                                 BoxedNode::from(yystack.owned_value_at(3)),
                                 Token::from(yystack.owned_value_at(2)),
                                 NodeList::from(yystack.owned_value_at(1)),
                                 Token::from(yystack.owned_value_at(0))
                            )
                        );
                    },


  113 =>  /* mlhs_node: primary_value call_op "local variable or method"  */
  /* "src/parser/parse.y":1595  */
                    {
                        let op_t =  Token::from(yystack.owned_value_at(1));
                        if op_t.token_type == Lexer::tANDDOT {
                            return self.yyerror(yystack.location_at (1), DiagnosticMessage::CsendInsideMasgn {});
                        }

                        yyval = Value::Node(
                            self.builder.attr_asgn(
                                 BoxedNode::from(yystack.owned_value_at(2)),
                                op_t,
                                 Token::from(yystack.owned_value_at(0))
                            )
                        );
                    },


  114 =>  /* mlhs_node: primary_value "::" "local variable or method"  */
  /* "src/parser/parse.y":1610  */
                    {
                        yyval = Value::Node(
                            self.builder.attr_asgn(
                                 BoxedNode::from(yystack.owned_value_at(2)),
                                 Token::from(yystack.owned_value_at(1)),
                                 Token::from(yystack.owned_value_at(0))
                            )
                        );
                    },


  115 =>  /* mlhs_node: primary_value call_op "constant"  */
  /* "src/parser/parse.y":1620  */
                    {
                        let op_t =  Token::from(yystack.owned_value_at(1));
                        if op_t.token_type == Lexer::tANDDOT {
                            return self.yyerror(yystack.location_at (1), DiagnosticMessage::CsendInsideMasgn {});
                        }

                        yyval = Value::Node(
                            self.builder.attr_asgn(
                                 BoxedNode::from(yystack.owned_value_at(2)),
                                op_t,
                                 Token::from(yystack.owned_value_at(0))
                            )
                        );
                    },


  116 =>  /* mlhs_node: primary_value "::" "constant"  */
  /* "src/parser/parse.y":1635  */
                    {
                        yyval = Value::Node(
                            self.builder.assignable(
                                self.builder.const_fetch(
                                     BoxedNode::from(yystack.owned_value_at(2)),
                                     Token::from(yystack.owned_value_at(1)),
                                     Token::from(yystack.owned_value_at(0))
                                )
                            )?
                        );
                    },


  117 =>  /* mlhs_node: ":: at EXPR_BEG" "constant"  */
  /* "src/parser/parse.y":1647  */
                    {
                        yyval = Value::Node(
                            self.builder.assignable(
                                self.builder.const_global(
                                     Token::from(yystack.owned_value_at(1)),
                                     Token::from(yystack.owned_value_at(0))
                                )
                            )?
                        );
                    },


  118 =>  /* mlhs_node: backref  */
  /* "src/parser/parse.y":1658  */
                    {
                        yyval = Value::Node(
                            self.builder.assignable(
                                 BoxedNode::from(yystack.owned_value_at(0))
                            )?
                        );
                    },


  119 =>  /* lhs: user_variable  */
  /* "src/parser/parse.y":1668  */
                    {
                        yyval = Value::Node(
                            self.builder.assignable( BoxedNode::from(yystack.owned_value_at(0)))?
                        );
                    },


  120 =>  /* lhs: keyword_variable  */
  /* "src/parser/parse.y":1674  */
                    {
                        yyval = Value::Node(
                            self.builder.assignable( BoxedNode::from(yystack.owned_value_at(0)))?
                        );
                    },


  121 =>  /* lhs: primary_value "[ (tLBRACK2)" opt_call_args rbracket  */
  /* "src/parser/parse.y":1680  */
                    {
                        yyval = Value::Node(
                            self.builder.index_asgn(
                                 BoxedNode::from(yystack.owned_value_at(3)),
                                 Token::from(yystack.owned_value_at(2)),
                                 NodeList::from(yystack.owned_value_at(1)),
                                 Token::from(yystack.owned_value_at(0))
                            )
                        )
                    },


  122 =>  /* lhs: primary_value call_op "local variable or method"  */
  /* "src/parser/parse.y":1691  */
                    {
                        yyval = Value::Node(
                            self.builder.attr_asgn(
                                 BoxedNode::from(yystack.owned_value_at(2)),
                                 Token::from(yystack.owned_value_at(1)),
                                 Token::from(yystack.owned_value_at(0))
                            )
                        );
                    },


  123 =>  /* lhs: primary_value "::" "local variable or method"  */
  /* "src/parser/parse.y":1701  */
                    {
                        yyval = Value::Node(
                            self.builder.attr_asgn(
                                 BoxedNode::from(yystack.owned_value_at(2)),
                                 Token::from(yystack.owned_value_at(1)),
                                 Token::from(yystack.owned_value_at(0))
                            )
                        );
                    },


  124 =>  /* lhs: primary_value call_op "constant"  */
  /* "src/parser/parse.y":1711  */
                    {
                        yyval = Value::Node(
                            self.builder.attr_asgn(
                                 BoxedNode::from(yystack.owned_value_at(2)),
                                 Token::from(yystack.owned_value_at(1)),
                                 Token::from(yystack.owned_value_at(0))
                            )
                        );
                    },


  125 =>  /* lhs: primary_value "::" "constant"  */
  /* "src/parser/parse.y":1721  */
                    {
                        yyval = Value::Node(
                            self.builder.assignable(
                                self.builder.const_fetch(
                                     BoxedNode::from(yystack.owned_value_at(2)),
                                     Token::from(yystack.owned_value_at(1)),
                                     Token::from(yystack.owned_value_at(0)),
                                )
                            )?
                        );
                    },


  126 =>  /* lhs: ":: at EXPR_BEG" "constant"  */
  /* "src/parser/parse.y":1733  */
                    {
                        yyval = Value::Node(
                            self.builder.assignable(
                                self.builder.const_global(
                                     Token::from(yystack.owned_value_at(1)),
                                     Token::from(yystack.owned_value_at(0)),
                                )
                            )?
                        );
                    },


  127 =>  /* lhs: backref  */
  /* "src/parser/parse.y":1744  */
                    {
                        yyval = Value::Node(
                            self.builder.assignable(
                                 BoxedNode::from(yystack.owned_value_at(0))
                            )?
                        );
                    },


  128 =>  /* cname: "local variable or method"  */
  /* "src/parser/parse.y":1754  */
                    {
                        return self.yyerror(yystack.location_at (0), DiagnosticMessage::ClassOrModuleNameMustBeConstant {});
                    },


  129 =>  /* cname: "constant"  */
  /* "src/parser/parse.y":1758  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  130 =>  /* cpath: ":: at EXPR_BEG" cname  */
  /* "src/parser/parse.y":1764  */
                    {
                        yyval = Value::Node(
                            self.builder.const_global( Token::from(yystack.owned_value_at(1)),  Token::from(yystack.owned_value_at(0)))
                        );
                    },


  131 =>  /* cpath: cname  */
  /* "src/parser/parse.y":1770  */
                    {
                        yyval = Value::Node(
                            self.builder.const_( Token::from(yystack.owned_value_at(0)))
                        );
                    },


  132 =>  /* cpath: primary_value "::" cname  */
  /* "src/parser/parse.y":1776  */
                    {
                        yyval = Value::Node(
                            self.builder.const_fetch(
                                 BoxedNode::from(yystack.owned_value_at(2)),
                                 Token::from(yystack.owned_value_at(1)),
                                 Token::from(yystack.owned_value_at(0)),
                            )
                        );
                    },


  133 =>  /* fname: "local variable or method"  */
  /* "src/parser/parse.y":1788  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  134 =>  /* fname: "constant"  */
  /* "src/parser/parse.y":1792  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  135 =>  /* fname: "method"  */
  /* "src/parser/parse.y":1796  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  136 =>  /* fname: op  */
  /* "src/parser/parse.y":1800  */
                    {
                        self.yylexer.lex_state.set(EXPR_ENDFN);
                        yyval =  yystack.owned_value_at(0);
                    },


  137 =>  /* fname: reswords  */
  /* "src/parser/parse.y":1805  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  138 =>  /* fitem: fname  */
  /* "src/parser/parse.y":1811  */
                    {
                        yyval = Value::Node(
                            self.builder.symbol_internal( Token::from(yystack.owned_value_at(0)))
                        );
                    },


  139 =>  /* fitem: symbol  */
  /* "src/parser/parse.y":1817  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  140 =>  /* undef_list: fitem  */
  /* "src/parser/parse.y":1823  */
                    {
                        yyval = Value::NodeList( Box::new(vec![  Node::from(yystack.owned_value_at(0)) ]) );
                    },


  141 =>  /* @9: %empty  */
  /* "src/parser/parse.y":1827  */
                    {
                        self.yylexer.lex_state.set(EXPR_FNAME|EXPR_FITEM);
                        yyval = Value::None;
                    },


  142 =>  /* undef_list: undef_list "," @9 fitem  */
  /* "src/parser/parse.y":1832  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(3));
                        nodes.push(  Node::from(yystack.owned_value_at(0)) );
                        yyval = Value::NodeList(nodes);
                    },


  143 =>  /* op: "|"  */
  /* "src/parser/parse.y":1839  */
                             { yyval =  yystack.owned_value_at(0); },


  144 =>  /* op: "^"  */
  /* "src/parser/parse.y":1840  */
                             { yyval =  yystack.owned_value_at(0); },


  145 =>  /* op: "& (tAMPER2)"  */
  /* "src/parser/parse.y":1841  */
                             { yyval =  yystack.owned_value_at(0); },


  146 =>  /* op: "<=>"  */
  /* "src/parser/parse.y":1842  */
                             { yyval =  yystack.owned_value_at(0); },


  147 =>  /* op: "=="  */
  /* "src/parser/parse.y":1843  */
                             { yyval =  yystack.owned_value_at(0); },


  148 =>  /* op: "==="  */
  /* "src/parser/parse.y":1844  */
                             { yyval =  yystack.owned_value_at(0); },


  149 =>  /* op: "=~"  */
  /* "src/parser/parse.y":1845  */
                             { yyval =  yystack.owned_value_at(0); },


  150 =>  /* op: "!~"  */
  /* "src/parser/parse.y":1846  */
                             { yyval =  yystack.owned_value_at(0); },


  151 =>  /* op: ">"  */
  /* "src/parser/parse.y":1847  */
                             { yyval =  yystack.owned_value_at(0); },


  152 =>  /* op: ">="  */
  /* "src/parser/parse.y":1848  */
                             { yyval =  yystack.owned_value_at(0); },


  153 =>  /* op: "<"  */
  /* "src/parser/parse.y":1849  */
                             { yyval =  yystack.owned_value_at(0); },


  154 =>  /* op: "<="  */
  /* "src/parser/parse.y":1850  */
                             { yyval =  yystack.owned_value_at(0); },


  155 =>  /* op: "!="  */
  /* "src/parser/parse.y":1851  */
                             { yyval =  yystack.owned_value_at(0); },


  156 =>  /* op: "<<"  */
  /* "src/parser/parse.y":1852  */
                             { yyval =  yystack.owned_value_at(0); },


  157 =>  /* op: ">>"  */
  /* "src/parser/parse.y":1853  */
                             { yyval =  yystack.owned_value_at(0); },


  158 =>  /* op: "+"  */
  /* "src/parser/parse.y":1854  */
                             { yyval =  yystack.owned_value_at(0); },


  159 =>  /* op: "-"  */
  /* "src/parser/parse.y":1855  */
                             { yyval =  yystack.owned_value_at(0); },


  160 =>  /* op: "* (tSTAR2)"  */
  /* "src/parser/parse.y":1856  */
                             { yyval =  yystack.owned_value_at(0); },


  161 =>  /* op: "*"  */
  /* "src/parser/parse.y":1857  */
                             { yyval =  yystack.owned_value_at(0); },


  162 =>  /* op: "/"  */
  /* "src/parser/parse.y":1858  */
                             { yyval =  yystack.owned_value_at(0); },


  163 =>  /* op: "%"  */
  /* "src/parser/parse.y":1859  */
                             { yyval =  yystack.owned_value_at(0); },


  164 =>  /* op: "**"  */
  /* "src/parser/parse.y":1860  */
                             { yyval =  yystack.owned_value_at(0); },


  165 =>  /* op: "**arg"  */
  /* "src/parser/parse.y":1861  */
                             { yyval =  yystack.owned_value_at(0); },


  166 =>  /* op: "!"  */
  /* "src/parser/parse.y":1862  */
                             { yyval =  yystack.owned_value_at(0); },


  167 =>  /* op: "~"  */
  /* "src/parser/parse.y":1863  */
                             { yyval =  yystack.owned_value_at(0); },


  168 =>  /* op: "unary+"  */
  /* "src/parser/parse.y":1864  */
                             { yyval =  yystack.owned_value_at(0); },


  169 =>  /* op: "unary-"  */
  /* "src/parser/parse.y":1865  */
                             { yyval =  yystack.owned_value_at(0); },


  170 =>  /* op: "[]"  */
  /* "src/parser/parse.y":1866  */
                             { yyval =  yystack.owned_value_at(0); },


  171 =>  /* op: "[]="  */
  /* "src/parser/parse.y":1867  */
                             { yyval =  yystack.owned_value_at(0); },


  172 =>  /* op: "`"  */
  /* "src/parser/parse.y":1868  */
                             { yyval =  yystack.owned_value_at(0); },


  173 =>  /* reswords: "`__LINE__'"  */
  /* "src/parser/parse.y":1871  */
                                { yyval =  yystack.owned_value_at(0); },


  174 =>  /* reswords: "`__FILE__'"  */
  /* "src/parser/parse.y":1872  */
                                { yyval =  yystack.owned_value_at(0); },


  175 =>  /* reswords: "`__ENCODING__'"  */
  /* "src/parser/parse.y":1873  */
                                { yyval =  yystack.owned_value_at(0); },


  176 =>  /* reswords: "`BEGIN'"  */
  /* "src/parser/parse.y":1874  */
                                { yyval =  yystack.owned_value_at(0); },


  177 =>  /* reswords: "`END'"  */
  /* "src/parser/parse.y":1875  */
                                { yyval =  yystack.owned_value_at(0); },


  178 =>  /* reswords: "`alias'"  */
  /* "src/parser/parse.y":1876  */
                                { yyval =  yystack.owned_value_at(0); },


  179 =>  /* reswords: "`and'"  */
  /* "src/parser/parse.y":1877  */
                                { yyval =  yystack.owned_value_at(0); },


  180 =>  /* reswords: "`begin'"  */
  /* "src/parser/parse.y":1878  */
                                { yyval =  yystack.owned_value_at(0); },


  181 =>  /* reswords: "`break'"  */
  /* "src/parser/parse.y":1879  */
                                { yyval =  yystack.owned_value_at(0); },


  182 =>  /* reswords: "`case'"  */
  /* "src/parser/parse.y":1880  */
                                { yyval =  yystack.owned_value_at(0); },


  183 =>  /* reswords: "`class'"  */
  /* "src/parser/parse.y":1881  */
                                { yyval =  yystack.owned_value_at(0); },


  184 =>  /* reswords: "`def'"  */
  /* "src/parser/parse.y":1882  */
                                { yyval =  yystack.owned_value_at(0); },


  185 =>  /* reswords: "`defined?'"  */
  /* "src/parser/parse.y":1883  */
                                { yyval =  yystack.owned_value_at(0); },


  186 =>  /* reswords: "`do'"  */
  /* "src/parser/parse.y":1884  */
                                { yyval =  yystack.owned_value_at(0); },


  187 =>  /* reswords: "`else'"  */
  /* "src/parser/parse.y":1885  */
                                { yyval =  yystack.owned_value_at(0); },


  188 =>  /* reswords: "`elsif'"  */
  /* "src/parser/parse.y":1886  */
                                { yyval =  yystack.owned_value_at(0); },


  189 =>  /* reswords: "`end'"  */
  /* "src/parser/parse.y":1887  */
                                { yyval =  yystack.owned_value_at(0); },


  190 =>  /* reswords: "`ensure'"  */
  /* "src/parser/parse.y":1888  */
                                { yyval =  yystack.owned_value_at(0); },


  191 =>  /* reswords: "`false'"  */
  /* "src/parser/parse.y":1889  */
                                { yyval =  yystack.owned_value_at(0); },


  192 =>  /* reswords: "`for'"  */
  /* "src/parser/parse.y":1890  */
                                { yyval =  yystack.owned_value_at(0); },


  193 =>  /* reswords: "`in'"  */
  /* "src/parser/parse.y":1891  */
                                { yyval =  yystack.owned_value_at(0); },


  194 =>  /* reswords: "`module'"  */
  /* "src/parser/parse.y":1892  */
                                { yyval =  yystack.owned_value_at(0); },


  195 =>  /* reswords: "`next'"  */
  /* "src/parser/parse.y":1893  */
                                { yyval =  yystack.owned_value_at(0); },


  196 =>  /* reswords: "`nil'"  */
  /* "src/parser/parse.y":1894  */
                                { yyval =  yystack.owned_value_at(0); },


  197 =>  /* reswords: "`not'"  */
  /* "src/parser/parse.y":1895  */
                                { yyval =  yystack.owned_value_at(0); },


  198 =>  /* reswords: "`or'"  */
  /* "src/parser/parse.y":1896  */
                                { yyval =  yystack.owned_value_at(0); },


  199 =>  /* reswords: "`redo'"  */
  /* "src/parser/parse.y":1897  */
                                { yyval =  yystack.owned_value_at(0); },


  200 =>  /* reswords: "`rescue'"  */
  /* "src/parser/parse.y":1898  */
                                { yyval =  yystack.owned_value_at(0); },


  201 =>  /* reswords: "`retry'"  */
  /* "src/parser/parse.y":1899  */
                                { yyval =  yystack.owned_value_at(0); },


  202 =>  /* reswords: "`return'"  */
  /* "src/parser/parse.y":1900  */
                                { yyval =  yystack.owned_value_at(0); },


  203 =>  /* reswords: "`self'"  */
  /* "src/parser/parse.y":1901  */
                                { yyval =  yystack.owned_value_at(0); },


  204 =>  /* reswords: "`super'"  */
  /* "src/parser/parse.y":1902  */
                                { yyval =  yystack.owned_value_at(0); },


  205 =>  /* reswords: "`then'"  */
  /* "src/parser/parse.y":1903  */
                                { yyval =  yystack.owned_value_at(0); },


  206 =>  /* reswords: "`true'"  */
  /* "src/parser/parse.y":1904  */
                                { yyval =  yystack.owned_value_at(0); },


  207 =>  /* reswords: "`undef'"  */
  /* "src/parser/parse.y":1905  */
                                { yyval =  yystack.owned_value_at(0); },


  208 =>  /* reswords: "`when'"  */
  /* "src/parser/parse.y":1906  */
                                { yyval =  yystack.owned_value_at(0); },


  209 =>  /* reswords: "`yield'"  */
  /* "src/parser/parse.y":1907  */
                                { yyval =  yystack.owned_value_at(0); },


  210 =>  /* reswords: "`if'"  */
  /* "src/parser/parse.y":1908  */
                                { yyval =  yystack.owned_value_at(0); },


  211 =>  /* reswords: "`unless'"  */
  /* "src/parser/parse.y":1909  */
                                { yyval =  yystack.owned_value_at(0); },


  212 =>  /* reswords: "`while'"  */
  /* "src/parser/parse.y":1910  */
                                { yyval =  yystack.owned_value_at(0); },


  213 =>  /* reswords: "`until'"  */
  /* "src/parser/parse.y":1911  */
                                { yyval =  yystack.owned_value_at(0); },


  214 =>  /* arg: lhs "=" arg_rhs  */
  /* "src/parser/parse.y":1915  */
                    {
                        yyval = Value::Node(
                            self.builder.assign(
                                 BoxedNode::from(yystack.owned_value_at(2)),
                                 Token::from(yystack.owned_value_at(1)),
                                 BoxedNode::from(yystack.owned_value_at(0))
                            )
                        );
                    },


  215 =>  /* arg: var_lhs "operator-assignment" arg_rhs  */
  /* "src/parser/parse.y":1925  */
                    {
                        yyval = Value::Node(
                            self.builder.op_assign(
                                 BoxedNode::from(yystack.owned_value_at(2)),
                                 Token::from(yystack.owned_value_at(1)),
                                 BoxedNode::from(yystack.owned_value_at(0))
                            )?
                        );
                    },


  216 =>  /* arg: primary_value "[ (tLBRACK2)" opt_call_args rbracket "operator-assignment" arg_rhs  */
  /* "src/parser/parse.y":1935  */
                    {
                        yyval = Value::Node(
                            self.builder.op_assign(
                                self.builder.index(
                                     BoxedNode::from(yystack.owned_value_at(5)),
                                     Token::from(yystack.owned_value_at(4)),
                                     NodeList::from(yystack.owned_value_at(3)),
                                     Token::from(yystack.owned_value_at(2))
                                ),
                                 Token::from(yystack.owned_value_at(1)),
                                 BoxedNode::from(yystack.owned_value_at(0))
                            )?
                        );
                    },


  217 =>  /* arg: primary_value call_op "local variable or method" "operator-assignment" arg_rhs  */
  /* "src/parser/parse.y":1950  */
                    {
                        yyval = Value::Node(
                            self.builder.op_assign(
                                self.builder.call_method(
                                    Some( BoxedNode::from(yystack.owned_value_at(4))),
                                    Some( Token::from(yystack.owned_value_at(3))),
                                    Some( Token::from(yystack.owned_value_at(2))),
                                    None,
                                    vec![],
                                    None
                                ),
                                 Token::from(yystack.owned_value_at(1)),
                                 BoxedNode::from(yystack.owned_value_at(0))
                            )?
                        );
                    },


  218 =>  /* arg: primary_value call_op "constant" "operator-assignment" arg_rhs  */
  /* "src/parser/parse.y":1967  */
                    {
                        yyval = Value::Node(
                            self.builder.op_assign(
                                self.builder.call_method(
                                    Some( BoxedNode::from(yystack.owned_value_at(4))),
                                    Some( Token::from(yystack.owned_value_at(3))),
                                    Some( Token::from(yystack.owned_value_at(2))),
                                    None,
                                    vec![],
                                    None
                                ),
                                 Token::from(yystack.owned_value_at(1)),
                                 BoxedNode::from(yystack.owned_value_at(0))
                            )?
                        );
                    },


  219 =>  /* arg: primary_value "::" "local variable or method" "operator-assignment" arg_rhs  */
  /* "src/parser/parse.y":1984  */
                    {
                        yyval = Value::Node(
                            self.builder.op_assign(
                                self.builder.call_method(
                                    Some( BoxedNode::from(yystack.owned_value_at(4))),
                                    Some( Token::from(yystack.owned_value_at(3))),
                                    Some( Token::from(yystack.owned_value_at(2))),
                                    None,
                                    vec![],
                                    None
                                ),
                                 Token::from(yystack.owned_value_at(1)),
                                 BoxedNode::from(yystack.owned_value_at(0))
                            )?
                        );
                    },


  220 =>  /* arg: primary_value "::" "constant" "operator-assignment" arg_rhs  */
  /* "src/parser/parse.y":2001  */
                    {
                        let const_ = self.builder.const_op_assignable(
                            self.builder.const_fetch(
                                 BoxedNode::from(yystack.owned_value_at(4)),
                                 Token::from(yystack.owned_value_at(3)),
                                 Token::from(yystack.owned_value_at(2))
                            )
                        );
                        yyval = Value::Node(
                            self.builder.op_assign(
                                const_,
                                 Token::from(yystack.owned_value_at(1)),
                                 BoxedNode::from(yystack.owned_value_at(0))
                            )?
                        );
                    },


  221 =>  /* arg: ":: at EXPR_BEG" "constant" "operator-assignment" arg_rhs  */
  /* "src/parser/parse.y":2018  */
                    {
                        let const_ = self.builder.const_op_assignable(
                            self.builder.const_global(
                                 Token::from(yystack.owned_value_at(3)),
                                 Token::from(yystack.owned_value_at(2))
                            )
                        );
                        yyval = Value::Node(
                            self.builder.op_assign(
                                const_,
                                 Token::from(yystack.owned_value_at(1)),
                                 BoxedNode::from(yystack.owned_value_at(0))
                            )?
                        );
                    },


  222 =>  /* arg: backref "operator-assignment" arg_rhs  */
  /* "src/parser/parse.y":2034  */
                    {
                        yyval = Value::Node(
                            self.builder.op_assign(
                                 BoxedNode::from(yystack.owned_value_at(2)),
                                 Token::from(yystack.owned_value_at(1)),
                                 BoxedNode::from(yystack.owned_value_at(0))
                            )?
                        );
                    },


  223 =>  /* arg: arg ".." arg  */
  /* "src/parser/parse.y":2044  */
                    {
                        let left =  BoxedNode::from(yystack.owned_value_at(2));
                        self.value_expr(&left)?;

                        let right =  BoxedNode::from(yystack.owned_value_at(0));
                        self.value_expr(&right)?;

                        yyval = Value::Node(
                            self.builder.range_inclusive(
                                Some(left),
                                 Token::from(yystack.owned_value_at(1)),
                                Some(right)
                            )
                        );
                    },


  224 =>  /* arg: arg "..." arg  */
  /* "src/parser/parse.y":2060  */
                    {
                        let left =  BoxedNode::from(yystack.owned_value_at(2));
                        self.value_expr(&left)?;

                        let right =  BoxedNode::from(yystack.owned_value_at(0));
                        self.value_expr(&right)?;

                        yyval = Value::Node(
                            self.builder.range_exclusive(
                                Some(left),
                                 Token::from(yystack.owned_value_at(1)),
                                Some(right)
                            )
                        );
                    },


  225 =>  /* arg: arg ".."  */
  /* "src/parser/parse.y":2076  */
                    {
                        let left =  BoxedNode::from(yystack.owned_value_at(1));
                        self.value_expr(&left)?;

                        yyval = Value::Node(
                            self.builder.range_inclusive(
                                Some(left),
                                 Token::from(yystack.owned_value_at(0)),
                                None
                            )
                        );
                    },


  226 =>  /* arg: arg "..."  */
  /* "src/parser/parse.y":2089  */
                    {
                        let left =  BoxedNode::from(yystack.owned_value_at(1));
                        self.value_expr(&left)?;

                        yyval = Value::Node(
                            self.builder.range_exclusive(
                                Some(left),
                                 Token::from(yystack.owned_value_at(0)),
                                None
                            )
                        );
                    },


  227 =>  /* arg: "(.." arg  */
  /* "src/parser/parse.y":2102  */
                    {
                        let right =  BoxedNode::from(yystack.owned_value_at(0));
                        self.value_expr(&right)?;

                        yyval = Value::Node(
                            self.builder.range_inclusive(
                                None,
                                 Token::from(yystack.owned_value_at(1)),
                                Some(right)
                            )
                        );
                    },


  228 =>  /* arg: "(..." arg  */
  /* "src/parser/parse.y":2115  */
                    {
                        let right =  BoxedNode::from(yystack.owned_value_at(0));
                        self.value_expr(&right)?;

                        yyval = Value::Node(
                            self.builder.range_exclusive(
                                None,
                                 Token::from(yystack.owned_value_at(1)),
                                Some(right)
                            )
                        );
                    },


  229 =>  /* arg: arg "+" arg  */
  /* "src/parser/parse.y":2128  */
                    {
                        yyval = Value::Node(
                            self.builder.binary_op( BoxedNode::from(yystack.owned_value_at(2)),  Token::from(yystack.owned_value_at(1)),  BoxedNode::from(yystack.owned_value_at(0)))?
                        );
                    },


  230 =>  /* arg: arg "-" arg  */
  /* "src/parser/parse.y":2134  */
                    {
                        yyval = Value::Node(
                            self.builder.binary_op( BoxedNode::from(yystack.owned_value_at(2)),  Token::from(yystack.owned_value_at(1)),  BoxedNode::from(yystack.owned_value_at(0)))?
                        );
                    },


  231 =>  /* arg: arg "* (tSTAR2)" arg  */
  /* "src/parser/parse.y":2140  */
                    {
                        yyval = Value::Node(
                            self.builder.binary_op( BoxedNode::from(yystack.owned_value_at(2)),  Token::from(yystack.owned_value_at(1)),  BoxedNode::from(yystack.owned_value_at(0)))?
                        );
                    },


  232 =>  /* arg: arg "/" arg  */
  /* "src/parser/parse.y":2146  */
                    {
                        yyval = Value::Node(
                            self.builder.binary_op( BoxedNode::from(yystack.owned_value_at(2)),  Token::from(yystack.owned_value_at(1)),  BoxedNode::from(yystack.owned_value_at(0)))?
                        );
                    },


  233 =>  /* arg: arg "%" arg  */
  /* "src/parser/parse.y":2152  */
                    {
                        yyval = Value::Node(
                            self.builder.binary_op( BoxedNode::from(yystack.owned_value_at(2)),  Token::from(yystack.owned_value_at(1)),  BoxedNode::from(yystack.owned_value_at(0)))?
                        );
                    },


  234 =>  /* arg: arg "**" arg  */
  /* "src/parser/parse.y":2158  */
                    {
                        yyval = Value::Node(
                            self.builder.binary_op( BoxedNode::from(yystack.owned_value_at(2)),  Token::from(yystack.owned_value_at(1)),  BoxedNode::from(yystack.owned_value_at(0)))?
                        );
                    },


  235 =>  /* arg: tUMINUS_NUM simple_numeric "**" arg  */
  /* "src/parser/parse.y":2164  */
                    {
                        yyval = Value::Node(
                            self.builder.unary_op(
                                 Token::from(yystack.owned_value_at(3)),
                                self.builder.binary_op(
                                     BoxedNode::from(yystack.owned_value_at(2)),
                                     Token::from(yystack.owned_value_at(1)),
                                     BoxedNode::from(yystack.owned_value_at(0))
                                )?
                            )?
                        );
                    },


  236 =>  /* arg: "unary+" arg  */
  /* "src/parser/parse.y":2177  */
                    {
                        yyval = Value::Node(
                            self.builder.unary_op(
                                 Token::from(yystack.owned_value_at(1)),
                                 BoxedNode::from(yystack.owned_value_at(0))
                            )?
                        );
                    },


  237 =>  /* arg: "unary-" arg  */
  /* "src/parser/parse.y":2186  */
                    {
                        yyval = Value::Node(
                            self.builder.unary_op(
                                 Token::from(yystack.owned_value_at(1)),
                                 BoxedNode::from(yystack.owned_value_at(0))
                            )?
                        );
                    },


  238 =>  /* arg: arg "|" arg  */
  /* "src/parser/parse.y":2195  */
                    {
                        yyval = Value::Node(
                            self.builder.binary_op( BoxedNode::from(yystack.owned_value_at(2)),  Token::from(yystack.owned_value_at(1)),  BoxedNode::from(yystack.owned_value_at(0)))?
                        );
                    },


  239 =>  /* arg: arg "^" arg  */
  /* "src/parser/parse.y":2201  */
                    {
                        yyval = Value::Node(
                            self.builder.binary_op( BoxedNode::from(yystack.owned_value_at(2)),  Token::from(yystack.owned_value_at(1)),  BoxedNode::from(yystack.owned_value_at(0)))?
                        );
                    },


  240 =>  /* arg: arg "& (tAMPER2)" arg  */
  /* "src/parser/parse.y":2207  */
                    {
                        yyval = Value::Node(
                            self.builder.binary_op( BoxedNode::from(yystack.owned_value_at(2)),  Token::from(yystack.owned_value_at(1)),  BoxedNode::from(yystack.owned_value_at(0)))?
                        );
                    },


  241 =>  /* arg: arg "<=>" arg  */
  /* "src/parser/parse.y":2213  */
                    {
                        yyval = Value::Node(
                            self.builder.binary_op( BoxedNode::from(yystack.owned_value_at(2)),  Token::from(yystack.owned_value_at(1)),  BoxedNode::from(yystack.owned_value_at(0)))?
                        );
                    },


  242 =>  /* arg: rel_expr  */
  /* "src/parser/parse.y":2219  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  243 =>  /* arg: arg "==" arg  */
  /* "src/parser/parse.y":2223  */
                    {
                        yyval = Value::Node(
                            self.builder.binary_op( BoxedNode::from(yystack.owned_value_at(2)),  Token::from(yystack.owned_value_at(1)),  BoxedNode::from(yystack.owned_value_at(0)))?
                        );
                    },


  244 =>  /* arg: arg "===" arg  */
  /* "src/parser/parse.y":2229  */
                    {
                        yyval = Value::Node(
                            self.builder.binary_op( BoxedNode::from(yystack.owned_value_at(2)),  Token::from(yystack.owned_value_at(1)),  BoxedNode::from(yystack.owned_value_at(0)))?
                        );
                    },


  245 =>  /* arg: arg "!=" arg  */
  /* "src/parser/parse.y":2235  */
                    {
                        yyval = Value::Node(
                            self.builder.binary_op( BoxedNode::from(yystack.owned_value_at(2)),  Token::from(yystack.owned_value_at(1)),  BoxedNode::from(yystack.owned_value_at(0)))?
                        );
                    },


  246 =>  /* arg: arg "=~" arg  */
  /* "src/parser/parse.y":2241  */
                    {
                        yyval = Value::Node(
                            self.builder.match_op( BoxedNode::from(yystack.owned_value_at(2)),  Token::from(yystack.owned_value_at(1)),  BoxedNode::from(yystack.owned_value_at(0)))?
                        );
                    },


  247 =>  /* arg: arg "!~" arg  */
  /* "src/parser/parse.y":2247  */
                    {
                        yyval = Value::Node(
                            self.builder.binary_op(
                                 BoxedNode::from(yystack.owned_value_at(2)),
                                 Token::from(yystack.owned_value_at(1)),
                                 BoxedNode::from(yystack.owned_value_at(0))
                            )?
                        );
                    },


  248 =>  /* arg: "!" arg  */
  /* "src/parser/parse.y":2257  */
                    {
                        yyval = Value::Node(
                            self.builder.not_op(
                                 Token::from(yystack.owned_value_at(1)),
                                None,
                                Some( BoxedNode::from(yystack.owned_value_at(0))),
                                None
                            )?
                        );
                    },


  249 =>  /* arg: "~" arg  */
  /* "src/parser/parse.y":2268  */
                    {
                        yyval = Value::Node(
                            self.builder.unary_op(
                                 Token::from(yystack.owned_value_at(1)),
                                 BoxedNode::from(yystack.owned_value_at(0))
                            )?
                        );
                    },


  250 =>  /* arg: arg "<<" arg  */
  /* "src/parser/parse.y":2277  */
                    {
                        yyval = Value::Node(
                            self.builder.binary_op( BoxedNode::from(yystack.owned_value_at(2)),  Token::from(yystack.owned_value_at(1)),  BoxedNode::from(yystack.owned_value_at(0)))?
                        );
                    },


  251 =>  /* arg: arg ">>" arg  */
  /* "src/parser/parse.y":2283  */
                    {
                        yyval = Value::Node(
                            self.builder.binary_op( BoxedNode::from(yystack.owned_value_at(2)),  Token::from(yystack.owned_value_at(1)),  BoxedNode::from(yystack.owned_value_at(0)))?
                        );
                    },


  252 =>  /* arg: arg "&&" arg  */
  /* "src/parser/parse.y":2289  */
                    {
                        yyval = Value::Node(
                            self.builder.logical_op(
                                LogicalOp::And,
                                 BoxedNode::from(yystack.owned_value_at(2)),
                                 Token::from(yystack.owned_value_at(1)),
                                 BoxedNode::from(yystack.owned_value_at(0))
                            )?
                        );
                    },


  253 =>  /* arg: arg "||" arg  */
  /* "src/parser/parse.y":2300  */
                    {
                        yyval = Value::Node(
                            self.builder.logical_op(
                                LogicalOp::Or,
                                 BoxedNode::from(yystack.owned_value_at(2)),
                                 Token::from(yystack.owned_value_at(1)),
                                 BoxedNode::from(yystack.owned_value_at(0))
                            )?
                        );
                    },


  254 =>  /* @10: %empty  */
  /* "src/parser/parse.y":2311  */
                    {
                        self.context.set_in_defined(true);
                        yyval = Value::None;
                    },


  255 =>  /* arg: "`defined?'" opt_nl @10 arg  */
  /* "src/parser/parse.y":2316  */
                    {
                        self.context.set_in_defined(false);
                        yyval = Value::Node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Defined,
                                 Token::from(yystack.owned_value_at(3)),
                                None,
                                vec![  Node::from(yystack.owned_value_at(0)) ],
                                None
                            )?
                        );
                    },


  256 =>  /* arg: arg tEH arg opt_nl tCOLON arg  */
  /* "src/parser/parse.y":2329  */
                    {
                        let expr =  BoxedNode::from(yystack.owned_value_at(5));
                        self.value_expr(&expr)?;

                        yyval = Value::Node(
                            self.builder.ternary(
                                expr,
                                 Token::from(yystack.owned_value_at(4)),
                                 BoxedNode::from(yystack.owned_value_at(3)),
                                 Token::from(yystack.owned_value_at(1)),
                                 BoxedNode::from(yystack.owned_value_at(0))
                            )
                        );
                    },


  257 =>  /* arg: defn_head f_opt_paren_args "=" arg  */
  /* "src/parser/parse.y":2344  */
                    {
                        let DefnHead { def_t, name_t } =  DefnHead::from(yystack.owned_value_at(3));
                        let TokenWithContext { token: name_t, ctx } = name_t;
                        self.validate_endless_method_name(&name_t)?;

                        yyval = Value::Node(
                            self.builder.def_endless_method(
                                def_t,
                                name_t,
                                 MaybeBoxedNode::from(yystack.owned_value_at(2)),
                                 Token::from(yystack.owned_value_at(1)),
                                Some( BoxedNode::from(yystack.owned_value_at(0)))
                            )?
                        );

                        self.local_pop();
                        self.current_arg_stack.pop();
                        self.context.set_in_def(ctx.in_def());
                    },


  258 =>  /* arg: defn_head f_opt_paren_args "=" arg "`rescue' modifier" arg  */
  /* "src/parser/parse.y":2364  */
                    {
                        let DefnHead { def_t, name_t } =  DefnHead::from(yystack.owned_value_at(5));
                        let TokenWithContext { token: name_t, ctx } = name_t;
                        self.validate_endless_method_name(&name_t)?;

                        let rescue_body = self.builder.rescue_body(
                             Token::from(yystack.owned_value_at(1)),
                            None,
                            None,
                            None,
                            None,
                            Some( BoxedNode::from(yystack.owned_value_at(0)))
                        );

                        let method_body = self.builder.begin_body(
                            Some( BoxedNode::from(yystack.owned_value_at(2))),
                            vec![ *rescue_body ],
                            None,
                            None,
                        );

                        yyval = Value::Node(
                            self.builder.def_endless_method(
                                def_t,
                                name_t,
                                 MaybeBoxedNode::from(yystack.owned_value_at(4)),
                                 Token::from(yystack.owned_value_at(3)),
                                method_body
                            )?
                        );

                        self.local_pop();
                        self.current_arg_stack.pop();
                        self.context.set_in_def(ctx.in_def());
                    },


  259 =>  /* arg: defs_head f_opt_paren_args "=" arg  */
  /* "src/parser/parse.y":2400  */
                    {
                        let DefsHead { def_t, definee, dot_t, name_t } =  DefsHead::from(yystack.owned_value_at(3));
                        let TokenWithContext { token: name_t, ctx } = name_t;
                        self.validate_endless_method_name(&name_t)?;

                        yyval = Value::Node(
                            self.builder.def_endless_singleton(
                                def_t,
                                definee,
                                dot_t,
                                name_t,
                                 MaybeBoxedNode::from(yystack.owned_value_at(2)),
                                 Token::from(yystack.owned_value_at(1)),
                                Some( BoxedNode::from(yystack.owned_value_at(0)))
                            )?
                        );

                        self.local_pop();
                        self.current_arg_stack.pop();
                        self.context.set_in_def(ctx.in_def());
                    },


  260 =>  /* arg: defs_head f_opt_paren_args "=" arg "`rescue' modifier" arg  */
  /* "src/parser/parse.y":2422  */
                    {
                        let DefsHead { def_t, definee, dot_t, name_t } =  DefsHead::from(yystack.owned_value_at(5));
                        let TokenWithContext { token: name_t, ctx } = name_t;
                        self.validate_endless_method_name(&name_t)?;

                        let rescue_body = self.builder.rescue_body(
                             Token::from(yystack.owned_value_at(1)),
                            None,
                            None,
                            None,
                            None,
                            Some( BoxedNode::from(yystack.owned_value_at(0)))
                        );

                        let method_body = self.builder.begin_body(
                            Some( BoxedNode::from(yystack.owned_value_at(2))),
                            vec![ *rescue_body ],
                            None,
                            None,
                        );

                        yyval = Value::Node(
                            self.builder.def_endless_singleton(
                                def_t,
                                definee,
                                dot_t,
                                name_t,
                                 MaybeBoxedNode::from(yystack.owned_value_at(4)),
                                 Token::from(yystack.owned_value_at(3)),
                                method_body
                            )?
                        );

                        self.local_pop();
                        self.current_arg_stack.pop();
                        self.context.set_in_def(ctx.in_def());
                    },


  261 =>  /* arg: primary  */
  /* "src/parser/parse.y":2460  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  262 =>  /* relop: ">"  */
  /* "src/parser/parse.y":2466  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  263 =>  /* relop: "<"  */
  /* "src/parser/parse.y":2470  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  264 =>  /* relop: ">="  */
  /* "src/parser/parse.y":2474  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  265 =>  /* relop: "<="  */
  /* "src/parser/parse.y":2478  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  266 =>  /* rel_expr: arg relop arg  */
  /* "src/parser/parse.y":2484  */
                    {
                        yyval = Value::Node(
                            self.builder.binary_op(
                                 BoxedNode::from(yystack.owned_value_at(2)),
                                 Token::from(yystack.owned_value_at(1)),
                                 BoxedNode::from(yystack.owned_value_at(0))
                            )?
                        );
                    },


  267 =>  /* rel_expr: rel_expr relop arg  */
  /* "src/parser/parse.y":2494  */
                    {
                        let op_t =  Token::from(yystack.owned_value_at(1));
                        self.warn(
                            yystack.location_at (1),
                            DiagnosticMessage::ComparisonAfterComparison { comparison: clone_value(&op_t) }
                        );
                        yyval = Value::Node(
                            self.builder.binary_op(
                                 BoxedNode::from(yystack.owned_value_at(2)),
                                op_t,
                                 BoxedNode::from(yystack.owned_value_at(0))
                            )?
                        );
                    },


  268 =>  /* arg_value: arg  */
  /* "src/parser/parse.y":2511  */
                    {
                        let arg =  BoxedNode::from(yystack.owned_value_at(0));
                        self.value_expr(&arg)?;
                        yyval = Value::Node(arg);
                    },


  269 =>  /* aref_args: none  */
  /* "src/parser/parse.y":2519  */
                    {
                        yyval = Value::NodeList( Box::new( vec![] ) );
                    },


  270 =>  /* aref_args: args trailer  */
  /* "src/parser/parse.y":2523  */
                    {
                        yyval =  yystack.owned_value_at(1);
                    },


  271 =>  /* aref_args: args "," assocs trailer  */
  /* "src/parser/parse.y":2527  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(3));
                        nodes.push(
                            *self.builder.associate(
                                None,
                                 NodeList::from(yystack.owned_value_at(1)),
                                None
                            )
                        );
                        yyval = Value::NodeList(nodes);
                    },


  272 =>  /* aref_args: assocs trailer  */
  /* "src/parser/parse.y":2539  */
                    {
                        yyval = Value::NodeList(
                            Box::new(
                                vec![
                                    *self.builder.associate(
                                        None,
                                         NodeList::from(yystack.owned_value_at(1)),
                                        None
                                    )
                                ]
                            )
                        );
                    },


  273 =>  /* arg_rhs: arg  */
  /* "src/parser/parse.y":2555  */
                    {
                        let arg =  BoxedNode::from(yystack.owned_value_at(0));
                        self.value_expr(&arg)?;
                        yyval = Value::Node(arg);
                    },


  274 =>  /* arg_rhs: arg "`rescue' modifier" arg  */
  /* "src/parser/parse.y":2561  */
                    {
                        let arg =  BoxedNode::from(yystack.owned_value_at(2));
                        self.value_expr(&arg)?;

                        let rescue_body = self.builder.rescue_body(
                             Token::from(yystack.owned_value_at(1)),
                            None,
                            None,
                            None,
                            None,
                            Some( BoxedNode::from(yystack.owned_value_at(0)))
                        );

                        yyval = Value::Node(
                            self.builder.begin_body(
                                Some(arg),
                                vec![ *rescue_body ],
                                None,
                                None,
                            ).expect("expected begin_body to return Some (compound_stmt was given)")
                        );
                    },


  275 =>  /* paren_args: "( (tLPAREN2)" opt_call_args rparen  */
  /* "src/parser/parse.y":2586  */
                    {
                        yyval = Value::new_paren_args(
                            ParenArgs {
                                begin_t:  Token::from(yystack.owned_value_at(2)),
                                args:  NodeList::from(yystack.owned_value_at(1)),
                                end_t:  Token::from(yystack.owned_value_at(0))
                            }
                        );
                    },


  276 =>  /* paren_args: "( (tLPAREN2)" args "," args_forward rparen  */
  /* "src/parser/parse.y":2596  */
                    {
                        if !self.static_env.is_forward_args_declared() {
                            return self.yyerror(
                                yystack.location_at (1),
                                DiagnosticMessage::UnexpectedToken { token_name: "tBDOT3".to_string() }
                            );
                        }

                        let mut args =  NodeList::from(yystack.owned_value_at(3));
                        let forwarded_args = *self.builder.forwarded_args( Token::from(yystack.owned_value_at(1)));
                        args.push(forwarded_args);

                        yyval = Value::new_paren_args(
                            ParenArgs {
                                begin_t:  Token::from(yystack.owned_value_at(4)),
                                args,
                                end_t:  Token::from(yystack.owned_value_at(0))
                            }
                        );
                    },


  277 =>  /* paren_args: "( (tLPAREN2)" args_forward rparen  */
  /* "src/parser/parse.y":2617  */
                    {
                        if !self.static_env.is_forward_args_declared() {
                            return self.yyerror(yystack.location_at (1), DiagnosticMessage::UnexpectedToken { token_name: "tBDOT3".to_string() });
                        }

                        yyval = Value::new_paren_args(
                            ParenArgs {
                                begin_t:  Token::from(yystack.owned_value_at(2)),
                                args: vec![ *self.builder.forwarded_args( Token::from(yystack.owned_value_at(1))) ],
                                end_t:  Token::from(yystack.owned_value_at(0))
                            }
                        );
                    },


  278 =>  /* opt_paren_args: none  */
  /* "src/parser/parse.y":2633  */
                    {
                        yyval = Value::new_opt_paren_args(
                            OptParenArgs {
                                begin_t: None,
                                args: vec![],
                                end_t: None
                            }
                        );
                    },


  279 =>  /* opt_paren_args: paren_args  */
  /* "src/parser/parse.y":2643  */
                    {
                        let ParenArgs { begin_t, args, end_t } =  ParenArgs::from(yystack.owned_value_at(0));
                        yyval = Value::new_opt_paren_args(
                            OptParenArgs {
                                begin_t: Some(begin_t),
                                args,
                                end_t: Some(end_t)
                            }
                        );
                    },


  280 =>  /* opt_call_args: none  */
  /* "src/parser/parse.y":2656  */
                    {
                        yyval = Value::NodeList( Box::new(vec![]) );
                    },


  281 =>  /* opt_call_args: call_args  */
  /* "src/parser/parse.y":2660  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  282 =>  /* opt_call_args: args ","  */
  /* "src/parser/parse.y":2664  */
                    {
                        yyval =  yystack.owned_value_at(1);
                    },


  283 =>  /* opt_call_args: args "," assocs ","  */
  /* "src/parser/parse.y":2668  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(3));
                        let pair = *self.builder.associate(None,  NodeList::from(yystack.owned_value_at(1)), None);
                        nodes.push(pair);
                        yyval = Value::NodeList(nodes);
                    },


  284 =>  /* opt_call_args: assocs ","  */
  /* "src/parser/parse.y":2675  */
                    {
                        yyval = Value::NodeList(
                            Box::new(
                                vec![
                                    *self.builder.associate(
                                        None,
                                         NodeList::from(yystack.owned_value_at(1)),
                                        None
                                    )
                                ]
                            )
                        );
                    },


  285 =>  /* call_args: command  */
  /* "src/parser/parse.y":2691  */
                    {
                        let command =  Node::from(yystack.owned_value_at(0));
                        self.value_expr(&command)?;
                        yyval = Value::NodeList( Box::new(vec![ command ]) );
                    },


  286 =>  /* call_args: args opt_block_arg  */
  /* "src/parser/parse.y":2697  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(1));
                        nodes.append(&mut  NodeList::from(yystack.owned_value_at(0)));

                        yyval = Value::NodeList(nodes);
                    },


  287 =>  /* call_args: assocs opt_block_arg  */
  /* "src/parser/parse.y":2704  */
                    {
                        let hash = *self.builder.associate(None,  NodeList::from(yystack.owned_value_at(1)), None);
                        let mut opt_block_arg =  NodeList::from(yystack.owned_value_at(0));

                        let mut nodes = Box::new(Vec::with_capacity(1 + opt_block_arg.len()));
                        nodes.push(hash);
                        nodes.append(&mut opt_block_arg);

                        yyval = Value::NodeList(nodes);
                    },


  288 =>  /* call_args: args "," assocs opt_block_arg  */
  /* "src/parser/parse.y":2715  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(3));
                        let hash = *self.builder.associate(None,  NodeList::from(yystack.owned_value_at(1)), None);
                        let mut opt_block_arg =  NodeList::from(yystack.owned_value_at(0));

                        nodes.reserve(1 + opt_block_arg.len());
                        nodes.push(hash);
                        nodes.append(&mut opt_block_arg);

                        yyval = Value::NodeList(nodes);
                    },


  289 =>  /* call_args: block_arg  */
  /* "src/parser/parse.y":2727  */
                    {
                        yyval = Value::NodeList( Box::new(vec![  Node::from(yystack.owned_value_at(0)) ]) );
                    },


  290 =>  /* @11: %empty  */
  /* "src/parser/parse.y":2732  */
                    {
                        let lookahead =
                            matches!(
                                self.last_token_type,
                                Lexer::tLPAREN2
                                    | Lexer::tLPAREN
                                    | Lexer:: tLPAREN_ARG
                                    | Lexer::tLBRACK2
                                    | Lexer::tLBRACK
                            );

                        if lookahead { self.yylexer.cmdarg.pop() }
                        self.yylexer.cmdarg.push(true);
                        if lookahead { self.yylexer.cmdarg.push(false) }
                        yyval = Value::None;
                    },


  291 =>  /* command_args: @11 call_args  */
  /* "src/parser/parse.y":2749  */
                    {
                        let lookahead = matches!(self.last_token_type, Lexer::tLBRACE_ARG);

                        if lookahead { self.yylexer.cmdarg.pop() }
                        self.yylexer.cmdarg.pop();
                        if lookahead { self.yylexer.cmdarg.push(false) }

                        yyval =  yystack.owned_value_at(0);
                    },


  292 =>  /* block_arg: "&" arg_value  */
  /* "src/parser/parse.y":2761  */
                    {
                        yyval = Value::Node(
                            self.builder.block_pass(
                                 Token::from(yystack.owned_value_at(1)),
                                Some( BoxedNode::from(yystack.owned_value_at(0)))
                            )
                        );
                    },


  293 =>  /* block_arg: "&"  */
  /* "src/parser/parse.y":2770  */
                    {
                        if !self.static_env.is_anonymous_blockarg_declared() {
                            return self.yyerror(yystack.location_at (0), DiagnosticMessage::NoAnonymousBlockarg {});
                        }

                        yyval = Value::Node(
                            self.builder.block_pass(
                                 Token::from(yystack.owned_value_at(0)),
                                None,
                            )
                        );
                    },


  294 =>  /* opt_block_arg: "," block_arg  */
  /* "src/parser/parse.y":2785  */
                    {
                        yyval = Value::NodeList( Box::new(vec![  Node::from(yystack.owned_value_at(0)) ]) );
                    },


  295 =>  /* opt_block_arg: none  */
  /* "src/parser/parse.y":2789  */
                    {
                        yyval = Value::NodeList( Box::new(vec![]) );
                    },


  296 =>  /* args: arg_value  */
  /* "src/parser/parse.y":2795  */
                    {
                        yyval = Value::NodeList( Box::new(vec![  Node::from(yystack.owned_value_at(0)) ]) );
                    },


  297 =>  /* args: "*" arg_value  */
  /* "src/parser/parse.y":2799  */
                    {
                        yyval = Value::NodeList(
                            Box::new(
                                vec![
                                    *self.builder.splat(
                                         Token::from(yystack.owned_value_at(1)),
                                        Some( BoxedNode::from(yystack.owned_value_at(0)))
                                    )
                                ]
                            )
                        );
                    },


  298 =>  /* args: args "," arg_value  */
  /* "src/parser/parse.y":2812  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(2));
                        nodes.push(  Node::from(yystack.owned_value_at(0)) );
                        yyval = Value::NodeList(nodes);
                    },


  299 =>  /* args: args "," "*" arg_value  */
  /* "src/parser/parse.y":2818  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(3));
                        let splat = *self.builder.splat( Token::from(yystack.owned_value_at(1)), Some( BoxedNode::from(yystack.owned_value_at(0))));
                        nodes.push(splat);
                        yyval = Value::NodeList(nodes);
                    },


  300 =>  /* mrhs_arg: mrhs  */
  /* "src/parser/parse.y":2827  */
                    {
                        yyval = Value::Node(
                            self.builder.array(None,  NodeList::from(yystack.owned_value_at(0)), None)
                        );
                    },


  301 =>  /* mrhs_arg: arg_value  */
  /* "src/parser/parse.y":2833  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  302 =>  /* mrhs: args "," arg_value  */
  /* "src/parser/parse.y":2839  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(2));
                        nodes.push(  Node::from(yystack.owned_value_at(0)) );
                        yyval = Value::NodeList(nodes);
                    },


  303 =>  /* mrhs: args "," "*" arg_value  */
  /* "src/parser/parse.y":2845  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(3));
                        nodes.push(
                            *self.builder.splat( Token::from(yystack.owned_value_at(1)), Some( BoxedNode::from(yystack.owned_value_at(0))))
                        );
                        yyval = Value::NodeList(nodes);
                    },


  304 =>  /* mrhs: "*" arg_value  */
  /* "src/parser/parse.y":2853  */
                    {
                        yyval = Value::NodeList(
                            Box::new(
                                vec![
                                    *self.builder.splat(
                                         Token::from(yystack.owned_value_at(1)),
                                        Some( BoxedNode::from(yystack.owned_value_at(0)))
                                    )
                                ]
                            )
                        );
                    },


  305 =>  /* primary: literal  */
  /* "src/parser/parse.y":2868  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  306 =>  /* primary: strings  */
  /* "src/parser/parse.y":2872  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  307 =>  /* primary: xstring  */
  /* "src/parser/parse.y":2876  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  308 =>  /* primary: regexp  */
  /* "src/parser/parse.y":2880  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  309 =>  /* primary: words  */
  /* "src/parser/parse.y":2884  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  310 =>  /* primary: qwords  */
  /* "src/parser/parse.y":2888  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  311 =>  /* primary: symbols  */
  /* "src/parser/parse.y":2892  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  312 =>  /* primary: qsymbols  */
  /* "src/parser/parse.y":2896  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  313 =>  /* primary: var_ref  */
  /* "src/parser/parse.y":2900  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  314 =>  /* primary: backref  */
  /* "src/parser/parse.y":2904  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  315 =>  /* primary: "method"  */
  /* "src/parser/parse.y":2908  */
                    {
                        yyval = Value::Node(
                            self.builder.call_method(
                                None,
                                None,
                                Some( Token::from(yystack.owned_value_at(0))),
                                None,
                                vec![],
                                None
                            )
                        );
                    },


  316 =>  /* @12: %empty  */
  /* "src/parser/parse.y":2921  */
                    {
                        self.yylexer.cmdarg.push(false);
                        yyval = Value::None;
                    },


  317 =>  /* primary: k_begin @12 bodystmt k_end  */
  /* "src/parser/parse.y":2927  */
                    {
                        self.yylexer.cmdarg.pop();

                        yyval = Value::Node(
                            self.builder.begin_keyword( Token::from(yystack.owned_value_at(3)),  MaybeBoxedNode::from(yystack.owned_value_at(1)),  Token::from(yystack.owned_value_at(0)))
                        );
                    },


  318 =>  /* @13: %empty  */
  /* "src/parser/parse.y":2934  */
                              { self.yylexer.lex_state.set(EXPR_ENDARG); yyval = Value::None; },


  319 =>  /* primary: "( arg" @13 rparen  */
  /* "src/parser/parse.y":2935  */
                    {
                        yyval = Value::Node(
                            self.builder.begin(
                                 Token::from(yystack.owned_value_at(2)),
                                None,
                                 Token::from(yystack.owned_value_at(0))
                            )
                        );
                    },


  320 =>  /* @14: %empty  */
  /* "src/parser/parse.y":2944  */
                                   { self.yylexer.lex_state.set(EXPR_ENDARG); yyval = Value::None; },


  321 =>  /* primary: "( arg" stmt @14 rparen  */
  /* "src/parser/parse.y":2945  */
                    {
                        yyval = Value::Node(
                            self.builder.begin(
                                 Token::from(yystack.owned_value_at(3)),
                                Some( BoxedNode::from(yystack.owned_value_at(2))),
                                 Token::from(yystack.owned_value_at(0))
                            )
                        );
                    },


  322 =>  /* primary: "(" compstmt ")"  */
  /* "src/parser/parse.y":2955  */
                    {
                        yyval = Value::Node(
                            self.builder.begin(
                                 Token::from(yystack.owned_value_at(2)),
                                 MaybeBoxedNode::from(yystack.owned_value_at(1)),
                                 Token::from(yystack.owned_value_at(0))
                            )
                        );
                    },


  323 =>  /* primary: primary_value "::" "constant"  */
  /* "src/parser/parse.y":2965  */
                    {
                        yyval = Value::Node(
                            self.builder.const_fetch(
                                 BoxedNode::from(yystack.owned_value_at(2)),
                                 Token::from(yystack.owned_value_at(1)),
                                 Token::from(yystack.owned_value_at(0)),
                            )
                        );
                    },


  324 =>  /* primary: ":: at EXPR_BEG" "constant"  */
  /* "src/parser/parse.y":2975  */
                    {
                        yyval = Value::Node(
                            self.builder.const_global( Token::from(yystack.owned_value_at(1)),  Token::from(yystack.owned_value_at(0)))
                        );
                    },


  325 =>  /* primary: "[" aref_args "]"  */
  /* "src/parser/parse.y":2981  */
                    {
                        yyval = Value::Node(
                            self.builder.array(
                                Some( Token::from(yystack.owned_value_at(2))),
                                 NodeList::from(yystack.owned_value_at(1)),
                                Some( Token::from(yystack.owned_value_at(0)))
                            )
                        );
                    },


  326 =>  /* primary: "{" assoc_list "}"  */
  /* "src/parser/parse.y":2991  */
                    {
                        yyval = Value::Node(
                            self.builder.associate(
                                Some( Token::from(yystack.owned_value_at(2))),
                                 NodeList::from(yystack.owned_value_at(1)),
                                Some( Token::from(yystack.owned_value_at(0)))
                            )
                        );
                    },


  327 =>  /* primary: k_return  */
  /* "src/parser/parse.y":3001  */
                    {
                        yyval = Value::Node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Return,
                                 Token::from(yystack.owned_value_at(0)),
                                None,
                                vec![],
                                None
                            )?
                        );
                    },


  328 =>  /* primary: "`yield'" "( (tLPAREN2)" call_args rparen  */
  /* "src/parser/parse.y":3013  */
                    {
                        yyval = Value::Node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Yield,
                                 Token::from(yystack.owned_value_at(3)),
                                Some( Token::from(yystack.owned_value_at(2))),
                                 NodeList::from(yystack.owned_value_at(1)),
                                Some( Token::from(yystack.owned_value_at(0)))
                            )?
                        );
                    },


  329 =>  /* primary: "`yield'" "( (tLPAREN2)" rparen  */
  /* "src/parser/parse.y":3025  */
                    {
                        yyval = Value::Node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Yield,
                                 Token::from(yystack.owned_value_at(2)),
                                Some( Token::from(yystack.owned_value_at(1))),
                                vec![],
                                Some( Token::from(yystack.owned_value_at(0)))
                            )?
                        );
                    },


  330 =>  /* primary: "`yield'"  */
  /* "src/parser/parse.y":3037  */
                    {
                        yyval = Value::Node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Yield,
                                 Token::from(yystack.owned_value_at(0)),
                                None,
                                vec![],
                                None
                            )?
                        );
                    },


  331 =>  /* @15: %empty  */
  /* "src/parser/parse.y":3049  */
                    {
                        self.context.set_in_defined(true);
                        yyval = Value::None;
                    },


  332 =>  /* primary: "`defined?'" opt_nl "( (tLPAREN2)" @15 expr rparen  */
  /* "src/parser/parse.y":3054  */
                    {
                        self.context.set_in_defined(false);
                        yyval = Value::Node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Defined,
                                 Token::from(yystack.owned_value_at(5)),
                                Some( Token::from(yystack.owned_value_at(3))),
                                vec![  Node::from(yystack.owned_value_at(1)) ],
                                Some( Token::from(yystack.owned_value_at(0)))
                            )?
                        );
                    },


  333 =>  /* primary: "`not'" "( (tLPAREN2)" expr rparen  */
  /* "src/parser/parse.y":3067  */
                    {
                        yyval = Value::Node(
                            self.builder.not_op(
                                 Token::from(yystack.owned_value_at(3)),
                                Some( Token::from(yystack.owned_value_at(2))),
                                Some( BoxedNode::from(yystack.owned_value_at(1))),
                                Some( Token::from(yystack.owned_value_at(0)))
                            )?
                        );
                    },


  334 =>  /* primary: "`not'" "( (tLPAREN2)" rparen  */
  /* "src/parser/parse.y":3078  */
                    {
                        yyval = Value::Node(
                            self.builder.not_op(
                                 Token::from(yystack.owned_value_at(2)),
                                Some( Token::from(yystack.owned_value_at(1))),
                                None,
                                Some( Token::from(yystack.owned_value_at(0)))
                            )?
                        );
                    },


  335 =>  /* primary: fcall brace_block  */
  /* "src/parser/parse.y":3089  */
                    {
                        let method_call = self.builder.call_method(
                            None,
                            None,
                            Some( Token::from(yystack.owned_value_at(1))),
                            None,
                            vec![],
                            None
                        );
                        let BraceBlock { begin_t, args_type, body, end_t } =  BraceBlock::from(yystack.owned_value_at(0));

                        yyval = Value::Node(
                            self.builder.block(
                                method_call,
                                begin_t,
                                args_type,
                                body,
                                end_t
                            )?
                        );
                    },


  336 =>  /* primary: method_call  */
  /* "src/parser/parse.y":3111  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  337 =>  /* primary: method_call brace_block  */
  /* "src/parser/parse.y":3115  */
                    {
                        let BraceBlock { begin_t, args_type, body, end_t } =  BraceBlock::from(yystack.owned_value_at(0));
                        yyval = Value::Node(
                            self.builder.block(
                                 BoxedNode::from(yystack.owned_value_at(1)),
                                begin_t,
                                args_type,
                                body,
                                end_t
                            )?
                        );
                    },


  338 =>  /* primary: lambda  */
  /* "src/parser/parse.y":3128  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  339 =>  /* primary: k_if expr_value then compstmt if_tail k_end  */
  /* "src/parser/parse.y":3135  */
                    {
                        let IfTail { keyword_t, body: else_body } =  IfTail::from(yystack.owned_value_at(1));

                        yyval = Value::Node(
                            self.builder.condition(
                                 Token::from(yystack.owned_value_at(5)),
                                 BoxedNode::from(yystack.owned_value_at(4)),
                                 Token::from(yystack.owned_value_at(3)),
                                 MaybeBoxedNode::from(yystack.owned_value_at(2)),
                                keyword_t,
                                else_body,
                                Some( Token::from(yystack.owned_value_at(0)))
                            )
                        );
                    },


  340 =>  /* primary: k_unless expr_value then compstmt opt_else k_end  */
  /* "src/parser/parse.y":3154  */
                    {
                        let (else_t, body) =  OptElse::from(yystack.owned_value_at(1)).map(|else_| (Some(else_.else_t), else_.body)).unwrap_or_else(|| (None, None));

                        yyval = Value::Node(
                            self.builder.condition(
                                 Token::from(yystack.owned_value_at(5)),
                                 BoxedNode::from(yystack.owned_value_at(4)),
                                 Token::from(yystack.owned_value_at(3)),
                                body,
                                else_t,
                                 MaybeBoxedNode::from(yystack.owned_value_at(2)),
                                Some( Token::from(yystack.owned_value_at(0)))
                            )
                        );
                    },


  341 =>  /* primary: k_while expr_value_do compstmt k_end  */
  /* "src/parser/parse.y":3172  */
                    {
                        let ExprValueDo { value, do_t } =  ExprValueDo::from(yystack.owned_value_at(2));
                        yyval = Value::Node(
                            self.builder.loop_(
                                LoopType::While,
                                 Token::from(yystack.owned_value_at(3)),
                                value,
                                do_t,
                                 MaybeBoxedNode::from(yystack.owned_value_at(1)),
                                 Token::from(yystack.owned_value_at(0))
                            )
                        );
                    },


  342 =>  /* primary: k_until expr_value_do compstmt k_end  */
  /* "src/parser/parse.y":3188  */
                    {
                        let ExprValueDo { value, do_t } =  ExprValueDo::from(yystack.owned_value_at(2));
                        yyval = Value::Node(
                            self.builder.loop_(
                                LoopType::Until,
                                 Token::from(yystack.owned_value_at(3)),
                                value,
                                do_t,
                                 MaybeBoxedNode::from(yystack.owned_value_at(1)),
                                 Token::from(yystack.owned_value_at(0))
                            )
                        );
                    },


  343 =>  /* @16: %empty  */
  /* "src/parser/parse.y":3202  */
                    {
                        // TODO: there's a warning that wq/parser doesn't trigger,
                        // search for `p->case_labels`
                        yyval = Value::None;
                    },


  344 =>  /* primary: k_case expr_value opt_terms @16 case_body k_end  */
  /* "src/parser/parse.y":3209  */
                    {
                        let CaseBody { when_bodies, opt_else } =  CaseBody::from(yystack.owned_value_at(1));
                        let (else_t, else_body) = opt_else.map(|else_| (Some(else_.else_t), else_.body)).unwrap_or_else(|| (None, None));

                        yyval = Value::Node(
                            self.builder.case(
                                 Token::from(yystack.owned_value_at(5)),
                                Some( BoxedNode::from(yystack.owned_value_at(4))),
                                when_bodies,
                                else_t,
                                else_body,
                                 Token::from(yystack.owned_value_at(0))
                            )
                        );
                    },


  345 =>  /* @17: %empty  */
  /* "src/parser/parse.y":3225  */
                    {
                        // TODO: there's a warning that wq/parser doesn't trigger,
                        // search for `p->case_labels`
                        yyval = Value::None;
                    },


  346 =>  /* primary: k_case opt_terms @17 case_body k_end  */
  /* "src/parser/parse.y":3232  */
                    {
                        let CaseBody { when_bodies, opt_else } =  CaseBody::from(yystack.owned_value_at(1));
                        let (else_t, else_body) = opt_else.map(|else_| (Some(else_.else_t), else_.body)).unwrap_or_else(|| (None, None));

                        yyval = Value::Node(
                            self.builder.case(
                                 Token::from(yystack.owned_value_at(4)),
                                None,
                                when_bodies,
                                else_t,
                                else_body,
                                 Token::from(yystack.owned_value_at(0))
                            )
                        );
                    },


  347 =>  /* primary: k_case expr_value opt_terms p_case_body k_end  */
  /* "src/parser/parse.y":3250  */
                    {
                        let PCaseBody { in_bodies, opt_else } =  PCaseBody::from(yystack.owned_value_at(1));
                        let (else_t, else_body) = opt_else.map(|else_| (Some(else_.else_t), else_.body)).unwrap_or_else(|| (None, None));

                        yyval = Value::Node(
                            self.builder.case_match(
                                 Token::from(yystack.owned_value_at(4)),
                                 BoxedNode::from(yystack.owned_value_at(3)),
                                in_bodies,
                                else_t,
                                else_body,
                                 Token::from(yystack.owned_value_at(0))
                            )
                        );
                    },


  348 =>  /* primary: k_for for_var "`in'" expr_value_do compstmt k_end  */
  /* "src/parser/parse.y":3268  */
                    {
                        let ExprValueDo { value, do_t } =  ExprValueDo::from(yystack.owned_value_at(2));
                        yyval = Value::Node(
                            self.builder.for_(
                                 Token::from(yystack.owned_value_at(5)),
                                 BoxedNode::from(yystack.owned_value_at(4)),
                                 Token::from(yystack.owned_value_at(3)),
                                value,
                                do_t,
                                 MaybeBoxedNode::from(yystack.owned_value_at(1)),
                                 Token::from(yystack.owned_value_at(0))
                            )
                        );
                    },


  349 =>  /* @18: %empty  */
  /* "src/parser/parse.y":3283  */
                    {
                        self.local_push();
                        self.context.set_in_class(true);
                        yyval = Value::None;
                    },


  350 =>  /* primary: k_class cpath superclass @18 bodystmt k_end  */
  /* "src/parser/parse.y":3290  */
                    {
                        let TokenWithContext { token: k_class, ctx } =  TokenWithContext::from(yystack.owned_value_at(5));
                        if self.context.in_def() {
                            return self.yyerror(&k_class.loc, DiagnosticMessage::ClassDefinitionInMethodBody {});
                        }

                        let Superclass { lt_t, value } =  Superclass::from(yystack.owned_value_at(3));

                        yyval = Value::Node(
                            self.builder.def_class(
                                k_class,
                                 BoxedNode::from(yystack.owned_value_at(4)),
                                lt_t,
                                value,
                                 MaybeBoxedNode::from(yystack.owned_value_at(1)),
                                 Token::from(yystack.owned_value_at(0))
                            )
                        );

                        self.local_pop();
                        self.context.set_in_class(ctx.in_class());
                    },


  351 =>  /* @19: %empty  */
  /* "src/parser/parse.y":3313  */
                    {
                        self.context.set_in_def(false);
                        self.context.set_in_class(false);
                        self.local_push();
                        yyval = Value::None;
                    },


  352 =>  /* primary: k_class "<<" expr @19 term bodystmt k_end  */
  /* "src/parser/parse.y":3322  */
                    {
                        let TokenWithContext { token: k_class, ctx } =  TokenWithContext::from(yystack.owned_value_at(6));
                        yyval = Value::Node(
                            self.builder.def_sclass(
                                k_class,
                                 Token::from(yystack.owned_value_at(5)),
                                 BoxedNode::from(yystack.owned_value_at(4)),
                                 MaybeBoxedNode::from(yystack.owned_value_at(1)),
                                 Token::from(yystack.owned_value_at(0))
                            )
                        );

                        self.local_pop();
                        self.context.set_in_def(ctx.in_def());
                        self.context.set_in_class(ctx.in_class());
                    },


  353 =>  /* @20: %empty  */
  /* "src/parser/parse.y":3339  */
                    {
                        self.local_push();
                        self.context.set_in_class(true);
                        yyval = Value::None;
                    },


  354 =>  /* primary: k_module cpath @20 bodystmt k_end  */
  /* "src/parser/parse.y":3346  */
                    {
                        let TokenWithContext { token: k_module, ctx } =  TokenWithContext::from(yystack.owned_value_at(4));
                        if self.context.in_def() {
                            return self.yyerror(&k_module.loc, DiagnosticMessage::ModuleDefinitionInMethodBody {});
                        }

                        yyval = Value::Node(
                            self.builder.def_module(
                                k_module,
                                 BoxedNode::from(yystack.owned_value_at(3)),
                                 MaybeBoxedNode::from(yystack.owned_value_at(1)),
                                 Token::from(yystack.owned_value_at(0))
                            )
                        );

                        self.local_pop();
                        self.context.set_in_class(ctx.in_class());
                    },


  355 =>  /* primary: defn_head f_arglist bodystmt k_end  */
  /* "src/parser/parse.y":3365  */
                    {
                        let DefnHead { def_t, name_t } =  DefnHead::from(yystack.owned_value_at(3));
                        let TokenWithContext { token: name_t, ctx } = name_t;

                        yyval = Value::Node(
                            self.builder.def_method(
                                def_t,
                                name_t,
                                 MaybeBoxedNode::from(yystack.owned_value_at(2)),
                                 MaybeBoxedNode::from(yystack.owned_value_at(1)),
                                 Token::from(yystack.owned_value_at(0))
                            )?
                        );

                        self.local_pop();
                        self.current_arg_stack.pop();
                        self.context.set_in_def(ctx.in_def());
                    },


  356 =>  /* primary: defs_head f_arglist bodystmt k_end  */
  /* "src/parser/parse.y":3384  */
                    {
                        let DefsHead { def_t, definee, dot_t, name_t } =  DefsHead::from(yystack.owned_value_at(3));
                        let TokenWithContext { token: name_t, ctx } = name_t;

                        yyval = Value::Node(
                            self.builder.def_singleton(
                                def_t,
                                definee,
                                dot_t,
                                name_t,
                                 MaybeBoxedNode::from(yystack.owned_value_at(2)),
                                 MaybeBoxedNode::from(yystack.owned_value_at(1)),
                                 Token::from(yystack.owned_value_at(0))
                            )?
                        );

                        self.local_pop();
                        self.current_arg_stack.pop();
                        self.context.set_in_def(ctx.in_def());
                    },


  357 =>  /* primary: "`break'"  */
  /* "src/parser/parse.y":3405  */
                    {
                        yyval = Value::Node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Break,
                                 Token::from(yystack.owned_value_at(0)),
                                None,
                                vec![],
                                None
                            )?
                        );
                    },


  358 =>  /* primary: "`next'"  */
  /* "src/parser/parse.y":3417  */
                    {
                        yyval = Value::Node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Next,
                                 Token::from(yystack.owned_value_at(0)),
                                None,
                                vec![],
                                None
                            )?
                        );
                    },


  359 =>  /* primary: "`redo'"  */
  /* "src/parser/parse.y":3429  */
                    {
                        yyval = Value::Node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Redo,
                                 Token::from(yystack.owned_value_at(0)),
                                None,
                                vec![],
                                None
                            )?
                        );
                    },


  360 =>  /* primary: "`retry'"  */
  /* "src/parser/parse.y":3441  */
                    {
                        yyval = Value::Node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Retry,
                                 Token::from(yystack.owned_value_at(0)),
                                None,
                                vec![],
                                None
                            )?
                        );
                    },


  361 =>  /* primary_value: primary  */
  /* "src/parser/parse.y":3455  */
                    {
                        let primary =  BoxedNode::from(yystack.owned_value_at(0));
                        self.value_expr(&primary)?;
                        yyval = Value::Node(primary);
                    },


  362 =>  /* k_begin: "`begin'"  */
  /* "src/parser/parse.y":3463  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  363 =>  /* k_if: "`if'"  */
  /* "src/parser/parse.y":3469  */
                    {
                        self.warn_eol(yystack.location_at (0), "if");
                        yyval =  yystack.owned_value_at(0);
                    },


  364 =>  /* k_unless: "`unless'"  */
  /* "src/parser/parse.y":3476  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  365 =>  /* k_while: "`while'"  */
  /* "src/parser/parse.y":3482  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  366 =>  /* k_until: "`until'"  */
  /* "src/parser/parse.y":3488  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  367 =>  /* k_case: "`case'"  */
  /* "src/parser/parse.y":3494  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  368 =>  /* k_for: "`for'"  */
  /* "src/parser/parse.y":3500  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  369 =>  /* k_class: "`class'"  */
  /* "src/parser/parse.y":3506  */
                    {
                        yyval = Value::TokenWithContext(
                            Box::new(
                                TokenWithContext {
                                    token:  Token::from(yystack.owned_value_at(0)),
                                    ctx: self.context.dump(),
                                }
                            )
                        );
                    },


  370 =>  /* k_module: "`module'"  */
  /* "src/parser/parse.y":3519  */
                    {
                        yyval = Value::TokenWithContext(
                            Box::new(
                                TokenWithContext {
                                    token:  Token::from(yystack.owned_value_at(0)),
                                    ctx: self.context.dump(),
                                }
                            )
                        );
                    },


  371 =>  /* k_def: "`def'"  */
  /* "src/parser/parse.y":3532  */
                    {
                        yyval =  yystack.owned_value_at(0);
                        self.context.set_in_argdef(true);
                    },


  372 =>  /* k_do: "`do'"  */
  /* "src/parser/parse.y":3539  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  373 =>  /* k_do_block: "`do' for block"  */
  /* "src/parser/parse.y":3545  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  374 =>  /* k_rescue: "`rescue'"  */
  /* "src/parser/parse.y":3551  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  375 =>  /* k_ensure: "`ensure'"  */
  /* "src/parser/parse.y":3557  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  376 =>  /* k_when: "`when'"  */
  /* "src/parser/parse.y":3563  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  377 =>  /* k_else: "`else'"  */
  /* "src/parser/parse.y":3569  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  378 =>  /* k_elsif: "`elsif'"  */
  /* "src/parser/parse.y":3575  */
                    {
                        self.warn_eol(yystack.location_at (0), "elsif");
                        yyval =  yystack.owned_value_at(0);
                    },


  379 =>  /* k_end: "`end'"  */
  /* "src/parser/parse.y":3582  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  380 =>  /* k_return: "`return'"  */
  /* "src/parser/parse.y":3588  */
                    {
                        if self.context.in_class() && !self.context.in_def() && !(self.context.in_block() || self.context.in_lambda()) {
                            return self.yyerror(yystack.location_at (0), DiagnosticMessage::InvalidReturnInClassOrModuleBody {});
                        }
                        yyval =  yystack.owned_value_at(0);
                    },


  381 =>  /* then: term  */
  /* "src/parser/parse.y":3597  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  382 =>  /* then: "`then'"  */
  /* "src/parser/parse.y":3601  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  383 =>  /* then: term "`then'"  */
  /* "src/parser/parse.y":3605  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  384 =>  /* do: term  */
  /* "src/parser/parse.y":3611  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  385 =>  /* do: "`do' for condition"  */
  /* "src/parser/parse.y":3615  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  386 =>  /* if_tail: opt_else  */
  /* "src/parser/parse.y":3621  */
                    {
                        let (keyword_t, body) =  OptElse::from(yystack.owned_value_at(0)).map(|else_| (Some(else_.else_t), else_.body)).unwrap_or_else(|| (None, None));
                        yyval = Value::new_if_tail(IfTail { keyword_t, body });
                    },


  387 =>  /* if_tail: k_elsif expr_value then compstmt if_tail  */
  /* "src/parser/parse.y":3628  */
                    {
                        let IfTail { keyword_t, body: else_body } =  IfTail::from(yystack.owned_value_at(0));

                        let elsif_t =  Token::from(yystack.owned_value_at(4));

                        yyval = Value::new_if_tail(
                            IfTail {
                                keyword_t: Some(elsif_t.clone()),
                                body: Some(
                                    self.builder.condition(
                                        elsif_t,
                                         BoxedNode::from(yystack.owned_value_at(3)),
                                         Token::from(yystack.owned_value_at(2)),
                                         MaybeBoxedNode::from(yystack.owned_value_at(1)),
                                        keyword_t,
                                        else_body,
                                        None
                                    )
                                )
                            }
                        );
                    },


  388 =>  /* opt_else: none  */
  /* "src/parser/parse.y":3653  */
                    {
                        yyval = Value::new_opt_else(None);
                    },


  389 =>  /* opt_else: k_else compstmt  */
  /* "src/parser/parse.y":3657  */
                    {
                        let else_t =  Token::from(yystack.owned_value_at(1));
                        let body   =  MaybeBoxedNode::from(yystack.owned_value_at(0));
                        yyval = Value::new_opt_else(Some(Else { else_t, body }));
                    },


  390 =>  /* for_var: lhs  */
  /* "src/parser/parse.y":3665  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  391 =>  /* for_var: mlhs  */
  /* "src/parser/parse.y":3669  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  392 =>  /* f_marg: f_norm_arg  */
  /* "src/parser/parse.y":3675  */
                    {
                        yyval = Value::Node(
                            self.builder.arg( Token::from(yystack.owned_value_at(0)))?
                        );
                    },


  393 =>  /* f_marg: "(" f_margs rparen  */
  /* "src/parser/parse.y":3681  */
                    {
                        yyval = Value::Node(
                            self.builder.multi_lhs(
                                Some( Token::from(yystack.owned_value_at(2))),
                                 NodeList::from(yystack.owned_value_at(1)),
                                Some( Token::from(yystack.owned_value_at(0)))
                            )
                        );
                    },


  394 =>  /* f_marg_list: f_marg  */
  /* "src/parser/parse.y":3693  */
                    {
                        yyval = Value::NodeList( Box::new(vec![  Node::from(yystack.owned_value_at(0)) ]) );
                    },


  395 =>  /* f_marg_list: f_marg_list "," f_marg  */
  /* "src/parser/parse.y":3697  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(2));
                        nodes.push(  Node::from(yystack.owned_value_at(0)) );
                        yyval = Value::NodeList(nodes);
                    },


  396 =>  /* f_margs: f_marg_list  */
  /* "src/parser/parse.y":3705  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  397 =>  /* f_margs: f_marg_list "," f_rest_marg  */
  /* "src/parser/parse.y":3709  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(2));
                        nodes.push(  Node::from(yystack.owned_value_at(0)) );
                        yyval = Value::NodeList(nodes);
                    },


  398 =>  /* f_margs: f_marg_list "," f_rest_marg "," f_marg_list  */
  /* "src/parser/parse.y":3715  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(4));
                        let f_rest_marg =  Node::from(yystack.owned_value_at(2));
                        let mut f_marg_list =  NodeList::from(yystack.owned_value_at(0));

                        nodes.reserve(1 + f_marg_list.len());
                        nodes.push(f_rest_marg);
                        nodes.append(&mut f_marg_list);

                        yyval = Value::NodeList(nodes);
                    },


  399 =>  /* f_margs: f_rest_marg  */
  /* "src/parser/parse.y":3727  */
                    {
                        yyval = Value::NodeList( Box::new(vec![  Node::from(yystack.owned_value_at(0)) ]) );
                    },


  400 =>  /* f_margs: f_rest_marg "," f_marg_list  */
  /* "src/parser/parse.y":3731  */
                    {
                        let f_rest_marg =  Node::from(yystack.owned_value_at(2));
                        let mut f_marg_list =  NodeList::from(yystack.owned_value_at(0));

                        let mut nodes = Box::new( Vec::with_capacity(1 + f_marg_list.len()) );
                        nodes.push(f_rest_marg);
                        nodes.append(&mut f_marg_list);

                        yyval = Value::NodeList(nodes);
                    },


  401 =>  /* f_rest_marg: "*" f_norm_arg  */
  /* "src/parser/parse.y":3744  */
                    {
                        yyval = Value::Node(
                            self.builder.restarg( Token::from(yystack.owned_value_at(1)), Some( Token::from(yystack.owned_value_at(0))))?
                        );
                    },


  402 =>  /* f_rest_marg: "*"  */
  /* "src/parser/parse.y":3750  */
                    {
                        yyval = Value::Node(
                            self.builder.restarg( Token::from(yystack.owned_value_at(0)), None)?
                        );
                    },


  403 =>  /* f_any_kwrest: f_kwrest  */
  /* "src/parser/parse.y":3758  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  404 =>  /* f_any_kwrest: f_no_kwarg  */
  /* "src/parser/parse.y":3762  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  405 =>  /* @21: %empty  */
  /* "src/parser/parse.y":3767  */
                    {
                        self.context.set_in_argdef(false);
                        yyval = Value::None;
                    },


  406 =>  /* f_eq: @21 "="  */
  /* "src/parser/parse.y":3772  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  407 =>  /* block_args_tail: f_block_kwarg "," f_kwrest opt_f_block_arg  */
  /* "src/parser/parse.y":3778  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(3));
                        let mut f_kwrest =  NodeList::from(yystack.owned_value_at(1));
                        let mut opt_f_block_arg =  NodeList::from(yystack.owned_value_at(0));

                        nodes.reserve(f_kwrest.len() + opt_f_block_arg.len());
                        nodes.append(&mut f_kwrest);
                        nodes.append(&mut opt_f_block_arg);

                        yyval = Value::NodeList(nodes);
                    },


  408 =>  /* block_args_tail: f_block_kwarg opt_f_block_arg  */
  /* "src/parser/parse.y":3790  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(1));
                        nodes.append(&mut  NodeList::from(yystack.owned_value_at(0)));

                        yyval = Value::NodeList(nodes);
                    },


  409 =>  /* block_args_tail: f_any_kwrest opt_f_block_arg  */
  /* "src/parser/parse.y":3797  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(1));
                        nodes.append(&mut  NodeList::from(yystack.owned_value_at(0)));

                        yyval = Value::NodeList(nodes);
                    },


  410 =>  /* block_args_tail: f_block_arg  */
  /* "src/parser/parse.y":3804  */
                    {
                        yyval = Value::NodeList( Box::new(vec![  Node::from(yystack.owned_value_at(0)) ]) );
                    },


  411 =>  /* opt_block_args_tail: "," block_args_tail  */
  /* "src/parser/parse.y":3811  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  412 =>  /* opt_block_args_tail: %empty  */
  /* "src/parser/parse.y":3815  */
                    {
                        yyval = Value::NodeList( Box::new(vec![]) );
                    },


  413 =>  /* excessed_comma: ","  */
  /* "src/parser/parse.y":3821  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  414 =>  /* block_param: f_arg "," f_block_optarg "," f_rest_arg opt_block_args_tail  */
  /* "src/parser/parse.y":3827  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(5));
                        let mut f_block_optarg =  NodeList::from(yystack.owned_value_at(3));
                        let mut f_rest_arg =  NodeList::from(yystack.owned_value_at(1));
                        let mut opt_block_args_tail =  NodeList::from(yystack.owned_value_at(0));

                        nodes.reserve(f_block_optarg.len() + f_rest_arg.len() + opt_block_args_tail.len());
                        nodes.append(&mut f_block_optarg);
                        nodes.append(&mut f_rest_arg);
                        nodes.append(&mut opt_block_args_tail);

                        yyval = Value::NodeList(nodes);
                    },


  415 =>  /* block_param: f_arg "," f_block_optarg "," f_rest_arg "," f_arg opt_block_args_tail  */
  /* "src/parser/parse.y":3841  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(7));
                        let mut f_block_optarg =  NodeList::from(yystack.owned_value_at(5));
                        let mut f_rest_arg =  NodeList::from(yystack.owned_value_at(3));
                        let mut f_arg =  NodeList::from(yystack.owned_value_at(1));
                        let mut opt_block_args_tail =  NodeList::from(yystack.owned_value_at(0));

                        nodes.reserve(f_block_optarg.len() + f_rest_arg.len() + f_arg.len() + opt_block_args_tail.len());
                        nodes.append(&mut f_block_optarg);
                        nodes.append(&mut f_rest_arg);
                        nodes.append(&mut f_arg);
                        nodes.append(&mut opt_block_args_tail);

                        yyval = Value::NodeList(nodes);
                    },


  416 =>  /* block_param: f_arg "," f_block_optarg opt_block_args_tail  */
  /* "src/parser/parse.y":3857  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(3));
                        let mut f_block_optarg =  NodeList::from(yystack.owned_value_at(1));
                        let mut opt_block_args_tail =  NodeList::from(yystack.owned_value_at(0));

                        nodes.reserve(f_block_optarg.len() + opt_block_args_tail.len());
                        nodes.append(&mut f_block_optarg);
                        nodes.append(&mut opt_block_args_tail);

                        yyval = Value::NodeList(nodes);
                    },


  417 =>  /* block_param: f_arg "," f_block_optarg "," f_arg opt_block_args_tail  */
  /* "src/parser/parse.y":3869  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(5));
                        let mut f_block_optarg =  NodeList::from(yystack.owned_value_at(3));
                        let mut f_arg =  NodeList::from(yystack.owned_value_at(1));
                        let mut opt_block_args_tail =  NodeList::from(yystack.owned_value_at(0));

                        nodes.reserve(f_block_optarg.len() + f_arg.len() + opt_block_args_tail.len());
                        nodes.append(&mut f_block_optarg);
                        nodes.append(&mut f_arg);
                        nodes.append(&mut opt_block_args_tail);

                        yyval = Value::NodeList(nodes);
                    },


  418 =>  /* block_param: f_arg "," f_rest_arg opt_block_args_tail  */
  /* "src/parser/parse.y":3883  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(3));
                        let mut f_rest_arg =  NodeList::from(yystack.owned_value_at(1));
                        let mut opt_block_args_tail =  NodeList::from(yystack.owned_value_at(0));

                        nodes.reserve(f_rest_arg.len() + opt_block_args_tail.len());
                        nodes.append(&mut f_rest_arg);
                        nodes.append(&mut opt_block_args_tail);

                        yyval = Value::NodeList(nodes);
                    },


  419 =>  /* block_param: f_arg excessed_comma  */
  /* "src/parser/parse.y":3895  */
                    {
                        yyval =  yystack.owned_value_at(1);
                    },


  420 =>  /* block_param: f_arg "," f_rest_arg "," f_arg opt_block_args_tail  */
  /* "src/parser/parse.y":3899  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(5));
                        let mut f_rest_arg =  NodeList::from(yystack.owned_value_at(3));
                        let mut f_arg =  NodeList::from(yystack.owned_value_at(1));
                        let mut opt_block_args_tail =  NodeList::from(yystack.owned_value_at(0));

                        nodes.reserve(f_rest_arg.len() + f_arg.len() + opt_block_args_tail.len());
                        nodes.append(&mut f_rest_arg);
                        nodes.append(&mut f_arg);
                        nodes.append(&mut opt_block_args_tail);

                        yyval = Value::NodeList(nodes);
                    },


  421 =>  /* block_param: f_arg opt_block_args_tail  */
  /* "src/parser/parse.y":3913  */
                    {
                        let f_arg =  BoxedNodeList::from(yystack.owned_value_at(1));
                        let mut opt_block_args_tail =  NodeList::from(yystack.owned_value_at(0));
                        let mut nodes;

                        if opt_block_args_tail.is_empty() && f_arg.len() == 1 {
                            let procarg0 = *self.builder.procarg0(
                                Box::new(f_arg.into_iter().next().unwrap())
                            );
                            nodes = Box::new( vec![ procarg0 ] );
                        } else {
                            nodes = f_arg;
                            nodes.append(&mut opt_block_args_tail);
                        }

                        yyval = Value::NodeList(nodes);
                    },


  422 =>  /* block_param: f_block_optarg "," f_rest_arg opt_block_args_tail  */
  /* "src/parser/parse.y":3931  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(3));
                        let mut f_rest_arg =  NodeList::from(yystack.owned_value_at(1));
                        let mut opt_block_args_tail =  NodeList::from(yystack.owned_value_at(0));

                        nodes.reserve(f_rest_arg.len() + opt_block_args_tail.len());
                        nodes.append(&mut f_rest_arg);
                        nodes.append(&mut opt_block_args_tail);

                        yyval = Value::NodeList(nodes);
                    },


  423 =>  /* block_param: f_block_optarg "," f_rest_arg "," f_arg opt_block_args_tail  */
  /* "src/parser/parse.y":3943  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(5));
                        let mut f_rest_arg =  NodeList::from(yystack.owned_value_at(3));
                        let mut f_arg =  NodeList::from(yystack.owned_value_at(1));
                        let mut opt_block_args_tail =  NodeList::from(yystack.owned_value_at(0));

                        nodes.reserve(f_rest_arg.len() + f_arg.len() + opt_block_args_tail.len());
                        nodes.append(&mut f_rest_arg);
                        nodes.append(&mut f_arg);
                        nodes.append(&mut opt_block_args_tail);

                        yyval = Value::NodeList(nodes);
                    },


  424 =>  /* block_param: f_block_optarg opt_block_args_tail  */
  /* "src/parser/parse.y":3957  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(1));
                        nodes.append(&mut  NodeList::from(yystack.owned_value_at(0)));

                        yyval = Value::NodeList(nodes);
                    },


  425 =>  /* block_param: f_block_optarg "," f_arg opt_block_args_tail  */
  /* "src/parser/parse.y":3964  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(3));
                        let mut f_arg =  NodeList::from(yystack.owned_value_at(1));
                        let mut opt_block_args_tail =  NodeList::from(yystack.owned_value_at(0));

                        nodes.reserve(f_arg.len() + opt_block_args_tail.len());
                        nodes.append(&mut f_arg);
                        nodes.append(&mut opt_block_args_tail);

                        yyval = Value::NodeList(nodes);
                    },


  426 =>  /* block_param: f_rest_arg opt_block_args_tail  */
  /* "src/parser/parse.y":3976  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(1));
                        nodes.append(&mut  NodeList::from(yystack.owned_value_at(0)));

                        yyval = Value::NodeList(nodes);
                    },


  427 =>  /* block_param: f_rest_arg "," f_arg opt_block_args_tail  */
  /* "src/parser/parse.y":3983  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(3));
                        let mut f_arg =  NodeList::from(yystack.owned_value_at(1));
                        let mut opt_block_args_tail =  NodeList::from(yystack.owned_value_at(0));

                        nodes.reserve(f_arg.len() + opt_block_args_tail.len());
                        nodes.append(&mut f_arg);
                        nodes.append(&mut opt_block_args_tail);

                        yyval = Value::NodeList(nodes);
                    },


  428 =>  /* block_param: block_args_tail  */
  /* "src/parser/parse.y":3995  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  429 =>  /* opt_block_param: none  */
  /* "src/parser/parse.y":4001  */
                    {
                        yyval = Value::MaybeNode(
                            self.builder.args(None, vec![], None)
                        );
                    },


  430 =>  /* opt_block_param: block_param_def  */
  /* "src/parser/parse.y":4007  */
                    {
                        self.yylexer.command_start = true;
                        yyval =  yystack.owned_value_at(0);
                    },


  431 =>  /* block_param_def: "|" opt_bv_decl "|"  */
  /* "src/parser/parse.y":4014  */
                    {
                        self.max_numparam_stack.set_has_ordinary_params();
                        self.current_arg_stack.set(None);
                        self.context.set_in_argdef(false);

                        yyval = Value::MaybeNode(
                            self.builder.args(
                                Some( Token::from(yystack.owned_value_at(2))),
                                 NodeList::from(yystack.owned_value_at(1)),
                                Some( Token::from(yystack.owned_value_at(0)))
                            )
                        );
                    },


  432 =>  /* block_param_def: "|" block_param opt_bv_decl "|"  */
  /* "src/parser/parse.y":4028  */
                    {
                        self.max_numparam_stack.set_has_ordinary_params();
                        self.current_arg_stack.set(None);
                        self.context.set_in_argdef(false);

                        let mut nodes =  NodeList::from(yystack.owned_value_at(2));
                        nodes.append(&mut  NodeList::from(yystack.owned_value_at(1)));

                        yyval = Value::MaybeNode(
                            self.builder.args(
                                Some( Token::from(yystack.owned_value_at(3))),
                                nodes,
                                Some( Token::from(yystack.owned_value_at(0)))
                            )
                        );
                    },


  433 =>  /* opt_bv_decl: opt_nl  */
  /* "src/parser/parse.y":4048  */
                    {
                        yyval = Value::NodeList( Box::new(vec![]) );
                    },


  434 =>  /* opt_bv_decl: opt_nl ";" bv_decls opt_nl  */
  /* "src/parser/parse.y":4052  */
                    {
                        yyval =  yystack.owned_value_at(1);
                    },


  435 =>  /* bv_decls: bvar  */
  /* "src/parser/parse.y":4058  */
                    {
                        yyval = Value::NodeList( Box::new(vec![  Node::from(yystack.owned_value_at(0)) ]) );
                    },


  436 =>  /* bv_decls: bv_decls "," bvar  */
  /* "src/parser/parse.y":4062  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(2));
                        nodes.push(  Node::from(yystack.owned_value_at(0)) );
                        yyval = Value::NodeList(nodes);
                    },


  437 =>  /* bvar: "local variable or method"  */
  /* "src/parser/parse.y":4070  */
                    {
                        let ident_t =  Token::from(yystack.owned_value_at(0));
                        self.static_env.declare(clone_value(&ident_t).as_str());
                        yyval = Value::Node(
                            self.builder.shadowarg(ident_t)?
                        );
                    },


  438 =>  /* bvar: f_bad_arg  */
  /* "src/parser/parse.y":4078  */
                    {
                        yyval = Value::None;
                    },


  439 =>  /* @22: %empty  */
  /* "src/parser/parse.y":4084  */
                    {
                        self.static_env.extend_dynamic();
                        self.max_numparam_stack.push(false);
                        yyval = Value::Num(self.yylexer.lpar_beg);
                        self.yylexer.lpar_beg = self.yylexer.paren_nest;
                    },


  440 =>  /* @23: %empty  */
  /* "src/parser/parse.y":4090  */
                    {
                        yyval = Value::Context(self.context.dump());
                        self.context.set_in_lambda(true);
                    },


  441 =>  /* @24: %empty  */
  /* "src/parser/parse.y":4095  */
                    {
                        self.yylexer.cmdarg.push(false);
                        yyval = Value::None;
                    },


  442 =>  /* lambda: "->" @22 @23 f_larglist @24 lambda_body  */
  /* "src/parser/parse.y":4100  */
                    {
                        self.yylexer.lpar_beg =  Num::from(yystack.owned_value_at(4));

                        let lambda_call = self.builder.call_lambda( Token::from(yystack.owned_value_at(5)));
                        let args = if self.max_numparam_stack.has_numparams() {
                            ArgsType::Numargs(self.max_numparam_stack.top() as u8)
                        } else {
                            ArgsType::Args( MaybeBoxedNode::from(yystack.owned_value_at(2)))
                        };
                        let LambdaBody { begin_t, body, end_t } =  LambdaBody::from(yystack.owned_value_at(0));

                        self.max_numparam_stack.pop();
                        self.static_env.unextend();
                        self.yylexer.cmdarg.pop();
                        self.context.set_in_lambda( Context::from(yystack.owned_value_at(3)).in_lambda());

                        yyval = Value::Node(
                            self.builder.block(
                                lambda_call,
                                begin_t,
                                args,
                                body,
                                end_t
                            )?
                        );
                    },


  443 =>  /* f_larglist: "( (tLPAREN2)" f_args opt_bv_decl ")"  */
  /* "src/parser/parse.y":4129  */
                    {
                        self.context.set_in_argdef(false);
                        self.max_numparam_stack.set_has_ordinary_params();

                        let mut nodes =  NodeList::from(yystack.owned_value_at(2));
                        nodes.append(&mut  NodeList::from(yystack.owned_value_at(1)));

                        yyval = Value::MaybeNode(
                            self.builder.args(
                                Some( Token::from(yystack.owned_value_at(3))),
                                nodes,
                                Some( Token::from(yystack.owned_value_at(0)))
                            )
                        );
                    },


  444 =>  /* f_larglist: f_args  */
  /* "src/parser/parse.y":4145  */
                    {
                        self.context.set_in_argdef(false);
                        let args =  NodeList::from(yystack.owned_value_at(0));
                        if !args.is_empty() {
                            self.max_numparam_stack.set_has_ordinary_params();
                        }
                        yyval = Value::MaybeNode(
                            self.builder.args(None, args, None)
                        );
                    },


  445 =>  /* @25: %empty  */
  /* "src/parser/parse.y":4158  */
                    {
                        yyval = Value::Context(self.context.dump());
                        self.context.set_in_lambda(true);
                    },


  446 =>  /* lambda_body: tLAMBEG @25 compstmt "}"  */
  /* "src/parser/parse.y":4163  */
                    {
                        self.context.set_in_lambda( Context::from(yystack.owned_value_at(2)).in_lambda());
                        yyval = Value::new_lambda_body(
                            LambdaBody {
                                begin_t:  Token::from(yystack.owned_value_at(3)),
                                body:  MaybeBoxedNode::from(yystack.owned_value_at(1)),
                                end_t:  Token::from(yystack.owned_value_at(0))
                            }
                        );
                    },


  447 =>  /* @26: %empty  */
  /* "src/parser/parse.y":4174  */
                    {
                        yyval = Value::Context(self.context.dump());
                        self.context.set_in_lambda(true);
                    },


  448 =>  /* lambda_body: "`do' for lambda" @26 bodystmt k_end  */
  /* "src/parser/parse.y":4179  */
                    {
                        self.context.set_in_lambda( Context::from(yystack.owned_value_at(2)).in_lambda());
                        yyval = Value::new_lambda_body(
                            LambdaBody {
                                begin_t:  Token::from(yystack.owned_value_at(3)),
                                body:  MaybeBoxedNode::from(yystack.owned_value_at(1)),
                                end_t:  Token::from(yystack.owned_value_at(0))
                            }
                        );
                    },


  449 =>  /* @27: %empty  */
  /* "src/parser/parse.y":4192  */
                    {
                        yyval = Value::Context(self.context.dump());
                        self.context.set_in_block(true);
                    },


  450 =>  /* do_block: k_do_block @27 do_body k_end  */
  /* "src/parser/parse.y":4197  */
                    {
                        self.context.set_in_block( Context::from(yystack.owned_value_at(2)).in_block());
                        let DoBody { args_type, body } =  DoBody::from(yystack.owned_value_at(1));
                        yyval = Value::new_do_block(
                            DoBlock {
                                begin_t:  Token::from(yystack.owned_value_at(3)),
                                args_type,
                                body,
                                end_t:  Token::from(yystack.owned_value_at(0))
                            }
                        );
                    },


  451 =>  /* block_call: command do_block  */
  /* "src/parser/parse.y":4212  */
                    {
                        let DoBlock { begin_t, args_type, body, end_t } =  DoBlock::from(yystack.owned_value_at(0));
                        yyval = Value::Node(
                            self.builder.block(
                                 BoxedNode::from(yystack.owned_value_at(1)),
                                begin_t,
                                args_type,
                                body,
                                end_t
                            )?
                        );
                    },


  452 =>  /* block_call: block_call call_op2 operation2 opt_paren_args  */
  /* "src/parser/parse.y":4225  */
                    {
                        let OptParenArgs { begin_t, args, end_t } =  OptParenArgs::from(yystack.owned_value_at(0));
                        yyval = Value::Node(
                            self.builder.call_method(
                                Some( BoxedNode::from(yystack.owned_value_at(3))),
                                Some( Token::from(yystack.owned_value_at(2))),
                                Some( Token::from(yystack.owned_value_at(1))),
                                begin_t,
                                args,
                                end_t
                            )
                        );
                    },


  453 =>  /* block_call: block_call call_op2 operation2 opt_paren_args brace_block  */
  /* "src/parser/parse.y":4239  */
                    {
                        let OptParenArgs { begin_t, args, end_t } =  OptParenArgs::from(yystack.owned_value_at(1));
                        let method_call = self.builder.call_method(
                            Some( BoxedNode::from(yystack.owned_value_at(4))),
                            Some( Token::from(yystack.owned_value_at(3))),
                            Some( Token::from(yystack.owned_value_at(2))),
                            begin_t,
                            args,
                            end_t
                        );

                        let BraceBlock { begin_t, args_type, body, end_t } =  BraceBlock::from(yystack.owned_value_at(0));
                        yyval = Value::Node(
                            self.builder.block(
                                method_call,
                                begin_t,
                                args_type,
                                body,
                                end_t
                            )?
                        );
                    },


  454 =>  /* block_call: block_call call_op2 operation2 command_args do_block  */
  /* "src/parser/parse.y":4262  */
                    {
                        let method_call = self.builder.call_method(
                            Some( BoxedNode::from(yystack.owned_value_at(4))),
                            Some( Token::from(yystack.owned_value_at(3))),
                            Some( Token::from(yystack.owned_value_at(2))),
                            None,
                             NodeList::from(yystack.owned_value_at(1)),
                            None
                        );

                        let DoBlock { begin_t, args_type, body, end_t } =  DoBlock::from(yystack.owned_value_at(0));
                        yyval = Value::Node(
                            self.builder.block(
                                method_call,
                                begin_t,
                                args_type,
                                body,
                                end_t
                            )?
                        );
                    },


  455 =>  /* method_call: fcall paren_args  */
  /* "src/parser/parse.y":4286  */
                    {
                        let ParenArgs { begin_t, args, end_t } =  ParenArgs::from(yystack.owned_value_at(0));

                        yyval = Value::Node(
                            self.builder.call_method(
                                None,
                                None,
                                Some( Token::from(yystack.owned_value_at(1))),
                                Some(begin_t),
                                args,
                                Some(end_t)
                            )
                        );
                    },


  456 =>  /* method_call: primary_value call_op operation2 opt_paren_args  */
  /* "src/parser/parse.y":4301  */
                    {
                        let OptParenArgs { begin_t, args, end_t } =  OptParenArgs::from(yystack.owned_value_at(0));

                        yyval = Value::Node(
                            self.builder.call_method(
                                Some( BoxedNode::from(yystack.owned_value_at(3))),
                                Some( Token::from(yystack.owned_value_at(2))),
                                Some( Token::from(yystack.owned_value_at(1))),
                                begin_t,
                                args,
                                end_t
                            )
                        );
                    },


  457 =>  /* method_call: primary_value "::" operation2 paren_args  */
  /* "src/parser/parse.y":4316  */
                    {
                        let ParenArgs { begin_t, args, end_t } =  ParenArgs::from(yystack.owned_value_at(0));

                        yyval = Value::Node(
                            self.builder.call_method(
                                Some( BoxedNode::from(yystack.owned_value_at(3))),
                                Some( Token::from(yystack.owned_value_at(2))),
                                Some( Token::from(yystack.owned_value_at(1))),
                                Some(begin_t),
                                args,
                                Some(end_t)
                            )
                        );
                    },


  458 =>  /* method_call: primary_value "::" operation3  */
  /* "src/parser/parse.y":4331  */
                    {
                        yyval = Value::Node(
                            self.builder.call_method(
                                Some( BoxedNode::from(yystack.owned_value_at(2))),
                                Some( Token::from(yystack.owned_value_at(1))),
                                Some( Token::from(yystack.owned_value_at(0))),
                                None,
                                vec![],
                                None
                            )
                        );
                    },


  459 =>  /* method_call: primary_value call_op paren_args  */
  /* "src/parser/parse.y":4344  */
                    {
                        let ParenArgs { begin_t, args, end_t } =  ParenArgs::from(yystack.owned_value_at(0));

                        yyval = Value::Node(
                            self.builder.call_method(
                                Some( BoxedNode::from(yystack.owned_value_at(2))),
                                Some( Token::from(yystack.owned_value_at(1))),
                                None,
                                Some(begin_t),
                                args,
                                Some(end_t)
                            )
                        );
                    },


  460 =>  /* method_call: primary_value "::" paren_args  */
  /* "src/parser/parse.y":4359  */
                    {
                        let ParenArgs { begin_t, args, end_t } =  ParenArgs::from(yystack.owned_value_at(0));

                        yyval = Value::Node(
                            self.builder.call_method(
                                Some( BoxedNode::from(yystack.owned_value_at(2))),
                                Some( Token::from(yystack.owned_value_at(1))),
                                None,
                                Some(begin_t),
                                args,
                                Some(end_t)
                            )
                        );
                    },


  461 =>  /* method_call: "`super'" paren_args  */
  /* "src/parser/parse.y":4374  */
                    {
                        let ParenArgs { begin_t, args, end_t } =  ParenArgs::from(yystack.owned_value_at(0));

                        yyval = Value::Node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Super,
                                 Token::from(yystack.owned_value_at(1)),
                                Some(begin_t),
                                args,
                                Some(end_t)
                            )?
                        );
                    },


  462 =>  /* method_call: "`super'"  */
  /* "src/parser/parse.y":4388  */
                    {
                        yyval = Value::Node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Zsuper,
                                 Token::from(yystack.owned_value_at(0)),
                                None,
                                vec![],
                                None
                            )?
                        );
                    },


  463 =>  /* method_call: primary_value "[ (tLBRACK2)" opt_call_args rbracket  */
  /* "src/parser/parse.y":4400  */
                    {
                        yyval = Value::Node(
                            self.builder.index(
                                 BoxedNode::from(yystack.owned_value_at(3)),
                                 Token::from(yystack.owned_value_at(2)),
                                 NodeList::from(yystack.owned_value_at(1)),
                                 Token::from(yystack.owned_value_at(0))
                            )
                        );
                    },


  464 =>  /* @28: %empty  */
  /* "src/parser/parse.y":4413  */
                    {
                        yyval = Value::Context(self.context.dump());
                        self.context.set_in_block(true);
                    },


  465 =>  /* brace_block: "{ (tLCURLY)" @28 brace_body "}"  */
  /* "src/parser/parse.y":4418  */
                    {
                        let BraceBody { args_type, body } =  BraceBody::from(yystack.owned_value_at(1));
                        self.context.set_in_block( Context::from(yystack.owned_value_at(2)).in_block());

                        yyval = Value::new_brace_block(
                            BraceBlock {
                                begin_t:  Token::from(yystack.owned_value_at(3)),
                                args_type,
                                body,
                                end_t:  Token::from(yystack.owned_value_at(0))
                            }
                        );
                    },


  466 =>  /* @29: %empty  */
  /* "src/parser/parse.y":4432  */
                    {
                        yyval = Value::Context(self.context.dump());
                        self.context.set_in_block(true);
                    },


  467 =>  /* brace_block: k_do @29 do_body k_end  */
  /* "src/parser/parse.y":4437  */
                    {
                        let DoBody { args_type, body } =  DoBody::from(yystack.owned_value_at(1));
                        self.context.set_in_block( Context::from(yystack.owned_value_at(2)).in_block());

                        yyval = Value::new_brace_block(
                            BraceBlock {
                                begin_t:  Token::from(yystack.owned_value_at(3)),
                                args_type,
                                body,
                                end_t:  Token::from(yystack.owned_value_at(0))
                            }
                        );
                    },


  468 =>  /* @30: %empty  */
  /* "src/parser/parse.y":4452  */
                    {
                        self.static_env.extend_dynamic();
                        self.max_numparam_stack.push(false);
                        yyval = Value::None;
                    },


  469 =>  /* brace_body: @30 opt_block_param compstmt  */
  /* "src/parser/parse.y":4458  */
                    {
                        let args_type = if self.max_numparam_stack.has_numparams() {
                            ArgsType::Numargs(self.max_numparam_stack.top() as u8)
                        } else {
                            ArgsType::Args( MaybeBoxedNode::from(yystack.owned_value_at(1)))
                        };

                        self.max_numparam_stack.pop();
                        self.static_env.unextend();

                        yyval = Value::new_brace_body(
                            BraceBody {
                                args_type,
                                body:  MaybeBoxedNode::from(yystack.owned_value_at(0))
                            }
                        );
                    },


  470 =>  /* @31: %empty  */
  /* "src/parser/parse.y":4477  */
                    {
                        self.static_env.extend_dynamic();
                        self.max_numparam_stack.push(false);
                        self.yylexer.cmdarg.push(false);
                        yyval = Value::None;
                    },


  471 =>  /* do_body: @31 opt_block_param bodystmt  */
  /* "src/parser/parse.y":4484  */
                    {
                        let args_type = if self.max_numparam_stack.has_numparams() {
                            ArgsType::Numargs(self.max_numparam_stack.top() as u8)
                        } else {
                            ArgsType::Args( MaybeBoxedNode::from(yystack.owned_value_at(1)))
                        };

                        self.max_numparam_stack.pop();
                        self.static_env.unextend();
                        self.yylexer.cmdarg.pop();

                        yyval = Value::new_do_body(
                            DoBody { args_type, body:  MaybeBoxedNode::from(yystack.owned_value_at(0)) }
                        );
                    },


  472 =>  /* case_args: arg_value  */
  /* "src/parser/parse.y":4502  */
                    {
                        yyval = Value::NodeList( Box::new(vec![  Node::from(yystack.owned_value_at(0)) ]) );
                    },


  473 =>  /* case_args: "*" arg_value  */
  /* "src/parser/parse.y":4506  */
                    {
                        yyval = Value::NodeList(
                            Box::new(
                                vec![
                                    *self.builder.splat(
                                         Token::from(yystack.owned_value_at(1)),
                                        Some( BoxedNode::from(yystack.owned_value_at(0)))
                                    )
                                ]
                            )
                        );
                    },


  474 =>  /* case_args: case_args "," arg_value  */
  /* "src/parser/parse.y":4519  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(2));
                        nodes.push(  Node::from(yystack.owned_value_at(0)) );
                        yyval = Value::NodeList(nodes);
                    },


  475 =>  /* case_args: case_args "," "*" arg_value  */
  /* "src/parser/parse.y":4525  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(3));
                        let splat = *self.builder.splat( Token::from(yystack.owned_value_at(1)), Some( BoxedNode::from(yystack.owned_value_at(0))));
                        nodes.push(splat);
                        yyval = Value::NodeList(nodes);
                    },


  476 =>  /* case_body: k_when case_args then compstmt cases  */
  /* "src/parser/parse.y":4536  */
                    {
                        let when = *self.builder.when( Token::from(yystack.owned_value_at(4)),  NodeList::from(yystack.owned_value_at(3)),  Token::from(yystack.owned_value_at(2)),  MaybeBoxedNode::from(yystack.owned_value_at(1)));
                        let Cases { mut when_bodies, opt_else } =  Cases::from(yystack.owned_value_at(0));

                        let mut nodes = Vec::with_capacity(1 + when_bodies.len());
                        nodes.push(when);
                        nodes.append(&mut when_bodies);

                        yyval = Value::new_case_body(CaseBody { when_bodies: nodes, opt_else });
                    },


  477 =>  /* cases: opt_else  */
  /* "src/parser/parse.y":4549  */
                    {
                        yyval = Value::new_cases(Cases { when_bodies: vec![], opt_else:  OptElse::from(yystack.owned_value_at(0)) });
                    },


  478 =>  /* cases: case_body  */
  /* "src/parser/parse.y":4553  */
                    {
                        let CaseBody { when_bodies, .. } =  CaseBody::from(yystack.owned_value_at(0));
                        yyval = Value::new_cases(Cases { when_bodies, opt_else: None });
                    },


  479 =>  /* @32: %empty  */
  /* "src/parser/parse.y":4560  */
                    {
                        self.yylexer.lex_state.set(EXPR_BEG|EXPR_LABEL);
                        self.yylexer.command_start = false;
                        self.pattern_variables.push();
                        self.pattern_hash_keys.push();

                        yyval = Value::Bool(self.context.in_kwarg());
                        self.context.set_in_kwarg(true);
                    },


  480 =>  /* @33: %empty  */
  /* "src/parser/parse.y":4570  */
                    {
                        self.context.set_in_kwarg( Bool::from(yystack.owned_value_at(2)));
                        self.pattern_variables.pop();
                        self.pattern_hash_keys.pop();
                        yyval = Value::None;
                    },


  481 =>  /* p_case_body: "`in'" @32 p_top_expr then @33 compstmt p_cases  */
  /* "src/parser/parse.y":4578  */
                    {
                        let PCases { mut in_bodies, opt_else } =  PCases::from(yystack.owned_value_at(0));
                        let PTopExpr { pattern, guard } =  PTopExpr::from(yystack.owned_value_at(4));

                        let mut nodes = Vec::with_capacity(1 + in_bodies.len());
                        nodes.push(
                            *self.builder.in_pattern(
                                 Token::from(yystack.owned_value_at(6)),
                                pattern,
                                guard,
                                 Token::from(yystack.owned_value_at(3)),
                                 MaybeBoxedNode::from(yystack.owned_value_at(1))
                            )
                        );
                        nodes.append(&mut in_bodies);

                        yyval = Value::new_p_case_body(PCaseBody { in_bodies: nodes, opt_else  });
                    },


  482 =>  /* p_cases: opt_else  */
  /* "src/parser/parse.y":4599  */
                    {
                        yyval = Value::new_p_cases(PCases { in_bodies: vec![], opt_else:  OptElse::from(yystack.owned_value_at(0)) });
                    },


  483 =>  /* p_cases: p_case_body  */
  /* "src/parser/parse.y":4603  */
                    {
                        let PCaseBody { in_bodies, .. } =  PCaseBody::from(yystack.owned_value_at(0));
                        yyval = Value::new_p_cases(PCases { in_bodies, opt_else: None });
                    },


  484 =>  /* p_top_expr: p_top_expr_body  */
  /* "src/parser/parse.y":4610  */
                    {
                        yyval = Value::new_p_top_expr(PTopExpr { pattern:  BoxedNode::from(yystack.owned_value_at(0)), guard: None });
                    },


  485 =>  /* p_top_expr: p_top_expr_body "`if' modifier" expr_value  */
  /* "src/parser/parse.y":4614  */
                    {
                        let guard = self.builder.if_guard( Token::from(yystack.owned_value_at(1)),  BoxedNode::from(yystack.owned_value_at(0)));
                        yyval = Value::new_p_top_expr(PTopExpr { pattern:  BoxedNode::from(yystack.owned_value_at(2)), guard: Some(guard) });
                    },


  486 =>  /* p_top_expr: p_top_expr_body "`unless' modifier" expr_value  */
  /* "src/parser/parse.y":4619  */
                    {
                        let guard = self.builder.unless_guard( Token::from(yystack.owned_value_at(1)),  BoxedNode::from(yystack.owned_value_at(0)));
                        yyval = Value::new_p_top_expr(PTopExpr { pattern:  BoxedNode::from(yystack.owned_value_at(2)), guard: Some(guard) });
                    },


  487 =>  /* p_top_expr_body: p_expr  */
  /* "src/parser/parse.y":4626  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  488 =>  /* p_top_expr_body: p_expr ","  */
  /* "src/parser/parse.y":4630  */
                    {
                        yyval = Value::Node(
                            self.builder.array_pattern(
                                None,
                                vec![  Node::from(yystack.owned_value_at(1)) ],
                                Some( Token::from(yystack.owned_value_at(0))),
                                None
                            )
                        );
                    },


  489 =>  /* p_top_expr_body: p_expr "," p_args  */
  /* "src/parser/parse.y":4641  */
                    {
                        let MatchPatternWithTrailingComma { mut elements, trailing_comma } =  MatchPatternWithTrailingComma::from(yystack.owned_value_at(0));

                        let mut nodes = Vec::with_capacity(1 + elements.len());
                        nodes.push( Node::from(yystack.owned_value_at(2)));
                        nodes.append(&mut elements);

                        yyval = Value::Node(
                            self.builder.array_pattern(None, nodes, trailing_comma, None)
                        );
                    },


  490 =>  /* p_top_expr_body: p_find  */
  /* "src/parser/parse.y":4653  */
                    {
                        yyval = Value::Node(
                            self.builder.find_pattern(None,  NodeList::from(yystack.owned_value_at(0)), None)
                        );
                    },


  491 =>  /* p_top_expr_body: p_args_tail  */
  /* "src/parser/parse.y":4659  */
                    {
                        yyval = Value::Node(
                            self.builder.array_pattern(None,  NodeList::from(yystack.owned_value_at(0)), None, None)
                        );
                    },


  492 =>  /* p_top_expr_body: p_kwargs  */
  /* "src/parser/parse.y":4665  */
                    {
                        yyval = Value::Node(
                            self.builder.hash_pattern(None,  NodeList::from(yystack.owned_value_at(0)), None)
                        );
                    },


  493 =>  /* p_expr: p_as  */
  /* "src/parser/parse.y":4673  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  494 =>  /* p_as: p_expr "=>" p_variable  */
  /* "src/parser/parse.y":4679  */
                    {
                        yyval = Value::Node(
                            self.builder.match_as(
                                 BoxedNode::from(yystack.owned_value_at(2)),
                                 Token::from(yystack.owned_value_at(1)),
                                 BoxedNode::from(yystack.owned_value_at(0))
                            )
                        );
                    },


  495 =>  /* p_as: p_alt  */
  /* "src/parser/parse.y":4689  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  496 =>  /* p_alt: p_alt "|" p_expr_basic  */
  /* "src/parser/parse.y":4695  */
                    {
                        yyval = Value::Node(
                            self.builder.match_alt(
                                 BoxedNode::from(yystack.owned_value_at(2)),
                                 Token::from(yystack.owned_value_at(1)),
                                 BoxedNode::from(yystack.owned_value_at(0))
                            )
                        );
                    },


  497 =>  /* p_alt: p_expr_basic  */
  /* "src/parser/parse.y":4705  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  498 =>  /* p_lparen: "( (tLPAREN2)"  */
  /* "src/parser/parse.y":4711  */
                    {
                        yyval =  yystack.owned_value_at(0);
                        self.pattern_hash_keys.push();
                    },


  499 =>  /* p_lbracket: "[ (tLBRACK2)"  */
  /* "src/parser/parse.y":4718  */
                    {
                        yyval =  yystack.owned_value_at(0);
                        self.pattern_hash_keys.push();
                    },


  500 =>  /* p_expr_basic: p_value  */
  /* "src/parser/parse.y":4725  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  501 =>  /* p_expr_basic: p_variable  */
  /* "src/parser/parse.y":4729  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  502 =>  /* p_expr_basic: p_const p_lparen p_args rparen  */
  /* "src/parser/parse.y":4733  */
                    {
                        self.pattern_hash_keys.pop();
                        let MatchPatternWithTrailingComma { elements, trailing_comma } =  MatchPatternWithTrailingComma::from(yystack.owned_value_at(1));
                        let pattern = self.builder.array_pattern(None, elements, trailing_comma, None);
                        yyval = Value::Node(
                            self.builder.const_pattern(
                                 BoxedNode::from(yystack.owned_value_at(3)),
                                 Token::from(yystack.owned_value_at(2)),
                                pattern,
                                 Token::from(yystack.owned_value_at(0))
                            )
                        );
                    },


  503 =>  /* p_expr_basic: p_const p_lparen p_find rparen  */
  /* "src/parser/parse.y":4747  */
                    {
                        self.pattern_hash_keys.pop();
                        let pattern = self.builder.find_pattern(None,  NodeList::from(yystack.owned_value_at(1)), None);
                        yyval = Value::Node(
                            self.builder.const_pattern(
                                 BoxedNode::from(yystack.owned_value_at(3)),
                                 Token::from(yystack.owned_value_at(2)),
                                pattern,
                                 Token::from(yystack.owned_value_at(0))
                            )
                        );
                    },


  504 =>  /* p_expr_basic: p_const p_lparen p_kwargs rparen  */
  /* "src/parser/parse.y":4760  */
                    {
                        self.pattern_hash_keys.pop();
                        let pattern = self.builder.hash_pattern(None,  NodeList::from(yystack.owned_value_at(1)), None);
                        yyval = Value::Node(
                            self.builder.const_pattern(
                                 BoxedNode::from(yystack.owned_value_at(3)),
                                 Token::from(yystack.owned_value_at(2)),
                                pattern,
                                 Token::from(yystack.owned_value_at(0))
                            )
                        );
                    },


  505 =>  /* p_expr_basic: p_const "( (tLPAREN2)" rparen  */
  /* "src/parser/parse.y":4773  */
                    {
                        let lparen =  Token::from(yystack.owned_value_at(1));
                        let rparen =  Token::from(yystack.owned_value_at(0));
                        let pattern = self.builder.array_pattern(
                            Some(lparen.clone()),
                            vec![],
                            None,
                            Some(rparen.clone())
                        );
                        yyval = Value::Node(
                            self.builder.const_pattern(
                                 BoxedNode::from(yystack.owned_value_at(2)),
                                lparen,
                                pattern,
                                rparen
                            )
                        );
                    },


  506 =>  /* p_expr_basic: p_const p_lbracket p_args rbracket  */
  /* "src/parser/parse.y":4792  */
                    {
                        self.pattern_hash_keys.pop();
                        let MatchPatternWithTrailingComma { elements, trailing_comma } =  MatchPatternWithTrailingComma::from(yystack.owned_value_at(1));
                        let pattern = self.builder.array_pattern(None, elements, trailing_comma, None);
                        yyval = Value::Node(
                            self.builder.const_pattern(
                                 BoxedNode::from(yystack.owned_value_at(3)),
                                 Token::from(yystack.owned_value_at(2)),
                                pattern,
                                 Token::from(yystack.owned_value_at(0))
                            )
                        );
                    },


  507 =>  /* p_expr_basic: p_const p_lbracket p_find rbracket  */
  /* "src/parser/parse.y":4806  */
                    {
                        self.pattern_hash_keys.pop();
                        let pattern = self.builder.find_pattern(None,  NodeList::from(yystack.owned_value_at(1)), None);
                        yyval = Value::Node(
                            self.builder.const_pattern(
                                 BoxedNode::from(yystack.owned_value_at(3)),
                                 Token::from(yystack.owned_value_at(2)),
                                pattern,
                                 Token::from(yystack.owned_value_at(0))
                            )
                        );
                    },


  508 =>  /* p_expr_basic: p_const p_lbracket p_kwargs rbracket  */
  /* "src/parser/parse.y":4819  */
                    {
                        self.pattern_hash_keys.pop();
                        let pattern = self.builder.hash_pattern(None,  NodeList::from(yystack.owned_value_at(1)), None);
                        yyval = Value::Node(
                            self.builder.const_pattern(
                                 BoxedNode::from(yystack.owned_value_at(3)),
                                 Token::from(yystack.owned_value_at(2)),
                                pattern,
                                 Token::from(yystack.owned_value_at(0))
                            )
                        );
                    },


  509 =>  /* p_expr_basic: p_const "[ (tLBRACK2)" rbracket  */
  /* "src/parser/parse.y":4832  */
                    {
                        let lparen =  Token::from(yystack.owned_value_at(1));
                        let rparen =  Token::from(yystack.owned_value_at(0));
                        let pattern = self.builder.array_pattern(
                            Some(lparen.clone()),
                            vec![],
                            None,
                            Some(rparen.clone())
                        );
                        yyval = Value::Node(
                            self.builder.const_pattern(
                                 BoxedNode::from(yystack.owned_value_at(2)),
                                lparen,
                                pattern,
                                rparen
                            )
                        );
                    },


  510 =>  /* p_expr_basic: "[" p_args rbracket  */
  /* "src/parser/parse.y":4851  */
                    {
                        let MatchPatternWithTrailingComma { elements, trailing_comma } =  MatchPatternWithTrailingComma::from(yystack.owned_value_at(1));
                        yyval = Value::Node(
                            self.builder.array_pattern(
                                Some( Token::from(yystack.owned_value_at(2))),
                                elements,
                                trailing_comma,
                                Some( Token::from(yystack.owned_value_at(0)))
                            )
                        );
                    },


  511 =>  /* p_expr_basic: "[" p_find rbracket  */
  /* "src/parser/parse.y":4863  */
                    {
                        yyval = Value::Node(
                            self.builder.find_pattern(
                                Some( Token::from(yystack.owned_value_at(2))),
                                 NodeList::from(yystack.owned_value_at(1)),
                                Some( Token::from(yystack.owned_value_at(0)))
                            )
                        );
                    },


  512 =>  /* p_expr_basic: "[" rbracket  */
  /* "src/parser/parse.y":4873  */
                    {
                        yyval = Value::Node(
                            self.builder.array_pattern(
                                Some( Token::from(yystack.owned_value_at(1))),
                                vec![],
                                None,
                                Some( Token::from(yystack.owned_value_at(0)))
                            )
                        );
                    },


  513 =>  /* @34: %empty  */
  /* "src/parser/parse.y":4884  */
                    {
                        self.pattern_hash_keys.push();
                        yyval = Value::Bool(self.context.in_kwarg());
                        self.context.set_in_kwarg(false);
                    },


  514 =>  /* p_expr_basic: "{" @34 p_kwargs rbrace  */
  /* "src/parser/parse.y":4890  */
                    {
                        self.pattern_hash_keys.pop();
                        self.context.set_in_kwarg( Bool::from(yystack.owned_value_at(2)));
                        yyval = Value::Node(
                            self.builder.hash_pattern(
                                Some( Token::from(yystack.owned_value_at(3))),
                                 NodeList::from(yystack.owned_value_at(1)),
                                Some( Token::from(yystack.owned_value_at(0)))
                            )
                        );
                    },


  515 =>  /* p_expr_basic: "{" rbrace  */
  /* "src/parser/parse.y":4902  */
                    {
                        yyval = Value::Node(
                            self.builder.hash_pattern(
                                Some( Token::from(yystack.owned_value_at(1))),
                                vec![],
                                Some( Token::from(yystack.owned_value_at(0))),
                            )
                        );
                    },


  516 =>  /* @35: %empty  */
  /* "src/parser/parse.y":4912  */
                    {
                        self.pattern_hash_keys.push();
                        yyval = Value::None;
                    },


  517 =>  /* p_expr_basic: "(" @35 p_expr rparen  */
  /* "src/parser/parse.y":4917  */
                    {
                        self.pattern_hash_keys.pop();
                        yyval = Value::Node(
                            self.builder.begin(
                                 Token::from(yystack.owned_value_at(3)),
                                Some( BoxedNode::from(yystack.owned_value_at(1))),
                                 Token::from(yystack.owned_value_at(0))
                            )
                        );
                    },


  518 =>  /* p_args: p_expr  */
  /* "src/parser/parse.y":4930  */
                    {
                        yyval = Value::new_match_pattern_with_trailing_comma(
                            MatchPatternWithTrailingComma {
                                elements: vec![  Node::from(yystack.owned_value_at(0)) ],
                                trailing_comma: None
                            }
                        );
                    },


  519 =>  /* p_args: p_args_head  */
  /* "src/parser/parse.y":4939  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  520 =>  /* p_args: p_args_head p_arg  */
  /* "src/parser/parse.y":4943  */
                    {
                        let mut elements =  MatchPatternWithTrailingComma::from(yystack.owned_value_at(1)).elements;
                        elements.push( Node::from(yystack.owned_value_at(0)));

                        yyval = Value::new_match_pattern_with_trailing_comma(
                            MatchPatternWithTrailingComma {
                                elements,
                                trailing_comma: None
                            }
                        );
                    },


  521 =>  /* p_args: p_args_head p_rest  */
  /* "src/parser/parse.y":4955  */
                    {
                        let mut elements =  MatchPatternWithTrailingComma::from(yystack.owned_value_at(1)).elements;
                        let p_rest =  Node::from(yystack.owned_value_at(0));
                        elements.push(p_rest);

                        yyval = Value::new_match_pattern_with_trailing_comma(
                            MatchPatternWithTrailingComma {
                                elements,
                                trailing_comma: None
                            }
                        );
                    },


  522 =>  /* p_args: p_args_head p_rest "," p_args_post  */
  /* "src/parser/parse.y":4968  */
                    {
                        let mut elements =  MatchPatternWithTrailingComma::from(yystack.owned_value_at(3)).elements;
                        let p_rest =  Node::from(yystack.owned_value_at(2));
                        let mut p_args_post =  NodeList::from(yystack.owned_value_at(0));

                        elements.reserve(1 + p_args_post.len());
                        elements.push(p_rest);
                        elements.append(&mut p_args_post);

                        yyval = Value::new_match_pattern_with_trailing_comma(
                            MatchPatternWithTrailingComma {
                                elements,
                                trailing_comma: None
                            }
                        );
                    },


  523 =>  /* p_args: p_args_tail  */
  /* "src/parser/parse.y":4985  */
                    {
                        yyval = Value::new_match_pattern_with_trailing_comma(
                            MatchPatternWithTrailingComma {
                                elements:  NodeList::from(yystack.owned_value_at(0)),
                                trailing_comma: None
                            }
                        );
                    },


  524 =>  /* p_args_head: p_arg ","  */
  /* "src/parser/parse.y":4996  */
                    {
                        yyval = Value::new_match_pattern_with_trailing_comma(
                            MatchPatternWithTrailingComma {
                                elements: vec![ Node::from(yystack.owned_value_at(1))],
                                trailing_comma: Some( Token::from(yystack.owned_value_at(0))),
                            }
                        );
                    },


  525 =>  /* p_args_head: p_args_head p_arg ","  */
  /* "src/parser/parse.y":5005  */
                    {
                        let mut elements =  MatchPatternWithTrailingComma::from(yystack.owned_value_at(2)).elements;
                        elements.push( Node::from(yystack.owned_value_at(1)));

                        yyval = Value::new_match_pattern_with_trailing_comma(
                            MatchPatternWithTrailingComma {
                                elements,
                                trailing_comma: Some( Token::from(yystack.owned_value_at(0))),
                            }
                        );
                    },


  526 =>  /* p_args_tail: p_rest  */
  /* "src/parser/parse.y":5019  */
                    {
                        yyval = Value::NodeList( Box::new(vec![  Node::from(yystack.owned_value_at(0)) ]) );
                    },


  527 =>  /* p_args_tail: p_rest "," p_args_post  */
  /* "src/parser/parse.y":5023  */
                    {
                        let mut p_args_post =  NodeList::from(yystack.owned_value_at(0));
                        let mut nodes = Box::new(Vec::with_capacity(1 + p_args_post.len()));
                        nodes.push( Node::from(yystack.owned_value_at(2)));
                        nodes.append(&mut p_args_post);

                        yyval = Value::NodeList(nodes);
                    },


  528 =>  /* p_find: p_rest "," p_args_post "," p_rest  */
  /* "src/parser/parse.y":5034  */
                    {
                        let mut p_args_post =  NodeList::from(yystack.owned_value_at(2));
                        let mut nodes = Box::new(Vec::with_capacity(1 + p_args_post.len() + 1));
                        nodes.push( Node::from(yystack.owned_value_at(4)));
                        nodes.append(&mut p_args_post);
                        nodes.push( Node::from(yystack.owned_value_at(0)));

                        yyval = Value::NodeList(nodes);
                    },


  529 =>  /* p_rest: "*" "local variable or method"  */
  /* "src/parser/parse.y":5047  */
                    {
                        yyval = Value::Node(
                            self.builder.match_rest( Token::from(yystack.owned_value_at(1)), Some( Token::from(yystack.owned_value_at(0))))?
                        );
                    },


  530 =>  /* p_rest: "*"  */
  /* "src/parser/parse.y":5053  */
                    {
                        yyval = Value::Node(
                            self.builder.match_rest( Token::from(yystack.owned_value_at(0)), None)?
                        );
                    },


  531 =>  /* p_args_post: p_arg  */
  /* "src/parser/parse.y":5061  */
                    {
                        yyval = Value::NodeList( Box::new(vec![  Node::from(yystack.owned_value_at(0)) ]) );
                    },


  532 =>  /* p_args_post: p_args_post "," p_arg  */
  /* "src/parser/parse.y":5065  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(2));
                        nodes.push(  Node::from(yystack.owned_value_at(0)) );
                        yyval = Value::NodeList(nodes);
                    },


  533 =>  /* p_arg: p_expr  */
  /* "src/parser/parse.y":5073  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  534 =>  /* p_kwargs: p_kwarg "," p_any_kwrest  */
  /* "src/parser/parse.y":5079  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(2));
                        nodes.append(&mut  NodeList::from(yystack.owned_value_at(0)));

                        yyval = Value::NodeList(nodes);
                    },


  535 =>  /* p_kwargs: p_kwarg  */
  /* "src/parser/parse.y":5086  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  536 =>  /* p_kwargs: p_kwarg ","  */
  /* "src/parser/parse.y":5090  */
                    {
                        yyval =  yystack.owned_value_at(1);
                    },


  537 =>  /* p_kwargs: p_any_kwrest  */
  /* "src/parser/parse.y":5094  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  538 =>  /* p_kwarg: p_kw  */
  /* "src/parser/parse.y":5100  */
                    {
                        yyval = Value::NodeList( Box::new(vec![  Node::from(yystack.owned_value_at(0)) ]) );
                    },


  539 =>  /* p_kwarg: p_kwarg "," p_kw  */
  /* "src/parser/parse.y":5104  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(2));
                        nodes.push(  Node::from(yystack.owned_value_at(0)) );
                        yyval = Value::NodeList(nodes);
                    },


  540 =>  /* p_kw: p_kw_label p_expr  */
  /* "src/parser/parse.y":5112  */
                    {
                        yyval = Value::Node(
                            self.builder.match_pair(
                                 PKwLabel::from(yystack.owned_value_at(1)),
                                 BoxedNode::from(yystack.owned_value_at(0))
                            )?
                        );
                    },


  541 =>  /* p_kw: p_kw_label  */
  /* "src/parser/parse.y":5121  */
                    {
                        yyval = Value::Node(
                            self.builder.match_label(
                                 PKwLabel::from(yystack.owned_value_at(0)),
                            )?
                        );
                    },


  542 =>  /* p_kw_label: "label"  */
  /* "src/parser/parse.y":5131  */
                    {
                        yyval = Value::new_p_kw_label(
                            PKwLabel::PlainLabel( Token::from(yystack.owned_value_at(0)))
                        );
                    },


  543 =>  /* p_kw_label: "string begin" string_contents tLABEL_END  */
  /* "src/parser/parse.y":5137  */
                    {
                        yyval = Value::new_p_kw_label(
                            PKwLabel::QuotedLabel( ( Token::from(yystack.owned_value_at(2)),  NodeList::from(yystack.owned_value_at(1)),  Token::from(yystack.owned_value_at(0))) )
                        );
                    },


  544 =>  /* p_kwrest: kwrest_mark "local variable or method"  */
  /* "src/parser/parse.y":5145  */
                    {
                        yyval = Value::NodeList(
                            Box::new(
                                vec![
                                    *self.builder.match_rest(
                                         Token::from(yystack.owned_value_at(1)),
                                        Some( Token::from(yystack.owned_value_at(0)))
                                    )?
                                ]
                            )
                        );
                    },


  545 =>  /* p_kwrest: kwrest_mark  */
  /* "src/parser/parse.y":5158  */
                    {
                        yyval = Value::NodeList(
                            Box::new(
                                vec![
                                    *self.builder.match_rest(
                                         Token::from(yystack.owned_value_at(0)),
                                        None
                                    )?
                                ]
                            )
                        );
                    },


  546 =>  /* p_kwnorest: kwrest_mark "`nil'"  */
  /* "src/parser/parse.y":5173  */
                    {
                        yyval = Value::new_no_kw_rest(
                            NoKwRest {
                                kwrest_mark:  Token::from(yystack.owned_value_at(1)),
                                k_nil:  Token::from(yystack.owned_value_at(0))
                            }
                        );
                    },


  547 =>  /* p_any_kwrest: p_kwrest  */
  /* "src/parser/parse.y":5184  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  548 =>  /* p_any_kwrest: p_kwnorest  */
  /* "src/parser/parse.y":5188  */
                    {
                        let NoKwRest { kwrest_mark, k_nil } =  NoKwRest::from(yystack.owned_value_at(0));
                        yyval = Value::NodeList(
                            Box::new(
                                vec![
                                    *self.builder.match_nil_pattern(
                                        kwrest_mark,
                                        k_nil
                                    )
                                ]
                            )
                        );
                    },


  549 =>  /* p_value: p_primitive  */
  /* "src/parser/parse.y":5204  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  550 =>  /* p_value: p_primitive ".." p_primitive  */
  /* "src/parser/parse.y":5208  */
                    {
                        let left =  BoxedNode::from(yystack.owned_value_at(2));
                        self.value_expr(&left)?;

                        let right =  BoxedNode::from(yystack.owned_value_at(0));
                        self.value_expr(&right)?;

                        yyval = Value::Node(
                            self.builder.range_inclusive(
                                Some(left),
                                 Token::from(yystack.owned_value_at(1)),
                                Some(right)
                            )
                        );
                    },


  551 =>  /* p_value: p_primitive "..." p_primitive  */
  /* "src/parser/parse.y":5224  */
                    {
                        let left =  BoxedNode::from(yystack.owned_value_at(2));
                        self.value_expr(&left)?;

                        let right =  BoxedNode::from(yystack.owned_value_at(0));
                        self.value_expr(&right)?;

                        yyval = Value::Node(
                            self.builder.range_exclusive(
                                Some(left),
                                 Token::from(yystack.owned_value_at(1)),
                                Some(right)
                            )
                        );
                    },


  552 =>  /* p_value: p_primitive ".."  */
  /* "src/parser/parse.y":5240  */
                    {
                        let left =  BoxedNode::from(yystack.owned_value_at(1));
                        self.value_expr(&left)?;

                        yyval = Value::Node(
                            self.builder.range_inclusive(
                                Some(left),
                                 Token::from(yystack.owned_value_at(0)),
                                None
                            )
                        );
                    },


  553 =>  /* p_value: p_primitive "..."  */
  /* "src/parser/parse.y":5253  */
                    {
                        let left =  BoxedNode::from(yystack.owned_value_at(1));
                        self.value_expr(&left)?;

                        yyval = Value::Node(
                            self.builder.range_exclusive(
                                Some(left),
                                 Token::from(yystack.owned_value_at(0)),
                                None
                            )
                        );
                    },


  554 =>  /* p_value: p_var_ref  */
  /* "src/parser/parse.y":5266  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  555 =>  /* p_value: p_expr_ref  */
  /* "src/parser/parse.y":5270  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  556 =>  /* p_value: p_const  */
  /* "src/parser/parse.y":5274  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  557 =>  /* p_value: "(.." p_primitive  */
  /* "src/parser/parse.y":5278  */
                    {
                        let right =  BoxedNode::from(yystack.owned_value_at(0));
                        self.value_expr(&right)?;

                        yyval = Value::Node(
                            self.builder.range_inclusive(
                                None,
                                 Token::from(yystack.owned_value_at(1)),
                                Some(right)
                            )
                        );
                    },


  558 =>  /* p_value: "(..." p_primitive  */
  /* "src/parser/parse.y":5291  */
                    {
                        let right =  BoxedNode::from(yystack.owned_value_at(0));
                        self.value_expr(&right)?;

                        yyval = Value::Node(
                            self.builder.range_exclusive(
                                None,
                                 Token::from(yystack.owned_value_at(1)),
                                Some(right)
                            )
                        );
                    },


  559 =>  /* p_primitive: literal  */
  /* "src/parser/parse.y":5306  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  560 =>  /* p_primitive: strings  */
  /* "src/parser/parse.y":5310  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  561 =>  /* p_primitive: xstring  */
  /* "src/parser/parse.y":5314  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  562 =>  /* p_primitive: regexp  */
  /* "src/parser/parse.y":5318  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  563 =>  /* p_primitive: words  */
  /* "src/parser/parse.y":5322  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  564 =>  /* p_primitive: qwords  */
  /* "src/parser/parse.y":5326  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  565 =>  /* p_primitive: symbols  */
  /* "src/parser/parse.y":5330  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  566 =>  /* p_primitive: qsymbols  */
  /* "src/parser/parse.y":5334  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  567 =>  /* p_primitive: keyword_variable  */
  /* "src/parser/parse.y":5338  */
                    {
                        yyval = Value::Node(
                            self.builder.accessible( BoxedNode::from(yystack.owned_value_at(0)))
                        );
                    },


  568 =>  /* p_primitive: lambda  */
  /* "src/parser/parse.y":5344  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  569 =>  /* p_variable: "local variable or method"  */
  /* "src/parser/parse.y":5350  */
                    {
                        yyval = Value::Node(
                            self.builder.assignable(
                                self.builder.match_var( Token::from(yystack.owned_value_at(0)))?
                            )?
                        );
                    },


  570 =>  /* p_var_ref: "^" "local variable or method"  */
  /* "src/parser/parse.y":5360  */
                    {
                        let ident_t =  Token::from(yystack.owned_value_at(0));
                        let name = clone_value(&ident_t);

                        if !self.static_env.is_declared(name.as_str()) {
                            return self.yyerror(
                                yystack.location_at (0),
                                DiagnosticMessage::NoSuchLocalVariable { var_name: name }
                            );
                        }

                        let lvar = self.builder.accessible(self.builder.lvar(ident_t));
                        yyval = Value::Node(
                            self.builder.pin( Token::from(yystack.owned_value_at(1)), lvar)
                        );
                    },


  571 =>  /* p_var_ref: "^" nonlocal_var  */
  /* "src/parser/parse.y":5377  */
                    {
                        let non_lvar = self.builder.accessible( BoxedNode::from(yystack.owned_value_at(0)));
                        yyval = Value::Node(
                            self.builder.pin(
                                 Token::from(yystack.owned_value_at(1)),
                                non_lvar,
                            )
                        );
                    },


  572 =>  /* p_expr_ref: "^" "(" expr_value ")"  */
  /* "src/parser/parse.y":5389  */
                    {
                        let expr = self.builder.begin(
                             Token::from(yystack.owned_value_at(2)),
                            Some( BoxedNode::from(yystack.owned_value_at(1))),
                             Token::from(yystack.owned_value_at(0))
                        );
                        yyval = Value::Node(
                            self.builder.pin(
                                 Token::from(yystack.owned_value_at(3)),
                                expr
                            )
                        );
                    },


  573 =>  /* p_const: ":: at EXPR_BEG" cname  */
  /* "src/parser/parse.y":5405  */
                    {
                        yyval = Value::Node(
                            self.builder.const_global( Token::from(yystack.owned_value_at(1)),  Token::from(yystack.owned_value_at(0)))
                        );
                    },


  574 =>  /* p_const: p_const "::" cname  */
  /* "src/parser/parse.y":5411  */
                    {
                        yyval = Value::Node(
                            self.builder.const_fetch(
                                 BoxedNode::from(yystack.owned_value_at(2)),
                                 Token::from(yystack.owned_value_at(1)),
                                 Token::from(yystack.owned_value_at(0)),
                            )
                        );
                    },


  575 =>  /* p_const: "constant"  */
  /* "src/parser/parse.y":5421  */
                    {
                        yyval = Value::Node(self.builder.const_( Token::from(yystack.owned_value_at(0))));
                    },


  576 =>  /* opt_rescue: k_rescue exc_list exc_var then compstmt opt_rescue  */
  /* "src/parser/parse.y":5429  */
                    {
                        let ExcVar { assoc_t, exc_var } =  ExcVar::from(yystack.owned_value_at(3));

                        let exc_list =  NodeList::from(yystack.owned_value_at(4));
                        let exc_list = if exc_list.is_empty() {
                            None
                        } else {
                            Some(self.builder.array(None, exc_list, None))
                        };

                        let rescue_body = *self.builder.rescue_body(
                             Token::from(yystack.owned_value_at(5)),
                            exc_list,
                            assoc_t,
                            exc_var,
                            Some( Token::from(yystack.owned_value_at(2))),
                             MaybeBoxedNode::from(yystack.owned_value_at(1))
                        );
                        let mut opt_rescue =  NodeList::from(yystack.owned_value_at(0));
                        let mut nodes = Box::new(Vec::with_capacity(1 + opt_rescue.len()));
                        nodes.push(rescue_body);
                        nodes.append(&mut opt_rescue);

                        yyval = Value::NodeList(nodes);
                    },


  577 =>  /* opt_rescue: none  */
  /* "src/parser/parse.y":5455  */
                    {
                        yyval = Value::NodeList( Box::new(vec![]) );
                    },


  578 =>  /* exc_list: arg_value  */
  /* "src/parser/parse.y":5461  */
                    {
                        yyval = Value::NodeList( Box::new(vec![  Node::from(yystack.owned_value_at(0)) ]) );
                    },


  579 =>  /* exc_list: mrhs  */
  /* "src/parser/parse.y":5465  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  580 =>  /* exc_list: none  */
  /* "src/parser/parse.y":5469  */
                    {
                        yyval = Value::NodeList( Box::new(vec![]) );
                    },


  581 =>  /* exc_var: "=>" lhs  */
  /* "src/parser/parse.y":5475  */
                    {
                        let assoc_t = Some( Token::from(yystack.owned_value_at(1)));
                        let exc_var = Some( BoxedNode::from(yystack.owned_value_at(0)));
                        yyval = Value::new_exc_var(ExcVar { assoc_t, exc_var });
                    },


  582 =>  /* exc_var: none  */
  /* "src/parser/parse.y":5481  */
                    {
                        yyval = Value::new_exc_var(ExcVar { assoc_t: None, exc_var: None });
                    },


  583 =>  /* opt_ensure: k_ensure compstmt  */
  /* "src/parser/parse.y":5487  */
                    {
                        let ensure_t =  Token::from(yystack.owned_value_at(1));
                        let body =  MaybeBoxedNode::from(yystack.owned_value_at(0));
                        yyval = Value::new_opt_ensure(Some(Ensure { ensure_t, body }));
                    },


  584 =>  /* opt_ensure: none  */
  /* "src/parser/parse.y":5493  */
                    {
                        yyval = Value::new_opt_ensure(None);
                    },


  585 =>  /* literal: numeric  */
  /* "src/parser/parse.y":5499  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  586 =>  /* literal: symbol  */
  /* "src/parser/parse.y":5503  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  587 =>  /* strings: string  */
  /* "src/parser/parse.y":5509  */
                    {
                        yyval = Value::Node(
                            self.builder.string_compose(
                                None,
                                 NodeList::from(yystack.owned_value_at(0)),
                                None
                            )
                        );
                    },


  588 =>  /* string: "char literal"  */
  /* "src/parser/parse.y":5521  */
                    {
                        yyval = Value::NodeList(
                            Box::new(
                                vec![
                                    *self.builder.character( Token::from(yystack.owned_value_at(0)))
                                ]
                            )
                        );
                    },


  589 =>  /* string: string1  */
  /* "src/parser/parse.y":5531  */
                    {
                        yyval = Value::NodeList( Box::new(vec![  Node::from(yystack.owned_value_at(0)) ]) );
                    },


  590 =>  /* string: string string1  */
  /* "src/parser/parse.y":5535  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(1));
                        nodes.push(  Node::from(yystack.owned_value_at(0)) );
                        yyval = Value::NodeList(nodes);
                    },


  591 =>  /* string1: "string begin" string_contents "string end"  */
  /* "src/parser/parse.y":5543  */
                    {
                        let mut string = self.builder.string_compose(
                            Some( Token::from(yystack.owned_value_at(2))),
                             NodeList::from(yystack.owned_value_at(1)),
                            Some( Token::from(yystack.owned_value_at(0)))
                        );
                        let indent = self.yylexer.buffer.heredoc_indent;
                        self.yylexer.buffer.heredoc_indent = 0;
                        string = self.builder.heredoc_dedent(string, indent);
                        yyval = Value::Node(string);
                    },


  592 =>  /* xstring: "backtick literal" xstring_contents "string end"  */
  /* "src/parser/parse.y":5557  */
                    {
                        let mut string = self.builder.xstring_compose(
                             Token::from(yystack.owned_value_at(2)),
                             NodeList::from(yystack.owned_value_at(1)),
                             Token::from(yystack.owned_value_at(0))
                        );
                        let indent = self.yylexer.buffer.heredoc_indent;
                        self.yylexer.buffer.heredoc_indent = 0;
                        string = self.builder.heredoc_dedent(string, indent);
                        yyval = Value::Node(string);
                    },


  593 =>  /* regexp: "regexp literal" regexp_contents tREGEXP_END  */
  /* "src/parser/parse.y":5571  */
                    {
                        let regexp_end =  Token::from(yystack.owned_value_at(0));
                        let opts = self.builder.regexp_options(regexp_end.clone());
                        yyval = Value::Node(
                            self.builder.regexp_compose(
                                 Token::from(yystack.owned_value_at(2)),
                                 NodeList::from(yystack.owned_value_at(1)),
                                regexp_end,
                                opts
                            )
                        );
                    },


  594 =>  /* words: "word list" " " word_list "string end"  */
  /* "src/parser/parse.y":5586  */
                    {
                        yyval = Value::Node(
                            self.builder.words_compose(
                                 Token::from(yystack.owned_value_at(3)),
                                 NodeList::from(yystack.owned_value_at(1)),
                                 Token::from(yystack.owned_value_at(0))
                            )
                        );
                    },


  595 =>  /* word_list: %empty  */
  /* "src/parser/parse.y":5598  */
                    {
                        yyval = Value::NodeList( Box::new(vec![]) );

                    },


  596 =>  /* word_list: word_list word " "  */
  /* "src/parser/parse.y":5603  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(2));
                        nodes.push(
                            *self.builder.word(  NodeList::from(yystack.owned_value_at(1)) )
                        );
                        yyval = Value::NodeList(nodes);
                    },


  597 =>  /* word: string_content  */
  /* "src/parser/parse.y":5613  */
                    {
                        yyval = Value::NodeList( Box::new(vec![  Node::from(yystack.owned_value_at(0)) ]) );
                    },


  598 =>  /* word: word string_content  */
  /* "src/parser/parse.y":5617  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(1));
                        nodes.push(  Node::from(yystack.owned_value_at(0)) );
                        yyval = Value::NodeList(nodes);
                    },


  599 =>  /* symbols: "symbol list" " " symbol_list "string end"  */
  /* "src/parser/parse.y":5625  */
                    {
                        yyval = Value::Node(
                            self.builder.symbols_compose(
                                 Token::from(yystack.owned_value_at(3)),
                                 NodeList::from(yystack.owned_value_at(1)),
                                 Token::from(yystack.owned_value_at(0))
                            )
                        );
                    },


  600 =>  /* symbol_list: %empty  */
  /* "src/parser/parse.y":5637  */
                    {
                        yyval = Value::NodeList( Box::new(vec![]) );
                    },


  601 =>  /* symbol_list: symbol_list word " "  */
  /* "src/parser/parse.y":5641  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(2));
                        nodes.push(
                            *self.builder.word(  NodeList::from(yystack.owned_value_at(1)) )
                        );
                        yyval = Value::NodeList(nodes);
                    },


  602 =>  /* qwords: "verbatim word list" " " qword_list "string end"  */
  /* "src/parser/parse.y":5651  */
                    {
                        yyval = Value::Node(
                            self.builder.words_compose(
                                 Token::from(yystack.owned_value_at(3)),
                                 NodeList::from(yystack.owned_value_at(1)),
                                 Token::from(yystack.owned_value_at(0))
                            )
                        );
                    },


  603 =>  /* qsymbols: "verbatim symbol list" " " qsym_list "string end"  */
  /* "src/parser/parse.y":5663  */
                    {
                        yyval = Value::Node(
                            self.builder.symbols_compose(
                                 Token::from(yystack.owned_value_at(3)),
                                 NodeList::from(yystack.owned_value_at(1)),
                                 Token::from(yystack.owned_value_at(0))
                            )
                        );
                    },


  604 =>  /* qword_list: %empty  */
  /* "src/parser/parse.y":5675  */
                    {
                        yyval = Value::NodeList( Box::new(vec![]) );
                    },


  605 =>  /* qword_list: qword_list "literal content" " "  */
  /* "src/parser/parse.y":5679  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(2));
                        nodes.push(
                            *self.builder.string_internal(  Token::from(yystack.owned_value_at(1)) )
                        );
                        yyval = Value::NodeList(nodes);
                    },


  606 =>  /* qsym_list: %empty  */
  /* "src/parser/parse.y":5689  */
                    {
                        yyval = Value::NodeList( Box::new(vec![]) );
                    },


  607 =>  /* qsym_list: qsym_list "literal content" " "  */
  /* "src/parser/parse.y":5693  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(2));
                        nodes.push(
                            *self.builder.symbol_internal(  Token::from(yystack.owned_value_at(1)) )
                        );
                        yyval = Value::NodeList(nodes);
                    },


  608 =>  /* string_contents: %empty  */
  /* "src/parser/parse.y":5703  */
                    {
                        yyval = Value::NodeList( Box::new(vec![]) );
                    },


  609 =>  /* string_contents: string_contents string_content  */
  /* "src/parser/parse.y":5707  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(1));
                        nodes.push( Node::from(yystack.owned_value_at(0)));
                        yyval = Value::NodeList(nodes);
                    },


  610 =>  /* xstring_contents: %empty  */
  /* "src/parser/parse.y":5715  */
                    {
                        yyval = Value::NodeList( Box::new(vec![]) );
                    },


  611 =>  /* xstring_contents: xstring_contents string_content  */
  /* "src/parser/parse.y":5719  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(1));
                        nodes.push( Node::from(yystack.owned_value_at(0)));
                        yyval = Value::NodeList(nodes);
                    },


  612 =>  /* regexp_contents: %empty  */
  /* "src/parser/parse.y":5727  */
                    {
                        yyval = Value::NodeList( Box::new(vec![]) );
                    },


  613 =>  /* regexp_contents: regexp_contents string_content  */
  /* "src/parser/parse.y":5731  */
                    {
                        let mut  nodes =  BoxedNodeList::from(yystack.owned_value_at(1));
                        nodes.push(  Node::from(yystack.owned_value_at(0)) );
                        yyval = Value::NodeList(nodes);
                    },


  614 =>  /* string_content: "literal content"  */
  /* "src/parser/parse.y":5739  */
                    {
                        yyval = Value::Node(
                            self.builder.string_internal( Token::from(yystack.owned_value_at(0)))
                        );
                    },


  615 =>  /* @36: %empty  */
  /* "src/parser/parse.y":5745  */
                    {
                        yyval = Value::MaybeStrTerm(std::mem::take(&mut self.yylexer.strterm));
                        self.yylexer.lex_state.set(EXPR_BEG);
                    },


  616 =>  /* string_content: tSTRING_DVAR @36 string_dvar  */
  /* "src/parser/parse.y":5750  */
                    {
                        self.yylexer.strterm =  MaybeStrTerm::from(yystack.owned_value_at(1));
                        yyval =  yystack.owned_value_at(0);
                    },


  617 =>  /* @37: %empty  */
  /* "src/parser/parse.y":5755  */
                    {
                        self.yylexer.cmdarg.push(false);
                        self.yylexer.cond.push(false);
                        yyval = Value::None;
                    },


  618 =>  /* @38: %empty  */
  /* "src/parser/parse.y":5760  */
                    {
                        yyval = Value::MaybeStrTerm(std::mem::take(&mut self.yylexer.strterm));
                    },


  619 =>  /* @39: %empty  */
  /* "src/parser/parse.y":5763  */
                    {
                        yyval = Value::Num( self.yylexer.lex_state.get() );
                        self.yylexer.lex_state.set(EXPR_BEG);
                    },


  620 =>  /* @40: %empty  */
  /* "src/parser/parse.y":5767  */
                    {
                        yyval = Value::Num( self.yylexer.brace_nest );
                        self.yylexer.brace_nest = 0;
                    },


  621 =>  /* @41: %empty  */
  /* "src/parser/parse.y":5771  */
                    {
                        yyval = Value::Num( self.yylexer.buffer.heredoc_indent );
                        self.yylexer.buffer.heredoc_indent = 0;
                    },


  622 =>  /* string_content: tSTRING_DBEG @37 @38 @39 @40 @41 compstmt "tRCURLY"  */
  /* "src/parser/parse.y":5776  */
                    {
                        self.yylexer.cond.pop();
                        self.yylexer.cmdarg.pop();
                        self.yylexer.strterm =  MaybeStrTerm::from(yystack.owned_value_at(5));
                        self.yylexer.lex_state.set( Num::from(yystack.owned_value_at(4)));
                        self.yylexer.brace_nest =  Num::from(yystack.owned_value_at(3));
                        self.yylexer.buffer.heredoc_indent =  Num::from(yystack.owned_value_at(2));
                        self.yylexer.buffer.heredoc_line_indent = -1;

                        yyval = Value::Node(
                            self.builder.begin(
                                 Token::from(yystack.owned_value_at(7)),
                                 MaybeBoxedNode::from(yystack.owned_value_at(1)),
                                 Token::from(yystack.owned_value_at(0))
                            )
                        );
                    },


  623 =>  /* string_dvar: "global variable"  */
  /* "src/parser/parse.y":5796  */
                    {
                        yyval = Value::Node(self.builder.gvar( Token::from(yystack.owned_value_at(0))));
                    },


  624 =>  /* string_dvar: "instance variable"  */
  /* "src/parser/parse.y":5800  */
                    {
                        yyval = Value::Node(self.builder.ivar( Token::from(yystack.owned_value_at(0))));

                    },


  625 =>  /* string_dvar: "class variable"  */
  /* "src/parser/parse.y":5805  */
                    {
                        yyval = Value::Node(self.builder.cvar( Token::from(yystack.owned_value_at(0))));
                    },


  626 =>  /* string_dvar: backref  */
  /* "src/parser/parse.y":5809  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  627 =>  /* symbol: ssym  */
  /* "src/parser/parse.y":5814  */
                       { yyval =  yystack.owned_value_at(0); },


  628 =>  /* symbol: dsym  */
  /* "src/parser/parse.y":5815  */
                       { yyval =  yystack.owned_value_at(0); },


  629 =>  /* ssym: "symbol literal" sym  */
  /* "src/parser/parse.y":5819  */
                    {
                        self.yylexer.lex_state.set(EXPR_END);
                        yyval = Value::Node(
                            self.builder.symbol( Token::from(yystack.owned_value_at(1)),  Token::from(yystack.owned_value_at(0)))
                        );
                    },


  630 =>  /* sym: fname  */
  /* "src/parser/parse.y":5827  */
                        { yyval =  yystack.owned_value_at(0); },


  631 =>  /* sym: "instance variable"  */
  /* "src/parser/parse.y":5828  */
                        { yyval =  yystack.owned_value_at(0); },


  632 =>  /* sym: "global variable"  */
  /* "src/parser/parse.y":5829  */
                        { yyval =  yystack.owned_value_at(0); },


  633 =>  /* sym: "class variable"  */
  /* "src/parser/parse.y":5830  */
                        { yyval =  yystack.owned_value_at(0); },


  634 =>  /* dsym: "symbol literal" string_contents "string end"  */
  /* "src/parser/parse.y":5834  */
                    {
                        self.yylexer.lex_state.set(EXPR_END);
                        yyval = Value::Node(
                            self.builder.symbol_compose( Token::from(yystack.owned_value_at(2)),  NodeList::from(yystack.owned_value_at(1)),  Token::from(yystack.owned_value_at(0)))
                        );
                    },


  635 =>  /* numeric: simple_numeric  */
  /* "src/parser/parse.y":5843  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  636 =>  /* numeric: tUMINUS_NUM simple_numeric  */
  /* "src/parser/parse.y":5847  */
                    {
                        yyval = Value::Node(
                            self.builder.unary_num(
                                 Token::from(yystack.owned_value_at(1)),
                                 BoxedNode::from(yystack.owned_value_at(0))
                            )
                        );
                    },


  637 =>  /* simple_numeric: "integer literal"  */
  /* "src/parser/parse.y":5858  */
                    {
                        yyval = Value::Node(
                            self.builder.integer( Token::from(yystack.owned_value_at(0)))
                        );
                    },


  638 =>  /* simple_numeric: "float literal"  */
  /* "src/parser/parse.y":5864  */
                    {
                        yyval = Value::Node(
                            self.builder.float( Token::from(yystack.owned_value_at(0)))
                        );
                    },


  639 =>  /* simple_numeric: "rational literal"  */
  /* "src/parser/parse.y":5870  */
                    {
                        yyval = Value::Node(
                            self.builder.rational( Token::from(yystack.owned_value_at(0)))
                        );
                    },


  640 =>  /* simple_numeric: "imaginary literal"  */
  /* "src/parser/parse.y":5876  */
                    {
                        yyval = Value::Node(
                            self.builder.complex( Token::from(yystack.owned_value_at(0)))
                        );
                    },


  641 =>  /* nonlocal_var: "instance variable"  */
  /* "src/parser/parse.y":5884  */
                    {
                        yyval = Value::Node(
                            self.builder.ivar( Token::from(yystack.owned_value_at(0)))
                        );
                    },


  642 =>  /* nonlocal_var: "global variable"  */
  /* "src/parser/parse.y":5890  */
                    {
                        yyval = Value::Node(
                            self.builder.gvar( Token::from(yystack.owned_value_at(0)))
                        );
                    },


  643 =>  /* nonlocal_var: "class variable"  */
  /* "src/parser/parse.y":5896  */
                    {
                        yyval = Value::Node(
                            self.builder.cvar( Token::from(yystack.owned_value_at(0)))
                        );
                    },


  644 =>  /* user_variable: "local variable or method"  */
  /* "src/parser/parse.y":5904  */
                    {
                        yyval = Value::Node(
                            self.builder.lvar( Token::from(yystack.owned_value_at(0)))
                        );
                    },


  645 =>  /* user_variable: "instance variable"  */
  /* "src/parser/parse.y":5910  */
                    {
                        yyval = Value::Node(
                            self.builder.ivar( Token::from(yystack.owned_value_at(0)))
                        );
                    },


  646 =>  /* user_variable: "global variable"  */
  /* "src/parser/parse.y":5916  */
                    {
                        yyval = Value::Node(
                            self.builder.gvar( Token::from(yystack.owned_value_at(0)))
                        );
                    },


  647 =>  /* user_variable: "constant"  */
  /* "src/parser/parse.y":5922  */
                    {
                        yyval = Value::Node(
                            self.builder.const_( Token::from(yystack.owned_value_at(0)))
                        );
                    },


  648 =>  /* user_variable: "class variable"  */
  /* "src/parser/parse.y":5928  */
                    {
                        yyval = Value::Node(
                            self.builder.cvar( Token::from(yystack.owned_value_at(0)))
                        );
                    },


  649 =>  /* keyword_variable: "`nil'"  */
  /* "src/parser/parse.y":5936  */
                    {
                        yyval = Value::Node(
                            self.builder.nil( Token::from(yystack.owned_value_at(0)))
                        );
                    },


  650 =>  /* keyword_variable: "`self'"  */
  /* "src/parser/parse.y":5942  */
                    {
                        yyval = Value::Node(
                            self.builder.self_( Token::from(yystack.owned_value_at(0)))
                        );
                    },


  651 =>  /* keyword_variable: "`true'"  */
  /* "src/parser/parse.y":5948  */
                    {
                        yyval = Value::Node(
                            self.builder.true_( Token::from(yystack.owned_value_at(0)))
                        );
                    },


  652 =>  /* keyword_variable: "`false'"  */
  /* "src/parser/parse.y":5954  */
                    {
                        yyval = Value::Node(
                            self.builder.false_( Token::from(yystack.owned_value_at(0)))
                        );
                    },


  653 =>  /* keyword_variable: "`__FILE__'"  */
  /* "src/parser/parse.y":5960  */
                    {
                        yyval = Value::Node(
                            self.builder.__file__( Token::from(yystack.owned_value_at(0)))
                        );
                    },


  654 =>  /* keyword_variable: "`__LINE__'"  */
  /* "src/parser/parse.y":5966  */
                    {
                        yyval = Value::Node(
                            self.builder.__line__( Token::from(yystack.owned_value_at(0)))
                        );
                    },


  655 =>  /* keyword_variable: "`__ENCODING__'"  */
  /* "src/parser/parse.y":5972  */
                    {
                        yyval = Value::Node(
                            self.builder.__encoding__( Token::from(yystack.owned_value_at(0)))
                        );
                    },


  656 =>  /* var_ref: user_variable  */
  /* "src/parser/parse.y":5980  */
                    {
                        yyval = Value::Node(
                            self.builder.accessible( BoxedNode::from(yystack.owned_value_at(0)))
                        );
                    },


  657 =>  /* var_ref: keyword_variable  */
  /* "src/parser/parse.y":5986  */
                    {
                        yyval = Value::Node(
                            self.builder.accessible( BoxedNode::from(yystack.owned_value_at(0)))
                        );
                    },


  658 =>  /* var_lhs: user_variable  */
  /* "src/parser/parse.y":5994  */
                    {
                        yyval = Value::Node(
                            self.builder.assignable( BoxedNode::from(yystack.owned_value_at(0)))?
                        );
                    },


  659 =>  /* var_lhs: keyword_variable  */
  /* "src/parser/parse.y":6000  */
                    {
                        yyval = Value::Node(
                            self.builder.assignable( BoxedNode::from(yystack.owned_value_at(0)))?
                        );
                    },


  660 =>  /* backref: "numbered reference"  */
  /* "src/parser/parse.y":6008  */
                    {
                        yyval = Value::Node(
                            self.builder.nth_ref( Token::from(yystack.owned_value_at(0)))
                        );
                    },


  661 =>  /* backref: "back reference"  */
  /* "src/parser/parse.y":6014  */
                    {
                        yyval = Value::Node(
                            self.builder.back_ref( Token::from(yystack.owned_value_at(0)))
                        );
                    },


  662 =>  /* @42: %empty  */
  /* "src/parser/parse.y":6022  */
                    {
                        self.yylexer.lex_state.set(EXPR_BEG);
                        self.yylexer.command_start = true;
                        yyval = Value::None;
                    },


  663 =>  /* superclass: "<" @42 expr_value term  */
  /* "src/parser/parse.y":6028  */
                    {
                        let lt_t  = Some( Token::from(yystack.owned_value_at(3)));
                        let value = Some( BoxedNode::from(yystack.owned_value_at(1)));
                        yyval = Value::new_superclass(
                            Superclass { lt_t, value }
                        );
                    },


  664 =>  /* superclass: %empty  */
  /* "src/parser/parse.y":6036  */
                    {
                        yyval = Value::new_superclass(Superclass { lt_t: None, value: None });
                    },


  665 =>  /* f_opt_paren_args: f_paren_args  */
  /* "src/parser/parse.y":6042  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  666 =>  /* f_opt_paren_args: none  */
  /* "src/parser/parse.y":6046  */
                    {
                        self.context.set_in_argdef(false);
                        yyval = Value::MaybeNode(None);
                    },


  667 =>  /* f_paren_args: "( (tLPAREN2)" f_args rparen  */
  /* "src/parser/parse.y":6053  */
                    {
                        yyval = Value::MaybeNode(
                            self.builder.args(Some( Token::from(yystack.owned_value_at(2))),  NodeList::from(yystack.owned_value_at(1)), Some( Token::from(yystack.owned_value_at(0))))
                        );

                        self.yylexer.lex_state.set(EXPR_BEG);
                        self.yylexer.command_start = true;
                        self.context.set_in_argdef(false);
                    },


  668 =>  /* f_arglist: f_paren_args  */
  /* "src/parser/parse.y":6065  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  669 =>  /* @43: %empty  */
  /* "src/parser/parse.y":6068  */
                    {
                        yyval = Value::Context(self.context.dump());
                        self.context.set_in_kwarg(true);
                        self.context.set_in_argdef(true);
                        self.yylexer.lex_state.set(self.yylexer.lex_state.get()|EXPR_LABEL);
                    },


  670 =>  /* f_arglist: @43 f_args term  */
  /* "src/parser/parse.y":6075  */
                    {
                        self.context.set_in_kwarg( Context::from(yystack.owned_value_at(2)).in_kwarg());
                        self.context.set_in_argdef(false);
                        yyval = Value::MaybeNode(
                            self.builder.args(None,  NodeList::from(yystack.owned_value_at(1)), None)
                        );
                        self.yylexer.lex_state.set(EXPR_BEG);
                        self.yylexer.command_start = true;
                    },


  671 =>  /* args_tail: f_kwarg "," f_kwrest opt_f_block_arg  */
  /* "src/parser/parse.y":6087  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(3));
                        let mut f_kwrest =  NodeList::from(yystack.owned_value_at(1));
                        let mut opt_f_block_arg =  NodeList::from(yystack.owned_value_at(0));

                        nodes.reserve(f_kwrest.len() + opt_f_block_arg.len());
                        nodes.append(&mut f_kwrest);
                        nodes.append(&mut opt_f_block_arg);

                        yyval = Value::NodeList(nodes);
                    },


  672 =>  /* args_tail: f_kwarg opt_f_block_arg  */
  /* "src/parser/parse.y":6099  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(1));
                        nodes.append(&mut  NodeList::from(yystack.owned_value_at(0)));

                        yyval = Value::NodeList(nodes);
                    },


  673 =>  /* args_tail: f_any_kwrest opt_f_block_arg  */
  /* "src/parser/parse.y":6106  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(1));
                        nodes.append(&mut  NodeList::from(yystack.owned_value_at(0)));

                        yyval = Value::NodeList(nodes);
                    },


  674 =>  /* args_tail: f_block_arg  */
  /* "src/parser/parse.y":6113  */
                    {
                        yyval = Value::NodeList(
                            Box::new(
                                vec![  Node::from(yystack.owned_value_at(0)) ]
                            )
                        );
                    },


  675 =>  /* args_tail: args_forward  */
  /* "src/parser/parse.y":6121  */
                    {
                        let forward_arg = *self.builder.forward_arg( Token::from(yystack.owned_value_at(0)));
                        self.static_env.declare_forward_args();
                        yyval = Value::NodeList(
                            Box::new(
                                vec![ forward_arg ]
                            )
                        );
                    },


  676 =>  /* opt_args_tail: "," args_tail  */
  /* "src/parser/parse.y":6133  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  677 =>  /* opt_args_tail: %empty  */
  /* "src/parser/parse.y":6137  */
                    {
                        yyval = Value::NodeList( Box::new(vec![]) );
                    },


  678 =>  /* f_args: f_arg "," f_optarg "," f_rest_arg opt_args_tail  */
  /* "src/parser/parse.y":6143  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(5));
                        let mut f_optarg =  NodeList::from(yystack.owned_value_at(3));
                        let mut f_rest_arg =  NodeList::from(yystack.owned_value_at(1));
                        let mut opt_args_tail =  NodeList::from(yystack.owned_value_at(0));

                        nodes.reserve(f_optarg.len() + f_rest_arg.len() + opt_args_tail.len());
                        nodes.append(&mut f_optarg);
                        nodes.append(&mut f_rest_arg);
                        nodes.append(&mut opt_args_tail);

                        yyval = Value::NodeList(nodes);
                    },


  679 =>  /* f_args: f_arg "," f_optarg "," f_rest_arg "," f_arg opt_args_tail  */
  /* "src/parser/parse.y":6157  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(7));
                        let mut f_optarg =  NodeList::from(yystack.owned_value_at(5));
                        let mut f_rest_arg =  NodeList::from(yystack.owned_value_at(3));
                        let mut f_arg =  NodeList::from(yystack.owned_value_at(1));
                        let mut opt_args_tail =  NodeList::from(yystack.owned_value_at(0));

                        nodes.reserve(f_optarg.len() + f_rest_arg.len() + f_arg.len() + opt_args_tail.len());
                        nodes.append(&mut f_optarg);
                        nodes.append(&mut f_rest_arg);
                        nodes.append(&mut f_arg);
                        nodes.append(&mut opt_args_tail);

                        yyval = Value::NodeList(nodes);
                    },


  680 =>  /* f_args: f_arg "," f_optarg opt_args_tail  */
  /* "src/parser/parse.y":6173  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(3));
                        let mut f_optarg =  NodeList::from(yystack.owned_value_at(1));
                        let mut opt_args_tail =  NodeList::from(yystack.owned_value_at(0));

                        nodes.reserve(f_optarg.len() + opt_args_tail.len());
                        nodes.append(&mut f_optarg);
                        nodes.append(&mut opt_args_tail);

                        yyval = Value::NodeList(nodes);
                    },


  681 =>  /* f_args: f_arg "," f_optarg "," f_arg opt_args_tail  */
  /* "src/parser/parse.y":6185  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(5));
                        let mut f_optarg =  NodeList::from(yystack.owned_value_at(3));
                        let mut f_arg =  NodeList::from(yystack.owned_value_at(1));
                        let mut opt_args_tail =  NodeList::from(yystack.owned_value_at(0));

                        nodes.reserve(f_optarg.len() + f_arg.len() + opt_args_tail.len());
                        nodes.append(&mut f_optarg);
                        nodes.append(&mut f_arg);
                        nodes.append(&mut opt_args_tail);

                        yyval = Value::NodeList(nodes);
                    },


  682 =>  /* f_args: f_arg "," f_rest_arg opt_args_tail  */
  /* "src/parser/parse.y":6199  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(3));
                        let mut f_rest_arg =  NodeList::from(yystack.owned_value_at(1));
                        let mut opt_args_tail =  NodeList::from(yystack.owned_value_at(0));

                        nodes.reserve(f_rest_arg.len() + opt_args_tail.len());
                        nodes.append(&mut f_rest_arg);
                        nodes.append(&mut opt_args_tail);

                        yyval = Value::NodeList(nodes);
                    },


  683 =>  /* f_args: f_arg "," f_rest_arg "," f_arg opt_args_tail  */
  /* "src/parser/parse.y":6211  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(5));
                        let mut f_rest_arg =  NodeList::from(yystack.owned_value_at(3));
                        let mut f_arg =  NodeList::from(yystack.owned_value_at(1));
                        let mut opt_args_tail =  NodeList::from(yystack.owned_value_at(0));

                        nodes.reserve(f_rest_arg.len() + f_arg.len() + opt_args_tail.len());
                        nodes.append(&mut f_rest_arg);
                        nodes.append(&mut f_arg);
                        nodes.append(&mut opt_args_tail);

                        yyval = Value::NodeList(nodes);
                    },


  684 =>  /* f_args: f_arg opt_args_tail  */
  /* "src/parser/parse.y":6225  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(1));
                        nodes.append(&mut  NodeList::from(yystack.owned_value_at(0)));

                        yyval = Value::NodeList(nodes);
                    },


  685 =>  /* f_args: f_optarg "," f_rest_arg opt_args_tail  */
  /* "src/parser/parse.y":6232  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(3));
                        let mut f_rest_arg =  NodeList::from(yystack.owned_value_at(1));
                        let mut opt_args_tail =  NodeList::from(yystack.owned_value_at(0));

                        nodes.reserve(f_rest_arg.len() + opt_args_tail.len());
                        nodes.append(&mut f_rest_arg);
                        nodes.append(&mut opt_args_tail);

                        yyval = Value::NodeList(nodes);
                    },


  686 =>  /* f_args: f_optarg "," f_rest_arg "," f_arg opt_args_tail  */
  /* "src/parser/parse.y":6244  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(5));
                        let mut f_rest_arg =  NodeList::from(yystack.owned_value_at(3));
                        let mut f_arg =  NodeList::from(yystack.owned_value_at(1));
                        let mut opt_args_tail =  NodeList::from(yystack.owned_value_at(0));

                        nodes.reserve(f_rest_arg.len() + f_arg.len() + opt_args_tail.len());
                        nodes.append(&mut f_rest_arg);
                        nodes.append(&mut f_arg);
                        nodes.append(&mut opt_args_tail);

                        yyval = Value::NodeList(nodes);
                    },


  687 =>  /* f_args: f_optarg opt_args_tail  */
  /* "src/parser/parse.y":6258  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(1));
                        nodes.append(&mut  NodeList::from(yystack.owned_value_at(0)));

                        yyval = Value::NodeList(nodes);
                    },


  688 =>  /* f_args: f_optarg "," f_arg opt_args_tail  */
  /* "src/parser/parse.y":6265  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(3));
                        let mut f_arg =  NodeList::from(yystack.owned_value_at(1));
                        let mut opt_args_tail =  NodeList::from(yystack.owned_value_at(0));

                        nodes.reserve(f_arg.len() + opt_args_tail.len());
                        nodes.append(&mut f_arg);
                        nodes.append(&mut opt_args_tail);

                        yyval = Value::NodeList(nodes);
                    },


  689 =>  /* f_args: f_rest_arg opt_args_tail  */
  /* "src/parser/parse.y":6277  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(1));
                        nodes.append(&mut  NodeList::from(yystack.owned_value_at(0)));

                        yyval = Value::NodeList(nodes);
                    },


  690 =>  /* f_args: f_rest_arg "," f_arg opt_args_tail  */
  /* "src/parser/parse.y":6284  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(3));
                        let mut f_arg =  NodeList::from(yystack.owned_value_at(1));
                        let mut opt_args_tail =  NodeList::from(yystack.owned_value_at(0));

                        nodes.reserve(f_arg.len() + opt_args_tail.len());
                        nodes.append(&mut f_arg);
                        nodes.append(&mut opt_args_tail);

                        yyval = Value::NodeList(nodes);
                    },


  691 =>  /* f_args: args_tail  */
  /* "src/parser/parse.y":6296  */
                    {
                        yyval = Value::NodeList( BoxedNodeList::from(yystack.owned_value_at(0)));
                    },


  692 =>  /* f_args: %empty  */
  /* "src/parser/parse.y":6300  */
                    {
                        yyval = Value::NodeList( Box::new(vec![]) );
                    },


  693 =>  /* args_forward: "(..."  */
  /* "src/parser/parse.y":6306  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  694 =>  /* f_bad_arg: "constant"  */
  /* "src/parser/parse.y":6312  */
                    {
                        return self.yyerror(yystack.location_at (0), DiagnosticMessage::ConstArgument {});
                    },


  695 =>  /* f_bad_arg: "instance variable"  */
  /* "src/parser/parse.y":6316  */
                    {
                        return self.yyerror(yystack.location_at (0), DiagnosticMessage::IvarArgument {});
                    },


  696 =>  /* f_bad_arg: "global variable"  */
  /* "src/parser/parse.y":6320  */
                    {
                        return self.yyerror(yystack.location_at (0), DiagnosticMessage::GvarArgument {});
                    },


  697 =>  /* f_bad_arg: "class variable"  */
  /* "src/parser/parse.y":6324  */
                    {
                        return self.yyerror(yystack.location_at (0), DiagnosticMessage::CvarArgument {});
                    },


  698 =>  /* f_norm_arg: f_bad_arg  */
  /* "src/parser/parse.y":6330  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  699 =>  /* f_norm_arg: "local variable or method"  */
  /* "src/parser/parse.y":6334  */
                    {
                        let ident_t =  Token::from(yystack.owned_value_at(0));
                        let name = clone_value(&ident_t);
                        self.static_env.declare(name.as_str());
                        self.max_numparam_stack.set_has_ordinary_params();
                        yyval = Value::Token(ident_t);
                    },


  700 =>  /* f_arg_asgn: f_norm_arg  */
  /* "src/parser/parse.y":6344  */
                    {
                        let arg_t =  Token::from(yystack.owned_value_at(0));
                        let arg_name = clone_value(&arg_t);
                        self.current_arg_stack.set(Some(arg_name));
                        yyval = Value::Token(arg_t);
                    },


  701 =>  /* f_arg_item: f_arg_asgn  */
  /* "src/parser/parse.y":6353  */
                    {
                        self.current_arg_stack.set(None);
                        yyval = Value::Node(
                            self.builder.arg( Token::from(yystack.owned_value_at(0)))?
                        );
                    },


  702 =>  /* f_arg_item: "(" f_margs rparen  */
  /* "src/parser/parse.y":6360  */
                    {
                        yyval = Value::Node(
                            self.builder.multi_lhs(
                                Some( Token::from(yystack.owned_value_at(2))),
                                 NodeList::from(yystack.owned_value_at(1)),
                                Some( Token::from(yystack.owned_value_at(0)))
                            )
                        );
                    },


  703 =>  /* f_arg: f_arg_item  */
  /* "src/parser/parse.y":6372  */
                    {
                        yyval = Value::NodeList( Box::new(vec![  Node::from(yystack.owned_value_at(0)) ]) );
                    },


  704 =>  /* f_arg: f_arg "," f_arg_item  */
  /* "src/parser/parse.y":6376  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(2));
                        nodes.push(  Node::from(yystack.owned_value_at(0)) );
                        yyval = Value::NodeList(nodes);
                    },


  705 =>  /* f_label: "label"  */
  /* "src/parser/parse.y":6385  */
                    {
                        let ident_t =  Token::from(yystack.owned_value_at(0));
                        self.check_kwarg_name(&ident_t)?;

                        let ident = clone_value(&ident_t);
                        self.static_env.declare(&ident);

                        self.max_numparam_stack.set_has_ordinary_params();

                        self.current_arg_stack.set(Some(ident));
                        self.context.set_in_argdef(false);

                        yyval = Value::Token(ident_t);
                    },


  706 =>  /* f_kw: f_label arg_value  */
  /* "src/parser/parse.y":6402  */
                    {
                        self.current_arg_stack.set(None);
                        self.context.set_in_argdef(true);
                        yyval = Value::Node(
                            self.builder.kwoptarg( Token::from(yystack.owned_value_at(1)),  BoxedNode::from(yystack.owned_value_at(0)))?
                        );
                    },


  707 =>  /* f_kw: f_label  */
  /* "src/parser/parse.y":6410  */
                    {
                        self.current_arg_stack.set(None);
                        self.context.set_in_argdef(true);
                        yyval = Value::Node(
                            self.builder.kwarg( Token::from(yystack.owned_value_at(0)))?
                        );
                    },


  708 =>  /* f_block_kw: f_label primary_value  */
  /* "src/parser/parse.y":6420  */
                    {
                        self.context.set_in_argdef(true);
                        yyval = Value::Node(
                            self.builder.kwoptarg( Token::from(yystack.owned_value_at(1)),  BoxedNode::from(yystack.owned_value_at(0)))?
                        );
                    },


  709 =>  /* f_block_kw: f_label  */
  /* "src/parser/parse.y":6427  */
                    {
                        self.context.set_in_argdef(true);
                        yyval = Value::Node(
                            self.builder.kwarg( Token::from(yystack.owned_value_at(0)))?
                        );
                    },


  710 =>  /* f_block_kwarg: f_block_kw  */
  /* "src/parser/parse.y":6436  */
                    {
                        yyval = Value::NodeList( Box::new(vec![  Node::from(yystack.owned_value_at(0)) ]) );
                    },


  711 =>  /* f_block_kwarg: f_block_kwarg "," f_block_kw  */
  /* "src/parser/parse.y":6440  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(2));
                        nodes.push(  Node::from(yystack.owned_value_at(0)) );
                        yyval = Value::NodeList(nodes);
                    },


  712 =>  /* f_kwarg: f_kw  */
  /* "src/parser/parse.y":6449  */
                    {
                        yyval = Value::NodeList( Box::new(vec![  Node::from(yystack.owned_value_at(0)) ]) );
                    },


  713 =>  /* f_kwarg: f_kwarg "," f_kw  */
  /* "src/parser/parse.y":6453  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(2));
                        nodes.push(  Node::from(yystack.owned_value_at(0)) );
                        yyval = Value::NodeList(nodes);
                    },


  714 =>  /* kwrest_mark: "**"  */
  /* "src/parser/parse.y":6461  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  715 =>  /* kwrest_mark: "**arg"  */
  /* "src/parser/parse.y":6465  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  716 =>  /* f_no_kwarg: p_kwnorest  */
  /* "src/parser/parse.y":6471  */
                    {
                        let NoKwRest { kwrest_mark, k_nil } =  NoKwRest::from(yystack.owned_value_at(0));
                        yyval = Value::NodeList(
                            Box::new(
                                vec![
                                    *self.builder.kwnilarg(
                                        kwrest_mark,
                                        k_nil
                                    )
                                ]
                            )
                        );
                    },


  717 =>  /* f_kwrest: kwrest_mark "local variable or method"  */
  /* "src/parser/parse.y":6487  */
                    {
                        let ident_t =  Token::from(yystack.owned_value_at(0));
                        self.static_env.declare(clone_value(&ident_t).as_str());
                        yyval = Value::NodeList(
                            Box::new(
                                vec![
                                    *(self.builder.kwrestarg( Token::from(yystack.owned_value_at(1)), Some(ident_t))?)
                                ]
                            )
                        );
                    },


  718 =>  /* f_kwrest: kwrest_mark  */
  /* "src/parser/parse.y":6499  */
                    {
                        yyval = Value::NodeList(
                            Box::new(
                                vec![
                                    *(self.builder.kwrestarg( Token::from(yystack.owned_value_at(0)), None)?)
                                ]
                            )
                        );
                    },


  719 =>  /* f_opt: f_arg_asgn f_eq arg_value  */
  /* "src/parser/parse.y":6511  */
                    {
                        self.current_arg_stack.set(None);
                        self.context.set_in_argdef(true);
                        yyval = Value::Node(
                            self.builder.optarg(
                                 Token::from(yystack.owned_value_at(2)),
                                 Token::from(yystack.owned_value_at(1)),
                                 BoxedNode::from(yystack.owned_value_at(0))
                            )?
                        );
                    },


  720 =>  /* f_block_opt: f_arg_asgn f_eq primary_value  */
  /* "src/parser/parse.y":6525  */
                    {
                        self.current_arg_stack.set(None);
                        self.context.set_in_argdef(true);
                        yyval = Value::Node(
                            self.builder.optarg(
                                 Token::from(yystack.owned_value_at(2)),
                                 Token::from(yystack.owned_value_at(1)),
                                 BoxedNode::from(yystack.owned_value_at(0))
                            )?
                        );
                    },


  721 =>  /* f_block_optarg: f_block_opt  */
  /* "src/parser/parse.y":6539  */
                    {
                        yyval = Value::NodeList( Box::new(vec![  Node::from(yystack.owned_value_at(0)) ]) );
                    },


  722 =>  /* f_block_optarg: f_block_optarg "," f_block_opt  */
  /* "src/parser/parse.y":6543  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(2));
                        nodes.push(  Node::from(yystack.owned_value_at(0)) );
                        yyval = Value::NodeList(nodes);
                    },


  723 =>  /* f_optarg: f_opt  */
  /* "src/parser/parse.y":6551  */
                    {
                        yyval = Value::NodeList( Box::new(vec![  Node::from(yystack.owned_value_at(0)) ]) );
                    },


  724 =>  /* f_optarg: f_optarg "," f_opt  */
  /* "src/parser/parse.y":6555  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(2));
                        nodes.push(  Node::from(yystack.owned_value_at(0)) );
                        yyval = Value::NodeList(nodes);
                    },


  725 =>  /* restarg_mark: "* (tSTAR2)"  */
  /* "src/parser/parse.y":6563  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  726 =>  /* restarg_mark: "*"  */
  /* "src/parser/parse.y":6567  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  727 =>  /* f_rest_arg: restarg_mark "local variable or method"  */
  /* "src/parser/parse.y":6573  */
                    {
                        let ident_t =  Token::from(yystack.owned_value_at(0));
                        self.static_env.declare(clone_value(&ident_t).as_str());

                        yyval = Value::NodeList(
                            Box::new(
                                vec![
                                    *(self.builder.restarg( Token::from(yystack.owned_value_at(1)), Some(ident_t))?)
                                ]
                            )
                        );
                    },


  728 =>  /* f_rest_arg: restarg_mark  */
  /* "src/parser/parse.y":6586  */
                    {
                        yyval = Value::NodeList(
                            Box::new(
                                vec![
                                    *(self.builder.restarg( Token::from(yystack.owned_value_at(0)), None)?)
                                ]
                            )
                        );
                    },


  729 =>  /* blkarg_mark: "& (tAMPER2)"  */
  /* "src/parser/parse.y":6598  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  730 =>  /* blkarg_mark: "&"  */
  /* "src/parser/parse.y":6602  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  731 =>  /* f_block_arg: blkarg_mark "local variable or method"  */
  /* "src/parser/parse.y":6608  */
                    {
                        let ident_t =  Token::from(yystack.owned_value_at(0));
                        self.static_env.declare(clone_value(&ident_t).as_str());
                        yyval = Value::Node(
                            self.builder.blockarg(
                                 Token::from(yystack.owned_value_at(1)),
                                Some(ident_t),
                            )?
                        );
                    },


  732 =>  /* f_block_arg: blkarg_mark  */
  /* "src/parser/parse.y":6619  */
                    {
                        self.static_env.declare_anonymous_blockarg();
                        yyval = Value::Node(
                            self.builder.blockarg(
                                 Token::from(yystack.owned_value_at(0)),
                                None
                            )?
                        );
                    },


  733 =>  /* opt_f_block_arg: "," f_block_arg  */
  /* "src/parser/parse.y":6631  */
                    {
                        yyval = Value::NodeList( Box::new(vec![  Node::from(yystack.owned_value_at(0)) ]) );
                    },


  734 =>  /* opt_f_block_arg: none  */
  /* "src/parser/parse.y":6635  */
                    {
                        yyval = Value::NodeList( Box::new(vec![]) );
                    },


  735 =>  /* singleton: var_ref  */
  /* "src/parser/parse.y":6641  */
                    {
                        let var_ref =  BoxedNode::from(yystack.owned_value_at(0));
                        self.value_expr(&var_ref)?;
                        yyval = Value::Node(var_ref);
                    },


  736 =>  /* @44: %empty  */
  /* "src/parser/parse.y":6646  */
                           { self.yylexer.lex_state.set(EXPR_BEG); yyval = Value::None; },


  737 =>  /* singleton: "( (tLPAREN2)" @44 expr rparen  */
  /* "src/parser/parse.y":6647  */
                    {
                        let expr =  BoxedNode::from(yystack.owned_value_at(1));

                        match &*expr {
                            Node::Int(nodes::Int { expression_l, .. })
                            | Node::Float(nodes::Float { expression_l, .. })
                            | Node::Rational(nodes::Rational { expression_l, .. })
                            | Node::Complex(nodes::Complex { expression_l, .. })
                            | Node::Str(nodes::Str { expression_l, .. })
                            | Node::Dstr(nodes::Dstr { expression_l, .. })
                            | Node::Sym(nodes::Sym { expression_l, .. })
                            | Node::Dsym(nodes::Dsym { expression_l, .. })
                            | Node::Heredoc(nodes::Heredoc { expression_l, .. })
                            | Node::XHeredoc(nodes::XHeredoc { expression_l, .. })
                            | Node::Regexp(nodes::Regexp { expression_l, .. })
                            | Node::Array(nodes::Array { expression_l, .. })
                            | Node::Hash(nodes::Hash { expression_l, .. }) => {
                                self.yyerror1(
                                    DiagnosticMessage::SingletonLiteral {},
                                    *expression_l,
                                )?;
                            }
                            other => {
                                self.value_expr(other)?
                            }
                        }

                        yyval = Value::Node(expr);
                    },


  738 =>  /* assoc_list: none  */
  /* "src/parser/parse.y":6679  */
                    {
                        yyval = Value::NodeList( Box::new(vec![]) );
                    },


  739 =>  /* assoc_list: assocs trailer  */
  /* "src/parser/parse.y":6683  */
                    {
                        yyval =  yystack.owned_value_at(1);
                    },


  740 =>  /* assocs: assoc  */
  /* "src/parser/parse.y":6689  */
                    {
                        yyval = Value::NodeList( Box::new(vec![  Node::from(yystack.owned_value_at(0)) ]) );
                    },


  741 =>  /* assocs: assocs "," assoc  */
  /* "src/parser/parse.y":6693  */
                    {
                        let mut nodes =  BoxedNodeList::from(yystack.owned_value_at(2));
                        nodes.push( Node::from(yystack.owned_value_at(0)));
                        yyval = Value::NodeList(nodes);
                    },


  742 =>  /* assoc: arg_value "=>" arg_value  */
  /* "src/parser/parse.y":6701  */
                    {
                        yyval = Value::Node(
                            self.builder.pair(
                                 BoxedNode::from(yystack.owned_value_at(2)),
                                 Token::from(yystack.owned_value_at(1)),
                                 BoxedNode::from(yystack.owned_value_at(0))
                            )
                        );
                    },


  743 =>  /* assoc: "label" arg_value  */
  /* "src/parser/parse.y":6711  */
                    {
                        yyval = Value::Node(
                            self.builder.pair_keyword(
                                 Token::from(yystack.owned_value_at(1)),
                                 BoxedNode::from(yystack.owned_value_at(0))
                            )
                        );
                    },


  744 =>  /* assoc: "label"  */
  /* "src/parser/parse.y":6720  */
                    {
                        yyval = Value::Node(
                            self.builder.pair_label( Token::from(yystack.owned_value_at(0)))
                        );
                    },


  745 =>  /* assoc: "string begin" string_contents tLABEL_END arg_value  */
  /* "src/parser/parse.y":6726  */
                    {
                        yyval = Value::Node(
                            self.builder.pair_quoted(
                                 Token::from(yystack.owned_value_at(3)),
                                 NodeList::from(yystack.owned_value_at(2)),
                                 Token::from(yystack.owned_value_at(1)),
                                 BoxedNode::from(yystack.owned_value_at(0))
                            )
                        );
                    },


  746 =>  /* assoc: "**arg" arg_value  */
  /* "src/parser/parse.y":6737  */
                    {
                        yyval = Value::Node(
                            self.builder.kwsplat( Token::from(yystack.owned_value_at(1)),  BoxedNode::from(yystack.owned_value_at(0)))
                        );
                    },


  747 =>  /* operation: "local variable or method"  */
  /* "src/parser/parse.y":6745  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  748 =>  /* operation: "constant"  */
  /* "src/parser/parse.y":6749  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  749 =>  /* operation: "method"  */
  /* "src/parser/parse.y":6753  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  750 =>  /* operation2: operation  */
  /* "src/parser/parse.y":6759  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  751 =>  /* operation2: op  */
  /* "src/parser/parse.y":6763  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  752 =>  /* operation3: "local variable or method"  */
  /* "src/parser/parse.y":6769  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  753 =>  /* operation3: "method"  */
  /* "src/parser/parse.y":6773  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  754 =>  /* operation3: op  */
  /* "src/parser/parse.y":6777  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  755 =>  /* dot_or_colon: tDOT  */
  /* "src/parser/parse.y":6783  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  756 =>  /* dot_or_colon: "::"  */
  /* "src/parser/parse.y":6787  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  757 =>  /* call_op: tDOT  */
  /* "src/parser/parse.y":6793  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  758 =>  /* call_op: "&."  */
  /* "src/parser/parse.y":6797  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  759 =>  /* call_op2: call_op  */
  /* "src/parser/parse.y":6803  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  760 =>  /* call_op2: "::"  */
  /* "src/parser/parse.y":6807  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  761 =>  /* opt_terms: %empty  */
  /* "src/parser/parse.y":6813  */
                    {
                        yyval = Value::None;
                    },


  762 =>  /* opt_terms: terms  */
  /* "src/parser/parse.y":6817  */
                    {
                        yyval = Value::None;
                    },


  763 =>  /* opt_nl: %empty  */
  /* "src/parser/parse.y":6823  */
                    {
                        yyval = Value::None;
                    },


  764 =>  /* opt_nl: "\n"  */
  /* "src/parser/parse.y":6827  */
                    {
                        yyval = Value::None;
                    },


  765 =>  /* rparen: opt_nl ")"  */
  /* "src/parser/parse.y":6833  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  766 =>  /* rbracket: opt_nl "]"  */
  /* "src/parser/parse.y":6839  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  767 =>  /* rbrace: opt_nl "}"  */
  /* "src/parser/parse.y":6845  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  768 =>  /* trailer: %empty  */
  /* "src/parser/parse.y":6851  */
                    {
                        yyval = Value::None;
                    },


  769 =>  /* trailer: "\n"  */
  /* "src/parser/parse.y":6855  */
                    {
                        yyval = Value::None;
                    },


  770 =>  /* trailer: ","  */
  /* "src/parser/parse.y":6859  */
                    {
                        yyval = Value::None;
                    },


  771 =>  /* term: ";"  */
  /* "src/parser/parse.y":6865  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  772 =>  /* term: "\n"  */
  /* "src/parser/parse.y":6869  */
                    {
                        yyval =  yystack.owned_value_at(0);
                    },


  773 =>  /* terms: term  */
  /* "src/parser/parse.y":6875  */
                    {
                        yyval = Value::TokenList( Box::new(vec![]) );
                    },


  774 =>  /* terms: terms ";"  */
  /* "src/parser/parse.y":6879  */
                    {
                        yyval = Value::TokenList( Box::new(vec![]) );
                    },


  775 =>  /* none: %empty  */
  /* "src/parser/parse.y":6885  */
                  {
                        yyval = Value::None;
                  },



/* "src/parser/parse.rs":11356  */

            _ => {}
        }

        assert!(
            !yyval.is_uninitialized(),
            "yyval is Uninitialized in rule at line {}",
            Self::yyrline_[i32_to_usize(yyn)],
        );

        self.yy_symbol_print("-> $$ =", SymbolKind::get(Self::yyr1_[i32_to_usize(yyn)]), &yyval, &yyloc);

        yystack.pop_n(*yylen);
        *yylen = 0;
        /* Shift the result of the reduction.  */
        let yystate = self.yy_lr_goto_state(yystack.state_at(0), Self::yyr1_[i32_to_usize(yyn)]);
        yystack.push(yystate, yyval, yyloc);
        Ok(Self::YYNEWSTATE)
    }

    // Print this symbol on YYOUTPUT.
    fn yy_symbol_print(&self, s: &str, yykind: &SymbolKind, yyvalue: &YYValue, yylocation: &YYLoc) {
        if  self.is_debug()  {
            self.yycdebug(
                &format!("{}{} {:?} ( {:?}: {:?} )", // " fix highlighting
                s,
                if yykind.code() < Self::YYNTOKENS_ { " token " } else { " nterm " },
                yykind.name(),
                yylocation,
                yyvalue
                )
            )
        }
    }

    /// Parses given input. Returns true if the parsing was successful.
    #[allow(clippy::manual_range_contains)]
    pub fn parse(&mut self) -> bool {
        /* @$.  */
        let mut yyloc: YYLoc;
        
    /* Lookahead token kind.  */
    let mut yychar: i32 = Self::YYEMPTY_;
    /* Lookahead symbol kind.  */
    let mut yytoken = &DYMMY_SYMBOL_KIND;

    /* State.  */
    let mut yyn: i32 = 0;
    let mut yylen: usize = 0;
    let mut yystate: i32 = 0;
    let mut yystack = YYStack::new();
    let mut label: i32 = Self::YYNEWSTATE;

    /* The location where the error started.  */
    let mut yyerrloc: YYLoc = YYLoc { begin: 0, end: 0 };

    /* Location. */
    let mut yylloc: YYLoc = YYLoc { begin: 0, end: 0 };

    /* Semantic value of the lookahead.  */
    let mut yylval: YYValue = YYValue::new_uninitialized();

        self.yycdebug("Starting parse");
        self.yyerrstatus_ = 0;
        self.yynerrs = 0;

        /* Initialize the stack.  */
        yystack.push(yystate, yylval.clone(), yylloc);

        loop {
            match label {
                // New state.  Unlike in the C/C++ skeletons, the state is already
                // pushed when we come here.

                Self::YYNEWSTATE => {
                    if  self.is_debug()  {
                        self.yycdebug(&format!("Entering state {}", yystate));
                        eprintln!("{}", yystack);
                    }

                    /* Accept? */
                    if yystate == Self::YYFINAL_ {
                        return true;
                    }

                    /* Take a decision.  First try without lookahead.  */
                    yyn = Self::yypact_[i32_to_usize(yystate)];
                    if yy_pact_value_is_default(yyn) {
                        label = Self::YYDEFAULT;
                        continue;
                    }

                    /* Read a lookahead token.  */
                    if yychar == Self::YYEMPTY_ {
                        self.yycdebug("Reading a token");
                        let token = self.next_token();
                        yychar = token.token_type;
                        yylloc = token.loc;
                        yylval = YYValue::from_token(token);
                    }

                    /* Convert token to internal form.  */
                    yytoken = Self::yytranslate_(yychar);
                    self.yy_symbol_print("Next token is", yytoken, &yylval, &yylloc);

                    if yytoken == SymbolKind::get(1) {
                        // The scanner already issued an error message, process directly
                        // to error recovery.  But do not keep the error token as
                        // lookahead, it is too special and may lead us to an endless
                        // loop in error recovery. */
                        yychar = Lexer::YYUNDEF;
                        yytoken = SymbolKind::get(2);
                        yyerrloc = yylloc;
                        label = Self::YYERRLAB1;
                    } else {
                        // If the proper action on seeing token YYTOKEN is to reduce or to
                        // detect an error, take that action.
                        yyn += yytoken.code();
                        if yyn < 0 || Self::YYLAST_ < yyn || Self::yycheck_[i32_to_usize(yyn)] != yytoken.code() {
                            label = Self::YYDEFAULT;
                        }

                        /* <= 0 means reduce or error.  */
                        else {
                            yyn = Self::yytable_[i32_to_usize(yyn)];
                            if yyn <= 0 {
                                if yy_table_value_is_error(yyn) {
                                    label = Self::YYERRLAB;
                                } else {
                                    yyn = -yyn;
                                    label = Self::YYREDUCE;
                                }
                            } else {
                                /* Shift the lookahead token.  */
                                self.yy_symbol_print("Shifting", yytoken, &yylval, &yylloc);

                                /* Discard the token being shifted.  */
                                yychar = Self::YYEMPTY_;

                                /* Count tokens shifted since error; after three, turn off error status.  */
                                if self.yyerrstatus_ > 0 {
                                    self.yyerrstatus_ -= 1;
                                }

                                yystate = yyn;
                                yystack.push(yystate, std::mem::take(&mut yylval), std::mem::take(&mut yylloc));
                                label = Self::YYNEWSTATE;
                            }
                        }
                    }
                    continue;
                }, // YYNEWSTATE

                // yydefault -- do the default action for the current state.
                Self::YYDEFAULT => {
                    yyn = Self::yydefact_[i32_to_usize(yystate)];
                    if yyn == 0 {
                        label = Self::YYERRLAB;
                    } else {
                        label = Self::YYREDUCE;
                    }
                    continue;
                } // YYDEFAULT

                // yyreduce -- Do a reduction.
                Self::YYREDUCE => {
                    yylen = i32_to_usize(Self::yyr2_[i32_to_usize(yyn)]);
                    label = match self.yyaction(yyn, &mut yystack, &mut yylen) {
                        Ok(label) => label,
                        Err(_) => Self::YYERROR
                    };
                    yystate = yystack.state_at(0);
                    continue;
                }, // YYREDUCE

                // yyerrlab -- here on detecting error
                Self::YYERRLAB => {
                    /* If not already recovering from an error, report this error.  */
                    if self.yyerrstatus_ == 0 {
                        self.yynerrs += 1;
                        if yychar == Self::YYEMPTY_ {
                            yytoken = &DYMMY_SYMBOL_KIND;
                        }
                        self.report_syntax_error(&yystack, yytoken, yylloc);
                    }
                    yyerrloc = yylloc;
                    if self.yyerrstatus_ == 3 {
                        // If just tried and failed to reuse lookahead token after an error, discard it.

                        if yychar <= Lexer::END_OF_INPUT {
                            /* Return failure if at end of input.  */
                            if yychar == Lexer::END_OF_INPUT {
                                return false;
                            }
                        }
                        else {
                            yychar = Self::YYEMPTY_;
                        }
                    }

                    // Else will try to reuse lookahead token after shifting the error token.
                    label = Self::YYERRLAB1;
                    continue;
                }, // YYERRLAB

                // errorlab -- error raised explicitly by YYERROR.
                Self::YYERROR => {
                    /* Do not reclaim the symbols of the rule which action triggered
                    this YYERROR.  */
                    yystack.pop_n(yylen);
                    yylen = 0;
                    yystate = yystack.state_at(0);
                    label = Self::YYERRLAB1;
                    continue;
                }, // YYERROR

                // yyerrlab1 -- common code for both syntax error and YYERROR.
                Self::YYERRLAB1 => {
                    self.yyerrstatus_ = 3;       /* Each real token shifted decrements this.  */

                    // Pop stack until we find a state that shifts the error token.
                    loop {
                        yyn = Self::yypact_[i32_to_usize(yystate)];
                        if !yy_pact_value_is_default(yyn) {
                            yyn += SymbolKind { value: SymbolKind::S_YYerror }.code();
                            if (0..=Self::YYLAST_).contains(&yyn) {
                                let yyn_usize = i32_to_usize(yyn);
                                if Self::yycheck_[yyn_usize] == SymbolKind::S_YYerror {
                                    yyn = Self::yytable_[yyn_usize];
                                    if 0 < yyn {
                                        break;
                                    }
                                }
                            }
                        }

                        // Pop the current state because it cannot handle the error token.
                        if yystack.len() == 1 {
                            return false;
                        }

                        yyerrloc = *yystack.location_at(0);
                        yystack.pop();
                        yystate = yystack.state_at(0);
                        if  self.is_debug()  {
                            eprintln!("{}", yystack);
                        }
                    }

                    if label == Self::YYABORT {
                        /* Leave the switch.  */
                        continue;
                    }

                    /* Muck with the stack to setup for yylloc.  */
                    yystack.push(0, YYValue::new_uninitialized(), yylloc);
                    yystack.push(0, YYValue::new_uninitialized(), yyerrloc);
                    yyloc = make_yylloc(&yystack, 2);
                    yystack.pop_n(2);

                    /* Shift the error token.  */
                    self.yy_symbol_print("Shifting", SymbolKind::get(Self::yystos_[i32_to_usize(yyn)]), &yylval, &yyloc);

                    yystate = yyn;
                    yystack.push(yyn, yylval.clone(), yyloc);
                    label = Self::YYNEWSTATE;
                    continue;
                }, // YYERRLAB1

                // Accept
                Self::YYACCEPT => {
                    return true;
                }

                // Abort.
                Self::YYABORT => {
                    return false;
                },

                _ => {
                    panic!("internal bison error: unknown label {}", label);
                }
            }
        }
    }
}

// Whether the given `yypact_` value indicates a defaulted state.
fn yy_pact_value_is_default(yyvalue: i32) -> bool {
    yyvalue == YYPACT_NINF_
}

// Whether the given `yytable_`
// value indicates a syntax error.
// yyvalue: the value to check
fn yy_table_value_is_error(yyvalue: i32) -> bool {
    yyvalue == YYTABLE_NINF_
}

const YYPACT_NINF_: i32 = -1039;
const YYTABLE_NINF_: i32 = -776;

impl  Parser  {

/* YYPACT[STATE-NUM] -- Index in YYTABLE of the portion describing
   STATE-NUM.  */
  #[allow(non_upper_case_globals)]
const yypact_: &'static [i32] = &[  -1039,   124,  4301, -1039,  8635, -1039, -1039, -1039, 12927, -1039,
   -1039, -1039, -1039, -1039, -1039, -1039,  8910,  8910, -1039, -1039,
   -1039,  7630,  7338, -1039, -1039, -1039, -1039,   579, 12782,    24,
      72,   106, -1039, -1039, -1039,  6462,  7192, -1039, -1039,  6608,
   -1039, -1039, -1039, -1039, -1039, -1039, -1039, -1039, 11104, 11104,
   11104, 11104,    94,  5491,  9047,  9595, 10006,  4455, -1039, 12637,
   -1039, -1039, -1039,   134,   145,   197,   256, 11104, 11241,  1173,
   -1039,   520, -1039,  1294, -1039,   484,   617,   617, -1039, -1039,
     176,   213,   314, -1039,   290, 11515, -1039,   334,  3059,   465,
     569,   641, -1039, 11378, 11378, -1039, -1039, 10143, 11637, 11759,
   12004, 12492,  8910, -1039,   839,    60, -1039, -1039,   444, -1039,
   -1039, -1039, -1039, -1039, -1039, -1039, -1039, -1039, -1039, -1039,
   -1039,   292,   303, -1039,   533,   593, -1039, -1039, -1039, -1039,
   -1039, -1039, -1039, -1039, -1039, -1039, -1039, -1039, -1039, -1039,
   -1039, -1039, -1039, -1039, -1039, -1039, -1039, -1039, -1039, -1039,
   -1039, -1039, -1039, -1039, -1039, -1039, -1039, -1039, -1039, -1039,
   -1039, -1039, -1039, -1039, -1039, -1039, -1039, -1039, -1039, -1039,
   -1039, -1039, -1039, -1039, -1039, -1039, -1039, -1039, -1039, -1039,
   -1039, -1039, -1039, -1039, -1039, -1039, -1039, -1039, -1039, -1039,
   -1039, -1039, -1039, -1039, -1039, -1039, -1039, -1039, -1039, -1039,
   -1039, -1039, -1039, -1039, -1039,   359, -1039, -1039, -1039,   448,
   11104,   544,  5629, 11104, 11104, 11104, -1039, 11104,   617,   617,
   -1039,   511, 14388,   576, -1039, -1039,   595,   741,     6,    40,
     605,    43,   643, -1039, -1039,  8772, -1039,  8910,  9184, -1039,
   -1039, 10281, -1039, 11378,   510, -1039,   606,  5767, -1039,  5905,
   -1039, -1039,   630,   675,   176, -1039,   748, -1039,   700, 14461,
   14461,   705, -1039, -1039,  5491,   676,   520, -1039,  1294,    24,
     672, -1039,  1294,    24,   682,    27,   483, -1039,   576,   708,
     483, -1039,    24,   803, 12126,  1173,   715,   715,   743, -1039,
     768,   795,   814,   844, -1039, -1039, -1039, -1039, -1039,   464,
   -1039,   720,   862,   650, -1039, -1039, -1039, -1039, -1039, -1039,
   -1039,   789, -1039, -1039, -1039, -1039,  8361, 11378, 11378, 11378,
   11378,  9047, 11378, 11378,  1440,   765,   773,  4639,  1440, -1039,
     785,  4639, -1039, -1039, -1039,   818, -1039, -1039, -1039, -1039,
   -1039, 10419, -1039,  5491, 11881,   792, 10419, -1039, 11104, 11104,
   11104, 11104, 11104, -1039, -1039, 11104, 11104, 11104, 11104, 11104,
   11104, 11104, 11104, -1039, 11104, 11104, -1039, -1039, 11104, 11104,
   11104, 11104, 11104, 11104, 11104, 11104, 11104, -1039, -1039, 13261,
    8910, 13334,  4639,   484,   104,   104,  6043, 11378,  6043,   520,
   -1039,   782,   878, -1039, -1039,   983,   921,    89,   146,   286,
     830,   899, 11378,    91, -1039,   816,  1008, -1039, -1039, -1039,
   -1039,   233,   282,   284,   385,   442,   642,   731,   825,   833,
   -1039, -1039, -1039, -1039,   847, -1039, -1039, -1039, 14356, -1039,
   -1039, 11241, 11241, -1039, -1039,    50, -1039, -1039, -1039,   766,
     822,   828, 11104, 11104,  9321, -1039, -1039, 13407,  8910, 13480,
   11104, 11104,  9732, -1039,    24,   851, -1039, -1039, 11104,    24,
   -1039,   853,    24,   868, -1039,    75, -1039, -1039, -1039, -1039,
   -1039, 12927, -1039, 11104,   842,   879, 13407, 13480, 11104,    72,
      24, -1039, -1039,  8498,   888,    24, -1039, -1039,  9869, -1039,
   -1039, 10006, -1039, -1039, -1039,   606,  1035, -1039,   895, -1039,
   -1039, 12126, 13553,  8910, 13626,  1268, -1039, -1039, -1039, -1039,
   -1039, -1039, -1039, -1039, -1039, -1039,   866,    36,   897,    71,
   11104, -1039, -1039, -1039, -1039, -1039, -1039, -1039, -1039, -1039,
   -1039, -1039, -1039, -1039, -1039, -1039, -1039, -1039,  1308, -1039,
   -1039, -1039, -1039, -1039,   901, -1039, -1039,    24, -1039, -1039,
   -1039,   900, -1039,   903, 11104, -1039,   908,   102, -1039, -1039,
   -1039,   913,   970,   916,   980, -1039, 11241,   963,  1034,   520,
   11241,   963, -1039, -1039, -1039, -1039, -1039, 11104, -1039,   928,
     930,  1012, -1039,    24, 12126,   932, -1039, -1039, -1039,  1017,
     936,  3757, -1039, -1039, -1039,  1038,   271,  3665,   700,  4153,
    4153,  4153,  4153,  4630,  3201,  4153,  4153, 14461, 14461,   616,
     616,  3665,  1174,  1478,  1174,   435,   435,   700,   700,   700,
    3086,  1694,  1694,  7776,  6754,  8068,  6900, -1039, -1039,   675,
   -1039,    24,   947,   804, -1039,   846, -1039, -1039,  7484,   963,
   -1039,  4790,  1070,  5204,   963,    93,   963,  1059,  1069,   340,
   13699,  8910, 13772, -1039,   484, -1039,  1035, -1039, -1039, -1039,
   13845,  8910, 13918,  4639, 11378, -1039, -1039, -1039, -1039, -1039,
    2736, -1039,  3317, -1039, -1039, -1039, 12927, 11104, 11104, 11104,
   -1039, 11104,   576, -1039,   643,  6316,  7046,    24,   294,   306,
   -1039, -1039, -1039, -1039,  9458, -1039,  9732, -1039, -1039, 11378,
   14388, -1039, -1039,   675,   675, -1039, -1039,   688, -1039, -1039,
     483, 12126,   895,   452,   639,    24,   566,   571,  1440, -1039,
   -1039, -1039,  1150, -1039,   480, -1039,   951, -1039, -1039,   506,
     959, -1039,   700,  1308,  1275, -1039,   974,    24,   976, -1039,
     149, -1039, -1039, -1039, 11104,   979,  1440, -1039, -1039,   406,
   -1039, -1039, -1039,  1440, -1039, -1039,  1740, -1039, -1039,  1066,
    4120, -1039, -1039, -1039, 10556,   324, -1039, -1039,  1076, 13177,
   -1039,   997,   996, -1039,   963,   996,   963,  1002, 10693,  9047,
   -1039,   895, 12126,  9047, 11241, 11104, 13991,  8910, 14064, -1039,
   -1039, -1039,  2470,  2470,   801, -1039,  3442,    47,  1078, -1039,
     673, -1039, -1039,   151, -1039,  1003, -1039, -1039, -1039,  1013,
   -1039,  1018, -1039, 13135, -1039, -1039, -1039, -1039,   811, -1039,
   -1039, -1039,   258, -1039, -1039, -1039, -1039, -1039, -1039, -1039,
   -1039, -1039,   186, -1039,   987, 11241, 11241, -1039,   818,  1009,
    1098,  9321, 11241, 11241, -1039, -1039,   818, -1039, -1039,   869,
   -1039,  1124, -1039, -1039, -1039, -1039, -1039, -1039,  1069,   963,
   -1039, 10830,   963,    59,   309,    24,   343,   372,  6043,   520,
   11378,  4639,   966,   639, -1039,    24,   963,    75, 13072,    60,
     213, -1039, -1039, -1039, -1039, 11104, 11104,   491, 11104, 11104,
      24,  1028,    75, -1039, -1039,   574,    24,    70, -1039, -1039,
   -1039, -1039, -1039, -1039, -1039, -1039, -1039, -1039, -1039,    24,
   -1039,  1308, -1039,  1186, -1039, -1039, -1039, -1039, -1039,  1031,
    1036, -1039,  1113,   901,  1042, -1039,  1043, -1039,  1042, 11104,
   11104,   928, -1039,  1071, -1039, -1039, -1039,  6043,  5353, -1039,
   -1039, 11104, 11104, -1039,  2257,  5905, -1039, -1039,  1046, -1039,
    4639, -1039, 11104,  1049, -1039,   895, -1039, 14388,  7922,  8214,
      24,   665,   689, -1039, -1039, -1039, -1039, 13135,   416,    24,
    4028, -1039,    24,  1052, -1039,   459,  1053, -1039, -1039,   893,
   -1039, -1039, -1039, -1039, 11378, -1039,  1129,  4028, 13135, 13135,
     459,  1084,  2470,  2470,   801,   561,    22,  3665,  3665, -1039,
   11104, -1039, -1039, -1039, -1039, -1039, -1039, 11241, -1039, -1039,
   -1039, -1039, -1039, -1039,  6043, 11378,   963, -1039, -1039,   963,
    3665,   963, -1039, 11104, -1039,   150, -1039,   374,   963,  4639,
     520,   963, -1039, -1039, -1039, -1039, -1039, -1039, 11104, -1039,
    9732, -1039,  1083,  1058, -1039, -1039, -1039, -1039, -1039, -1039,
    1065,  1077,  1440, -1039,  1740, -1039, -1039,  1740, -1039,  1740,
   -1039, -1039, 14388, 14388, 12248,   104, -1039, -1039,  1182, 14388,
   14388,   901, -1039,    24,  1089,   900,  1095, 12370, -1039,  1097,
   -1039,  1099,  1112, -1039, -1039, -1039, -1039,  1119,   711,     4,
   -1039,  1084,  1125,  1127, -1039, -1039, -1039,    24, -1039, -1039,
    1146, -1039, -1039,  1131, -1039,  1133, -1039, -1039, -1039, -1039,
   -1039, -1039, -1039, -1039,    24,    24,    24,    24,    24,    24,
   14388, -1039, -1039, -1039,   104, -1039, -1039,   104,   960, -1039,
   -1039, 10967,  4928, -1039,   963, -1039, -1039, -1039,  1367,  4639,
    5905, -1039,  1186,  1186,  1042,  1134,  1042,  1042,  1198, -1039,
    1044,   160,   200,   237,  4639, -1039, -1039,  1135, -1039, 12370,
    2230, -1039, -1039,  1205,  1051,   406, -1039,  2230, -1039,  1865,
   -1039, -1039, 13135, -1039,   563, -1039, -1039, 13135,  4028, -1039,
   -1039, -1039, -1039, -1039, -1039,  4790, -1039, 11378, 11378, 11104,
   -1039,   633, -1039, -1039,   501, -1039, -1039,   963,  1145,  6181,
    1077, -1039,  1740, -1039, -1039, -1039,   270, 14137,  8910, 14210,
    1034, -1039,  1051, -1039,  1151,  1154, -1039, 14283, -1039,   901,
    1155, -1039,  1157,  1155,  1160,  1160, -1039, -1039,   869,  5066,
   -1039, -1039, -1039, -1039, -1039, -1039,  1367, -1039, -1039, -1039,
    1166,  1042,    87,   183,    24,   276,   280, -1039,  2230, -1039,
    1865, -1039,  1152,  1156, -1039,  1865, -1039,  1865, -1039, -1039,
   13135, -1039,   493, -1039, -1039, -1039,   367,  1155,  1169,  1155,
    1155, -1039, -1039, -1039, -1039,  1865, -1039, -1039, -1039,  1155,
   -1039 ];

/* YYDEFACT[STATE-NUM] -- Default reduction number in state STATE-NUM.
   Performed when YYTABLE does not specify something else to do.  Zero
   means the default is an error.  */
  #[allow(non_upper_case_globals)]
const yydefact_: &'static [i32] = &[      2,     0,     0,     1,     0,   369,   370,   371,     0,   362,
     363,   364,   367,   365,   366,   368,   357,   358,   359,   360,
     380,   290,   290,   650,   649,   651,   652,   763,     0,   763,
       0,     0,   654,   653,   655,   747,   749,   646,   645,   748,
     648,   637,   638,   639,   640,   588,   660,   661,     0,     0,
       0,     0,     0,     0,   318,   775,   775,   102,   439,   608,
     608,   610,   612,     0,     0,     0,     0,     0,     0,     0,
       3,   761,     6,     9,    34,    39,   669,   669,    55,    73,
     290,    72,     0,    90,     0,    94,   104,     0,    64,   242,
     261,     0,   316,     0,     0,    70,    70,   761,     0,     0,
       0,     0,   327,   338,    74,   336,   305,   306,   587,   589,
     307,   308,   309,   311,   310,   312,   586,   627,   628,   585,
     635,   656,   657,   313,     0,   314,    78,     5,     8,   183,
     194,   184,   207,   180,   200,   190,   189,   210,   211,   205,
     188,   187,   182,   208,   212,   213,   192,   181,   195,   199,
     201,   193,   186,   202,   209,   204,   203,   196,   206,   191,
     179,   198,   197,   178,   185,   176,   177,   173,   174,   175,
     133,   135,   134,   168,   169,   164,   146,   147,   148,   155,
     152,   154,   149,   150,   170,   171,   156,   157,   161,   165,
     143,   145,   151,   153,   172,   144,   158,   159,   160,   162,
     163,   167,   166,   138,   140,    27,   136,   137,   139,     0,
     744,     0,     0,     0,     0,   293,   608,     0,   669,   669,
     285,     0,   268,   296,    88,   289,   775,     0,   656,   657,
       0,   314,   775,   740,    89,   763,    86,     0,   775,   461,
      85,   763,   764,     0,     0,    22,   254,     0,    10,     0,
     357,   358,   330,   462,     0,   236,     0,   327,   237,   227,
     228,   324,    18,    20,     0,     0,   761,    16,    19,   763,
      92,    15,   320,   763,     0,   768,   768,   269,     0,     0,
     768,   738,   763,     0,     0,     0,   669,   669,   100,   361,
       0,   110,   111,   118,   440,   632,   631,   633,   630,     0,
     629,     0,     0,     0,   595,   604,   600,   606,   249,    59,
     248,   636,   771,   772,     4,   773,   762,     0,     0,     0,
       0,     0,     0,     0,   692,     0,   668,     0,   692,   666,
       0,     0,   372,   464,   455,    79,   466,   335,   373,   449,
     451,     0,   106,     0,    98,    95,     0,    62,     0,     0,
       0,     0,     0,   264,   265,     0,     0,     0,     0,   225,
     226,     0,     0,    60,     0,     0,   262,   263,     0,     0,
       0,     0,     0,     0,     0,     0,     0,   757,   758,     0,
     775,     0,     0,    69,     0,     0,     0,     0,     0,   761,
     345,   762,     0,   391,   390,     0,     0,   656,   657,   314,
     128,   129,     0,     0,   131,   664,     0,   656,   657,   314,
     353,   203,   196,   206,   191,   173,   174,   175,   133,   134,
     736,    66,    65,   735,     0,    87,   760,   759,     0,   337,
     590,     0,     0,   141,   743,   324,   297,   746,   292,     0,
       0,     0,     0,     0,     0,   286,   295,     0,   775,     0,
       0,     0,     0,   287,   763,     0,   329,   291,   693,   763,
     281,   775,   763,   775,   280,   763,   334,    58,    24,    26,
      25,     0,   331,     0,     0,     0,     0,     0,     0,     0,
     763,   322,    14,   762,    91,   763,   319,   325,   770,   769,
     270,   770,   272,   326,   739,     0,   117,   108,   103,   636,
     668,     0,     0,   775,     0,   692,   614,   634,   617,   615,
     609,   591,   592,   611,   593,   613,     0,     0,     0,     0,
       0,   774,     7,    28,    29,    30,    31,    32,    56,    57,
     699,   696,   695,   694,   697,   705,   714,   693,     0,   726,
     715,   730,   729,   725,   775,   716,   691,   763,   675,   698,
     700,   701,   703,   677,   707,   712,   775,   718,   404,   403,
     723,   677,   728,   677,   732,   674,     0,     0,   775,     0,
       0,     0,   468,    76,    80,   470,   470,     0,    35,   301,
       0,    38,   300,   763,     0,    96,   107,    54,    40,    52,
       0,   273,   296,   214,    36,     0,   314,     0,   234,   241,
     243,   244,   245,   252,   253,   246,   247,   223,   224,   250,
     251,     0,   238,   240,   239,   229,   230,   231,   232,   233,
     763,   266,   267,   747,   749,   748,   751,   460,   750,   290,
     458,   763,   775,   747,   749,   748,   751,   459,   290,     0,
     382,     0,   381,     0,     0,     0,     0,   343,     0,   324,
       0,   775,     0,    70,   351,   128,   129,   130,   662,   349,
       0,   775,     0,     0,     0,   755,   756,    67,   747,   748,
     290,    41,   273,   215,    51,   222,     0,     0,     0,     0,
     742,     0,   298,   294,   775,   747,   748,   763,   747,   748,
     741,   328,   765,   275,   282,   277,   284,   333,    23,     0,
     255,    11,    33,     0,   775,   221,    21,    93,    17,   321,
     768,     0,   101,   752,   116,   763,   747,   748,   692,   441,
     444,   618,     0,   594,     0,   597,     0,   602,   599,     0,
       0,   603,   235,     0,   402,   394,   396,   763,   399,   392,
       0,   673,   734,   667,     0,     0,     0,   684,   706,     0,
     672,   546,   717,     0,   687,   727,     0,   689,   731,    47,
     257,   379,   355,   374,   775,   775,   577,   670,    49,   259,
     356,     0,   775,   468,     0,   775,     0,   304,     0,     0,
     105,    99,     0,     0,     0,     0,     0,   775,     0,   569,
     575,   542,     0,     0,     0,   516,   763,   513,   530,   608,
       0,   568,    63,   487,   493,   495,   497,   491,   490,   526,
     492,   535,   538,   541,   547,   548,   537,   500,   549,   501,
     554,   555,   556,   559,   560,   561,   562,   563,   565,   564,
     566,   567,   545,    61,     0,     0,     0,   457,    83,     0,
     463,   282,     0,     0,   279,   456,    81,   278,   317,   775,
     383,   775,   341,   385,    71,   384,   342,   479,     0,     0,
     376,     0,     0,   752,   323,   763,   747,   748,     0,     0,
       0,     0,   128,   129,   132,   763,     0,   763,     0,   452,
      75,   142,   745,   299,   288,     0,     0,   463,     0,     0,
     763,   775,   763,   271,   109,   463,   763,     0,   619,   623,
     624,   625,   616,   626,   596,   598,   605,   601,   607,   763,
     401,     0,   702,     0,   733,   719,   406,   676,   704,   677,
     677,   713,   718,   775,   677,   724,   677,   701,   677,     0,
       0,   578,   579,   775,   580,   375,   377,     0,     0,    13,
     584,     0,     0,   465,   763,     0,   430,   429,     0,   467,
       0,   450,     0,   302,    37,    97,    53,   274,   747,   748,
     763,   747,   748,   557,   558,   129,   573,     0,   518,   763,
     519,   523,   763,     0,   512,     0,     0,   515,   529,     0,
     570,   642,   641,   643,     0,   571,     0,   488,     0,     0,
     536,   540,   552,   553,     0,   499,   498,     0,     0,   544,
       0,    46,   219,    45,   220,    84,   766,     0,    43,   217,
      44,   218,    82,   378,     0,     0,     0,   386,   388,     0,
       0,     0,   347,     0,   472,     0,   346,   463,     0,     0,
       0,     0,   463,   354,   737,    68,   453,   454,     0,   276,
     283,   332,     0,   433,   447,   445,   442,   620,   393,   395,
     397,   400,     0,   680,     0,   682,   671,     0,   688,     0,
     685,   690,    48,   258,     0,     0,   582,   583,   775,    50,
     260,   775,   428,   763,     0,   701,   412,   709,   710,   775,
     721,   412,   412,   410,   469,    77,   471,   303,   463,   763,
     510,   533,   521,   520,   511,   524,   608,   763,   767,   543,
       0,   494,   489,   526,   496,   527,   531,   539,   534,   550,
     551,   574,   509,   505,   763,   763,   763,   763,   763,   763,
     256,    42,   216,   389,     0,   339,   340,     0,   484,   344,
     473,     0,     0,   348,     0,   663,   350,   443,     0,     0,
       0,   621,     0,     0,   677,   677,   677,   677,     0,   581,
       0,   656,   657,   314,     0,    12,   409,     0,   431,     0,
     413,   421,   419,     0,   708,     0,   408,     0,   424,     0,
     426,   517,     0,   525,     0,   514,   572,     0,     0,   502,
     503,   504,   506,   507,   508,     0,   480,     0,     0,     0,
     474,   775,   352,   437,   763,   435,   438,     0,     0,     0,
     398,   681,     0,   678,   683,   686,   324,     0,   775,     0,
     775,   432,   720,   411,   412,   412,   324,     0,   711,   775,
     412,   722,   412,   412,   522,   527,   528,   532,   775,     0,
     485,   486,   475,   477,   478,   476,     0,   434,   448,   446,
       0,   677,   752,   323,   763,   747,   748,   576,     0,   416,
       0,   418,   752,   323,   407,     0,   425,     0,   422,   427,
       0,   387,   775,   436,   622,   679,   463,   412,   412,   412,
     412,   482,   483,   481,   417,     0,   414,   420,   423,   412,
     415 ];

/* YYPGOTO[NTERM-NUM].  */
  #[allow(non_upper_case_globals)]
const yypgoto_: &'static [i32] = &[  -1039, -1039, -1039,  1048, -1039,    16,   817,  -306,   217, -1039,
     823, -1039,   126, -1039,  -225,  -340,    25, -1039, -1039,   421,
    2287,  2600, -1039,   -65,   -85, -1039,   -28, -1039,  -232, -1039,
    1287,    -9,  1202,  -149,   -27,   -49, -1039,  -419,    14,  2729,
    -369,  1201,   -56,    -1, -1039, -1039,     2, -1039,  3595, -1039,
    1219, -1039,  1138, -1039,  -273,     9,   644,  -347,    80,    -7,
   -1039,  -397,  -186,    90, -1039,  -309,   -41, -1039, -1039, -1039,
   -1039, -1039, -1039, -1039, -1039, -1039,   727, -1039, -1039, -1039,
   -1039, -1039, -1039, -1039, -1039, -1039, -1039, -1039, -1039, -1039,
   -1039, -1039,   547, -1039,  1184,  1706,  -379, -1039,    81,  -782,
   -1039,  -861,  -832,   588,   431,  -888,   268, -1039,   400,   -22,
   -1039, -1039,   581, -1039,  -882, -1039,   118,   470, -1039, -1039,
   -1039, -1039, -1039, -1039, -1039,   478, -1039, -1039, -1039,   -96,
   -1039, -1039,   594, -1039,   796, -1039, -1039,  -819, -1039,   111,
   -1039, -1039, -1039, -1039,  -598,  -363, -1039, -1039, -1039, -1039,
     386, -1039, -1039,     7, -1039,  -545,  -702,  -854,  -619,  -771,
    -243, -1039,   389, -1039, -1039,  -123,   392, -1039,  -604,   397,
   -1039, -1039, -1039,   175, -1039, -1039,   318,   485,   891, -1039,
    1279,   907,  1014,  1176, -1039,   875,  1451, -1039,  1464,  1797,
   -1039, -1039,   -59, -1039, -1039,  -142, -1039, -1039, -1039, -1039,
   -1039, -1039, -1039,    -4, -1039, -1039, -1039, -1039,     1, -1039,
     207,    10,  1293,  3042,  2181, -1039, -1039,    21,   651,     0,
   -1039,  -256,  -395,  -249,  -216, -1015,  -487,  -305,  -711,  -371,
     -72,   647,   232, -1039, -1039,  -471, -1039,  -708,  -688, -1038,
     238,   655, -1039,  -615, -1039,   476,  -539, -1039, -1039, -1039,
      32,  -431,  -301,  -296, -1039, -1039,   -86, -1039,   -54,   156,
     848,   295,   308,  -238,   -69,    19,    -2 ];

/* YYDEFGOTO[NTERM-NUM].  */
  #[allow(non_upper_case_globals)]
const yydefgoto_: &'static [i32] = &[      0,     1,     2,    70,    71,    72,   248,   567,   568,   266,
     267,   479,   268,   471,    74,   588,    75,   611,   597,   421,
     218,   219,   878,   384,   386,   387,    78,    79,   574,   773,
     254,    81,    82,   269,    83,    84,    85,   498,    86,   221,
     404,   405,   203,   204,   205,   676,   626,   207,    88,   473,
     375,    89,   223,   274,   593,   627,   845,   459,   460,   236,
     237,   225,   445,   632,   581,   582,    90,   382,   273,   485,
     699,   858,   648,   871,   869,   663,   256,    92,    93,    94,
      95,    96,    97,    98,    99,   100,   101,   336,   339,   764,
     937,   861,  1014,  1015,   762,   257,   641,   854,  1016,  1017,
     396,   735,   736,   737,   738,   544,   744,   745,  1213,  1161,
    1162,  1073,   945,   946,  1042,  1194,  1195,   103,   294,   505,
     897,   719,  1046,  1140,  1139,   340,   576,   104,   105,   337,
     572,   575,   771,   772,   774,   775,  1025,   862,  1235,   859,
    1020,  1229,  1273,  1127,   802,  1091,   804,   805,   997,   998,
     806,   975,   967,   969,   970,   971,   808,   809,  1105,   973,
     810,   811,   812,   813,   814,   545,   816,   817,   818,   819,
     820,   821,   822,   765,   933,  1065,   939,   106,   107,   108,
     109,   110,   111,   112,   516,   724,   113,   518,   114,   115,
     517,   519,   299,   302,   303,   510,   722,   721,   898,  1047,
    1141,  1199,   902,   116,   117,   300,   118,   119,   120,   985,
     228,   229,   123,   230,   231,   659,   870,   325,   326,   327,
     328,   917,   747,   547,   548,   549,   550,   927,   552,   553,
     554,   555,  1078,  1079,   556,   557,   558,   559,   560,  1080,
    1081,   561,   562,   563,   564,   565,   741,   424,   664,   279,
     463,   233,   126,   703,   630,   667,   662,   428,   314,   455,
     456,   840,   977,   490,   642,   391,   271 ];

/* YYTABLE[YYPACT[STATE-NUM]] -- What to do in state STATE-NUM.  If
   positive, shift that token.  If negative, reduce the rule whose
   number is the opposite.  If YYTABLE_NINF, syntax error.  */
  #[allow(non_upper_case_globals)]
const yytable_: &'static [i32] = &[    127,   301,   315,   298,   208,   381,   643,   220,   220,   429,
     206,   388,   122,   833,   122,   240,   289,   750,   427,   551,
     128,   690,   462,   551,   208,   571,   270,   245,   315,   385,
     206,   239,   389,   631,   657,   918,   345,   594,   492,  1021,
     309,   923,   494,   390,   289,   422,   453,   683,   232,   232,
    1049,   739,   807,   277,   281,   683,  1071,   289,   289,   289,
     690,   206,  1074,   122,   122,   925,   807,   292,   546,  1019,
     311,   288,   546,   335,   329,   329,   639,   331,   628,   569,
     628,  1051,   712,   629,  -123,   638,   332,   276,   280,   334,
     316,   671,   674,   220,   972,   292,   224,   234,   330,  1044,
    -123,   687,   726,   206,   986,  -658,   226,   226,   398,   408,
     408,   408,   322,   323,  -119,   480,  1092,   640,   383,   383,
     853,   587,   383,  1196,     3,  -763,   832,   628,    73,  1221,
      73,   920,   670,  1103,   232,  -119,   751,   730,   926,  -659,
     832,   449,   451,   655,   242,   275,   628,   656,   628,   478,
     261,   629,   488,   638,   752,   727,   715,   439,   673,   675,
     513,   515,   242,   640,   242,   781,   754,   489,   757,  -120,
     477,  -120,  -127,  -119,  -763,   628,   628,   673,   675,  -126,
     272,   704,   425,   243,  -114,   246,   333,   242,   963,   964,
     731,  1157,   226,  1045,   583,  -747,  -125,   315,   247,  1093,
     551,   628,   332,   628,   504,   705,   587,   587,   704,   121,
    1221,   121,   482,  -120,  -110,   242,   329,   329,  1106,   331,
     751,  1196,   122,  -747,   446,  -123,   220,  -123,   220,   220,
     446,   312,   249,   313,   803,   497,   464,   270,   999,   440,
     441,   338,   312,   289,   313,   127,   739,   910,   803,   546,
    -127,   986,   523,   524,   525,   526,   720,   122,   541,   122,
     121,   121,   239,   334,   291,   690,   465,   232,   467,   232,
     265,  -111,  1071,   304,   122,  1131,   987,   453,   922,  1071,
     542,  1071,  1049,  -126,   305,   483,   499,   331,   312,  -122,
     313,   874,   291,  -124,   292,  1115,  1118,   683,  -119,   683,
    -119,  -650,   333,   289,   865,   397,   407,   407,   407,   652,
    1200,  -127,   238,   578,   875,   454,   270,   457,   589,  -748,
     315,  -125,   645,  -125,  1226,   226,   122,   226,   461,  1082,
    -650,   122,   522,   935,  -125,   647,   306,   122,  -120,   936,
    -120,   122,   383,   383,   383,   383,   918,   528,   529,   628,
    -649,   628,  -651,   122,   292,   994,   704,   876,   585,   628,
    1071,   628,  1071,   955,   925,  -126,   704,  1071,  -122,  1071,
     432,   220,  1234,    73,   725,  -127,   725,  -127,   464,  -649,
    -121,  -651,   924,   636,  1056,   928,   995,  1071,  1109,  1110,
     637,  -658,   122,   888,   996,   307,   122,  -124,   122,  -121,
    -127,  1106,  -659,   589,   589,   889,  1106,  1227,  -126,  1233,
    -126,  -118,   383,   551,  -122,   342,  -122,  -110,  -124,   121,
    -124,  -119,  1128,  -122,   739,   966,   739,   654,  -111,   265,
     636,   580,  -120,   968,  -116,  -124,   580,  1145,   495,   220,
     960,   551,    73,   341,   683,  -748,   464,   527,   551,   918,
     991,   636,   497,  -652,   121,   932,   121,  1219,   637,   446,
     289,   446,   546,   346,   535,  -117,   475,   208,  -113,   896,
     698,   121,   893,   206,   815,   807,   684,  -114,   890,   636,
    1271,   265,  -652,   536,   433,   628,   637,   628,   815,  1227,
     629,   291,   638,   122,   220,  1001,  1003,  -115,   884,  -112,
     767,   464,  1008,  1010,   832,  -121,   636,  -121,   936,   788,
    -654,   292,   348,   637,   540,   541,   986,   791,   857,   832,
     710,   322,   323,   121,  1053,  1055,   832,   832,   121,  1058,
     506,  1060,  1156,  1061,   121,   497,   536,   542,   121,  -654,
    1166,  -533,   742,   289,   918,  1215,   506,   353,   354,   832,
     121,   291,  1222,  1224,   742,  -114,    60,   759,  1225,   587,
     265,   768,  1002,  1004,   468,  1031,   766,   540,   868,  1009,
    1011,  1096,   506,  1076,   469,   470,   855,  -114,   371,   372,
     373,  -114,   905,   507,   241,   508,   509,   905,  -747,   121,
    1038,  -113,  -114,   121,   292,   121,  -115,   366,   367,  -112,
     435,   508,   509,   644,  1089,   646,  1005,   831,   491,   690,
     587,   587,  1002,  1004,  1012,  1009,  1011,   587,   587,   904,
    -121,   831,   838,   489,   968,  1111,  1236,   508,   509,   506,
     446,   846,   431,  1268,   968,   968,   847,  -361,   837,  1075,
     442,   242,   220,   683,  1086,   907,  1132,   844,   936,   464,
     860,   122,   220,   122,   636,   739,   739,   803,   312,   464,
     313,   637,   894,   880,   636,  -361,  -361,  1121,   847,  -113,
     289,   637,   208,   122,  -115,   881,   443,  -112,   206,   844,
    1254,  1144,   446,  1146,   508,   509,  1154,  1099,  1147,   877,
     121,  -113,   432,   348,   922,  -113,  -115,  -361,  -763,  -112,
    -115,   242,   847,  -112,   450,   884,  -113,  -323,   291,   377,
    -653,  -115,   837,   844,  -112,   241,   506,   514,  -118,   242,
     444,   292,  -127,  1134,   892,   980,   891,   981,   982,    91,
     983,    91,  1097,   497,  1122,  -323,  -323,   378,   379,  -653,
     979,   289,   472,   227,   227,  1185,  -775,   551,  1186,  1201,
    1203,  1204,  1205,   324,  1116,  1119,   589,   369,   370,   371,
     372,   373,   934,   940,   842,  1122,   235,  -323,   452,   380,
     947,   508,   509,   947,   984,  -748,   834,   348,   220,   481,
      91,    91,   587,  1036,   290,   464,   506,   839,   843,   122,
     636,   291,   292,   122,  -122,   227,  1220,   637,  1223,  -655,
    1029,   -90,   831,   831,   478,  1030,   831,   589,   589,   377,
    1007,   238,   290,  -105,   589,   589,   377,   -91,  -124,   487,
     227,   227,   422,   831,   227,   395,   406,   406,  -655,   227,
    -117,  1241,   506,  1197,  -126,   493,   377,   378,   447,   511,
    -121,   508,   509,   839,   378,   476,  1265,  1018,   121,  1018,
     121,   324,   815,   655,   580,  1075,  -747,   965,   849,   496,
     851,  1244,  1075,  -656,   378,   502,   520,   815,   501,   448,
     121,   839,  1077,   891,   815,   815,   448,  1267,   122,  1269,
     206,   122,  -657,  1013,   936,   511,  1270,   508,   509,   446,
     677,  -656,  -656,  -644,   566,   383,   503,   815,  -644,   992,
     993,  -647,  -665,   842,  1279,   954,   628,   377,   628,   956,
    -657,  -657,  -314,   704,   570,   665,   628,   586,   291,  1100,
     521,   742,  -644,  -656,   573,  -748,  -644,  -644,   506,  -113,
    -647,  1066,   506,  -122,   649,   378,   426,   500,   500,    91,
    -314,  -314,  -657,  1075,   666,   843,   653,   122,   122,   658,
    1124,   678,   839,   976,   692,   122,  -747,   679,  -644,   506,
     122,  1135,   227,   506,   227,   227,  -747,  -647,   227,   701,
     227,  -115,  -314,   761,    91,  -124,    91,   831,   694,   589,
     831,   512,   887,   508,   509,   723,   121,   508,   509,   291,
     121,    91,  -752,   696,  1102,  -647,  -647,   831,   831,   831,
    1187,  1188,   831,   831,  1114,  1117,   702,   831,   831,   383,
     895,   290,   511,  -105,   508,   509,   728,  1099,   508,   509,
     711,   839,   755,   289,   122,  -748,   740,  -647,   746,  -405,
     831,   839,   758,   749,  -752,  -748,   289,  1174,   753,   122,
     383,   756,   763,    91,   227,   227,   227,   227,    91,   227,
     227,   377,  1043,  -296,    91,   778,   779,   782,    91,  1168,
    1170,   783,  -752,  -752,  1209,   784,   940,   801,   227,   742,
      91,   290,   841,   595,  1152,   121,   377,   742,   121,   378,
     650,   801,   823,   850,   857,  1028,   860,   408,  1077,   466,
     906,   974,  -752,  1077,  -752,  1077,   823,  1077,   908,   911,
    1043,   913,  -747,  -324,   378,   660,   377,   227,   916,    91,
     929,   651,   377,    91,   227,    91,   839,   484,   289,   377,
     941,   486,  1230,  1231,   943,   839,   944,  -297,   839,   227,
     978,  -324,  -324,   988,   378,   786,   661,  1000,   989,   936,
     378,  1207,   122,   990,   121,   121,  1006,   378,  1217,   122,
     122,   839,   121,  1040,  1067,  1068,  1052,   121,   595,   595,
    1027,  1054,  1084,  -324,   122,   752,   787,  1057,  1059,   408,
    1032,  1064,  1208,  1085,  -298,   227,  1077,  1095,  1077,   661,
    1098,   789,   831,  1077,   986,  1077,  1137,   831,   831,  1018,
    1142,   935,  1249,  1251,   278,   122,  1138,  1007,  1256,   220,
    1258,  1259,  1143,  1077,   899,   900,   464,   901,   766,   122,
      91,   636,   383,   383,    46,    47,   914,   742,   637,  1158,
    1160,   121,  1165,  -112,  1167,   914,  1018,  -121,   290,  1043,
     227,  1123,    41,    42,    43,    44,   121,  1169,   530,   122,
     531,   532,   533,   534,  -299,  1274,  1276,  1277,  1278,  1176,
    1172,   348,  1173,   976,  1206,  1088,  1177,  1280,  1178,  1202,
    1018,  1216,   801,   801,  1090,  1211,   801,  1094,   361,   362,
     831,  1151,  1239,   839,   839,   839,  1248,   823,   823,  1250,
    1255,   823,  1257,   801,   407,  1260,  1264,   733,  -747,    80,
    1112,    80,  -748,   227,  1275,   474,   706,   227,   823,  1035,
     393,   410,   691,    80,    80,   365,   708,   693,   376,  1261,
     695,   290,   938,   697,   879,   369,   370,   371,   372,   373,
     530,   909,   531,   532,   533,   534,   535,   530,   707,   531,
     532,   533,   534,   709,   317,   318,   319,   320,   321,   121,
      80,    80,  1050,  1159,  1072,   536,   121,   121,   434,  1191,
    1237,   436,   437,   438,  1263,    80,   950,  1198,  1037,   537,
     530,   121,   531,   532,   533,   534,   407,   948,    91,   538,
      91,  1210,   776,  1272,  1104,   539,   540,   541,   227,  1107,
      80,    80,  1108,  1101,    80,  1247,  1155,   430,   227,    80,
      91,   227,   121,   729,   423,   743,   921,  1218,  1214,   542,
     839,   919,  1228,     0,   718,  1175,   121,     0,     0,   733,
       0,   543,  1182,  1183,  1184,   734,  1240,     0,     0,  1193,
    1083,   531,   532,   533,   534,     0,   227,     0,     0,     0,
       0,   780,     0,     0,     0,     0,   121,   801,   290,     0,
     801,     0,     0,     0,     0,     0,  1262,     0,     0,     0,
       0,     0,   823,     0,     0,   823,     0,   801,   801,   801,
       0,     0,   801,   801,     0,     0,     0,   801,   801,     0,
       0,     0,   823,   823,   823,     0,     0,   823,   823,   579,
       0,     0,   823,   823,   592,     0,     0,     0,   824,     0,
     801,     0,   530,     0,   531,   532,   533,   534,   535,    80,
       0,     0,   824,     0,   825,   823,    91,     0,     0,   290,
      91,   595,     0,     0,   227,     0,     0,   536,   825,     0,
       0,     0,    80,     0,    80,    80,     0,     0,    80,     0,
      80,   537,     0,     0,    80,     0,    80,     0,     0,  1266,
       0,   538,     0,     0,     0,     0,     0,   539,   540,   541,
       0,    80,     0,     0,     0,   348,     0,     0,     0,     0,
       0,     0,   595,   595,     0,     0,     0,     0,     0,   595,
     595,   542,   361,   362,     0,     0,     0,     0,     0,     0,
       0,   680,   682,   543,     0,   912,     0,     0,     0,     0,
     278,     0,     0,     0,     0,    91,     0,   227,    91,     0,
       0,     0,     0,    80,    80,    80,    80,    80,    80,    80,
      80,   826,     0,     0,    80,     0,     0,     0,    80,   369,
     370,   371,   372,   373,     0,   826,   682,     0,    80,   278,
      80,     0,     0,    80,     0,     0,  1083,     0,     0,     0,
       0,   914,   801,  1083,     0,  1083,     0,   801,   801,     0,
       0,     0,     0,     0,     0,     0,     0,   823,     0,     0,
       0,     0,   823,   823,    91,    91,     0,    80,     0,    80,
       0,     0,    91,    80,    80,    80,     0,    91,     0,     0,
       0,     0,     0,   824,   824,     0,     0,   824,     0,    80,
       0,     0,   748,     0,     0,     0,     0,     0,     0,   825,
     825,     0,     0,   825,   824,     0,     0,     0,   102,     0,
     102,   227,     0,     0,     0,   777,     0,     0,    80,    80,
     825,     0,   102,   102,  1083,  1034,  1083,     0,     0,     0,
     801,  1083,     0,  1083,   595,    80,     0,     0,  1039,     0,
    1041,    91,   227,     0,     0,   823,     0,     0,     0,     0,
       0,  1083,     0,     0,     0,   770,    91,  1048,     0,   102,
     102,     0,     0,     0,     0,     0,     0,     0,     0,     0,
      80,   348,     0,   827,   102,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,   827,   361,   362,
      80,  1150,   530,     0,   531,   532,   533,   534,   535,   102,
     102,     0,     0,   102,  1164,     0,   826,   826,   102,     0,
     826,     0,     0,     0,     0,   882,     0,   536,     0,   883,
       0,     0,     0,   848,   364,   365,     0,   826,   852,   368,
     856,   537,   682,     0,   278,   369,   370,   371,   372,   373,
       0,   538,     0,     0,  1113,     0,     0,     0,   540,   541,
       0,     0,     0,    80,     0,     0,     0,    80,   824,    91,
       0,   824,     0,     0,     0,     0,    91,    91,     0,     0,
       0,   542,     0,     0,   825,     0,     0,   825,   824,   824,
     824,    91,   915,   824,   824,     0,  1212,     0,   824,   824,
       0,     0,     0,     0,   825,   825,   825,     0,     0,   825,
     825,     0,   931,     0,   825,   825,     0,     0,     0,     0,
       0,   824,    91,     0,   227,   227,   953,   530,   102,   531,
     532,   533,   534,   535,     0,     0,    91,   825,    80,     0,
      80,     0,     0,     0,     0,   227,     0,  1171,    80,     0,
       0,   102,   536,   102,   102,     0,     0,   102,    80,   102,
      80,    80,     0,   102,     0,   102,    91,     0,   949,     0,
     951,     0,  1179,  1180,  1181,     0,   538,     0,   827,   827,
     102,     0,   827,   540,   541,     0,     0,     0,     0,   682,
       0,   826,     0,     0,   826,     0,    80,     0,     0,   827,
       0,     0,     0,     0,     0,     0,   542,     0,     0,  1024,
       0,   826,   826,   826,     0,     0,   826,   826,     0,     0,
       0,   826,   826,     0,     0,     0,     0,     0,     0,     0,
       0,     0,   102,   102,   102,   102,   102,   102,   102,   102,
       0,     0,     0,   102,   826,     0,     0,   102,     0,     0,
       0,     0,     0,  1022,     0,     0,  1026,   102,   828,   102,
       0,     0,   102,     0,     0,     0,     0,     0,     0,     0,
    1033,   829,   828,   824,     0,     0,    80,     0,   824,   824,
      80,    80,     0,     0,    80,   829,     0,     0,     0,   825,
       0,     0,     0,     0,   825,   825,   102,     0,   102,     0,
    1087,     0,   102,   102,   102,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,   102,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,    80,    80,     0,     0,     0,     0,     0,    80,
      80,     0,     0,     0,     0,     0,     0,   102,   102,     0,
       0,     0,     0,   827,     0,     0,   827,     0,     0,     0,
       0,   824,     0,     0,   102,    80,     0,    80,    80,     0,
       0,  1130,     0,   827,   827,   827,     0,   825,   827,   827,
       0,     0,     0,   827,   827,     0,     0,     0,   278,     0,
       0,     0,     0,   125,     0,   125,   826,     0,     0,   102,
       0,   826,   826,     0,     0,     0,   827,     0,     0,     0,
    1125,     0,     0,  1126,     0,  1129,     0,     0,     0,   102,
       0,     0,  1133,     0,     0,  1136,     0,     0,     0,     0,
       0,     0,     0,     0,    80,    80,     0,     0,     0,     0,
       0,     0,    80,     0,   125,   125,     0,    80,   293,     0,
       0,     0,     0,   828,   828,     0,     0,   828,     0,     0,
       0,     0,     0,     0,     0,     0,   829,   829,     0,     0,
     829,     0,     0,     0,   828,     0,   293,     0,     0,  1190,
       0,    80,   102,     0,   826,     0,   102,   829,     0,   399,
     409,   409,   530,     0,   531,   532,   533,   534,   535,    76,
       0,    76,     0,     0,    80,     0,     0,     0,     0,     0,
       0,    80,    80,     0,     0,     0,     0,   536,     0,   530,
       0,   531,   532,   533,   534,   535,    80,     0,  1192,     0,
       0,     0,     0,     0,     0,     0,     0,  1232,     0,     0,
       0,   538,     0,     0,   536,     0,     0,   539,   540,   541,
      76,    76,     0,     0,   286,     0,     0,   102,   827,   102,
       0,     0,     0,   827,   827,     0,     0,   102,   538,     0,
       0,   542,     0,     0,   539,   540,   541,   102,     0,   102,
     102,     0,   286,   543,     0,     0,     0,     0,     0,     0,
       0,  1238,     0,     0,     0,   286,   286,   286,   542,     0,
       0,     0,     0,   125,   830,     0,     0,   242,     0,     0,
     543,     0,     0,     0,     0,   102,     0,     0,   830,     0,
       0,     0,     0,     0,     0,     0,     0,     0,   828,    80,
       0,   828,     0,     0,     0,     0,    80,    80,   125,     0,
     125,   829,     0,     0,   829,     0,   827,     0,   828,   828,
     828,    80,     0,   828,   828,   125,     0,     0,   828,   828,
       0,   829,   829,   829,     0,     0,   829,   829,     0,     0,
       0,   829,   829,     0,     0,   293,     0,     0,     0,     0,
       0,   828,    80,     0,    80,    80,     0,     0,     0,     0,
       0,     0,     0,     0,   829,   102,    80,     0,     0,   102,
     102,     0,     0,   102,     0,    80,     0,   125,     0,    76,
       0,     0,   125,    23,    24,    25,    26,     0,   125,     0,
       0,     0,   125,     0,     0,     0,    80,     0,     0,    32,
      33,    34,     0,     0,   125,   293,     0,   596,     0,    41,
      42,    43,    44,    45,    76,     0,    76,     0,     0,     0,
       0,   102,   102,     0,     0,     0,     0,     0,   102,   102,
       0,    76,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,   125,     0,     0,     0,   125,     0,   125,
       0,   286,     0,     0,   102,     0,   102,   102,     0,     0,
      58,    59,    60,    61,    62,    63,    64,    65,    66,   830,
     830,     0,     0,   830,     0,     0,     0,     0,     0,     0,
       0,     0,    77,    76,    77,     0,     0,     0,    76,     0,
     830,     0,   596,   596,    76,     0,     0,     0,    76,     0,
       0,   285,     0,   828,     0,     0,     0,     0,   828,   828,
      76,   286,     0,    76,     0,     0,   829,     0,     0,     0,
       0,   829,   829,   102,   102,     0,     0,     0,     0,     0,
       0,   102,     0,    77,    77,     0,   102,   287,     0,     0,
       0,     0,     0,     0,   125,     0,     0,     0,     0,    76,
       0,     0,     0,    76,     0,    76,     0,     0,     0,     0,
       0,     0,   293,     0,     0,   287,     0,     0,     0,     0,
     102,     0,     0,     0,     0,     0,     0,     0,   287,   287,
     287,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,   828,     0,   102,     0,     0,     0,     0,    76,    76,
     102,   102,     0,     0,   829,     0,     0,     0,     0,     0,
       0,    87,     0,    87,     0,   102,  -775,     0,     0,     0,
       0,     0,     0,     0,  -775,  -775,  -775,     0,     0,  -775,
    -775,  -775,     0,  -775,     0,     0,     0,     0,     0,     0,
       0,  -775,  -775,  -775,   830,   293,     0,   830,     0,     0,
      76,     0,     0,  -775,  -775,     0,  -775,  -775,  -775,  -775,
    -775,     0,    87,    87,   830,   830,   830,     0,   286,   830,
     830,     0,     0,     0,   830,   830,     0,     0,     0,     0,
       0,     0,     0,     0,  -775,     0,     0,     0,     0,     0,
       0,     0,    77,     0,     0,     0,     0,   830,     0,     0,
       0,     0,   125,     0,   125,     0,     0,   394,     0,     0,
       0,     0,  -775,  -775,     0,     0,     0,     0,   102,  -775,
       0,     0,     0,     0,   125,   102,   102,    77,     0,    77,
       0,     0,     0,     0,     0,     0,  -775,     0,     0,     0,
     102,     0,  -775,  -775,    77,     0,     0,     0,     0,     0,
       0,   286,   238,     0,  -775,     0,  -775,     0,     0,     0,
       0,     0,     0,     0,   287,     0,     0,     0,     0,     0,
       0,   102,   293,   102,   102,     0,     0,     0,     0,     0,
       0,     0,     0,   903,     0,   102,     0,     0,     0,     0,
       0,     0,     0,     0,   102,     0,    77,     0,     0,     0,
       0,    77,     0,     0,     0,     0,     0,    77,    76,     0,
      76,    77,     0,     0,     0,   102,     0,     0,     0,     0,
       0,    87,     0,    77,   287,     0,    77,     0,     0,     0,
      76,     0,     0,     0,     0,     0,     0,     0,     0,     0,
     125,     0,     0,   293,   125,   596,     0,     0,     0,   830,
       0,     0,     0,     0,   830,   830,    87,     0,    87,     0,
       0,     0,    77,     0,     0,     0,    77,     0,    77,     0,
       0,     0,     0,    87,     0,     0,     0,     0,   286,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,   596,   596,     0,     0,
       0,     0,     0,   596,   596,     0,     0,     0,     0,     0,
       0,    77,    77,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,   124,    87,   124,     0,     0,   125,
      87,     0,   125,     0,     0,     0,    87,   830,     0,     0,
      87,     0,     0,     0,     0,     0,    76,     0,     0,   286,
      76,    76,    87,     0,     0,   590,     0,     0,     0,     0,
       0,     0,     0,    77,   347,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,   124,   124,     0,     0,     0,
       0,   287,     0,     0,     0,     0,     0,     0,     0,     0,
       0,    87,     0,     0,     0,    87,     0,    87,   125,   125,
       0,     0,    76,    76,     0,     0,   125,     0,     0,    76,
      76,   125,     0,     0,     0,     0,   348,   349,   350,   351,
     352,   353,   354,   355,   356,   357,   358,   359,   360,     0,
       0,     0,     0,   361,   362,    76,     0,     0,    76,   363,
     590,   590,     0,   348,   349,   350,   351,   352,   353,   354,
     355,   356,   357,   358,   359,   360,     0,     0,     0,     0,
     361,   362,     0,     0,   287,     0,     0,     0,   596,   364,
     365,   366,   367,     0,   368,   125,     0,     0,     0,     0,
     369,   370,   371,   372,   373,     0,     0,     0,   374,     0,
     125,     0,    87,     0,     0,     0,   364,   365,   366,   367,
       0,   368,     0,     0,    76,    76,   242,   369,   370,   371,
     372,   373,    76,     0,     0,   374,     0,    76,     0,     0,
       0,    77,     0,    77,     0,  1153,     0,     0,     0,     0,
       0,     0,     0,     0,   124,     0,     0,     0,   409,     0,
       0,     0,     0,    77,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,   348,   349,
     350,   351,   352,   353,   354,   355,     0,   357,   358,   124,
       0,   124,     0,     0,    76,   361,   362,     0,     0,     0,
       0,    76,     0,     0,     0,     0,   124,     0,     0,     0,
       0,   287,     0,   125,     0,     0,    76,     0,     0,     0,
     125,   125,     0,     0,     0,     0,     0,     0,     0,     0,
       0,   364,   365,   366,   367,   125,   368,     0,     0,     0,
     409,     0,   369,   370,   371,   372,   373,     0,     0,     0,
       0,   286,     0,     0,     0,     0,     0,     0,   124,     0,
       0,   785,     0,   124,   286,     0,   125,     0,     0,   124,
      87,     0,    87,   124,     0,     0,     0,     0,     0,    77,
     125,     0,   287,    77,    77,   124,     0,     0,   124,     0,
       0,     0,    87,     0,   348,   349,   350,   351,   352,   353,
     354,   355,   356,   357,   358,   359,   360,     0,     0,     0,
     125,   361,   362,     0,     0,     0,     0,     0,     0,    76,
       0,     0,     0,     0,   124,     0,    76,    76,   124,     0,
     124,     0,     0,     0,     0,    77,    77,     0,     0,     0,
       0,    76,    77,    77,     0,     0,   286,   364,   365,   366,
     367,     0,   368,     0,     0,     0,     0,     0,   369,   370,
     371,   372,   373,     0,     0,     0,   374,     0,    77,     0,
       0,    77,    76,   124,   124,    23,    24,    25,    26,     0,
       0,     0,     0,     0,     0,     0,    76,     0,     0,     0,
       0,    32,    33,    34,   789,     0,     0,     0,   790,     0,
       0,    41,    42,    43,    44,    45,     0,     0,    87,     0,
       0,     0,    87,   590,     0,     0,    76,     0,     0,     0,
       0,     0,     0,     0,     0,   124,     0,     0,     0,     0,
       0,     0,   792,   793,     0,     0,     0,    77,    77,     0,
     794,     0,     0,   795,     0,    77,   796,   797,     0,   798,
      77,     0,    58,    59,    60,    61,    62,    63,    64,    65,
      66,     0,     0,     0,   590,   590,     0,     0,     0,     0,
       0,   590,   590,     0,     0,     0,     0,   800,     0,     0,
       0,     0,   242,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,   285,     0,     0,     0,    87,     0,     0,
      87,     0,     0,     0,     0,     0,     0,    77,     0,     0,
       0,   222,   222,     0,    77,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,    77,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,   255,   258,   259,   260,     0,     0,     0,
     222,   222,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,   308,   310,   287,     0,    87,    87,     0,     0,
       0,     0,     0,     0,    87,     0,     0,   287,     0,    87,
       0,     0,     0,   124,     0,   124,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,   222,    23,    24,
      25,    26,     0,     0,     0,   124,     0,     0,     0,     0,
       0,     0,     0,     0,    32,    33,    34,   789,     0,     0,
       0,   790,     0,   791,    41,    42,    43,    44,    45,     0,
       0,     0,    77,     0,     0,     0,   590,     0,     0,    77,
      77,     0,   536,    87,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,    77,   792,   793,     0,    87,   287,
       0,     0,     0,   794,     0,     0,   795,     0,     0,   796,
     797,     0,   798,   540,     0,    58,    59,   799,    61,    62,
      63,    64,    65,    66,     0,    77,     0,     0,     0,     0,
       0,     0,     0,  1149,     0,     0,     0,     0,     0,    77,
     800,   785,     0,     0,     0,   222,     0,     0,   222,   222,
     222,     0,   310,     0,     0,     0,   285,     0,     0,     0,
       0,   124,     0,     0,     0,   124,   124,     0,     0,    77,
     222,     0,   222,   222,   348,   349,   350,   351,   352,   353,
     354,   355,   356,   357,   358,   359,   360,     0,     0,     0,
       0,   361,   362,     0,     0,     0,     0,     0,     0,     0,
       0,    87,     0,     0,     0,     0,     0,     0,    87,    87,
       0,     0,     0,     0,     0,     0,     0,   124,   124,     0,
       0,     0,  -268,    87,   124,   124,     0,   364,   365,   366,
     367,     0,   368,     0,     0,     0,     0,     0,   369,   370,
     371,   372,   373,     0,     0,     0,   374,     0,     0,     0,
     124,     0,     0,   124,    87,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,    87,     0,
       0,     0,     0,     0,     0,     0,   222,     0,     0,     0,
       0,   591,     0,   598,   599,   600,   601,   602,     0,     0,
     603,   604,   605,   606,   607,   608,   609,   610,    87,   612,
     613,     0,     0,   614,   615,   616,   617,   618,   619,   620,
     621,   622,     0,     0,     0,   222,     0,     0,     0,   124,
     124,     0,     0,     0,     0,     0,     0,   124,     0,     0,
       0,     0,   124,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,   672,   672,     0,     0,
       0,     0,     0,     0,     0,     0,     0,   672,   222,   222,
       0,     0,     0,   222,     0,   672,   672,   222,     0,   124,
       0,     0,     0,   260,     0,     0,   124,     0,     0,     0,
       0,    23,    24,    25,    26,     0,     0,     0,   700,     0,
       0,   124,     0,   672,     0,     0,     0,    32,    33,    34,
     789,     0,     0,   222,   790,     0,   222,    41,    42,    43,
      44,    45,     0,     0,     0,     0,     0,     0,   222,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,   732,     0,     0,   792,   793,
       0,     0,     0,     0,     0,     0,   794,     0,     0,   795,
       0,     0,   796,   797,     0,   798,     0,     0,    58,    59,
      60,    61,    62,    63,    64,    65,    66,     0,     0,   222,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,   760,     0,   800,   930,   769,     0,     0,     0,     0,
       0,     0,   222,     0,   124,     0,     0,     0,     0,   285,
       0,   124,   124,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,   124,   348,   349,   350,
     351,   352,   353,   354,   355,   356,   357,   358,   359,   360,
       0,     0,     0,     0,   361,   362,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,   124,     0,     0,
     348,  -776,  -776,  -776,  -776,   353,   354,     0,     0,  -776,
    -776,   124,     0,     0,     0,     0,   222,   361,   362,     0,
     364,   365,   366,   367,     0,   368,   222,     0,     0,     0,
       0,   369,   370,   371,   372,   373,     0,     0,     0,   374,
       0,   124,   222,   760,   769,     0,   222,     0,     0,     0,
       0,     0,     0,   364,   365,   366,   367,     0,   368,   222,
       0,   222,     0,     0,   369,   370,   371,   372,   373,     0,
       0,  -775,     4,     0,     5,     6,     7,     8,     9,     0,
       0,     0,    10,    11,     0,     0,     0,    12,     0,    13,
      14,    15,    16,    17,    18,    19,     0,     0,     0,     0,
       0,    20,    21,    22,    23,    24,    25,    26,     0,   222,
      27,     0,     0,     0,     0,     0,    28,    29,    30,    31,
      32,    33,    34,    35,    36,    37,    38,    39,    40,   222,
      41,    42,    43,    44,    45,    46,    47,     0,     0,     0,
       0,     0,     0,   222,     0,     0,    48,    49,     0,   672,
     957,     0,   222,     0,     0,     0,     0,     0,     0,     0,
       0,    50,    51,     0,     0,     0,     0,     0,     0,    52,
       0,     0,    53,    54,     0,    55,    56,     0,    57,     0,
       0,    58,    59,    60,    61,    62,    63,    64,    65,    66,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
     672,   672,     0,     0,     0,     0,   222,   672,   672,  -775,
       0,  -775,     0,     0,     0,     0,     0,    67,    68,     0,
       0,     0,    69,     0,     0,     0,   222,     0,     5,     6,
       7,     0,     9,     0,     0,     0,    10,    11,     0,     0,
       0,    12,     0,    13,    14,    15,   250,   251,    18,    19,
     672,   672,     0,   672,   672,    20,   252,   253,    23,    24,
      25,    26,     0,     0,   209,     0,     0,     0,     0,     0,
       0,   282,     0,     0,    32,    33,    34,    35,    36,    37,
      38,    39,    40,     0,    41,    42,    43,    44,    45,    46,
      47,     0,     0,     0,  1062,  1063,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,  1069,  1070,     0,     0,
       0,     0,     0,     0,     0,     0,     0,   222,     0,     0,
       0,     0,     0,   283,     0,     0,   212,    54,     0,    55,
      56,     0,     0,     0,     0,    58,    59,    60,    61,    62,
      63,    64,    65,    66,     0,     0,     0,     0,     0,     0,
     284,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,  1120,     0,     0,     0,     0,
       0,     0,   672,     0,     0,     0,   285,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,   222,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,   672,     0,   222,     0,     0,     0,     0,
     262,     0,     5,     6,     7,     8,     9,  -775,  -775,  -775,
      10,    11,     0,     0,  -775,    12,     0,    13,    14,    15,
      16,    17,    18,    19,     0,     0,     0,     0,     0,    20,
      21,    22,    23,    24,    25,    26,     0,     0,    27,     0,
       0,     0,     0,     0,    28,    29,   263,    31,    32,    33,
      34,    35,    36,    37,    38,    39,    40,     0,    41,    42,
      43,    44,    45,    46,    47,     0,     0,   348,   349,   350,
     351,   352,   353,   354,    48,    49,   357,   358,     0,     0,
       0,     0,     0,     0,   361,   362,   222,     0,     0,    50,
      51,     0,     0,     0,     0,     0,     0,    52,     0,     0,
      53,    54,     0,    55,    56,     0,    57,     0,     0,    58,
      59,    60,    61,    62,    63,    64,    65,    66,     0,     0,
     364,   365,   366,   367,     0,   368,     0,     0,     0,     0,
       0,   369,   370,   371,   372,   373,     0,  -775,     0,  -775,
       0,     0,     0,     0,   222,    67,    68,     0,     0,     0,
      69,   262,     0,     5,     6,     7,     8,     9,     0,     0,
    -775,    10,    11,   222,  -775,  -775,    12,     0,    13,    14,
      15,    16,    17,    18,    19,     0,     0,     0,     0,     0,
      20,    21,    22,    23,    24,    25,    26,     0,     0,    27,
       0,     0,     0,     0,     0,    28,    29,   263,    31,    32,
      33,    34,    35,    36,    37,    38,    39,    40,     0,    41,
      42,    43,    44,    45,    46,    47,     0,     0,     0,     0,
       0,     0,     0,     0,     0,    48,    49,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
      50,    51,     0,     0,     0,     0,     0,     0,    52,     0,
       0,    53,    54,     0,    55,    56,     0,    57,     0,     0,
      58,    59,    60,    61,    62,    63,    64,    65,    66,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,  -775,   262,
    -775,     5,     6,     7,     8,     9,    67,    68,  -775,    10,
      11,    69,     0,  -775,    12,  -775,    13,    14,    15,    16,
      17,    18,    19,     0,     0,     0,     0,     0,    20,    21,
      22,    23,    24,    25,    26,     0,     0,    27,     0,     0,
       0,     0,     0,    28,    29,   263,    31,    32,    33,    34,
      35,    36,    37,    38,    39,    40,     0,    41,    42,    43,
      44,    45,    46,    47,     0,     0,     0,     0,     0,     0,
       0,     0,     0,    48,    49,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,    50,    51,
       0,     0,     0,     0,     0,     0,    52,     0,     0,    53,
      54,     0,    55,    56,     0,    57,     0,     0,    58,    59,
      60,    61,    62,    63,    64,    65,    66,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,  -775,   262,  -775,     5,
       6,     7,     8,     9,    67,    68,  -775,    10,    11,    69,
       0,  -775,    12,     0,    13,    14,    15,    16,    17,    18,
      19,  -775,     0,     0,     0,     0,    20,    21,    22,    23,
      24,    25,    26,     0,     0,    27,     0,     0,     0,     0,
       0,    28,    29,   263,    31,    32,    33,    34,    35,    36,
      37,    38,    39,    40,     0,    41,    42,    43,    44,    45,
      46,    47,     0,     0,     0,     0,     0,     0,     0,     0,
       0,    48,    49,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,    50,    51,     0,     0,
       0,     0,     0,     0,    52,     0,     0,    53,    54,     0,
      55,    56,     0,    57,     0,     0,    58,    59,    60,    61,
      62,    63,    64,    65,    66,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,  -775,   262,  -775,     5,     6,     7,
       8,     9,    67,    68,  -775,    10,    11,    69,     0,  -775,
      12,     0,    13,    14,    15,    16,    17,    18,    19,     0,
       0,     0,     0,     0,    20,    21,    22,    23,    24,    25,
      26,     0,     0,    27,     0,     0,     0,     0,     0,    28,
      29,   263,    31,    32,    33,    34,    35,    36,    37,    38,
      39,    40,     0,    41,    42,    43,    44,    45,    46,    47,
       0,     0,     0,     0,     0,     0,     0,     0,     0,    48,
      49,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,    50,    51,     0,     0,     0,     0,
       0,     0,    52,     0,     0,    53,    54,     0,    55,    56,
       0,    57,     0,     0,    58,    59,    60,    61,    62,    63,
      64,    65,    66,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,  -775,     0,  -775,     0,     0,     0,     0,     0,
      67,    68,     0,     0,   262,    69,     5,     6,     7,     8,
       9,     0,  -775,  -775,    10,    11,     0,     0,     0,    12,
       0,    13,    14,    15,    16,    17,    18,    19,     0,     0,
       0,     0,     0,    20,    21,    22,    23,    24,    25,    26,
       0,     0,    27,     0,     0,     0,     0,     0,    28,    29,
     263,    31,    32,    33,    34,    35,    36,    37,    38,    39,
      40,     0,    41,    42,    43,    44,    45,    46,    47,     0,
       0,     0,     0,     0,     0,     0,     0,     0,    48,    49,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,    50,    51,     0,     0,     0,     0,     0,
       0,    52,     0,     0,    53,    54,     0,    55,    56,     0,
      57,     0,     0,    58,    59,    60,    61,    62,    63,    64,
      65,    66,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,  -775,   262,  -775,     5,     6,     7,     8,     9,    67,
      68,     0,    10,    11,    69,     0,     0,    12,     0,    13,
      14,    15,    16,    17,    18,    19,     0,     0,     0,     0,
       0,    20,    21,    22,    23,    24,    25,    26,     0,     0,
      27,     0,     0,     0,     0,     0,    28,    29,   263,    31,
      32,    33,    34,    35,    36,    37,    38,    39,    40,     0,
      41,    42,    43,    44,    45,    46,    47,     0,     0,     0,
       0,     0,     0,     0,     0,     0,    48,    49,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,    50,    51,     0,     0,     0,     0,     0,     0,    52,
       0,     0,   264,    54,  -775,    55,    56,     0,    57,     0,
       0,    58,    59,    60,    61,    62,    63,    64,    65,    66,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,  -775,
     262,  -775,     5,     6,     7,     8,     9,    67,    68,     0,
      10,    11,    69,     0,     0,    12,     0,    13,    14,    15,
      16,    17,    18,    19,     0,     0,     0,     0,     0,    20,
      21,    22,    23,    24,    25,    26,     0,     0,    27,     0,
       0,     0,     0,     0,    28,    29,   263,    31,    32,    33,
      34,    35,    36,    37,    38,    39,    40,     0,    41,    42,
      43,    44,    45,    46,    47,     0,     0,     0,     0,     0,
       0,     0,     0,     0,    48,    49,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,    50,
      51,     0,     0,     0,     0,     0,     0,    52,     0,     0,
      53,    54,  -775,    55,    56,     0,    57,     0,     0,    58,
      59,    60,    61,    62,    63,    64,    65,    66,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,  -775,     4,  -775,
       5,     6,     7,     8,     9,    67,    68,     0,    10,    11,
      69,     0,     0,    12,     0,    13,    14,    15,    16,    17,
      18,    19,     0,     0,     0,     0,     0,    20,    21,    22,
      23,    24,    25,    26,     0,     0,    27,     0,     0,     0,
       0,     0,    28,    29,    30,    31,    32,    33,    34,    35,
      36,    37,    38,    39,    40,     0,    41,    42,    43,    44,
      45,    46,    47,     0,     0,     0,     0,     0,     0,     0,
       0,     0,    48,    49,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,    50,    51,     0,
       0,     0,     0,     0,     0,    52,     0,     0,    53,    54,
       0,    55,    56,     0,    57,     0,     0,    58,    59,    60,
      61,    62,    63,    64,    65,    66,     0,     0,     0,     0,
       0,     0,     0,     0,  -775,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,  -775,   262,  -775,     5,     6,
       7,     8,     9,    67,    68,     0,    10,    11,    69,     0,
       0,    12,     0,    13,    14,    15,    16,    17,    18,    19,
       0,     0,     0,     0,     0,    20,    21,    22,    23,    24,
      25,    26,     0,     0,    27,     0,     0,     0,     0,     0,
      28,    29,   263,    31,    32,    33,    34,    35,    36,    37,
      38,    39,    40,     0,    41,    42,    43,    44,    45,    46,
      47,     0,     0,     0,     0,     0,     0,     0,     0,     0,
      48,    49,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,    50,    51,     0,     0,     0,
       0,     0,     0,    52,     0,     0,    53,    54,     0,    55,
      56,     0,    57,     0,     0,    58,    59,    60,    61,    62,
      63,    64,    65,    66,     0,     0,     0,     0,     0,     0,
       0,     0,  -775,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,  -775,   262,  -775,     5,     6,     7,     8,
       9,    67,    68,  -775,    10,    11,    69,     0,     0,    12,
       0,    13,    14,    15,    16,    17,    18,    19,     0,     0,
       0,     0,     0,    20,    21,    22,    23,    24,    25,    26,
       0,     0,    27,     0,     0,     0,     0,     0,    28,    29,
     263,    31,    32,    33,    34,    35,    36,    37,    38,    39,
      40,     0,    41,    42,    43,    44,    45,    46,    47,     0,
       0,     0,     0,     0,     0,     0,     0,     0,    48,    49,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,    50,    51,     0,     0,     0,     0,     0,
       0,    52,     0,     0,    53,    54,     0,    55,    56,     0,
      57,     0,     0,    58,    59,    60,    61,    62,    63,    64,
      65,    66,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,  -775,   262,  -775,     5,     6,     7,     8,     9,    67,
      68,     0,    10,    11,    69,     0,     0,    12,     0,    13,
      14,    15,    16,    17,    18,    19,     0,     0,     0,     0,
       0,    20,    21,    22,    23,    24,    25,    26,     0,     0,
      27,     0,     0,     0,     0,     0,    28,    29,   263,    31,
      32,    33,    34,    35,    36,    37,    38,    39,    40,     0,
      41,    42,    43,    44,    45,    46,    47,     0,     0,     0,
       0,     0,     0,     0,     0,     0,    48,    49,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,    50,    51,     0,     0,     0,     0,     0,     0,    52,
       0,     0,    53,    54,     0,    55,    56,     0,    57,     0,
       0,    58,    59,    60,    61,    62,    63,    64,    65,    66,
       0,  -775,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,  -752,     0,     0,  -775,
       0,  -775,     0,     0,  -752,  -752,  -752,    67,    68,  -752,
    -752,  -752,    69,  -752,     0,     0,     0,     0,     0,     0,
       0,  -752,  -752,  -752,  -752,  -752,     0,     0,     0,     0,
       0,     0,     0,  -752,  -752,     0,  -752,  -752,  -752,  -752,
    -752,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,  -752,     0,     0,     0,     0,     0,
       0,     0,     0,  -752,  -752,  -752,  -752,  -752,  -752,  -752,
    -752,  -752,  -752,  -752,  -752,  -752,     0,     0,     0,     0,
    -752,  -752,  -752,  -752,     0,   885,  -752,     0,     0,  -752,
       0,     0,  -752,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,  -752,     0,     0,  -752,
       0,  -752,  -752,  -752,  -752,  -123,  -752,  -752,  -752,  -752,
       0,  -752,     0,  -752,  -752,     0,  -752,  -752,  -752,  -752,
    -752,  -752,  -644,     0,     0,  -752,  -752,     0,     0,     0,
    -644,  -644,  -644,     0,     0,  -644,  -644,  -644,     0,  -644,
       0,     0,     0,     0,     0,     0,     0,  -644,     0,  -644,
    -644,  -644,     0,     0,     0,     0,     0,     0,     0,  -644,
    -644,     0,  -644,  -644,  -644,  -644,  -644,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
    -644,     0,     0,     0,     0,     0,     0,     0,     0,  -644,
    -644,  -644,  -644,  -644,  -644,  -644,  -644,  -644,  -644,  -644,
    -644,  -644,     0,     0,     0,     0,  -644,  -644,  -644,  -644,
       0,  -644,  -644,     0,     0,  -644,     0,     0,  -644,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,  -644,     0,     0,  -644,     0,  -644,     0,  -644,
    -644,  -644,  -644,  -644,  -644,  -644,     0,  -644,     0,  -644,
    -644,     0,  -644,  -644,  -644,  -644,  -644,  -644,  -647,     0,
       0,  -644,  -644,     0,     0,     0,  -647,  -647,  -647,     0,
       0,  -647,  -647,  -647,     0,  -647,     0,     0,     0,     0,
       0,     0,     0,  -647,     0,  -647,  -647,  -647,     0,     0,
       0,     0,     0,     0,     0,  -647,  -647,     0,  -647,  -647,
    -647,  -647,  -647,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,  -647,     0,     0,     0,
       0,     0,     0,     0,     0,  -647,  -647,  -647,  -647,  -647,
    -647,  -647,  -647,  -647,  -647,  -647,  -647,  -647,     0,     0,
       0,     0,  -647,  -647,  -647,  -647,     0,  -647,  -647,     0,
       0,  -647,     0,     0,  -647,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,  -647,     0,
       0,  -647,     0,  -647,     0,  -647,  -647,  -647,  -647,  -647,
    -647,  -647,     0,  -647,     0,  -647,  -647,     0,  -647,  -647,
    -647,  -647,  -647,  -647,  -753,     0,     0,  -647,  -647,     0,
       0,     0,  -753,  -753,  -753,     0,     0,  -753,  -753,  -753,
       0,  -753,     0,     0,     0,     0,     0,     0,     0,  -753,
    -753,  -753,  -753,  -753,     0,     0,     0,     0,     0,     0,
       0,  -753,  -753,     0,  -753,  -753,  -753,  -753,  -753,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,  -753,     0,     0,     0,     0,     0,     0,     0,
       0,  -753,  -753,  -753,  -753,  -753,  -753,  -753,  -753,  -753,
    -753,  -753,  -753,  -753,     0,     0,     0,     0,  -753,  -753,
    -753,  -753,     0,     0,  -753,     0,     0,  -753,     0,     0,
    -753,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,  -753,     0,     0,  -753,     0,  -753,
    -753,  -753,  -753,     0,  -753,  -753,  -753,  -753,     0,  -753,
       0,  -753,  -753,     0,  -753,  -753,  -753,  -753,  -753,  -753,
    -754,     0,     0,  -753,  -753,     0,     0,     0,  -754,  -754,
    -754,     0,     0,  -754,  -754,  -754,     0,  -754,     0,     0,
       0,     0,     0,     0,     0,  -754,  -754,  -754,  -754,  -754,
       0,     0,     0,     0,     0,     0,     0,  -754,  -754,     0,
    -754,  -754,  -754,  -754,  -754,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,  -754,     0,
       0,     0,     0,     0,     0,     0,     0,  -754,  -754,  -754,
    -754,  -754,  -754,  -754,  -754,  -754,  -754,  -754,  -754,  -754,
       0,     0,     0,     0,  -754,  -754,  -754,  -754,     0,     0,
    -754,     0,     0,  -754,     0,     0,  -754,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
    -754,     0,     0,  -754,     0,  -754,  -754,  -754,  -754,     0,
    -754,  -754,  -754,  -754,     0,  -754,     0,  -754,  -754,     0,
    -754,  -754,  -754,  -754,  -754,  -754,  -323,     0,     0,  -754,
    -754,     0,     0,     0,  -323,  -323,  -323,     0,     0,  -323,
    -323,  -323,     0,  -323,     0,     0,     0,     0,     0,     0,
       0,  -323,     0,  -323,  -323,  -323,     0,     0,     0,     0,
       0,     0,     0,  -323,  -323,     0,  -323,  -323,  -323,  -323,
    -323,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,  -323,     0,     0,     0,     0,     0,
       0,     0,     0,  -323,  -323,  -323,  -323,  -323,  -323,  -323,
    -323,  -323,  -323,  -323,  -323,  -323,     0,     0,     0,     0,
    -323,  -323,  -323,  -323,     0,   886,  -323,     0,     0,  -323,
       0,     0,  -323,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,  -323,     0,     0,  -323,
       0,  -323,     0,  -323,  -323,  -125,  -323,  -323,  -323,  -323,
       0,  -323,     0,  -323,  -323,     0,  -323,  -323,  -323,  -323,
    -323,  -323,  -315,     0,     0,  -323,  -323,     0,     0,     0,
    -315,  -315,  -315,     0,     0,  -315,  -315,  -315,     0,  -315,
       0,     0,     0,     0,     0,     0,     0,  -315,     0,  -315,
    -315,  -315,     0,     0,     0,     0,     0,     0,     0,  -315,
    -315,     0,  -315,  -315,  -315,  -315,  -315,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
    -315,     0,     0,     0,     0,     0,     0,     0,     0,  -315,
    -315,  -315,  -315,  -315,  -315,  -315,  -315,  -315,  -315,  -315,
    -315,  -315,     0,     0,     0,     0,  -315,  -315,  -315,  -315,
       0,     0,  -315,     0,     0,  -315,     0,     0,  -315,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,  -315,     0,     0,  -315,     0,  -315,     0,  -315,
    -315,     0,  -315,  -315,  -315,  -315,     0,  -315,     0,  -315,
    -315,     0,  -315,  -315,  -315,  -315,  -315,  -315,  -462,     0,
       0,  -315,  -315,     0,     0,     0,  -462,  -462,  -462,     0,
       0,  -462,  -462,  -462,     0,  -462,     0,     0,     0,     0,
       0,     0,     0,  -462,  -462,  -462,  -462,     0,     0,     0,
       0,     0,     0,     0,     0,  -462,  -462,     0,  -462,  -462,
    -462,  -462,  -462,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,  -462,     0,     0,     0,
       0,     0,     0,     0,     0,  -462,  -462,  -462,  -462,  -462,
    -462,  -462,  -462,  -462,  -462,  -462,  -462,  -462,     0,     0,
       0,     0,  -462,  -462,  -462,  -462,     0,     0,  -462,     0,
       0,  -462,     0,     0,  -462,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,  -462,     0,
       0,     0,     0,  -462,  -462,  -462,  -462,     0,  -462,  -462,
    -462,  -462,     0,  -462,   238,  -462,  -462,     0,  -462,  -462,
    -462,  -462,  -462,  -462,  -775,     0,     0,  -462,     0,     0,
       0,     0,  -775,  -775,  -775,     0,     0,  -775,  -775,  -775,
       0,  -775,     0,     0,     0,     0,     0,     0,     0,  -775,
    -775,  -775,  -775,     0,     0,     0,     0,     0,     0,     0,
       0,  -775,  -775,     0,  -775,  -775,  -775,  -775,  -775,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,  -775,     0,     0,     0,     0,     0,     0,     0,
       0,  -775,  -775,  -775,  -775,  -775,  -775,  -775,  -775,  -775,
    -775,  -775,  -775,  -775,     0,     0,     0,     0,  -775,  -775,
    -775,  -775,     0,     0,  -775,     0,     0,  -775,     0,     0,
    -775,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,  -775,     0,     0,     0,     0,  -775,
    -775,  -775,  -775,     0,  -775,  -775,  -775,  -775,     0,  -775,
     238,  -775,  -775,     0,  -775,  -775,  -775,  -775,  -775,  -775,
    -330,     0,     0,  -775,     0,     0,     0,     0,  -330,  -330,
    -330,     0,     0,  -330,  -330,  -330,     0,  -330,     0,     0,
       0,     0,     0,     0,     0,  -330,     0,  -330,  -330,     0,
       0,     0,     0,     0,     0,     0,     0,  -330,  -330,     0,
    -330,  -330,  -330,  -330,  -330,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,  -330,     0,
       0,     0,     0,     0,     0,     0,     0,  -330,  -330,  -330,
    -330,  -330,  -330,  -330,  -330,  -330,  -330,  -330,  -330,  -330,
       0,     0,     0,     0,  -330,  -330,  -330,  -330,     0,     0,
    -330,     0,     0,  -330,     0,     0,  -330,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
    -330,     0,     0,     0,     0,  -330,     0,  -330,  -330,     0,
    -330,  -330,  -330,  -330,     0,  -330,   235,  -330,  -330,     0,
    -330,  -330,  -330,  -330,  -330,  -330,  -752,     0,     0,  -330,
       0,     0,     0,     0,  -752,  -752,  -752,     0,     0,     0,
    -752,  -752,     0,  -752,     0,     0,     0,     0,     0,     0,
       0,  -752,  -752,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,  -752,  -752,     0,  -752,  -752,  -752,  -752,
    -752,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,  -752,     0,     0,     0,     0,     0,
       0,     0,     0,  -752,  -752,  -752,  -752,  -752,  -752,  -752,
    -752,  -752,  -752,  -752,  -752,  -752,     0,     0,     0,     0,
    -752,  -752,  -752,  -752,     0,   835,  -752,     0,     0,  -752,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,  -752,     0,     0,     0,
       0,  -114,  -752,  -752,  -752,  -123,  -752,  -752,  -752,  -752,
       0,  -752,     0,     0,  -752,     0,  -752,  -752,  -752,  -752,
    -752,  -752,  -752,     0,     0,  -752,     0,     0,     0,     0,
    -752,  -752,  -752,     0,     0,     0,  -752,  -752,     0,  -752,
       0,     0,     0,     0,     0,     0,     0,  -752,  -752,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,  -752,
    -752,     0,  -752,  -752,  -752,  -752,  -752,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
    -752,     0,     0,     0,     0,     0,     0,     0,     0,  -752,
    -752,  -752,  -752,  -752,  -752,  -752,  -752,  -752,  -752,  -752,
    -752,  -752,     0,     0,     0,     0,  -752,  -752,  -752,  -752,
       0,   835,  -752,     0,     0,  -752,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,  -752,     0,     0,     0,     0,  -752,  -752,  -752,
    -752,  -123,  -752,  -752,  -752,  -752,     0,  -752,     0,     0,
    -752,     0,  -752,  -752,  -752,  -752,  -752,  -752,  -323,     0,
       0,  -752,     0,     0,     0,     0,  -323,  -323,  -323,     0,
       0,     0,  -323,  -323,     0,  -323,     0,     0,     0,     0,
       0,     0,     0,  -323,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,  -323,  -323,     0,  -323,  -323,
    -323,  -323,  -323,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,  -323,     0,     0,     0,
       0,     0,     0,     0,     0,  -323,  -323,  -323,  -323,  -323,
    -323,  -323,  -323,  -323,  -323,  -323,  -323,  -323,     0,     0,
       0,     0,  -323,  -323,  -323,  -323,     0,   836,  -323,     0,
       0,  -323,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,  -323,     0,
       0,     0,     0,  -116,     0,  -323,  -323,  -125,  -323,  -323,
    -323,  -323,     0,  -323,     0,     0,  -323,     0,  -323,  -323,
    -323,  -323,  -323,  -323,  -323,     0,     0,  -323,     0,     0,
       0,     0,  -323,  -323,  -323,     0,     0,     0,  -323,  -323,
       0,  -323,     0,     0,     0,     0,     0,     0,     0,  -323,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,  -323,  -323,     0,  -323,  -323,  -323,  -323,  -323,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,  -323,     0,     0,     0,     0,     0,     0,     0,
       0,  -323,  -323,  -323,  -323,  -323,  -323,  -323,  -323,  -323,
    -323,  -323,  -323,  -323,     0,     0,     0,     0,  -323,  -323,
    -323,  -323,     0,   836,  -323,     0,     0,  -323,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,  -323,     0,     0,     0,     0,  -323,
       0,  -323,  -323,  -125,  -323,  -323,  -323,  -323,     0,  -323,
       0,     0,  -323,     0,  -323,  -323,  -323,  -323,  -323,  -323,
       0,     0,     0,  -323,     5,     6,     7,     8,     9,     0,
       0,     0,    10,    11,     0,     0,     0,    12,     0,    13,
      14,    15,    16,    17,    18,    19,     0,     0,     0,     0,
       0,    20,    21,    22,    23,    24,    25,    26,     0,     0,
      27,     0,     0,     0,     0,     0,    28,    29,    30,    31,
      32,    33,    34,    35,    36,    37,    38,    39,    40,     0,
      41,    42,    43,    44,    45,    46,    47,     0,     0,     0,
       0,     0,     0,     0,     0,     0,    48,    49,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,    50,    51,     0,     0,     0,     0,     0,     0,    52,
       0,     0,    53,    54,     0,    55,    56,     0,    57,     0,
       0,    58,    59,    60,    61,    62,    63,    64,    65,    66,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,   521,
       0,     5,     6,     7,     8,     9,     0,    67,    68,    10,
      11,     0,    69,     0,    12,     0,    13,    14,    15,    16,
      17,    18,    19,     0,     0,     0,     0,     0,    20,    21,
      22,    23,    24,    25,    26,     0,     0,    27,     0,     0,
       0,     0,     0,    28,    29,   263,    31,    32,    33,    34,
      35,    36,    37,    38,    39,    40,     0,    41,    42,    43,
      44,    45,    46,    47,     0,     0,     0,     0,     0,     0,
       0,     0,     0,    48,    49,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,    50,    51,
       0,     0,     0,     0,     0,     0,    52,     0,     0,    53,
      54,     0,    55,    56,     0,    57,     0,     0,    58,    59,
      60,    61,    62,    63,    64,    65,    66,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,   521,     0,     5,     6,
       7,     8,     9,     0,    67,    68,    10,    11,     0,    69,
       0,    12,     0,    13,    14,    15,    16,    17,    18,    19,
       0,     0,     0,     0,     0,    20,    21,    22,    23,    24,
      25,    26,     0,     0,    27,     0,     0,     0,     0,     0,
      28,    29,    30,    31,    32,    33,    34,    35,    36,    37,
      38,    39,    40,     0,    41,    42,    43,    44,    45,    46,
      47,     0,     0,     0,     0,     0,     0,     0,     0,     0,
      48,    49,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,    50,    51,     0,     0,     0,
       0,     0,     0,    52,     0,     0,    53,    54,     0,    55,
      56,     0,    57,     0,     0,    58,    59,    60,    61,    62,
      63,    64,    65,    66,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     5,     6,     7,     0,     9,
       0,    67,    68,    10,    11,     0,    69,     0,    12,     0,
      13,    14,    15,    16,    17,    18,    19,     0,     0,     0,
       0,     0,    20,    21,    22,    23,    24,    25,    26,     0,
       0,   209,     0,     0,     0,     0,     0,     0,    29,     0,
       0,    32,    33,    34,    35,    36,    37,    38,    39,    40,
     210,    41,    42,    43,    44,    45,    46,    47,     0,     0,
       0,     0,     0,     0,     0,     0,     0,    48,    49,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,    50,    51,     0,     0,     0,     0,     0,     0,
     211,     0,     0,   212,    54,     0,    55,    56,     0,   213,
     214,   215,    58,    59,   216,    61,    62,    63,    64,    65,
      66,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,   242,     5,     6,     7,     0,     9,    67,   217,
       0,    10,    11,    69,     0,     0,    12,     0,    13,    14,
      15,    16,    17,    18,    19,     0,     0,     0,     0,     0,
      20,    21,    22,    23,    24,    25,    26,     0,     0,   209,
       0,     0,     0,     0,     0,     0,    29,     0,     0,    32,
      33,    34,    35,    36,    37,    38,    39,    40,   210,    41,
      42,    43,    44,    45,    46,    47,     0,     0,     0,     0,
       0,     0,     0,     0,     0,    48,    49,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
      50,    51,     0,     0,     0,     0,     0,     0,   211,     0,
       0,   212,    54,     0,    55,    56,     0,   213,   214,   215,
      58,    59,   216,    61,    62,    63,    64,    65,    66,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       5,     6,     7,     8,     9,     0,    67,   217,    10,    11,
       0,    69,     0,    12,     0,    13,    14,    15,    16,    17,
      18,    19,     0,     0,     0,     0,     0,    20,    21,    22,
      23,    24,    25,    26,     0,     0,    27,     0,     0,     0,
       0,     0,    28,    29,     0,    31,    32,    33,    34,    35,
      36,    37,    38,    39,    40,     0,    41,    42,    43,    44,
      45,    46,    47,     0,     0,     0,     0,     0,     0,     0,
       0,     0,    48,    49,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,    50,    51,     0,
       0,     0,     0,     0,     0,    52,     0,     0,    53,    54,
       0,    55,    56,     0,    57,     0,     0,    58,    59,    60,
      61,    62,    63,    64,    65,    66,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     5,     6,     7,
       0,     9,     0,    67,    68,    10,    11,     0,    69,     0,
      12,     0,    13,    14,    15,    16,    17,    18,    19,     0,
       0,     0,     0,     0,    20,    21,    22,    23,    24,    25,
      26,     0,     0,   209,     0,     0,     0,     0,     0,     0,
      29,     0,     0,    32,    33,    34,    35,    36,    37,    38,
      39,    40,   210,    41,    42,    43,    44,    45,    46,    47,
       0,     0,     0,     0,     0,     0,     0,     0,     0,    48,
      49,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,    50,   458,     0,     0,     0,     0,
       0,     0,   211,     0,     0,   212,    54,     0,    55,    56,
       0,   213,   214,   215,    58,    59,   216,    61,    62,    63,
      64,    65,    66,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     5,     6,     7,     0,     9,     0,
      67,   217,    10,    11,     0,    69,     0,    12,     0,    13,
      14,    15,   250,   251,    18,    19,     0,     0,     0,     0,
       0,    20,   252,   253,    23,    24,    25,    26,     0,     0,
     209,     0,     0,     0,     0,     0,     0,    29,     0,     0,
      32,    33,    34,    35,    36,    37,    38,    39,    40,   210,
      41,    42,    43,    44,    45,    46,    47,     0,     0,     0,
       0,     0,     0,     0,     0,     0,    48,    49,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,    50,    51,     0,     0,     0,     0,     0,     0,   211,
       0,     0,   212,    54,     0,    55,    56,     0,   681,   214,
     215,    58,    59,   216,    61,    62,    63,    64,    65,    66,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     5,     6,     7,     0,     9,     0,    67,   217,    10,
      11,     0,    69,     0,    12,     0,    13,    14,    15,   250,
     251,    18,    19,     0,     0,     0,     0,     0,    20,   252,
     253,    23,    24,    25,    26,     0,     0,   209,     0,     0,
       0,     0,     0,     0,    29,     0,     0,    32,    33,    34,
      35,    36,    37,    38,    39,    40,   210,    41,    42,    43,
      44,    45,    46,    47,     0,     0,     0,     0,     0,     0,
       0,     0,     0,    48,    49,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,    50,   458,
       0,     0,     0,     0,     0,     0,   211,     0,     0,   212,
      54,     0,    55,    56,     0,   681,   214,   215,    58,    59,
     216,    61,    62,    63,    64,    65,    66,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     5,     6,
       7,     0,     9,     0,    67,   217,    10,    11,     0,    69,
       0,    12,     0,    13,    14,    15,   250,   251,    18,    19,
       0,     0,     0,     0,     0,    20,   252,   253,    23,    24,
      25,    26,     0,     0,   209,     0,     0,     0,     0,     0,
       0,    29,     0,     0,    32,    33,    34,    35,    36,    37,
      38,    39,    40,   210,    41,    42,    43,    44,    45,    46,
      47,     0,     0,     0,     0,     0,     0,     0,     0,     0,
      48,    49,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,    50,    51,     0,     0,     0,
       0,     0,     0,   211,     0,     0,   212,    54,     0,    55,
      56,     0,   213,   214,     0,    58,    59,   216,    61,    62,
      63,    64,    65,    66,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     5,     6,     7,     0,     9,
       0,    67,   217,    10,    11,     0,    69,     0,    12,     0,
      13,    14,    15,   250,   251,    18,    19,     0,     0,     0,
       0,     0,    20,   252,   253,    23,    24,    25,    26,     0,
       0,   209,     0,     0,     0,     0,     0,     0,    29,     0,
       0,    32,    33,    34,    35,    36,    37,    38,    39,    40,
     210,    41,    42,    43,    44,    45,    46,    47,     0,     0,
       0,     0,     0,     0,     0,     0,     0,    48,    49,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,    50,    51,     0,     0,     0,     0,     0,     0,
     211,     0,     0,   212,    54,     0,    55,    56,     0,     0,
     214,   215,    58,    59,   216,    61,    62,    63,    64,    65,
      66,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     5,     6,     7,     0,     9,     0,    67,   217,
      10,    11,     0,    69,     0,    12,     0,    13,    14,    15,
     250,   251,    18,    19,     0,     0,     0,     0,     0,    20,
     252,   253,    23,    24,    25,    26,     0,     0,   209,     0,
       0,     0,     0,     0,     0,    29,     0,     0,    32,    33,
      34,    35,    36,    37,    38,    39,    40,   210,    41,    42,
      43,    44,    45,    46,    47,     0,     0,     0,     0,     0,
       0,     0,     0,     0,    48,    49,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,    50,
      51,     0,     0,     0,     0,     0,     0,   211,     0,     0,
     212,    54,     0,    55,    56,     0,   681,   214,     0,    58,
      59,   216,    61,    62,    63,    64,    65,    66,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     5,
       6,     7,     0,     9,     0,    67,   217,    10,    11,     0,
      69,     0,    12,     0,    13,    14,    15,   250,   251,    18,
      19,     0,     0,     0,     0,     0,    20,   252,   253,    23,
      24,    25,    26,     0,     0,   209,     0,     0,     0,     0,
       0,     0,    29,     0,     0,    32,    33,    34,    35,    36,
      37,    38,    39,    40,   210,    41,    42,    43,    44,    45,
      46,    47,     0,     0,     0,     0,     0,     0,     0,     0,
       0,    48,    49,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,    50,    51,     0,     0,
       0,     0,     0,     0,   211,     0,     0,   212,    54,     0,
      55,    56,     0,     0,   214,     0,    58,    59,   216,    61,
      62,    63,    64,    65,    66,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     5,     6,     7,     0,
       9,     0,    67,   217,    10,    11,     0,    69,     0,    12,
       0,    13,    14,    15,    16,    17,    18,    19,     0,     0,
       0,     0,     0,    20,    21,    22,    23,    24,    25,    26,
       0,     0,    27,     0,     0,     0,     0,     0,     0,    29,
       0,     0,    32,    33,    34,    35,    36,    37,    38,    39,
      40,     0,    41,    42,    43,    44,    45,    46,    47,     0,
       0,     0,     0,     0,     0,     0,     0,     0,    48,    49,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,    50,    51,     0,     0,     0,     0,     0,
       0,   211,     0,     0,   212,    54,     0,    55,    56,     0,
       0,     0,     0,    58,    59,    60,    61,    62,    63,    64,
      65,    66,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,   312,     0,   313,     5,     6,     7,     0,     9,    67,
      68,     0,    10,    11,    69,     0,     0,    12,     0,    13,
      14,    15,    16,    17,    18,    19,     0,     0,     0,     0,
       0,    20,    21,    22,    23,    24,    25,    26,     0,     0,
      27,     0,     0,     0,     0,     0,     0,    29,     0,     0,
      32,    33,    34,    35,    36,    37,    38,    39,    40,     0,
      41,    42,    43,    44,    45,    46,    47,     0,     0,     0,
       0,     0,     0,     0,     0,     0,    48,    49,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,    50,    51,     0,     0,     0,     0,     0,     0,   211,
       0,     0,   212,    54,     0,    55,    56,     0,     0,     0,
       0,    58,    59,    60,    61,    62,    63,    64,    65,    66,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,   242,     5,     6,     7,     0,     9,    67,    68,     0,
      10,    11,    69,     0,     0,    12,     0,    13,    14,    15,
      16,    17,    18,    19,     0,     0,     0,     0,     0,    20,
      21,    22,    23,    24,    25,    26,     0,     0,   209,     0,
       0,     0,     0,     0,     0,    29,     0,     0,    32,    33,
      34,    35,    36,    37,    38,    39,    40,     0,    41,    42,
      43,    44,    45,    46,    47,     0,     0,     0,     0,     0,
       0,     0,     0,     0,    48,    49,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,    50,
      51,     0,     0,     0,     0,     0,     0,   211,     0,     0,
     212,    54,     0,    55,    56,     0,   577,     0,     0,    58,
      59,    60,    61,    62,    63,    64,    65,    66,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     5,
       6,     7,     0,     9,     0,    67,   217,    10,    11,     0,
      69,     0,    12,     0,    13,    14,    15,   250,   251,    18,
      19,     0,     0,     0,     0,     0,    20,   252,   253,    23,
      24,    25,    26,     0,     0,   209,     0,     0,     0,     0,
       0,     0,    29,     0,     0,    32,    33,    34,    35,    36,
      37,    38,    39,    40,     0,    41,    42,    43,    44,    45,
      46,    47,     0,     0,     0,     0,     0,     0,     0,     0,
       0,    48,    49,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,    50,    51,     0,     0,
       0,     0,     0,     0,   211,     0,     0,   212,    54,     0,
      55,    56,     0,   577,     0,     0,    58,    59,    60,    61,
      62,    63,    64,    65,    66,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     5,     6,     7,     0,
       9,     0,    67,   217,    10,    11,     0,    69,     0,    12,
       0,    13,    14,    15,   250,   251,    18,    19,     0,     0,
       0,     0,     0,    20,   252,   253,    23,    24,    25,    26,
       0,     0,   209,     0,     0,     0,     0,     0,     0,    29,
       0,     0,    32,    33,    34,    35,    36,    37,    38,    39,
      40,     0,    41,    42,    43,    44,    45,    46,    47,     0,
       0,     0,     0,     0,     0,     0,     0,     0,    48,    49,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,    50,    51,     0,     0,     0,     0,     0,
       0,   211,     0,     0,   212,    54,     0,    55,    56,     0,
     952,     0,     0,    58,    59,    60,    61,    62,    63,    64,
      65,    66,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     5,     6,     7,     0,     9,     0,    67,
     217,    10,    11,     0,    69,     0,    12,     0,    13,    14,
      15,   250,   251,    18,    19,     0,     0,     0,     0,     0,
      20,   252,   253,    23,    24,    25,    26,     0,     0,   209,
       0,     0,     0,     0,     0,     0,    29,     0,     0,    32,
      33,    34,    35,    36,    37,    38,    39,    40,     0,    41,
      42,    43,    44,    45,    46,    47,     0,     0,     0,     0,
       0,     0,     0,     0,     0,    48,    49,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
      50,    51,     0,     0,     0,     0,     0,     0,   211,     0,
       0,   212,    54,     0,    55,    56,     0,  1023,     0,     0,
      58,    59,    60,    61,    62,    63,    64,    65,    66,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       5,     6,     7,     0,     9,     0,    67,   217,    10,    11,
       0,    69,     0,    12,     0,    13,    14,    15,   250,   251,
      18,    19,     0,     0,     0,     0,     0,    20,   252,   253,
      23,    24,    25,    26,     0,     0,   209,     0,     0,     0,
       0,     0,     0,    29,     0,     0,    32,    33,    34,    35,
      36,    37,    38,    39,    40,     0,    41,    42,    43,    44,
      45,    46,    47,     0,     0,     0,     0,     0,     0,     0,
       0,     0,    48,    49,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,    50,    51,     0,
       0,     0,     0,     0,     0,   211,     0,     0,   212,    54,
       0,    55,    56,     0,  1189,     0,     0,    58,    59,    60,
      61,    62,    63,    64,    65,    66,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     5,     6,     7,
       0,     9,     0,    67,   217,    10,    11,     0,    69,     0,
      12,     0,    13,    14,    15,   250,   251,    18,    19,     0,
       0,     0,     0,     0,    20,   252,   253,    23,    24,    25,
      26,     0,     0,   209,     0,     0,     0,     0,     0,     0,
      29,     0,     0,    32,    33,    34,    35,    36,    37,    38,
      39,    40,     0,    41,    42,    43,    44,    45,    46,    47,
       0,     0,     0,     0,     0,     0,     0,     0,     0,    48,
      49,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,    50,    51,     0,     0,     0,     0,
       0,     0,   211,     0,     0,   212,    54,     0,    55,    56,
       0,     0,     0,     0,    58,    59,    60,    61,    62,    63,
      64,    65,    66,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     5,     6,     7,     0,     9,     0,
      67,   217,    10,    11,     0,    69,     0,    12,     0,    13,
      14,    15,    16,    17,    18,    19,     0,     0,     0,     0,
       0,    20,    21,    22,    23,    24,    25,    26,     0,     0,
     209,     0,     0,     0,     0,     0,     0,    29,     0,     0,
      32,    33,    34,    35,    36,    37,    38,    39,    40,     0,
      41,    42,    43,    44,    45,    46,    47,     0,     0,     0,
       0,     0,     0,     0,     0,     0,    48,    49,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,    50,    51,     0,     0,     0,     0,     0,     0,   211,
       0,     0,   212,    54,     0,    55,    56,     0,     0,     0,
       0,    58,    59,    60,    61,    62,    63,    64,    65,    66,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     5,     6,     7,     0,     9,     0,    67,   217,    10,
      11,     0,    69,     0,    12,     0,    13,    14,    15,    16,
      17,    18,    19,     0,     0,     0,     0,     0,    20,    21,
      22,    23,    24,    25,    26,     0,     0,    27,     0,     0,
       0,     0,     0,     0,    29,     0,     0,    32,    33,    34,
      35,    36,    37,    38,    39,    40,     0,    41,    42,    43,
      44,    45,    46,    47,     0,     0,     0,     0,     0,     0,
       0,     0,     0,    48,    49,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,    50,    51,
       0,     0,     0,     0,     0,     0,   211,     0,     0,   212,
      54,     0,    55,    56,     0,     0,     0,     0,    58,    59,
      60,    61,    62,    63,    64,    65,    66,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     5,     6,
       7,     0,     9,     0,    67,    68,    10,    11,     0,    69,
       0,    12,     0,    13,    14,    15,   250,   251,    18,    19,
       0,     0,     0,     0,     0,    20,   252,   253,    23,    24,
      25,    26,     0,     0,   209,     0,     0,     0,     0,     0,
       0,   282,     0,     0,    32,    33,    34,    35,    36,    37,
      38,    39,    40,     0,    41,    42,    43,    44,    45,    46,
      47,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,   283,     0,     0,   343,    54,     0,    55,
      56,     0,   344,     0,     0,    58,    59,    60,    61,    62,
      63,    64,    65,    66,     0,     0,     0,     0,     0,     0,
       5,     6,     7,     0,     9,     0,     0,     0,    10,    11,
       0,     0,     0,    12,     0,    13,    14,    15,   250,   251,
      18,    19,     0,     0,     0,     0,   285,    20,   252,   253,
      23,    24,    25,    26,     0,     0,   209,     0,     0,     0,
       0,     0,     0,   282,     0,     0,    32,    33,    34,    35,
      36,    37,    38,    39,    40,     0,    41,    42,    43,    44,
      45,    46,    47,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,   392,     0,     0,    53,    54,
       0,    55,    56,     0,    57,     0,     0,    58,    59,    60,
      61,    62,    63,    64,    65,    66,     0,     0,     0,     0,
       0,     0,     5,     6,     7,     0,     9,     0,     0,     0,
      10,    11,     0,     0,     0,    12,     0,    13,    14,    15,
     250,   251,    18,    19,     0,     0,     0,     0,   285,    20,
     252,   253,    23,    24,    25,    26,     0,     0,   209,     0,
       0,     0,     0,     0,     0,   282,     0,     0,    32,    33,
      34,   400,    36,    37,    38,   401,    40,     0,    41,    42,
      43,    44,    45,    46,    47,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,   402,     0,     0,     0,   403,     0,     0,
     212,    54,     0,    55,    56,     0,     0,     0,     0,    58,
      59,    60,    61,    62,    63,    64,    65,    66,     0,     0,
       0,     0,     0,     0,     5,     6,     7,     0,     9,     0,
       0,     0,    10,    11,     0,     0,     0,    12,     0,    13,
      14,    15,   250,   251,    18,    19,     0,     0,     0,     0,
     285,    20,   252,   253,    23,    24,    25,    26,     0,     0,
     209,     0,     0,     0,     0,     0,     0,   282,     0,     0,
      32,    33,    34,    35,    36,    37,    38,    39,    40,     0,
      41,    42,    43,    44,    45,    46,    47,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,   283,
       0,     0,   212,    54,     0,    55,    56,     0,     0,     0,
       0,    58,    59,    60,    61,    62,    63,    64,    65,    66,
       0,     0,     0,     0,     0,     0,   584,     5,     6,     7,
       0,     9,     0,     0,     0,    10,    11,     0,     0,     0,
      12,     0,    13,    14,    15,   250,   251,    18,    19,     0,
       0,     0,   285,     0,    20,   252,   253,    23,    24,    25,
      26,     0,     0,   209,     0,     0,     0,     0,     0,     0,
     282,     0,     0,    32,    33,    34,   400,    36,    37,    38,
     401,    40,     0,    41,    42,    43,    44,    45,    46,    47,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,   403,     0,     0,   212,    54,     0,    55,    56,
       0,     0,     0,     0,    58,    59,    60,    61,    62,    63,
      64,    65,    66,     0,     0,     0,     0,     0,     0,     5,
       6,     7,     0,     9,     0,     0,     0,    10,    11,     0,
       0,     0,    12,     0,    13,    14,    15,   250,   251,    18,
      19,     0,     0,     0,     0,   285,    20,   252,   253,    23,
      24,    25,    26,     0,     0,   209,     0,     0,     0,     0,
       0,     0,   282,     0,     0,    32,    33,    34,    35,    36,
      37,    38,    39,    40,     0,    41,    42,    43,    44,    45,
      46,    47,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,   283,     0,     0,   343,    54,     0,
      55,    56,     0,     0,     0,     0,    58,    59,    60,    61,
      62,    63,    64,    65,    66,     0,     0,     0,     0,     0,
       0,     5,     6,     7,     0,     9,     0,     0,     0,    10,
      11,     0,     0,     0,    12,     0,    13,    14,    15,   250,
     251,    18,    19,     0,     0,     0,     0,   285,    20,   252,
     253,    23,    24,    25,    26,     0,     0,   209,     0,     0,
       0,     0,     0,     0,   282,     0,     0,    32,    33,    34,
      35,    36,    37,    38,    39,    40,     0,    41,    42,    43,
      44,    45,    46,    47,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,  1148,     0,     0,   212,
      54,     0,    55,    56,     0,     0,     0,     0,    58,    59,
      60,    61,    62,    63,    64,    65,    66,     0,     0,     0,
       0,     0,     0,     5,     6,     7,     0,     9,     0,     0,
       0,    10,    11,     0,     0,     0,    12,     0,    13,    14,
      15,   250,   251,    18,    19,     0,     0,     0,     0,   285,
      20,   252,   253,    23,    24,    25,    26,     0,     0,   209,
       0,     0,     0,     0,     0,     0,   282,     0,     0,    32,
      33,    34,    35,    36,    37,    38,    39,    40,     0,    41,
      42,    43,    44,    45,    46,    47,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,  1163,     0,
       0,   212,    54,     0,    55,    56,     0,     0,     0,     0,
      58,    59,    60,    61,    62,    63,    64,    65,    66,     0,
       0,     0,     0,     0,     0,   129,   130,   131,   132,   133,
     134,   135,   136,   137,   138,   139,   140,   141,   142,   143,
     144,   145,   146,   147,   148,   149,   150,   151,   152,     0,
       0,   285,   153,   154,   155,   411,   412,   413,   414,   160,
     161,   162,     0,     0,     0,     0,     0,   163,   164,   165,
     166,   415,   416,   417,   418,   171,    37,    38,   419,    40,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,   173,   174,   175,
     176,   177,   178,   179,   180,   181,     0,     0,   182,   183,
       0,     0,     0,     0,   184,   185,   186,   187,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,   188,
     189,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,   190,   191,   192,   193,   194,   195,   420,     0,
       0,     0,     0,   196,   197,   198,   199,   200,   201,   202,
     129,   130,   131,   132,   133,   134,   135,   136,   137,   138,
     139,   140,   141,   142,   143,   144,   145,   146,   147,   148,
     149,   150,   151,   152,     0,     0,     0,   153,   154,   155,
     156,   157,   158,   159,   160,   161,   162,     0,     0,     0,
       0,     0,   163,   164,   165,   166,   167,   168,   169,   170,
     171,   295,   296,   172,   297,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,   173,   174,   175,   176,   177,   178,   179,   180,
     181,     0,     0,   182,   183,     0,     0,     0,     0,   184,
     185,   186,   187,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,   188,   189,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,   190,   191,   192,
     193,   194,   195,     0,     0,     0,     0,     0,   196,   197,
     198,   199,   200,   201,   202,   129,   130,   131,   132,   133,
     134,   135,   136,   137,   138,   139,   140,   141,   142,   143,
     144,   145,   146,   147,   148,   149,   150,   151,   152,     0,
       0,     0,   153,   154,   155,   156,   157,   158,   159,   160,
     161,   162,     0,     0,     0,     0,     0,   163,   164,   165,
     166,   167,   168,   169,   170,   171,   244,     0,   172,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,   173,   174,   175,
     176,   177,   178,   179,   180,   181,     0,     0,   182,   183,
       0,     0,     0,     0,   184,   185,   186,   187,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,   188,
     189,     0,     0,    59,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,   190,   191,   192,   193,   194,   195,     0,     0,
       0,     0,     0,   196,   197,   198,   199,   200,   201,   202,
     129,   130,   131,   132,   133,   134,   135,   136,   137,   138,
     139,   140,   141,   142,   143,   144,   145,   146,   147,   148,
     149,   150,   151,   152,     0,     0,     0,   153,   154,   155,
     156,   157,   158,   159,   160,   161,   162,     0,     0,     0,
       0,     0,   163,   164,   165,   166,   167,   168,   169,   170,
     171,     0,     0,   172,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,   173,   174,   175,   176,   177,   178,   179,   180,
     181,     0,     0,   182,   183,     0,     0,     0,     0,   184,
     185,   186,   187,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,   188,   189,     0,     0,    59,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,   190,   191,   192,
     193,   194,   195,     0,     0,     0,     0,     0,   196,   197,
     198,   199,   200,   201,   202,   129,   130,   131,   132,   133,
     134,   135,   136,   137,   138,   139,   140,   141,   142,   143,
     144,   145,   146,   147,   148,   149,   150,   151,   152,     0,
       0,     0,   153,   154,   155,   156,   157,   158,   159,   160,
     161,   162,     0,     0,     0,     0,     0,   163,   164,   165,
     166,   167,   168,   169,   170,   171,     0,     0,   172,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,   173,   174,   175,
     176,   177,   178,   179,   180,   181,     0,     0,   182,   183,
       0,     0,     0,     0,   184,   185,   186,   187,    23,    24,
      25,    26,     0,     0,     0,     0,     0,     0,     0,   188,
     189,     0,     0,     0,    32,    33,    34,   789,     0,     0,
       0,   790,     0,     0,    41,    42,    43,    44,    45,     0,
       0,     0,   190,   191,   192,   193,   194,   195,     0,     0,
       0,     0,     0,   196,   197,   198,   199,   200,   201,   202,
       0,   942,     0,     0,     0,   792,   793,     0,     0,     0,
       0,     0,     0,   794,     0,     0,   795,     0,     0,   796,
     797,     0,     0,     0,     0,    58,    59,    60,    61,    62,
      63,    64,    65,    66,   348,   349,   350,   351,   352,   353,
     354,   355,   356,   357,   358,   359,   360,     0,     0,     0,
     800,   361,   362,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,   285,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,   364,   365,   366,
     367,     0,   368,   623,   624,     0,     0,   625,   369,   370,
     371,   372,   373,     0,     0,     0,   374,     0,     0,     0,
       0,     0,     0,     0,     0,     0,   173,   174,   175,   176,
     177,   178,   179,   180,   181,     0,     0,   182,   183,     0,
       0,     0,     0,   184,   185,   186,   187,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,   188,   189,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,   633,   634,     0,     0,
     635,   190,   191,   192,   193,   194,   195,   238,     0,     0,
       0,     0,   196,   197,   198,   199,   200,   201,   202,   173,
     174,   175,   176,   177,   178,   179,   180,   181,     0,     0,
     182,   183,     0,     0,     0,     0,   184,   185,   186,   187,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,   188,   189,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,   685,
     624,     0,     0,   686,   190,   191,   192,   193,   194,   195,
     238,     0,     0,     0,     0,   196,   197,   198,   199,   200,
     201,   202,   173,   174,   175,   176,   177,   178,   179,   180,
     181,     0,     0,   182,   183,     0,     0,     0,     0,   184,
     185,   186,   187,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,   188,   189,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,   688,   634,     0,     0,   689,   190,   191,   192,
     193,   194,   195,   238,     0,     0,     0,     0,   196,   197,
     198,   199,   200,   201,   202,   173,   174,   175,   176,   177,
     178,   179,   180,   181,     0,     0,   182,   183,     0,     0,
       0,     0,   184,   185,   186,   187,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,   188,   189,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,   713,   624,     0,     0,   714,
     190,   191,   192,   193,   194,   195,   238,     0,     0,     0,
       0,   196,   197,   198,   199,   200,   201,   202,   173,   174,
     175,   176,   177,   178,   179,   180,   181,     0,     0,   182,
     183,     0,     0,     0,     0,   184,   185,   186,   187,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
     188,   189,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,   716,   634,
       0,     0,   717,   190,   191,   192,   193,   194,   195,   238,
       0,     0,     0,     0,   196,   197,   198,   199,   200,   201,
     202,   173,   174,   175,   176,   177,   178,   179,   180,   181,
       0,     0,   182,   183,     0,     0,     0,     0,   184,   185,
     186,   187,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,   188,   189,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,   863,   624,     0,     0,   864,   190,   191,   192,   193,
     194,   195,   238,     0,     0,     0,     0,   196,   197,   198,
     199,   200,   201,   202,   173,   174,   175,   176,   177,   178,
     179,   180,   181,     0,     0,   182,   183,     0,     0,     0,
       0,   184,   185,   186,   187,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,   188,   189,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,   866,   634,     0,     0,   867,   190,
     191,   192,   193,   194,   195,   238,     0,     0,     0,     0,
     196,   197,   198,   199,   200,   201,   202,   173,   174,   175,
     176,   177,   178,   179,   180,   181,     0,     0,   182,   183,
       0,     0,     0,     0,   184,   185,   186,   187,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,   188,
     189,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,   872,   624,     0,
       0,   873,   190,   191,   192,   193,   194,   195,   238,     0,
       0,     0,     0,   196,   197,   198,   199,   200,   201,   202,
     173,   174,   175,   176,   177,   178,   179,   180,   181,     0,
       0,   182,   183,     0,     0,     0,     0,   184,   185,   186,
     187,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,   188,   189,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
     668,   634,     0,     0,   669,   190,   191,   192,   193,   194,
     195,   238,     0,     0,     0,     0,   196,   197,   198,   199,
     200,   201,   202,   173,   174,   175,   176,   177,   178,   179,
     180,   181,     0,     0,   182,   183,     0,     0,     0,     0,
     184,   185,   186,   187,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,   188,   189,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,   958,   624,     0,     0,   959,   190,   191,
     192,   193,   194,   195,   238,     0,     0,     0,     0,   196,
     197,   198,   199,   200,   201,   202,   173,   174,   175,   176,
     177,   178,   179,   180,   181,     0,     0,   182,   183,     0,
       0,     0,     0,   184,   185,   186,   187,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,   188,   189,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,   961,   634,     0,     0,
     962,   190,   191,   192,   193,   194,   195,   238,     0,     0,
       0,     0,   196,   197,   198,   199,   200,   201,   202,   173,
     174,   175,   176,   177,   178,   179,   180,   181,     0,     0,
     182,   183,     0,     0,     0,     0,   184,   185,   186,   187,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,   188,   189,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,  1242,
     624,     0,     0,  1243,   190,   191,   192,   193,   194,   195,
     238,     0,     0,     0,     0,   196,   197,   198,   199,   200,
     201,   202,   173,   174,   175,   176,   177,   178,   179,   180,
     181,     0,     0,   182,   183,     0,     0,     0,     0,   184,
     185,   186,   187,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,   188,   189,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,  1245,   634,     0,     0,  1246,   190,   191,   192,
     193,   194,   195,   238,     0,     0,     0,     0,   196,   197,
     198,   199,   200,   201,   202,   173,   174,   175,   176,   177,
     178,   179,   180,   181,     0,     0,   182,   183,     0,     0,
       0,     0,   184,   185,   186,   187,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,   188,   189,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,  1252,   624,     0,     0,  1253,
     190,   191,   192,   193,   194,   195,   238,     0,     0,     0,
       0,   196,   197,   198,   199,   200,   201,   202,   173,   174,
     175,   176,   177,   178,   179,   180,   181,     0,     0,   182,
     183,     0,     0,     0,     0,   184,   185,   186,   187,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
     188,   189,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,   668,   634,
       0,     0,   669,   190,   191,   192,   193,   194,   195,   238,
       0,     0,     0,     0,   196,   197,   198,   199,   200,   201,
     202,   173,   174,   175,   176,   177,   178,   179,   180,   181,
       0,     0,   182,   183,     0,     0,     0,     0,   184,   185,
     186,   187,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,   188,   189,   348,   349,   350,   351,   352,
     353,   354,   355,   356,   357,   358,   359,   360,     0,     0,
       0,     0,   361,   362,     0,     0,   190,   191,   192,   193,
     194,   195,     0,     0,     0,     0,     0,   196,   197,   198,
     199,   200,   201,   202,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,   364,   365,
     366,   367,     0,   368,     0,     0,     0,     0,     0,   369,
     370,   371,   372,   373,     0,     0,     0,   374,   348,   349,
     350,   351,   352,   353,   354,   355,   356,   357,   358,  -776,
    -776,     0,     0,     0,     0,   361,   362,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,   364,   365,   366,   367,     0,   368,     0,     0,     0,
       0,     0,   369,   370,   371,   372,   373 ];

#[allow(non_upper_case_globals)]
const yycheck_: &'static [i32] = &[      2,    60,    71,    59,     8,    91,   385,    16,    17,   105,
       8,    96,     2,   611,     4,    22,    57,   556,   104,   324,
       4,   452,   238,   328,    28,   331,    53,    28,    97,    94,
      28,    22,    97,   380,   403,   746,    85,   346,   276,   858,
      68,   749,   280,    97,    85,   101,   232,   444,    16,    17,
     911,   538,   597,    55,    56,   452,   944,    98,    99,   100,
     491,    59,   944,    53,    54,   753,   611,    57,   324,   851,
      69,    57,   328,    80,    76,    77,   382,    77,   379,   328,
     381,   913,   501,   379,    25,   381,    26,    55,    56,    80,
      71,   431,   432,   102,   796,    85,    16,    17,    77,    29,
      13,   448,    66,   101,   100,    99,    16,    17,    98,    99,
     100,   101,    37,    38,    25,   264,   970,    13,    93,    94,
      27,   346,    97,  1138,     0,   103,   597,   428,     2,  1167,
       4,   746,   428,   987,   102,   129,    34,    66,   753,    99,
     611,   227,    99,    52,   140,    55,   447,    56,   449,    99,
      56,   447,   125,   449,    52,   119,   503,   216,   431,   432,
     302,   303,   140,    13,   140,   584,   561,   140,   563,   129,
     256,    25,   129,    13,   127,   476,   477,   450,   451,   129,
      54,   477,   102,    27,   125,    29,   126,   140,   792,   793,
     119,  1073,   102,   123,   343,   136,    13,   266,   126,   970,
     505,   502,    26,   504,   290,   478,   431,   432,   504,     2,
    1248,     4,   266,    13,   125,   140,   218,   219,   989,   219,
      34,  1236,   212,   136,   226,   138,   235,   140,   237,   238,
     232,   138,   126,   140,   597,   284,   238,   264,    52,   218,
     219,    28,   138,   284,   140,   247,   733,   734,   611,   505,
      13,   100,   317,   318,   319,   320,   505,   247,   109,   249,
      53,    54,   253,   254,    57,   696,   241,   235,   243,   237,
      53,   125,  1160,   139,   264,   125,   125,   463,   749,  1167,
     131,  1169,  1143,    13,   139,   266,   285,   287,   138,    13,
     140,   660,    85,    13,   284,   997,   998,   694,   138,   696,
     140,    68,   126,   344,   651,    98,    99,   100,   101,   395,
    1142,    25,   136,   341,   661,   235,   343,   237,   346,   136,
     389,   138,   387,   140,  1178,   235,   316,   237,   238,   944,
      97,   321,   316,     9,    25,   389,   139,   327,   138,    15,
     140,   331,   317,   318,   319,   320,  1057,   322,   323,   650,
      68,   652,    68,   343,   344,    97,   652,   663,   344,   660,
    1248,   662,  1250,   782,  1052,    25,   662,  1255,    25,  1257,
      99,   380,  1191,   247,   516,   138,   518,   140,   380,    97,
      13,    97,   753,   381,   923,   756,   128,  1275,   992,   993,
     381,    99,   382,    99,   136,   139,   386,    25,   388,    25,
     129,  1172,    99,   431,   432,    99,  1177,  1178,   138,  1191,
     140,   125,   387,   718,   138,   125,   140,   125,   138,   212,
     140,   129,  1020,   129,   911,   794,   913,   402,   125,   212,
     428,   341,   129,   796,   125,   129,   346,  1052,   282,   448,
     787,   746,   316,   129,   841,   136,   448,   321,   753,  1160,
     813,   449,   501,    68,   247,   764,   249,  1165,   449,   461,
     501,   463,   718,   129,    58,   125,   249,   471,   125,   718,
     471,   264,   710,   471,   597,  1020,   444,    25,   694,   477,
    1262,   264,    97,    77,   125,   786,   477,   788,   611,  1260,
     786,   284,   788,   483,   503,   835,   836,   125,   684,   125,
     569,   503,   842,   843,   975,   138,   504,   140,    15,   595,
      68,   501,    77,   504,   108,   109,   100,    58,    25,   990,
     488,    37,    38,   316,   919,   920,   997,   998,   321,   924,
      66,   926,  1071,   928,   327,   584,    77,   131,   331,    97,
    1079,   125,   544,   584,  1255,  1160,    66,    82,    83,  1020,
     343,   344,  1167,  1172,   556,   103,   112,   566,  1177,   784,
     343,   570,   835,   836,    54,   871,   568,   108,   653,   842,
     843,   112,    66,   944,    64,    65,   645,   125,   143,   144,
     145,   129,   724,   119,   136,   121,   122,   729,   136,   382,
      99,    25,   140,   386,   584,   388,    25,   132,   133,    25,
      56,   121,   122,   386,   967,   388,   838,   597,   125,  1040,
     835,   836,   885,   886,   846,   888,   889,   842,   843,   139,
     129,   611,   629,   140,   987,   994,   125,   121,   122,    66,
     632,   638,    99,  1248,   997,   998,   638,    68,   629,   944,
     129,   140,   651,  1040,   950,   139,  1025,   638,    15,   651,
      17,   641,   661,   643,   652,  1142,  1143,  1020,   138,   661,
     140,   652,   711,   670,   662,    96,    97,  1007,   670,   103,
     711,   662,   676,   663,   103,   676,   100,   103,   676,   670,
    1219,  1052,   684,  1054,   121,   122,  1065,   124,  1059,   664,
     483,   125,    99,    77,  1165,   129,   125,   128,   137,   125,
     129,   140,   704,   129,    99,   891,   140,    68,   501,    68,
      68,   140,   703,   704,   140,   136,    66,    67,   125,   140,
     125,   711,   129,  1029,   699,    52,   694,    54,    55,     2,
      57,     4,   975,   782,  1007,    96,    97,    96,    97,    97,
     799,   782,   136,    16,    17,  1124,   129,  1052,  1127,  1144,
    1145,  1146,  1147,   136,   997,   998,   784,   141,   142,   143,
     144,   145,   764,   765,    99,  1038,   136,   128,   125,   128,
     772,   121,   122,   775,   101,   136,   620,    77,   787,   103,
      53,    54,  1007,   879,    57,   787,    66,   631,    99,   779,
     788,   584,   782,   783,   129,    68,  1167,   788,  1169,    68,
     869,   129,   792,   793,    99,   870,   796,   835,   836,    68,
      99,   136,    85,   125,   842,   843,    68,   129,   129,   137,
      93,    94,   878,   813,    97,    98,    99,   100,    97,   102,
     125,  1202,    66,  1139,   129,   127,    68,    96,    97,   119,
     129,   121,   122,   687,    96,    97,  1241,   849,   641,   851,
     643,   136,   975,    52,   764,  1160,    26,    56,   641,    56,
     643,  1208,  1167,    68,    96,    97,    77,   990,   125,   128,
     663,   715,   944,   841,   997,   998,   128,  1248,   868,  1250,
     878,   871,    68,    14,    15,   119,  1257,   121,   122,   891,
     124,    96,    97,    68,   129,   870,   128,  1020,    68,    88,
      89,    68,   129,    99,  1275,   779,  1207,    68,  1209,   783,
      96,    97,    68,  1209,   129,    68,  1217,   125,   711,   984,
     138,   923,    97,   128,   106,    26,    96,    97,    66,   125,
      97,   933,    66,   129,    56,    96,    97,   286,   287,   212,
      96,    97,   128,  1248,    97,    99,    25,   937,   938,   133,
    1015,   129,   796,   797,   103,   945,   126,   129,   128,    66,
     950,  1030,   235,    66,   237,   238,   136,    68,   241,   127,
     243,   125,   128,    10,   247,   129,   249,   967,   125,  1007,
     970,   119,   687,   121,   122,   119,   779,   121,   122,   782,
     783,   264,    26,   125,   987,    96,    97,   987,   988,   989,
      40,    41,   992,   993,   997,   998,   127,   997,   998,   984,
     715,   284,   119,   125,   121,   122,   119,   124,   121,   122,
     125,   865,    52,  1064,  1014,   126,   125,   128,   125,   129,
    1020,   875,    52,   125,    68,   136,  1077,  1096,   125,  1029,
    1015,   125,     8,   316,   317,   318,   319,   320,   321,   322,
     323,    68,   896,   125,   327,   125,    44,   125,   331,  1081,
    1082,    44,    96,    97,  1150,   129,  1068,   597,   341,  1071,
     343,   344,   125,   346,  1064,   868,    68,  1079,   871,    96,
      97,   611,   597,    13,    25,   868,    17,  1077,  1160,   241,
     139,   796,   126,  1165,   128,  1167,   611,  1169,   139,   125,
     944,   125,   136,    68,    96,    97,    68,   380,   129,   382,
      44,   128,    68,   386,   387,   388,   960,   269,  1159,    68,
      44,   273,  1187,  1188,   127,   969,   130,   125,   972,   402,
      52,    96,    97,   130,    96,    97,   128,   150,   125,    15,
      96,    97,  1132,   125,   937,   938,   137,    96,    97,  1139,
    1140,   995,   945,   125,   937,   938,   125,   950,   431,   432,
     865,   125,   945,   128,  1154,    52,   128,   125,   125,  1159,
     875,   100,   128,   127,   125,   448,  1248,   125,  1250,   128,
     127,    52,  1172,  1255,   100,  1257,   103,  1177,  1178,  1191,
     125,     9,  1214,  1215,    56,  1185,   138,    99,  1220,  1208,
    1222,  1223,   125,  1275,    54,    55,  1208,    57,  1210,  1199,
     483,  1209,  1187,  1188,    64,    65,   740,  1219,  1209,   130,
     125,  1014,   125,   125,   125,   749,  1228,   129,   501,  1073,
     503,  1014,    59,    60,    61,    62,  1029,   125,    52,  1229,
      54,    55,    56,    57,   125,  1267,  1268,  1269,  1270,   103,
     125,    77,   125,  1097,    56,   960,   125,  1279,   125,   125,
    1262,    56,   792,   793,   969,   130,   796,   972,    94,    95,
    1260,  1064,   127,  1117,  1118,  1119,   125,   792,   793,   125,
     125,   796,   125,   813,  1077,   125,   120,   101,   136,     2,
     995,     4,   136,   566,   125,   247,   479,   570,   813,   878,
      98,   100,   454,    16,    17,   131,   483,   459,    89,  1228,
     462,   584,   765,   465,   670,   141,   142,   143,   144,   145,
      52,   733,    54,    55,    56,    57,    58,    52,   480,    54,
      55,    56,    57,   485,    40,    41,    42,    43,    44,  1132,
      53,    54,   911,  1075,   944,    77,  1139,  1140,   210,  1132,
    1194,   213,   214,   215,  1236,    68,   775,  1140,   880,    91,
      52,  1154,    54,    55,    56,    57,  1159,   773,   641,   101,
     643,  1154,   576,  1262,   988,   107,   108,   109,   651,   990,
      93,    94,   990,   986,    97,  1210,  1068,   108,   661,   102,
     663,   664,  1185,   518,   101,   547,   749,  1165,  1160,   131,
    1244,   746,  1185,    -1,   136,  1097,  1199,    -1,    -1,   101,
      -1,   143,  1117,  1118,  1119,   107,  1199,    -1,    -1,    52,
     944,    54,    55,    56,    57,    -1,   699,    -1,    -1,    -1,
      -1,   583,    -1,    -1,    -1,    -1,  1229,   967,   711,    -1,
     970,    -1,    -1,    -1,    -1,    -1,  1229,    -1,    -1,    -1,
      -1,    -1,   967,    -1,    -1,   970,    -1,   987,   988,   989,
      -1,    -1,   992,   993,    -1,    -1,    -1,   997,   998,    -1,
      -1,    -1,   987,   988,   989,    -1,    -1,   992,   993,   341,
      -1,    -1,   997,   998,   346,    -1,    -1,    -1,   597,    -1,
    1020,    -1,    52,    -1,    54,    55,    56,    57,    58,   212,
      -1,    -1,   611,    -1,   597,  1020,   779,    -1,    -1,   782,
     783,   784,    -1,    -1,   787,    -1,    -1,    77,   611,    -1,
      -1,    -1,   235,    -1,   237,   238,    -1,    -1,   241,    -1,
     243,    91,    -1,    -1,   247,    -1,   249,    -1,    -1,  1244,
      -1,   101,    -1,    -1,    -1,    -1,    -1,   107,   108,   109,
      -1,   264,    -1,    -1,    -1,    77,    -1,    -1,    -1,    -1,
      -1,    -1,   835,   836,    -1,    -1,    -1,    -1,    -1,   842,
     843,   131,    94,    95,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,   443,   444,   143,    -1,   737,    -1,    -1,    -1,    -1,
     452,    -1,    -1,    -1,    -1,   868,    -1,   870,   871,    -1,
      -1,    -1,    -1,   316,   317,   318,   319,   320,   321,   322,
     323,   597,    -1,    -1,   327,    -1,    -1,    -1,   331,   141,
     142,   143,   144,   145,    -1,   611,   488,    -1,   341,   491,
     343,    -1,    -1,   346,    -1,    -1,  1160,    -1,    -1,    -1,
      -1,  1165,  1172,  1167,    -1,  1169,    -1,  1177,  1178,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,  1172,    -1,    -1,
      -1,    -1,  1177,  1178,   937,   938,    -1,   380,    -1,   382,
      -1,    -1,   945,   386,   387,   388,    -1,   950,    -1,    -1,
      -1,    -1,    -1,   792,   793,    -1,    -1,   796,    -1,   402,
      -1,    -1,   554,    -1,    -1,    -1,    -1,    -1,    -1,   792,
     793,    -1,    -1,   796,   813,    -1,    -1,    -1,     2,    -1,
       4,   984,    -1,    -1,    -1,   577,    -1,    -1,   431,   432,
     813,    -1,    16,    17,  1248,   877,  1250,    -1,    -1,    -1,
    1260,  1255,    -1,  1257,  1007,   448,    -1,    -1,   890,    -1,
     892,  1014,  1015,    -1,    -1,  1260,    -1,    -1,    -1,    -1,
      -1,  1275,    -1,    -1,    -1,   571,  1029,   909,    -1,    53,
      54,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
     483,    77,    -1,   597,    68,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,   611,    94,    95,
     503,  1064,    52,    -1,    54,    55,    56,    57,    58,    93,
      94,    -1,    -1,    97,  1077,    -1,   792,   793,   102,    -1,
     796,    -1,    -1,    -1,    -1,   677,    -1,    77,    -1,   681,
      -1,    -1,    -1,   639,   130,   131,    -1,   813,   644,   135,
     646,    91,   694,    -1,   696,   141,   142,   143,   144,   145,
      -1,   101,    -1,    -1,   996,    -1,    -1,    -1,   108,   109,
      -1,    -1,    -1,   566,    -1,    -1,    -1,   570,   967,  1132,
      -1,   970,    -1,    -1,    -1,    -1,  1139,  1140,    -1,    -1,
      -1,   131,    -1,    -1,   967,    -1,    -1,   970,   987,   988,
     989,  1154,   744,   992,   993,    -1,  1159,    -1,   997,   998,
      -1,    -1,    -1,    -1,   987,   988,   989,    -1,    -1,   992,
     993,    -1,   764,    -1,   997,   998,    -1,    -1,    -1,    -1,
      -1,  1020,  1185,    -1,  1187,  1188,   778,    52,   212,    54,
      55,    56,    57,    58,    -1,    -1,  1199,  1020,   641,    -1,
     643,    -1,    -1,    -1,    -1,  1208,    -1,  1089,   651,    -1,
      -1,   235,    77,   237,   238,    -1,    -1,   241,   661,   243,
     663,   664,    -1,   247,    -1,   249,  1229,    -1,   774,    -1,
     776,    -1,  1114,  1115,  1116,    -1,   101,    -1,   792,   793,
     264,    -1,   796,   108,   109,    -1,    -1,    -1,    -1,   841,
      -1,   967,    -1,    -1,   970,    -1,   699,    -1,    -1,   813,
      -1,    -1,    -1,    -1,    -1,    -1,   131,    -1,    -1,   861,
      -1,   987,   988,   989,    -1,    -1,   992,   993,    -1,    -1,
      -1,   997,   998,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,   316,   317,   318,   319,   320,   321,   322,   323,
      -1,    -1,    -1,   327,  1020,    -1,    -1,   331,    -1,    -1,
      -1,    -1,    -1,   859,    -1,    -1,   862,   341,   597,   343,
      -1,    -1,   346,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
     876,   597,   611,  1172,    -1,    -1,   779,    -1,  1177,  1178,
     783,   784,    -1,    -1,   787,   611,    -1,    -1,    -1,  1172,
      -1,    -1,    -1,    -1,  1177,  1178,   380,    -1,   382,    -1,
     952,    -1,   386,   387,   388,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,   402,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,   835,   836,    -1,    -1,    -1,    -1,    -1,   842,
     843,    -1,    -1,    -1,    -1,    -1,    -1,   431,   432,    -1,
      -1,    -1,    -1,   967,    -1,    -1,   970,    -1,    -1,    -1,
      -1,  1260,    -1,    -1,   448,   868,    -1,   870,   871,    -1,
      -1,  1023,    -1,   987,   988,   989,    -1,  1260,   992,   993,
      -1,    -1,    -1,   997,   998,    -1,    -1,    -1,  1040,    -1,
      -1,    -1,    -1,     2,    -1,     4,  1172,    -1,    -1,   483,
      -1,  1177,  1178,    -1,    -1,    -1,  1020,    -1,    -1,    -1,
    1016,    -1,    -1,  1019,    -1,  1021,    -1,    -1,    -1,   503,
      -1,    -1,  1028,    -1,    -1,  1031,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,   937,   938,    -1,    -1,    -1,    -1,
      -1,    -1,   945,    -1,    53,    54,    -1,   950,    57,    -1,
      -1,    -1,    -1,   792,   793,    -1,    -1,   796,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,   792,   793,    -1,    -1,
     796,    -1,    -1,    -1,   813,    -1,    85,    -1,    -1,  1131,
      -1,   984,   566,    -1,  1260,    -1,   570,   813,    -1,    98,
      99,   100,    52,    -1,    54,    55,    56,    57,    58,     2,
      -1,     4,    -1,    -1,  1007,    -1,    -1,    -1,    -1,    -1,
      -1,  1014,  1015,    -1,    -1,    -1,    -1,    77,    -1,    52,
      -1,    54,    55,    56,    57,    58,  1029,    -1,  1134,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,  1189,    -1,    -1,
      -1,   101,    -1,    -1,    77,    -1,    -1,   107,   108,   109,
      53,    54,    -1,    -1,    57,    -1,    -1,   641,  1172,   643,
      -1,    -1,    -1,  1177,  1178,    -1,    -1,   651,   101,    -1,
      -1,   131,    -1,    -1,   107,   108,   109,   661,    -1,   663,
     664,    -1,    85,   143,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,  1197,    -1,    -1,    -1,    98,    99,   100,   131,    -1,
      -1,    -1,    -1,   212,   597,    -1,    -1,   140,    -1,    -1,
     143,    -1,    -1,    -1,    -1,   699,    -1,    -1,   611,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,   967,  1132,
      -1,   970,    -1,    -1,    -1,    -1,  1139,  1140,   247,    -1,
     249,   967,    -1,    -1,   970,    -1,  1260,    -1,   987,   988,
     989,  1154,    -1,   992,   993,   264,    -1,    -1,   997,   998,
      -1,   987,   988,   989,    -1,    -1,   992,   993,    -1,    -1,
      -1,   997,   998,    -1,    -1,   284,    -1,    -1,    -1,    -1,
      -1,  1020,  1185,    -1,  1187,  1188,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,  1020,   779,  1199,    -1,    -1,   783,
     784,    -1,    -1,   787,    -1,  1208,    -1,   316,    -1,   212,
      -1,    -1,   321,    33,    34,    35,    36,    -1,   327,    -1,
      -1,    -1,   331,    -1,    -1,    -1,  1229,    -1,    -1,    49,
      50,    51,    -1,    -1,   343,   344,    -1,   346,    -1,    59,
      60,    61,    62,    63,   247,    -1,   249,    -1,    -1,    -1,
      -1,   835,   836,    -1,    -1,    -1,    -1,    -1,   842,   843,
      -1,   264,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,   382,    -1,    -1,    -1,   386,    -1,   388,
      -1,   284,    -1,    -1,   868,    -1,   870,   871,    -1,    -1,
     110,   111,   112,   113,   114,   115,   116,   117,   118,   792,
     793,    -1,    -1,   796,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,     2,   316,     4,    -1,    -1,    -1,   321,    -1,
     813,    -1,   431,   432,   327,    -1,    -1,    -1,   331,    -1,
      -1,   151,    -1,  1172,    -1,    -1,    -1,    -1,  1177,  1178,
     343,   344,    -1,   346,    -1,    -1,  1172,    -1,    -1,    -1,
      -1,  1177,  1178,   937,   938,    -1,    -1,    -1,    -1,    -1,
      -1,   945,    -1,    53,    54,    -1,   950,    57,    -1,    -1,
      -1,    -1,    -1,    -1,   483,    -1,    -1,    -1,    -1,   382,
      -1,    -1,    -1,   386,    -1,   388,    -1,    -1,    -1,    -1,
      -1,    -1,   501,    -1,    -1,    85,    -1,    -1,    -1,    -1,
     984,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    98,    99,
     100,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,  1260,    -1,  1007,    -1,    -1,    -1,    -1,   431,   432,
    1014,  1015,    -1,    -1,  1260,    -1,    -1,    -1,    -1,    -1,
      -1,     2,    -1,     4,    -1,  1029,     0,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,     8,     9,    10,    -1,    -1,    13,
      14,    15,    -1,    17,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    25,    26,    27,   967,   584,    -1,   970,    -1,    -1,
     483,    -1,    -1,    37,    38,    -1,    40,    41,    42,    43,
      44,    -1,    53,    54,   987,   988,   989,    -1,   501,   992,
     993,    -1,    -1,    -1,   997,   998,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    68,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,   212,    -1,    -1,    -1,    -1,  1020,    -1,    -1,
      -1,    -1,   641,    -1,   643,    -1,    -1,    98,    -1,    -1,
      -1,    -1,    96,    97,    -1,    -1,    -1,    -1,  1132,   103,
      -1,    -1,    -1,    -1,   663,  1139,  1140,   247,    -1,   249,
      -1,    -1,    -1,    -1,    -1,    -1,   120,    -1,    -1,    -1,
    1154,    -1,   126,   127,   264,    -1,    -1,    -1,    -1,    -1,
      -1,   584,   136,    -1,   138,    -1,   140,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,   284,    -1,    -1,    -1,    -1,    -1,
      -1,  1185,   711,  1187,  1188,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,   722,    -1,  1199,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,  1208,    -1,   316,    -1,    -1,    -1,
      -1,   321,    -1,    -1,    -1,    -1,    -1,   327,   641,    -1,
     643,   331,    -1,    -1,    -1,  1229,    -1,    -1,    -1,    -1,
      -1,   212,    -1,   343,   344,    -1,   346,    -1,    -1,    -1,
     663,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
     779,    -1,    -1,   782,   783,   784,    -1,    -1,    -1,  1172,
      -1,    -1,    -1,    -1,  1177,  1178,   247,    -1,   249,    -1,
      -1,    -1,   382,    -1,    -1,    -1,   386,    -1,   388,    -1,
      -1,    -1,    -1,   264,    -1,    -1,    -1,    -1,   711,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,   835,   836,    -1,    -1,
      -1,    -1,    -1,   842,   843,    -1,    -1,    -1,    -1,    -1,
      -1,   431,   432,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,     2,   316,     4,    -1,    -1,   868,
     321,    -1,   871,    -1,    -1,    -1,   327,  1260,    -1,    -1,
     331,    -1,    -1,    -1,    -1,    -1,   779,    -1,    -1,   782,
     783,   784,   343,    -1,    -1,   346,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,   483,    25,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    53,    54,    -1,    -1,    -1,
      -1,   501,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,   382,    -1,    -1,    -1,   386,    -1,   388,   937,   938,
      -1,    -1,   835,   836,    -1,    -1,   945,    -1,    -1,   842,
     843,   950,    -1,    -1,    -1,    -1,    77,    78,    79,    80,
      81,    82,    83,    84,    85,    86,    87,    88,    89,    -1,
      -1,    -1,    -1,    94,    95,   868,    -1,    -1,   871,   100,
     431,   432,    -1,    77,    78,    79,    80,    81,    82,    83,
      84,    85,    86,    87,    88,    89,    -1,    -1,    -1,    -1,
      94,    95,    -1,    -1,   584,    -1,    -1,    -1,  1007,   130,
     131,   132,   133,    -1,   135,  1014,    -1,    -1,    -1,    -1,
     141,   142,   143,   144,   145,    -1,    -1,    -1,   149,    -1,
    1029,    -1,   483,    -1,    -1,    -1,   130,   131,   132,   133,
      -1,   135,    -1,    -1,   937,   938,   140,   141,   142,   143,
     144,   145,   945,    -1,    -1,   149,    -1,   950,    -1,    -1,
      -1,   641,    -1,   643,    -1,  1064,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,   212,    -1,    -1,    -1,  1077,    -1,
      -1,    -1,    -1,   663,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    77,    78,
      79,    80,    81,    82,    83,    84,    -1,    86,    87,   247,
      -1,   249,    -1,    -1,  1007,    94,    95,    -1,    -1,    -1,
      -1,  1014,    -1,    -1,    -1,    -1,   264,    -1,    -1,    -1,
      -1,   711,    -1,  1132,    -1,    -1,  1029,    -1,    -1,    -1,
    1139,  1140,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,   130,   131,   132,   133,  1154,   135,    -1,    -1,    -1,
    1159,    -1,   141,   142,   143,   144,   145,    -1,    -1,    -1,
      -1,  1064,    -1,    -1,    -1,    -1,    -1,    -1,   316,    -1,
      -1,    44,    -1,   321,  1077,    -1,  1185,    -1,    -1,   327,
     641,    -1,   643,   331,    -1,    -1,    -1,    -1,    -1,   779,
    1199,    -1,   782,   783,   784,   343,    -1,    -1,   346,    -1,
      -1,    -1,   663,    -1,    77,    78,    79,    80,    81,    82,
      83,    84,    85,    86,    87,    88,    89,    -1,    -1,    -1,
    1229,    94,    95,    -1,    -1,    -1,    -1,    -1,    -1,  1132,
      -1,    -1,    -1,    -1,   382,    -1,  1139,  1140,   386,    -1,
     388,    -1,    -1,    -1,    -1,   835,   836,    -1,    -1,    -1,
      -1,  1154,   842,   843,    -1,    -1,  1159,   130,   131,   132,
     133,    -1,   135,    -1,    -1,    -1,    -1,    -1,   141,   142,
     143,   144,   145,    -1,    -1,    -1,   149,    -1,   868,    -1,
      -1,   871,  1185,   431,   432,    33,    34,    35,    36,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,  1199,    -1,    -1,    -1,
      -1,    49,    50,    51,    52,    -1,    -1,    -1,    56,    -1,
      -1,    59,    60,    61,    62,    63,    -1,    -1,   779,    -1,
      -1,    -1,   783,   784,    -1,    -1,  1229,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,   483,    -1,    -1,    -1,    -1,
      -1,    -1,    90,    91,    -1,    -1,    -1,   937,   938,    -1,
      98,    -1,    -1,   101,    -1,   945,   104,   105,    -1,   107,
     950,    -1,   110,   111,   112,   113,   114,   115,   116,   117,
     118,    -1,    -1,    -1,   835,   836,    -1,    -1,    -1,    -1,
      -1,   842,   843,    -1,    -1,    -1,    -1,   135,    -1,    -1,
      -1,    -1,   140,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,   151,    -1,    -1,    -1,   868,    -1,    -1,
     871,    -1,    -1,    -1,    -1,    -1,    -1,  1007,    -1,    -1,
      -1,    16,    17,    -1,  1014,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,  1029,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    48,    49,    50,    51,    -1,    -1,    -1,
      55,    56,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    67,    68,  1064,    -1,   937,   938,    -1,    -1,
      -1,    -1,    -1,    -1,   945,    -1,    -1,  1077,    -1,   950,
      -1,    -1,    -1,   641,    -1,   643,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,   102,    33,    34,
      35,    36,    -1,    -1,    -1,   663,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    49,    50,    51,    52,    -1,    -1,
      -1,    56,    -1,    58,    59,    60,    61,    62,    63,    -1,
      -1,    -1,  1132,    -1,    -1,    -1,  1007,    -1,    -1,  1139,
    1140,    -1,    77,  1014,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,  1154,    90,    91,    -1,  1029,  1159,
      -1,    -1,    -1,    98,    -1,    -1,   101,    -1,    -1,   104,
     105,    -1,   107,   108,    -1,   110,   111,   112,   113,   114,
     115,   116,   117,   118,    -1,  1185,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,  1064,    -1,    -1,    -1,    -1,    -1,  1199,
     135,    44,    -1,    -1,    -1,   210,    -1,    -1,   213,   214,
     215,    -1,   217,    -1,    -1,    -1,   151,    -1,    -1,    -1,
      -1,   779,    -1,    -1,    -1,   783,   784,    -1,    -1,  1229,
     235,    -1,   237,   238,    77,    78,    79,    80,    81,    82,
      83,    84,    85,    86,    87,    88,    89,    -1,    -1,    -1,
      -1,    94,    95,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,  1132,    -1,    -1,    -1,    -1,    -1,    -1,  1139,  1140,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,   835,   836,    -1,
      -1,    -1,   125,  1154,   842,   843,    -1,   130,   131,   132,
     133,    -1,   135,    -1,    -1,    -1,    -1,    -1,   141,   142,
     143,   144,   145,    -1,    -1,    -1,   149,    -1,    -1,    -1,
     868,    -1,    -1,   871,  1185,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,  1199,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,   341,    -1,    -1,    -1,
      -1,   346,    -1,   348,   349,   350,   351,   352,    -1,    -1,
     355,   356,   357,   358,   359,   360,   361,   362,  1229,   364,
     365,    -1,    -1,   368,   369,   370,   371,   372,   373,   374,
     375,   376,    -1,    -1,    -1,   380,    -1,    -1,    -1,   937,
     938,    -1,    -1,    -1,    -1,    -1,    -1,   945,    -1,    -1,
      -1,    -1,   950,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,   431,   432,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,   442,   443,   444,
      -1,    -1,    -1,   448,    -1,   450,   451,   452,    -1,  1007,
      -1,    -1,    -1,   458,    -1,    -1,  1014,    -1,    -1,    -1,
      -1,    33,    34,    35,    36,    -1,    -1,    -1,   473,    -1,
      -1,  1029,    -1,   478,    -1,    -1,    -1,    49,    50,    51,
      52,    -1,    -1,   488,    56,    -1,   491,    59,    60,    61,
      62,    63,    -1,    -1,    -1,    -1,    -1,    -1,   503,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,   520,    -1,    -1,    90,    91,
      -1,    -1,    -1,    -1,    -1,    -1,    98,    -1,    -1,   101,
      -1,    -1,   104,   105,    -1,   107,    -1,    -1,   110,   111,
     112,   113,   114,   115,   116,   117,   118,    -1,    -1,   554,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,   566,    -1,   135,    44,   570,    -1,    -1,    -1,    -1,
      -1,    -1,   577,    -1,  1132,    -1,    -1,    -1,    -1,   151,
      -1,  1139,  1140,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,  1154,    77,    78,    79,
      80,    81,    82,    83,    84,    85,    86,    87,    88,    89,
      -1,    -1,    -1,    -1,    94,    95,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,  1185,    -1,    -1,
      77,    78,    79,    80,    81,    82,    83,    -1,    -1,    86,
      87,  1199,    -1,    -1,    -1,    -1,   651,    94,    95,    -1,
     130,   131,   132,   133,    -1,   135,   661,    -1,    -1,    -1,
      -1,   141,   142,   143,   144,   145,    -1,    -1,    -1,   149,
      -1,  1229,   677,   678,   679,    -1,   681,    -1,    -1,    -1,
      -1,    -1,    -1,   130,   131,   132,   133,    -1,   135,   694,
      -1,   696,    -1,    -1,   141,   142,   143,   144,   145,    -1,
      -1,     0,     1,    -1,     3,     4,     5,     6,     7,    -1,
      -1,    -1,    11,    12,    -1,    -1,    -1,    16,    -1,    18,
      19,    20,    21,    22,    23,    24,    -1,    -1,    -1,    -1,
      -1,    30,    31,    32,    33,    34,    35,    36,    -1,   744,
      39,    -1,    -1,    -1,    -1,    -1,    45,    46,    47,    48,
      49,    50,    51,    52,    53,    54,    55,    56,    57,   764,
      59,    60,    61,    62,    63,    64,    65,    -1,    -1,    -1,
      -1,    -1,    -1,   778,    -1,    -1,    75,    76,    -1,   784,
     785,    -1,   787,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    90,    91,    -1,    -1,    -1,    -1,    -1,    -1,    98,
      -1,    -1,   101,   102,    -1,   104,   105,    -1,   107,    -1,
      -1,   110,   111,   112,   113,   114,   115,   116,   117,   118,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
     835,   836,    -1,    -1,    -1,    -1,   841,   842,   843,   138,
      -1,   140,    -1,    -1,    -1,    -1,    -1,   146,   147,    -1,
      -1,    -1,   151,    -1,    -1,    -1,   861,    -1,     3,     4,
       5,    -1,     7,    -1,    -1,    -1,    11,    12,    -1,    -1,
      -1,    16,    -1,    18,    19,    20,    21,    22,    23,    24,
     885,   886,    -1,   888,   889,    30,    31,    32,    33,    34,
      35,    36,    -1,    -1,    39,    -1,    -1,    -1,    -1,    -1,
      -1,    46,    -1,    -1,    49,    50,    51,    52,    53,    54,
      55,    56,    57,    -1,    59,    60,    61,    62,    63,    64,
      65,    -1,    -1,    -1,   929,   930,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,   941,   942,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,   952,    -1,    -1,
      -1,    -1,    -1,    98,    -1,    -1,   101,   102,    -1,   104,
     105,    -1,    -1,    -1,    -1,   110,   111,   112,   113,   114,
     115,   116,   117,   118,    -1,    -1,    -1,    -1,    -1,    -1,
     125,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,  1000,    -1,    -1,    -1,    -1,
      -1,    -1,  1007,    -1,    -1,    -1,   151,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,  1023,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,  1038,    -1,  1040,    -1,    -1,    -1,    -1,
       1,    -1,     3,     4,     5,     6,     7,     8,     9,    10,
      11,    12,    -1,    -1,    15,    16,    -1,    18,    19,    20,
      21,    22,    23,    24,    -1,    -1,    -1,    -1,    -1,    30,
      31,    32,    33,    34,    35,    36,    -1,    -1,    39,    -1,
      -1,    -1,    -1,    -1,    45,    46,    47,    48,    49,    50,
      51,    52,    53,    54,    55,    56,    57,    -1,    59,    60,
      61,    62,    63,    64,    65,    -1,    -1,    77,    78,    79,
      80,    81,    82,    83,    75,    76,    86,    87,    -1,    -1,
      -1,    -1,    -1,    -1,    94,    95,  1131,    -1,    -1,    90,
      91,    -1,    -1,    -1,    -1,    -1,    -1,    98,    -1,    -1,
     101,   102,    -1,   104,   105,    -1,   107,    -1,    -1,   110,
     111,   112,   113,   114,   115,   116,   117,   118,    -1,    -1,
     130,   131,   132,   133,    -1,   135,    -1,    -1,    -1,    -1,
      -1,   141,   142,   143,   144,   145,    -1,   138,    -1,   140,
      -1,    -1,    -1,    -1,  1189,   146,   147,    -1,    -1,    -1,
     151,     1,    -1,     3,     4,     5,     6,     7,    -1,    -1,
      10,    11,    12,  1208,    14,    15,    16,    -1,    18,    19,
      20,    21,    22,    23,    24,    -1,    -1,    -1,    -1,    -1,
      30,    31,    32,    33,    34,    35,    36,    -1,    -1,    39,
      -1,    -1,    -1,    -1,    -1,    45,    46,    47,    48,    49,
      50,    51,    52,    53,    54,    55,    56,    57,    -1,    59,
      60,    61,    62,    63,    64,    65,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    75,    76,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      90,    91,    -1,    -1,    -1,    -1,    -1,    -1,    98,    -1,
      -1,   101,   102,    -1,   104,   105,    -1,   107,    -1,    -1,
     110,   111,   112,   113,   114,   115,   116,   117,   118,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,   138,     1,
     140,     3,     4,     5,     6,     7,   146,   147,    10,    11,
      12,   151,    -1,    15,    16,    17,    18,    19,    20,    21,
      22,    23,    24,    -1,    -1,    -1,    -1,    -1,    30,    31,
      32,    33,    34,    35,    36,    -1,    -1,    39,    -1,    -1,
      -1,    -1,    -1,    45,    46,    47,    48,    49,    50,    51,
      52,    53,    54,    55,    56,    57,    -1,    59,    60,    61,
      62,    63,    64,    65,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    75,    76,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    90,    91,
      -1,    -1,    -1,    -1,    -1,    -1,    98,    -1,    -1,   101,
     102,    -1,   104,   105,    -1,   107,    -1,    -1,   110,   111,
     112,   113,   114,   115,   116,   117,   118,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,   138,     1,   140,     3,
       4,     5,     6,     7,   146,   147,    10,    11,    12,   151,
      -1,    15,    16,    -1,    18,    19,    20,    21,    22,    23,
      24,    25,    -1,    -1,    -1,    -1,    30,    31,    32,    33,
      34,    35,    36,    -1,    -1,    39,    -1,    -1,    -1,    -1,
      -1,    45,    46,    47,    48,    49,    50,    51,    52,    53,
      54,    55,    56,    57,    -1,    59,    60,    61,    62,    63,
      64,    65,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    75,    76,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    90,    91,    -1,    -1,
      -1,    -1,    -1,    -1,    98,    -1,    -1,   101,   102,    -1,
     104,   105,    -1,   107,    -1,    -1,   110,   111,   112,   113,
     114,   115,   116,   117,   118,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,   138,     1,   140,     3,     4,     5,
       6,     7,   146,   147,    10,    11,    12,   151,    -1,    15,
      16,    -1,    18,    19,    20,    21,    22,    23,    24,    -1,
      -1,    -1,    -1,    -1,    30,    31,    32,    33,    34,    35,
      36,    -1,    -1,    39,    -1,    -1,    -1,    -1,    -1,    45,
      46,    47,    48,    49,    50,    51,    52,    53,    54,    55,
      56,    57,    -1,    59,    60,    61,    62,    63,    64,    65,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    75,
      76,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    90,    91,    -1,    -1,    -1,    -1,
      -1,    -1,    98,    -1,    -1,   101,   102,    -1,   104,   105,
      -1,   107,    -1,    -1,   110,   111,   112,   113,   114,   115,
     116,   117,   118,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,   138,    -1,   140,    -1,    -1,    -1,    -1,    -1,
     146,   147,    -1,    -1,     1,   151,     3,     4,     5,     6,
       7,    -1,     9,    10,    11,    12,    -1,    -1,    -1,    16,
      -1,    18,    19,    20,    21,    22,    23,    24,    -1,    -1,
      -1,    -1,    -1,    30,    31,    32,    33,    34,    35,    36,
      -1,    -1,    39,    -1,    -1,    -1,    -1,    -1,    45,    46,
      47,    48,    49,    50,    51,    52,    53,    54,    55,    56,
      57,    -1,    59,    60,    61,    62,    63,    64,    65,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    75,    76,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    90,    91,    -1,    -1,    -1,    -1,    -1,
      -1,    98,    -1,    -1,   101,   102,    -1,   104,   105,    -1,
     107,    -1,    -1,   110,   111,   112,   113,   114,   115,   116,
     117,   118,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,   138,     1,   140,     3,     4,     5,     6,     7,   146,
     147,    -1,    11,    12,   151,    -1,    -1,    16,    -1,    18,
      19,    20,    21,    22,    23,    24,    -1,    -1,    -1,    -1,
      -1,    30,    31,    32,    33,    34,    35,    36,    -1,    -1,
      39,    -1,    -1,    -1,    -1,    -1,    45,    46,    47,    48,
      49,    50,    51,    52,    53,    54,    55,    56,    57,    -1,
      59,    60,    61,    62,    63,    64,    65,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    75,    76,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    90,    91,    -1,    -1,    -1,    -1,    -1,    -1,    98,
      -1,    -1,   101,   102,   103,   104,   105,    -1,   107,    -1,
      -1,   110,   111,   112,   113,   114,   115,   116,   117,   118,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,   138,
       1,   140,     3,     4,     5,     6,     7,   146,   147,    -1,
      11,    12,   151,    -1,    -1,    16,    -1,    18,    19,    20,
      21,    22,    23,    24,    -1,    -1,    -1,    -1,    -1,    30,
      31,    32,    33,    34,    35,    36,    -1,    -1,    39,    -1,
      -1,    -1,    -1,    -1,    45,    46,    47,    48,    49,    50,
      51,    52,    53,    54,    55,    56,    57,    -1,    59,    60,
      61,    62,    63,    64,    65,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    75,    76,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    90,
      91,    -1,    -1,    -1,    -1,    -1,    -1,    98,    -1,    -1,
     101,   102,   103,   104,   105,    -1,   107,    -1,    -1,   110,
     111,   112,   113,   114,   115,   116,   117,   118,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,   138,     1,   140,
       3,     4,     5,     6,     7,   146,   147,    -1,    11,    12,
     151,    -1,    -1,    16,    -1,    18,    19,    20,    21,    22,
      23,    24,    -1,    -1,    -1,    -1,    -1,    30,    31,    32,
      33,    34,    35,    36,    -1,    -1,    39,    -1,    -1,    -1,
      -1,    -1,    45,    46,    47,    48,    49,    50,    51,    52,
      53,    54,    55,    56,    57,    -1,    59,    60,    61,    62,
      63,    64,    65,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    75,    76,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    90,    91,    -1,
      -1,    -1,    -1,    -1,    -1,    98,    -1,    -1,   101,   102,
      -1,   104,   105,    -1,   107,    -1,    -1,   110,   111,   112,
     113,   114,   115,   116,   117,   118,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,   127,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,   138,     1,   140,     3,     4,
       5,     6,     7,   146,   147,    -1,    11,    12,   151,    -1,
      -1,    16,    -1,    18,    19,    20,    21,    22,    23,    24,
      -1,    -1,    -1,    -1,    -1,    30,    31,    32,    33,    34,
      35,    36,    -1,    -1,    39,    -1,    -1,    -1,    -1,    -1,
      45,    46,    47,    48,    49,    50,    51,    52,    53,    54,
      55,    56,    57,    -1,    59,    60,    61,    62,    63,    64,
      65,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      75,    76,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    90,    91,    -1,    -1,    -1,
      -1,    -1,    -1,    98,    -1,    -1,   101,   102,    -1,   104,
     105,    -1,   107,    -1,    -1,   110,   111,   112,   113,   114,
     115,   116,   117,   118,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,   127,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,   138,     1,   140,     3,     4,     5,     6,
       7,   146,   147,    10,    11,    12,   151,    -1,    -1,    16,
      -1,    18,    19,    20,    21,    22,    23,    24,    -1,    -1,
      -1,    -1,    -1,    30,    31,    32,    33,    34,    35,    36,
      -1,    -1,    39,    -1,    -1,    -1,    -1,    -1,    45,    46,
      47,    48,    49,    50,    51,    52,    53,    54,    55,    56,
      57,    -1,    59,    60,    61,    62,    63,    64,    65,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    75,    76,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    90,    91,    -1,    -1,    -1,    -1,    -1,
      -1,    98,    -1,    -1,   101,   102,    -1,   104,   105,    -1,
     107,    -1,    -1,   110,   111,   112,   113,   114,   115,   116,
     117,   118,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,   138,     1,   140,     3,     4,     5,     6,     7,   146,
     147,    -1,    11,    12,   151,    -1,    -1,    16,    -1,    18,
      19,    20,    21,    22,    23,    24,    -1,    -1,    -1,    -1,
      -1,    30,    31,    32,    33,    34,    35,    36,    -1,    -1,
      39,    -1,    -1,    -1,    -1,    -1,    45,    46,    47,    48,
      49,    50,    51,    52,    53,    54,    55,    56,    57,    -1,
      59,    60,    61,    62,    63,    64,    65,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    75,    76,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    90,    91,    -1,    -1,    -1,    -1,    -1,    -1,    98,
      -1,    -1,   101,   102,    -1,   104,   105,    -1,   107,    -1,
      -1,   110,   111,   112,   113,   114,   115,   116,   117,   118,
      -1,   120,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,     0,    -1,    -1,   138,
      -1,   140,    -1,    -1,     8,     9,    10,   146,   147,    13,
      14,    15,   151,    17,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    25,    26,    27,    28,    29,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    37,    38,    -1,    40,    41,    42,    43,
      44,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    68,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    77,    78,    79,    80,    81,    82,    83,
      84,    85,    86,    87,    88,    89,    -1,    -1,    -1,    -1,
      94,    95,    96,    97,    -1,    99,   100,    -1,    -1,   103,
      -1,    -1,   106,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,   120,    -1,    -1,   123,
      -1,   125,   126,   127,   128,   129,   130,   131,   132,   133,
      -1,   135,    -1,   137,   138,    -1,   140,   141,   142,   143,
     144,   145,     0,    -1,    -1,   149,   150,    -1,    -1,    -1,
       8,     9,    10,    -1,    -1,    13,    14,    15,    -1,    17,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    25,    -1,    27,
      28,    29,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    37,
      38,    -1,    40,    41,    42,    43,    44,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      68,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    77,
      78,    79,    80,    81,    82,    83,    84,    85,    86,    87,
      88,    89,    -1,    -1,    -1,    -1,    94,    95,    96,    97,
      -1,    99,   100,    -1,    -1,   103,    -1,    -1,   106,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,   120,    -1,    -1,   123,    -1,   125,    -1,   127,
     128,   129,   130,   131,   132,   133,    -1,   135,    -1,   137,
     138,    -1,   140,   141,   142,   143,   144,   145,     0,    -1,
      -1,   149,   150,    -1,    -1,    -1,     8,     9,    10,    -1,
      -1,    13,    14,    15,    -1,    17,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    25,    -1,    27,    28,    29,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    37,    38,    -1,    40,    41,
      42,    43,    44,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    68,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    77,    78,    79,    80,    81,
      82,    83,    84,    85,    86,    87,    88,    89,    -1,    -1,
      -1,    -1,    94,    95,    96,    97,    -1,    99,   100,    -1,
      -1,   103,    -1,    -1,   106,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,   120,    -1,
      -1,   123,    -1,   125,    -1,   127,   128,   129,   130,   131,
     132,   133,    -1,   135,    -1,   137,   138,    -1,   140,   141,
     142,   143,   144,   145,     0,    -1,    -1,   149,   150,    -1,
      -1,    -1,     8,     9,    10,    -1,    -1,    13,    14,    15,
      -1,    17,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    25,
      26,    27,    28,    29,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    37,    38,    -1,    40,    41,    42,    43,    44,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    68,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    77,    78,    79,    80,    81,    82,    83,    84,    85,
      86,    87,    88,    89,    -1,    -1,    -1,    -1,    94,    95,
      96,    97,    -1,    -1,   100,    -1,    -1,   103,    -1,    -1,
     106,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,   120,    -1,    -1,   123,    -1,   125,
     126,   127,   128,    -1,   130,   131,   132,   133,    -1,   135,
      -1,   137,   138,    -1,   140,   141,   142,   143,   144,   145,
       0,    -1,    -1,   149,   150,    -1,    -1,    -1,     8,     9,
      10,    -1,    -1,    13,    14,    15,    -1,    17,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    25,    26,    27,    28,    29,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    37,    38,    -1,
      40,    41,    42,    43,    44,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    68,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    77,    78,    79,
      80,    81,    82,    83,    84,    85,    86,    87,    88,    89,
      -1,    -1,    -1,    -1,    94,    95,    96,    97,    -1,    -1,
     100,    -1,    -1,   103,    -1,    -1,   106,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
     120,    -1,    -1,   123,    -1,   125,   126,   127,   128,    -1,
     130,   131,   132,   133,    -1,   135,    -1,   137,   138,    -1,
     140,   141,   142,   143,   144,   145,     0,    -1,    -1,   149,
     150,    -1,    -1,    -1,     8,     9,    10,    -1,    -1,    13,
      14,    15,    -1,    17,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    25,    -1,    27,    28,    29,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    37,    38,    -1,    40,    41,    42,    43,
      44,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    68,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    77,    78,    79,    80,    81,    82,    83,
      84,    85,    86,    87,    88,    89,    -1,    -1,    -1,    -1,
      94,    95,    96,    97,    -1,    99,   100,    -1,    -1,   103,
      -1,    -1,   106,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,   120,    -1,    -1,   123,
      -1,   125,    -1,   127,   128,   129,   130,   131,   132,   133,
      -1,   135,    -1,   137,   138,    -1,   140,   141,   142,   143,
     144,   145,     0,    -1,    -1,   149,   150,    -1,    -1,    -1,
       8,     9,    10,    -1,    -1,    13,    14,    15,    -1,    17,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    25,    -1,    27,
      28,    29,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    37,
      38,    -1,    40,    41,    42,    43,    44,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      68,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    77,
      78,    79,    80,    81,    82,    83,    84,    85,    86,    87,
      88,    89,    -1,    -1,    -1,    -1,    94,    95,    96,    97,
      -1,    -1,   100,    -1,    -1,   103,    -1,    -1,   106,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,   120,    -1,    -1,   123,    -1,   125,    -1,   127,
     128,    -1,   130,   131,   132,   133,    -1,   135,    -1,   137,
     138,    -1,   140,   141,   142,   143,   144,   145,     0,    -1,
      -1,   149,   150,    -1,    -1,    -1,     8,     9,    10,    -1,
      -1,    13,    14,    15,    -1,    17,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    25,    26,    27,    28,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    37,    38,    -1,    40,    41,
      42,    43,    44,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    68,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    77,    78,    79,    80,    81,
      82,    83,    84,    85,    86,    87,    88,    89,    -1,    -1,
      -1,    -1,    94,    95,    96,    97,    -1,    -1,   100,    -1,
      -1,   103,    -1,    -1,   106,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,   120,    -1,
      -1,    -1,    -1,   125,   126,   127,   128,    -1,   130,   131,
     132,   133,    -1,   135,   136,   137,   138,    -1,   140,   141,
     142,   143,   144,   145,     0,    -1,    -1,   149,    -1,    -1,
      -1,    -1,     8,     9,    10,    -1,    -1,    13,    14,    15,
      -1,    17,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    25,
      26,    27,    28,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    37,    38,    -1,    40,    41,    42,    43,    44,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    68,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    77,    78,    79,    80,    81,    82,    83,    84,    85,
      86,    87,    88,    89,    -1,    -1,    -1,    -1,    94,    95,
      96,    97,    -1,    -1,   100,    -1,    -1,   103,    -1,    -1,
     106,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,   120,    -1,    -1,    -1,    -1,   125,
     126,   127,   128,    -1,   130,   131,   132,   133,    -1,   135,
     136,   137,   138,    -1,   140,   141,   142,   143,   144,   145,
       0,    -1,    -1,   149,    -1,    -1,    -1,    -1,     8,     9,
      10,    -1,    -1,    13,    14,    15,    -1,    17,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    25,    -1,    27,    28,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    37,    38,    -1,
      40,    41,    42,    43,    44,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    68,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    77,    78,    79,
      80,    81,    82,    83,    84,    85,    86,    87,    88,    89,
      -1,    -1,    -1,    -1,    94,    95,    96,    97,    -1,    -1,
     100,    -1,    -1,   103,    -1,    -1,   106,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
     120,    -1,    -1,    -1,    -1,   125,    -1,   127,   128,    -1,
     130,   131,   132,   133,    -1,   135,   136,   137,   138,    -1,
     140,   141,   142,   143,   144,   145,     0,    -1,    -1,   149,
      -1,    -1,    -1,    -1,     8,     9,    10,    -1,    -1,    -1,
      14,    15,    -1,    17,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    25,    26,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    37,    38,    -1,    40,    41,    42,    43,
      44,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    68,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    77,    78,    79,    80,    81,    82,    83,
      84,    85,    86,    87,    88,    89,    -1,    -1,    -1,    -1,
      94,    95,    96,    97,    -1,    99,   100,    -1,    -1,   103,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,   120,    -1,    -1,    -1,
      -1,   125,   126,   127,   128,   129,   130,   131,   132,   133,
      -1,   135,    -1,    -1,   138,    -1,   140,   141,   142,   143,
     144,   145,     0,    -1,    -1,   149,    -1,    -1,    -1,    -1,
       8,     9,    10,    -1,    -1,    -1,    14,    15,    -1,    17,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    25,    26,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    37,
      38,    -1,    40,    41,    42,    43,    44,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      68,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    77,
      78,    79,    80,    81,    82,    83,    84,    85,    86,    87,
      88,    89,    -1,    -1,    -1,    -1,    94,    95,    96,    97,
      -1,    99,   100,    -1,    -1,   103,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,   120,    -1,    -1,    -1,    -1,   125,   126,   127,
     128,   129,   130,   131,   132,   133,    -1,   135,    -1,    -1,
     138,    -1,   140,   141,   142,   143,   144,   145,     0,    -1,
      -1,   149,    -1,    -1,    -1,    -1,     8,     9,    10,    -1,
      -1,    -1,    14,    15,    -1,    17,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    25,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    37,    38,    -1,    40,    41,
      42,    43,    44,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    68,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    77,    78,    79,    80,    81,
      82,    83,    84,    85,    86,    87,    88,    89,    -1,    -1,
      -1,    -1,    94,    95,    96,    97,    -1,    99,   100,    -1,
      -1,   103,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,   120,    -1,
      -1,    -1,    -1,   125,    -1,   127,   128,   129,   130,   131,
     132,   133,    -1,   135,    -1,    -1,   138,    -1,   140,   141,
     142,   143,   144,   145,     0,    -1,    -1,   149,    -1,    -1,
      -1,    -1,     8,     9,    10,    -1,    -1,    -1,    14,    15,
      -1,    17,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    25,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    37,    38,    -1,    40,    41,    42,    43,    44,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    68,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    77,    78,    79,    80,    81,    82,    83,    84,    85,
      86,    87,    88,    89,    -1,    -1,    -1,    -1,    94,    95,
      96,    97,    -1,    99,   100,    -1,    -1,   103,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,   120,    -1,    -1,    -1,    -1,   125,
      -1,   127,   128,   129,   130,   131,   132,   133,    -1,   135,
      -1,    -1,   138,    -1,   140,   141,   142,   143,   144,   145,
      -1,    -1,    -1,   149,     3,     4,     5,     6,     7,    -1,
      -1,    -1,    11,    12,    -1,    -1,    -1,    16,    -1,    18,
      19,    20,    21,    22,    23,    24,    -1,    -1,    -1,    -1,
      -1,    30,    31,    32,    33,    34,    35,    36,    -1,    -1,
      39,    -1,    -1,    -1,    -1,    -1,    45,    46,    47,    48,
      49,    50,    51,    52,    53,    54,    55,    56,    57,    -1,
      59,    60,    61,    62,    63,    64,    65,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    75,    76,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    90,    91,    -1,    -1,    -1,    -1,    -1,    -1,    98,
      -1,    -1,   101,   102,    -1,   104,   105,    -1,   107,    -1,
      -1,   110,   111,   112,   113,   114,   115,   116,   117,   118,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,   138,
      -1,     3,     4,     5,     6,     7,    -1,   146,   147,    11,
      12,    -1,   151,    -1,    16,    -1,    18,    19,    20,    21,
      22,    23,    24,    -1,    -1,    -1,    -1,    -1,    30,    31,
      32,    33,    34,    35,    36,    -1,    -1,    39,    -1,    -1,
      -1,    -1,    -1,    45,    46,    47,    48,    49,    50,    51,
      52,    53,    54,    55,    56,    57,    -1,    59,    60,    61,
      62,    63,    64,    65,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    75,    76,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    90,    91,
      -1,    -1,    -1,    -1,    -1,    -1,    98,    -1,    -1,   101,
     102,    -1,   104,   105,    -1,   107,    -1,    -1,   110,   111,
     112,   113,   114,   115,   116,   117,   118,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,   138,    -1,     3,     4,
       5,     6,     7,    -1,   146,   147,    11,    12,    -1,   151,
      -1,    16,    -1,    18,    19,    20,    21,    22,    23,    24,
      -1,    -1,    -1,    -1,    -1,    30,    31,    32,    33,    34,
      35,    36,    -1,    -1,    39,    -1,    -1,    -1,    -1,    -1,
      45,    46,    47,    48,    49,    50,    51,    52,    53,    54,
      55,    56,    57,    -1,    59,    60,    61,    62,    63,    64,
      65,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      75,    76,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    90,    91,    -1,    -1,    -1,
      -1,    -1,    -1,    98,    -1,    -1,   101,   102,    -1,   104,
     105,    -1,   107,    -1,    -1,   110,   111,   112,   113,   114,
     115,   116,   117,   118,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,     3,     4,     5,    -1,     7,
      -1,   146,   147,    11,    12,    -1,   151,    -1,    16,    -1,
      18,    19,    20,    21,    22,    23,    24,    -1,    -1,    -1,
      -1,    -1,    30,    31,    32,    33,    34,    35,    36,    -1,
      -1,    39,    -1,    -1,    -1,    -1,    -1,    -1,    46,    -1,
      -1,    49,    50,    51,    52,    53,    54,    55,    56,    57,
      58,    59,    60,    61,    62,    63,    64,    65,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    75,    76,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    90,    91,    -1,    -1,    -1,    -1,    -1,    -1,
      98,    -1,    -1,   101,   102,    -1,   104,   105,    -1,   107,
     108,   109,   110,   111,   112,   113,   114,   115,   116,   117,
     118,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,   140,     3,     4,     5,    -1,     7,   146,   147,
      -1,    11,    12,   151,    -1,    -1,    16,    -1,    18,    19,
      20,    21,    22,    23,    24,    -1,    -1,    -1,    -1,    -1,
      30,    31,    32,    33,    34,    35,    36,    -1,    -1,    39,
      -1,    -1,    -1,    -1,    -1,    -1,    46,    -1,    -1,    49,
      50,    51,    52,    53,    54,    55,    56,    57,    58,    59,
      60,    61,    62,    63,    64,    65,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    75,    76,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      90,    91,    -1,    -1,    -1,    -1,    -1,    -1,    98,    -1,
      -1,   101,   102,    -1,   104,   105,    -1,   107,   108,   109,
     110,   111,   112,   113,   114,   115,   116,   117,   118,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
       3,     4,     5,     6,     7,    -1,   146,   147,    11,    12,
      -1,   151,    -1,    16,    -1,    18,    19,    20,    21,    22,
      23,    24,    -1,    -1,    -1,    -1,    -1,    30,    31,    32,
      33,    34,    35,    36,    -1,    -1,    39,    -1,    -1,    -1,
      -1,    -1,    45,    46,    -1,    48,    49,    50,    51,    52,
      53,    54,    55,    56,    57,    -1,    59,    60,    61,    62,
      63,    64,    65,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    75,    76,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    90,    91,    -1,
      -1,    -1,    -1,    -1,    -1,    98,    -1,    -1,   101,   102,
      -1,   104,   105,    -1,   107,    -1,    -1,   110,   111,   112,
     113,   114,   115,   116,   117,   118,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,     3,     4,     5,
      -1,     7,    -1,   146,   147,    11,    12,    -1,   151,    -1,
      16,    -1,    18,    19,    20,    21,    22,    23,    24,    -1,
      -1,    -1,    -1,    -1,    30,    31,    32,    33,    34,    35,
      36,    -1,    -1,    39,    -1,    -1,    -1,    -1,    -1,    -1,
      46,    -1,    -1,    49,    50,    51,    52,    53,    54,    55,
      56,    57,    58,    59,    60,    61,    62,    63,    64,    65,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    75,
      76,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    90,    91,    -1,    -1,    -1,    -1,
      -1,    -1,    98,    -1,    -1,   101,   102,    -1,   104,   105,
      -1,   107,   108,   109,   110,   111,   112,   113,   114,   115,
     116,   117,   118,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,     3,     4,     5,    -1,     7,    -1,
     146,   147,    11,    12,    -1,   151,    -1,    16,    -1,    18,
      19,    20,    21,    22,    23,    24,    -1,    -1,    -1,    -1,
      -1,    30,    31,    32,    33,    34,    35,    36,    -1,    -1,
      39,    -1,    -1,    -1,    -1,    -1,    -1,    46,    -1,    -1,
      49,    50,    51,    52,    53,    54,    55,    56,    57,    58,
      59,    60,    61,    62,    63,    64,    65,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    75,    76,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    90,    91,    -1,    -1,    -1,    -1,    -1,    -1,    98,
      -1,    -1,   101,   102,    -1,   104,   105,    -1,   107,   108,
     109,   110,   111,   112,   113,   114,   115,   116,   117,   118,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,     3,     4,     5,    -1,     7,    -1,   146,   147,    11,
      12,    -1,   151,    -1,    16,    -1,    18,    19,    20,    21,
      22,    23,    24,    -1,    -1,    -1,    -1,    -1,    30,    31,
      32,    33,    34,    35,    36,    -1,    -1,    39,    -1,    -1,
      -1,    -1,    -1,    -1,    46,    -1,    -1,    49,    50,    51,
      52,    53,    54,    55,    56,    57,    58,    59,    60,    61,
      62,    63,    64,    65,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    75,    76,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    90,    91,
      -1,    -1,    -1,    -1,    -1,    -1,    98,    -1,    -1,   101,
     102,    -1,   104,   105,    -1,   107,   108,   109,   110,   111,
     112,   113,   114,   115,   116,   117,   118,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,     3,     4,
       5,    -1,     7,    -1,   146,   147,    11,    12,    -1,   151,
      -1,    16,    -1,    18,    19,    20,    21,    22,    23,    24,
      -1,    -1,    -1,    -1,    -1,    30,    31,    32,    33,    34,
      35,    36,    -1,    -1,    39,    -1,    -1,    -1,    -1,    -1,
      -1,    46,    -1,    -1,    49,    50,    51,    52,    53,    54,
      55,    56,    57,    58,    59,    60,    61,    62,    63,    64,
      65,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      75,    76,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    90,    91,    -1,    -1,    -1,
      -1,    -1,    -1,    98,    -1,    -1,   101,   102,    -1,   104,
     105,    -1,   107,   108,    -1,   110,   111,   112,   113,   114,
     115,   116,   117,   118,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,     3,     4,     5,    -1,     7,
      -1,   146,   147,    11,    12,    -1,   151,    -1,    16,    -1,
      18,    19,    20,    21,    22,    23,    24,    -1,    -1,    -1,
      -1,    -1,    30,    31,    32,    33,    34,    35,    36,    -1,
      -1,    39,    -1,    -1,    -1,    -1,    -1,    -1,    46,    -1,
      -1,    49,    50,    51,    52,    53,    54,    55,    56,    57,
      58,    59,    60,    61,    62,    63,    64,    65,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    75,    76,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    90,    91,    -1,    -1,    -1,    -1,    -1,    -1,
      98,    -1,    -1,   101,   102,    -1,   104,   105,    -1,    -1,
     108,   109,   110,   111,   112,   113,   114,   115,   116,   117,
     118,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,     3,     4,     5,    -1,     7,    -1,   146,   147,
      11,    12,    -1,   151,    -1,    16,    -1,    18,    19,    20,
      21,    22,    23,    24,    -1,    -1,    -1,    -1,    -1,    30,
      31,    32,    33,    34,    35,    36,    -1,    -1,    39,    -1,
      -1,    -1,    -1,    -1,    -1,    46,    -1,    -1,    49,    50,
      51,    52,    53,    54,    55,    56,    57,    58,    59,    60,
      61,    62,    63,    64,    65,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    75,    76,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    90,
      91,    -1,    -1,    -1,    -1,    -1,    -1,    98,    -1,    -1,
     101,   102,    -1,   104,   105,    -1,   107,   108,    -1,   110,
     111,   112,   113,   114,   115,   116,   117,   118,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,     3,
       4,     5,    -1,     7,    -1,   146,   147,    11,    12,    -1,
     151,    -1,    16,    -1,    18,    19,    20,    21,    22,    23,
      24,    -1,    -1,    -1,    -1,    -1,    30,    31,    32,    33,
      34,    35,    36,    -1,    -1,    39,    -1,    -1,    -1,    -1,
      -1,    -1,    46,    -1,    -1,    49,    50,    51,    52,    53,
      54,    55,    56,    57,    58,    59,    60,    61,    62,    63,
      64,    65,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    75,    76,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    90,    91,    -1,    -1,
      -1,    -1,    -1,    -1,    98,    -1,    -1,   101,   102,    -1,
     104,   105,    -1,    -1,   108,    -1,   110,   111,   112,   113,
     114,   115,   116,   117,   118,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,     3,     4,     5,    -1,
       7,    -1,   146,   147,    11,    12,    -1,   151,    -1,    16,
      -1,    18,    19,    20,    21,    22,    23,    24,    -1,    -1,
      -1,    -1,    -1,    30,    31,    32,    33,    34,    35,    36,
      -1,    -1,    39,    -1,    -1,    -1,    -1,    -1,    -1,    46,
      -1,    -1,    49,    50,    51,    52,    53,    54,    55,    56,
      57,    -1,    59,    60,    61,    62,    63,    64,    65,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    75,    76,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    90,    91,    -1,    -1,    -1,    -1,    -1,
      -1,    98,    -1,    -1,   101,   102,    -1,   104,   105,    -1,
      -1,    -1,    -1,   110,   111,   112,   113,   114,   115,   116,
     117,   118,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,   138,    -1,   140,     3,     4,     5,    -1,     7,   146,
     147,    -1,    11,    12,   151,    -1,    -1,    16,    -1,    18,
      19,    20,    21,    22,    23,    24,    -1,    -1,    -1,    -1,
      -1,    30,    31,    32,    33,    34,    35,    36,    -1,    -1,
      39,    -1,    -1,    -1,    -1,    -1,    -1,    46,    -1,    -1,
      49,    50,    51,    52,    53,    54,    55,    56,    57,    -1,
      59,    60,    61,    62,    63,    64,    65,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    75,    76,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    90,    91,    -1,    -1,    -1,    -1,    -1,    -1,    98,
      -1,    -1,   101,   102,    -1,   104,   105,    -1,    -1,    -1,
      -1,   110,   111,   112,   113,   114,   115,   116,   117,   118,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,   140,     3,     4,     5,    -1,     7,   146,   147,    -1,
      11,    12,   151,    -1,    -1,    16,    -1,    18,    19,    20,
      21,    22,    23,    24,    -1,    -1,    -1,    -1,    -1,    30,
      31,    32,    33,    34,    35,    36,    -1,    -1,    39,    -1,
      -1,    -1,    -1,    -1,    -1,    46,    -1,    -1,    49,    50,
      51,    52,    53,    54,    55,    56,    57,    -1,    59,    60,
      61,    62,    63,    64,    65,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    75,    76,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    90,
      91,    -1,    -1,    -1,    -1,    -1,    -1,    98,    -1,    -1,
     101,   102,    -1,   104,   105,    -1,   107,    -1,    -1,   110,
     111,   112,   113,   114,   115,   116,   117,   118,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,     3,
       4,     5,    -1,     7,    -1,   146,   147,    11,    12,    -1,
     151,    -1,    16,    -1,    18,    19,    20,    21,    22,    23,
      24,    -1,    -1,    -1,    -1,    -1,    30,    31,    32,    33,
      34,    35,    36,    -1,    -1,    39,    -1,    -1,    -1,    -1,
      -1,    -1,    46,    -1,    -1,    49,    50,    51,    52,    53,
      54,    55,    56,    57,    -1,    59,    60,    61,    62,    63,
      64,    65,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    75,    76,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    90,    91,    -1,    -1,
      -1,    -1,    -1,    -1,    98,    -1,    -1,   101,   102,    -1,
     104,   105,    -1,   107,    -1,    -1,   110,   111,   112,   113,
     114,   115,   116,   117,   118,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,     3,     4,     5,    -1,
       7,    -1,   146,   147,    11,    12,    -1,   151,    -1,    16,
      -1,    18,    19,    20,    21,    22,    23,    24,    -1,    -1,
      -1,    -1,    -1,    30,    31,    32,    33,    34,    35,    36,
      -1,    -1,    39,    -1,    -1,    -1,    -1,    -1,    -1,    46,
      -1,    -1,    49,    50,    51,    52,    53,    54,    55,    56,
      57,    -1,    59,    60,    61,    62,    63,    64,    65,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    75,    76,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    90,    91,    -1,    -1,    -1,    -1,    -1,
      -1,    98,    -1,    -1,   101,   102,    -1,   104,   105,    -1,
     107,    -1,    -1,   110,   111,   112,   113,   114,   115,   116,
     117,   118,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,     3,     4,     5,    -1,     7,    -1,   146,
     147,    11,    12,    -1,   151,    -1,    16,    -1,    18,    19,
      20,    21,    22,    23,    24,    -1,    -1,    -1,    -1,    -1,
      30,    31,    32,    33,    34,    35,    36,    -1,    -1,    39,
      -1,    -1,    -1,    -1,    -1,    -1,    46,    -1,    -1,    49,
      50,    51,    52,    53,    54,    55,    56,    57,    -1,    59,
      60,    61,    62,    63,    64,    65,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    75,    76,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      90,    91,    -1,    -1,    -1,    -1,    -1,    -1,    98,    -1,
      -1,   101,   102,    -1,   104,   105,    -1,   107,    -1,    -1,
     110,   111,   112,   113,   114,   115,   116,   117,   118,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
       3,     4,     5,    -1,     7,    -1,   146,   147,    11,    12,
      -1,   151,    -1,    16,    -1,    18,    19,    20,    21,    22,
      23,    24,    -1,    -1,    -1,    -1,    -1,    30,    31,    32,
      33,    34,    35,    36,    -1,    -1,    39,    -1,    -1,    -1,
      -1,    -1,    -1,    46,    -1,    -1,    49,    50,    51,    52,
      53,    54,    55,    56,    57,    -1,    59,    60,    61,    62,
      63,    64,    65,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    75,    76,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    90,    91,    -1,
      -1,    -1,    -1,    -1,    -1,    98,    -1,    -1,   101,   102,
      -1,   104,   105,    -1,   107,    -1,    -1,   110,   111,   112,
     113,   114,   115,   116,   117,   118,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,     3,     4,     5,
      -1,     7,    -1,   146,   147,    11,    12,    -1,   151,    -1,
      16,    -1,    18,    19,    20,    21,    22,    23,    24,    -1,
      -1,    -1,    -1,    -1,    30,    31,    32,    33,    34,    35,
      36,    -1,    -1,    39,    -1,    -1,    -1,    -1,    -1,    -1,
      46,    -1,    -1,    49,    50,    51,    52,    53,    54,    55,
      56,    57,    -1,    59,    60,    61,    62,    63,    64,    65,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    75,
      76,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    90,    91,    -1,    -1,    -1,    -1,
      -1,    -1,    98,    -1,    -1,   101,   102,    -1,   104,   105,
      -1,    -1,    -1,    -1,   110,   111,   112,   113,   114,   115,
     116,   117,   118,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,     3,     4,     5,    -1,     7,    -1,
     146,   147,    11,    12,    -1,   151,    -1,    16,    -1,    18,
      19,    20,    21,    22,    23,    24,    -1,    -1,    -1,    -1,
      -1,    30,    31,    32,    33,    34,    35,    36,    -1,    -1,
      39,    -1,    -1,    -1,    -1,    -1,    -1,    46,    -1,    -1,
      49,    50,    51,    52,    53,    54,    55,    56,    57,    -1,
      59,    60,    61,    62,    63,    64,    65,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    75,    76,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    90,    91,    -1,    -1,    -1,    -1,    -1,    -1,    98,
      -1,    -1,   101,   102,    -1,   104,   105,    -1,    -1,    -1,
      -1,   110,   111,   112,   113,   114,   115,   116,   117,   118,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,     3,     4,     5,    -1,     7,    -1,   146,   147,    11,
      12,    -1,   151,    -1,    16,    -1,    18,    19,    20,    21,
      22,    23,    24,    -1,    -1,    -1,    -1,    -1,    30,    31,
      32,    33,    34,    35,    36,    -1,    -1,    39,    -1,    -1,
      -1,    -1,    -1,    -1,    46,    -1,    -1,    49,    50,    51,
      52,    53,    54,    55,    56,    57,    -1,    59,    60,    61,
      62,    63,    64,    65,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    75,    76,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    90,    91,
      -1,    -1,    -1,    -1,    -1,    -1,    98,    -1,    -1,   101,
     102,    -1,   104,   105,    -1,    -1,    -1,    -1,   110,   111,
     112,   113,   114,   115,   116,   117,   118,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,     3,     4,
       5,    -1,     7,    -1,   146,   147,    11,    12,    -1,   151,
      -1,    16,    -1,    18,    19,    20,    21,    22,    23,    24,
      -1,    -1,    -1,    -1,    -1,    30,    31,    32,    33,    34,
      35,    36,    -1,    -1,    39,    -1,    -1,    -1,    -1,    -1,
      -1,    46,    -1,    -1,    49,    50,    51,    52,    53,    54,
      55,    56,    57,    -1,    59,    60,    61,    62,    63,    64,
      65,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    98,    -1,    -1,   101,   102,    -1,   104,
     105,    -1,   107,    -1,    -1,   110,   111,   112,   113,   114,
     115,   116,   117,   118,    -1,    -1,    -1,    -1,    -1,    -1,
       3,     4,     5,    -1,     7,    -1,    -1,    -1,    11,    12,
      -1,    -1,    -1,    16,    -1,    18,    19,    20,    21,    22,
      23,    24,    -1,    -1,    -1,    -1,   151,    30,    31,    32,
      33,    34,    35,    36,    -1,    -1,    39,    -1,    -1,    -1,
      -1,    -1,    -1,    46,    -1,    -1,    49,    50,    51,    52,
      53,    54,    55,    56,    57,    -1,    59,    60,    61,    62,
      63,    64,    65,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    98,    -1,    -1,   101,   102,
      -1,   104,   105,    -1,   107,    -1,    -1,   110,   111,   112,
     113,   114,   115,   116,   117,   118,    -1,    -1,    -1,    -1,
      -1,    -1,     3,     4,     5,    -1,     7,    -1,    -1,    -1,
      11,    12,    -1,    -1,    -1,    16,    -1,    18,    19,    20,
      21,    22,    23,    24,    -1,    -1,    -1,    -1,   151,    30,
      31,    32,    33,    34,    35,    36,    -1,    -1,    39,    -1,
      -1,    -1,    -1,    -1,    -1,    46,    -1,    -1,    49,    50,
      51,    52,    53,    54,    55,    56,    57,    -1,    59,    60,
      61,    62,    63,    64,    65,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    94,    -1,    -1,    -1,    98,    -1,    -1,
     101,   102,    -1,   104,   105,    -1,    -1,    -1,    -1,   110,
     111,   112,   113,   114,   115,   116,   117,   118,    -1,    -1,
      -1,    -1,    -1,    -1,     3,     4,     5,    -1,     7,    -1,
      -1,    -1,    11,    12,    -1,    -1,    -1,    16,    -1,    18,
      19,    20,    21,    22,    23,    24,    -1,    -1,    -1,    -1,
     151,    30,    31,    32,    33,    34,    35,    36,    -1,    -1,
      39,    -1,    -1,    -1,    -1,    -1,    -1,    46,    -1,    -1,
      49,    50,    51,    52,    53,    54,    55,    56,    57,    -1,
      59,    60,    61,    62,    63,    64,    65,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    98,
      -1,    -1,   101,   102,    -1,   104,   105,    -1,    -1,    -1,
      -1,   110,   111,   112,   113,   114,   115,   116,   117,   118,
      -1,    -1,    -1,    -1,    -1,    -1,   125,     3,     4,     5,
      -1,     7,    -1,    -1,    -1,    11,    12,    -1,    -1,    -1,
      16,    -1,    18,    19,    20,    21,    22,    23,    24,    -1,
      -1,    -1,   151,    -1,    30,    31,    32,    33,    34,    35,
      36,    -1,    -1,    39,    -1,    -1,    -1,    -1,    -1,    -1,
      46,    -1,    -1,    49,    50,    51,    52,    53,    54,    55,
      56,    57,    -1,    59,    60,    61,    62,    63,    64,    65,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    98,    -1,    -1,   101,   102,    -1,   104,   105,
      -1,    -1,    -1,    -1,   110,   111,   112,   113,   114,   115,
     116,   117,   118,    -1,    -1,    -1,    -1,    -1,    -1,     3,
       4,     5,    -1,     7,    -1,    -1,    -1,    11,    12,    -1,
      -1,    -1,    16,    -1,    18,    19,    20,    21,    22,    23,
      24,    -1,    -1,    -1,    -1,   151,    30,    31,    32,    33,
      34,    35,    36,    -1,    -1,    39,    -1,    -1,    -1,    -1,
      -1,    -1,    46,    -1,    -1,    49,    50,    51,    52,    53,
      54,    55,    56,    57,    -1,    59,    60,    61,    62,    63,
      64,    65,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    98,    -1,    -1,   101,   102,    -1,
     104,   105,    -1,    -1,    -1,    -1,   110,   111,   112,   113,
     114,   115,   116,   117,   118,    -1,    -1,    -1,    -1,    -1,
      -1,     3,     4,     5,    -1,     7,    -1,    -1,    -1,    11,
      12,    -1,    -1,    -1,    16,    -1,    18,    19,    20,    21,
      22,    23,    24,    -1,    -1,    -1,    -1,   151,    30,    31,
      32,    33,    34,    35,    36,    -1,    -1,    39,    -1,    -1,
      -1,    -1,    -1,    -1,    46,    -1,    -1,    49,    50,    51,
      52,    53,    54,    55,    56,    57,    -1,    59,    60,    61,
      62,    63,    64,    65,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    98,    -1,    -1,   101,
     102,    -1,   104,   105,    -1,    -1,    -1,    -1,   110,   111,
     112,   113,   114,   115,   116,   117,   118,    -1,    -1,    -1,
      -1,    -1,    -1,     3,     4,     5,    -1,     7,    -1,    -1,
      -1,    11,    12,    -1,    -1,    -1,    16,    -1,    18,    19,
      20,    21,    22,    23,    24,    -1,    -1,    -1,    -1,   151,
      30,    31,    32,    33,    34,    35,    36,    -1,    -1,    39,
      -1,    -1,    -1,    -1,    -1,    -1,    46,    -1,    -1,    49,
      50,    51,    52,    53,    54,    55,    56,    57,    -1,    59,
      60,    61,    62,    63,    64,    65,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    98,    -1,
      -1,   101,   102,    -1,   104,   105,    -1,    -1,    -1,    -1,
     110,   111,   112,   113,   114,   115,   116,   117,   118,    -1,
      -1,    -1,    -1,    -1,    -1,     3,     4,     5,     6,     7,
       8,     9,    10,    11,    12,    13,    14,    15,    16,    17,
      18,    19,    20,    21,    22,    23,    24,    25,    26,    -1,
      -1,   151,    30,    31,    32,    33,    34,    35,    36,    37,
      38,    39,    -1,    -1,    -1,    -1,    -1,    45,    46,    47,
      48,    49,    50,    51,    52,    53,    54,    55,    56,    57,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    75,    76,    77,
      78,    79,    80,    81,    82,    83,    -1,    -1,    86,    87,
      -1,    -1,    -1,    -1,    92,    93,    94,    95,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,   107,
     108,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,   130,   131,   132,   133,   134,   135,   136,    -1,
      -1,    -1,    -1,   141,   142,   143,   144,   145,   146,   147,
       3,     4,     5,     6,     7,     8,     9,    10,    11,    12,
      13,    14,    15,    16,    17,    18,    19,    20,    21,    22,
      23,    24,    25,    26,    -1,    -1,    -1,    30,    31,    32,
      33,    34,    35,    36,    37,    38,    39,    -1,    -1,    -1,
      -1,    -1,    45,    46,    47,    48,    49,    50,    51,    52,
      53,    54,    55,    56,    57,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    75,    76,    77,    78,    79,    80,    81,    82,
      83,    -1,    -1,    86,    87,    -1,    -1,    -1,    -1,    92,
      93,    94,    95,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,   107,   108,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,   130,   131,   132,
     133,   134,   135,    -1,    -1,    -1,    -1,    -1,   141,   142,
     143,   144,   145,   146,   147,     3,     4,     5,     6,     7,
       8,     9,    10,    11,    12,    13,    14,    15,    16,    17,
      18,    19,    20,    21,    22,    23,    24,    25,    26,    -1,
      -1,    -1,    30,    31,    32,    33,    34,    35,    36,    37,
      38,    39,    -1,    -1,    -1,    -1,    -1,    45,    46,    47,
      48,    49,    50,    51,    52,    53,    54,    -1,    56,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    75,    76,    77,
      78,    79,    80,    81,    82,    83,    -1,    -1,    86,    87,
      -1,    -1,    -1,    -1,    92,    93,    94,    95,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,   107,
     108,    -1,    -1,   111,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,   130,   131,   132,   133,   134,   135,    -1,    -1,
      -1,    -1,    -1,   141,   142,   143,   144,   145,   146,   147,
       3,     4,     5,     6,     7,     8,     9,    10,    11,    12,
      13,    14,    15,    16,    17,    18,    19,    20,    21,    22,
      23,    24,    25,    26,    -1,    -1,    -1,    30,    31,    32,
      33,    34,    35,    36,    37,    38,    39,    -1,    -1,    -1,
      -1,    -1,    45,    46,    47,    48,    49,    50,    51,    52,
      53,    -1,    -1,    56,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    75,    76,    77,    78,    79,    80,    81,    82,
      83,    -1,    -1,    86,    87,    -1,    -1,    -1,    -1,    92,
      93,    94,    95,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,   107,   108,    -1,    -1,   111,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,   130,   131,   132,
     133,   134,   135,    -1,    -1,    -1,    -1,    -1,   141,   142,
     143,   144,   145,   146,   147,     3,     4,     5,     6,     7,
       8,     9,    10,    11,    12,    13,    14,    15,    16,    17,
      18,    19,    20,    21,    22,    23,    24,    25,    26,    -1,
      -1,    -1,    30,    31,    32,    33,    34,    35,    36,    37,
      38,    39,    -1,    -1,    -1,    -1,    -1,    45,    46,    47,
      48,    49,    50,    51,    52,    53,    -1,    -1,    56,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    75,    76,    77,
      78,    79,    80,    81,    82,    83,    -1,    -1,    86,    87,
      -1,    -1,    -1,    -1,    92,    93,    94,    95,    33,    34,
      35,    36,    -1,    -1,    -1,    -1,    -1,    -1,    -1,   107,
     108,    -1,    -1,    -1,    49,    50,    51,    52,    -1,    -1,
      -1,    56,    -1,    -1,    59,    60,    61,    62,    63,    -1,
      -1,    -1,   130,   131,   132,   133,   134,   135,    -1,    -1,
      -1,    -1,    -1,   141,   142,   143,   144,   145,   146,   147,
      -1,    44,    -1,    -1,    -1,    90,    91,    -1,    -1,    -1,
      -1,    -1,    -1,    98,    -1,    -1,   101,    -1,    -1,   104,
     105,    -1,    -1,    -1,    -1,   110,   111,   112,   113,   114,
     115,   116,   117,   118,    77,    78,    79,    80,    81,    82,
      83,    84,    85,    86,    87,    88,    89,    -1,    -1,    -1,
     135,    94,    95,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,   151,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,   130,   131,   132,
     133,    -1,   135,    52,    53,    -1,    -1,    56,   141,   142,
     143,   144,   145,    -1,    -1,    -1,   149,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    75,    76,    77,    78,
      79,    80,    81,    82,    83,    -1,    -1,    86,    87,    -1,
      -1,    -1,    -1,    92,    93,    94,    95,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,   107,   108,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    52,    53,    -1,    -1,
      56,   130,   131,   132,   133,   134,   135,   136,    -1,    -1,
      -1,    -1,   141,   142,   143,   144,   145,   146,   147,    75,
      76,    77,    78,    79,    80,    81,    82,    83,    -1,    -1,
      86,    87,    -1,    -1,    -1,    -1,    92,    93,    94,    95,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,   107,   108,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    52,
      53,    -1,    -1,    56,   130,   131,   132,   133,   134,   135,
     136,    -1,    -1,    -1,    -1,   141,   142,   143,   144,   145,
     146,   147,    75,    76,    77,    78,    79,    80,    81,    82,
      83,    -1,    -1,    86,    87,    -1,    -1,    -1,    -1,    92,
      93,    94,    95,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,   107,   108,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    52,    53,    -1,    -1,    56,   130,   131,   132,
     133,   134,   135,   136,    -1,    -1,    -1,    -1,   141,   142,
     143,   144,   145,   146,   147,    75,    76,    77,    78,    79,
      80,    81,    82,    83,    -1,    -1,    86,    87,    -1,    -1,
      -1,    -1,    92,    93,    94,    95,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,   107,   108,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    52,    53,    -1,    -1,    56,
     130,   131,   132,   133,   134,   135,   136,    -1,    -1,    -1,
      -1,   141,   142,   143,   144,   145,   146,   147,    75,    76,
      77,    78,    79,    80,    81,    82,    83,    -1,    -1,    86,
      87,    -1,    -1,    -1,    -1,    92,    93,    94,    95,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
     107,   108,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    52,    53,
      -1,    -1,    56,   130,   131,   132,   133,   134,   135,   136,
      -1,    -1,    -1,    -1,   141,   142,   143,   144,   145,   146,
     147,    75,    76,    77,    78,    79,    80,    81,    82,    83,
      -1,    -1,    86,    87,    -1,    -1,    -1,    -1,    92,    93,
      94,    95,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,   107,   108,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    52,    53,    -1,    -1,    56,   130,   131,   132,   133,
     134,   135,   136,    -1,    -1,    -1,    -1,   141,   142,   143,
     144,   145,   146,   147,    75,    76,    77,    78,    79,    80,
      81,    82,    83,    -1,    -1,    86,    87,    -1,    -1,    -1,
      -1,    92,    93,    94,    95,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,   107,   108,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    52,    53,    -1,    -1,    56,   130,
     131,   132,   133,   134,   135,   136,    -1,    -1,    -1,    -1,
     141,   142,   143,   144,   145,   146,   147,    75,    76,    77,
      78,    79,    80,    81,    82,    83,    -1,    -1,    86,    87,
      -1,    -1,    -1,    -1,    92,    93,    94,    95,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,   107,
     108,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    52,    53,    -1,
      -1,    56,   130,   131,   132,   133,   134,   135,   136,    -1,
      -1,    -1,    -1,   141,   142,   143,   144,   145,   146,   147,
      75,    76,    77,    78,    79,    80,    81,    82,    83,    -1,
      -1,    86,    87,    -1,    -1,    -1,    -1,    92,    93,    94,
      95,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,   107,   108,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      52,    53,    -1,    -1,    56,   130,   131,   132,   133,   134,
     135,   136,    -1,    -1,    -1,    -1,   141,   142,   143,   144,
     145,   146,   147,    75,    76,    77,    78,    79,    80,    81,
      82,    83,    -1,    -1,    86,    87,    -1,    -1,    -1,    -1,
      92,    93,    94,    95,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,   107,   108,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    52,    53,    -1,    -1,    56,   130,   131,
     132,   133,   134,   135,   136,    -1,    -1,    -1,    -1,   141,
     142,   143,   144,   145,   146,   147,    75,    76,    77,    78,
      79,    80,    81,    82,    83,    -1,    -1,    86,    87,    -1,
      -1,    -1,    -1,    92,    93,    94,    95,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,   107,   108,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    52,    53,    -1,    -1,
      56,   130,   131,   132,   133,   134,   135,   136,    -1,    -1,
      -1,    -1,   141,   142,   143,   144,   145,   146,   147,    75,
      76,    77,    78,    79,    80,    81,    82,    83,    -1,    -1,
      86,    87,    -1,    -1,    -1,    -1,    92,    93,    94,    95,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,   107,   108,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    52,
      53,    -1,    -1,    56,   130,   131,   132,   133,   134,   135,
     136,    -1,    -1,    -1,    -1,   141,   142,   143,   144,   145,
     146,   147,    75,    76,    77,    78,    79,    80,    81,    82,
      83,    -1,    -1,    86,    87,    -1,    -1,    -1,    -1,    92,
      93,    94,    95,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,   107,   108,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    52,    53,    -1,    -1,    56,   130,   131,   132,
     133,   134,   135,   136,    -1,    -1,    -1,    -1,   141,   142,
     143,   144,   145,   146,   147,    75,    76,    77,    78,    79,
      80,    81,    82,    83,    -1,    -1,    86,    87,    -1,    -1,
      -1,    -1,    92,    93,    94,    95,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,   107,   108,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    52,    53,    -1,    -1,    56,
     130,   131,   132,   133,   134,   135,   136,    -1,    -1,    -1,
      -1,   141,   142,   143,   144,   145,   146,   147,    75,    76,
      77,    78,    79,    80,    81,    82,    83,    -1,    -1,    86,
      87,    -1,    -1,    -1,    -1,    92,    93,    94,    95,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
     107,   108,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    52,    53,
      -1,    -1,    56,   130,   131,   132,   133,   134,   135,   136,
      -1,    -1,    -1,    -1,   141,   142,   143,   144,   145,   146,
     147,    75,    76,    77,    78,    79,    80,    81,    82,    83,
      -1,    -1,    86,    87,    -1,    -1,    -1,    -1,    92,    93,
      94,    95,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,   107,   108,    77,    78,    79,    80,    81,
      82,    83,    84,    85,    86,    87,    88,    89,    -1,    -1,
      -1,    -1,    94,    95,    -1,    -1,   130,   131,   132,   133,
     134,   135,    -1,    -1,    -1,    -1,    -1,   141,   142,   143,
     144,   145,   146,   147,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,   130,   131,
     132,   133,    -1,   135,    -1,    -1,    -1,    -1,    -1,   141,
     142,   143,   144,   145,    -1,    -1,    -1,   149,    77,    78,
      79,    80,    81,    82,    83,    84,    85,    86,    87,    88,
      89,    -1,    -1,    -1,    -1,    94,    95,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,   130,   131,   132,   133,    -1,   135,    -1,    -1,    -1,
      -1,    -1,   141,   142,   143,   144,   145 ];

/* YYSTOS[STATE-NUM] -- The symbol kind of the accessing symbol of
   state STATE-NUM.  */
  #[allow(non_upper_case_globals)]
const yystos_: &'static [i32] = &[      0,   154,   155,     0,     1,     3,     4,     5,     6,     7,
      11,    12,    16,    18,    19,    20,    21,    22,    23,    24,
      30,    31,    32,    33,    34,    35,    36,    39,    45,    46,
      47,    48,    49,    50,    51,    52,    53,    54,    55,    56,
      57,    59,    60,    61,    62,    63,    64,    65,    75,    76,
      90,    91,    98,   101,   102,   104,   105,   107,   110,   111,
     112,   113,   114,   115,   116,   117,   118,   146,   147,   151,
     156,   157,   158,   165,   167,   169,   173,   174,   179,   180,
     183,   184,   185,   187,   188,   189,   191,   192,   201,   204,
     219,   229,   230,   231,   232,   233,   234,   235,   236,   237,
     238,   239,   248,   270,   280,   281,   330,   331,   332,   333,
     334,   335,   336,   339,   341,   342,   356,   357,   359,   360,
     361,   363,   364,   365,   366,   367,   405,   419,   158,     3,
       4,     5,     6,     7,     8,     9,    10,    11,    12,    13,
      14,    15,    16,    17,    18,    19,    20,    21,    22,    23,
      24,    25,    26,    30,    31,    32,    33,    34,    35,    36,
      37,    38,    39,    45,    46,    47,    48,    49,    50,    51,
      52,    53,    56,    75,    76,    77,    78,    79,    80,    81,
      82,    83,    86,    87,    92,    93,    94,    95,   107,   108,
     130,   131,   132,   133,   134,   135,   141,   142,   143,   144,
     145,   146,   147,   195,   196,   197,   199,   200,   356,    39,
      58,    98,   101,   107,   108,   109,   112,   147,   173,   174,
     184,   192,   201,   205,   211,   214,   216,   229,   363,   364,
     366,   367,   403,   404,   211,   136,   212,   213,   136,   208,
     212,   136,   140,   412,    54,   196,   412,   126,   159,   126,
      21,    22,    31,    32,   183,   201,   229,   248,   201,   201,
     201,    56,     1,    47,   101,   161,   162,   163,   165,   186,
     187,   419,   165,   221,   206,   216,   403,   419,   205,   402,
     403,   419,    46,    98,   125,   151,   173,   174,   191,   219,
     229,   363,   364,   367,   271,    54,    55,    57,   195,   345,
     358,   345,   346,   347,   139,   139,   139,   139,   201,   179,
     201,   361,   138,   140,   411,   417,   418,    40,    41,    42,
      43,    44,    37,    38,   136,   370,   371,   372,   373,   419,
     370,   372,    26,   126,   208,   212,   240,   282,    28,   241,
     278,   129,   125,   101,   107,   188,   129,    25,    77,    78,
      79,    80,    81,    82,    83,    84,    85,    86,    87,    88,
      89,    94,    95,   100,   130,   131,   132,   133,   135,   141,
     142,   143,   144,   145,   149,   203,   203,    68,    96,    97,
     128,   409,   220,   169,   176,   176,   177,   178,   177,   176,
     411,   418,    98,   185,   192,   229,   253,   363,   364,   367,
      52,    56,    94,    98,   193,   194,   229,   363,   364,   367,
     194,    33,    34,    35,    36,    49,    50,    51,    52,    56,
     136,   172,   195,   365,   400,   211,    97,   409,   410,   282,
     333,    99,    99,   125,   205,    56,   205,   205,   205,   345,
     370,   370,   129,   100,   125,   215,   419,    97,   128,   409,
      99,    99,   125,   215,   211,   412,   413,   211,    91,   210,
     211,   216,   377,   403,   419,   169,   413,   169,    54,    64,
      65,   166,   136,   202,   156,   161,    97,   409,    99,   164,
     186,   103,   411,   418,   413,   222,   413,   137,   125,   140,
     416,   125,   416,   127,   416,   412,    56,   188,   190,   361,
     371,   125,    97,   128,   409,   272,    66,   119,   121,   122,
     348,   119,   119,   348,    67,   348,   337,   343,   340,   344,
      77,   138,   158,   176,   176,   176,   176,   165,   169,   169,
      52,    54,    55,    56,    57,    58,    77,    91,   101,   107,
     108,   109,   131,   143,   258,   318,   374,   376,   377,   378,
     379,   380,   381,   382,   383,   384,   387,   388,   389,   390,
     391,   394,   395,   396,   397,   398,   129,   160,   161,   376,
     129,   160,   283,   106,   181,   284,   279,   107,   179,   205,
     216,   217,   218,   186,   125,   191,   125,   167,   168,   179,
     192,   201,   205,   207,   218,   229,   367,   171,   201,   201,
     201,   201,   201,   201,   201,   201,   201,   201,   201,   201,
     201,   170,   201,   201,   201,   201,   201,   201,   201,   201,
     201,   201,   201,    52,    53,    56,   199,   208,   405,   406,
     407,   210,   216,    52,    53,    56,   199,   208,   406,   160,
      13,   249,   417,   249,   161,   176,   161,   411,   225,    56,
      97,   128,   409,    25,   169,    52,    56,   193,   133,   368,
      97,   128,   409,   228,   401,    68,    97,   408,    52,    56,
     406,   168,   201,   207,   168,   207,   198,   124,   129,   129,
     205,   107,   205,   214,   403,    52,    56,   210,    52,    56,
     404,   413,   103,   413,   125,   413,   125,   413,   196,   223,
     201,   127,   127,   406,   406,   207,   159,   413,   163,   413,
     403,   125,   190,    52,    56,   210,    52,    56,   136,   274,
     376,   350,   349,   119,   338,   348,    66,   119,   119,   338,
      66,   119,   201,   101,   107,   254,   255,   256,   257,   379,
     125,   399,   419,   413,   259,   260,   125,   375,   205,   125,
     399,    34,    52,   125,   375,    52,   125,   375,    52,   184,
     201,    10,   247,     8,   242,   326,   419,   417,   184,   201,
     247,   285,   286,   182,   287,   288,   287,   205,   125,    44,
     413,   190,   125,    44,   129,    44,    97,   128,   409,    52,
      56,    58,    90,    91,    98,   101,   104,   105,   107,   112,
     135,   270,   297,   298,   299,   300,   303,   308,   309,   310,
     313,   314,   315,   316,   317,   318,   319,   320,   321,   322,
     323,   324,   325,   330,   331,   334,   335,   336,   339,   341,
     342,   364,   388,   297,   412,    99,    99,   208,   212,   412,
     414,   125,    99,    99,   208,   209,   212,   419,   247,   161,
      13,   161,   247,    27,   250,   417,   247,    25,   224,   292,
      17,   244,   290,    52,    56,   210,    52,    56,   177,   227,
     369,   226,    52,    56,   193,   210,   160,   169,   175,   209,
     212,   196,   205,   205,   215,    99,    99,   414,    99,    99,
     377,   403,   169,   416,   188,   414,   376,   273,   351,    54,
      55,    57,   355,   367,   139,   348,   139,   139,   139,   256,
     379,   125,   413,   125,   398,   205,   129,   374,   381,   394,
     396,   384,   388,   390,   382,   391,   396,   380,   382,    44,
      44,   205,   218,   327,   419,     9,    15,   243,   245,   329,
     419,    44,    44,   127,   130,   265,   266,   419,   285,   247,
     265,   247,   107,   205,   165,   190,   165,   201,    52,    56,
     210,    52,    56,   321,   321,    56,   193,   305,   298,   306,
     307,   308,   309,   312,   414,   304,   412,   415,    52,   345,
      52,    54,    55,    57,   101,   362,   100,   125,   130,   125,
     125,   298,    88,    89,    97,   128,   136,   301,   302,    52,
     150,   168,   207,   168,   207,   181,   137,    99,   168,   207,
     168,   207,   181,    14,   245,   246,   251,   252,   419,   252,
     293,   290,   247,   107,   205,   289,   247,   414,   161,   417,
     176,   160,   414,   247,   413,   172,   282,   278,    99,   413,
     125,   413,   267,   412,    29,   123,   275,   352,   413,   254,
     257,   255,   125,   375,   125,   375,   399,   125,   375,   125,
     375,   375,   201,   201,   100,   328,   419,   161,   161,   201,
     201,   258,   261,   264,   267,   380,   382,   383,   385,   386,
     392,   393,   396,   398,   161,   127,   160,   205,   414,   298,
     414,   298,   310,   312,   414,   125,   112,   313,   127,   124,
     176,   322,   306,   310,   303,   311,   312,   315,   319,   321,
     321,   193,   414,   413,   306,   309,   313,   306,   309,   313,
     201,   168,   207,   161,   176,   247,   247,   296,   297,   247,
     205,   125,   249,   247,   160,   417,   247,   103,   138,   277,
     276,   353,   125,   125,   382,   396,   382,   382,    98,   192,
     229,   363,   364,   367,   249,   329,   399,   267,   130,   259,
     125,   262,   263,    98,   229,   125,   399,   125,   262,   125,
     262,   413,   125,   125,   345,   415,   103,   125,   125,   413,
     413,   413,   414,   414,   414,   249,   249,    40,    41,   107,
     205,   161,   247,    52,   268,   269,   378,   160,   161,   354,
     255,   375,   125,   375,   375,   375,    56,    97,   128,   409,
     161,   130,   229,   261,   393,   396,    56,    97,   385,   390,
     382,   392,   396,   382,   311,   311,   310,   312,   161,   294,
     176,   176,   205,   252,   290,   291,   125,   412,   247,   127,
     161,   382,    52,    56,   210,    52,    56,   326,   125,   262,
     125,   262,    52,    56,   399,   125,   262,   125,   262,   262,
     125,   251,   161,   269,   120,   375,   414,   382,   396,   382,
     382,   252,   292,   295,   262,   125,   262,   262,   262,   382,
     262 ];

/* YYR1[RULE-NUM] -- Symbol kind of the left-hand side of rule RULE-NUM.  */
  #[allow(non_upper_case_globals)]
const yyr1_: &'static [i32] = &[      0,   153,   155,   154,   156,   157,   157,   157,   157,   158,
     158,   159,   160,   160,   161,   162,   162,   162,   162,   163,
     164,   163,   166,   165,   165,   165,   165,   165,   165,   165,
     165,   165,   165,   165,   165,   165,   165,   165,   165,   165,
     167,   167,   167,   167,   167,   167,   167,   167,   167,   167,
     167,   167,   168,   168,   168,   169,   169,   169,   169,   169,
     170,   169,   171,   169,   169,   172,   173,   175,   174,   176,
     178,   177,   179,   179,   180,   180,   182,   181,   183,   184,
     184,   184,   184,   184,   184,   184,   184,   184,   184,   184,
     185,   185,   186,   186,   187,   187,   187,   187,   187,   187,
     187,   187,   187,   187,   188,   188,   189,   189,   190,   190,
     191,   191,   191,   191,   191,   191,   191,   191,   191,   192,
     192,   192,   192,   192,   192,   192,   192,   192,   193,   193,
     194,   194,   194,   195,   195,   195,   195,   195,   196,   196,
     197,   198,   197,   199,   199,   199,   199,   199,   199,   199,
     199,   199,   199,   199,   199,   199,   199,   199,   199,   199,
     199,   199,   199,   199,   199,   199,   199,   199,   199,   199,
     199,   199,   199,   200,   200,   200,   200,   200,   200,   200,
     200,   200,   200,   200,   200,   200,   200,   200,   200,   200,
     200,   200,   200,   200,   200,   200,   200,   200,   200,   200,
     200,   200,   200,   200,   200,   200,   200,   200,   200,   200,
     200,   200,   200,   200,   201,   201,   201,   201,   201,   201,
     201,   201,   201,   201,   201,   201,   201,   201,   201,   201,
     201,   201,   201,   201,   201,   201,   201,   201,   201,   201,
     201,   201,   201,   201,   201,   201,   201,   201,   201,   201,
     201,   201,   201,   201,   202,   201,   201,   201,   201,   201,
     201,   201,   203,   203,   203,   203,   204,   204,   205,   206,
     206,   206,   206,   207,   207,   208,   208,   208,   209,   209,
     210,   210,   210,   210,   210,   211,   211,   211,   211,   211,
     213,   212,   214,   214,   215,   215,   216,   216,   216,   216,
     217,   217,   218,   218,   218,   219,   219,   219,   219,   219,
     219,   219,   219,   219,   219,   219,   220,   219,   221,   219,
     222,   219,   219,   219,   219,   219,   219,   219,   219,   219,
     219,   223,   219,   219,   219,   219,   219,   219,   219,   219,
     219,   219,   219,   224,   219,   225,   219,   219,   219,   226,
     219,   227,   219,   228,   219,   219,   219,   219,   219,   219,
     219,   229,   230,   231,   232,   233,   234,   235,   236,   237,
     238,   239,   240,   241,   242,   243,   244,   245,   246,   247,
     248,   249,   249,   249,   250,   250,   251,   251,   252,   252,
     253,   253,   254,   254,   255,   255,   256,   256,   256,   256,
     256,   257,   257,   258,   258,   260,   259,   261,   261,   261,
     261,   262,   262,   263,   264,   264,   264,   264,   264,   264,
     264,   264,   264,   264,   264,   264,   264,   264,   264,   265,
     265,   266,   266,   267,   267,   268,   268,   269,   269,   271,
     272,   273,   270,   274,   274,   276,   275,   277,   275,   279,
     278,   280,   280,   280,   280,   281,   281,   281,   281,   281,
     281,   281,   281,   281,   283,   282,   284,   282,   286,   285,
     288,   287,   289,   289,   289,   289,   290,   291,   291,   293,
     294,   292,   295,   295,   296,   296,   296,   297,   297,   297,
     297,   297,   297,   298,   299,   299,   300,   300,   301,   302,
     303,   303,   303,   303,   303,   303,   303,   303,   303,   303,
     303,   303,   303,   304,   303,   303,   305,   303,   306,   306,
     306,   306,   306,   306,   307,   307,   308,   308,   309,   310,
     310,   311,   311,   312,   313,   313,   313,   313,   314,   314,
     315,   315,   316,   316,   317,   317,   318,   319,   319,   320,
     320,   320,   320,   320,   320,   320,   320,   320,   320,   321,
     321,   321,   321,   321,   321,   321,   321,   321,   321,   322,
     323,   323,   324,   325,   325,   325,   326,   326,   327,   327,
     327,   328,   328,   329,   329,   330,   330,   331,   332,   332,
     332,   333,   334,   335,   336,   337,   337,   338,   338,   339,
     340,   340,   341,   342,   343,   343,   344,   344,   345,   345,
     346,   346,   347,   347,   348,   349,   348,   350,   351,   352,
     353,   354,   348,   355,   355,   355,   355,   356,   356,   357,
     358,   358,   358,   358,   359,   360,   360,   361,   361,   361,
     361,   362,   362,   362,   363,   363,   363,   363,   363,   364,
     364,   364,   364,   364,   364,   364,   365,   365,   366,   366,
     367,   367,   369,   368,   368,   370,   370,   371,   372,   373,
     372,   374,   374,   374,   374,   374,   375,   375,   376,   376,
     376,   376,   376,   376,   376,   376,   376,   376,   376,   376,
     376,   376,   376,   377,   378,   378,   378,   378,   379,   379,
     380,   381,   381,   382,   382,   383,   384,   384,   385,   385,
     386,   386,   387,   387,   388,   388,   389,   390,   390,   391,
     392,   393,   393,   394,   394,   395,   395,   396,   396,   397,
     397,   398,   398,   399,   399,   400,   401,   400,   402,   402,
     403,   403,   404,   404,   404,   404,   404,   405,   405,   405,
     406,   406,   407,   407,   407,   408,   408,   409,   409,   410,
     410,   411,   411,   412,   412,   413,   414,   415,   416,   416,
     416,   417,   417,   418,   418,   419 ];

/* YYR2[RULE-NUM] -- Number of symbols on the right-hand side of rule RULE-NUM.  */
  #[allow(non_upper_case_globals)]
const yyr2_: &'static [i32] = &[      0,     2,     0,     2,     2,     1,     1,     3,     2,     1,
       2,     3,     5,     3,     2,     1,     1,     3,     1,     1,
       0,     3,     0,     4,     3,     3,     3,     2,     3,     3,
       3,     3,     3,     4,     1,     3,     3,     5,     3,     1,
       3,     3,     6,     5,     5,     5,     5,     4,     6,     4,
       6,     3,     1,     3,     1,     1,     3,     3,     3,     2,
       0,     4,     0,     4,     1,     1,     2,     0,     5,     1,
       0,     3,     1,     1,     1,     4,     0,     4,     1,     2,
       3,     4,     5,     4,     5,     2,     2,     2,     2,     2,
       1,     3,     1,     3,     1,     2,     3,     5,     2,     4,
       2,     4,     1,     3,     1,     3,     2,     3,     1,     3,
       1,     1,     4,     3,     3,     3,     3,     2,     1,     1,
       1,     4,     3,     3,     3,     3,     2,     1,     1,     1,
       2,     1,     3,     1,     1,     1,     1,     1,     1,     1,
       1,     0,     4,     1,     1,     1,     1,     1,     1,     1,
       1,     1,     1,     1,     1,     1,     1,     1,     1,     1,
       1,     1,     1,     1,     1,     1,     1,     1,     1,     1,
       1,     1,     1,     1,     1,     1,     1,     1,     1,     1,
       1,     1,     1,     1,     1,     1,     1,     1,     1,     1,
       1,     1,     1,     1,     1,     1,     1,     1,     1,     1,
       1,     1,     1,     1,     1,     1,     1,     1,     1,     1,
       1,     1,     1,     1,     3,     3,     6,     5,     5,     5,
       5,     4,     3,     3,     3,     2,     2,     2,     2,     3,
       3,     3,     3,     3,     3,     4,     2,     2,     3,     3,
       3,     3,     1,     3,     3,     3,     3,     3,     2,     2,
       3,     3,     3,     3,     0,     4,     6,     4,     6,     4,
       6,     1,     1,     1,     1,     1,     3,     3,     1,     1,
       2,     4,     2,     1,     3,     3,     5,     3,     1,     1,
       1,     1,     2,     4,     2,     1,     2,     2,     4,     1,
       0,     2,     2,     1,     2,     1,     1,     2,     3,     4,
       1,     1,     3,     4,     2,     1,     1,     1,     1,     1,
       1,     1,     1,     1,     1,     1,     0,     4,     0,     3,
       0,     4,     3,     3,     2,     3,     3,     1,     4,     3,
       1,     0,     6,     4,     3,     2,     1,     2,     1,     6,
       6,     4,     4,     0,     6,     0,     5,     5,     6,     0,
       6,     0,     7,     0,     5,     4,     4,     1,     1,     1,
       1,     1,     1,     1,     1,     1,     1,     1,     1,     1,
       1,     1,     1,     1,     1,     1,     1,     1,     1,     1,
       1,     1,     1,     2,     1,     1,     1,     5,     1,     2,
       1,     1,     1,     3,     1,     3,     1,     3,     5,     1,
       3,     2,     1,     1,     1,     0,     2,     4,     2,     2,
       1,     2,     0,     1,     6,     8,     4,     6,     4,     2,
       6,     2,     4,     6,     2,     4,     2,     4,     1,     1,
       1,     3,     4,     1,     4,     1,     3,     1,     1,     0,
       0,     0,     6,     4,     1,     0,     4,     0,     4,     0,
       4,     2,     4,     5,     5,     2,     4,     4,     3,     3,
       3,     2,     1,     4,     0,     4,     0,     4,     0,     3,
       0,     3,     1,     2,     3,     4,     5,     1,     1,     0,
       0,     7,     1,     1,     1,     3,     3,     1,     2,     3,
       1,     1,     1,     1,     3,     1,     3,     1,     1,     1,
       1,     1,     4,     4,     4,     3,     4,     4,     4,     3,
       3,     3,     2,     0,     4,     2,     0,     4,     1,     1,
       2,     2,     4,     1,     2,     3,     1,     3,     5,     2,
       1,     1,     3,     1,     3,     1,     2,     1,     1,     3,
       2,     1,     1,     3,     2,     1,     2,     1,     1,     1,
       3,     3,     2,     2,     1,     1,     1,     2,     2,     1,
       1,     1,     1,     1,     1,     1,     1,     1,     1,     1,
       2,     2,     4,     2,     3,     1,     6,     1,     1,     1,
       1,     2,     1,     2,     1,     1,     1,     1,     1,     1,
       2,     3,     3,     3,     4,     0,     3,     1,     2,     4,
       0,     3,     4,     4,     0,     3,     0,     3,     0,     2,
       0,     2,     0,     2,     1,     0,     3,     0,     0,     0,
       0,     0,     8,     1,     1,     1,     1,     1,     1,     2,
       1,     1,     1,     1,     3,     1,     2,     1,     1,     1,
       1,     1,     1,     1,     1,     1,     1,     1,     1,     1,
       1,     1,     1,     1,     1,     1,     1,     1,     1,     1,
       1,     1,     0,     4,     0,     1,     1,     3,     1,     0,
       3,     4,     2,     2,     1,     1,     2,     0,     6,     8,
       4,     6,     4,     6,     2,     4,     6,     2,     4,     2,
       4,     1,     0,     1,     1,     1,     1,     1,     1,     1,
       1,     1,     3,     1,     3,     1,     2,     1,     2,     1,
       1,     3,     1,     3,     1,     1,     1,     2,     1,     3,
       3,     1,     3,     1,     3,     1,     1,     2,     1,     1,
       1,     2,     1,     2,     1,     1,     0,     4,     1,     2,
       1,     3,     3,     2,     1,     4,     2,     1,     1,     1,
       1,     1,     1,     1,     1,     1,     1,     1,     1,     1,
       1,     0,     1,     0,     1,     2,     2,     2,     0,     1,
       1,     1,     1,     1,     2,     0 ];


/* YYRLINE[YYN] -- Source line where rule number YYN was defined.  */
  #[allow(non_upper_case_globals)]
const yyrline_: &'static [i32] = &[      0,   346,   346,   346,   364,   373,   377,   381,   387,   393,
     397,   406,   418,   441,   460,   469,   473,   477,   483,   489,
     494,   493,   504,   503,   514,   524,   534,   538,   547,   558,
     569,   580,   591,   611,   626,   630,   643,   660,   689,   699,
     705,   715,   725,   740,   757,   774,   791,   808,   828,   864,
     886,   924,   936,   942,   965,   971,   975,   986,   997,  1008,
    1020,  1019,  1048,  1047,  1075,  1081,  1099,  1111,  1110,  1131,
    1139,  1139,  1157,  1161,  1167,  1171,  1187,  1186,  1206,  1212,
    1225,  1247,  1260,  1282,  1295,  1317,  1329,  1341,  1353,  1365,
    1379,  1389,  1401,  1411,  1433,  1437,  1443,  1450,  1462,  1469,
    1481,  1494,  1505,  1518,  1531,  1535,  1547,  1551,  1559,  1563,
    1571,  1577,  1583,  1594,  1609,  1619,  1634,  1646,  1657,  1667,
    1673,  1679,  1690,  1700,  1710,  1720,  1732,  1743,  1753,  1757,
    1763,  1769,  1775,  1787,  1791,  1795,  1799,  1804,  1810,  1816,
    1822,  1827,  1826,  1839,  1840,  1841,  1842,  1843,  1844,  1845,
    1846,  1847,  1848,  1849,  1850,  1851,  1852,  1853,  1854,  1855,
    1856,  1857,  1858,  1859,  1860,  1861,  1862,  1863,  1864,  1865,
    1866,  1867,  1868,  1871,  1872,  1873,  1874,  1875,  1876,  1877,
    1878,  1879,  1880,  1881,  1882,  1883,  1884,  1885,  1886,  1887,
    1888,  1889,  1890,  1891,  1892,  1893,  1894,  1895,  1896,  1897,
    1898,  1899,  1900,  1901,  1902,  1903,  1904,  1905,  1906,  1907,
    1908,  1909,  1910,  1911,  1914,  1924,  1934,  1949,  1966,  1983,
    2000,  2017,  2033,  2043,  2059,  2075,  2088,  2101,  2114,  2127,
    2133,  2139,  2145,  2151,  2157,  2163,  2176,  2185,  2194,  2200,
    2206,  2212,  2218,  2222,  2228,  2234,  2240,  2246,  2256,  2267,
    2276,  2282,  2288,  2299,  2311,  2310,  2328,  2343,  2363,  2399,
    2421,  2459,  2465,  2469,  2473,  2477,  2483,  2493,  2510,  2518,
    2522,  2526,  2538,  2554,  2560,  2585,  2595,  2616,  2632,  2642,
    2655,  2659,  2663,  2667,  2674,  2690,  2696,  2703,  2714,  2726,
    2732,  2732,  2760,  2769,  2784,  2788,  2794,  2798,  2811,  2817,
    2826,  2832,  2838,  2844,  2852,  2867,  2871,  2875,  2879,  2883,
    2887,  2891,  2895,  2899,  2903,  2907,  2921,  2920,  2934,  2934,
    2944,  2944,  2954,  2964,  2974,  2980,  2990,  3000,  3012,  3024,
    3036,  3049,  3048,  3066,  3077,  3088,  3110,  3114,  3127,  3131,
    3150,  3169,  3185,  3202,  3201,  3225,  3224,  3247,  3265,  3283,
    3282,  3313,  3312,  3339,  3338,  3364,  3383,  3404,  3416,  3428,
    3440,  3454,  3462,  3468,  3475,  3481,  3487,  3493,  3499,  3505,
    3518,  3531,  3538,  3544,  3550,  3556,  3562,  3568,  3574,  3581,
    3587,  3596,  3600,  3604,  3610,  3614,  3620,  3625,  3652,  3656,
    3664,  3668,  3674,  3680,  3692,  3696,  3704,  3708,  3714,  3726,
    3730,  3743,  3749,  3757,  3761,  3767,  3767,  3777,  3789,  3796,
    3803,  3810,  3815,  3820,  3826,  3840,  3856,  3868,  3882,  3894,
    3898,  3912,  3930,  3942,  3956,  3963,  3975,  3982,  3994,  4000,
    4006,  4013,  4027,  4047,  4051,  4057,  4061,  4069,  4077,  4084,
    4090,  4095,  4083,  4128,  4144,  4158,  4157,  4174,  4173,  4192,
    4191,  4211,  4224,  4238,  4261,  4285,  4300,  4315,  4330,  4343,
    4358,  4373,  4387,  4399,  4413,  4412,  4432,  4431,  4452,  4452,
    4477,  4477,  4501,  4505,  4518,  4524,  4533,  4548,  4552,  4560,
    4570,  4559,  4598,  4602,  4609,  4613,  4618,  4625,  4629,  4640,
    4652,  4658,  4664,  4672,  4678,  4688,  4694,  4704,  4710,  4717,
    4724,  4728,  4732,  4746,  4759,  4772,  4791,  4805,  4818,  4831,
    4850,  4862,  4872,  4884,  4883,  4901,  4912,  4911,  4929,  4938,
    4942,  4954,  4967,  4984,  4995,  5004,  5018,  5022,  5033,  5046,
    5052,  5060,  5064,  5072,  5078,  5085,  5089,  5093,  5099,  5103,
    5111,  5120,  5130,  5136,  5144,  5157,  5172,  5183,  5187,  5203,
    5207,  5223,  5239,  5252,  5265,  5269,  5273,  5277,  5290,  5305,
    5309,  5313,  5317,  5321,  5325,  5329,  5333,  5337,  5343,  5349,
    5359,  5376,  5388,  5404,  5410,  5420,  5426,  5454,  5460,  5464,
    5468,  5474,  5480,  5486,  5492,  5498,  5502,  5508,  5520,  5530,
    5534,  5542,  5556,  5570,  5585,  5598,  5602,  5612,  5616,  5624,
    5637,  5640,  5650,  5662,  5675,  5678,  5689,  5692,  5703,  5706,
    5715,  5718,  5727,  5730,  5738,  5745,  5744,  5755,  5760,  5763,
    5767,  5771,  5754,  5795,  5799,  5804,  5808,  5814,  5815,  5818,
    5827,  5828,  5829,  5830,  5833,  5842,  5846,  5857,  5863,  5869,
    5875,  5883,  5889,  5895,  5903,  5909,  5915,  5921,  5927,  5935,
    5941,  5947,  5953,  5959,  5965,  5971,  5979,  5985,  5993,  5999,
    6007,  6013,  6022,  6021,  6036,  6041,  6045,  6052,  6064,  6068,
    6068,  6086,  6098,  6105,  6112,  6120,  6132,  6137,  6142,  6156,
    6172,  6184,  6198,  6210,  6224,  6231,  6243,  6257,  6264,  6276,
    6283,  6295,  6300,  6305,  6311,  6315,  6319,  6323,  6329,  6333,
    6343,  6352,  6359,  6371,  6375,  6384,  6401,  6409,  6419,  6426,
    6435,  6439,  6448,  6452,  6460,  6464,  6470,  6486,  6498,  6510,
    6524,  6538,  6542,  6550,  6554,  6562,  6566,  6572,  6585,  6597,
    6601,  6607,  6618,  6630,  6634,  6640,  6646,  6646,  6678,  6682,
    6688,  6692,  6700,  6710,  6719,  6725,  6736,  6744,  6748,  6752,
    6758,  6762,  6768,  6772,  6776,  6782,  6786,  6792,  6796,  6802,
    6806,  6813,  6816,  6823,  6826,  6832,  6838,  6844,  6851,  6854,
    6858,  6864,  6868,  6874,  6878,  6885 ];


  // Report on the debug stream that the rule yyrule is going to be reduced.
  fn yy_reduce_print(&self, yyrule: i32, yystack: &YYStack) {
        if !( self.is_debug() ) {
            return;
        }

        let yylno = Self::yyrline_[i32_to_usize(yyrule)];
        let yynrhs = Self::yyr2_[i32_to_usize(yyrule)];
        // Print the symbols being reduced, and their result.
        self.yycdebug(&format!("Reducing stack by rule {} (line {}):", /* " fix */ yyrule - 1, yylno));

        // The symbols being reduced.
        for yyi in 0..yynrhs {
            let state: usize = i32_to_usize(yystack.state_at(i32_to_usize(yynrhs - (yyi + 1))));
            self.yy_symbol_print(
                &format!("   ${} =", yyi + 1),
                SymbolKind::get(Self::yystos_[state]),
                yystack.borrow_value_at(i32_to_usize(yynrhs - (yyi + 1))),
                yystack.location_at(i32_to_usize(yynrhs - (yyi + 1)))
            );
        }
  }

  /* YYTRANSLATE_(TOKEN-NUM) -- Symbol number corresponding to TOKEN-NUM
     as returned by yylex, with out-of-bounds checking.  */
  fn yytranslate_(t: i32) -> &'static SymbolKind
  {
        // Last valid token kind.
        let code_max: i32 = 407;
        if t <= 0 {
            SymbolKind::get(0)
        } else if t <= code_max {
            let t = i32_to_usize(t);
            SymbolKind::get(Self::yytranslate_table_[t])
        } else {
            SymbolKind::get(2)
        }
  }
  #[allow(non_upper_case_globals)]
const yytranslate_table_: &'static [i32] = &[      0,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     1,     2,     3,     4,
       5,     6,     7,     8,     9,    10,    11,    12,    13,    14,
      15,    16,    17,    18,    19,    20,    21,    22,    23,    24,
      25,    26,    27,    28,    29,    30,    31,    32,    33,    34,
      35,    36,    37,    38,    39,    40,    41,    42,    43,    44,
      45,    46,    47,    48,    49,    50,    51,    52,    53,    54,
      55,    56,    57,    58,    59,    60,    61,    62,    63,    64,
      65,    66,    67,    68,    69,    70,    71,    72,    73,    74,
      75,    76,    77,    78,    79,    80,    81,    82,    83,    84,
      85,    86,    87,    88,    89,    90,    91,    92,    93,    94,
      95,    96,    97,    98,    99,   100,   101,   102,   103,   104,
     105,   106,   107,   108,   109,   110,   111,   112,   113,   114,
     115,   116,   117,   118,   119,   120,   121,   122,   123,   124,
     125,   126,   127,   128,   129,   130,   131,   132,   133,   134,
     135,   136,   137,   138,   139,   140,   141,   142,   143,   144,
     145,   146,   147,   148,   149,   150,   151,   152 ];


const YYLAST_: i32 = 14606;
const YYEMPTY_: i32 = -2;
const YYFINAL_: i32 = 3;
const YYNTOKENS_: i32 = 153;


}

/* Unqualified %code blocks.  */
/* "src/parser/parse.y":73  */

    // pre-code

/* "src/parser/parse.rs":15390  */


/* "src/parser/parse.y":6890  */


impl Parser {
    /// Constructs a parser with given `input` and `options`.
    ///
    /// Returns an error if given `input` is invalid.
    pub fn new<TInput>(input: TInput, options: ParserOptions) -> Self
    where
        TInput: Into<Vec<u8>>
    {
        let ParserOptions {
            buffer_name,
            decoder,
            token_rewriter,
            record_tokens,
        } = options;

        let context = ParserContext::new();
        let current_arg_stack = CurrentArgStack::new();
        let max_numparam_stack = MaxNumparamStack::new();
        let pattern_variables = VariablesStack::new();
        let pattern_hash_keys = VariablesStack::new();
        let static_env = StaticEnvironment::new();
        let diagnostics = Diagnostics::new();
        let tokens_pool = Pool::new();

        let input: Vec<u8> = input.into();
        let buffer_name: String = buffer_name;

        let mut lexer = Lexer::new(input, buffer_name, decoder);
        lexer.context = context.clone();
        lexer.static_env = static_env.clone();
        lexer.diagnostics = diagnostics.clone();
        lexer.tokens_factory = tokens_pool.factory();

        let builder = Builder::new(
            static_env.clone(),
            context.clone(),
            current_arg_stack.clone(),
            max_numparam_stack.clone(),
            pattern_variables.clone(),
            pattern_hash_keys.clone(),
            diagnostics.clone(),
            tokens_pool.factory(),
        );

        let last_token_type = 0;

        Self {
            yy_error_verbose: true,
            yynerrs: 0,
            yyerrstatus_: 0,
            result: None,

            builder,
            context,
            current_arg_stack,
            max_numparam_stack,
            pattern_variables,
            pattern_hash_keys,
            static_env,
            last_token_type,
            tokens: vec![],
            diagnostics,
            yylexer: lexer,
            token_rewriter,
            record_tokens,
            tokens_pool,
        }
    }

    /// Parses given input and returns:
    ///
    /// 1. AST
    /// 2. tokens
    /// 3. diagnostics
    /// 4. coments
    /// 5. magic comments
    pub fn do_parse(mut self) -> ParserResult  {
        self.parse();

        ParserResult {
            ast: self.result,
            tokens: self.tokens,
            diagnostics: self.diagnostics.take_inner(),
            comments: self.yylexer.comments,
            magic_comments: self.yylexer.magic_comments,
            input: self.yylexer.buffer.input.decoded,
        }
    }

    #[doc(hidden)]
    pub fn do_parse_with_state_validation(mut self) -> ParserResult {
        self.parse();

        self.assert_state_is_final();

        ParserResult {
            ast: self.result,
            tokens: self.tokens,
            diagnostics: self.diagnostics.take_inner(),
            comments: self.yylexer.comments,
            magic_comments: self.yylexer.magic_comments,
            input: self.yylexer.buffer.input.decoded,
        }
    }

    fn warn(&mut self, loc: &Loc, message: DiagnosticMessage) {
        let diagnostic = Diagnostic {
            level: ErrorLevel::Warning,
            message,
            loc: *loc,
        };
        self.diagnostics.emit(diagnostic);
    }

    fn yylex(&mut self) -> PoolValue<Token> {
        self.yylexer.yylex()
    }

    fn next_token(&mut self) -> PoolValue<Token> {
        let mut token = self.yylex();

        if let Some(token_rewriter) = self.token_rewriter.as_ref() {
            let boxed_token = token.take_boxed_value();
            let TokenRewriterResult { rewritten_token, token_action, lex_state_action } =
                token_rewriter.call(boxed_token, self.yylexer.buffer.input.as_shared_bytes());

            match lex_state_action {
                LexStateAction::Keep => {
                    // keep
                }
                LexStateAction::Set(next_state) => {
                    self.yylexer.lex_state.set(next_state);
                }
            }

            match token_action {
                RewriteAction::Drop => {
                    return self.next_token();
                }
                RewriteAction::Keep => {
                    token = self.tokens_pool.alloc(*rewritten_token);
                }
            }
        }

        self.last_token_type = token.token_type;

        if self.record_tokens {
            let token = token.clone();
            self.tokens.push(token.take_value());
        }

        token
    }

    fn check_kwarg_name(&self, ident_t: &Token) -> Result<(), ()> {
        let name = clone_value(ident_t);
        let first_char = name.as_str().chars().next().expect("kwarg name can't be empty");
        if first_char.is_lowercase() || first_char == '_' {
            Ok(())
        } else {
            let loc = ident_t.loc;
            self.diagnostics.emit(
                Diagnostic {
                    level: ErrorLevel::Error,
                    message: DiagnosticMessage::ConstArgument {},
                    loc
                }
            );
            Err(())
        }
    }

    fn validate_endless_method_name(&mut self, name_t: &Token) -> Result<(), ()> {
        let name = clone_value(name_t);
        match &name[..] {
            "==" | "===" | ">=" | "<=" | "!=" => Ok(()),
            other if other.ends_with('=') => {
                self.yyerror(&name_t.loc, DiagnosticMessage::EndlessSetterDefinition {}).map(|_| ())
            }
            _ => Ok(())
        }
    }

    fn yyerror(&mut self, loc: &Loc, message: DiagnosticMessage) -> Result<i32, ()> {
        self.yyerror1(
            message,
            *loc
        )
    }

    fn yyerror1(&mut self, message: DiagnosticMessage, loc: Loc) -> Result<i32, ()> {
        let diagnostic = Diagnostic { level: ErrorLevel::Error, message, loc };
        self.diagnostics.emit(diagnostic);
        Err(())
    }

    fn report_syntax_error(&mut self, _stack: &YYStack, yytoken: &SymbolKind, loc: YYLoc) {
        let id: usize = yytoken.code().try_into().expect("failed to convert token code into i32, is it too big?");
        let diagnostic = Diagnostic {
            level: ErrorLevel::Error,
            message: DiagnosticMessage::UnexpectedToken {
                token_name: Lexer::TOKEN_NAMES[id].to_string()
            },
            loc,
        };
        self.diagnostics.emit(diagnostic);
    }

    fn warn_eol(&mut self, loc: &Loc, tok: &str) {
        if self.yylexer.buffer.is_looking_at_eol() {
            self.warn(loc, DiagnosticMessage::TokAtEolWithoutExpression { token_name: tok.to_string() });
        }
    }

    fn value_expr(&self, node: &Node) -> Result<(), ()> {
        self.builder.value_expr(node)
    }

    #[doc(hidden)]
    fn assert_state_is_final(&self) {
        assert!(self.yylexer.cmdarg.is_empty());
        assert!(self.yylexer.cond.is_empty());
        assert!(self.yylexer.paren_nest == 0);

        assert!(self.static_env.is_empty());
        assert!(self.context.is_empty());
        assert!(self.max_numparam_stack.is_empty());
        assert!(self.current_arg_stack.is_empty());
        assert!(self.pattern_variables.is_empty());
        assert!(self.pattern_hash_keys.is_empty());
    }

    #[inline]
    fn is_debug(&self) -> bool {
        cfg!(feature = "debug-parser")
    }

    fn local_push(&mut self) {
        self.static_env.extend_static();
        self.yylexer.cmdarg.push(false);
        self.yylexer.cond.push(false);
        self.max_numparam_stack.push(true);
    }

    fn local_pop(&mut self) {
        self.static_env.unextend();
        self.yylexer.cmdarg.pop();
        self.yylexer.cond.pop();
        self.max_numparam_stack.pop();
    }
}
