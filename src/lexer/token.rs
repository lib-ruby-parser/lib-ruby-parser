#[derive(Clone, Eq, PartialEq)]
pub enum Token { // enum yytokentype
    END_OF_INPUT(String, usize, usize),
    kCLASS(String, usize, usize),
    kMODULE(String, usize, usize),
    kDEF(String, usize, usize),
    kUNDEF(String, usize, usize),
    kBEGIN(String, usize, usize),
    kRESCUE(String, usize, usize),
    kENSURE(String, usize, usize),
    kEND(String, usize, usize),
    kIF(String, usize, usize),
    kUNLESS(String, usize, usize),
    kTHEN(String, usize, usize),
    kELSIF(String, usize, usize),
    kELSE(String, usize, usize),
    kCASE(String, usize, usize),
    kWHEN(String, usize, usize),
    kWHILE(String, usize, usize),
    kUNTIL(String, usize, usize),
    kFOR(String, usize, usize),
    kBREAK(String, usize, usize),
    kNEXT(String, usize, usize),
    kREDO(String, usize, usize),
    kRETRY(String, usize, usize),
    kIN(String, usize, usize),
    kDO(String, usize, usize),
    kDO_COND(String, usize, usize),
    kDO_BLOCK(String, usize, usize),
    kDO_LAMBDA(String, usize, usize),
    kRETURN(String, usize, usize),
    kYIELD(String, usize, usize),
    kSUPER(String, usize, usize),
    kSELF(String, usize, usize),
    kNIL(String, usize, usize),
    kTRUE(String, usize, usize),
    kFALSE(String, usize, usize),
    kAND(String, usize, usize),
    kOR(String, usize, usize),
    kNOT(String, usize, usize),
    kIF_MOD(String, usize, usize),
    kUNLESS_MOD(String, usize, usize),
    kWHILE_MOD(String, usize, usize),
    kUNTIL_MOD(String, usize, usize),
    kRESCUE_MOD(String, usize, usize),
    kALIAS(String, usize, usize),
    kDEFINED(String, usize, usize),
    klBEGIN(String, usize, usize),
    klEND(String, usize, usize),
    k__LINE__(String, usize, usize),
    k__FILE__(String, usize, usize),
    k__ENCODING__(String, usize, usize),
    tIDENTIFIER(String, usize, usize),
    tFID(String, usize, usize),
    tGVAR(String, usize, usize),
    tIVAR(String, usize, usize),
    tCONSTANT(String, usize, usize),
    tCVAR(String, usize, usize),
    tLABEL(String, usize, usize),
    tINTEGER(String, usize, usize),
    tFLOAT(String, usize, usize),
    tRATIONAL(String, usize, usize),
    tIMAGINARY(String, usize, usize),
    tCHAR(String, usize, usize),
    tNTH_REF(String, usize, usize),
    tBACK_REF(String, usize, usize),
    tSTRING_CONTENT(String, usize, usize),
    tREGEXP_END(String, usize, usize),
    tSP(String, usize, usize),
    tUPLUS(String, usize, usize),
    tUMINUS(String, usize, usize),
    tPOW(String, usize, usize),
    tCMP(String, usize, usize),
    tEQ(String, usize, usize),
    tEQQ(String, usize, usize),
    tNEQ(String, usize, usize),
    tGEQ(String, usize, usize),
    tLEQ(String, usize, usize),
    tANDOP(String, usize, usize),
    tOROP(String, usize, usize),
    tMATCH(String, usize, usize),
    tNMATCH(String, usize, usize),
    tDOT2(String, usize, usize),
    tDOT3(String, usize, usize),
    tBDOT2(String, usize, usize),
    tBDOT3(String, usize, usize),
    tAREF(String, usize, usize),
    tASET(String, usize, usize),
    tLSHFT(String, usize, usize),
    tRSHFT(String, usize, usize),
    tANDDOT(String, usize, usize),
    tCOLON2(String, usize, usize),
    tCOLON3(String, usize, usize),
    tOP_ASGN(String, usize, usize),
    tASSOC(String, usize, usize),
    tLPAREN(String, usize, usize),
    tLPAREN_ARG(String, usize, usize),
    tRPAREN(String, usize, usize),
    tLBRACK(String, usize, usize),
    tLBRACE(String, usize, usize),
    tLBRACE_ARG(String, usize, usize),
    tSTAR(String, usize, usize),
    tDSTAR(String, usize, usize),
    tAMPER(String, usize, usize),
    tLAMBDA(String, usize, usize),
    tSYMBEG(String, usize, usize),
    tSTRING_BEG(String, usize, usize),
    tXSTRING_BEG(String, usize, usize),
    tREGEXP_BEG(String, usize, usize),
    tWORDS_BEG(String, usize, usize),
    tQWORDS_BEG(String, usize, usize),
    tSYMBOLS_BEG(String, usize, usize),
    tQSYMBOLS_BEG(String, usize, usize),
    tSTRING_END(String, usize, usize),
    tSTRING_DEND(String, usize, usize),
    tSTRING_DBEG(String, usize, usize),
    tSTRING_DVAR(String, usize, usize),
    tLAMBEG(String, usize, usize),
    tLABEL_END(String, usize, usize),
    tLOWEST(String, usize, usize),
    tUMINUS_NUM(String, usize, usize),
    tLAST_TOKEN(String, usize, usize),

