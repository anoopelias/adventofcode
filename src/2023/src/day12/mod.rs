#![allow(unused)]
const DAY: &str = "day12";

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
    let mut conditions: Vec<Condition> = vec![];
    for line in lines {
        let (springs, groups) = line.split_in_two(" ");
        conditions.push(Condition::new(springs.to_string(), groups.parse_usize(",")))
    }

    let sum: usize = conditions
        .iter_mut()
        .map(|condition| condition.count())
        .sum();

    sum.to_string()
}

fn part2(lines: &Vec<String>) -> String {
    "".to_string()
}

struct Transition {
    dot: usize,
    hash: usize,
    end: usize,
}

impl Transition {
    fn new(dot: usize, hash: usize, end: usize) -> Transition {
        Transition { dot, hash, end }
    }
}

struct Condition {
    springs: String,
    groups: Vec<usize>,
}

impl Condition {
    fn new(springs: String, groups: Vec<usize>) -> Condition {
        // 1, 1, 3
        // 0 1 2 3 4 5 6 7
        //   # . # . # # #

        // State machine ::
        // st dot ha end
        // 0  0   1  9
        // 1  2   9  9
        // 2  2   3  9
        // 3  4   n  9
        // 4  4   5  9
        // 5  9   6  9
        // 6  9   7  9
        // 7  7   n  8
        // 8
        // 9
        // ? 0,    1
        // ? 0, 1, 2

        let yes = groups.iter().sum::<usize>() + groups.len();
        let no = yes + 1;

        let mut transitions = vec![];
        transitions.push(Transition::new(0, 1, no));

        Condition { springs, groups }
    }

    fn next(&self) -> Option<usize> {
        self.springs.find("?")
    }

    fn is_valid(&self) -> bool {
        let groups: Vec<usize> = self
            .springs
            .parse_separator(".")
            .iter()
            .map(|hs| hs.len())
            .collect();
        if groups.len() != self.groups.len() {
            return false;
        }

        for (i, val) in groups.iter().enumerate() {
            if self.groups[i] != *val {
                return false;
            }
        }
        true
    }

    fn count(&mut self) -> usize {
        match self.next() {
            Some(pos) => {
                self.springs.replace_range(pos..pos + 1, ".");
                let count_dot = self.count();
                self.springs.replace_range(pos..pos + 1, "#");
                let count_hash = self.count();
                self.springs.replace_range(pos..pos + 1, "?");
                count_dot + count_hash
            }
            None => {
                if self.is_valid() {
                    1
                } else {
                    0
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, DAY};
    use crate::utils::util;

    #[test]
    fn test_part1_sample() {
        let lines = util::lines_in(&format!("./src/{}/input", DAY));
        assert_eq!("21", part1(&lines))
    }

    #[test]
    fn test_part1_input() {
        let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        assert_eq!("6827", part1(&lines))
    }

    #[test]
    fn test_part2_sample() {
        // let lines = util::lines_in(&format!("./src/{}/input", DAY));
        // assert_eq!("82000210", part2(&lines))
    }

    #[test]
    fn test_part2_input() {
        // let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        // assert_eq!("618800410814", part2(&lines))
    }
}
