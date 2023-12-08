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

        let mut starts = ends_with(&map.keys().map(|st| st.as_str()).collect(), 'A');
        let len = starts.len();

        let mut ip = 0;
        let mut cnt = 0;

        while ends_with(&starts, 'Z').len() != len {
            let mut new_starts = vec![];
            for start in starts {
                let new_start = if instr.get(ip).unwrap() == &'L' {
                    map.get(start).unwrap().0.as_str()
                } else {
                    map.get(start).unwrap().1.as_str()
                };
                new_starts.push(new_start);
            }
            starts = new_starts;
            cnt += 1;
            ip += 1;
            if ip == instr.len() {
                ip = 0;
            }
        }

        cnt.to_string()
    }
}

fn ends_with<'a>(strs: &Vec<&'a str>, ch: char) -> Vec<&'a str> {
    strs.iter()
        .filter(|st| st.chars().nth(2).unwrap() == ch)
        .map(|st| *st)
        .collect()
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
        assert_eq!("249781879", problem.solve())
    }
}
