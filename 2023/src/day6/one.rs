use crate::utils::parser::{I32Parser, SeparatorParser};

pub(crate) fn solve(lines: Vec<String>) -> String {
    let times = line_to_nums(lines.get(0).unwrap());
    let distances = line_to_nums(lines.get(1).unwrap());

    let races = times.iter().zip(distances.iter());

    races
        .fold(1, |acc, race| {
            acc * (0..*race.0).filter(|i| (race.0 - i) * i > *race.1).count()
        })
        .to_string()
}

fn line_to_nums(line: &str) -> Vec<i32> {
    line.parse_separator(":").get(1).unwrap().parse_i32()
}

#[cfg(test)]
mod tests {
    use crate::day6::one::solve;
    use crate::utils::util;

    #[test]
    fn test_sample() {
        let lines = util::lines_in("../../aoc-files/2023/day6/input");
        assert_eq!("288", solve(lines))
    }

    #[test]
    fn test_input() {
        let lines = util::lines_in("../../aoc-files/2023/day6/input1");
        assert_eq!("503424", solve(lines))
    }
}