    tNL(String, usize, usize),
    tIGNORED_NL(String, usize, usize),

    tBANG(String, usize, usize),
    tLT(String, usize, usize),
    tGT(String, usize, usize),
    tAMPER2(String, usize, usize),
    tPIPE(String, usize, usize),
    tPLUS(String, usize, usize),
    tMINUS(String, usize, usize),
    tDOT(String, usize, usize),
    tRBRACK(String, usize, usize),
    tRCURLY(String, usize, usize),
    tCOLON(String, usize, usize),
    tDIVIDE(String, usize, usize),
    tCARET(String, usize, usize),
    tSEMI(String, usize, usize),
    tCOMMA(String, usize, usize),
    tTILDE(String, usize, usize),
    tLPAREN2(String, usize, usize),
    tLBRACK2(String, usize, usize),
    tLCURLY(String, usize, usize),

    // FIXME: these nodes are different in MRI and wq/parser
    tUNARY_NUM(String, usize, usize),
    tSYMBOL(String, usize, usize),
    tSTRING(String, usize, usize),
    tBACK_REF2(String, usize, usize),
    tEQL(String, usize, usize),
    tCHARACTER(String, usize, usize),
    tREGEXP_OPT(String, usize, usize),
    tSPACE(String, usize, usize),
    tEH(String, usize, usize),
    tPERCENT(String, usize, usize),
    tSTAR2(String, usize, usize),
}

