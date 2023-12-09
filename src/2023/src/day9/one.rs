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
        todo!()
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
