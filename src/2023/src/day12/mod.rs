#![allow(unused)]
const DAY: &str = "day12";

use std::time::Instant;

use crate::utils::{
    parser::{I32Parser, SeparatorParser, TwoSplitter, UsizeParser},
    util,
};

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
    lines
        .iter()
        .map(|line| {
            let (springs, groups) = line.split_in_two(" ");
            count(springs.to_string(), groups.parse_usize(","))
        })
        .sum::<usize>()
        .to_string()
}

fn part2(lines: &Vec<String>) -> String {
    lines
        .iter()
        .map(|line| {
            let (springs, groups) = line.split_in_two(" ");
            count(
                five_times(springs, "?"),
                five_times(groups, ",").parse_usize(","),
            )
        })
        .sum::<usize>()
        .to_string()
}

struct Transition {
    dot: usize,
    hash: usize,
    end: usize,
}

impl Transition {
    fn new(dot: usize, hash: usize, end: usize) -> Transition {
        Transition { dot, hash, end }
    }
}

fn count(springs: String, groups: Vec<usize>) -> usize {
    // 1, 1, 3
    // 0 1 2 3 4 5 6 7
    //   # . # . # # #

    // State machine:
    // st dot ha end
    // 0  0   1  9
    // 1  2   9  9
    // 2  2   3  9
    // 3  4   9  9
    // 4  4   5  9
    // 5  9   6  9
    // 6  9   7  9
    // 7  7   9  8
    // 8
    // 9

    // Execution
    // ? 0,    1
    // ? 0, 1, 2

    let yes = groups.iter().sum::<usize>() + groups.len();
    let no = yes + 1;
    let machine = build_state_machine(groups, yes, no);

    // Optimize by not doing same state multiple times.
    let mut state_count = vec![0usize; machine.len()];
    state_count[0] = 1;

    for ch in springs.chars() {
        let new_states: Vec<(usize, usize)> = state_count
            .into_iter()
            .enumerate()
            .filter(|&(state, count)| state != no && count != 0)
            .flat_map(|(state, count)| match ch {
                '.' => vec![(machine[state].dot, count)],
                '#' => vec![(machine[state].hash, count)],
                _ => vec![(machine[state].dot, count), (machine[state].hash, count)],
            })
            .collect();

        state_count = vec![0usize; machine.len()];
        for (state, count) in new_states {
            state_count[state] += count;
        }
    }

    state_count
        .into_iter()
        .enumerate()
        .filter(|&(state, count)| state != no && count != 0)
        .map(|(state, count)| (machine[state].end, count))
        .filter(|&(state, count)| state == yes)
        .map(|(_, count)| count)
        .sum()
}

fn build_state_machine(groups: Vec<usize>, yes: usize, no: usize) -> Vec<Transition> {
    let mut transitions = vec![];
    transitions.push(Transition::new(0, 1, no));
    let mut state = 1;

    for group in groups {
        for i in 0..group - 1 {
            transitions.push(Transition::new(no, state + 1, no));
            state += 1;
        }
        transitions.push(Transition::new(state + 1, no, no));
        state += 1;

        transitions.push(Transition::new(state, state + 1, no));
        state += 1;
    }

    transitions.pop();
    transitions.pop();

    state -= 2;

    transitions.push(Transition::new(state, no, yes));
    state += 1;

    // Yes
    transitions.push(Transition::new(state, no, no));
    state += 1;

    // No
    transitions.push(Transition::new(state, no, no));
    transitions
}

fn five_times(st: &str, sep: &str) -> String {
    vec![st, st, st, st, st].join(sep)
}

#[cfg(test)]
mod tests {
    use super::{count, part1, part2, DAY};
    use crate::utils::util;

    #[test]
    fn test_part1_sample() {
        let lines = util::lines_in(&format!("./src/{}/input", DAY));
        assert_eq!("21", part1(&lines))
    }

    #[test]
    fn test_part1_input() {
        let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        assert_eq!("6827", part1(&lines))
    }

    #[test]
    fn test_state_machine() {
        assert_eq!(1, count("???.###".to_string(), vec![1, 1, 3]));
        assert_eq!(4, count("????.######..#####.".to_string(), vec![1, 6, 5]));
        assert_eq!(10, count("?###????????".to_string(), vec![3, 2, 1]));
    }

    #[test]
    fn test_part2_sample() {
        let lines = util::lines_in(&format!("./src/{}/input", DAY));
        assert_eq!("525152", part2(&lines))
    }

    #[test]
    fn test_part2_input() {
        let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        assert_eq!("1537505634471", part2(&lines))
    }
}
