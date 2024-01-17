const DAY: &str = "day21";

use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash,
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
    let part2 = part2(&lines, 26501365);
    let elapsed2 = time.elapsed();
    return format!(
        "result1: {} in {:?} \nresult2: {} in {:?}",
        part1, elapsed1, part2, elapsed2,
    );
}

fn part1(lines: &Vec<String>) -> String {
    let grid = lines.to_grid();
    grid.bfs(grid.find('S').unwrap(), |neighbor| {
        neighbor.cell.val == &'.' || neighbor.cell.val == &'S'
    })
    .into_iter()
    .filter(|(_, dist)| *dist <= 64 && dist % 2 == 0)
    .count()
    .to_string()
}

fn part2(lines: &Vec<String>, steps: usize) -> String {
    let grid = lines.to_grid();
    let dist_map = grid.bfs(grid.find('S').unwrap(), |neighbor| {
        neighbor.cell.val == &'.' || neighbor.cell.val == &'S'
    });

    let width = grid.m;
    let half_width = width / 2;
    let n = (steps - half_width) / width;

    assert_eq!(steps, (n * width) + half_width);
    let mut evens = dist_map
        .values()
        .filter(|&&value| value % 2 == 0)
        .collect::<Vec<_>>()
        .len();

    let mut odds = dist_map
        .values()
        .filter(|&&value| value % 2 == 1)
        .collect::<Vec<_>>()
        .len();

    let mut odd_cuts = dist_map
        .values()
        .filter(|&&value| value % 2 == 1 && value as usize > half_width)
        .collect::<Vec<_>>()
        .len();

    let mut even_cuts = dist_map
        .values()
        .filter(|&&value| value % 2 == 0 && value as usize > half_width)
        .collect::<Vec<_>>()
        .len();

    if steps % 2 == 1 {
        (odds, evens) = (evens, odds);
        (odd_cuts, even_cuts) = (even_cuts, odd_cuts);
    }

    let n_sq = n * n;
    let np1 = n + 1;
    let np1_sq = np1 * np1;

    let result = (evens * np1_sq) + (odds * n_sq) - (np1 * even_cuts) + (n * odd_cuts);
    result.to_string()
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
        let lines = util::lines_in(&format!("./src/{}/input", DAY));
        assert_eq!("", part2(&lines, 500));
    }

    #[test]
    fn test_part2_input() {
        let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        assert_eq!("597102954104491", part2(&lines, 26501365));
    }
}
