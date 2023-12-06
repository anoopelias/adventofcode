pub(crate) fn solve(lines: Vec<String>) -> String {
    let mut sum = 0;

    for line in lines {
        let mut first = None;
        let mut last = None;

        for (i, _) in line.chars().enumerate() {
            if let Some(n) = to_num(&line[i..]) {
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

fn to_num(str: &str) -> Option<i32> {
    let ch = str.chars().nth(0).unwrap();

    if ch >= '0' && ch <= '9' {
        return Some(ch.to_digit(10).unwrap() as i32);
    }

    if str.starts_with("zero") {
        Some(0)
    } else if str.starts_with("one") {
        Some(1)
    } else if str.starts_with("two") {
        Some(2)
    } else if str.starts_with("three") {
        Some(3)
    } else if str.starts_with("four") {
        Some(4)
    } else if str.starts_with("five") {
        Some(5)
    } else if str.starts_with("six") {
        Some(6)
    } else if str.starts_with("seven") {
        Some(7)
    } else if str.starts_with("eight") {
        Some(8)
    } else if str.starts_with("nine") {
        Some(9)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::day1::two::solve;
    use crate::utils::util;

    #[test]
    fn test_sample() {
        let lines = util::lines_in("./src/day1/input");
        assert_eq!("281", solve(lines))
    }

    #[test]
    fn test_input() {
        let lines = util::lines_in("./src/day1/input1");
        assert_eq!("53866", solve(lines))
    }
}
