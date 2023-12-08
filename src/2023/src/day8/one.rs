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
        let (instr, map) = self.problem.parse();

        let mut start = "AAA";
        let mut ip = 0;
        let mut cnt = 0;

        while start != "ZZZ" {
            start = if instr.get(ip).unwrap() == &'L' {
                map.get(start).unwrap().0.as_str()
            } else {
                map.get(start).unwrap().1.as_str()
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
