use std::fs;

use super::grid::{Coord, Grid};

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

impl ToGrid<char> for Vec<String> {
    fn to_grid(&self) -> Grid<char> {
        let (m, n) = (self.len(), self.get(0).unwrap().len());
        let mut grid = Grid::new_fill(m, n, &'.');
        self.iter().enumerate().for_each(|(p, line)| {
            line.chars()
                .enumerate()
                .for_each(|(q, ch)| grid.set(&Coord::new(p, q), ch).unwrap())
        });
        grid
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
