use super::TokenRewriter;

impl std::fmt::Debug for TokenRewriter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenRewriter").finish()
    }
}
