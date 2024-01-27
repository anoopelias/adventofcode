use std::{
    cmp,
    collections::{HashMap, HashSet},
    hash::Hash,
    mem::replace,
};

use anyhow::{Context, Result};

pub struct Edge<V> {
    v1: V,
    v2: V,
    pub weight: i32,
}

impl<V> Edge<V> {
    fn new(v1: V, v2: V, weight: i32) -> Edge<V> {
        Edge { v1, v2, weight }
    }
}

impl<V: PartialEq> PartialEq for Edge<V> {
    fn eq(&self, other: &Self) -> bool {
        (self.v1 == other.v1 && self.v2 == other.v2) || (self.v1 == other.v2 && self.v2 == other.v1)
    }
}

impl<V: Eq> Eq for Edge<V> {}

impl<V: Ord + Hash> Hash for Edge<V> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if self.v1 > self.v2 {
            self.v2.hash(state);
            self.v1.hash(state);
        } else {
            self.v1.hash(state);
            self.v2.hash(state);
        }
    }
}

impl<V: PartialEq> Edge<V> {
    fn has(&self, v: &V) -> bool {
        self.v1 == *v || self.v2 == *v
    }
}

pub struct Graph<V> {
    adj: HashMap<V, HashMap<V, i32>>,
}

impl<V> Graph<V> {
    pub fn new() -> Graph<V> {
        Graph {
            adj: HashMap::new(),
        }
    }
}

impl<V: PartialEq + Ord + Hash + Copy> Graph<V> {
    pub fn vertices(&self) -> HashSet<&V> {
        self.adj.keys().collect()
    }

    pub fn add_edge(&mut self, v1: V, v2: V, weight: i32) {
        self.adj
            .entry(v1)
            .or_insert_with(|| HashMap::new())
            .insert(v2, weight);
        self.adj
            .entry(v2)
            .or_insert_with(|| HashMap::new())
            .insert(v1, weight);
    }

    pub fn edge(&self, v1: &V, v2: &V) -> Option<i32> {
        Some(*self.adj.get(v1)?.get(v2)?)
    }

    fn add_weight(&mut self, v1: &V, v2: &V, weight: i32) -> Option<i32> {
        let edge_weight = self.adj.get(v1)?.get(v2)?;
        let sum_weight = edge_weight + weight;
        let _ = replace(self.adj.get_mut(v1).unwrap().get_mut(v2)?, sum_weight);
        let _ = replace(self.adj.get_mut(v2).unwrap().get_mut(v1)?, sum_weight);
        Some(sum_weight)
    }

    fn move_edge(&mut self, pivot: &V, from: &V, to: &V) -> Option<()> {
        self.adj.get_mut(pivot)?.remove(from).unwrap();
        let weight = self.adj.get_mut(from)?.remove(pivot).unwrap();

        match self.add_weight(pivot, to, weight) {
            Some(_) => {}
            None => self.add_edge(*pivot, *to, weight),
        }
        Some(())
    }

    pub fn merge(&mut self, v1: &V, v2: &V) -> i32 {
        let pivots = self
            .adj
            .get(v2)
            .unwrap()
            .iter()
            .map(|(pivot, _)| *pivot)
            .filter(|pivot| pivot != v1)
            .collect::<HashSet<V>>();

        pivots.iter().for_each(|pivot| {
            self.move_edge(pivot, v2, v1).unwrap();
        });

        self.adj.remove(v2);
        self.adj.get_mut(v1).unwrap().remove(v2).unwrap()
    }

    fn cut_phase(&self) -> (V, V, i32) {
        let start = *self.vertices().iter().next().unwrap();
        let mut group = vec![*start];
        let mut others = self.vertices().clone();
        let mut max_weight = 0;
        others.remove(start);

        while !others.is_empty() {
            let (v, w) = others
                .iter()
                .map(|v_other| {
                    let weight_sum = group
                        .iter()
                        .map(|v_group| self.edge(&v_group, &v_other).unwrap_or(0))
                        .sum::<i32>();
                    (*v_other, weight_sum)
                })
                .reduce(|(v1, w1), (v2, w2)| if w1 > w2 { (v1, w1) } else { (v2, w2) })
                .unwrap();
            max_weight = cmp::max(max_weight, w);
            group.push(*others.take(&v).unwrap());
        }

        (group.pop().unwrap(), group.pop().unwrap(), max_weight)
    }

    pub fn mincut(&mut self) -> i32 {
        let mut mincut = i32::MAX;
        while self.vertices().len() > 1 {
            let (s, t, w) = self.cut_phase();
            if w < mincut {
                mincut = w;
            }
            self.merge(&s, &t);
        }
        mincut
    }
}

#[cfg(test)]
mod tests {
    use super::Graph;

    #[test]
    fn calculate_mincut() {
        let mut g = Graph::new();

        // See sample data from https://blog.thomasjungblut.com/graph/mincut/mincut/
        g.add_edge("1", "5", 3);
        g.add_edge("1", "2", 2);
        g.add_edge("2", "5", 2);
        g.add_edge("2", "6", 2);
        g.add_edge("2", "3", 3);
        g.add_edge("3", "7", 2);
        g.add_edge("3", "4", 4);
        g.add_edge("4", "7", 2);
        g.add_edge("4", "8", 2);
        g.add_edge("5", "6", 3);
        g.add_edge("6", "7", 1);
        g.add_edge("7", "8", 3);

        assert_eq!(4, g.mincut());
    }
}
