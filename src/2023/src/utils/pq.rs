use std::cmp::Ordering;

pub enum PqType {
    Min,
    Max,
}

pub struct Pq<T: Ord> {
    values: Vec<T>,
    ty: PqType,
}

impl<T: Ord> Pq<T> {
    pub fn new(strategy: PqType) -> Pq<T> {
        Pq {
            values: Vec::new(),
            ty: strategy,
        }
    }

    pub fn insert(&mut self, value: T) {
        self.values.push(value);
        self.swim(self.values.len() - 1);
    }

    pub fn remove(&mut self) -> Option<T> {
        if self.values.is_empty() {
            None
        } else {
            let result = self.values.swap_remove(0);
            self.sink(0);
            Some(result)
        }
    }

    pub fn is_empty(&self) -> bool {
        self.values.len() == 0
    }

    fn swim(&mut self, n: usize) {
        if n == 0 {
            return;
        }

        // 0 -> 1, 2
        // 1 -> 3, 4
        // 2 -> 5, 6
        // 3 -> 7, 8
        // 4 -> 9, 10
        //
        // Children: 2n + 1, 2n + 2
        // Parent: Math.floor((n - 1) / 2)
        //
        let pi = (n - 1) / 2;
        let child = self.values.get(n).unwrap();
        let parent = self.values.get(pi).unwrap();
        if !self.less(parent, child) {
            self.values.swap(n, pi);
            self.swim(pi);
        }
    }

    fn less(&self, a: &T, b: &T) -> bool {
        match self.ty {
            PqType::Min => a.cmp(b) == Ordering::Less,
            PqType::Max => b.cmp(a) == Ordering::Less,
        }
    }

    fn lesser_child(&self, n: usize) -> Option<(usize, &T)> {
        let ci = (2 * n) + 1;

        match (self.values.get(ci), self.values.get(ci + 1)) {
            (Some(child1), Some(child2)) if !self.less(child1, child2) => Some((ci + 1, child2)),
            (Some(child), _) => Some((ci, child)),
            (None, Some(child)) => Some((ci + 1, child)),
            _ => None,
        }
    }

    fn sink(&mut self, n: usize) {
        if let Some(parent) = self.values.get(n) {
            if let Some((ci, child)) = self.lesser_child(n) {
                if !self.less(parent, child) {
                    self.values.swap(n, ci);
                    self.sink(ci);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Pq;
    use super::PqType;

    #[test]
    fn insert_remove() {
        let mut pq = Pq::new(PqType::Min);
        pq.insert(10);
        assert_eq!(Some(10), pq.remove());
    }

    #[test]
    fn insert_remove_min_in_order() {
        let mut pq = Pq::new(PqType::Min);
        pq.insert(12);
        pq.insert(10);
        assert_eq!(Some(10), pq.remove());
    }

    #[test]
    fn insert_remove_min_out_of_order() {
        let mut pq = Pq::new(PqType::Min);
        pq.insert(10);
        pq.insert(12);
        assert_eq!(Some(10), pq.remove());
    }

    #[test]
    fn insert_remove_max_in_order() {
        let mut pq = Pq::new(PqType::Max);
        pq.insert(10);
        pq.insert(12);
        assert_eq!(Some(12), pq.remove());
    }

    #[test]
    fn insert_remove_max_out_of_order() {
        let mut pq = Pq::new(PqType::Max);
        pq.insert(12);
        pq.insert(10);
        assert_eq!(Some(12), pq.remove());
    }

    #[test]
    fn insert_remove_min_many() {
        let mut pq = Pq::new(PqType::Max);
        pq.insert(12);
        pq.insert(10);
        pq.insert(9);
        pq.insert(11);
        assert_eq!(Some(12), pq.remove());
        assert_eq!(Some(11), pq.remove());
        assert_eq!(Some(10), pq.remove());
        assert_eq!(Some(9), pq.remove());
        assert_eq!(None, pq.remove());
    }
}
