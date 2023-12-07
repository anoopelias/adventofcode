pub(crate) fn solve(lines: Vec<String>) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use crate::day7::one::solve;
    use crate::utils::util;

    #[test]
    fn test_sample() {
        let lines = util::lines_in("./src/day7/input");
        assert_eq!("288", solve(lines))
    }

    #[test]
    fn test_input() {
        let lines = util::lines_in("./src/day7/input1");
        assert_eq!("503424", solve(lines))
    }
}
