const DAY: &str = "day17";

use std::{
    cmp::{self, Ordering},
    collections::HashMap,
    time::Instant,
};

use crate::utils::{
    grid::{Coord, Direction, Grid, Neighbor},
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
    losses: HashMap<LossType, usize>,
}

impl Node {
    fn new(coord: Coord, from: Direction) -> Node {
        Node {
            from: from,
            coord,
            losses: HashMap::new(),
        }
    }
}

impl Node {
    fn min_loss(&self) -> usize {
        *self.losses.iter().map(|(_, loss)| loss).min().unwrap()
    }
    fn update_loss(&mut self, incoming_dir: &Direction, loss: usize) {
        let loss_type = LossType::from_dir(incoming_dir);
        match self.losses.get(&loss_type) {
            Some(&node_loss) => {
                if loss < node_loss {
                    self.losses.insert(loss_type, loss);
                }
            }
            None => {
                self.losses.insert(loss_type, loss);
            }
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
enum LossType {
    Vertical,
    Horizontal,
}

impl LossType {
    fn from_dir(dir: &Direction) -> LossType {
        match dir {
            Direction::Top => LossType::Vertical,
            Direction::Bottom => LossType::Vertical,
            Direction::Left => LossType::Horizontal,
            Direction::Right => LossType::Horizontal,
        }
    }

    fn opposite(&self) -> LossType {
        match self {
            LossType::Vertical => LossType::Horizontal,
            LossType::Horizontal => LossType::Vertical,
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.losses == other.losses
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.min_loss().cmp(&other.min_loss())
    }
}

fn part1(lines: &Vec<String>) -> String {
    let grid = lines.to_grid_with(|ch| ch.to_digit(10).unwrap() as usize);
    let mut pq = Pq::new(PqType::Min);
    let mut start_node_left = Node::new(Coord::new(0, 0), Direction::Left);
    let mut start_node_top = Node::new(Coord::new(0, 0), Direction::Top);

    start_node_left.losses.insert(LossType::Horizontal, 0);
    start_node_top.losses.insert(LossType::Vertical, 0);

    pq.push(start_node_top);
    pq.push(start_node_left);

    let mut heat_map = HashMap::new();
    heat_map.insert(Coord::new(0, 0), 0);

    while !pq.is_empty() {
        let node = pq.pop().unwrap();
        let node_loss = grid.get(&node.coord).unwrap();
        heat_map.insert(node.coord, node.min_loss() + node_loss);

        let neighbors = grid.neighbors(&node.coord);
        for neighbor in neighbors {
            if !heat_map.contains_key(&neighbor.cell.coord) && neighbor.dir != node.from {
                let loss_type = LossType::from_dir(&neighbor.dir).opposite();
                match node.losses.get(&loss_type) {
                    Some(dir_loss) => {
                        let mut nnode = get_or_create_node(&mut pq, &neighbor);
                        nnode.update_loss(&neighbor.dir, dir_loss + node_loss);
                        pq.push(nnode);
                    }
                    None => {}
                }
            }
        }
    }

    heat_map
        .get(&Coord::new(grid.m - 1, grid.n - 1))
        .unwrap()
        .to_string()
}

fn get_or_create_node(pq: &mut Pq<Node>, neighbor: &Neighbor<&usize>) -> Node {
    match pq.remove_first(|node| {
        node.coord == neighbor.cell.coord && node.from == neighbor.dir.opposite()
    }) {
        Some(node) => node,
        None => Node::new(neighbor.cell.coord, neighbor.dir.opposite()),
    }
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
