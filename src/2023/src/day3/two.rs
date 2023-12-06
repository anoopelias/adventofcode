use std::{
    collections::{HashMap, HashSet},
    usize, vec,
};

use crate::utils::util::neighbors;

pub(crate) fn solve(lines: Vec<String>) -> String {
    let (m, n) = (lines.len(), lines.get(0).unwrap().len());

    let mut sum = 0;
    let mut attached_stars = HashMap::new();

    for (p, line) in lines.iter().enumerate() {
        let chars = line.chars();
        let mut num_part = 0;
        let mut attached = HashSet::new();
        for (q, ch) in chars.enumerate() {
            if ch.is_numeric() {
                num_part = (num_part * 10) + ch.to_digit(10).unwrap();
                let stars = stars_around(&lines, p, q, m, n);

                for star in stars {
                    attached.insert(star);
                }
            } else {
                for star in attached {
                    attached_stars.entry(star).or_insert(vec![]).push(num_part);
                }
                num_part = 0;
                attached = HashSet::new();
            }
        }
        for star in attached {
            attached_stars.entry(star).or_insert(vec![]).push(num_part);
        }
    }

    for list in attached_stars.values() {
        if list.len() == 2 {
            sum += list.get(0).unwrap() * list.get(1).unwrap();
        }
    }

    sum.to_string()
}

fn stars_around(lines: &Vec<String>, p: usize, q: usize, m: usize, n: usize) -> Vec<String> {
    let neighbors = neighbors(p, q, m, n);

    let mut attached: Vec<String> = vec![];
    for (p, q) in neighbors {
        let ch = lines.get(p).unwrap().chars().nth(q).unwrap();
        if ch == '*' {
            let mut key = p.to_string();
            key.push(':');
            key.push_str(&q.to_string());
            attached.push(key);
        }
    }

    return attached;
}

#[cfg(test)]
mod tests {
    use crate::day3::two::solve;
    use crate::utils::util;

    #[test]
    fn test_sample() {
        let lines = util::lines_in("./src/day3/input");
        assert_eq!("467835", solve(lines))
    }

    #[test]
    fn test_input() {
        let lines = util::lines_in("./src/day3/input1");
        assert_eq!("81709807", solve(lines))
    }
}
