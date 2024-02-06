const DAY: &str = "day22";

use std::{
    cmp,
    collections::{HashMap, HashSet},
    str::FromStr,
    time::Instant,
};

use crate::utils::{
    grid3d::Coord3d,
    pq::{self, Pq},
};
use anyhow::Error;

use crate::utils::util;

#[allow(unused)]
pub(crate) fn solve() -> String {
    let lines = util::lines_in(&format!("../../aoc-files/2023/{}/input1", DAY));
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

    fn top(&self) -> u32 {
        self.coords
            .iter()
            .fold(0, |top, coord| cmp::max(top, coord.r))
    }
}

fn part1(lines: &Vec<String>) -> String {
    let mut bricks = parse_lines(lines);
    let mut coords = prepare_coords(&bricks);
    stabilize(&mut bricks, &mut coords);
    let supporting_bricks = supporting_bricks(&bricks, coords);

    (bricks.len() - supporting_bricks.len()).to_string()
}

fn supporting_bricks(bricks: &Vec<Brick>, coords: HashMap<Coord3d, usize>) -> HashSet<usize> {
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
    supporting_bricks
}

fn stabilize(bricks: &mut Vec<Brick>, coords: &mut HashMap<Coord3d, usize>) {
    let mut tick = true;
    while tick {
        tick = bricks.iter_mut().fold(false, |tick, brick| {
            if brick.can_go_down(&*coords) {
                brick.go_down(coords);
                return true;
            }
            tick
        });
    }
}

fn prepare_coords(bricks: &Vec<Brick>) -> HashMap<Coord3d, usize> {
    let coords: HashMap<Coord3d, usize> = bricks
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
    coords
}

fn parse_lines(lines: &Vec<String>) -> Vec<Brick> {
    lines
        .iter()
        .map(|line| Brick::from_str(line.as_str()).unwrap())
        .collect::<Vec<_>>()
}

struct PqNode {
    brick_num: usize,
    weight: u32,
}

impl Ord for PqNode {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for PqNode {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl PartialEq for PqNode {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl Eq for PqNode {}

fn part2(lines: &Vec<String>) -> String {
    let mut bricks = parse_lines(lines);
    let mut coords = prepare_coords(&bricks);
    stabilize(&mut bricks, &mut coords);

    let by_bricks_map = bricks
        .iter()
        .map(|brick| brick.supported_by(&coords))
        .collect::<Vec<HashSet<usize>>>();

    let to_brick_map = by_bricks_map.iter().enumerate().fold(
        HashMap::new(),
        |mut map: HashMap<usize, HashSet<usize>>, (i, by_bricks)| {
            by_bricks.iter().for_each(|by| match map.get_mut(by) {
                Some(set) => {
                    set.insert(i);
                }
                None => {
                    let mut set = HashSet::new();
                    set.insert(i);
                    map.insert(*by, set);
                }
            });
            map
        },
    );

    let supporting_bricks = supporting_bricks(&bricks, coords);
    let mut sum = 0;

    for brick_num in supporting_bricks {
        let mut pq = Pq::new(pq::PqType::Min);
        let mut disinteg_bricks = HashSet::new();
        disinteg_bricks.insert(brick_num);

        to_brick_map
            .get(&brick_num)
            .unwrap()
            .iter()
            .for_each(|to_brick| {
                let node = PqNode {
                    brick_num: *to_brick,
                    weight: bricks.get(*to_brick).unwrap().top(),
                };
                pq.push(node);
            });

        while !pq.is_empty() {
            let node = pq.pop().unwrap();
            let can_disintegrate = by_bricks_map
                .get(node.brick_num)
                .unwrap()
                .iter()
                .all(|by_brick| disinteg_bricks.contains(by_brick));

            if can_disintegrate {
                disinteg_bricks.insert(node.brick_num);
                match to_brick_map.get(&node.brick_num) {
                    Some(to_bricks) => to_bricks.iter().for_each(|to_brick| {
                        pq.push(PqNode {
                            brick_num: *to_brick,
                            weight: bricks.get(*to_brick).unwrap().top(),
                        })
                    }),
                    None => {}
                }
            }
        }

        sum += disinteg_bricks.len() - 1;
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use clearcheck::assertions::equal::EqualityAssertion;

    use super::{part1, part2, DAY};
    use crate::utils::util;

    #[test]
    fn test_part1_sample() {
        let lines = util::lines_in(&format!("../../aoc-files/2023/{}/input", DAY));
        part1(&lines).should_equal("5");
    }

    #[test]
    fn test_part1_input() {
        let lines = util::lines_in(&format!("../../aoc-files/2023/{}/input1", DAY));
        part1(&lines).should_equal("463");
    }

    #[test]
    fn test_part2_sample() {
        let lines = util::lines_in(&format!("../../aoc-files/2023/{}/input", DAY));
        part2(&lines).should_equal("7");
    }

    #[test]
    fn test_part2_input() {
        let _lines = util::lines_in(&format!("../../aoc-files/2023/{}/input1", DAY));
        // Too slow to test every time
        // part2(&_lines).should_equal("89727");
    }
}