impl Token {
    pub fn name(&self) -> String {
        match self {
            Self::END_OF_INPUT(..) => "END_OF_INPUT".to_owned(),
            Self::kCLASS(..) => "kCLASS".to_owned(),
            Self::kMODULE(..) => "kMODULE".to_owned(),
            Self::kDEF(..) => "kDEF".to_owned(),
            Self::kUNDEF(..) => "kUNDEF".to_owned(),
            Self::kBEGIN(..) => "kBEGIN".to_owned(),
            Self::kRESCUE(..) => "kRESCUE".to_owned(),
            Self::kENSURE(..) => "kENSURE".to_owned(),
            Self::kEND(..) => "kEND".to_owned(),
            Self::kIF(..) => "kIF".to_owned(),
            Self::kUNLESS(..) => "kUNLESS".to_owned(),
            Self::kTHEN(..) => "kTHEN".to_owned(),
            Self::kELSIF(..) => "kELSIF".to_owned(),
            Self::kELSE(..) => "kELSE".to_owned(),
            Self::kCASE(..) => "kCASE".to_owned(),
            Self::kWHEN(..) => "kWHEN".to_owned(),
            Self::kWHILE(..) => "kWHILE".to_owned(),
            Self::kUNTIL(..) => "kUNTIL".to_owned(),
            Self::kFOR(..) => "kFOR".to_owned(),
            Self::kBREAK(..) => "kBREAK".to_owned(),
            Self::kNEXT(..) => "kNEXT".to_owned(),
            Self::kREDO(..) => "kREDO".to_owned(),
            Self::kRETRY(..) => "kRETRY".to_owned(),
            Self::kIN(..) => "kIN".to_owned(),
            Self::kDO(..) => "kDO".to_owned(),
            Self::kDO_COND(..) => "kDO_COND".to_owned(),
            Self::kDO_BLOCK(..) => "kDO_BLOCK".to_owned(),
            Self::kDO_LAMBDA(..) => "kDO_LAMBDA".to_owned(),
            Self::kRETURN(..) => "kRETURN".to_owned(),
            Self::kYIELD(..) => "kYIELD".to_owned(),
            Self::kSUPER(..) => "kSUPER".to_owned(),
            Self::kSELF(..) => "kSELF".to_owned(),
            Self::kNIL(..) => "kNIL".to_owned(),
            Self::kTRUE(..) => "kTRUE".to_owned(),
            Self::kFALSE(..) => "kFALSE".to_owned(),
            Self::kAND(..) => "kAND".to_owned(),
            Self::kOR(..) => "kOR".to_owned(),
            Self::kNOT(..) => "kNOT".to_owned(),
            Self::kIF_MOD(..) => "kIF_MOD".to_owned(),
            Self::kUNLESS_MOD(..) => "kUNLESS_MOD".to_owned(),
            Self::kWHILE_MOD(..) => "kWHILE_MOD".to_owned(),
            Self::kUNTIL_MOD(..) => "kUNTIL_MOD".to_owned(),
            Self::kRESCUE_MOD(..) => "kRESCUE_MOD".to_owned(),
            Self::kALIAS(..) => "kALIAS".to_owned(),
            Self::kDEFINED(..) => "kDEFINED".to_owned(),
            Self::klBEGIN(..) => "klBEGIN".to_owned(),
            Self::klEND(..) => "klEND".to_owned(),
            Self::k__LINE__(..) => "k__LINE__".to_owned(),
            Self::k__FILE__(..) => "k__FILE__".to_owned(),
            Self::k__ENCODING__(..) => "k__ENCODING__".to_owned(),
            Self::tIDENTIFIER(..) => "tIDENTIFIER".to_owned(),
            Self::tFID(..) => "tFID".to_owned(),
            Self::tGVAR(..) => "tGVAR".to_owned(),
            Self::tIVAR(..) => "tIVAR".to_owned(),
            Self::tCONSTANT(..) => "tCONSTANT".to_owned(),
            Self::tCVAR(..) => "tCVAR".to_owned(),
            Self::tLABEL(..) => "tLABEL".to_owned(),
            Self::tINTEGER(..) => "tINTEGER".to_owned(),
            Self::tFLOAT(..) => "tFLOAT".to_owned(),
            Self::tRATIONAL(..) => "tRATIONAL".to_owned(),
            Self::tIMAGINARY(..) => "tIMAGINARY".to_owned(),
            Self::tCHAR(..) => "tCHAR".to_owned(),
            Self::tNTH_REF(..) => "tNTH_REF".to_owned(),
            Self::tBACK_REF(..) => "tBACK_REF".to_owned(),
            Self::tSTRING_CONTENT(..) => "tSTRING_CONTENT".to_owned(),
            Self::tREGEXP_END(..) => "tREGEXP_END".to_owned(),
            Self::tSP(..) => "tSP".to_owned(),
            Self::tUPLUS(..) => "tUPLUS".to_owned(),
            Self::tUMINUS(..) => "tUMINUS".to_owned(),
            Self::tPOW(..) => "tPOW".to_owned(),
            Self::tCMP(..) => "tCMP".to_owned(),
            Self::tEQ(..) => "tEQ".to_owned(),
            Self::tEQQ(..) => "tEQQ".to_owned(),
            Self::tNEQ(..) => "tNEQ".to_owned(),
            Self::tGEQ(..) => "tGEQ".to_owned(),
            Self::tLEQ(..) => "tLEQ".to_owned(),
            Self::tANDOP(..) => "tANDOP".to_owned(),
            Self::tOROP(..) => "tOROP".to_owned(),
            Self::tMATCH(..) => "tMATCH".to_owned(),
            Self::tNMATCH(..) => "tNMATCH".to_owned(),
            Self::tDOT2(..) => "tDOT2".to_owned(),
            Self::tDOT3(..) => "tDOT3".to_owned(),
            Self::tBDOT2(..) => "tBDOT2".to_owned(),
            Self::tBDOT3(..) => "tBDOT3".to_owned(),
            Self::tAREF(..) => "tAREF".to_owned(),
            Self::tASET(..) => "tASET".to_owned(),
            Self::tLSHFT(..) => "tLSHFT".to_owned(),
            Self::tRSHFT(..) => "tRSHFT".to_owned(),
            Self::tANDDOT(..) => "tANDDOT".to_owned(),
            Self::tCOLON2(..) => "tCOLON2".to_owned(),
            Self::tCOLON3(..) => "tCOLON3".to_owned(),
            Self::tOP_ASGN(..) => "tOP_ASGN".to_owned(),
            Self::tASSOC(..) => "tASSOC".to_owned(),
            Self::tLPAREN(..) => "tLPAREN".to_owned(),
            Self::tLPAREN_ARG(..) => "tLPAREN_ARG".to_owned(),
            Self::tRPAREN(..) => "tRPAREN".to_owned(),
            Self::tLBRACK(..) => "tLBRACK".to_owned(),
            Self::tLBRACE(..) => "tLBRACE".to_owned(),
            Self::tLBRACE_ARG(..) => "tLBRACE_ARG".to_owned(),
            Self::tSTAR(..) => "tSTAR".to_owned(),
            Self::tDSTAR(..) => "tDSTAR".to_owned(),
            Self::tAMPER(..) => "tAMPER".to_owned(),
            Self::tLAMBDA(..) => "tLAMBDA".to_owned(),
            Self::tSYMBEG(..) => "tSYMBEG".to_owned(),
            Self::tSTRING_BEG(..) => "tSTRING_BEG".to_owned(),
            Self::tXSTRING_BEG(..) => "tXSTRING_BEG".to_owned(),
            Self::tREGEXP_BEG(..) => "tREGEXP_BEG".to_owned(),
            Self::tWORDS_BEG(..) => "tWORDS_BEG".to_owned(),
            Self::tQWORDS_BEG(..) => "tQWORDS_BEG".to_owned(),
            Self::tSYMBOLS_BEG(..) => "tSYMBOLS_BEG".to_owned(),
            Self::tQSYMBOLS_BEG(..) => "tQSYMBOLS_BEG".to_owned(),
            Self::tSTRING_END(..) => "tSTRING_END".to_owned(),
            Self::tSTRING_DEND(..) => "tSTRING_DEND".to_owned(),
            Self::tSTRING_DBEG(..) => "tSTRING_DBEG".to_owned(),
            Self::tSTRING_DVAR(..) => "tSTRING_DVAR".to_owned(),
            Self::tLAMBEG(..) => "tLAMBEG".to_owned(),
            Self::tLABEL_END(..) => "tLABEL_END".to_owned(),
            Self::tLOWEST(..) => "tLOWEST".to_owned(),
            Self::tUMINUS_NUM(..) => "tUMINUS_NUM".to_owned(),
            Self::tLAST_TOKEN(..) => "tLAST_TOKEN".to_owned(),
            Self::tNL(..) => "tNL".to_owned(),
            Self::tIGNORED_NL(..) => "tIGNORED_NL".to_owned(),
            Self::tBANG(..) => "tBANG".to_owned(),
            Self::tLT(..) => "tLT".to_owned(),
            Self::tGT(..) => "tGT".to_owned(),
            Self::tAMPER2(..) => "tAMPER2".to_owned(),
            Self::tPIPE(..) => "tPIPE".to_owned(),
            Self::tPLUS(..) => "tPLUS".to_owned(),
            Self::tMINUS(..) => "tMINUS".to_owned(),
            Self::tDOT(..) => "tDOT".to_owned(),
            Self::tRBRACK(..) => "tRBRACK".to_owned(),
            Self::tRCURLY(..) => "tRCURLY".to_owned(),
            Self::tCOLON(..) => "tCOLON".to_owned(),
            Self::tDIVIDE(..) => "tDIVIDE".to_owned(),
            Self::tCARET(..) => "tCARET".to_owned(),
            Self::tSEMI(..) => "tSEMI".to_owned(),
            Self::tCOMMA(..) => "tCOMMA".to_owned(),
            Self::tTILDE(..) => "tTILDE".to_owned(),
            Self::tLPAREN2(..) => "tLPAREN2".to_owned(),
            Self::tLBRACK2(..) => "tLBRACK2".to_owned(),
            Self::tLCURLY(..) => "tLCURLY".to_owned(),
            Self::tUNARY_NUM(..) => "tUNARY_NUM".to_owned(),
            Self::tSYMBOL(..) => "tSYMBOL".to_owned(),
            Self::tSTRING(..) => "tSTRING".to_owned(),
            Self::tBACK_REF2(..) => "tBACK_REF2".to_owned(),
            Self::tEQL(..) => "tEQL".to_owned(),
            Self::tCHARACTER(..) => "tCHARACTER".to_owned(),
            Self::tREGEXP_OPT(..) => "tREGEXP_OPT".to_owned(),
            Self::tSPACE(..) => "tSPACE".to_owned(),
            Self::tEH(..) => "tEH".to_owned(),
            Self::tPERCENT(..) => "tPERCENT".to_owned(),
            Self::tSTAR2(..) => "tSTAR2".to_owned(),
        }
    }


