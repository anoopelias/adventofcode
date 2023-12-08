use std::collections::HashMap;

use crate::utils::{parser::SepParser, string::WrapperRemover, util};

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
    fn parse(&self) -> (Vec<char>, HashMap<&str, (String, String)>) {
        let mut map: HashMap<&str, (String, String)> = HashMap::new();
        let instr: Vec<char> = self.lines.get(0).unwrap().chars().collect();
        for line in self.lines.iter().skip(2) {
            let splits = line.parse_sep("=");
            let key = splits.get(0).unwrap();
            let mut values = splits.get(1).unwrap().remove_wrapping().parse_sep(", ");
            map.insert(
                key,
                (values.remove(0).to_string(), values.remove(0).to_string()),
            );
        }
        (instr, map)
    }
}
