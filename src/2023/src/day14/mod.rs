const DAY: &str = "day14";

use std::{time::Instant, usize};

use crate::utils::{
    grid::Grid,
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
    let mut grid = lines.to_grid();

    tilt_north(&mut grid);

    grid.all()
        .iter()
        .map(|cell| match &cell.val {
            'O' => grid.m - cell.coord.p,
            _ => 0,
        })
        .sum::<usize>()
        .to_string()
}

fn cycle(grid: &mut Grid<char>) {
    tilt_north(grid);
    tilt_west(grid);
    tilt_south(grid);
    tilt_east(grid);
}

fn tilt_north(grid: &mut Grid<char>) {
    tilt(grid.cols_mut());
}

fn tilt_west(grid: &mut Grid<char>) {
    tilt(grid.rows_mut().into_iter().rev().collect());
}

fn tilt_south(grid: &mut Grid<char>) {
    tilt(grid.cols_mut().into_iter().rev().collect());
}

fn tilt_east(grid: &mut Grid<char>) {
    tilt(grid.rows_mut());
}

fn tilt(lines: Vec<Vec<&mut char>>) {
    lines.into_iter().for_each(|mut line| {
        let mut free = next_free(&line, 0);
        let mut tip = free + 1;

        while tip < line.len() {
            match *line.get(tip).unwrap() {
                'O' => {
                    let _ = std::mem::replace(*line.get_mut(free).unwrap(), 'O');
                    let _ = std::mem::replace(*line.get_mut(tip).unwrap(), '.');
                    // col.swap(free, tip);
                    free += 1;
                    tip += 1;
                }
                '#' => {
                    free = next_free(&line, tip + 1);
                    tip = free + 1;
                }
                '.' => {
                    tip += 1;
                }
                _ => {}
            }
        }
    });
}

fn next_free(col: &Vec<&mut char>, p: usize) -> usize {
    let mut curr = p;
    while curr < col.len() {
        match **col.get(curr).unwrap() {
            'O' | '#' => curr = curr + 1,
            '.' => break,
            _ => unreachable!(),
        }
    }
    curr
}

fn part2(lines: &Vec<String>) -> String {
    let mut grid = lines.to_grid();

    for _ in 0..1000000000 {
        cycle(&mut grid);
    }

    grid.all()
        .iter()
        .map(|cell| match &cell.val {
            'O' => grid.m - cell.coord.p,
            _ => 0,
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, DAY};
    use crate::utils::util;

    #[test]
    fn test_part1_sample() {
        let lines = util::lines_in(&format!("./src/{}/input", DAY));
        assert_eq!("136", part1(&lines))
    }

    #[test]
    fn test_part1_input() {
        let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        assert_eq!("108144", part1(&lines))
    }

    #[test]
    fn test_part2_sample() {
        let lines = util::lines_in(&format!("./src/{}/input", DAY));
        assert_eq!("400", part2(&lines))
    }

    #[test]
    fn test_part2_input() {
        // let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        // assert_eq!("39359", part2(&lines))
    }
}
