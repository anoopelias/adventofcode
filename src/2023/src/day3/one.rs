pub(crate) fn solve(lines: Vec<String>) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use crate::day3::one::solve;
    use crate::util;

    #[test]
    fn test_sample() {
        let lines = util::lines_in("./src/day3/input");
        assert_eq!("8", solve(lines))
    }

    #[test]
    fn test_input() {
        let lines = util::lines_in("./src/day3/input1");
        assert_eq!("2447", solve(lines))
    }
}
