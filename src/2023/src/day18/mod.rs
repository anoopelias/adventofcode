const DAY: &str = "day18";

use std::time::Instant;

use crate::utils::{
    grid::{Coord, Direction, Grid},
    util::{self},
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

fn part1(lines: &Vec<String>) -> String {
    let mut dig_plan = Vec::new();

    let (mut curr_p, mut curr_q) = (0, 0);
    let (mut min_p, mut min_q) = (0, 0);
    let (mut max_p, mut max_q) = (0, 0);

    for line in lines {
        let mut splits = line.split(" ");
        let dir = splits.next().unwrap();
        let count = splits.next().unwrap().parse::<i32>().unwrap();
        let dir = match dir {
            "R" => {
                curr_q += count;
                Direction::Right
            }
            "L" => {
                curr_q -= count;
                Direction::Left
            }
            "U" => {
                curr_p -= count;
                Direction::Top
            }
            "D" => {
                curr_p += count;
                Direction::Bottom
            }
            _ => panic!("Invalid direction"),
        };

        min_p = min_p.min(curr_p);
        min_q = min_q.min(curr_q);

        max_p = max_p.max(curr_p);
        max_q = max_q.max(curr_q);
        dig_plan.push((dir, count));
    }

    let m = (max_p - min_p + 1) as usize;
    let n = (max_q - min_q + 1) as usize;
    let mut grid = Grid::new_fill(m, n, &None);
    let grid_coord = GridCoord { min_p, min_q };

    let mut curr = grid_coord.to_grid_coord(&Coord::new(0, 0));
    let mut prev_dir = dig_plan.get(dig_plan.len() - 1).unwrap().0.opposite();
    for (dir, count) in dig_plan {
        for _ in 0..count {
            grid.set(&curr, Some(CellConnect::from_dirs(prev_dir, dir)))
                .unwrap();
            curr = grid.next(&curr, &dir).unwrap().coord;
            prev_dir = dir.opposite();
        }
    }

    // Ray tracing
    let mut count = 0;
    for row in grid.rows() {
        let mut ray = State::Outside(Break::None);
        for cell in row {
            match cell {
                Some(connect) => {
                    count += 1;
                    ray = ray.next(connect);
                }
                None => match ray {
                    State::Inside(_) => {
                        count += 1;
                    }
                    _ => {}
                },
            }
        }
    }

    count.to_string()
}

trait ToGridCoord {
    fn to_grid_coord(&self, min_coord: &Coord) -> Coord;
}

struct GridCoord {
    min_p: i32,
    min_q: i32,
}

impl ToGridCoord for GridCoord {
    fn to_grid_coord(&self, coord: &Coord) -> Coord {
        Coord::new(
            (coord.p as i32 - self.min_p) as usize,
            (coord.q as i32 - self.min_q) as usize,
        )
    }
}

#[derive(Clone, PartialEq)]
enum CellConnect {
    LeftToRight,
    LeftToTop,
    LeftToBottom,
    TopToBottom,
    TopToRight,
    RightToBottom,
}

impl CellConnect {
    fn from_dirs(from: Direction, to: Direction) -> CellConnect {
        match (from, to) {
            (Direction::Left, Direction::Right) | (Direction::Right, Direction::Left) => {
                CellConnect::LeftToRight
            }
            (Direction::Left, Direction::Top) | (Direction::Top, Direction::Left) => {
                CellConnect::LeftToTop
            }
            (Direction::Left, Direction::Bottom) | (Direction::Bottom, Direction::Left) => {
                CellConnect::LeftToBottom
            }
            (Direction::Top, Direction::Bottom) | (Direction::Bottom, Direction::Top) => {
                CellConnect::TopToBottom
            }
            (Direction::Top, Direction::Right) | (Direction::Right, Direction::Top) => {
                CellConnect::TopToRight
            }
            (Direction::Right, Direction::Bottom) | (Direction::Bottom, Direction::Right) => {
                CellConnect::RightToBottom
            }
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq)]
enum State {
    Outside(Break),
    Inside(Break),
}

#[derive(PartialEq)]
enum Break {
    None,
    Up,
    Down,
}

impl State {
    fn next(&self, input: &CellConnect) -> State {
        match self {
            State::Outside(Break::None) => match input {
                CellConnect::TopToBottom => State::Inside(Break::None),
                CellConnect::TopToRight => State::Outside(Break::Up),
                CellConnect::RightToBottom => State::Outside(Break::Down),
                _ => State::Outside(Break::None),
            },
            State::Outside(Break::Up) => match input {
                CellConnect::LeftToTop => State::Outside(Break::None),
                CellConnect::LeftToBottom => State::Inside(Break::None),
                _ => State::Outside(Break::Up),
            },
            State::Outside(Break::Down) => match input {
                CellConnect::LeftToTop => State::Inside(Break::None),
                CellConnect::LeftToBottom => State::Outside(Break::None),
                _ => State::Outside(Break::Down),
            },
            State::Inside(Break::None) => match input {
                CellConnect::TopToBottom => State::Outside(Break::None),
                CellConnect::TopToRight => State::Inside(Break::Up),
                CellConnect::RightToBottom => State::Inside(Break::Down),
                _ => State::Inside(Break::None),
            },
            State::Inside(Break::Up) => match input {
                CellConnect::LeftToTop => State::Inside(Break::None),
                CellConnect::LeftToBottom => State::Outside(Break::None),
                _ => State::Inside(Break::Up),
            },
            State::Inside(Break::Down) => match input {
                CellConnect::LeftToTop => State::Outside(Break::None),
                CellConnect::LeftToBottom => State::Inside(Break::None),
                _ => State::Inside(Break::Down),
            },
        }
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
        let lines = util::lines_in(&format!("./src/{}/input", DAY));
        assert_eq!("62", part1(&lines))
    }

    #[test]
    fn test_part1_input() {
        let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        assert_eq!("56923", part1(&lines))
    }

    #[test]
    fn test_part2_sample() {
        // let lines = util::lines_in(&format!("./src/{}/input", DAY));
        // assert_eq!("94", part2(&lines))
    }

    #[test]
    fn test_part2_input() {
        // let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        // assert_eq!("925", part2(&lines))
    }
}
