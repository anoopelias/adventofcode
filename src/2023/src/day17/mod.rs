const DAY: &str = "day17";

use std::{
    cmp,
    collections::{HashMap, HashSet},
    time::Instant,
};

use crate::utils::{
    grid::{Coord, Direction, Grid},
    util::{self, ToGrid},
};

#[allow(unused)]
pub(crate) fn solve() -> String {
    let lines = util::lines_in(&format!("./src/{}/input1", DAY));
    let time = Instant::now();
    let part1 = part1(&lines);
    let elapsed1 = time.elapsed();

    let time = Instant::now();
    let part2 = part2(&lines);
    let elapsed2 = time.elapsed();
    return format!(
        "result1: {} in {:?} \nresult2: {} in {:?}",
        part1, elapsed1, part2, elapsed2,
    );
}

struct Node {
    ch: char,
    heat_loss: Option<usize>,
}

impl Node {
    fn new(ch: char) -> Node {
        Node {
            ch,
            heat_loss: None,
        }
    }
}

fn part1(lines: &Vec<String>) -> String {
    let grid = lines.to_grid_with(|ch| Node::new(ch));
    "".to_string()
}

fn part2(lines: &Vec<String>) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, DAY};
    use crate::utils::util;

    #[test]
    fn test_part1_sample() {
        // let lines = util::lines_in(&format!("./src/{}/input", DAY));
        // assert_eq!("46", part1(&lines))
    }

    #[test]
    fn test_part1_input() {
        // let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        // assert_eq!("8249", part1(&lines))
    }

    #[test]
    fn test_part2_sample() {
        // let lines = util::lines_in(&format!("./src/{}/input", DAY));
        // assert_eq!("51", part2(&lines))
    }

    #[test]
    fn test_part2_input() {
        // let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        // assert_eq!("8444", part2(&lines))
    }
}
