const DAY: &str = "day11";

use crate::utils::{
    grid::{Coord, Grid, GridCell},
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
            grid.set(&Coord { p, q }, Some(ch)).unwrap();
        }
    }
}

fn part1(lines: &Vec<String>) -> String {
    let (m, n) = (lines.len(), lines.get(0).unwrap().len());
    let mut grid: Grid<char> = Grid::new(m, n);
    parse_lines(&mut grid, lines);

    duplicate_row(&mut grid);
    duplicate_column(&mut grid);
    let all = grid.all();

    let all_hash: Vec<&GridCell<char>> =
        all.iter().filter(|cell| cell.val.unwrap() == '#').collect();

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
    let all = grid.all();

    let mut indices = vec![];

    for p in 0..grid.m {
        let hash_count = all
            .iter()
            .filter(|cell| cell.coord.p == p && cell.val.unwrap() == '#')
            .collect::<Vec<&GridCell<char>>>()
            .len();

        if hash_count == 0 {
            indices.push(p);
        }
    }

    indices.reverse();
    indices.into_iter().for_each(|p| grid.duplicate_row(p))
}

fn duplicate_column(grid: &mut Grid<char>) {
    let all = grid.all();
    let mut indices = vec![];

    for q in 0..grid.n {
        let hash_count = all
            .iter()
            .filter(|cell| cell.coord.q == q && cell.val.unwrap() == '#')
            .collect::<Vec<&GridCell<char>>>()
            .len();

        if hash_count == 0 {
            indices.push(q);
        }
    }

    indices.reverse();
    indices.iter().for_each(|q| grid.duplicate_column(*q));
}

fn parse_lines_part2(grid: &mut Grid<Value>, lines: &Vec<String>) {
    for (p, line) in lines.iter().enumerate() {
        for (q, ch) in line.chars().enumerate() {
            grid.set(&Coord { p, q }, Some(Value::new(ch))).unwrap();
        }
    }
}

fn part2(lines: &Vec<String>) -> String {
    let (m, n) = (lines.len(), lines.get(0).unwrap().len());
    let mut grid: Grid<Value> = Grid::new(m, n);
    parse_lines_part2(&mut grid, lines);

    "".to_string()
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

impl Value {
    fn new(ch: char) -> Value {
        Value {
            ch: ch,
            distance: Distance::new(),
        }
    }
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
        let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        assert_eq!("9648398", part1(&lines))
    }

    #[test]
    fn test_part2_sample() {
        let lines = util::lines_in(&format!("./src/{}/input", DAY));
        assert_eq!("4", part2(&lines))
    }

    #[test]
    fn test_part2_input() {
        // let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        // assert_eq!("527", part2(&lines))
    }
}
