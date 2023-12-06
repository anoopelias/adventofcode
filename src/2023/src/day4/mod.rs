use std::collections::HashSet;

use crate::utils::util;

pub(crate) mod one;
pub(crate) mod two;

#[allow(unused)]
pub(crate) fn solve() -> String {
    let lines = util::lines_in("./src/day4/input1");

    let result1 = one::solve(lines.clone());
    let result2 = two::solve(lines);
    return format!("result1: {}\nresult2: {}", result1, result2);
}

fn wins_for(line: &String) -> usize {
    let line: Vec<&str> = line.split(":").collect();
    let line = line.get(1).unwrap().trim();

    let line: Vec<&str> = line.split("|").collect();

    let winner = string_to_nums(line.get(0).unwrap());
    let card = string_to_nums(line.get(1).unwrap());

    card.iter()
        .filter(|num| winner.contains(num))
        .collect::<Vec<_>>()
        .len()
}

fn string_to_nums(str: &str) -> HashSet<i32> {
    str.trim()
        .split(" ")
        .collect::<Vec<_>>()
        .iter()
        .map(|s| s.trim())
        .filter(|s| s.len() > 0)
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}
