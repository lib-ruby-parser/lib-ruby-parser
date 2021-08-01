use super::ErrorLevel;

impl ToString for ErrorLevel {
    fn to_string(&self) -> String {
        if self.is_warning() {
            "warning"
        } else if self.is_error() {
            "error"
        } else {
            unreachable!("only error/warning supported")
        }
        .to_string()
    }
}

impl std::fmt::Debug for ErrorLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}
