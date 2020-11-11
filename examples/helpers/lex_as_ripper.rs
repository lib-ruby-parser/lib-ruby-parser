use lib_ruby_parser::token_name;
use std::fs;

use super::*;

#[allow(dead_code)]
pub fn lex_as_ripper(filepath: &str) -> Result<String, String> {
    let source = fs::read(filepath).map_err(|_| "failed to read a file".to_owned())?;
    let (parser, mut tokens) = lex(&source, filepath, false)?;
    tokens.sort_by(|a, b| a.loc.begin.cmp(&b.loc.begin));

    let mut output = String::from("");
    for token in tokens {
        if token_name(token.token_type) == "EOF" {
            continue;
        }
        let token_name = token_name(token.token_type);
        let bytes = token.to_bytes();

        let token_name = match &token_name[..] {
            "tNL" | "tSPACE" | "tSP" => continue,
            "tLPAREN2" => "tLPAREN",
            "tLCURLY" => "tLBRACE",
            "tRCURLY" => "tRBRACE",
            "tLBRACK2" => "tLBRACK",
            "kDO_BLOCK" => "kDO",
            "kDO_COND" => "kDO",
            "kDO_LAMBDA" => "kDO",
            "kIF_MOD" => "kIF",
            "kUNLESS_MOD" => "kUNLESS",
            "kWHILE_MOD" => "kWHILE",
            "kRESCUE_MOD" => "kRESCUE",
            "kUNTIL_MOD" => "kUNTIL",
            "tUMINUS_NUM" => "tMINUS",
            "tFID" => "tIDENTIFIER",
            "tAMPER2" => "tAMPER",
            "tSTAR2" => "tSTAR",
            "tPOW" => "tDSTAR",
            "tUMINUS" => "tMINUS",
            "tCOLON3" => "tCOLON2",
            "tNTH_REF" => "tBACK_REF",
            "tLPAREN_ARG" => "tLPAREN",
            "tLBRACE_ARG" => "tLBRACE",
            "tUPLUS" => "tPLUS",
            "tXSTRING_BEG" | "tBACK_REF2" => "tBACKTICK",
            other => other,
        }
        .to_owned();

        let (line, col) = parser
            .yylexer
            .line_col_for_loc(token.loc.begin)
            .ok_or_else(|| format!("token {:#?} has invalid range", token))?;

        output.push_str(&format!(
            "{} {:?} {}:{}\n",
            token_name,
            bytes,
            line + 1,
            col
        ));
    }
    Ok(output)
}
