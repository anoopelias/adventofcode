use std::{collections::HashMap, usize};

use crate::utils::{grid::Grid, util};

const DAY: &str = "day10";

#[allow(unused)]
pub(crate) fn solve() -> String {
    let lines = util::lines_in(&format!("./src/{}/input1", DAY));
    return format!("result1: {}\nresult2: {}", part1(&lines), part2(&lines));
}

fn part1(lines: &Vec<String>) -> String {
    let (m, n) = (lines.len(), lines.get(0).unwrap().len());
    let mut grid: Grid<char> = Grid::new(m, n);
    parse_lines(&mut grid, lines);

    let route = find_route(&grid);
    (route.len() / 2).to_string()
}

fn parse_lines(grid: &mut Grid<char>, lines: &Vec<String>) {
    for (p, line) in lines.iter().enumerate() {
        for (q, ch) in line.chars().enumerate() {
            grid.set(p, q, Some(ch)).unwrap();
        }
    }
}

fn find_route(grid: &Grid<char>) -> Vec<(usize, usize)> {
    let start = grid.find('S');
    let mut start_neighbors = connected_neighbors(&grid, start);
    assert_eq!(start_neighbors.len(), 2);

    let from = start_neighbors.pop().unwrap().pos;
    let to = start_neighbors.pop().unwrap().pos;

    let mut nexts = vec![from];
    let mut from_map = HashMap::new();
    from_map.insert(start, to);
    from_map.insert(from, start);

    while nexts.len() > 0 {
        let curr = nexts.remove(0);
        let mut neighbors = connected_neighbors(&grid, curr);

        while neighbors.len() != 0 {
            let neighbor = neighbors.pop().unwrap();
            if !from_map.contains_key(&neighbor.pos) {
                nexts.push(neighbor.pos);
                from_map.insert(neighbor.pos, curr);
            }
        }
    }

    let mut path = &to;
    let mut route = vec![to];
    while *path != from {
        path = from_map.get(path).unwrap();
        route.push(*path);
        //println!("path {:?}", path);
    }

    route.push(start);
    route
}

fn start_value(grid: &Grid<char>, start: (usize, usize)) -> char {
    let neighbors = connected_neighbors(grid, start);
    let dirs: Vec<&Direction> = neighbors
        .iter()
        .map(|n| &n.dir)
        .collect::<Vec<&Direction>>();

    let dir_tuple = (dirs.get(0).unwrap(), dirs.get(1).unwrap());

    match dir_tuple {
        (Direction::Left, Direction::Right) => '-',
        (Direction::Left, Direction::Top) => 'J',
        (Direction::Left, Direction::Bottom) => '7',
        (Direction::Right, Direction::Top) => 'L',
        (Direction::Right, Direction::Bottom) => 'F',
        (Direction::Top, Direction::Bottom) => '|',
        _ => 'S',
    }
}

#[derive(Clone, PartialEq)]
struct Neighbor {
    pos: (usize, usize),
    dir: Direction,
    val: char,
}

#[derive(Clone, PartialEq)]
enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

fn connected_neighbors(grid: &Grid<char>, start: (usize, usize)) -> Vec<Neighbor> {
    let left = grid.left_cell_by_tuple(start).map(|cell| Neighbor {
        pos: cell.to_tuple(),
        dir: Direction::Left,
        val: *cell.val.unwrap(),
    });
    let right = grid.right_cell_by_tuple(start).map(|cell| Neighbor {
        pos: cell.to_tuple(),
        dir: Direction::Right,
        val: *cell.val.unwrap(),
    });
    let top = grid.top_cell_by_tuple(start).map(|cell| Neighbor {
        pos: cell.to_tuple(),
        dir: Direction::Top,
        val: *cell.val.unwrap(),
    });
    let bottom = grid.bottom_cell_by_tuple(start).map(|cell| Neighbor {
        pos: cell.to_tuple(),
        dir: Direction::Bottom,
        val: *cell.val.unwrap(),
    });

    vec![left, right, top, bottom]
        .into_iter()
        .filter(|n| n.is_ok())
        .map(|n| n.unwrap())
        .filter(|n| match n.dir {
            Direction::Left => n.val == 'F' || n.val == '-' || n.val == 'L',
            Direction::Right => n.val == 'J' || n.val == '-' || n.val == '7',
            Direction::Top => n.val == '|' || n.val == '7' || n.val == 'F',
            Direction::Bottom => n.val == '|' || n.val == 'L' || n.val == 'J',
        })
        .collect()
}

