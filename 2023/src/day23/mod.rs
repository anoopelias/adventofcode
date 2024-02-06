const DAY: &str = "day23";

use crate::utils::grid::{Direction, Grid, Neighbor};
use crate::utils::util::ToGrid;

use crate::utils::{grid::Coord, util};
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;

#[allow(unused)]
pub(crate) fn solve() -> String {
    let lines = util::lines_in(&format!("../../aoc-files/2023/{}/input1", DAY));
    let time = Instant::now();
    let part1 = part1(&lines);
    let elapsed1 = time.elapsed();
    println!("result1: {} in {:?}", part1, elapsed1);

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
    let adj_list = build_adj_list(grid, start, end);

    let mut path = HashSet::new();
    path.insert(start);
    find_longest_path(&adj_list, start, end, &mut path)
        .unwrap()
        .to_string()
}

fn build_adj_list(
    grid: Grid<char>,
    start: Coord,
    end: Coord,
) -> HashMap<Coord, Vec<(Coord, usize)>> {
    let mut edges = vec![];

    let (start_edge_other, dist) = next(&grid, start, Direction::Top).unwrap();
    edges.push(Edge::new((start, start_edge_other), dist - 1));

    let (end_edge_other, dist) = next(&grid, end, Direction::Bottom).unwrap();
    edges.push(Edge::new((end, end_edge_other), dist - 1));

    let mut queue = VecDeque::new();
    queue.push_back(start_edge_other);
    while !queue.is_empty() {
        let jnode = queue.pop_front().unwrap();
        let neighbors = free_neighbors(&grid, jnode);

        for neighbor in neighbors {
            match next(&grid, neighbor.cell.coord, neighbor.dir.opposite()) {
                Some((jnode_other, dist)) => {
                    let edge = Edge::new((jnode, jnode_other), dist);
                    if !edges.contains(&edge) {
                        edges.push(edge);
                        queue.push_back(jnode_other);
                    }
                }
                None => {}
            }
        }
    }

    let mut adj_list = HashMap::new();
    for edge in edges {
        let (from, to) = edge.nodes;
        add_adj_node(&mut adj_list, from, to, edge.dist);
        add_adj_node(&mut adj_list, to, from, edge.dist);
    }
    adj_list
}

fn find_longest_path(
    adj_list: &HashMap<Coord, Vec<(Coord, usize)>>,
    start: Coord,
    end: Coord,
    path: &mut HashSet<Coord>,
) -> Option<usize> {
    let mut max_dist = None;
    let adjs = adj_list.get(&start).unwrap();
    for adj in adjs {
        let (adj_node, adj_dist) = adj;
        let dist = if *adj_node == end {
            Some(*adj_dist)
        } else if !path.contains(adj_node) {
            path.insert(*adj_node);
            let dist = find_longest_path(adj_list, *adj_node, end, path)
                .map(|dist_to_end| dist_to_end + adj_dist);
            path.remove(adj_node);
            dist
        } else {
            None
        };

        max_dist = match dist {
            Some(dist) => match max_dist {
                Some(max_dist) => Some(std::cmp::max(dist, max_dist)),
                None => Some(dist),
            },
            None => max_dist,
        };
    }

    max_dist
}

fn add_adj_node(
    adj: &mut HashMap<Coord, Vec<(Coord, usize)>>,
    from: Coord,
    to: Coord,
    dist: usize,
) {
    match adj.get_mut(&from) {
        Some(to_vec) => to_vec.push((to, dist)),
        None => {
            adj.insert(from, vec![(to, dist)]);
        }
    }
}

fn free_neighbors(grid: &Grid<char>, from: Coord) -> Vec<Neighbor<&char>> {
    grid.neighbors(&from)
        .into_iter()
        .filter(|neighbor| *neighbor.cell.val != '#')
        .collect::<Vec<Neighbor<&char>>>()
}

fn valid_neighbors_part2(
    grid: &Grid<char>,
    from: Coord,
    from_dir: Direction,
) -> Vec<Neighbor<&char>> {
    grid.neighbors(&from)
        .into_iter()
        .filter(|neighbor| *neighbor.cell.val != '#' && neighbor.dir != from_dir)
        .collect::<Vec<Neighbor<&char>>>()
}

fn next(grid: &Grid<char>, from: Coord, from_dir: Direction) -> Option<(Coord, usize)> {
    let mut next_neighbors = valid_neighbors_part2(grid, from, from_dir);
    let mut curr = from;
    let mut dist = 1;
    if next_neighbors.len() == 0 {
        return None;
    }

    while next_neighbors.len() == 1 {
        let neighbor = next_neighbors.pop().unwrap();
        curr = neighbor.cell.coord;
        dist += 1;
        next_neighbors = valid_neighbors_part2(grid, curr, neighbor.dir.opposite());

        if next_neighbors.len() == 0 {
            return None;
        }
    }
    Some((curr, dist))
}

#[derive(Debug)]
struct Edge {
    nodes: (Coord, Coord),
    dist: usize,
}

impl Edge {
    fn new(nodes: (Coord, Coord), dist: usize) -> Edge {
        Edge { nodes, dist }
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.nodes.0 == other.nodes.0 && self.nodes.1 == other.nodes.1)
            || (self.nodes.0 == other.nodes.1 && self.nodes.1 == other.nodes.0)
    }
}

impl Eq for Edge {}

#[cfg(test)]
mod tests {
    use clearcheck::assertions::equal::EqualityAssertion;

    use super::{part1, part2, DAY};
    use crate::utils::util;

    #[test]
    fn test_part1_sample() {
        let lines = util::lines_in(&format!("../../aoc-files/2023/{}/input", DAY));
        part1(&lines).should_equal("94");
    }

    #[test]
    fn test_part1_input() {
        let lines = util::lines_in(&format!("../../aoc-files/2023/{}/input1", DAY));
        part1(&lines).should_equal("2438");
    }

    #[test]
    fn test_part2_sample() {
        let lines = util::lines_in(&format!("../../aoc-files/2023/{}/input", DAY));
        part2(&lines).should_equal("154");
    }

    #[test]
    fn test_part2_input() {
        let _lines = util::lines_in(&format!("../../aoc-files/2023/{}/input1", DAY));
        // Too slow to test every time
        // part2(&_lines).should_equal("6658");
    }
}
