const DAY: &str = "day11";

use crate::utils::{
    grid::{Coord, Grid, GridCell},
    pq::{Pq, PqType},
    util,
};

#[allow(unused)]
pub(crate) fn solve() -> String {
    let lines = util::lines_in(&format!("./src/{}/input1", DAY));
    return format!("result1: {}\nresult2: {}", part1(&lines), part2(&lines));
}

fn parse_lines(grid: &mut Grid<char>, lines: &Vec<String>) {
    for (p, line) in lines.iter().enumerate() {
        for (q, ch) in line.chars().enumerate() {
            grid.set(&Coord { p, q }, ch).unwrap();
        }
    }
}

fn part1(lines: &Vec<String>) -> String {
    let (m, n) = (lines.len(), lines.get(0).unwrap().len());
    let mut grid: Grid<char> = Grid::new_fill(m, n, &'.');
    parse_lines(&mut grid, lines);

    duplicate_row(&mut grid);
    duplicate_column(&mut grid);
    let all_hash = grid.find_all(&'#');

    let mut sum = 0;
    for p in 0..all_hash.len() {
        let bfs_result = grid.bfs(&all_hash[p].coord);
        for q in p..all_hash.len() {
            sum += bfs_result.dist_map.get(&all_hash[q].coord).unwrap();
        }
    }

    sum.to_string()
}

fn duplicate_row(grid: &mut Grid<char>) {
    let indices = grid
        .rows()
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|cell| cell.val == &'.'))
        .map(|(p, _)| p)
        .rev()
        .collect::<Vec<usize>>();
    indices
        .into_iter()
        .for_each(|q| grid.duplicate_row(q).unwrap());
}

fn duplicate_column(grid: &mut Grid<char>) {
    let indices = grid
        .cols()
        .iter()
        .enumerate()
        .filter(|(_, col)| col.iter().all(|cell| cell.val == &'.'))
        .map(|(q, _)| q)
        .rev()
        .collect::<Vec<usize>>();
    indices
        .iter()
        .for_each(|q| grid.duplicate_column(*q).unwrap());
}

const DUPLICATES: usize = 2;
struct PqItem<T> {
    cell: GridCell<T>,
    distance: usize,
}

impl<T> PqItem<T> {
    fn new(cell: GridCell<T>, distance: usize) -> PqItem<T> {
        PqItem { cell, distance }
    }
}

impl<T> Eq for PqItem<T> {}

impl<T> PartialEq for PqItem<T> {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl<T> PartialOrd for PqItem<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl<T> Ord for PqItem<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
    }
}

#[derive(Clone)]
struct Distance {
    left: usize,
    right: usize,
    top: usize,
    bottom: usize,
}

impl Distance {
    fn new() -> Distance {
        Distance {
            left: 1,
            right: 1,
            top: 1,
            bottom: 1,
        }
    }
}

#[derive(Clone)]
struct Value {
    ch: char,
    distance: Distance,
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.ch == other.ch
    }
}

impl Value {
    fn new(ch: char) -> Value {
        Value {
            ch: ch,
            distance: Distance::new(),
        }
    }
}

fn empty_rows(grid: &Grid<Value>) -> Vec<usize> {
    grid.rows()
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|cell| cell.val.ch == '.'))
        .map(|(p, _)| p)
        .rev()
        .collect::<Vec<usize>>()
}

fn empty_cols(grid: &Grid<Value>) -> Vec<usize> {
    grid.cols()
        .iter()
        .enumerate()
        .filter(|(_, col)| col.iter().all(|cell| cell.val.ch == '.'))
        .map(|(q, _)| q)
        .rev()
        .collect::<Vec<usize>>()
}

fn parse_lines_part2(grid: &mut Grid<Value>, lines: &Vec<String>) {
    for (p, line) in lines.iter().enumerate() {
        for (q, ch) in line.chars().enumerate() {
            grid.set(&Coord { p, q }, Value::new(ch)).unwrap();
        }
    }
}

fn set_row_distance(grid: &mut Grid<Value>) {
    let empty_rows = empty_rows(&grid);
    empty_rows.iter().for_each(|&p| {
        if p > 0 {
            grid.row_mut(p - 1)
                .unwrap()
                .iter_mut()
                .for_each(|cell| cell.val.distance.bottom = DUPLICATES);
        }
        if p < grid.m - 1 {
            grid.row_mut(p + 1)
                .unwrap()
                .iter_mut()
                .for_each(|cell| cell.val.distance.top = DUPLICATES);
        }
    });
    empty_rows
        .iter()
        .rev()
        .for_each(|&p| grid.delete_row(p).unwrap());
}

fn set_col_distance(grid: &mut Grid<Value>) {
    let empty_cols = empty_cols(&grid);
    empty_cols.iter().for_each(|&q| {
        if q > 0 {
            grid.col_mut(q - 1)
                .unwrap()
                .iter_mut()
                .for_each(|cell| cell.val.distance.right = DUPLICATES)
        }
        if q < grid.n - 1 {
            grid.col_mut(q + 1)
                .unwrap()
                .iter_mut()
                .for_each(|cell| cell.val.distance.left = DUPLICATES)
        }
    });
    empty_cols
        .iter()
        .rev()
        .for_each(|&q| grid.delete_col(q).unwrap());
}

fn part2(lines: &Vec<String>) -> String {
    let (m, n) = (lines.len(), lines.get(0).unwrap().len());
    let mut grid: Grid<Value> = Grid::new_fill(m, n, &Value::new('.'));
    parse_lines_part2(&mut grid, lines);
    set_row_distance(&mut grid);
    set_col_distance(&mut grid);

    let hashes = grid.find_all(&Value::new('#'));

    for hash in hashes {
        let mut pq = Pq::new(PqType::Min);
        let start = PqItem::new(hash, 0);
        pq.insert(start);
    }

    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, DAY};
    use crate::utils::util;

    #[test]
    fn test_part1_sample() {
        let lines = util::lines_in(&format!("./src/{}/input", DAY));
        assert_eq!("374", part1(&lines))
    }

    #[test]
    fn test_part1_input() {
        // Too slow to test
        // let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        // assert_eq!("9648398", part1(&lines))
    }

    #[test]
    fn test_part2_sample() {
        // let lines = util::lines_in(&format!("./src/{}/input", DAY));
        // assert_eq!("4", part2(&lines))
    }

    #[test]
    fn test_part2_input() {
        // let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        // assert_eq!("527", part2(&lines))
    }
}
