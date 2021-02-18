pub struct Comment<'a> {
    comment: &'a str,
}

impl<'a> Comment<'a> {
    pub fn new(comment: &'a str) -> Self {
        Self { comment }
    }

    pub fn to_string(&self, offset: usize) -> String {
        self.comment
            .split("\n")
            .map(|l| {
                let mut l = l.to_owned();
                if !l.is_empty() {
                    l = format!(" {}", l);
                }
                format!("{}///{}", " ".repeat(offset), l)
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}