    pub fn value(&self) -> &String {
        match self {
            Self::END_OF_INPUT(s, ..) |
            Self::kCLASS(s, ..) |
            Self::kMODULE(s, ..) |
            Self::kDEF(s, ..) |
            Self::kUNDEF(s, ..) |
            Self::kBEGIN(s, ..) |
            Self::kRESCUE(s, ..) |
            Self::kENSURE(s, ..) |
            Self::kEND(s, ..) |
            Self::kIF(s, ..) |
            Self::kUNLESS(s, ..) |
            Self::kTHEN(s, ..) |
            Self::kELSIF(s, ..) |
            Self::kELSE(s, ..) |
            Self::kCASE(s, ..) |
            Self::kWHEN(s, ..) |
            Self::kWHILE(s, ..) |
            Self::kUNTIL(s, ..) |
            Self::kFOR(s, ..) |
            Self::kBREAK(s, ..) |
            Self::kNEXT(s, ..) |
            Self::kREDO(s, ..) |
            Self::kRETRY(s, ..) |
            Self::kIN(s, ..) |
            Self::kDO(s, ..) |
            Self::kDO_COND(s, ..) |
            Self::kDO_BLOCK(s, ..) |
            Self::kDO_LAMBDA(s, ..) |
            Self::kRETURN(s, ..) |
            Self::kYIELD(s, ..) |
            Self::kSUPER(s, ..) |
            Self::kSELF(s, ..) |
            Self::kNIL(s, ..) |
            Self::kTRUE(s, ..) |
            Self::kFALSE(s, ..) |
            Self::kAND(s, ..) |
            Self::kOR(s, ..) |
            Self::kNOT(s, ..) |
            Self::kIF_MOD(s, ..) |
            Self::kUNLESS_MOD(s, ..) |
            Self::kWHILE_MOD(s, ..) |
            Self::kUNTIL_MOD(s, ..) |
            Self::kRESCUE_MOD(s, ..) |
            Self::kALIAS(s, ..) |
            Self::kDEFINED(s, ..) |
            Self::klBEGIN(s, ..) |
            Self::klEND(s, ..) |
            Self::k__LINE__(s, ..) |
            Self::k__FILE__(s, ..) |
            Self::k__ENCODING__(s, ..) |
            Self::tIDENTIFIER(s, ..) |
            Self::tFID(s, ..) |
            Self::tGVAR(s, ..) |
            Self::tIVAR(s, ..) |
            Self::tCONSTANT(s, ..) |
            Self::tCVAR(s, ..) |
            Self::tLABEL(s, ..) |
            Self::tINTEGER(s, ..) |
            Self::tFLOAT(s, ..) |
            Self::tRATIONAL(s, ..) |
            Self::tIMAGINARY(s, ..) |
            Self::tCHAR(s, ..) |
            Self::tNTH_REF(s, ..) |
            Self::tBACK_REF(s, ..) |
            Self::tSTRING_CONTENT(s, ..) |
            Self::tREGEXP_END(s, ..) |
            Self::tSP(s, ..) |
            Self::tUPLUS(s, ..) |
            Self::tUMINUS(s, ..) |
            Self::tPOW(s, ..) |
            Self::tCMP(s, ..) |
            Self::tEQ(s, ..) |
            Self::tEQQ(s, ..) |
            Self::tNEQ(s, ..) |
            Self::tGEQ(s, ..) |
            Self::tLEQ(s, ..) |
            Self::tANDOP(s, ..) |
            Self::tOROP(s, ..) |
            Self::tMATCH(s, ..) |
            Self::tNMATCH(s, ..) |
            Self::tDOT2(s, ..) |
            Self::tDOT3(s, ..) |
            Self::tBDOT2(s, ..) |
            Self::tBDOT3(s, ..) |
            Self::tAREF(s, ..) |
            Self::tASET(s, ..) |
            Self::tLSHFT(s, ..) |
            Self::tRSHFT(s, ..) |
            Self::tANDDOT(s, ..) |
            Self::tCOLON2(s, ..) |
            Self::tCOLON3(s, ..) |
            Self::tOP_ASGN(s, ..) |
            Self::tASSOC(s, ..) |
            Self::tLPAREN(s, ..) |
            Self::tLPAREN_ARG(s, ..) |
            Self::tRPAREN(s, ..) |
            Self::tLBRACK(s, ..) |
            Self::tLBRACE(s, ..) |
            Self::tLBRACE_ARG(s, ..) |
            Self::tSTAR(s, ..) |
            Self::tDSTAR(s, ..) |
            Self::tAMPER(s, ..) |
            Self::tLAMBDA(s, ..) |
            Self::tSYMBEG(s, ..) |
            Self::tSTRING_BEG(s, ..) |
            Self::tXSTRING_BEG(s, ..) |
            Self::tREGEXP_BEG(s, ..) |
            Self::tWORDS_BEG(s, ..) |
            Self::tQWORDS_BEG(s, ..) |
            Self::tSYMBOLS_BEG(s, ..) |
            Self::tQSYMBOLS_BEG(s, ..) |
            Self::tSTRING_END(s, ..) |
            Self::tSTRING_DEND(s, ..) |
            Self::tSTRING_DBEG(s, ..) |
            Self::tSTRING_DVAR(s, ..) |
            Self::tLAMBEG(s, ..) |
            Self::tLABEL_END(s, ..) |
            Self::tLOWEST(s, ..) |
            Self::tUMINUS_NUM(s, ..) |
            Self::tLAST_TOKEN(s, ..) |
            Self::tNL(s, ..) |
            Self::tIGNORED_NL(s, ..) |
            Self::tBANG(s, ..) |
            Self::tLT(s, ..) |
            Self::tGT(s, ..) |
            Self::tAMPER2(s, ..) |
            Self::tPIPE(s, ..) |
            Self::tPLUS(s, ..) |
            Self::tMINUS(s, ..) |
            Self::tDOT(s, ..) |
            Self::tRBRACK(s, ..) |
            Self::tRCURLY(s, ..) |
            Self::tCOLON(s, ..) |
            Self::tDIVIDE(s, ..) |
            Self::tCARET(s, ..) |
            Self::tSEMI(s, ..) |
            Self::tCOMMA(s, ..) |
            Self::tTILDE(s, ..) |
            Self::tLPAREN2(s, ..) |
            Self::tLBRACK2(s, ..) |
            Self::tLCURLY(s, ..) |
            Self::tUNARY_NUM(s, ..) |
            Self::tSYMBOL(s, ..) |
            Self::tSTRING(s, ..) |
            Self::tBACK_REF2(s, ..) |
            Self::tEQL(s, ..) |
            Self::tCHARACTER(s, ..) |
            Self::tREGEXP_OPT(s, ..) |
            Self::tSPACE(s, ..) |
            Self::tEH(s, ..) |
            Self::tPERCENT(s, ..) |
            Self::tSTAR2(s, ..) => {
                s
            }
        }
    }

