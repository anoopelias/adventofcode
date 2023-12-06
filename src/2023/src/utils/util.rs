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

pub(crate) fn neighbors(p: usize, q: usize, m: usize, n: usize) -> Vec<(usize, usize)> {
    let mut neighbors = vec![];
    if p > 0 {
        if q > 0 {
            neighbors.push((p - 1, q - 1));
        }
        neighbors.push((p - 1, q));
        if q < n - 1 {
            neighbors.push((p - 1, q + 1));
        }
    }
    if q > 0 {
        neighbors.push((p, q - 1));
    }
    if q < n - 1 {
        neighbors.push((p, q + 1));
    }

    if p < m - 1 {
        if q > 0 {
            neighbors.push((p + 1, q - 1));
        }
        neighbors.push((p + 1, q));
        if q < n - 1 {
            neighbors.push((p + 1, q + 1));
        }
    }

    neighbors
}

#[allow(unused)]
pub(crate) fn neighbors_vh(p: usize, q: usize, m: usize, n: usize) -> Vec<(usize, usize)> {
    let mut neighbors = vec![];
    if p > 0 {
        neighbors.push((p - 1, q));
    }
    if q > 0 {
        neighbors.push((p, q - 1));
    }
    if q < n - 1 {
        neighbors.push((p, q + 1));
    }

    if p < m - 1 {
        neighbors.push((p + 1, q));
    }

    neighbors
}

pub(crate) fn string_to_i64_nums(str: &str) -> Vec<i64> {
    str.trim()
        .split(" ")
        .collect::<Vec<_>>()
        .iter()
        .map(|s| s.trim())
        .filter(|s| s.len() > 0)
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}
