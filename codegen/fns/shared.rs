use lib_ruby_parser_nodes::{
    template::TemplateFns, template::F, Message, MessageField, Node, NodeField,
};

pub(crate) mod nodes {
    use super::*;

    pub(crate) fn camelcase_name(node: &Node) -> String {
        node.camelcase_name.to_owned()
    }

    pub(crate) fn comment(node: &Node) -> String {
        node.render_comment("///", 0)
    }

    pub(crate) fn str_type(node: &Node) -> String {
        node.wqp_name.to_string()
    }

    pub(crate) fn upper_name(node: &Node) -> String {
        node.upper_name()
    }

    pub(crate) fn lower_name(node: &Node) -> String {
        node.lower_name()
    }

    pub(crate) fn is_last(node: &Node) -> bool {
        lib_ruby_parser_nodes::template::ALL_DATA
            .nodes
            .last()
            .unwrap()
            == &node
    }
}

pub(crate) mod node_fields {
    use super::*;

    pub(crate) fn name(node_field: &NodeField) -> String {
        node_field.snakecase_name.to_string()
    }

    pub(crate) fn comment(node_field: &NodeField) -> String {
        node_field.render_comment("///", 4)
    }

    pub(crate) fn is_last(node_field: &NodeField) -> bool {
        node_field.node.fields.last().unwrap() == &node_field
    }
}

pub(crate) mod messages {
    use super::*;

    pub(crate) fn camelcase_name(message: &Message) -> String {
        message.camelcase_name.to_owned()
    }

    pub(crate) fn upper_name(message: &Message) -> String {
        message.upper_name()
    }

    pub(crate) fn lower_name(message: &Message) -> String {
        message.lower_name()
    }

    pub(crate) fn has_no_fields(message: &Message) -> bool {
        message.fields.is_empty()
    }

    pub(crate) fn comment(message: &Message) -> String {
        message.render_comment("///", 0)
    }

    pub(crate) fn is_last(message: &Message) -> bool {
        lib_ruby_parser_nodes::template::ALL_DATA
            .messages
            .last()
            .unwrap()
            == &message
    }
}

pub(crate) mod message_fields {
    use super::*;

    pub(crate) fn name(message_field: &MessageField) -> String {
        message_field.snakecase_name.to_owned()
    }

    pub(crate) fn comment(message_field: &MessageField) -> String {
        message_field.render_comment("///", 4)
    }

    pub(crate) fn is_last(message_field: &MessageField) -> bool {
        message_field.message.fields.last().unwrap() == &message_field
    }
}

pub(crate) fn build() -> TemplateFns {
    let mut fns = TemplateFns::new();

    fns.register::<Node, F::Helper>("node-camelcase-name", nodes::camelcase_name);
    fns.register::<Node, F::Helper>("node-comment", nodes::comment);
    fns.register::<Node, F::Helper>("node-str-type", nodes::str_type);
    fns.register::<Node, F::Helper>("node-upper-name", nodes::upper_name);
    fns.register::<Node, F::Helper>("node-lower-name", nodes::lower_name);
    fns.register::<Node, F::Predicate>("node-is-last", nodes::is_last);

    fns.register::<NodeField, F::Helper>("node-field-name", node_fields::name);
    fns.register::<NodeField, F::Helper>("node-field-comment", node_fields::comment);
    fns.register::<NodeField, F::Predicate>("node-field-is-last", node_fields::is_last);

    fns.register::<Message, F::Helper>("message-camelcase-name", messages::camelcase_name);
    fns.register::<Message, F::Helper>("message-upper-name", messages::upper_name);
    fns.register::<Message, F::Helper>("message-lower-name", messages::lower_name);
    fns.register::<Message, F::Helper>("message-comment", messages::comment);
    fns.register::<Message, F::Predicate>("message-is-last", messages::is_last);
    fns.register::<Message, F::Predicate>("message-has-no-fields", messages::has_no_fields);

    fns.register::<MessageField, F::Helper>("message-field-name", message_fields::name);
    fns.register::<MessageField, F::Helper>("mesage-field-comment", message_fields::comment);
    fns.register::<MessageField, F::Predicate>("message-field-is-last", message_fields::is_last);

    fns
}
