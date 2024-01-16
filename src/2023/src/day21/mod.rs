const DAY: &str = "day21";

use std::{
    collections::{HashMap, HashSet, VecDeque},
    time::Instant,
};

use crate::{
    day3::grid::{self, Grid},
    utils::util::{self, ToGrid},
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
    let grid = lines.to_grid();
    let start_cell = grid.find('S').unwrap();
    let mut dist_map = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((start_cell, 0));

    // bfs
    while !queue.is_empty() {
        let (curr, dist) = queue.pop_front().unwrap();

        if !dist_map.contains_key(&curr) {
            dist_map.insert(curr, dist);

            grid.neighbors(&curr)
                .iter()
                .filter(|neighbor| neighbor.cell.val == &'.' || neighbor.cell.val == &'S')
                .for_each(|neighbor| queue.push_back((neighbor.cell.coord, dist + 1)));
        }
    }

    dist_map
        .into_iter()
        .filter(|(_, dist)| *dist <= 64 && dist % 2 == 0)
        .count()
        .to_string()
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
        assert_eq!("42", part1(&lines))
    }

    #[test]
    fn test_part1_input() {
        let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        assert_eq!("3585", part1(&lines))
    }

    #[test]
    fn test_part2_sample() {
        // let lines = util::lines_in(&format!("./src/{}/input", DAY));
        // assert_eq!("167409079868000", part2(&lines));
    }

    #[test]
    fn test_part2_input() {
        // let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        // assert_eq!("131029523269531", part2(&lines));
    }
}