    pub fn begin(&self) -> &usize {
        match self {
            Self::END_OF_INPUT(_s, begin, _end) |
            Self::kCLASS(_s, begin, _end) |
            Self::kMODULE(_s, begin, _end) |
            Self::kDEF(_s, begin, _end) |
            Self::kUNDEF(_s, begin, _end) |
            Self::kBEGIN(_s, begin, _end) |
            Self::kRESCUE(_s, begin, _end) |
            Self::kENSURE(_s, begin, _end) |
            Self::kEND(_s, begin, _end) |
            Self::kIF(_s, begin, _end) |
            Self::kUNLESS(_s, begin, _end) |
            Self::kTHEN(_s, begin, _end) |
            Self::kELSIF(_s, begin, _end) |
            Self::kELSE(_s, begin, _end) |
            Self::kCASE(_s, begin, _end) |
            Self::kWHEN(_s, begin, _end) |
            Self::kWHILE(_s, begin, _end) |
            Self::kUNTIL(_s, begin, _end) |
            Self::kFOR(_s, begin, _end) |
            Self::kBREAK(_s, begin, _end) |
            Self::kNEXT(_s, begin, _end) |
            Self::kREDO(_s, begin, _end) |
            Self::kRETRY(_s, begin, _end) |
            Self::kIN(_s, begin, _end) |
            Self::kDO(_s, begin, _end) |
            Self::kDO_COND(_s, begin, _end) |
            Self::kDO_BLOCK(_s, begin, _end) |
            Self::kDO_LAMBDA(_s, begin, _end) |
            Self::kRETURN(_s, begin, _end) |
            Self::kYIELD(_s, begin, _end) |
            Self::kSUPER(_s, begin, _end) |
            Self::kSELF(_s, begin, _end) |
            Self::kNIL(_s, begin, _end) |
            Self::kTRUE(_s, begin, _end) |
            Self::kFALSE(_s, begin, _end) |
            Self::kAND(_s, begin, _end) |
            Self::kOR(_s, begin, _end) |
            Self::kNOT(_s, begin, _end) |
            Self::kIF_MOD(_s, begin, _end) |
            Self::kUNLESS_MOD(_s, begin, _end) |
            Self::kWHILE_MOD(_s, begin, _end) |
            Self::kUNTIL_MOD(_s, begin, _end) |
            Self::kRESCUE_MOD(_s, begin, _end) |
            Self::kALIAS(_s, begin, _end) |
            Self::kDEFINED(_s, begin, _end) |
            Self::klBEGIN(_s, begin, _end) |
            Self::klEND(_s, begin, _end) |
            Self::k__LINE__(_s, begin, _end) |
            Self::k__FILE__(_s, begin, _end) |
            Self::k__ENCODING__(_s, begin, _end) |
            Self::tIDENTIFIER(_s, begin, _end) |
            Self::tFID(_s, begin, _end) |
            Self::tGVAR(_s, begin, _end) |
            Self::tIVAR(_s, begin, _end) |
            Self::tCONSTANT(_s, begin, _end) |
            Self::tCVAR(_s, begin, _end) |
            Self::tLABEL(_s, begin, _end) |
            Self::tINTEGER(_s, begin, _end) |
            Self::tFLOAT(_s, begin, _end) |
            Self::tRATIONAL(_s, begin, _end) |
            Self::tIMAGINARY(_s, begin, _end) |
            Self::tCHAR(_s, begin, _end) |
            Self::tNTH_REF(_s, begin, _end) |
            Self::tBACK_REF(_s, begin, _end) |
            Self::tSTRING_CONTENT(_s, begin, _end) |
            Self::tREGEXP_END(_s, begin, _end) |
            Self::tSP(_s, begin, _end) |
            Self::tUPLUS(_s, begin, _end) |
            Self::tUMINUS(_s, begin, _end) |
            Self::tPOW(_s, begin, _end) |
            Self::tCMP(_s, begin, _end) |
            Self::tEQ(_s, begin, _end) |
            Self::tEQQ(_s, begin, _end) |
            Self::tNEQ(_s, begin, _end) |
            Self::tGEQ(_s, begin, _end) |
            Self::tLEQ(_s, begin, _end) |
            Self::tANDOP(_s, begin, _end) |
            Self::tOROP(_s, begin, _end) |
            Self::tMATCH(_s, begin, _end) |
            Self::tNMATCH(_s, begin, _end) |
            Self::tDOT2(_s, begin, _end) |
            Self::tDOT3(_s, begin, _end) |
            Self::tBDOT2(_s, begin, _end) |
            Self::tBDOT3(_s, begin, _end) |
            Self::tAREF(_s, begin, _end) |
            Self::tASET(_s, begin, _end) |
            Self::tLSHFT(_s, begin, _end) |
            Self::tRSHFT(_s, begin, _end) |
            Self::tANDDOT(_s, begin, _end) |
            Self::tCOLON2(_s, begin, _end) |
            Self::tCOLON3(_s, begin, _end) |
            Self::tOP_ASGN(_s, begin, _end) |
            Self::tASSOC(_s, begin, _end) |
            Self::tLPAREN(_s, begin, _end) |
            Self::tLPAREN_ARG(_s, begin, _end) |
            Self::tRPAREN(_s, begin, _end) |
            Self::tLBRACK(_s, begin, _end) |
            Self::tLBRACE(_s, begin, _end) |
            Self::tLBRACE_ARG(_s, begin, _end) |
            Self::tSTAR(_s, begin, _end) |
            Self::tDSTAR(_s, begin, _end) |
            Self::tAMPER(_s, begin, _end) |
            Self::tLAMBDA(_s, begin, _end) |
            Self::tSYMBEG(_s, begin, _end) |
            Self::tSTRING_BEG(_s, begin, _end) |
            Self::tXSTRING_BEG(_s, begin, _end) |
            Self::tREGEXP_BEG(_s, begin, _end) |
            Self::tWORDS_BEG(_s, begin, _end) |
            Self::tQWORDS_BEG(_s, begin, _end) |
            Self::tSYMBOLS_BEG(_s, begin, _end) |
            Self::tQSYMBOLS_BEG(_s, begin, _end) |
            Self::tSTRING_END(_s, begin, _end) |
            Self::tSTRING_DEND(_s, begin, _end) |
            Self::tSTRING_DBEG(_s, begin, _end) |
            Self::tSTRING_DVAR(_s, begin, _end) |
            Self::tLAMBEG(_s, begin, _end) |
            Self::tLABEL_END(_s, begin, _end) |
            Self::tLOWEST(_s, begin, _end) |
            Self::tUMINUS_NUM(_s, begin, _end) |
            Self::tLAST_TOKEN(_s, begin, _end) |
            Self::tNL(_s, begin, _end) |
            Self::tIGNORED_NL(_s, begin, _end) |
            Self::tBANG(_s, begin, _end) |
            Self::tLT(_s, begin, _end) |
            Self::tGT(_s, begin, _end) |
            Self::tAMPER2(_s, begin, _end) |
            Self::tPIPE(_s, begin, _end) |
            Self::tPLUS(_s, begin, _end) |
            Self::tMINUS(_s, begin, _end) |
            Self::tDOT(_s, begin, _end) |
            Self::tRBRACK(_s, begin, _end) |
            Self::tRCURLY(_s, begin, _end) |
            Self::tCOLON(_s, begin, _end) |
            Self::tDIVIDE(_s, begin, _end) |
            Self::tCARET(_s, begin, _end) |
            Self::tSEMI(_s, begin, _end) |
            Self::tCOMMA(_s, begin, _end) |
            Self::tTILDE(_s, begin, _end) |
            Self::tLPAREN2(_s, begin, _end) |
            Self::tLBRACK2(_s, begin, _end) |
            Self::tLCURLY(_s, begin, _end) |
            Self::tUNARY_NUM(_s, begin, _end) |
            Self::tSYMBOL(_s, begin, _end) |
            Self::tSTRING(_s, begin, _end) |
            Self::tBACK_REF2(_s, begin, _end) |
            Self::tEQL(_s, begin, _end) |
            Self::tCHARACTER(_s, begin, _end) |
            Self::tREGEXP_OPT(_s, begin, _end) |
            Self::tSPACE(_s, begin, _end) |
            Self::tEH(_s, begin, _end) |
            Self::tPERCENT(_s, begin, _end) |
            Self::tSTAR2(_s, begin, _end) => {
                begin
            }
        }
    }

