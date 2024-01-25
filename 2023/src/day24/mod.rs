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

    fn is_n_future(&self, a: f64, n: usize) -> bool {
        // Check if the 'velocity' and 'distance to a' has same sign
        (a - self.pos.get(n)) * self.vel.get(n) > 0.0
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
    /*

    x1 + (t1 * vx) = rx + (t1 * vrx)
    (t1 * vx) - (t1 * vrx) = rx - x1
    t1 = (rx - x1) / (vx - vrx)
    t1 = (ry - y1) / (vy - vry)

    (rx - x1) / (vx - vrx) = (ry - y1) / (vy - vry)
    (rx - x1) * (vy - vry) = (ry - y1) * (vx - vrx)
    (rx vy) - (x1 vy) - (rx vry) + (x1 vry) = (ry vx) - (y1 vx) - (ry vrx) + (y1 vrx)


    (rx vy) - (ry vx) - (y1 vrx) + (x1 vry) + (y1 vx) - (x1 vy) = (rx vry) - (ry vrx)
     */

    let mut hails = vec![];
    for line in lines {
        let (pos_str, vel_str) = line.split_once(" @ ").unwrap();
        hails.push(Hail {
            pos: parse_vec_str(pos_str),
            vel: parse_vec_str(vel_str),
        });
    }

    let mut eqs_xy = vec![
        coeffs(&hails[0], &hails[1], 0, 1),
        coeffs(&hails[0], &hails[2], 0, 1),
        coeffs(&hails[0], &hails[3], 0, 1),
        coeffs(&hails[0], &hails[4], 0, 1),
    ];

    /*
    2x + 1y + -6u + 1v + -44 = 0
    3x + -0y + -12u + -1v + -35 = 0
    3x + 1y + -18u + 7v + -38 = 0
    6x + 3y + -6u + -1v + -164 = 0

    0x + 1y + 8u + 1v + -36 = 0
    2x + -0y + -4u + -1v + -34 = 0
    -1x + 1y + 2u + 7v + -6 = 0
    1x + 3y + 15u + -1v + -97 = 0
     */

    println!(
        "{}\n{}\n{}\n{}",
        to_eq(&eqs_xy[0]),
        to_eq(&eqs_xy[1]),
        to_eq(&eqs_xy[2]),
        to_eq(&eqs_xy[3]),
    );
    println!();
    let result_xy = gaussian_elimination(&mut eqs_xy);

    let mut eqs_xz = vec![
        coeffs(&hails[0], &hails[2], 1, 2),
        coeffs(&hails[0], &hails[1], 1, 2),
        coeffs(&hails[0], &hails[3], 1, 2),
        coeffs(&hails[0], &hails[4], 1, 2),
    ];

    println!(
        "{}\n{}\n{}\n{}",
        to_eq(&eqs_xz[0]),
        to_eq(&eqs_xz[1]),
        to_eq(&eqs_xz[2]),
        to_eq(&eqs_xz[3]),
    );
    let result_xz = gaussian_elimination(&mut eqs_xz);

    (result_xy[0] + result_xy[1] + result_xz[1])
        .round()
        .to_string()
}

pub fn gaussian_elimination(matrix: &mut [Vec<f64>]) -> Vec<f64> {
    let size = matrix.len();
    assert_eq!(size, matrix[0].len() - 1);

    for i in 0..size - 1 {
        for j in i..size - 1 {
            echelon(matrix, i, j);
        }
    }

    for i in (1..size).rev() {
        eliminate(matrix, i);
    }

    for i in 0..size {
        if matrix[i][i] == 0f64 {
            println!("Infinitely many solutions");
        }
    }

    let mut result: Vec<f64> = vec![0f64; size];
    for i in 0..size {
        result[i] = matrix[i][size] / matrix[i][i];
    }
    result
}

fn echelon(matrix: &mut [Vec<f64>], i: usize, j: usize) {
    let size = matrix.len();
    if matrix[i][i] == 0f64 {
    } else {
        let factor = matrix[j + 1][i] / matrix[i][i];
        (i..size + 1).for_each(|k| {
            matrix[j + 1][k] -= factor * matrix[i][k];
        });
    }
}

fn eliminate(matrix: &mut [Vec<f64>], i: usize) {
    let size = matrix.len();
    if matrix[i][i] == 0f64 {
    } else {
        for j in (1..i + 1).rev() {
            let factor = matrix[j - 1][i] / matrix[i][i];
            for k in (0..size + 1).rev() {
                matrix[j - 1][k] -= factor * matrix[i][k];
            }
        }
    }
}

fn to_eq(coeffs: &Vec<f64>) -> String {
    format!(
        "{}x + {}y + {}u + {}v = {}",
        coeffs[0], coeffs[1], coeffs[2], coeffs[3], coeffs[4]
    )
}

fn coeffs(hail_a: &Hail, hail_b: &Hail, c0: usize, c1: usize) -> Vec<f64> {
    // (rx vy) - (ry vx) - (y1 vrx) + (x1 vry) + (y1 vx) - (x1 vy) = (rx vry) - (ry vrx)
    vec![
        // (rx vy)
        hail_a.vel.get(c1) - hail_b.vel.get(c1),
        // - (ry vx)
        -(hail_a.vel.get(c0) - hail_b.vel.get(c0)),
        // - (y1 vrx)
        hail_a.pos.get(c1) - hail_b.pos.get(c1),
        // (x1 vry)
        hail_a.pos.get(c0) - hail_b.pos.get(c0),
        // (y1 vx)
        -((hail_a.pos.get(c1) * hail_a.vel.get(c0)) - (hail_b.pos.get(c1) * hail_b.vel.get(c0))
        // - (x1 vy)
        - ((hail_a.pos.get(c0) * hail_a.vel.get(c1)) - (hail_b.pos.get(c0) * hail_b.vel.get(c1)))),
    ]
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
        part2(&lines).should_equal("47");
    }

    #[test]
    fn test_part2_input() {
        let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        part2(&lines).should_equal("669042940632377");
    }
}
