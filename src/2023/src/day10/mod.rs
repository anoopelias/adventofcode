use crate::utils::{parser::I32Parser, util};

const DAY: &str = "day10";

#[allow(unused)]
pub(crate) fn solve() -> String {
    let lines = util::lines_in(&format!("./src/{}/input1", DAY));
    return format!("result1: {}\nresult2: {}", part1(&lines), part2(&lines));
}

fn part1(lines: &Vec<String>) -> String {
    let mut sum = 0;
    sum.to_string()
}

fn part2(lines: &Vec<String>) -> String {
    let mut sum = 0;
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, DAY};
    use crate::utils::util;

    #[test]
    fn test_part1_sample() {
        let lines = util::lines_in(&format!("./src/{}/input", DAY));
        //assert_eq!("114", part1(&lines))
    }

    #[test]
    fn test_part1_input() {
        let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        //assert_eq!("1974232246", part1(&lines))
    }

    #[test]
    fn test_sample() {
        let lines = util::lines_in(&format!("./src/{}/input", DAY));
        //assert_eq!("2", part2(&lines))
    }

    #[test]
    fn test_input() {
        let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        //assert_eq!("928", part2(&lines))
    }
}
