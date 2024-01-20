use anyhow::Error;
use std::{mem::swap, str::FromStr};

#[derive(Clone, PartialEq, Debug, Hash, Eq, Copy)]
pub struct Coord3d {
    pub p: u32,
    pub q: u32,
    pub r: u32,
}

impl FromStr for Coord3d {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split(",");
        let err = || Error::msg("parse error");
        Ok(Coord3d::new(
            splits.next().ok_or(err())?.parse::<u32>()?,
            splits.next().ok_or(err())?.parse::<u32>()?,
            splits.next().ok_or(err())?.parse::<u32>()?,
        ))
    }
}

impl Coord3d {
    pub fn new(p: u32, q: u32, r: u32) -> Coord3d {
        Coord3d { p, q, r }
    }

    fn get(&self, i: usize) -> u32 {
        match i {
            0 => self.p,
            1 => self.q,
            2 => self.r,
            _ => panic!("unknown coord index"),
        }
    }

    fn spread(&self, v: u32, i: usize) -> Coord3d {
        match i {
            0 => Coord3d { p: v, ..*self },
            1 => Coord3d { q: v, ..*self },
            2 => Coord3d { r: v, ..*self },
            _ => panic!("unknown coord index"),
        }
    }

    fn coords_for(&self, i: usize, end: &Coord3d) -> Option<Vec<Coord3d>> {
        let (mut ia, mut ib) = (self.get(i), end.get(i));

        if ia == ib {
            None
        } else {
            if ia > ib {
                swap(&mut ia, &mut ib);
            }

            Some(
                (ia..ib + 1)
                    .map(|p| Coord3d::spread(&self, p, i))
                    .collect::<Vec<_>>(),
            )
        }
    }

    pub fn coords_till(&self, other: &Coord3d) -> Vec<Coord3d> {
        self.coords_for(0, other)
            .or(self.coords_for(1, other))
            .or(self.coords_for(2, other))
            .or(Some(vec![*self]))
            .unwrap()
    }

    pub fn below(&self) -> Option<Coord3d> {
        if self.r == 1 {
            return None;
        } else {
            Some(self.spread(self.r - 1, 2))
        }
    }
}
