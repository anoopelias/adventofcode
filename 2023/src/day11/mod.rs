const DAY: &str = "day11";

use std::{collections::HashMap, time::Instant};

use crate::utils::{
    grid::{Coord, Direction, Grid},
    pq::{Pq, PqType},
    util,
};

#[allow(unused)]
pub(crate) fn solve() -> String {
    let lines = util::lines_in(&format!("../../aoc-files/2023/{}/input1", DAY));
    let time = Instant::now();
    let part1 = part1(&lines);
    let elapsed1 = time.elapsed();

    let time = Instant::now();
    let part2 = part2(&lines);
    let elapsed2 = time.elapsed();
    return format!(
        "result1: {} in {:?} \nresult2: {} in {:?}",
        part1, elapsed1, part2, elapsed2,
    );
}

struct PqItem<T> {
    cell: T,
    distance: usize,
}

impl<T> PqItem<T> {
    fn new(cell: T, distance: usize) -> PqItem<T> {
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
    dist: Distance,
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
            dist: Distance::new(),
        }
    }
}

fn empty_rows(grid: &Grid<Value>) -> Vec<usize> {
    grid.rows()
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|cell| cell.ch == '.'))
        .map(|(p, _)| p)
        .rev()
        .collect::<Vec<usize>>()
}

fn empty_cols(grid: &Grid<Value>) -> Vec<usize> {
    grid.cols()
        .iter()
        .enumerate()
        .filter(|(_, col)| col.iter().all(|cell| cell.ch == '.'))
        .map(|(q, _)| q)
        .rev()
        .collect::<Vec<usize>>()
}

fn parse_lines(grid: &mut Grid<Value>, lines: &Vec<String>) {
    for (p, line) in lines.iter().enumerate() {
        for (q, ch) in line.chars().enumerate() {
            grid.set(&Coord { p, q }, Value::new(ch)).unwrap();
        }
    }
}

fn set_row_distance(grid: &mut Grid<Value>, no_of_duplicates: usize) {
    let empty_rows = empty_rows(&grid);
    empty_rows.iter().for_each(|&p| {
        let row = grid.delete_row(p).unwrap();
        if p > 0 {
            grid.row_mut(p - 1)
                .unwrap()
                .iter_mut()
                .enumerate()
                .for_each(|(q, cell)| {
                    cell.dist.bottom = no_of_duplicates + row.get(q).unwrap().dist.bottom
                });
        }
        if p < grid.m {
            grid.row_mut(p)
                .unwrap()
                .iter_mut()
                .for_each(|cell| cell.dist.top += no_of_duplicates);
        }
    });
}

fn set_col_distance(grid: &mut Grid<Value>, no_of_duplicates: usize) {
    let empty_cols = empty_cols(&grid);
    empty_cols.iter().for_each(|&q| {
        let col = grid.delete_col(q).unwrap();
        if q > 0 {
            grid.col_mut(q - 1)
                .unwrap()
                .iter_mut()
                .enumerate()
                .for_each(|(p, cell)| {
                    cell.dist.right = no_of_duplicates + col.get(p).unwrap().dist.right
                })
        }
        if q < grid.n {
            grid.col_mut(q)
                .unwrap()
                .iter_mut()
                .for_each(|cell| cell.dist.left += no_of_duplicates)
        }
    });
}

fn sum_of_distances(grid: &Grid<Value>) -> usize {
    let all_hash = grid.find_all(&Value::new('#'));
    let mut sum = 0;

    for p in 0..all_hash.len() {
        let hash = all_hash.get(p).unwrap();
        let start = PqItem::new(hash.clone(), 0);
        let mut pq = Pq::new(PqType::Min);
        let mut dist_map: HashMap<Coord, usize> = HashMap::new();

        dist_map.insert(start.cell.coord.clone(), 0);
        pq.push(start);

        // Djikstra's shortest path algo
        while !pq.is_empty() {
            let curr = pq.pop().unwrap();
            let neighbors = grid.neighbors(&curr.cell.coord);
            let curr_dist = *dist_map.get(&curr.cell.coord).unwrap();

            for neighbor in neighbors {
                let ncoord = &neighbor.cell.coord;
                let distance = curr_dist + dist_in(neighbor.dir, &curr.cell.val.dist);
                if !dist_map.contains_key(ncoord) || *dist_map.get(ncoord).unwrap() > distance {
                    dist_map.insert(neighbor.cell.coord, distance);
                    pq.push(PqItem::new(neighbor.cell.clone(), distance));
                }
            }
        }

        for q in p + 1..all_hash.len() {
            let other = all_hash.get(q).unwrap();
            sum += dist_map.get(&other.coord).unwrap();
        }
    }
    sum
}

fn part1(lines: &Vec<String>) -> String {
    let (m, n) = (lines.len(), lines.get(0).unwrap().len());
    let mut grid: Grid<Value> = Grid::new_fill(m, n, &Value::new('.'));
    parse_lines(&mut grid, lines);
    set_row_distance(&mut grid, 2);
    set_col_distance(&mut grid, 2);

    sum_of_distances(&grid).to_string()
}

fn part2(lines: &Vec<String>) -> String {
    let (m, n) = (lines.len(), lines.get(0).unwrap().len());
    let mut grid: Grid<Value> = Grid::new_fill(m, n, &Value::new('.'));
    parse_lines(&mut grid, lines);
    set_row_distance(&mut grid, 1000000);
    set_col_distance(&mut grid, 1000000);

    sum_of_distances(&grid).to_string()
}

fn dist_in(dir: Direction, dist: &Distance) -> usize {
    match dir {
        Direction::Top => dist.top,
        Direction::Bottom => dist.bottom,
        Direction::Left => dist.left,
        Direction::Right => dist.right,
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, DAY};
    use crate::utils::util;

    #[test]
    fn test_part1_sample() {
        let lines = util::lines_in(&format!("../../aoc-files/2023/{}/input", DAY));
        assert_eq!("374", part1(&lines))
    }

    #[test]
    fn test_part1_input() {
        // Too slow to test
        // let lines = util::lines_in(&format!("../../aoc-files/2023/{}/input1", DAY));
        // assert_eq!("9648398", part1(&lines))
    }

    #[test]
    fn test_part2_sample() {
        let lines = util::lines_in(&format!("../../aoc-files/2023/{}/input", DAY));
        assert_eq!("82000210", part2(&lines))
    }

    #[test]
    fn test_part2_input() {
        // Too slow to test
        // let lines = util::lines_in(&format!("../../aoc-files/2023/{}/input1", DAY));
        // assert_eq!("618800410814", part2(&lines))
    }
}
