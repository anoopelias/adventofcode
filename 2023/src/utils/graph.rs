use std::{
    cmp,
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::{BuildHasherDefault, Hash},
};

use anyhow::{Context, Result};
use twox_hash::XxHash64;
use union_find_rs::prelude::*;

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
    pub vertices: HashSet<V>,
    // pub vertices: HashSet<V, BuildHasherDefault<XxHash64>>,
    adj: HashMap<V, HashMap<V, Edge<V>>>,
    // adj: HashMap<V, HashMap<V, Edge<V>>, BuildHasherDefault<XxHash64>>,
}

impl<V> Graph<V> {
    pub fn new() -> Graph<V> {
        Graph {
            vertices: Default::default(),
            adj: Default::default(),
            // vertices: Default::default(),
            // adj: Default::default(),
        }
    }
}

impl<V: PartialEq + Ord + Hash + Copy + Debug + Clone> Graph<V> {
    pub fn add_vertex(&mut self, v: V) {
        self.adj.entry(v).or_insert_with(|| Default::default());
        self.vertices.insert(v);
    }

    pub fn add_edge(&mut self, v1: V, v2: V, weight: i32) -> Result<()> {
        self.adj
            .get_mut(&v1)
            .context("no v1")?
            .insert(v2, Edge { v1, v2, weight });
        self.adj
            .get_mut(&v2)
            .context("no v2")?
            .insert(v1, Edge { v1, v2, weight });
        Ok(())
    }

    pub fn edge(&self, v1: &V, v2: &V) -> Option<&Edge<V>> {
        self.adj.get(v1)?.get(v2)
    }

    fn add_weight(&mut self, v1: &V, v2: &V, weight: i32) -> Option<()> {
        self.adj.get_mut(v1)?.get_mut(v2)?.weight += weight;
        self.adj.get_mut(v2)?.get_mut(v1)?.weight += weight;
        Some(())
    }

    fn move_edge(&mut self, pivot: &V, from: &V, to: &V) -> Option<()> {
        self.adj.get_mut(pivot)?.remove(from).unwrap();
        let weight = self.adj.get_mut(from)?.remove(pivot).unwrap().weight;

        match self.add_weight(pivot, to, weight) {
            Some(_) => {}
            None => self.add_edge(*pivot, *to, weight).unwrap(),
        }
        Some(())
    }

    pub fn merge(&mut self, v1: &V, v2: &V) -> Option<Edge<V>> {
        let pivots = self
            .adj
            .get(v2)
            .unwrap()
            .iter()
            .map(|(pivot, _)| *pivot)
            .filter(|pivot| pivot != v1)
            .collect::<HashSet<V>>();
        // .collect::<HashSet<V, BuildHasherDefault<XxHash64>>>();

        pivots.iter().for_each(|pivot| {
            self.move_edge(pivot, v2, v1).unwrap();
        });

        self.vertices.remove(v2);
        self.adj.remove(v2);
        self.adj.get_mut(v1).unwrap().remove(v2)
    }

    fn cut_phase(&self, start: V) -> (V, V, i32) {
        let mut group = vec![start];
        let mut others = self.vertices.clone();
        others.remove(&start);
        let mut cut_weight = 0;

        while !others.is_empty() {
            let (v, w) = others
                .iter()
                .map(|v_other| {
                    let weight_sum = group
                        .iter()
                        .map(|v_group| match self.edge(&v_group, &v_other) {
                            Some(edge) => edge.weight,
                            None => 0,
                        })
                        .sum::<i32>();
                    (*v_other, weight_sum)
                })
                .reduce(|(v1, w1), (v2, w2)| if w1 > w2 { (v1, w1) } else { (v2, w2) })
                .unwrap();
            cut_weight = w;
            group.push(others.take(&v).unwrap());
        }

        let t = group.pop().unwrap();
        let s = group.pop().unwrap();

        (s, t, cut_weight)
    }

    pub fn mincut(&mut self) -> (Vec<V>, i32) {
        // Stoer–Wagner algorithm
        let mut mincut = i32::MAX;
        let mut start = *self.vertices.iter().next().unwrap();
        let mut cut = HashMap::new();

        let mut uf: DisjointSets<V> = DisjointSets::new();
        self.vertices.iter().for_each(|v| {
            uf.make_set(*v).unwrap();
        });

        let vertices = self.vertices.iter().map(|v| *v).collect::<Vec<V>>();

        while self.vertices.len() > 1 {
            let (s, t, w) = self.cut_phase(start);
            if w < mincut {
                mincut = w;
                vertices.iter().for_each(|v| {
                    cut.insert(*v, uf.find_set(&v).unwrap() == uf.find_set(&t).unwrap());
                });
            }
            self.merge(&s, &t);
            uf.union(&s, &t).unwrap();
            start = s;
        }
        let partition = vertices
            .into_iter()
            .filter(|v| *cut.get(v).unwrap())
            .collect::<Vec<V>>();
        (partition, mincut)
    }
}

#[cfg(test)]
mod tests {
    use super::Graph;

    #[test]
    fn create_graph() {
        let mut g = Graph::new();
        g.add_vertex("kevin");
        g.add_vertex("tom");
        g.add_edge("kevin", "tom", 1).unwrap();
    }

    #[test]
    fn calculate_mincut() {
        let mut g = Graph::new();
        g.add_vertex("1");
        g.add_vertex("2");
        g.add_vertex("3");
        g.add_vertex("4");
        g.add_vertex("5");
        g.add_vertex("6");
        g.add_vertex("7");
        g.add_vertex("8");

        // See sample data from https://blog.thomasjungblut.com/graph/mincut/mincut/
        g.add_edge("1", "5", 3).unwrap();
        g.add_edge("1", "2", 2).unwrap();
        g.add_edge("2", "5", 2).unwrap();
        g.add_edge("2", "6", 2).unwrap();
        g.add_edge("2", "3", 3).unwrap();
        g.add_edge("3", "7", 2).unwrap();
        g.add_edge("3", "4", 4).unwrap();
        g.add_edge("4", "7", 2).unwrap();
        g.add_edge("4", "8", 2).unwrap();
        g.add_edge("5", "6", 3).unwrap();
        g.add_edge("6", "7", 1).unwrap();
        g.add_edge("7", "8", 3).unwrap();

        let (partition, cuts) = g.mincut();
        assert_eq!(4, cuts);
        assert!(partition.len() == 4 || partition.len() == 5);
    }

    #[test]
    fn test_sample_graph() {
        let mut g = Graph::new();
        g.add_vertex("lhk");
        g.add_vertex("rzs");
        g.add_vertex("jqt");

        g.add_edge("lhk", "rzs", 8).unwrap();
        g.add_edge("lhk", "jqt", 1).unwrap();
        g.add_edge("rzs", "jqt", 1).unwrap();

        let (s, v, w) = g.cut_phase("jqt");
        println!("s: {}, v: {}, w: {}", s, v, w);
    }
}
