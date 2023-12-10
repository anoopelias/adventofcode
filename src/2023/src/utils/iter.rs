pub trait Pairs<T> {
    fn to_pairs(&self) -> Vec<(&T, &T)>;
}

impl<T> Pairs<T> for Vec<T> {
    fn to_pairs(&self) -> Vec<(&T, &T)> {
        self.iter().zip(self[1..].iter()).collect()
    }
}
