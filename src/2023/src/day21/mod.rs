const DAY: &str = "day21";

use std::{collections::HashSet, time::Instant};

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
    let mut tiles = HashSet::new();
    tiles.insert(start_cell);

    for _ in 0..64 {
        tiles = tiles
            .iter()
            .flat_map(|tile| {
                grid.neighbors(tile)
                    .iter()
                    .filter(|neighbor| neighbor.cell.val == &'.' || neighbor.cell.val == &'S')
                    .map(|neighbor| neighbor.cell.coord)
                    .collect::<Vec<_>>()
            })
            .collect::<HashSet<_>>()
    }

    tiles.len().to_string()
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
