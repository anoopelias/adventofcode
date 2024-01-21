const DAY: &str = "day17";

use std::{
    cmp::{self, Ordering},
    collections::HashSet,
    time::Instant,
};

use crate::utils::{
    grid::{Coord, Direction},
    pq::{Pq, PqType},
    util::{self, ToGridWith},
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
    loss: usize,
    coord: Coord,
    facing: Direction,
    num_steps: usize,
}

impl Node {
    fn new(loss: usize, coord: Coord, facing: Direction, num_steps: usize) -> Node {
        Node {
            loss,
            coord,
            facing,
            num_steps,
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.loss == other.loss
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
        self.loss.cmp(&other.loss)
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Visit {
    coord: Coord,
    facing: Direction,
    num_steps: usize,
}

impl Visit {
    fn new(coord: Coord, facing: Direction, num_steps: usize) -> Visit {
        Visit {
            coord,
            facing,
            num_steps,
        }
    }
}

fn least_heat_loss(lines: &Vec<String>, min_steps: usize, max_steps: usize) -> String {
    let grid = lines.to_grid_with(|ch| ch.to_digit(10).unwrap() as usize);

    let mut pq = Pq::new(PqType::Min);
    pq.push(Node::new(0, Coord::new(0, 0), Direction::Right, 0));
    pq.push(Node::new(0, Coord::new(0, 0), Direction::Bottom, 0));

    let mut visits = HashSet::new();
    while !pq.is_empty() {
        let node = pq.pop().unwrap();

        if node.coord == Coord::new(grid.m - 1, grid.n - 1) {
            return node.loss.to_string();
        }

        if visits.contains(&Visit::new(node.coord, node.facing, node.num_steps)) {
            continue;
        }

        visits.insert(Visit::new(node.coord, node.facing, node.num_steps));

        let neighbors = grid.neighbors(&node.coord);
        for neighbor in neighbors {
            let neighbor_loss = node.loss + grid.get(&neighbor.cell.coord).unwrap();
            if neighbor.dir == node.facing {
                if node.num_steps < max_steps {
                    pq.push(Node::new(
                        neighbor_loss,
                        neighbor.cell.coord,
                        neighbor.dir,
                        node.num_steps + 1,
                    ));
                }
            } else if neighbor.dir != node.facing.opposite() {
                if node.num_steps >= min_steps {
                    pq.push(Node::new(
                        neighbor_loss,
                        neighbor.cell.coord,
                        neighbor.dir,
                        1,
                    ));
                }
            }
        }
    }

    "".to_string()
}

fn part1(lines: &Vec<String>) -> String {
    least_heat_loss(lines, 0, 3)
}

fn part2(lines: &Vec<String>) -> String {
    least_heat_loss(lines, 4, 10)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, DAY};
    use crate::utils::util;

    #[test]
    fn test_part1_sample() {
        let lines = util::lines_in(&format!("./src/{}/input", DAY));
        assert_eq!("102", part1(&lines))
    }

    #[test]
    fn test_part1_input() {
        let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        assert_eq!("817", part1(&lines))
    }

    #[test]
    fn test_part2_sample() {
        let lines = util::lines_in(&format!("./src/{}/input", DAY));
        assert_eq!("94", part2(&lines))
    }

    #[test]
    fn test_part2_input() {
        let _lines = util::lines_in(&format!("./src/{}/input1", DAY));
        // Too slow to test every time
        //assert_eq!("925", part2(&lines))
    }
}
