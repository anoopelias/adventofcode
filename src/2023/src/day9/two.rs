use crate::utils::parser::I32Parser;

use super::{Problem, Solution};

pub(crate) struct ProblemTwo<'a> {
    problem: &'a Problem,
}

impl<'a> ProblemTwo<'a> {
    pub(crate) fn new(problem: &Problem) -> ProblemTwo {
        ProblemTwo { problem }
    }
}

impl<'a> Solution for ProblemTwo<'a> {
    fn solve(&self) -> String {
        let mut sum = 0;
        for line in self.problem.lines.iter() {
            let nums = line.parse_i32();

            let mut firsts = vec![nums.first().unwrap().clone()];
            let mut row = nums.clone();
            while row.iter().filter(|n| n == &&0).count() != row.len() {
                let pairs = row
                    .iter()
                    .zip(row[1..].iter())
                    .collect::<Vec<(&i32, &i32)>>();
                row = pairs.iter().map(|(p, q)| *q - *p).collect();
                firsts.push(row.first().unwrap().clone());
            }

            let mut tip = 0;
            firsts.reverse();
            for first in firsts {
                tip = first - tip;
            }

            println!("{}", tip);
            sum += tip;
        }

        sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::{super::Solution, Problem, ProblemTwo};
    use crate::utils::util;

    #[test]
    fn test_sample() {
        let problem = Problem {
            lines: util::lines_in("./src/day9/input"),
        };
        let problem = ProblemTwo::new(&problem);
        assert_eq!("2", problem.solve())
    }

    #[test]
    fn test_input() {
        let problem = Problem {
            lines: util::lines_in("./src/day9/input1"),
        };
        let problem = ProblemTwo::new(&problem);
        assert_eq!("928", problem.solve())
    }
}
