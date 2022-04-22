use lib_ruby_parser_nodes::{
    reexports::{
        liquid::value,
        serde::{Serialize, SerializeStruct, Serializer},
    },
    LiquidTemplate,
};

pub(crate) fn codegen() {
    let contents = LiquidTemplate::new("codegen/rust/nodes/loc_name.liquid")
        .with_global("loc_names", value!(LOC_NAMES.to_owned()))
        .render();
    std::fs::write("src/test_helpers/loc_matcher/loc_name_gen.rs", contents).unwrap();
}

struct LocName {
    lower: &'static str,
    camelcase: &'static str,
}

impl Serialize for LocName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("LocName", 2)?;
        state.serialize_field("lower", &self.lower)?;
        state.serialize_field("camelcase", &self.camelcase)?;
        state.end()
    }
}

const LOC_NAMES: &[LocName] = &[
    LocName {
        camelcase: "Begin",
        lower: "begin_l",
    },
    LocName {
        camelcase: "End",
        lower: "end_l",
    },
    LocName {
        camelcase: "Expression",
        lower: "expression_l",
    },
    LocName {
        camelcase: "Keyword",
        lower: "keyword_l",
    },
    LocName {
        camelcase: "Name",
        lower: "name_l",
    },
    LocName {
        camelcase: "Assignment",
        lower: "assignment_l",
    },
    LocName {
        camelcase: "Colon",
        lower: "colon_l",
    },
    LocName {
        camelcase: "DoubleColon",
        lower: "double_colon_l",
    },
    LocName {
        camelcase: "Else",
        lower: "else_l",
    },
    LocName {
        camelcase: "HeredocBody",
        lower: "heredoc_body_l",
    },
    LocName {
        camelcase: "Operator",
        lower: "operator_l",
    },
    LocName {
        camelcase: "Selector",
        lower: "selector_l",
    },
    LocName {
        camelcase: "Assoc",
        lower: "assoc_l",
    },
    LocName {
        camelcase: "Question",
        lower: "question_l",
    },
    LocName {
        camelcase: "HeredocEnd",
        lower: "heredoc_end_l",
    },
];