#[derive(PartialEq)]
enum State {
    Outside(Break),
    Inside(Break),
}

#[derive(PartialEq)]
enum Break {
    None,
    Up,
    Down,
}

impl State {
    fn next(&self, input: &char) -> State {
        match self {
            State::Outside(Break::None) => match input {
                '|' => State::Inside(Break::None),
                'L' => State::Outside(Break::Up),
                'F' => State::Outside(Break::Down),
                _ => State::Outside(Break::None),
            },
            State::Outside(Break::Up) => match input {
                'J' => State::Outside(Break::None),
                '7' => State::Inside(Break::None),
                _ => State::Outside(Break::Up),
            },
            State::Outside(Break::Down) => match input {
                'J' => State::Inside(Break::None),
                '7' => State::Outside(Break::None),
                _ => State::Outside(Break::Down),
            },
            State::Inside(Break::None) => match input {
                '|' => State::Outside(Break::None),
                'L' => State::Inside(Break::Up),
                'F' => State::Inside(Break::Down),
                _ => State::Inside(Break::None),
            },
            State::Inside(Break::Up) => match input {
                'J' => State::Inside(Break::None),
                '7' => State::Outside(Break::None),
                _ => State::Inside(Break::Up),
            },
            State::Inside(Break::Down) => match input {
                'J' => State::Outside(Break::None),
                '7' => State::Inside(Break::None),
                _ => State::Inside(Break::Down),
            },
        }
    }
}

fn part2(lines: &Vec<String>) -> String {
    let (m, n) = (lines.len(), lines.get(0).unwrap().len());
    let mut grid: Grid<char> = Grid::new(m, n);
    parse_lines(&mut grid, lines);

    let route = find_route(&grid);
    let start = grid.find('S');
    let start_value = start_value(&grid, start);
    grid.set(start.0, start.1, Some(start_value)).unwrap();
    let mut count = 0;

    for p in 0..m {
        let mut state = State::Outside(Break::None);
        for q in 0..n {
            if let Ok(Some(val)) = grid.get(p, q) {
                if route.contains(&(p, q)) {
                    state = state.next(val);
                    // grid.set(p, q, Some('I')).unwrap();
                } else if let State::Inside(_) = state {
                    grid.set(p, q, Some('O')).unwrap();
                    count += 1;
                } else {
                    //grid.set(p, q, Some('.')).unwrap();
                }
            }
        }
    }

    print_grid(&grid);

    count.to_string()
}

fn print_grid(grid: &Grid<char>) {
    for i in 0..grid.m {
        for j in 0..grid.n {
            print!("{}", grid.get(i, j).unwrap().unwrap());
        }
        println!()
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, DAY};
    use crate::utils::util;

    #[test]
    fn test_part1_sample() {
        let lines = util::lines_in(&format!("./src/{}/input", DAY));
        assert_eq!("4", part1(&lines))
    }

    #[test]
    fn test_part1_sample2() {
        let lines = util::lines_in(&format!("./src/{}/input3", DAY));
        assert_eq!("8", part1(&lines))
    }

    #[test]
    fn test_part1_input() {
        let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        assert_eq!("6812", part1(&lines))
    }

    #[test]
    fn test_part2_sample() {
        let lines = util::lines_in(&format!("./src/{}/input4", DAY));
        assert_eq!("4", part2(&lines))
    }
    #[test]
    fn test_part2_sample2() {
        let lines = util::lines_in(&format!("./src/{}/input5", DAY));
        assert_eq!("8", part2(&lines))
    }
    #[test]
    fn test_part2_sample3() {
        let lines = util::lines_in(&format!("./src/{}/input6", DAY));
        assert_eq!("10", part2(&lines));
    }

    #[test]
    fn test_part2_input() {
        let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        assert_eq!("527", part2(&lines))
    }
}
