pub(crate) fn solve(lines: Vec<String>) -> String {
    let mut sum = 0;

    for line in lines {
        let mut first = None;
        let mut last = None;

        for ch in line.chars() {
            if let Some(n) = to_num(ch) {
                if first == None {
                    first = Some(n);
                } else {
                    last = Some(n);
                }
            }
        }

        if last == None {
            last = first;
        }

        sum += first.unwrap() * 10 + last.unwrap();
    }

    sum.to_string()
}

fn to_num(ch: char) -> Option<i32> {
    if ch >= '0' && ch <= '9' {
        return Some(ch.to_digit(10).unwrap() as i32);
    }
    None
}
#[cfg(test)]
mod tests {
    use crate::day1::one::solve;
    use crate::util;

    #[test]
    fn test_sample() {
        let lines = util::lines_in("./src/day1/input3");
        assert_eq!("142", solve(lines))
    }

    #[test]
    fn test_input() {
        let lines = util::lines_in("./src/day1/input1");
        assert_eq!("54159", solve(lines))
    }
}
