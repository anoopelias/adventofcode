const DAY: &str = "day21";

use std::time::Instant;

use crate::utils::grid::{Coord, Grid};
use crate::utils::util::{self, ToGrid};

#[allow(unused)]
pub(crate) fn solve() -> String {
    let lines = util::lines_in(&format!("../../aoc-files/2023/{}/input1", DAY));
    let time = Instant::now();
    let part1 = part1(&lines, 64);
    let elapsed1 = time.elapsed();

    let time = Instant::now();
    let part2 = part2(&lines, 26501365);
    let elapsed2 = time.elapsed();
    return format!(
        "result1: {} in {:?} \nresult2: {} in {:?}",
        part1, elapsed1, part2, elapsed2,
    );
}

fn part1(lines: &Vec<String>, steps: usize) -> String {
    let grid = lines.to_grid();
    let count = count_reachable_plots(&grid, steps);
    count.to_string()
}

fn count_reachable_plots(grid: &Grid<char>, steps: usize) -> usize {
    let parity = steps % 2;
    let center = Coord::new((grid.m() - 1) / 2, (grid.n() - 1) / 2);
    grid.bfs(center, |neighbor| neighbor.cell.val != &'#')
        .into_iter()
        .filter(|(_, dist)| *dist <= steps && dist % 2 == parity)
        .count()
}

fn part2(lines: &Vec<String>, steps: usize) -> String {
    let mut grid = lines.to_grid();
    grid.expand(5, 5);
    let width = grid.m;
    let half_width = width / 2;
    let n = (steps - half_width) / width;
    assert_eq!(steps, (n * width) + half_width);

    let center = Coord::new((grid.m() - 1) / 2, (grid.n() - 1) / 2);
    let dist_map = grid.bfs(center, |neighbor| neighbor.cell.val != &'#');

    // n = 1
    let r1 = dist_map
        .iter()
        .filter(|(_, dist)| **dist <= half_width && **dist % 2 == 1)
        .count();

    // n = 1
    let r2 = dist_map
        .iter()
        .filter(|(_, dist)| **dist <= (half_width + width) && **dist % 2 == 0)
        .count();

    // n = 2
    let r3 = dist_map
        .iter()
        .filter(|(_, dist)| **dist <= (half_width + (2 * width)) && **dist % 2 == 1)
        .count();

    let c = r1;

    let ap_bp_c = r2;
    let ap_b = ap_bp_c - c;
    let a4p_b2p_c = r3;
    let a4p_b2p = a4p_b2p_c - c;
    let a2 = a4p_b2p - (2 * ap_b);

    let a = a2 / 2;
    let b = ap_b - a;

    // a n^2 + b n + c
    let result = (a * n * n) + (b * n) + c;

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, DAY};
    use crate::utils::util;

    #[test]
    fn test_part1_sample() {
        let lines = util::lines_in(&format!("../../aoc-files/2023/{}/input", DAY));
        assert_eq!("16", part1(&lines, 6));
    }

    #[test]
    fn test_part1_input() {
        let lines = util::lines_in(&format!("../../aoc-files/2023/{}/input1", DAY));
        assert_eq!("3585", part1(&lines, 64));
    }

    #[test]
    fn test_part2_input() {
        let lines = util::lines_in(&format!("../../aoc-files/2023/{}/input1", DAY));
        assert_eq!("597102953699891", part2(&lines, 26501365));

        // too low:     597102911216785
        // not correct: 597102911419086
        //              597102953699891
        // too hi :     597102954104491
    }
}
