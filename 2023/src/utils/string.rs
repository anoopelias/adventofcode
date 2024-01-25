pub(crate) trait WrapperRemover {
    fn remove_wrapping(&self) -> Self;
}

impl WrapperRemover for &str {
    fn remove_wrapping(&self) -> Self {
        &self[1..self.len() - 1]
    }
}
