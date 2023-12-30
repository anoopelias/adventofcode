const DAY: &str = "day13";

use std::{time::Instant, usize};

use itertools::Itertools;

use crate::utils::{
    grid::{Coord, Grid},
    util,
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
    let grids = grids_for(lines);
    grids
        .iter()
        .map(|grid| summary_for(mirror_for(&grid.cols(), 0), mirror_for(&grid.rows(), 0)))
        .sum::<usize>()
        .to_string()
}

fn summary_for(row_mirror: Option<usize>, col_mirror: Option<usize>) -> usize {
    match (row_mirror, col_mirror) {
        (Some(top_rows), None) => top_rows * 100,
        (None, Some(left_cols)) => left_cols,
        _ => unreachable!(),
    }
}

fn grids_for(lines: &Vec<String>) -> Vec<Grid<char>> {
    let line_group = lines.iter().group_by(|s| s.len() == 0);
    let groups: Vec<_> = line_group
        .into_iter()
        .map(|(_, group)| group.collect::<Vec<&String>>())
        .filter(|group| group.len() != 1)
        .collect();

    groups
        .iter()
        .map(|group| {
            let (m, n) = (group.len(), group.get(0).unwrap().len());
            let mut grid = Grid::new_fill(m, n, &'.');
            group.iter().enumerate().for_each(|(p, line)| {
                line.chars()
                    .enumerate()
                    .for_each(|(q, ch)| grid.set(&Coord::new(p, q), ch).unwrap())
            });
            grid
        })
        .collect()
}

fn smudges_at(cells: &Vec<&char>, i: usize) -> usize {
    cells[0..i]
        .iter()
        .rev()
        .zip(cells[i..cells.len()].iter())
        .filter(|(a, b)| a != b)
        .collect::<Vec<_>>()
        .len()
}

fn mirror_for(lines: &Vec<Vec<&char>>, allowed_smudges: usize) -> Option<usize> {
    let v = lines.get(0).unwrap().len();
    (1..v)
        .filter(|&i| {
            lines
                .iter()
                .map(|cells| smudges_at(cells, i))
                .sum::<usize>()
                == allowed_smudges
        })
        .collect::<Vec<_>>()
        .pop()
}

fn part2(lines: &Vec<String>) -> String {
    let grids = grids_for(lines);
    grids
        .iter()
        .map(|grid| summary_for(mirror_for(&grid.cols(), 1), mirror_for(&grid.rows(), 1)))
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, DAY};
    use crate::utils::util;

    #[test]
    fn test_part1_sample() {
        let lines = util::lines_in(&format!("./src/{}/input", DAY));
        assert_eq!("405", part1(&lines))
    }

    #[test]
    fn test_part1_input() {
        let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        assert_eq!("31265", part1(&lines))
    }

    #[test]
    fn test_part1_sample2() {
        let lines = util::lines_in(&format!("./src/{}/input3", DAY));
        assert_eq!("1200", part1(&lines))
    }

    #[test]
    fn test_part2_sample() {
        let lines = util::lines_in(&format!("./src/{}/input", DAY));
        assert_eq!("400", part2(&lines))
    }

    #[test]
    fn test_part2_input() {
        let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        assert_eq!("39359", part2(&lines))
    }
}
