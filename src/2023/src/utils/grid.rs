#![allow(unused)]

use std::{cell, f32::consts::E, io::Read, sync::RwLock};

use anyhow::Result;
use num::{complex::ComplexFloat, Float};

#[derive(PartialEq, Debug, Clone)]
pub struct GridCell<T> {
    pub val: Option<T>,
    pub coord: Coord,
}

impl<T> GridCell<T> {
    fn new(coord: Coord, val: Option<T>) -> GridCell<T> {
        GridCell { val, coord }
    }
}

pub struct Grid<T = ()> {
    grid: Vec<Vec<Option<T>>>,
    pub m: usize,
    pub n: usize,
}

#[derive(Clone, PartialEq, Debug, Hash, Eq)]
pub struct Coord {
    pub p: usize,
    pub q: usize,
}

impl Coord {
    fn new(p: usize, q: usize) -> Coord {
        Coord { p, q }
    }
}

#[derive(Clone, PartialEq)]
pub enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Clone, PartialEq)]
pub struct Neighbor<T> {
    pub dir: Direction,
    pub cell: GridCell<T>,
}

impl<T> Neighbor<T> {
    fn new(cell: GridCell<T>, dir: Direction) -> Neighbor<T> {
        Neighbor { cell, dir }
    }
}

impl<T> Grid<T> {
    pub fn new(m: usize, n: usize) -> Grid<T> {
        let grid = (0..m)
            .into_iter()
            .map(|_| (0..n).into_iter().map(|_| None).collect())
            .collect();

        Grid { grid, m, n }
    }

    pub fn get(&self, coord: &Coord) -> Result<Option<&T>> {
        self.check_bounds(coord)?;
        Ok(self.grid[coord.p][coord.p].as_ref())
    }

    pub fn set(&mut self, coord: &Coord, val: Option<T>) -> Result<()> {
        self.check_bounds(coord)?;
        self.grid[coord.p][coord.q] = val;
        Ok(())
    }

    pub fn fill(&mut self, val: T)
    where
        T: Clone,
    {
        for p in (0..self.m) {
            for q in (0..self.n) {
                self.grid[p][q] = Some(val.clone());
            }
        }
    }

    pub fn find(&self, v: T) -> Coord
    where
        T: Sized + PartialEq,
    {
        for (p, row) in self.grid.iter().enumerate() {
            for (q, cell) in row.iter().enumerate() {
                if let Some(value) = cell {
                    if *value == v {
                        return Coord { p, q };
                    }
                }
            }
        }
        // TODO: Throw error
        Coord {
            p: usize::MAX,
            q: usize::MAX,
        }
    }

    pub fn top(&self, coord: &Coord) -> Result<Option<&T>> {
        self.check_bounds(coord)?;
        if coord.p == 0 {
            return Err(anyhow::anyhow!("No top element"));
        }
        Ok(self.grid[coord.p - 1][coord.q].as_ref())
    }

    pub fn top_cell(&self, coord: &Coord) -> Result<GridCell<&T>> {
        let top = self.top(coord)?;
        Ok(GridCell::new(Coord::new(coord.p - 1, coord.q), top))
    }

    pub fn bottom(&self, coord: &Coord) -> Result<Option<&T>> {
        self.check_bounds(coord)?;
        if coord.p == self.m - 1 {
            return Err(anyhow::anyhow!("No bottom element"));
        }
        Ok(self.grid[coord.p + 1][coord.q].as_ref())
    }

    pub fn bottom_cell(&self, coord: &Coord) -> Result<GridCell<&T>> {
        let bottom = self.bottom(coord)?;
        Ok(GridCell::new(Coord::new(coord.p + 1, coord.q), bottom))
    }

    pub fn left(&self, coord: &Coord) -> Result<Option<&T>> {
        self.check_bounds(coord)?;
        if coord.q == 0 {
            return Err(anyhow::anyhow!("No left element"));
        }
        Ok(self.grid[coord.p][coord.q - 1].as_ref())
    }

    pub fn left_cell(&self, coord: &Coord) -> Result<GridCell<&T>> {
        let left = self.left(coord)?;
        Ok(GridCell::new(Coord::new(coord.p, coord.q - 1), left))
    }

    pub fn right(&self, coord: &Coord) -> Result<Option<&T>> {
        self.check_bounds(coord)?;
        if coord.q == self.n - 1 {
            return Err(anyhow::anyhow!("No right element"));
        }
        Ok(self.grid[coord.p][coord.q + 1].as_ref())
    }

    pub fn right_cell(&self, coord: &Coord) -> Result<GridCell<&T>> {
        let right = self.right(coord)?;
        Ok(GridCell::new(Coord::new(coord.p, coord.q + 1), right))
    }

    fn check_bounds(&self, coord: &Coord) -> Result<()> {
        if coord.p >= self.m || coord.q >= self.n {
            return Err(anyhow::anyhow!("Index out of bounds"));
        }
        Ok(())
    }

    pub fn neighbors(&self, coord: &Coord) -> Vec<Neighbor<&T>> {
        vec![
            self.left_cell(coord)
                .map(|cell| Neighbor::new(cell, Direction::Left)),
            self.right_cell(coord)
                .map(|cell| Neighbor::new(cell, Direction::Right)),
            self.top_cell(coord)
                .map(|cell| Neighbor::new(cell, Direction::Top)),
            self.bottom_cell(coord)
                .map(|cell| Neighbor::new(cell, Direction::Bottom)),
        ]
        .into_iter()
        .filter(|result| result.is_ok())
        .map(|result| result.unwrap())
        .collect()
    }
    pub fn all_neighbors(&self, coord: &Coord) -> Vec<Neighbor<&T>> {
        vec![
            self.left_cell(coord)
                .map(|cell| Neighbor::new(cell, Direction::Left)),
            self.right_cell(coord)
                .map(|cell| Neighbor::new(cell, Direction::Right)),
            self.top_cell(coord)
                .map(|cell| Neighbor::new(cell, Direction::Top)),
            self.bottom_cell(coord)
                .map(|cell| Neighbor::new(cell, Direction::Bottom)),
        ]
        .into_iter()
        .filter(|result| result.is_ok())
        .map(|result| result.unwrap())
        .collect()
    }
}
