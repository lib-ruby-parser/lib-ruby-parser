pub(crate) struct Comment<'a> {
    comment: &'a str,
}

impl<'a> Comment<'a> {
    pub(crate) fn new(comment: &'a str) -> Self {
        Self { comment }
    }

    pub(crate) fn to_string(&self, offset: usize) -> String {
        self.comment
            .split("\n")
            .map(|l| {
                let mut l = l.to_string();
                if !l.is_empty() {
                    l = format!(" {}", l);
                }
                format!("{}///{}", " ".repeat(offset), l)
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}
