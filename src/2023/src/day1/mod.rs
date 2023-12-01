use crate::util;

pub(crate) mod one;
pub(crate) mod two;

pub(crate) fn solve() -> String {
    let lines = util::lines_in("./src/day1/input1");

    let result1 = one::solve(lines.clone());
    let result2 = two::solve(lines);
    return format!("result1: {}\nresult2: {}", result1, result2);
}
