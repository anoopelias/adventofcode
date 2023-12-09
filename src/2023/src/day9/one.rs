use crate::utils::parser::I32Parser;

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
        let mut sum = 0;
        for line in self.problem.lines.iter() {
            let nums = line.parse_i32();

            let mut lasts = vec![nums.last().unwrap().clone()];
            let mut row = nums.clone();
            while row.iter().filter(|n| n == &&0).count() != row.len() {
                let pairs = row
                    .iter()
                    .zip(row[1..].iter())
                    .collect::<Vec<(&i32, &i32)>>();
                row = pairs.iter().map(|(p, q)| *q - *p).collect();
                lasts.push(row.last().unwrap().clone());
            }

            let mut tip = 0;
            lasts.reverse();
            for last in lasts {
                tip += last;
            }

            println!("{}", tip);
            sum += tip;
        }

        sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::{super::Solution, Problem, ProblemOne};
    use crate::utils::util;

    #[test]
    fn test_sample() {
        let lines = util::lines_in("./src/day9/input");
        let problem = ProblemOne::new(Problem { lines });
        assert_eq!("114", problem.solve())
    }

    #[test]
    fn test_input() {
        let lines = util::lines_in("./src/day9/input1");
        let problem = ProblemOne::new(Problem { lines });
        assert_eq!("1974232246", problem.solve())
    }
}
