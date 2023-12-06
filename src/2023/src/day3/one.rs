use std::usize;

use crate::utils::grid::Grid;

pub(crate) fn solve(lines: Vec<String>) -> String {
    let (m, n) = (lines.len(), lines.get(0).unwrap().len());
    let grid = Grid::new(m, n);

    let mut sum = 0;

    for (p, line) in lines.iter().enumerate() {
        let chars = line.chars();
        let mut num_part = 0;
        let mut should_add = false;
        for (q, ch) in chars.enumerate() {
            if ch.is_numeric() {
                num_part = (num_part * 10) + ch.to_digit(10).unwrap();
                if has_symbol(&lines, p, q, &grid) {
                    should_add = true;
                }
            } else {
                if should_add {
                    sum += num_part;
                }
                num_part = 0;
                should_add = false;
            }
        }
        if should_add {
            sum += num_part;
        }
    }

    sum.to_string()
}
fn has_symbol(lines: &Vec<String>, p: usize, q: usize, grid: &Grid) -> bool {
    let neighbors = grid.all_neighbor_tuples(p, q).unwrap();

    for (p, q) in neighbors {
        let ch = lines
            .get(p as usize)
            .unwrap()
            .chars()
            .nth(q as usize)
            .unwrap();
        if ch != '.' && !ch.is_numeric() {
            return true;
        }
    }

    return false;
}

#[cfg(test)]
mod tests {
    use crate::day3::one::solve;
    use crate::utils::util;

    #[test]
    fn test_sample() {
        let lines = util::lines_in("./src/day3/input");
        assert_eq!("4361", solve(lines))
    }

    #[test]
    fn test_input() {
        let lines = util::lines_in("./src/day3/input1");
        assert_eq!("538046", solve(lines))
    }
}
