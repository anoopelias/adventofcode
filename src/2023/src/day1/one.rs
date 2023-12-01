use std::cmp;

pub(crate) fn solve(lines: Vec<String>) -> i32 {

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

    return max;
}