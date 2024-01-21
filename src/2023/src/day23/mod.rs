const DAY: &str = "day23";

use crate::utils::grid::{Direction, Grid, Neighbor};
use crate::utils::util::ToGrid;

use crate::utils::{grid::Coord, util};
use std::collections::HashSet;
use std::time::Instant;

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
    let (grid, start, end) = parse_lines(lines);

    let mut path = HashSet::new();
    path.insert(start);
    longest_path(&grid, start, end, &mut path, false)
        .unwrap()
        .to_string()
}

fn parse_lines(lines: &Vec<String>) -> (Grid<char>, Coord, Coord) {
    let grid = lines.to_grid();
    let start = Coord::new(
        0,
        grid.row(0)
            .unwrap()
            .iter()
            .position(|val| **val == '.')
            .unwrap(),
    );
    let end = Coord::new(
        grid.m - 1,
        grid.row(grid.m - 1)
            .unwrap()
            .iter()
            .position(|val| **val == '.')
            .unwrap(),
    );
    (grid, start, end)
}

fn longest_path(
    grid: &Grid<char>,
    from: Coord,
    to: Coord,
    path: &mut HashSet<Coord>,
    dry: bool,
) -> Option<usize> {
    if from == to {
        return Some(0);
    }

    let mut track = HashSet::new();
    let mut neighbors = valid_neighbors(grid, from, path, &track, dry);

    while neighbors.len() == 1 {
        let neighbor = neighbors.into_iter().next().unwrap();
        let coord = neighbor.cell.coord;
        track.insert(coord);
        if coord == to {
            return Some(track.len());
        }
        neighbors = valid_neighbors(grid, neighbor.cell.coord, path, &track, dry);
    }

    path.extend(track.clone());
    let mut max_len = None;
    for neighbor in neighbors {
        let coord = neighbor.cell.coord;
        path.insert(coord);
        match longest_path(grid, coord, to, path, dry) {
            None => {}
            Some(len) => match max_len {
                None => max_len = Some(len),
                Some(curr_len) => {
                    if len > curr_len {
                        max_len = Some(len);
                    }
                }
            },
        }
        path.remove(&coord);
    }
    path.retain(|elem| !track.contains(elem));
    max_len.map(|len| len + track.len() + 1)
}

fn valid_neighbors<'a>(
    grid: &'a Grid<char>,
    from: Coord,
    path: &HashSet<Coord>,
    track: &HashSet<Coord>,
    dry: bool,
) -> Vec<Neighbor<&'a char>> {
    let val = grid.get(&from).unwrap();
    grid.neighbors(&from)
        .into_iter()
        .filter(|neighbor| {
            *neighbor.cell.val != '#'
                && !path.contains(&neighbor.cell.coord)
                && !track.contains(&neighbor.cell.coord)
                && (dry || check_direction(val, &neighbor.dir))
        })
        .collect::<Vec<Neighbor<&char>>>()
}

fn check_direction(val: &char, dir: &Direction) -> bool {
    match val {
        '^' => *dir == Direction::Top,
        '<' => *dir == Direction::Left,
        'v' => *dir == Direction::Bottom,
        '>' => *dir == Direction::Right,
        _ => true,
    }
}

fn part2(lines: &Vec<String>) -> String {
    let (grid, start, end) = parse_lines(lines);

    let mut path = HashSet::new();
    path.insert(start);
    longest_path(&grid, start, end, &mut path, true)
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use clearcheck::assertions::equal::EqualityAssertion;

    use super::{part1, part2, DAY};
    use crate::utils::util;

    #[test]
    fn test_part1_sample() {
        let lines = util::lines_in(&format!("./src/{}/input", DAY));
        part1(&lines).should_equal("94");
    }

    #[test]
    fn test_part1_input() {
        let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        part1(&lines).should_equal("2438");
    }

    #[test]
    fn test_part2_sample() {
        let lines = util::lines_in(&format!("./src/{}/input", DAY));
        part2(&lines).should_equal("154");
    }

    #[test]
    fn test_part2_input() {
        let _lines = util::lines_in(&format!("./src/{}/input1", DAY));
        part2(&_lines).should_equal("89727");
    }
}
