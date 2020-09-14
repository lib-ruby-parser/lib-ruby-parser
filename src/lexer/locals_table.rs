use std::collections::HashSet;

pub struct LocalsTable {
    args: HashSet<String>,
    vars: HashSet<String>,
    used: HashSet<String>,
    prev: Option<Box<LocalsTable>>,
    // ??numparam
}
