pub(crate) fn solve(lines: Vec<String>) -> String {
    let time = line_to_num(lines.get(0).unwrap());
    let distance = line_to_num(lines.get(1).unwrap());

    (0..time)
        .filter(|i| (time - i) * i > distance)
        .count()
        .to_string()
}

fn line_to_num(line: &str) -> i64 {
    line.split(":")
        .nth(1)
        .unwrap()
        .split(" ")
        .collect::<Vec<&str>>()
        .join("")
        .parse::<i64>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{day6::two::solve, utils::util};

    #[test]
    fn test_sample() {
        let lines = util::lines_in("../../aoc-files/2023/day6/input");
        assert_eq!("71503", solve(lines))
    }

    #[test]
    fn test_input() {
        let lines = util::lines_in("../../aoc-files/2023/day6/input1");
        assert_eq!("32607562", solve(lines))
    }
}
