pub(crate) fn solve(lines: Vec<String>) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use crate::day2::one::solve;
    use crate::util;

    #[test]
    fn test_sample() {
        let lines = util::lines_in("./src/day2/input");
        assert_eq!("142", solve(lines))
    }

    #[test]
    fn test_input() {
        let lines = util::lines_in("./src/day2/input1");
        assert_eq!("54159", solve(lines))
    }
}
