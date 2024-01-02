use std::collections::HashMap;

pub struct Edge<'a, N> {
    weight: i32,
    node: &'a N,
}

#[derive()]
pub struct Adj<'a, N> {
    list: HashMap<N, Vec<Edge<'a, N>>>,
}

impl<'a, N> Adj<'a, N> {
    fn new() -> Self {
        Adj {
            list: HashMap::new(),
        }
    }
    fn insert(&mut self, node: N) {
        // Implementation of the insert method
    }
}

#[cfg(test)]
mod tests {
    use super::Adj;

    struct Node {
        value: i32,
    }

    #[test]
    fn add_node() {
        let mut adj = Adj::new();
        adj.insert(Node { value: 1 });
    }
}
