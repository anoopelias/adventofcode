#![allow(unused)]

pub enum PqStrategy {
    Min,
    Max,
}

pub struct Pq<T: Ord> {
    values: Vec<T>,
    strategy: PqStrategy,
}

impl<T: Ord> Pq<T> {
    pub fn new(strategy: PqStrategy) -> Pq<T> {
        Pq {
            values: Vec::new(),
            strategy,
        }
    }

    pub fn insert(&mut self, value: T) {
        self.values.push(value);
        self.values.sort_by(|a, b| match self.strategy {
            PqStrategy::Min => b.cmp(a),
            PqStrategy::Max => a.cmp(b),
        });
    }

    pub fn remove(&mut self) -> Option<T> {
        self.values.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::Pq;
    use super::PqStrategy;

    #[test]
    fn insert_remove() {
        let mut pq = Pq::new(PqStrategy::Min);
        pq.insert(10);
        assert_eq!(Some(10), pq.remove());
    }

    #[test]
    fn insert_remove_min_in_order() {
        let mut pq = Pq::new(PqStrategy::Min);
        pq.insert(12);
        pq.insert(10);
        assert_eq!(Some(10), pq.remove());
    }

    #[test]
    fn insert_remove_min_out_of_order() {
        let mut pq = Pq::new(PqStrategy::Min);
        pq.insert(10);
        pq.insert(12);
        assert_eq!(Some(10), pq.remove());
    }

    #[test]
    fn insert_remove_max_in_order() {
        let mut pq = Pq::new(PqStrategy::Max);
        pq.insert(10);
        pq.insert(12);
        assert_eq!(Some(12), pq.remove());
    }

    #[test]
    fn insert_remove_max_out_of_order() {
        let mut pq = Pq::new(PqStrategy::Max);
        pq.insert(12);
        pq.insert(10);
        assert_eq!(Some(12), pq.remove());
    }
}
