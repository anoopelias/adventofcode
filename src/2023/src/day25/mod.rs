const DAY: &str = "day25";

use rand::Rng;

use crate::utils::util;
use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

#[allow(unused)]
pub(crate) fn solve() -> String {
    let lines = util::lines_in(&format!("./src/{}/input1", DAY));
    let time = Instant::now();
    let part1 = part1(&lines);
    let elapsed1 = time.elapsed();
    return format!("result1: {} in {:?}", part1, elapsed1);
}

#[derive(Clone)]
struct Edge<'a> {
    node1: &'a str,
    node2: &'a str,
}

impl<'a> Edge<'a> {
    fn has(&self, node: &str) -> bool {
        self.node1 == node || self.node2 == node
    }

    fn other(&self, node: &'a str) -> &'a str {
        if node == self.node1 {
            self.node2
        } else {
            self.node1
        }
    }

    fn replace(self, other_edge: &Edge<'a>, new_node: &'a str) -> Edge<'a> {
        if self.has(other_edge.node1) && self.has(other_edge.node2) {
            Edge {
                node1: new_node,
                node2: new_node,
            }
        } else if self.has(other_edge.node1) {
            Edge {
                node1: new_node,
                node2: self.other(other_edge.node1),
            }
        } else if self.has(other_edge.node2) {
            Edge {
                node1: new_node,
                node2: self.other(other_edge.node2),
            }
        } else {
            self
        }
    }
}

fn part1(lines: &Vec<String>) -> String {
    let edges = lines
        .iter()
        .flat_map(|line| {
            let (node1, nodes_str) = line.split_once(": ").unwrap();
            nodes_str.split(" ").map(|node2| Edge { node1, node2 })
        })
        .collect::<Vec<_>>();

    let nodes = edges
        .iter()
        .flat_map(|edge| vec![edge.node1, edge.node2])
        .collect::<HashSet<&str>>()
        .iter()
        .map(|node| (*node, 1))
        .collect::<HashMap<_, _>>();

    // Karger's algorithm
    loop {
        let mut edges = edges.clone();
        let mut nodes = nodes.clone();
        while nodes.len() > 2 {
            let random_edge = edges.remove(rand::thread_rng().gen_range(0..edges.len()));
            let new_node = random_edge.node1;
            edges = edges
                .into_iter()
                .map(|edge| edge.replace(&random_edge, new_node))
                .filter(|edge| edge.node1 != edge.node2)
                .collect();

            let count = nodes.remove(random_edge.node1).unwrap()
                + nodes.remove(random_edge.other(random_edge.node1)).unwrap();
            nodes.insert(new_node, count);
        }

        if edges.len() == 3 {
            break nodes.values().fold(1, |acc, n| acc * n).to_string();
        }
    }
}

#[cfg(test)]
mod tests {
    use clearcheck::assertions::equal::EqualityAssertion;

    use super::{part1, DAY};
    use crate::utils::util;

    #[test]
    fn test_part1_sample() {
        let lines = util::lines_in(&format!("./src/{}/input", DAY));
        part1(&lines).should_equal("54");
    }

    #[test]
    fn test_part1_input() {
        let _lines = util::lines_in(&format!("./src/{}/input1", DAY));
        // Too slow to test always
        // part1(&_lines).should_equal("558376");
    }
}
