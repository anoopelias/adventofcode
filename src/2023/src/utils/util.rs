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
