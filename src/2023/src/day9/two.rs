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
        todo!()
    }
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
