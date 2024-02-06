const DAY: &str = "day18";

use std::time::Instant;

use crate::utils::{
    string::WrapperRemover,
    util::{self},
};

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

fn volume_of(lines: &Vec<String>, f: impl Fn(&str) -> (char, i64)) -> i64 {
    let mut q = 0i64;
    let mut area = 0i64;
    let mut perimeter = 0i64;
    for line in lines {
        let (dir, count) = f(&line);
        perimeter += count;
        match dir {
            'R' => {
                q += count;
            }
            'L' => {
                q -= count;
            }
            'U' => {
                area -= q * count;
            }
            'D' => {
                area += q * count;
            }
            _ => panic!("Invalid direction"),
        };
    }
    area + perimeter / 2 + 1
}

fn line_to_instr_part1(line: &str) -> (char, i64) {
    let mut splits = line.split(" ");
    let dir = splits.next().unwrap().chars().next().unwrap();
    let count = splits.next().unwrap().parse::<i64>().unwrap();
    (dir, count)
}

fn line_to_instr_part2(line: &str) -> (char, i64) {
    let mut splits = line.split(" ");
    // ignore instruction parameters
    splits.next();
    splits.next();

    let color = splits.next().unwrap().remove_wrapping();
    let dir = match color.chars().rev().next().unwrap() {
        '0' => 'R',
        '1' => 'D',
        '2' => 'L',
        '3' => 'U',
        _ => panic!("Invalid color"),
    };

    let count = i64::from_str_radix(&color[1..color.len() - 1], 16).unwrap();
    (dir, count)
}

fn part1(lines: &Vec<String>) -> String {
    volume_of(lines, line_to_instr_part1).to_string()
}

fn part2(lines: &Vec<String>) -> String {
    volume_of(lines, line_to_instr_part2).to_string()
}

#[cfg(test)]
mod tests {
    use super::{line_to_instr_part2, part1, part2, DAY};
    use crate::utils::util;

    #[test]
    fn test_part1_sample() {
        let lines = util::lines_in(&format!("../../aoc-files/2023/{}/input", DAY));
        assert_eq!("62", part1(&lines))
    }

    #[test]
    fn test_part1_input() {
        let lines = util::lines_in(&format!("../../aoc-files/2023/{}/input1", DAY));
        assert_eq!("56923", part1(&lines))
    }

    #[test]
    fn test_line_to_instr_part2() {
        assert_eq!(('R', 461937), line_to_instr_part2("R 6 (#70c710)"));
        assert_eq!(('D', 56407), line_to_instr_part2("D 5 (#0dc571)"));
    }

    #[test]
    fn test_part2_sample() {
        let lines = util::lines_in(&format!("../../aoc-files/2023/{}/input", DAY));
        assert_eq!("952408144115", part2(&lines));
    }

    #[test]
    fn test_part2_input() {
        let lines = util::lines_in(&format!("../../aoc-files/2023/{}/input1", DAY));
        assert_eq!("66296566363189", part2(&lines));
    }
}
