use crate::day4::wins_for;

pub(crate) fn solve(lines: Vec<String>) -> String {
    let mut sum = 0;
    for line in lines.iter() {
        let wins = wins_for(line);

        let num: i32 = 2;
        if wins > 0 {
            sum += num.pow((wins - 1).try_into().unwrap());
        }
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use crate::day4::one::solve;
    use crate::util;

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
