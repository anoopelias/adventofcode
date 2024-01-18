const DAY: &str = "day21";

use std::time::Instant;

use crate::utils::util::{self, ToGrid};

#[allow(unused)]
pub(crate) fn solve() -> String {
    let lines = util::lines_in(&format!("./src/{}/input1", DAY));
    let time = Instant::now();
    let part1 = part1(&lines, 64);
    let elapsed1 = time.elapsed();

    let time = Instant::now();
    let part2 = part2(&lines, 26501365);
    let elapsed2 = time.elapsed();
    return format!(
        "result1: {} in {:?} \nresult2: {} in {:?}",
        part1, elapsed1, part2, elapsed2,
    );
}

fn part1(lines: &Vec<String>, steps: i32) -> String {
    let grid = lines.to_grid();
    let parity = steps % 2;

    let dist_map = grid.bfs(grid.find('S').unwrap(), |neighbor| {
        neighbor.cell.val == &'.' || neighbor.cell.val == &'S'
    });

    //////
    ////// Debugging starts

    let center_sq = dist_map
        .iter()
        .filter(|(c, _)| c.p >= 11 && c.p <= 21 && c.q >= 11 && c.q <= 21)
        .filter(|(_, dist)| **dist % 2 == 0)
        .map(|(c, _)| c)
        .collect::<Vec<_>>();

    println!("center_sq {}", center_sq.len());

    let even_cuts = dist_map
        .iter()
        .filter(|(c, _)| {
            (c.p < 11 && c.q < 11)
                || (c.p > 21 && c.q < 11)
                || (c.p < 11 && c.q > 21)
                || (c.p > 21 && c.q > 21)
        })
        .filter(|(_, dist)| **dist <= steps && **dist % 2 == 0)
        .map(|(c, _)| c)
        .collect::<Vec<_>>();
    println!("even_cuts {}", even_cuts.len());
    let odds_outside = dist_map
        .iter()
        .filter(|(c, _)| {
            (c.p < 11 && c.q >= 11 && c.q <= 21)
                || (c.p > 21 && c.q >= 11 && c.q <= 21)
                || (c.p >= 11 && c.p <= 21 && c.q < 11)
                || (c.p >= 11 && c.p <= 21 && c.q > 21)
        })
        .filter(|(_, dist)| **dist <= steps && **dist % 2 == 0)
        .count();
    println!("odds_outside {}", odds_outside);

    let center_sq_odds = dist_map
        .iter()
        .filter(|(c, _)| c.p >= 11 && c.p <= 21 && c.q >= 11 && c.q <= 21)
        .filter(|(_, dist)| **dist % 2 == 1)
        .count();
    println!("center_sq_odds {}", center_sq_odds);

    let center_sq_odds = dist_map
        .iter()
        .filter(|(c, _)| c.p >= 11 && c.p <= 21 && c.q >= 11 && c.q <= 21)
        .filter(|(_, dist)| **dist % 2 == 1)
        .count();
    println!("center_sq_odds {}", center_sq_odds);

    /////// Debugging ends
    ///////

    let all = grid
        .bfs(grid.find('S').unwrap(), |neighbor| {
            neighbor.cell.val == &'.' || neighbor.cell.val == &'S'
        })
        .into_iter()
        .filter(|(_, dist)| *dist <= steps && dist % 2 == parity)
        .map(|(c, _)| c)
        .collect::<Vec<_>>();

    println!(
        "{}",
        grid.to_string_by(|cell| match all.contains(&&cell.coord) {
            true => "O".to_string(),
            false => cell.val.to_string(),
        })
    );

    all.len().to_string()
}

fn part2(lines: &Vec<String>, steps: usize) -> String {
    let grid = lines.to_grid();
    let dist_map = grid.bfs(grid.find('S').unwrap(), |neighbor| {
        neighbor.cell.val == &'.' || neighbor.cell.val == &'S'
    });

    let width = grid.m;
    let half_width = width / 2;
    let n = (steps - half_width) / width;

    assert_eq!(steps, (n * width) + half_width);
    let mut evens = dist_map.values().filter(|&&value| value % 2 == 0).count();
    let mut even_cuts = dist_map
        .values()
        .filter(|&&value| value % 2 == 0 && value as usize > half_width)
        .count()
        // Reducing it by 1 since there is one node inside the box
        - 1;

    let mut odds = dist_map.values().filter(|&&value| value % 2 == 1).count();
    let mut odd_cuts = dist_map
        .values()
        .filter(|&&value| value % 2 == 1 && value as usize > half_width)
        .count()
        - 1;

    if steps % 2 == 1 {
        (odds, evens) = (evens, odds);
        (odd_cuts, even_cuts) = (even_cuts, odd_cuts);
    }

    let n_sq = n * n;
    let np1 = n + 1;
    let np1_sq = np1 * np1;

    let result = (evens * n_sq) + (odds * np1_sq) - (np1 * odd_cuts) + (n * even_cuts);
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, DAY};
    use crate::utils::util;

    #[test]
    fn test_part1_sample() {
        let lines = util::lines_in(&format!("./src/{}/input", DAY));
        assert_eq!("13", part1(&lines, 5))
    }

    #[test]
    fn test_part1_input() {
        let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        assert_eq!("3585", part1(&lines, 64))
    }

    #[test]
    fn test_part1_expanded_grid() {
        let lines = util::lines_in(&format!("./src/{}/input_n0", DAY));
        // assert_eq!("34", part1(&lines, 5));
        assert_eq!("34", part2(&lines, 5));
    }

    #[test]
    fn test_part2_sample() {
        let lines = util::lines_in(&format!("./src/{}/input", DAY));
        //assert_eq!("179", part2(&lines, 16));
        assert_eq!("179", part2(&lines, 500));
    }

    #[test]
    fn test_part2_input() {
        let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        assert_eq!("597102911216785", part2(&lines, 26501365));

        // too low:     597102911216785
        // not correct: 597102911419086
        // too hi :     597102954104491
    }
}
