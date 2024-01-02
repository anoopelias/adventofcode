const DAY: &str = "day16";

use std::{
    cmp,
    collections::{HashMap, HashSet},
    time::Instant,
};

use crate::utils::{
    grid::{Coord, Direction, Grid},
    util::{self, ToGrid},
};

#[allow(unused)]
pub(crate) fn solve() -> String {
    let lines = util::lines_in(&format!("./src/{}/input1", DAY));
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

fn part1(lines: &Vec<String>) -> String {
    let grid = lines.to_grid();
    let mut visited = HashMap::new();
    visit(&grid, &Coord::new(0, 0), Direction::Right, &mut visited);
    visited.len().to_string()
}

fn visit(
    grid: &Grid<char>,
    coord: &Coord,
    dir: Direction,
    visited: &mut HashMap<Coord, HashSet<Direction>>,
) {
    if !visited.contains_key(coord) {
        visited.insert(*coord, HashSet::new());
    } else {
        let dirs = visited.get_mut(coord).unwrap();
        if dirs.contains(&dir) {
            return;
        }
    }

    let dirs = visited.get_mut(coord).unwrap();
    dirs.insert(dir.clone());

    match grid.get(coord).unwrap() {
        &'.' => visit_next(grid, coord, dir, visited),
        &'-' => {
            match dir {
                Direction::Left | Direction::Right => visit_next(grid, coord, dir, visited),
                Direction::Top | Direction::Bottom => {
                    visit_next(grid, coord, Direction::Left, visited);
                    visit_next(grid, coord, Direction::Right, visited);
                }
            };
        }
        &'|' => {
            match dir {
                Direction::Top | Direction::Bottom => visit_next(grid, coord, dir, visited),
                Direction::Left | Direction::Right => {
                    visit_next(grid, coord, Direction::Top, visited);
                    visit_next(grid, coord, Direction::Bottom, visited);
                }
            };
        }
        &'/' => match dir {
            Direction::Bottom => visit_next(grid, coord, Direction::Left, visited),
            Direction::Right => visit_next(grid, coord, Direction::Top, visited),
            Direction::Top => visit_next(grid, coord, Direction::Right, visited),
            Direction::Left => visit_next(grid, coord, Direction::Bottom, visited),
        },
        &'\\' => match dir {
            Direction::Bottom => visit_next(grid, coord, Direction::Right, visited),
            Direction::Left => visit_next(grid, coord, Direction::Top, visited),
            Direction::Top => visit_next(grid, coord, Direction::Left, visited),
            Direction::Right => visit_next(grid, coord, Direction::Bottom, visited),
        },

        _ => unreachable!(),
    }
}

fn visit_next(
    grid: &Grid<char>,
    coord: &Coord,
    dir: Direction,
    visited: &mut HashMap<Coord, HashSet<Direction>>,
) {
    match grid.next(coord, &dir) {
        Ok(next) => visit(grid, &next.coord, dir, visited),
        Err(_) => {}
    }
}

fn part2(lines: &Vec<String>) -> String {
    let grid = lines.to_grid();
    let mut best = 0;

    for p in 0..grid.m {
        let mut visited = HashMap::new();
        visit(&grid, &Coord::new(p, 0), Direction::Right, &mut visited);
        best = cmp::max(best, visited.len());

        let mut visited = HashMap::new();
        visit(
            &grid,
            &Coord::new(p, grid.n - 1),
            Direction::Left,
            &mut visited,
        );
        best = cmp::max(best, visited.len());
    }

    for q in 0..grid.n {
        let mut visited = HashMap::new();
        visit(&grid, &Coord::new(0, q), Direction::Bottom, &mut visited);
        best = cmp::max(best, visited.len());

        let mut visited = HashMap::new();
        visit(
            &grid,
            &Coord::new(grid.n - 1, q),
            Direction::Bottom,
            &mut visited,
        );
        best = cmp::max(best, visited.len());
    }
    best.to_string()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, DAY};
    use crate::utils::util;

    #[test]
    fn test_part1_sample() {
        let lines = util::lines_in(&format!("./src/{}/input", DAY));
        assert_eq!("46", part1(&lines))
    }

    #[test]
    fn test_part1_input() {
        let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        assert_eq!("8249", part1(&lines))
    }

    #[test]
    fn test_part2_sample() {
        let lines = util::lines_in(&format!("./src/{}/input", DAY));
        assert_eq!("51", part2(&lines))
    }

    #[test]
    fn test_part2_input() {
        // Too slow to run all the time
        // let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        // assert_eq!("8444", part2(&lines))
    }
}
