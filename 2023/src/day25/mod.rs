const DAY: &str = "day25";

use aoc2023::utils::graph::Graph;
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

fn part1(lines: &Vec<String>) -> String {
    let mut graph = Graph::new();

    lines.iter().for_each(|line| {
        let (node1, nodes_str) = line.split_once(": ").unwrap();
        nodes_str.split(" ").for_each(|node2| {
            graph.add_vertex(node1);
            graph.add_vertex(node2);
            graph.add_edge(node1, node2, 1).unwrap();
        })
    });

    // let nodes = edges
    //     .iter()
    //     .flat_map(|edge| vec![(edge.node1, 1), (edge.node2, 1)])
    //     .collect::<HashMap<&str, i32>>();

    // // Karger's algorithm
    // loop {
    //     let mut edges = edges.clone();
    //     let mut nodes = nodes.clone();
    //     while nodes.len() > 2 {
    //         let random_edge = edges.remove(rand::thread_rng().gen_range(0..edges.len()));
    //         let new_node = random_edge.node1;
    //         edges = edges
    //             .into_iter()
    //             .filter(|edge| !(edge.has(random_edge.node1) && edge.has(random_edge.node2)))
    //             .map(|edge| edge.replace(&random_edge, new_node))
    //             .collect();

    //         let count = nodes.remove(random_edge.node1).unwrap()
    //             + nodes.remove(random_edge.other(random_edge.node1)).unwrap();
    //         nodes.insert(new_node, count);
    //     }

    //     if edges.len() == 3 {
    //         break nodes.values().fold(1, |acc, n| acc * n).to_string();
    //     }
    // }

    "".to_string()
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
