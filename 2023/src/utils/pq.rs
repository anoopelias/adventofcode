use std::cmp::Ordering;

pub enum PqType {
    Min,
    #[allow(unused)]
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

    pub fn push(&mut self, value: T) {
        self.values.push(value);
        self.swim(self.values.len() - 1);
    }

    pub fn pop(&mut self) -> Option<T> {
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

    #[allow(unused)]
    pub fn remove_one_if(&mut self, f: impl Fn(&T) -> bool) -> Option<T> {
        let value = self
            .values
            .iter()
            .enumerate()
            .filter(|(_, t)| f(t))
            .collect::<Vec<_>>()
            .pop();

        if value.is_none() {
            return None;
        }

        let (i, _) = value.unwrap();
        let t = self.values.swap_remove(i);

        // See `changeKey` here: https://algs4.cs.princeton.edu/24pq/IndexMinPQ.java.html
        if i < self.values.len() {
            self.swim(i);
            self.sink(i);
        }
        Some(t)
    }

    #[allow(unused)]
    pub fn has(&mut self, f: impl Fn(&T) -> bool) -> bool {
        self.values
            .iter()
            .filter(|t| f(t))
            .collect::<Vec<_>>()
            .len()
            != 0
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
    use rand::seq::SliceRandom;
    use rand::Rng;

    use super::Pq;
    use super::PqType;

    #[test]
    fn insert_remove() {
        let mut pq = Pq::new(PqType::Min);
        pq.push(10);
        assert_eq!(Some(10), pq.pop());
    }

    #[test]
    fn insert_remove_min_in_order() {
        let mut pq = Pq::new(PqType::Min);
        pq.push(12);
        pq.push(10);
        assert_eq!(Some(10), pq.pop());
    }

    #[test]
    fn insert_remove_min_out_of_order() {
        let mut pq = Pq::new(PqType::Min);
        pq.push(10);
        pq.push(12);
        assert_eq!(Some(10), pq.pop());
    }

    #[test]
    fn insert_remove_max_in_order() {
        let mut pq = Pq::new(PqType::Max);
        pq.push(10);
        pq.push(12);
        assert_eq!(Some(12), pq.pop());
    }

    #[test]
    fn insert_remove_max_out_of_order() {
        let mut pq = Pq::new(PqType::Max);
        pq.push(12);
        pq.push(10);
        assert_eq!(Some(12), pq.pop());
    }

    #[test]
    fn insert_remove_max_many() {
        let mut pq = Pq::new(PqType::Max);
        pq.push(12);
        pq.push(10);
        pq.push(9);
        pq.push(11);
        assert_eq!(Some(12), pq.pop());
        assert_eq!(Some(11), pq.pop());
        assert_eq!(Some(10), pq.pop());
        assert_eq!(Some(9), pq.pop());
        assert_eq!(None, pq.pop());
    }

    #[test]
    fn increase_key() {
        let mut pq = Pq::new(PqType::Max);
        pq.push(12);
        pq.push(10);
        assert_eq!(Some(12), pq.pop());
    }

    #[test]
    fn ten_thousand_random_numbers() {
        let mut rng = rand::thread_rng();
        let mut nums: Vec<usize> = vec![];

        for _ in 0..100000 {
            nums.push(rng.gen());
        }

        let mut pq = Pq::new(PqType::Max);
        for num in nums.clone() {
            pq.push(num);
        }

        // Remove a random 50 numbers
        nums.shuffle(&mut rng);
        for _ in 0..500 {
            let n = nums.pop().unwrap();
            pq.remove_one_if(|&num| num == n);
        }

        nums.sort();
        for _ in 0..nums.len() {
            assert_eq!(nums.pop(), pq.pop());
        }

        assert!(pq.is_empty());
    }

    #[test]
    fn test_remove_last_element() {
        let mut pq = Pq::new(PqType::Min);
        pq.push(2);
        pq.push(3);

        assert_eq!(Some(3), pq.remove_one_if(|&n| n == 3));
    }
}
