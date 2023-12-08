use std::collections::HashMap;

use crate::utils::parser::SeparatorParser;

use super::{Problem, Solution};

pub(crate) struct ProblemOne {
    problem: Problem,
}

impl ProblemOne {
    pub(crate) fn new(problem: Problem) -> ProblemOne {
        ProblemOne { problem }
    }
}

impl Solution for ProblemOne {
    fn solve(&self) -> String {
        let mut map: HashMap<String, (String, String)> = HashMap::new();
        let mut lines = self.problem.lines.clone();
        let instr: Vec<char> = lines.remove(0).chars().collect();
        lines.remove(0);
        for line in lines.iter() {
            let ref_line = line.as_str();
            let splits = ref_line.parse_separator("=");
            let key = splits.get(0).unwrap().to_string();
            let mut values = splits.get(1).unwrap().parse_separator(", ");
            map.insert(
                key,
                (
                    remove_first(values.remove(0).to_string()),
                    remove_last(values.remove(0).to_string()),
                ),
            );
        }

        let mut start = "AAA".to_string();
        let mut ip = 0;
        let mut cnt = 0;

        while start != "ZZZ" {
            start = if instr.get(ip).unwrap() == &'L' {
                map.get(&start).unwrap().0.clone()
            } else {
                map.get(&start).unwrap().1.clone()
            };
            cnt += 1;
            ip += 1;
            if ip == instr.len() {
                ip = 0;
            }
        }

        cnt.to_string()
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

#[cfg(test)]
mod tests {
    use super::{super::Solution, Problem, ProblemOne};
    use crate::utils::util;

    #[test]
    fn test_sample() {
        let lines = util::lines_in("./src/day8/input");
        let problem = ProblemOne::new(Problem { lines });
        assert_eq!("2", problem.solve())
    }

    #[test]
    fn test_input() {
        let lines = util::lines_in("./src/day8/input1");
        let problem = ProblemOne::new(Problem { lines });
        assert_eq!("12643", problem.solve())
    }
}
