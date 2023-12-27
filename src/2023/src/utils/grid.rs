#![allow(unused)]
use std::{collections::HashMap, io::SeekFrom};

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

pub struct Grid<T: Clone = ()> {
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

impl<T: Clone> Grid<T> {
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

    pub fn get_mut(&mut self, coord: &Coord) -> Result<Option<&mut T>> {
        self.check_bounds(coord)?;
        Ok(self.grid[coord.p][coord.p].as_mut())
    }

    pub fn set(&mut self, coord: &Coord, val: Option<T>) -> Result<()> {
        self.check_bounds(coord)?;
        self.grid[coord.p][coord.q] = val;
        Ok(())
    }

    pub fn all(&self) -> Vec<GridCell<T>> {
        let mut all = vec![];
        for p in 0..self.m {
            for q in 0..self.n {
                let val = self.grid.get(p).unwrap().get(q).unwrap().clone();
                all.push(GridCell {
                    val,
                    coord: Coord { p, q },
                })
            }
        }
        all
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

    pub fn duplicate_row(&mut self, row_num: usize) {
        let new_row = self.grid.get(row_num).unwrap().clone();
        self.grid.insert(row_num, new_row);
        self.m += 1;
    }

    pub fn duplicate_column(&mut self, col_num: usize) {
        for row in self.grid.iter_mut() {
            let new_value = row.get(col_num).unwrap().clone();
            row.insert(col_num, new_value);
        }
        self.n += 1;
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
