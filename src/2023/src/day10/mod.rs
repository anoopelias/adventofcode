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
    let start_key = tuple_to_key(&start);
    let mut start_ns = connected_neighbors(&grid, start);

    let from = start_ns.pop().unwrap();
    let from_key = tuple_to_key(&from);

    let to = start_ns.pop().unwrap();
    let to_key = tuple_to_key(&to);

    let mut nexts = vec![from];
    let mut from_map = HashMap::new();
    let mut navigation_map = HashMap::new();
    from_map.insert(from_key.clone(), start_key.clone());
    from_map.insert(start_key.clone(), to_key.clone());
    navigation_map.insert(from_key.clone(), from);
    navigation_map.insert(start_key.clone(), to);

    while nexts.len() > 0 {
        let curr = nexts.remove(0);
        let curr_key = tuple_to_key(&curr);
        let mut cns = connected_neighbors(&grid, curr);

        while cns.len() != 0 {
            let cn = cns.pop().unwrap();
            let nk = tuple_to_key(&cn);
            if !from_map.contains_key(&nk) {
                nexts.push(cn);
                from_map.insert(nk.clone(), curr_key.clone());
                navigation_map.insert(nk, cn);
            }
        }
    }

    let mut path = &to_key;
    let mut route = vec![navigation_map.remove(path).unwrap()];
    while *path != from_key {
        path = from_map.get(path).unwrap();
        route.push(navigation_map.remove(path).unwrap());
        println!("path {}", path);
    }

    route.push(navigation_map.remove(&start_key).unwrap());
    route
}

fn connected_neighbors(grid: &Grid<char>, start: (usize, usize)) -> Vec<(usize, usize)> {
    let mut connected_neighbors = vec![];
    let left = grid.left_cell(start.0, start.1);
    if let Ok(cell) = left {
        if cell.val.unwrap() == &'F' || cell.val.unwrap() == &'-' || cell.val.unwrap() == &'L' {
            connected_neighbors.push(cell.to_tuple())
        }
    }
    let right = grid.right_cell(start.0, start.1);
    if let Ok(cell) = right {
        if cell.val.unwrap() == &'-' || cell.val.unwrap() == &'J' || cell.val.unwrap() == &'7' {
            connected_neighbors.push(cell.to_tuple())
        }
    }
    let top = grid.top_cell(start.0, start.1);
    if let Ok(cell) = top {
        if cell.val.unwrap() == &'|' || cell.val.unwrap() == &'7' || cell.val.unwrap() == &'F' {
            connected_neighbors.push(cell.to_tuple())
        }
    }

    let bottom = grid.bottom_cell(start.0, start.1);
    if let Ok(cell) = bottom {
        if cell.val.unwrap() == &'|' || cell.val.unwrap() == &'L' || cell.val.unwrap() == &'J' {
            connected_neighbors.push(cell.to_tuple())
        }
    }
    connected_neighbors
}

fn tuple_to_key((p, q): &(usize, usize)) -> String {
    format!("{}:{}", p, q)
}

fn part2(lines: &Vec<String>) -> String {
    let (m, n) = (lines.len(), lines.get(0).unwrap().len());
    let mut grid: Grid<char> = Grid::new(m, n);
    parse_lines(&mut grid, lines);

    let route = find_route(&grid);

    let mut con_grid: Grid<bool> = Grid::new(m + 1, n + 1);
    con_grid.fill(true);

    for con in route {
        con_grid.set(con.0, con.0, Some(false)).unwrap();
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
    fn test_sample() {
        let lines = util::lines_in(&format!("./src/{}/input", DAY));
        //assert_eq!("2", part2(&lines))
    }

    #[test]
    fn test_input() {
        let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        //assert_eq!("928", part2(&lines))
    }
}
