use std::fs;

use super::grid::Grid;

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

pub trait ToGrid<T: Clone + PartialEq> {
    fn to_grid(&self) -> Grid<T>;
}

pub trait ToGridWith<T: Clone + PartialEq> {
    fn to_grid_with(&self, f: impl Fn(char) -> T) -> Grid<T>;
}

impl ToGrid<char> for Vec<String> {
    fn to_grid(&self) -> Grid<char> {
        Grid::with(self.iter().map(|line| line.chars().collect()).collect())
    }
}

impl<T: Clone + PartialEq> ToGridWith<T> for Vec<String> {
    fn to_grid_with(&self, f: impl Fn(char) -> T) -> Grid<T> {
        Grid::with(
            self.iter()
                .map(|line| line.chars().map(|ch| f(ch)).collect::<Vec<T>>())
                .collect(),
        )
    }
}

pub fn transpose<V>(v: Vec<Vec<V>>) -> Vec<Vec<V>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<V>>()
        })
        .collect()
}
