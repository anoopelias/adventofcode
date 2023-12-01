use std::cmp;

pub(crate) fn solve(lines: Vec<String>) -> String {

    let mut sums = vec![];
    let mut curr = 0;

    for line in lines {
        let n = match line.parse::<i32>() {
            Result::Ok(n) => n,
            _ => {
                sums.push(curr);
                curr = 0;
                continue
            }
        };

        curr += n;
    }

    sums.sort();

    return format!("{}", sums.pop().unwrap() + sums.pop().unwrap() + sums.pop().unwrap())
}