use crate::util::string_to_nums;

pub(crate) fn solve(lines: Vec<String>) -> String {
    let times = string_to_nums(lines.get(0).unwrap().split(":").nth(1).unwrap());
    let distances = string_to_nums(lines.get(1).unwrap().split(":").nth(1).unwrap());

    let races = times.iter().zip(distances.iter());

    races
        .fold(1, |acc, race| {
            acc * (0..*race.0).filter(|i| (race.0 - i) * i > *race.1).count()
        })
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::day6::one::solve;
    use crate::util;

    #[test]
    fn test_sample() {
        let lines = util::lines_in("./src/day6/input");
        assert_eq!("288", solve(lines))
    }

    #[test]
    fn test_input() {
        let lines = util::lines_in("./src/day6/input1");
        assert_eq!("503424", solve(lines))
    }
}
