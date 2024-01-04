const DAY: &str = "day17";

use std::{
    cmp,
    collections::{BinaryHeap, HashMap, HashSet},
    time::Instant,
};

use crate::utils::{
    grid::{Coord, Direction, Grid},
    pq::{Pq, PqType},
    util::{self, ToGrid, ToGridWith},
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
    coord: Coord,
    from: Direction,
    lr_loss: Option<usize>,
    tb_loss: Option<usize>,
}

impl Node {
    fn new(coord: Coord, from: Direction) -> Node {
        Node {
            from: from,
            coord,
            lr_loss: None,
            tb_loss: None,
        }
    }
}

impl Node {
    fn eq_lr_loss(&self, other: &Node) -> bool {
        match (self.lr_loss, other.lr_loss) {
            (Some(self_loss), Some(other_loss)) => self_loss == other_loss,
            _ => false,
        }
    }
    fn eq_tb_loss(&self, other: &Node) -> bool {
        match (self.tb_loss, other.tb_loss) {
            (Some(self_loss), Some(other_loss)) => self_loss == other_loss,
            _ => false,
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.eq_lr_loss(other) && self.eq_tb_loss(other)
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        min_loss(self).cmp(&min_loss(other))
    }
}

fn min_loss(node: &Node) -> usize {
    match (node.lr_loss, node.tb_loss) {
        (Some(lr_loss), Some(tb_loss)) => {
            if lr_loss < tb_loss {
                lr_loss
            } else {
                tb_loss
            }
        }
        (Some(lr_loss), None) => lr_loss,
        (None, Some(tb_loss)) => tb_loss,
        _ => unreachable!(),
    }
}

fn part1(lines: &Vec<String>) -> String {
    let grid = lines.to_grid_with(|ch| ch.to_digit(10));
    let mut pq = BinaryHeap::new();
    pq.push(Node::new(Coord::new(0, 0), Direction::Left));

    let mut heat_map = HashMap::new();
    heat_map.insert(Coord::new(0, 0), 0);

    while !pq.is_empty() {
        let curr = pq.pop().unwrap();
        heat_map.insert(curr.coord, min_loss(&curr));

        let neighbors = grid.neighbors(&curr.coord);

        for neighbor in neighbors {
            // pq.retain(f)

            // let mut nnode =
            // match neighbor.dir {
            //     Direction::Left | Direction::Right =>

            // }
        }
    }

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
