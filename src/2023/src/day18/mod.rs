const DAY: &str = "day18";

use std::time::Instant;

use crate::utils::util::{self};

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

fn volume_of(lines: &Vec<String>, f: impl Fn(&str) -> (char, i32)) -> i32 {
    let mut q = 0;
    let mut area = 0;
    let mut perimeter = 0;
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

fn line_to_instr(line: &str) -> (char, i32) {
    let mut splits = line.split(" ");
    let dir = splits.next().unwrap().chars().next().unwrap();
    let count = splits.next().unwrap().parse::<i32>().unwrap();
    (dir, count)
}

fn part1(lines: &Vec<String>) -> String {
    volume_of(lines, line_to_instr).to_string()
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
        assert_eq!("62", part1(&lines))
    }

    #[test]
    fn test_part1_input() {
        let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        assert_eq!("56923", part1(&lines))
    }

    #[test]
    fn test_part2_sample() {
        // let lines = util::lines_in(&format!("./src/{}/input", DAY));
        // assert_eq!("94", part2(&lines))
    }

    #[test]
    fn test_part2_input() {
        // let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        // assert_eq!("925", part2(&lines))
    }
}
