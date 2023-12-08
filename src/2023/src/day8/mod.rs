use std::collections::HashMap;

use crate::utils::{parser::TwoSplitter, string::WrapperRemover, util};

use self::{one::ProblemOne, two::ProblemTwo};

pub(crate) mod one;
pub(crate) mod two;

#[allow(unused)]
pub(crate) fn solve() -> String {
    let lines = util::lines_in("./src/day8/input1");
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
    fn parse(&self) -> (Vec<char>, HashMap<&str, (&str, &str)>) {
        let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
        let instr: Vec<char> = self.lines.get(0).unwrap().chars().collect();
        for line in self.lines.iter().skip(2) {
            let (key, value) = line.split_in_two("=");
            map.insert(key, value.remove_wrapping().split_in_two(", "));
        }
        (instr, map)
    }
}
