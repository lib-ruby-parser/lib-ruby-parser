fn contents() -> String {
    let messages = lib_ruby_parser_nodes::messages();

    format!(
        "// This file is autogenerated by {generator}

use super::DiagnosticMessage;

#[cfg(feature = \"compile-with-external-structures\")]
use crate::containers::ExternalStringPtr;
#[cfg(feature = \"compile-with-external-structures\")]
type StringPtr = ExternalStringPtr;
#[cfg(not(feature = \"compile-with-external-structures\"))]
type StringPtr = String;

{tests}

fn make_str() -> StringPtr {{
    StringPtr::from(String::from(\"foo\"))
}}
fn make_byte() -> u8 {{
    42
}}
",
        generator = file!(),
        tests = messages.map(&message_test).join("\n"),
    )
}

pub(crate) fn codegen() {
    std::fs::write("src/error/message/tests.rs", contents()).unwrap();
}

fn message_test(message: &lib_ruby_parser_nodes::Message) -> String {
    let assert_getters = message
        .fields
        .map(&|field| {
            let lhs = format!("variant.get_{field_name}()", field_name = field.name);

            let rhs = format!("&{}()", make_field_code(field));

            format!("assert_eq!({}, {});", lhs, rhs)
        })
        .join("\n    ");

    format!(
        "#[test]
fn test_{variant}() {{
    let message = {make_message};
    let variant = message.as_{variant}().unwrap();
    {assert_getters}
    drop(variant);
    drop(message);
}}",
        variant = message.lower_name(),
        make_message = make_message_code(message),
        assert_getters = assert_getters
    )
}

fn make_field_code(field: &lib_ruby_parser_nodes::MessageField) -> &str {
    match field.field_type {
        lib_ruby_parser_nodes::MessageFieldType::Str => "make_str",
        lib_ruby_parser_nodes::MessageFieldType::Byte => "make_byte",
    }
}

fn make_message_code(message: &lib_ruby_parser_nodes::Message) -> String {
    let args = message
        .fields
        .map(&|field| format!("{}()", make_field_code(field)))
        .join(", ");

    format!(
        "DiagnosticMessage::new_{variant}({args})",
        variant = message.lower_name(),
        args = args
    )
}
