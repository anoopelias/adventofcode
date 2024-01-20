const DAY: &str = "day22";

use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
    time::Instant,
};

use crate::utils::grid3d::Coord3d;
use anyhow::Error;

use crate::utils::util;

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

#[derive(Debug)]
struct Brick {
    coords: Vec<Coord3d>,
}

impl FromStr for Brick {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start_str, end_str) = s.split_once("~").ok_or(Error::msg("parse error"))?;
        let start_coord = Coord3d::from_str(start_str)?;
        let end_coord = Coord3d::from_str(end_str)?;

        Ok(Brick {
            coords: start_coord.coords_till(&end_coord),
        })
    }
}

impl Brick {
    fn get_num(&self, coords: &HashMap<Coord3d, usize>) -> usize {
        *coords.get(&self.coords[0]).unwrap()
    }

    fn can_go_down(&self, coords: &HashMap<Coord3d, usize>) -> bool {
        let num = self.get_num(coords);
        self.coords.iter().all(|coord| match coord.below() {
            None => false,
            Some(coord_below) => match coords.get(&coord_below) {
                None => true,
                Some(brick_num) => *brick_num == num,
            },
        })
    }

    fn go_down(&mut self, coords: &mut HashMap<Coord3d, usize>) {
        let num = self.get_num(coords);
        self.coords.iter().for_each(|coord1| {
            coords.remove(coord1).unwrap();
        });
        self.coords = self
            .coords
            .iter()
            .map(|coord| coord.below().unwrap())
            .collect();
        self.coords.iter().for_each(|coord| {
            coords.insert(*coord, num);
        });
    }

    fn supported_by(&self, coords: &HashMap<Coord3d, usize>) -> HashSet<usize> {
        let num = self.get_num(coords);
        self.coords
            .iter()
            .filter_map(|coord| coord.below())
            .filter_map(|coord_below| coords.get(&coord_below).cloned())
            // For vertically aligned bricks
            .filter(|box_num| *box_num != num)
            .collect::<HashSet<_>>()
    }
}

fn part1(lines: &Vec<String>) -> String {
    let mut bricks = lines
        .iter()
        .map(|line| Brick::from_str(line.as_str()).unwrap())
        .collect::<Vec<_>>();

    let mut coords: HashMap<Coord3d, usize> = bricks
        .iter()
        .enumerate()
        .flat_map(|(n, brick)| {
            brick
                .coords
                .iter()
                .map(|coord| (*coord, n))
                .collect::<Vec<_>>()
        })
        .collect();

    let mut tick = true;
    while tick {
        tick = bricks.iter_mut().fold(false, |tick, brick| {
            if brick.can_go_down(&coords) {
                brick.go_down(&mut coords);
                return true;
            }
            tick
        });
    }

    let supporting_bricks = bricks
        .iter()
        .map(|brick| {
            let supported_by = brick.supported_by(&coords);
            if supported_by.len() == 1 {
                Some(supported_by.into_iter().next().unwrap())
            } else {
                None
            }
        })
        .filter(|s| s.is_some())
        .map(|s| s.unwrap())
        .collect::<HashSet<usize>>();

    (bricks.len() - supporting_bricks.len()).to_string()
}

fn part2(_lines: &Vec<String>) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::{part1, DAY};
    use crate::utils::util;

    #[test]
    fn test_part1_sample() {
        let lines = util::lines_in(&format!("./src/{}/input", DAY));
        assert_eq!("5", part1(&lines))
    }

    #[test]
    fn test_part1_input() {
        let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        // 660 too high
        assert_eq!("463", part1(&lines))
    }

    #[test]
    fn test_part2_sample() {
        // let lines = util::lines_in(&format!("./src/{}/input", DAY));
        // assert_eq!("145", part2(&lines))
    }

    #[test]
    fn test_part2_input() {
        // let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        // assert_eq!("259356", part2(&lines))
    }
}
