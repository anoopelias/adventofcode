use crate::utils::{iter::Pairs, parser::I32Parser, util};

const DAY: &str = "day9";

#[allow(unused)]
pub(crate) fn solve() -> String {
    let lines = util::lines_in(&format!("./src/{}/input1", DAY));
    return format!("result1: {}\nresult2: {}", part1(&lines), part2(&lines));
}

fn find_sum<V, O>(lines: &Vec<String>, v: V, o: O) -> String
where
    V: Fn(&Vec<i32>) -> &i32,
    O: Fn(i32, i32) -> i32,
{
    let mut sum = 0;
    for line in lines.iter() {
        let nums = line.parse_i32();

        let mut ends = vec![*v(&nums)];
        let mut row = nums.clone();
        while row.iter().filter(|n| n == &&0).count() != row.len() {
            let pairs = row.to_pairs();
            row = pairs.iter().map(|(p, q)| *q - *p).collect();
            ends.push(*v(&row));
        }

        ends.reverse();
        sum += ends.iter().fold(0, |tip, next| o(tip, *next));
    }

    sum.to_string()
}

fn part1(lines: &Vec<String>) -> String {
    find_sum(lines, |n| n.last().unwrap(), |tip, next| tip + next)
}

fn part2(lines: &Vec<String>) -> String {
    find_sum(lines, |n| n.first().unwrap(), |tip, next| next - tip)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, DAY};
    use crate::utils::util;

    #[test]
    fn test_part1_sample() {
        let lines = util::lines_in(&format!("./src/{}/input", DAY));
        assert_eq!("114", part1(&lines))
    }

    #[test]
    fn test_part1_input() {
        let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        assert_eq!("1974232246", part1(&lines))
    }

    #[test]
    fn test_sample() {
        let lines = util::lines_in(&format!("./src/{}/input", DAY));
        assert_eq!("2", part2(&lines))
    }

    #[test]
    fn test_input() {
        let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        assert_eq!("928", part2(&lines))
    }
}