    pub fn end(&self) -> &usize {
        match self {
            Self::END_OF_INPUT(_s, _begin, end) |
            Self::kCLASS(_s, _begin, end) |
            Self::kMODULE(_s, _begin, end) |
            Self::kDEF(_s, _begin, end) |
            Self::kUNDEF(_s, _begin, end) |
            Self::kBEGIN(_s, _begin, end) |
            Self::kRESCUE(_s, _begin, end) |
            Self::kENSURE(_s, _begin, end) |
            Self::kEND(_s, _begin, end) |
            Self::kIF(_s, _begin, end) |
            Self::kUNLESS(_s, _begin, end) |
            Self::kTHEN(_s, _begin, end) |
            Self::kELSIF(_s, _begin, end) |
            Self::kELSE(_s, _begin, end) |
            Self::kCASE(_s, _begin, end) |
            Self::kWHEN(_s, _begin, end) |
            Self::kWHILE(_s, _begin, end) |
            Self::kUNTIL(_s, _begin, end) |
            Self::kFOR(_s, _begin, end) |
            Self::kBREAK(_s, _begin, end) |
            Self::kNEXT(_s, _begin, end) |
            Self::kREDO(_s, _begin, end) |
            Self::kRETRY(_s, _begin, end) |
            Self::kIN(_s, _begin, end) |
            Self::kDO(_s, _begin, end) |
            Self::kDO_COND(_s, _begin, end) |
            Self::kDO_BLOCK(_s, _begin, end) |
            Self::kDO_LAMBDA(_s, _begin, end) |
            Self::kRETURN(_s, _begin, end) |
            Self::kYIELD(_s, _begin, end) |
            Self::kSUPER(_s, _begin, end) |
            Self::kSELF(_s, _begin, end) |
            Self::kNIL(_s, _begin, end) |
            Self::kTRUE(_s, _begin, end) |
            Self::kFALSE(_s, _begin, end) |
            Self::kAND(_s, _begin, end) |
            Self::kOR(_s, _begin, end) |
            Self::kNOT(_s, _begin, end) |
            Self::kIF_MOD(_s, _begin, end) |
            Self::kUNLESS_MOD(_s, _begin, end) |
            Self::kWHILE_MOD(_s, _begin, end) |
            Self::kUNTIL_MOD(_s, _begin, end) |
            Self::kRESCUE_MOD(_s, _begin, end) |
            Self::kALIAS(_s, _begin, end) |
            Self::kDEFINED(_s, _begin, end) |
            Self::klBEGIN(_s, _begin, end) |
            Self::klEND(_s, _begin, end) |
            Self::k__LINE__(_s, _begin, end) |
            Self::k__FILE__(_s, _begin, end) |
            Self::k__ENCODING__(_s, _begin, end) |
            Self::tIDENTIFIER(_s, _begin, end) |
            Self::tFID(_s, _begin, end) |
            Self::tGVAR(_s, _begin, end) |
            Self::tIVAR(_s, _begin, end) |
            Self::tCONSTANT(_s, _begin, end) |
            Self::tCVAR(_s, _begin, end) |
            Self::tLABEL(_s, _begin, end) |
            Self::tINTEGER(_s, _begin, end) |
            Self::tFLOAT(_s, _begin, end) |
            Self::tRATIONAL(_s, _begin, end) |
            Self::tIMAGINARY(_s, _begin, end) |
            Self::tCHAR(_s, _begin, end) |
            Self::tNTH_REF(_s, _begin, end) |
            Self::tBACK_REF(_s, _begin, end) |
            Self::tSTRING_CONTENT(_s, _begin, end) |
            Self::tREGEXP_END(_s, _begin, end) |
            Self::tSP(_s, _begin, end) |
            Self::tUPLUS(_s, _begin, end) |
            Self::tUMINUS(_s, _begin, end) |
            Self::tPOW(_s, _begin, end) |
            Self::tCMP(_s, _begin, end) |
            Self::tEQ(_s, _begin, end) |
            Self::tEQQ(_s, _begin, end) |
            Self::tNEQ(_s, _begin, end) |
            Self::tGEQ(_s, _begin, end) |
            Self::tLEQ(_s, _begin, end) |
            Self::tANDOP(_s, _begin, end) |
            Self::tOROP(_s, _begin, end) |
            Self::tMATCH(_s, _begin, end) |
            Self::tNMATCH(_s, _begin, end) |
            Self::tDOT2(_s, _begin, end) |
            Self::tDOT3(_s, _begin, end) |
            Self::tBDOT2(_s, _begin, end) |
            Self::tBDOT3(_s, _begin, end) |
            Self::tAREF(_s, _begin, end) |
            Self::tASET(_s, _begin, end) |
            Self::tLSHFT(_s, _begin, end) |
            Self::tRSHFT(_s, _begin, end) |
            Self::tANDDOT(_s, _begin, end) |
            Self::tCOLON2(_s, _begin, end) |
            Self::tCOLON3(_s, _begin, end) |
            Self::tOP_ASGN(_s, _begin, end) |
            Self::tASSOC(_s, _begin, end) |
            Self::tLPAREN(_s, _begin, end) |
            Self::tLPAREN_ARG(_s, _begin, end) |
            Self::tRPAREN(_s, _begin, end) |
            Self::tLBRACK(_s, _begin, end) |
            Self::tLBRACE(_s, _begin, end) |
            Self::tLBRACE_ARG(_s, _begin, end) |
            Self::tSTAR(_s, _begin, end) |
            Self::tDSTAR(_s, _begin, end) |
            Self::tAMPER(_s, _begin, end) |
            Self::tLAMBDA(_s, _begin, end) |
            Self::tSYMBEG(_s, _begin, end) |
            Self::tSTRING_BEG(_s, _begin, end) |
            Self::tXSTRING_BEG(_s, _begin, end) |
            Self::tREGEXP_BEG(_s, _begin, end) |
            Self::tWORDS_BEG(_s, _begin, end) |
            Self::tQWORDS_BEG(_s, _begin, end) |
            Self::tSYMBOLS_BEG(_s, _begin, end) |
            Self::tQSYMBOLS_BEG(_s, _begin, end) |
            Self::tSTRING_END(_s, _begin, end) |
            Self::tSTRING_DEND(_s, _begin, end) |
            Self::tSTRING_DBEG(_s, _begin, end) |
            Self::tSTRING_DVAR(_s, _begin, end) |
            Self::tLAMBEG(_s, _begin, end) |
            Self::tLABEL_END(_s, _begin, end) |
            Self::tLOWEST(_s, _begin, end) |
            Self::tUMINUS_NUM(_s, _begin, end) |
            Self::tLAST_TOKEN(_s, _begin, end) |
            Self::tNL(_s, _begin, end) |
            Self::tIGNORED_NL(_s, _begin, end) |
            Self::tBANG(_s, _begin, end) |
            Self::tLT(_s, _begin, end) |
            Self::tGT(_s, _begin, end) |
            Self::tAMPER2(_s, _begin, end) |
            Self::tPIPE(_s, _begin, end) |
            Self::tPLUS(_s, _begin, end) |
            Self::tMINUS(_s, _begin, end) |
            Self::tDOT(_s, _begin, end) |
            Self::tRBRACK(_s, _begin, end) |
            Self::tRCURLY(_s, _begin, end) |
            Self::tCOLON(_s, _begin, end) |
            Self::tDIVIDE(_s, _begin, end) |
            Self::tCARET(_s, _begin, end) |
            Self::tSEMI(_s, _begin, end) |
            Self::tCOMMA(_s, _begin, end) |
            Self::tTILDE(_s, _begin, end) |
            Self::tLPAREN2(_s, _begin, end) |
            Self::tLBRACK2(_s, _begin, end) |
            Self::tLCURLY(_s, _begin, end) |
            Self::tUNARY_NUM(_s, _begin, end) |
            Self::tSYMBOL(_s, _begin, end) |
            Self::tSTRING(_s, _begin, end) |
            Self::tBACK_REF2(_s, _begin, end) |
            Self::tEQL(_s, _begin, end) |
            Self::tCHARACTER(_s, _begin, end) |
            Self::tREGEXP_OPT(_s, _begin, end) |
            Self::tSPACE(_s, _begin, end) |
            Self::tEH(_s, _begin, end) |
            Self::tPERCENT(_s, _begin, end) |
            Self::tSTAR2(_s, _begin, end) => {
                end
            }
        }
    }
}

pub type TokenType = fn(String, usize, usize) -> Token;

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entry(&self.name())
            .entry(self.value())
            .entry(&[self.begin(), self.end()])
            .finish()
    }
}

pub type TokenData = (String, usize, usize);
