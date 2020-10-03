use crate::Lexer;

impl Lexer {
    pub fn is_lvar_defined(&self, name: &str) -> bool {
        self.static_env.is_declared(name)
    }
}
