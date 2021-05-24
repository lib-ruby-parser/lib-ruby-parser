use super::comment::Comment;

pub(crate) struct Messages<'a> {
    registry: &'a lib_ruby_parser_nodes::Messages,
}

impl<'a> Messages<'a> {
    pub(crate) fn new(registry: &'a lib_ruby_parser_nodes::Messages) -> Self {
        Self { registry }
    }

    pub(crate) fn write(&self) {
        std::fs::write("src/error/message_gen.rs", self.contents()).unwrap();
    }

    fn contents(&self) -> String {
        format!(
            "#[cfg(feature = \"compile-with-external-structures\")]
use crate::containers::ExternalStringPtr;
#[cfg(feature = \"compile-with-external-structures\")]
type StringPtr = ExternalStringPtr;
#[cfg(not(feature = \"compile-with-external-structures\"))]
type StringPtr = String;

/// Enum of all possible diagnostic message (both warnings and errors)
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub enum DiagnosticMessage {{
    {sections}
}}
",
            sections = self.sections().join("\n    ")
        )
    }

    fn sections(&self) -> Vec<String> {
        self.registry
            .sections
            .iter()
            .map(|s| Section::new(s).to_string())
            .collect()
    }
}

struct Section<'a> {
    section: &'a lib_ruby_parser_nodes::Section,
}

impl<'a> Section<'a> {
    fn new(section: &'a lib_ruby_parser_nodes::Section) -> Self {
        Self { section }
    }

    fn to_string(&self) -> String {
        format!(
            "/* {section_name} */

{messages}",
            section_name = self.section.name,
            messages = self.messages().join("\n\n")
        )
    }

    fn messages(&self) -> Vec<String> {
        self.section
            .messages
            .iter()
            .map(|m| Message::new(m).to_string())
            .collect()
    }
}

struct Message<'a> {
    message: &'a lib_ruby_parser_nodes::Message,
}

impl<'a> Message<'a> {
    fn new(message: &'a lib_ruby_parser_nodes::Message) -> Self {
        Self { message }
    }

    fn to_string(&self) -> String {
        format!(
            "{comment}
    {variant}{fields},",
            comment = Comment::new(&self.message.comment).to_string(4),
            variant = self.message.name,
            fields = self.fields()
        )
    }

    fn fields(&self) -> String {
        if self.message.fields.is_empty() {
            return "".to_string();
        }

        let fields = self
            .message
            .fields
            .iter()
            .map(|f| Field::new(f).to_string())
            .collect::<Vec<_>>();

        format!(
            " {{
{}
    }}",
            fields.join("\n\n")
        )
    }
}

struct Field<'a> {
    field: &'a lib_ruby_parser_nodes::MessageField,
}

impl<'a> Field<'a> {
    fn new(field: &'a lib_ruby_parser_nodes::MessageField) -> Self {
        Self { field }
    }

    fn to_string(&self) -> String {
        format!(
            "{comment}
        {field_name}: {field_type},",
            comment = Comment::new(&self.field.comment).to_string(8),
            field_name = self.field.name,
            field_type = self.field_type(),
        )
    }

    fn field_type(&self) -> &str {
        use lib_ruby_parser_nodes::MessageFieldType as FieldType;

        match self.field.field_type {
            FieldType::Str => "StringPtr",
            FieldType::Byte => "u8",
        }
    }
}
