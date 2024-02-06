const DAY: &str = "day25";

use crate::utils::graph::Graph;

use crate::utils::util;
use std::time::Instant;

#[allow(unused)]
pub(crate) fn solve() -> String {
    let lines = util::lines_in(&format!("../../aoc-files/2023/{}/input1", DAY));
    let time = Instant::now();
    let part1 = part1(&lines);
    let elapsed1 = time.elapsed();
    return format!("result1: {} in {:?}", part1, elapsed1);
}

fn part1(lines: &Vec<String>) -> String {
    let mut graph = Graph::new();
    lines.iter().for_each(|line| {
        let (node1, nodes_str) = line.split_once(": ").unwrap();
        graph.add_vertex(node1);
        nodes_str.split(" ").for_each(|node2| {
            graph.add_vertex(node2);
            graph.add_edge(node1, node2, 1).unwrap();
        })
    });

    let len = graph.vertices.len();
    let (partition, mincut) = graph.mincut();
    assert_eq!(3, mincut);
    (partition.len() * (len - partition.len())).to_string()
}

#[cfg(test)]
mod tests {
    use clearcheck::assertions::equal::EqualityAssertion;

    use super::{part1, DAY};
    use crate::utils::util;

    #[test]
    fn test_part1_sample() {
        let lines = util::lines_in(&format!("../../aoc-files/2023/{}/input", DAY));
        part1(&lines).should_equal("54");
    }

    #[test]
    fn test_part1_input() {
        let _lines = util::lines_in(&format!("../../aoc-files/2023/{}/input1", DAY));
        // Too slow to test
        // part1(&_lines).should_equal("558376");
    }
}
