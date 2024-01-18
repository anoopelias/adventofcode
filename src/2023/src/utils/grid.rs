#![allow(unused)]
use std::{
    collections::{HashMap, VecDeque},
    fmt::{Debug, Display},
};

use anyhow::Result;
use itertools::Itertools;

use super::util;

#[derive(PartialEq, Debug, Clone)]
pub struct GridCell<T> {
    pub val: T,
    pub coord: Coord,
}

impl<T> GridCell<T> {
    pub fn new(coord: Coord, val: T) -> GridCell<T> {
        GridCell { val, coord }
    }
}

pub struct Grid<T: Clone + PartialEq = ()> {
    grid: Vec<Vec<T>>,
    pub m: usize,
    pub n: usize,
    mx: usize,
    nx: usize,
}

#[derive(Clone, PartialEq, Debug, Hash, Eq, Copy)]
pub struct Coord {
    pub p: usize,
    pub q: usize,
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Coord({}, {})", self.p, self.q)
    }
}

impl Coord {
    pub fn new(p: usize, q: usize) -> Coord {
        Coord { p, q }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Copy)]
pub enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Top => Direction::Bottom,
            Direction::Bottom => Direction::Top,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
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

impl Grid<()> {
    pub fn new(m: usize, n: usize) -> Self {
        let grid = (0..m)
            .into_iter()
            .map(|_| (0..n).into_iter().map(|_| ()).collect())
            .collect();
        Grid {
            grid,
            m,
            n,
            mx: 1,
            nx: 1,
        }
    }
}

impl<T: Clone + PartialEq> Grid<T> {
    pub fn new_fill(m: usize, n: usize, fill: &T) -> Grid<T> {
        let grid = (0..m)
            .into_iter()
            .map(|_| (0..n).into_iter().map(|_| fill.clone()).collect())
            .collect();

        Grid {
            grid,
            m,
            n,
            mx: 1,
            nx: 1,
        }
    }

    pub fn with(grid: Vec<Vec<T>>) -> Grid<T> {
        let (m, n) = (grid.len(), grid.get(0).unwrap().len());
        Grid {
            grid,
            m,
            n,
            mx: 1,
            nx: 1,
        }
    }

    pub fn expand(&mut self, mx: usize, nx: usize) {
        self.mx = mx;
        self.nx = nx;
    }

    pub fn get(&self, coord: &Coord) -> Result<&T> {
        self.check_bounds(coord)?;
        Ok(&self.grid[coord.p % self.m][coord.q % self.n])
    }

    pub fn get_mut(&mut self, coord: &Coord) -> Result<&mut T> {
        self.check_bounds(coord)?;
        Ok(&mut self.grid[coord.p % self.m][coord.q % self.n])
    }

    pub fn set(&mut self, coord: &Coord, val: T) -> Result<()> {
        self.check_bounds(coord)?;
        self.grid[coord.p % self.m][coord.q % self.n] = val;
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

    pub fn m(&self) -> usize {
        self.m * self.mx
    }

    pub fn n(&self) -> usize {
        self.n * self.nx
    }

    pub fn top(&self, coord: &Coord) -> Result<GridCell<&T>> {
        self.check_bounds(coord)?;
        if coord.p == 0 {
            return Err(anyhow::anyhow!("No top element"));
        }
        let coord = Coord::new(coord.p - 1, coord.q);
        let top = &self.get(&coord).unwrap();
        Ok(GridCell::new(coord, top))
    }

    pub fn bottom(&self, coord: &Coord) -> Result<GridCell<&T>> {
        self.check_bounds(coord)?;
        if coord.p == self.m() - 1 {
            return Err(anyhow::anyhow!("No bottom element"));
        }
        let coord = Coord::new(coord.p + 1, coord.q);
        let bottom = &self.get(&coord).unwrap();
        Ok(GridCell::new(coord, bottom))
    }

    pub fn left(&self, coord: &Coord) -> Result<GridCell<&T>> {
        self.check_bounds(coord)?;
        if coord.q == 0 {
            return Err(anyhow::anyhow!("No left element"));
        }
        let coord = Coord::new(coord.p, coord.q - 1);
        let left = &self.get(&coord).unwrap();
        Ok(GridCell::new(coord, left))
    }

    pub fn right(&self, coord: &Coord) -> Result<GridCell<&T>> {
        self.check_bounds(coord)?;
        if coord.q == self.n() - 1 {
            return Err(anyhow::anyhow!("No right element"));
        }
        let coord = Coord::new(coord.p, coord.q + 1);
        let right = &self.get(&coord).unwrap();
        Ok(GridCell::new(coord, right))
    }

    fn check_bounds(&self, coord: &Coord) -> Result<()> {
        if coord.p >= self.m() || coord.q >= self.n() {
            return Err(anyhow::anyhow!("Index out of bounds"));
        }
        Ok(())
    }

    fn check_row_bounds(&self, p: usize) -> Result<()> {
        if p >= self.m() {
            return Err(anyhow::anyhow!("Index out of bounds"));
        }
        Ok(())
    }

    fn check_col_bounds(&self, q: usize) -> Result<()> {
        if q >= self.n() {
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

    pub fn next(&self, coord: &Coord, dir: &Direction) -> Result<GridCell<&T>> {
        self.check_bounds(coord)?;
        match dir {
            Direction::Top => self.top(coord),
            Direction::Bottom => self.bottom(coord),
            Direction::Left => self.left(coord),
            Direction::Right => self.right(coord),
        }
    }

    pub fn bfs(
        &self,
        from: Coord,
        filter_func: impl Fn(&Neighbor<&T>) -> bool,
    ) -> HashMap<Coord, i32> {
        let mut dist_map = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back((from, 0));

        while !queue.is_empty() {
            let (curr, dist) = queue.pop_front().unwrap();

            if !dist_map.contains_key(&curr) {
                dist_map.insert(curr, dist);

                self.neighbors(&curr)
                    .into_iter()
                    .filter(|neighbor| filter_func(neighbor))
                    .for_each(|neighbor| queue.push_back((neighbor.cell.coord, dist + 1)));
            }
        }
        dist_map
    }

    pub fn to_string_by(&self, cell_string: impl Fn(GridCell<&T>) -> String) -> String {
        self.grid
            .iter()
            .enumerate()
            .map(|(p, row)| {
                row.iter()
                    .enumerate()
                    .map(|(q, value)| cell_string(GridCell::new(Coord::new(p, q), value)))
                    .join("")
            })
            .join("\n")
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
