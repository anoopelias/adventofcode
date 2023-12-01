use std::fs;

pub(crate) mod one;
pub(crate) mod two;

pub(crate) fn solve() -> String {
    let path = "./src/day1/input1";
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    let mut lines = vec![];
    let splits = contents.split("\n");
    for part in splits {
        lines.push(String::from(part));
    }

    let result1 = one::solve(lines.clone());
    let result2 = two::solve(lines);
    return format!("result1: {}\nresult2: {}", result1, result2);
}
