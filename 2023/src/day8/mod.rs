use std::collections::HashMap;

use crate::utils::{string::WrapperRemover, util};

use self::{one::ProblemOne, two::ProblemTwo};

pub(crate) mod one;
pub(crate) mod two;

#[allow(unused)]
pub(crate) fn solve() -> String {
    let lines = util::lines_in("../../aoc-files/2023/day8/input1");
    let problem = Problem { lines };

    let problem_one = ProblemOne::new(problem.clone());
    let result1 = problem_one.solve();

    let problem_two = ProblemTwo::new(problem);
    let result2 = problem_two.solve();
    return format!("result1: {}\nresult2: {}", result1, result2);
}

#[derive(Clone)]
pub(crate) struct Problem {
    lines: Vec<String>,
}

trait Solution {
    fn solve(&self) -> String;
}

impl Problem {
    fn parse(&self) -> HashMap<&str, (&str, &str)> {
        let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
        for line in self.lines.iter().skip(2) {
            let (key, value) = line
                .as_str()
                .trim()
                .split_once("=")
                .map(|(s1, s2)| (s1.trim(), s2.trim()))
                .unwrap();
            map.insert(key, value.remove_wrapping().split_once(", ").unwrap());
        }
        map
    }

    fn instr(&self) -> Vec<char> {
        self.lines.get(0).unwrap().chars().collect()
    }
}
