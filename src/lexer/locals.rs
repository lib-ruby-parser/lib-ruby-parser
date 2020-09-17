use crate::State;

impl State {
    pub fn is_lvar_defined(&self, name: &str) -> bool {
        self.p.static_env.is_declared(name)
    }
}
