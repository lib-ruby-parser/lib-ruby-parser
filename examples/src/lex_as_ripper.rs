use lib_ruby_parser::{debug_level, token_name, DiagnosticMessage, ParserResult};
use std::fs;

use crate::parse;

pub fn lex_as_ripper(filepath: &str) -> Result<String, String> {
    let source = fs::read(filepath).map_err(|_| "failed to read a file".to_string())?;
    let ParserResult {
        mut tokens,
        input,
        diagnostics,
        ..
    } = parse(&source, filepath, debug_level::NONE, false);

    let mut encoding_error: Option<String> = None;
    for diagnostic in diagnostics.iter() {
        if let DiagnosticMessage::EncodingError { error } = &diagnostic.message {
            encoding_error = Some(error.clone())
        }
    }
    if let Some(encoding_error) = encoding_error {
        return Err(encoding_error);
    }

    tokens.sort_by(|a, b| a.loc.begin.cmp(&b.loc.begin));

    let mut output = String::from("");
    for token in tokens.iter() {
        if token_name(token.token_type) == "EOF" {
            continue;
        }
        let token_name = token_name(token.token_type);
        let bytes = token.token_value.as_bytes();

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
        };

        let (line, col) = input
            .line_col_for_pos(token.loc.begin)
            .ok_or_else(|| format!("token {:#?} has invalid loc", token))?;

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
