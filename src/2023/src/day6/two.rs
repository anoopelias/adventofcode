pub(crate) fn solve(lines: Vec<String>) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use crate::day6::two::solve;
    use crate::util;

    #[test]
    fn test_sample() {
        let lines = util::lines_in("./src/day6/input");
        assert_eq!("46", solve(lines))
    }

    #[test]
    fn test_input() {
        let lines = util::lines_in("./src/day6/input1");
        assert_eq!("12634632", solve(lines))
    }
}
