pub(crate) fn solve(lines: Vec<String>) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use crate::day5::one::solve;
    use crate::util;

    #[test]
    fn test_sample() {
        let lines = util::lines_in("./src/day5/input");
        assert_eq!("13", solve(lines))
    }

    #[test]
    fn test_input() {
        let lines = util::lines_in("./src/day5/input1");
        assert_eq!("26426", solve(lines))
    }
}
