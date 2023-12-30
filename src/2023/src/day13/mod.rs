#![allow(unused)]
const DAY: &str = "day13";

use std::time::Instant;

use itertools::Itertools;

use crate::utils::{
    grid::{Coord, Grid, GridCell},
    parser::{I32Parser, SeparatorParser, TwoSplitter, UsizeParser},
    util,
};

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
    let line_group = lines.iter().group_by(|s| s.len() == 0);
    let groups: Vec<_> = line_group
        .into_iter()
        .map(|(key, group)| group.collect::<Vec<&String>>())
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
        .map(|grid| match mirrors_for(&grid.cols()) {
            Some(row_num) => row_num * 100,
            None => match mirrors_for(&grid.rows()) {
                Some(col_num) => col_num,
                None => unreachable!(),
            },
        })
        .sum::<usize>()
        .to_string()
}

fn is_mirror_at(row: &Vec<GridCell<&char>>, q: usize) -> bool {
    row[0..q]
        .iter()
        .rev()
        .zip(row[q..row.len()].iter())
        .filter(|(a, b)| a.val != b.val)
        .collect::<Vec<_>>()
        .len()
        == 0
}

fn mirrors_for(rows: &Vec<Vec<GridCell<&char>>>) -> Option<usize> {
    rows.iter()
        .map(|row| {
            (1..row.len())
                .filter(|&q| is_mirror_at(row, q))
                .collect::<Vec<usize>>()
        })
        .reduce(|mut acc, mirrors| {
            acc.retain(|p| mirrors.contains(p));
            acc
        })
        .unwrap()
        .pop()
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
        // let lines = util::lines_in(&format!("./src/{}/input", DAY));
        // assert_eq!("525152", part2(&lines))
    }

    #[test]
    fn test_part2_input() {
        // let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        // assert_eq!("1537505634471", part2(&lines))
    }
}
