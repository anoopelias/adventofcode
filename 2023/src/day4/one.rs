use crate::day4::wins_for;

pub(crate) fn solve(lines: Vec<String>) -> String {
    lines
        .iter()
        .map(|line| wins_for(line))
        .filter(|wins| wins > &0)
        .map(|wins| 2i32.pow(wins as u32 - 1))
        .reduce(|a, b| a + b)
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::day4::one::solve;
    use crate::utils::util;

    #[test]
    fn test_sample() {
        let lines = util::lines_in("./src/day4/input");
        assert_eq!("13", solve(lines))
    }

    #[test]
    fn test_input() {
        let lines = util::lines_in("./src/day4/input1");
        assert_eq!("26426", solve(lines))
    }
}
