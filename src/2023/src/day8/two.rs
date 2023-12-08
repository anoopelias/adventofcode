use std::collections::HashMap;

use crate::utils::parser::SeparatorParser;

use super::{Problem, Solution};

pub(crate) struct ProblemTwo {
    problem: Problem,
}

impl ProblemTwo {
    pub(crate) fn new(problem: Problem) -> ProblemTwo {
        ProblemTwo { problem }
    }
}

impl Solution for ProblemTwo {
    fn solve(&self) -> String {
        let mut map: HashMap<String, (String, String)> = HashMap::new();
        let mut lines = self.problem.lines.clone();
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

        let starts = ends_with(&map.keys().map(|st| st.as_str()).collect(), 'A');
        let mut steps = vec![];

        for start in starts {
            let mut ip = 0;
            let mut cnt = 0;

            let mut st = start.to_string();
            while !is_ending_with(&st, 'Z') {
                st = if instr.get(ip).unwrap() == &'L' {
                    map.get(&st).unwrap().0.clone()
                } else {
                    map.get(&st).unwrap().1.clone()
                };
                cnt += 1;
                ip += 1;
                if ip == instr.len() {
                    ip = 0;
                }
            }
            steps.push(cnt);
        }

        let mut l = *steps.get(0).unwrap() as u64;

        for step in steps {
            l = num::integer::lcm(l, step);
        }

        l.to_string()
    }
}

fn ends_with<'a>(strs: &Vec<&'a str>, ch: char) -> Vec<&'a str> {
    strs.iter()
        .filter(|st| st.chars().nth(2).unwrap() == ch)
        .map(|st| *st)
        .collect()
}

fn is_ending_with(str: &str, ch: char) -> bool {
    str.chars().into_iter().nth(2).unwrap() == ch
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
    use super::{super::Solution, Problem, ProblemTwo};
    use crate::utils::util;

    #[test]
    fn test_sample() {
        let lines = util::lines_in("./src/day8/input3");
        let problem = ProblemTwo::new(Problem { lines });
        assert_eq!("6", problem.solve())
    }

    #[test]
    fn test_input() {
        let lines = util::lines_in("./src/day8/input1");
        let problem = ProblemTwo::new(Problem { lines });
        assert_eq!("13133452426987", problem.solve())
    }
}
