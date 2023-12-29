#![allow(unused)]
const DAY: &str = "day13";

use std::time::Instant;

use crate::utils::{
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
    "".to_string()
}

fn part2(lines: &Vec<String>) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::{count, part1, part2, DAY};
    use crate::utils::util;

    #[test]
    fn test_part1_sample() {
        // let lines = util::lines_in(&format!("./src/{}/input", DAY));
        // assert_eq!("21", part1(&lines))
    }

    #[test]
    fn test_part1_input() {
        // let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        // assert_eq!("6827", part1(&lines))
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
