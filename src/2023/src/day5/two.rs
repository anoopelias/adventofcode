pub(crate) fn solve(lines: Vec<String>) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use crate::day5::two::solve;
    use crate::util;

    #[test]
    fn test_sample() {
        let lines = util::lines_in("./src/day5/input");
        assert_eq!("30", solve(lines))
    }

    #[test]
    fn test_input() {
        let lines = util::lines_in("./src/day5/input1");
        assert_eq!("6227972", solve(lines))
    }
}
