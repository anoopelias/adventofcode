#![allow(unused)]
use std::{
    arch::aarch64::vaba_s16,
    collections::HashMap,
    fmt::{Debug, Display},
    io::SeekFrom,
};

use anyhow::{anyhow, Result};
use itertools::{Itertools, MapInto};
use num::{complex::ComplexFloat, Float};

use super::util;

#[derive(PartialEq, Debug, Clone)]
pub struct GridCell<T> {
    pub val: T,
    pub coord: Coord,
}

impl<T> GridCell<T> {
    fn new(coord: Coord, val: T) -> GridCell<T> {
        GridCell { val, coord }
    }
}

pub struct Grid<T: Clone + PartialEq = ()> {
    grid: Vec<Vec<T>>,
    pub m: usize,
    pub n: usize,
}

#[derive(Clone, PartialEq, Debug, Hash, Eq, Copy)]
pub struct Coord {
    pub p: usize,
    pub q: usize,
}

impl Coord {
    pub fn new(p: usize, q: usize) -> Coord {
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

pub struct BfsResult {
    pub from: Coord,
    pub from_map: HashMap<Coord, Option<Coord>>,
    pub dist_map: HashMap<Coord, usize>,
}

impl BfsResult {
    fn new(from: &Coord) -> BfsResult {
        let mut bfs_result = BfsResult {
            from: from.clone(),
            from_map: HashMap::new(),
            dist_map: HashMap::new(),
        };

        bfs_result.from_map.insert(from.clone(), None);
        bfs_result.dist_map.insert(from.clone(), 0);
        bfs_result
    }

    fn add_map(&mut self, coord: &Coord, from: &Coord) {
        let dist = self.dist_map.get(from).unwrap() + 1;
        self.from_map.insert(coord.clone(), Some(from.clone()));
        self.dist_map.insert(coord.clone(), dist);
    }

    fn has(&self, coord: &Coord) -> bool {
        self.from_map.contains_key(coord)
    }
}

impl Grid<()> {
    pub fn new(m: usize, n: usize) -> Self {
        let grid = (0..m)
            .into_iter()
            .map(|_| (0..n).into_iter().map(|_| ()).collect())
            .collect();
        Grid { grid, m, n }
    }
}

impl<T: Clone + PartialEq> Grid<T> {
    pub fn new_fill(m: usize, n: usize, fill: &T) -> Grid<T> {
        let grid = (0..m)
            .into_iter()
            .map(|_| (0..n).into_iter().map(|_| fill.clone()).collect())
            .collect();

        Grid { grid, m, n }
    }

    pub fn get(&self, coord: &Coord) -> Result<&T> {
        self.check_bounds(coord)?;
        Ok(&self.grid[coord.p][coord.q])
    }

    pub fn get_mut(&mut self, coord: &Coord) -> Result<&mut T> {
        self.check_bounds(coord)?;
        Ok(&mut self.grid[coord.p][coord.q])
    }

    pub fn set(&mut self, coord: &Coord, val: T) -> Result<()> {
        self.check_bounds(coord)?;
        self.grid[coord.p][coord.q] = val;
        Ok(())
    }

    pub fn all(&self) -> Vec<GridCell<&T>> {
        let mut all = vec![];
        for p in 0..self.m {
            for q in 0..self.n {
                let coord = Coord { p, q };
                all.push(GridCell {
                    val: self.get(&coord).unwrap(),
                    coord: coord,
                });
            }
        }
        all
    }

    pub fn find_all(&self, v: &T) -> Vec<GridCell<&T>> {
        self.all()
            .into_iter()
            .filter(|cell| cell.val == v)
            .collect()
    }

    pub fn row(&self, p: usize) -> Result<Vec<&T>> {
        self.check_row_bounds(p)?;
        Ok(self.grid.get(p).unwrap().iter().map(|t| t).collect())
    }

    pub fn row_mut(&mut self, p: usize) -> Result<Vec<&mut T>> {
        self.check_row_bounds(p)?;

        Ok(self
            .grid
            .get_mut(p)
            .unwrap()
            .iter_mut()
            .map(|t| t)
            .collect())
    }

    pub fn col(&self, q: usize) -> Result<Vec<&T>> {
        self.check_col_bounds(q)?;
        Ok(self.grid.iter().map(|row| row.get(q).unwrap()).collect())
    }

    pub fn col_mut(&mut self, q: usize) -> Result<Vec<&mut T>> {
        self.check_col_bounds(q)?;
        Ok(self
            .grid
            .iter_mut()
            .map(|row| row.get_mut(q).unwrap())
            .collect())
    }

    pub fn rows(&self) -> Vec<Vec<&T>> {
        (0..self.m)
            .into_iter()
            .map(|p| self.row(p).unwrap())
            .collect()
    }

    pub fn rows_mut(&mut self) -> Vec<Vec<&mut T>> {
        self.grid
            .iter_mut()
            .map(|row| row.iter_mut().collect())
            .collect()
    }

    pub fn cols(&self) -> Vec<Vec<&T>> {
        (0..self.n)
            .into_iter()
            .map(|q| self.col(q).unwrap())
            .collect()
    }

    pub fn cols_mut(&mut self) -> Vec<Vec<&mut T>> {
        util::transpose(self.rows_mut())
    }

    pub fn fill(&mut self, val: T)
    where
        T: Clone,
    {
        for p in (0..self.m) {
            for q in (0..self.n) {
                self.grid[p][q] = val.clone();
            }
        }
    }

    pub fn find(&self, v: T) -> Option<Coord>
    where
        T: Sized + PartialEq,
    {
        for (p, row) in self.grid.iter().enumerate() {
            for (q, cell) in row.iter().enumerate() {
                if *cell == v {
                    return Some(Coord { p, q });
                }
            }
        }
        None
    }

    pub fn top(&self, coord: &Coord) -> Result<GridCell<&T>> {
        self.check_bounds(coord)?;
        if coord.p == 0 {
            return Err(anyhow::anyhow!("No top element"));
        }
        let top = &self.grid[coord.p - 1][coord.q];
        Ok(GridCell::new(Coord::new(coord.p - 1, coord.q), top))
    }

    pub fn bottom(&self, coord: &Coord) -> Result<GridCell<&T>> {
        self.check_bounds(coord)?;
        if coord.p == self.m - 1 {
            return Err(anyhow::anyhow!("No bottom element"));
        }
        let bottom = &self.grid[coord.p + 1][coord.q];
        Ok(GridCell::new(Coord::new(coord.p + 1, coord.q), bottom))
    }

    pub fn left(&self, coord: &Coord) -> Result<GridCell<&T>> {
        self.check_bounds(coord)?;
        if coord.q == 0 {
            return Err(anyhow::anyhow!("No left element"));
        }
        let left = &self.grid[coord.p][coord.q - 1];
        Ok(GridCell::new(Coord::new(coord.p, coord.q - 1), left))
    }

    pub fn right(&self, coord: &Coord) -> Result<GridCell<&T>> {
        self.check_bounds(coord)?;
        if coord.q == self.n - 1 {
            return Err(anyhow::anyhow!("No right element"));
        }
        let right = &self.grid[coord.p][coord.q + 1];
        Ok(GridCell::new(Coord::new(coord.p, coord.q + 1), right))
    }

    fn check_bounds(&self, coord: &Coord) -> Result<()> {
        if coord.p >= self.m || coord.q >= self.n {
            return Err(anyhow::anyhow!("Index out of bounds"));
        }
        Ok(())
    }

    fn check_row_bounds(&self, p: usize) -> Result<()> {
        if p >= self.m {
            return Err(anyhow::anyhow!("Index out of bounds"));
        }
        Ok(())
    }

    fn check_col_bounds(&self, q: usize) -> Result<()> {
        if q >= self.n {
            return Err(anyhow::anyhow!("Index out of bounds"));
        }
        Ok(())
    }

    pub fn neighbors(&self, coord: &Coord) -> Vec<Neighbor<&T>> {
        vec![
            self.left(coord)
                .map(|cell| Neighbor::new(cell, Direction::Left)),
            self.right(coord)
                .map(|cell| Neighbor::new(cell, Direction::Right)),
            self.top(coord)
                .map(|cell| Neighbor::new(cell, Direction::Top)),
            self.bottom(coord)
                .map(|cell| Neighbor::new(cell, Direction::Bottom)),
        ]
        .into_iter()
        .filter(|result| result.is_ok())
        .map(|result| result.unwrap())
        .collect()
    }

    pub fn duplicate_row(&mut self, row_num: usize) -> Result<()> {
        self.check_row_bounds(row_num)?;
        let new_row = self.grid.get(row_num).unwrap().clone();
        self.grid.insert(row_num, new_row);
        self.m += 1;
        Ok(())
    }

    pub fn duplicate_column(&mut self, col_num: usize) -> Result<()> {
        self.check_col_bounds(col_num)?;
        for row in self.grid.iter_mut() {
            let new_value = row.get(col_num).unwrap().clone();
            row.insert(col_num, new_value);
        }
        self.n += 1;
        Ok(())
    }

    pub fn delete_row(&mut self, p: usize) -> Result<Vec<T>> {
        self.check_row_bounds(p)?;
        self.m -= 1;
        Ok(self.grid.remove(p))
    }

    pub fn delete_col(&mut self, q: usize) -> Result<Vec<T>> {
        self.check_col_bounds(q)?;
        let mut col = vec![];
        for row in self.grid.iter_mut() {
            col.push(row.remove(q));
        }
        self.n -= 1;
        Ok(col)
    }

    pub fn bfs(&self, from: &Coord) -> BfsResult {
        let mut bfs_result = BfsResult::new(from);

        let mut nexts = vec![from.clone()];

        while nexts.len() > 0 {
            let curr = nexts.remove(0);
            let mut neighbors = self.neighbors(&curr);

            while neighbors.len() != 0 {
                let neighbor = neighbors.pop().unwrap();
                let neighbor_coord = neighbor.cell.coord;
                if !bfs_result.has(&neighbor_coord) {
                    bfs_result.add_map(&neighbor_coord, &curr);
                    nexts.push(neighbor_coord);
                }
            }
        }

        bfs_result
    }
}

impl<T: Clone + PartialEq + Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.grid
                .iter()
                .map(|row| row.iter().map(|cell| cell.to_string()).join("").to_string())
                .join("\n")
                .to_string()
        )
    }
}
