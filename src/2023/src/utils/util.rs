use std::fs;

pub(crate) fn lines_in(path: &str) -> Vec<String> {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");

    let mut lines = vec![];
    let splits = contents.split("\n");
    for part in splits {
        lines.push(String::from(part));
    }

    lines.pop();

    lines
}
