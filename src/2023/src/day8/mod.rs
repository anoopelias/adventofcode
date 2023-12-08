use std::collections::HashMap;

use crate::utils::{parser::SeparatorParser, util};

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
    fn parse(&self) -> (Vec<char>, HashMap<String, (String, String)>) {
        let mut map: HashMap<String, (String, String)> = HashMap::new();
        let mut lines = self.lines.clone();
        let instr: Vec<char> = lines.remove(0).chars().collect();
        lines.remove(0);
        for line in lines {
            let splits = line.parse_separator("=");
            let key = splits.get(0).unwrap().to_string();
            let mut values = splits.get(1).unwrap().parse_separator(", ");
            map.insert(
                key,
                (
                    remove_first(values.remove(0)),
                    remove_last(values.remove(0)),
                ),
            );
        }
        (instr, map)
    }
}

fn remove_first(s: String) -> String {
    let mut chs = s.chars();
    chs.next();
    chs.as_str().to_string()
}

fn remove_last(s: String) -> String {
    let mut chs = s.chars();
    chs.next_back();
    chs.as_str().to_string()
}
