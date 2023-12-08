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
        let (instr, map) = self.problem.parse();

        let starts = ends_with(&map.keys().map(|st| *st).collect(), 'A');
        let mut steps = vec![];

        for mut start in starts {
            let mut ip = 0;
            let mut cnt = 0;

            while !is_ending_with(start, 'Z') {
                start = if instr.get(ip).unwrap() == &'L' {
                    map.get(start).unwrap().0
                } else {
                    map.get(start).unwrap().1
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
