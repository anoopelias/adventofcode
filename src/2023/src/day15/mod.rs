const DAY: &str = "day15";

use std::{collections::HashMap, time::Instant};

use itertools::Itertools;

use crate::utils::{
    parser::SeparatorParser,
    util::{self},
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
    let line = lines.get(0).unwrap();
    let steps = line.parse_separator(",");
    steps
        .into_iter()
        .map(|step| hash(step))
        .sum::<usize>()
        .to_string()
}

fn part2(lines: &Vec<String>) -> String {
    let line = lines.get(0).unwrap();
    let steps = line.parse_separator(",");
    let mut hmap = HashMap::new();
    let mut keys = vec![];
    steps.iter().for_each(|step| {
        if step.contains("=") {
            let (key, val) = step.split_once("=").unwrap();
            if !hmap.contains_key(key) {
                keys.push(key);
            }
            hmap.insert(key, val.parse::<usize>().unwrap());
        } else {
            let key = &step[0..step.len() - 1];
            hmap.remove(&key);
            keys.retain(|&k| k != key);
        }
    });

    keys.iter()
        .group_by(|key| hash(key))
        .into_iter()
        .map(|(bx, lenses)| {
            lenses
                .enumerate()
                .map(|(i, lens)| (bx + 1) * (i + 1) * hmap.get(lens).unwrap())
                .sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

fn hash(st: &str) -> usize {
    st.chars()
        .map(|ch| ch as usize)
        .fold(0, |curr, val| ((curr + val) * 17) % 256)
}

#[cfg(test)]
mod tests {
    use super::{hash, part1, part2, DAY};
    use crate::utils::util;

    #[test]
    fn test_hash() {
        assert_eq!(30, hash("rn=1"));
        assert_eq!(253, hash("cm-"));
        assert_eq!(0, hash("rn"));
    }

    #[test]
    fn test_part1_sample() {
        let lines = util::lines_in(&format!("./src/{}/input", DAY));
        assert_eq!("1320", part1(&lines))
    }

    #[test]
    fn test_part1_input() {
        let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        assert_eq!("497373", part1(&lines))
    }

    #[test]
    fn test_part2_sample() {
        let lines = util::lines_in(&format!("./src/{}/input", DAY));
        assert_eq!("145", part2(&lines))
    }

    #[test]
    fn test_part2_input() {
        let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        assert_eq!("177140", part2(&lines))
    }
}
