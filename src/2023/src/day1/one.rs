use std::cmp;

pub(crate) fn solve(lines: Vec<String>) -> String {

    let mut max = 0;
    let mut curr = 0;

    for line in lines {
        let n = match line.parse::<i32>() {
            Result::Ok(n) => n,
            _ => {
                max = cmp::max(max, curr);
                curr = 0;
                continue
            }
        };

        curr += n;
    }

    max = cmp::max(max, curr);

    return max.to_string()
}

#[cfg(test)]
mod tests {
    use crate::day1::one::solve;
    use crate::util;

    #[test]
    fn test_sample() {
        let lines = util::lines_in("./src/day1/input");
        assert_eq!("24000", solve(lines))
    }

    #[test]
    fn test_input() {
        let lines = util::lines_in("./src/day1/input1");
        assert_eq!("65912", solve(lines))
    }
}