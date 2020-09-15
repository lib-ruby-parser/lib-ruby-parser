use std::collections::HashSet;

#[derive(Debug, Clone, Default)]
pub struct LocalsTable {
    args: HashSet<String>,
    vars: HashSet<String>,
    used: HashSet<String>,
    prev: Option<Box<LocalsTable>>,
    // ??numparam
}

impl LocalsTable {
    pub fn new() -> Self {
        Self {
            args: HashSet::new(),
            vars: HashSet::new(),
            used: HashSet::new(),
            prev: None
        }
    }
}
