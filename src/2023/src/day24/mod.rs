const DAY: &str = "day24";

use anyhow::Error;

use crate::utils::util;
use std::str::FromStr;
use std::time::Instant;

#[allow(unused)]
pub(crate) fn solve() -> String {
    let lines = util::lines_in(&format!("./src/{}/input1", DAY));
    let time = Instant::now();
    let part1 = part1(&lines, 200000000000000.0, 400000000000000.0);
    let elapsed1 = time.elapsed();
    println!("result1: {} in {:?}", part1, elapsed1);

    let time = Instant::now();
    let part2 = part2(&lines);
    let elapsed2 = time.elapsed();
    return format!(
        "result1: {} in {:?} \nresult2: {} in {:?}",
        part1, elapsed1, part2, elapsed2,
    );
}

struct Hail {
    pos: Vec3d,
    vel: Vec3d,
}

impl Hail {
    fn m2d(&self) -> f64 {
        self.vel.q / self.vel.p
    }

    fn c2d(&self) -> f64 {
        self.pos.q - (self.m2d() * self.pos.p)
    }

    fn inter2d(&self, other: &Hail) -> (f64, f64) {
        let p = (self.c2d() - other.c2d()) / (other.m2d() - self.m2d());
        let q = (self.m2d() * p) + self.c2d();
        (p, q)
    }

    fn is_n_future(&self, v: f64, n: usize) -> bool {
        let diff = (self.pos.get(n) - v).abs();
        let diff_new = (self.pos.get(n) + self.vel.get(n) - v).abs();
        diff_new < diff
    }

    fn is_future(&self, p: f64, q: f64) -> bool {
        if self.vel.p != 0.0 {
            self.is_n_future(p, 0)
        } else {
            self.is_n_future(q, 1)
        }
    }
}

fn part1(lines: &Vec<String>, min: f64, max: f64) -> String {
    let mut hails = vec![];
    for line in lines {
        let (pos_str, vel_str) = line.split_once(" @ ").unwrap();
        hails.push(Hail {
            pos: parse_vec_str(pos_str),
            vel: parse_vec_str(vel_str),
        })
    }

    let mut count = 0;
    for i in 0..hails.len() {
        for j in i + 1..hails.len() {
            let (p, q) = hails[i].inter2d(&hails[j]);
            if p > min
                && p < max
                && q > min
                && q < max
                && hails[i].is_future(p, q)
                && hails[j].is_future(p, q)
            {
                count += 1;
            }
        }
    }

    count.to_string()
}

fn parse_vec_str(s: &str) -> Vec3d {
    let mut splits = s.split(", ");
    Vec3d::new(
        splits.next().unwrap().trim().parse::<f64>().unwrap(),
        splits.next().unwrap().trim().parse::<f64>().unwrap(),
        splits.next().unwrap().trim().parse::<f64>().unwrap(),
    )
}

fn part2(lines: &Vec<String>) -> String {
    "".to_string()
}

#[derive(Clone, PartialEq, Debug, Copy)]
pub struct Vec3d {
    pub p: f64,
    pub q: f64,
    pub r: f64,
}

impl FromStr for Vec3d {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split(",");
        let err = || Error::msg("parse error");
        Ok(Vec3d::new(
            splits.next().ok_or(err())?.parse::<f64>()?,
            splits.next().ok_or(err())?.parse::<f64>()?,
            splits.next().ok_or(err())?.parse::<f64>()?,
        ))
    }
}

impl Vec3d {
    pub fn new(p: f64, q: f64, r: f64) -> Vec3d {
        Vec3d { p, q, r }
    }

    fn get(&self, i: usize) -> f64 {
        match i {
            0 => self.p,
            1 => self.q,
            2 => self.r,
            _ => panic!("unknown coord index"),
        }
    }
}

#[cfg(test)]
mod tests {
    use clearcheck::assertions::equal::EqualityAssertion;

    use super::{part1, part2, DAY};
    use crate::utils::util;

    #[test]
    fn test_part1_sample() {
        let lines = util::lines_in(&format!("./src/{}/input", DAY));
        part1(&lines, 7.0, 27.0).should_equal("2");
    }

    #[test]
    fn test_part1_input() {
        let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        part1(&lines, 200000000000000.0, 400000000000000.0).should_equal("16050");
    }

    #[test]
    fn test_part2_sample() {
        let lines = util::lines_in(&format!("./src/{}/input", DAY));
        // part2(&lines).should_equal("154");
    }

    #[test]
    fn test_part2_input() {
        let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        // part2(&lines).should_equal("6658");
    }
}
