const DAY: &str = "day25";

use crate::utils::util;
use std::time::Instant;

#[allow(unused)]
pub(crate) fn solve() -> String {
    let lines = util::lines_in(&format!("./src/{}/input1", DAY));
    let time = Instant::now();
    let part1 = part1(&lines);
    let elapsed1 = time.elapsed();
    println!("result1: {} in {:?}", part1, elapsed1);

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
    use clearcheck::assertions::equal::EqualityAssertion;

    use super::{part1, part2, DAY};
    use crate::utils::util;

    #[test]
    fn test_part1_sample() {
        // let lines = util::lines_in(&format!("./src/{}/input", DAY));
        // part1(&lines, 7.0, 27.0).should_equal("2");
    }

    #[test]
    fn test_part1_input() {
        // let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        // part1(&lines, 200000000000000.0, 400000000000000.0).should_equal("16050");
    }

    #[test]
    fn test_part2_sample() {
        // let lines = util::lines_in(&format!("./src/{}/input", DAY));
        // part2(&lines).should_equal("47");
    }

    #[test]
    fn test_part2_input() {
        // let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        // part2(&lines).should_equal("669042940632377");
    }
}
