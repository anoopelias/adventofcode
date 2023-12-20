#![allow(unused)]

use std::{f32::consts::E, io::Read, sync::RwLock};

use anyhow::Result;
use num::{complex::ComplexFloat, Float};

#[derive(PartialEq, Debug)]
pub struct GridCell<T> {
    pub val: Option<T>,
    p: usize,
    q: usize,
}

impl<T> GridCell<T> {
    pub fn to_tuple(&self) -> (usize, usize) {
        (self.p, self.q)
    }
}

impl<T> GridCell<T> {
    fn new(p: usize, q: usize, val: Option<T>) -> GridCell<T> {
        GridCell { val, p, q }
    }
}

pub struct Grid<T = ()> {
    grid: Vec<Vec<Option<T>>>,
    m: usize,
    n: usize,
}

impl<T> Grid<T> {
    pub fn new(m: usize, n: usize) -> Grid<T> {
        let grid = (0..m)
            .into_iter()
            .map(|_| (0..n).into_iter().map(|_| None).collect())
            .collect();

        Grid { grid, m, n }
    }

    pub fn get(&self, p: usize, q: usize) -> Result<Option<&T>> {
        if p >= self.m || q >= self.n {
            return Err(anyhow::anyhow!("Index out of bounds"));
        }
        Ok(self.grid[p][q].as_ref())
    }

    pub fn set(&mut self, p: usize, q: usize, val: Option<T>) -> Result<()> {
        if p >= self.m || q >= self.n {
            return Err(anyhow::anyhow!("Index out of bounds"));
        }
        self.grid[p][q] = val;
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

    pub fn find(&self, v: T) -> (usize, usize)
    where
        T: Sized + PartialEq,
    {
        for (p, row) in self.grid.iter().enumerate() {
            for (q, cell) in row.iter().enumerate() {
                if let Some(value) = cell {
                    if *value == v {
                        return (p, q);
                    }
                }
            }
        }
        // TODO: Throw error
        (usize::MAX, usize::MAX)
    }

    pub fn set_by_tuple(&mut self, coord: (usize, usize), val: Option<T>) -> Result<()> {
        self.set(coord.0, coord.1, val)
    }

    pub fn get_by_tuple(&self, coord: (usize, usize)) -> Result<Option<&T>> {
        self.get(coord.0, coord.1)
    }

    pub fn set_by_cell(&mut self, cell: GridCell<T>) -> Result<()> {
        self.set(cell.p, cell.q, cell.val)
    }

    pub fn get_by_cell(&self, coord: (usize, usize)) -> Result<Option<GridCell<&T>>> {
        let val = self.get(coord.0, coord.1)?;
        Ok(Some(GridCell::new(coord.0, coord.1, val)))
    }

    pub fn top(&self, p: usize, q: usize) -> Result<Option<&T>> {
        self.check_bounds(p, q)?;
        if p == 0 {
            return Err(anyhow::anyhow!("No top element"));
        }
        Ok(self.grid[p - 1][q].as_ref())
    }

    pub fn top_by_tuple(&self, coord: (usize, usize)) -> Result<Option<&T>> {
        self.top(coord.0, coord.1)
    }

    pub fn top_cell(&self, p: usize, q: usize) -> Result<GridCell<&T>> {
        let top = self.top(p, q)?;
        Ok(GridCell::new(p - 1, q, top))
    }

    pub fn top_cell_by_tuple(&self, coord: (usize, usize)) -> Result<GridCell<&T>> {
        self.top_cell(coord.0, coord.1)
    }

    pub fn bottom(&self, p: usize, q: usize) -> Result<Option<&T>> {
        self.check_bounds(p, q)?;
        if p == self.m - 1 {
            return Err(anyhow::anyhow!("No bottom element"));
        }
        Ok(self.grid[p + 1][q].as_ref())
    }

    pub fn bottom_by_tuple(&self, coord: (usize, usize)) -> Result<Option<&T>> {
        self.bottom(coord.0, coord.1)
    }

    pub fn bottom_cell(&self, p: usize, q: usize) -> Result<GridCell<&T>> {
        let bottom = self.bottom(p, q)?;
        Ok(GridCell::new(p + 1, q, bottom))
    }

    pub fn bottom_cell_by_tuple(&self, coord: (usize, usize)) -> Result<GridCell<&T>> {
        self.bottom_cell(coord.0, coord.1)
    }

    pub fn left(&self, p: usize, q: usize) -> Result<Option<&T>> {
        self.check_bounds(p, q)?;
        if q == 0 {
            return Err(anyhow::anyhow!("No left element"));
        }
        Ok(self.grid[p][q - 1].as_ref())
    }

    pub fn left_by_tuple(&self, coord: (usize, usize)) -> Result<Option<&T>> {
        self.left(coord.0, coord.1)
    }

    pub fn left_cell(&self, p: usize, q: usize) -> Result<GridCell<&T>> {
        let left = self.left(p, q)?;
        Ok(GridCell::new(p, q - 1, left))
    }

    pub fn left_cell_by_tuple(&self, coord: (usize, usize)) -> Result<GridCell<&T>> {
        self.left_cell(coord.0, coord.1)
    }

    pub fn right(&self, p: usize, q: usize) -> Result<Option<&T>> {
        self.check_bounds(p, q)?;
        if q == self.n - 1 {
            return Err(anyhow::anyhow!("No right element"));
        }
        Ok(self.grid[p][q + 1].as_ref())
    }

    pub fn right_by_tuple(&self, coord: (usize, usize)) -> Result<Option<&T>> {
        self.right(coord.0, coord.1)
    }

    pub fn right_cell(&self, p: usize, q: usize) -> Result<GridCell<&T>> {
        let right = self.right(p, q)?;
        Ok(GridCell::new(p, q + 1, right))
    }

    pub fn right_cell_by_tuple(&self, coord: (usize, usize)) -> Result<GridCell<&T>> {
        self.right_cell(coord.0, coord.1)
    }

    fn check_bounds(&self, p: usize, q: usize) -> Result<()> {
        if p >= self.m || q >= self.n {
            return Err(anyhow::anyhow!("Index out of bounds"));
        }
        Ok(())
    }

    pub fn neighbor_tuples(&self, p: usize, q: usize) -> Result<Vec<(usize, usize)>> {
        self.check_bounds(p, q)?;
        let mut neighbors = vec![];
        if p > 0 {
            neighbors.push((p - 1, q));
        }
        if q > 0 {
            neighbors.push((p, q - 1));
        }
        if q < self.n - 1 {
            neighbors.push((p, q + 1));
        }
        if p < self.m - 1 {
            neighbors.push((p + 1, q));
        }
        Ok(neighbors)
    }

    pub fn neighbors(&self, p: usize, q: usize) -> Result<Vec<Option<&T>>> {
        let neighbor_tuples = self.neighbor_tuples(p, q)?;
        Ok(neighbor_tuples
            .iter()
            .map(|(p, q)| self.get(*p, *q).unwrap())
            .collect())
    }

    pub fn neighbors_by_tuple(&self, coord: (usize, usize)) -> Result<Vec<Option<&T>>> {
        self.neighbors(coord.0, coord.1)
    }

    pub fn neighbor_tuples_by_tuple(&self, coord: (usize, usize)) -> Result<Vec<(usize, usize)>> {
        self.neighbor_tuples(coord.0, coord.1)
    }

    pub fn neighbor_cells(&self, p: usize, q: usize) -> Result<Vec<GridCell<&T>>> {
        let neighbor_tuples = self.neighbor_tuples(p, q)?;
        Ok(neighbor_tuples
            .iter()
            .map(|(p, q)| GridCell::new(*p, *q, self.get(*p, *q).unwrap()))
            .collect())
    }

    pub fn neighbor_cells_by_tuple(&self, coord: (usize, usize)) -> Result<Vec<GridCell<&T>>> {
        self.neighbor_cells(coord.0, coord.1)
    }

    pub fn all_neighbor_tuples(&self, p: usize, q: usize) -> Result<Vec<(usize, usize)>> {
        self.check_bounds(p, q)?;
        let mut neighbors = vec![];

        if p > 0 {
            if q > 0 {
                neighbors.push((p - 1, q - 1));
            }
            neighbors.push((p - 1, q));
            if q < self.n - 1 {
                neighbors.push((p - 1, q + 1));
            }
        }
        if q > 0 {
            neighbors.push((p, q - 1));
        }
        if q < self.n - 1 {
            neighbors.push((p, q + 1));
        }
        if p < self.m - 1 {
            if q > 0 {
                neighbors.push((p + 1, q - 1));
            }
            neighbors.push((p + 1, q));
            if q < self.n - 1 {
                neighbors.push((p + 1, q + 1));
            }
        }
        Ok(neighbors)
    }

    pub fn all_neighbors(&self, p: usize, q: usize) -> Result<Vec<Option<&T>>> {
        let neighbor_tuples = self.all_neighbor_tuples(p, q)?;
        Ok(neighbor_tuples
            .iter()
            .map(|(p, q)| self.get(*p, *q).unwrap())
            .collect())
    }

    pub fn all_neighbors_by_tuple(&self, coord: (usize, usize)) -> Result<Vec<Option<&T>>> {
        self.all_neighbors(coord.0, coord.1)
    }

    pub fn all_neighbor_cells(&self, p: usize, q: usize) -> Result<Vec<GridCell<&T>>> {
        let neighbor_tuples = self.all_neighbor_tuples(p, q)?;
        Ok(neighbor_tuples
            .iter()
            .map(|(p, q)| GridCell::new(*p, *q, self.get(*p, *q).unwrap()))
            .collect())
    }

    pub fn all_neighbor_tuples_by_tuple(
        &self,
        coord: (usize, usize),
    ) -> Result<Vec<(usize, usize)>> {
        self.all_neighbor_tuples(coord.0, coord.1)
    }

    pub fn all_neighbor_cells_by_tuple(&self, coord: (usize, usize)) -> Result<Vec<GridCell<&T>>> {
        self.all_neighbor_cells(coord.0, coord.1)
    }
}

#[cfg(test)]
mod tests {

    use std::cell::Cell;

    use crate::utils::grid::GridCell;

    use super::Grid;

    #[test]
    fn test_empty_grid() {
        let grid: Grid<i32> = Grid::new(4, 3);
        assert_eq!(None, grid.get(0, 0).unwrap());
        assert!(grid.get(4, 2).is_err());
        assert!(grid.get(1, 3).is_err());
    }

    #[test]
    fn test_set_get() {
        let mut grid: Grid<i32> = Grid::new(4, 3);
        grid.set(0, 0, Some(1)).unwrap();
        assert_eq!(Some(&1), grid.get(0, 0).unwrap());
    }

    #[test]
    fn test_set_get_by_tuple() {
        let mut grid: Grid<i32> = Grid::new(4, 3);
        grid.set_by_tuple((0, 0), Some(1)).unwrap();
        assert_eq!(Some(&1), grid.get_by_tuple((0, 0)).unwrap());
    }

    #[test]
    fn test_set_by_cell() {
        let mut grid: Grid<i32> = Grid::new(4, 3);
        grid.set_by_cell(GridCell::new(0, 0, Some(1))).unwrap();
        assert_eq!(Some(&1), grid.get(0, 0).unwrap());
    }

    #[test]
    fn test_get_by_cell() {
        let mut grid: Grid<i32> = Grid::new(4, 3);
        grid.set(0, 0, Some(1)).unwrap();
        assert_eq!(
            Some(GridCell::new(0, 0, Some(&1))),
            grid.get_by_cell((0, 0)).unwrap()
        );
    }

    #[test]
    fn test_top() {
        let mut grid: Grid<i32> = Grid::new(4, 3);
        grid.set(0, 1, Some(2)).unwrap();
        grid.set(1, 1, Some(1)).unwrap();
        assert!(grid.top(0, 1).is_err());
        assert_eq!(Some(&2), grid.top(1, 1).unwrap());
    }

    #[test]
    fn test_top_by_tuple() {
        let mut grid: Grid<i32> = Grid::new(4, 3);
        grid.set(0, 1, Some(2)).unwrap();
        grid.set(1, 1, Some(1)).unwrap();
        assert!(grid.top_by_tuple((0, 1)).is_err());
        assert_eq!(Some(&2), grid.top_by_tuple((1, 1)).unwrap());
    }

    #[test]
    fn test_bottom() {
        let mut grid: Grid<i32> = Grid::new(4, 3);
        grid.set(0, 1, Some(2)).unwrap();
        grid.set(1, 1, Some(1)).unwrap();
        assert_eq!(Some(&1), grid.bottom(0, 1).unwrap());
        assert!(grid.bottom(3, 1).is_err());
    }

    #[test]
    fn test_bottom_by_tuple() {
        let mut grid: Grid<i32> = Grid::new(4, 3);
        grid.set(0, 1, Some(2)).unwrap();
        grid.set(1, 1, Some(1)).unwrap();
        assert_eq!(Some(&1), grid.bottom_by_tuple((0, 1)).unwrap());
        assert!(grid.bottom_by_tuple((3, 1)).is_err());
    }

    #[test]
    fn test_left() {
        let mut grid: Grid<i32> = Grid::new(4, 3);
        grid.set(0, 1, Some(2)).unwrap();
        grid.set(0, 2, Some(1)).unwrap();
        assert!(grid.left(0, 0).is_err());
        assert_eq!(Some(&2), grid.left(0, 2).unwrap());
    }

    #[test]
    fn test_left_by_tuple() {
        let mut grid: Grid<i32> = Grid::new(4, 3);
        grid.set(0, 1, Some(2)).unwrap();
        grid.set(0, 2, Some(1)).unwrap();
        assert!(grid.left_by_tuple((0, 0)).is_err());
        assert_eq!(Some(&2), grid.left_by_tuple((0, 2)).unwrap());
    }

    #[test]
    fn test_right() {
        let mut grid: Grid<i32> = Grid::new(4, 3);
        grid.set(0, 1, Some(2)).unwrap();
        grid.set(0, 2, Some(1)).unwrap();
        assert_eq!(Some(&1), grid.right(0, 1).unwrap());
        assert!(grid.right(0, 2).is_err());
    }

    #[test]
    fn test_right_by_tuple() {
        let mut grid: Grid<i32> = Grid::new(4, 3);
        grid.set(0, 1, Some(2)).unwrap();
        grid.set(0, 2, Some(1)).unwrap();
        assert_eq!(Some(&1), grid.right_by_tuple((0, 1)).unwrap());
        assert!(grid.right_by_tuple((0, 2)).is_err());
    }

    #[test]
    fn test_top_cell() {
        let mut grid: Grid<i32> = Grid::new(4, 3);
        grid.set(0, 1, Some(2)).unwrap();
        grid.set(1, 1, Some(1)).unwrap();
        assert!(grid.top_cell(0, 1).is_err());
        assert_eq!(GridCell::new(0, 1, Some(&2)), grid.top_cell(1, 1).unwrap());
    }

    #[test]
    fn test_top_cell_by_tuple() {
        let mut grid: Grid<i32> = Grid::new(4, 3);
        grid.set(0, 1, Some(2)).unwrap();
        grid.set(1, 1, Some(1)).unwrap();
        assert!(grid.top_cell_by_tuple((0, 1)).is_err());
        assert_eq!(
            GridCell::new(0, 1, Some(&2)),
            grid.top_cell_by_tuple((1, 1)).unwrap()
        );
    }

    #[test]
    fn test_bottom_cell() {
        let mut grid: Grid<i32> = Grid::new(4, 3);
        grid.set(0, 1, Some(2)).unwrap();
        grid.set(1, 1, Some(1)).unwrap();
        assert_eq!(
            GridCell::new(1, 1, Some(&1)),
            grid.bottom_cell(0, 1).unwrap()
        );
        assert!(grid.bottom_cell(3, 1).is_err());
    }

    #[test]
    fn test_bottom_cell_by_tuple() {
        let mut grid: Grid<i32> = Grid::new(4, 3);
        grid.set(0, 1, Some(2)).unwrap();
        grid.set(1, 1, Some(1)).unwrap();
        assert_eq!(
            GridCell::new(1, 1, Some(&1)),
            grid.bottom_cell_by_tuple((0, 1)).unwrap()
        );
        assert!(grid.bottom_cell_by_tuple((3, 1)).is_err());
    }

    #[test]
    fn test_left_cell() {
        let mut grid: Grid<i32> = Grid::new(4, 3);
        grid.set(0, 1, Some(1)).unwrap();
        grid.set(0, 2, Some(2)).unwrap();
        assert!(grid.left_cell(0, 0).is_err());
        assert_eq!(GridCell::new(0, 1, Some(&1)), grid.left_cell(0, 2).unwrap());
    }

    #[test]
    fn test_left_cell_by_tuple() {
        let mut grid: Grid<i32> = Grid::new(4, 3);
        grid.set(0, 1, Some(1)).unwrap();
        grid.set(0, 2, Some(2)).unwrap();
        assert!(grid.left_cell_by_tuple((0, 0)).is_err());
        assert_eq!(
            GridCell::new(0, 1, Some(&1)),
            grid.left_cell_by_tuple((0, 2)).unwrap()
        );
    }

    #[test]
    fn test_right_cell() {
        let mut grid: Grid<i32> = Grid::new(4, 3);
        grid.set(0, 1, Some(1)).unwrap();
        grid.set(0, 2, Some(2)).unwrap();
        assert_eq!(
            GridCell::new(0, 2, Some(&2)),
            grid.right_cell(0, 1).unwrap()
        );
        assert!(grid.right_cell(0, 2).is_err());
    }

    #[test]
    fn test_right_cell_by_tuple() {
        let mut grid: Grid<i32> = Grid::new(4, 3);
        grid.set(0, 1, Some(1)).unwrap();
        grid.set(0, 2, Some(2)).unwrap();
        assert_eq!(
            GridCell::new(0, 2, Some(&2)),
            grid.right_cell_by_tuple((0, 1)).unwrap()
        );
        assert!(grid.right_cell_by_tuple((0, 2)).is_err());
    }

    #[test]
    fn test_neighbor_tuples() {
        let mut grid: Grid<i32> = Grid::new(4, 3);
        grid.set(0, 1, Some(1)).unwrap();
        grid.set(1, 1, Some(2)).unwrap();
        grid.set(2, 1, Some(3)).unwrap();
        grid.set(1, 0, Some(4)).unwrap();
        grid.set(1, 2, Some(5)).unwrap();

        let neighbor_tuples = grid.neighbor_tuples(1, 1).unwrap();
        assert_eq!(4, neighbor_tuples.len());
        assert!(neighbor_tuples.contains(&(0, 1)));
        assert!(neighbor_tuples.contains(&(1, 0)));
        assert!(neighbor_tuples.contains(&(1, 2)));
        assert!(neighbor_tuples.contains(&(2, 1)));
    }

    #[test]
    fn test_neighbors() {
        let mut grid: Grid<i32> = Grid::new(4, 3);
        grid.set(0, 1, Some(1)).unwrap();
        grid.set(1, 1, Some(2)).unwrap();
        grid.set(2, 1, Some(3)).unwrap();
        grid.set(1, 0, Some(4)).unwrap();
        grid.set(1, 2, Some(5)).unwrap();

        let neighbors = grid.neighbors(1, 1).unwrap();
        assert_eq!(4, neighbors.len());
        assert_eq!(Some(&1), neighbors[0]);
        assert_eq!(Some(&4), neighbors[1]);
        assert_eq!(Some(&5), neighbors[2]);
        assert_eq!(Some(&3), neighbors[3]);
    }

    #[test]
    fn test_neighbors_top() {
        let mut grid: Grid<i32> = Grid::new(4, 3);
        grid.set(1, 1, Some(2)).unwrap();
        grid.set(2, 1, Some(3)).unwrap();
        grid.set(1, 0, Some(4)).unwrap();
        grid.set(1, 2, Some(5)).unwrap();

        let neighbor_tuples = grid.neighbor_tuples(0, 1).unwrap();
        assert_eq!(3, neighbor_tuples.len());
        assert!(neighbor_tuples.contains(&(0, 0)));
        assert!(neighbor_tuples.contains(&(0, 2)));
        assert!(neighbor_tuples.contains(&(1, 1)));
    }

    #[test]
    fn test_neighbors_bottom() {
        let mut grid: Grid<i32> = Grid::new(4, 3);
        grid.set(0, 1, Some(1)).unwrap();
        grid.set(1, 1, Some(2)).unwrap();
        grid.set(1, 0, Some(4)).unwrap();
        grid.set(1, 2, Some(5)).unwrap();

        let neighbor_tuples = grid.neighbor_tuples(3, 1).unwrap();
        assert_eq!(3, neighbor_tuples.len());
        assert!(neighbor_tuples.contains(&(2, 1)));
        assert!(neighbor_tuples.contains(&(3, 0)));
        assert!(neighbor_tuples.contains(&(3, 2)));
    }

    #[test]
    fn test_neighbors_left() {
        let mut grid: Grid<i32> = Grid::new(4, 3);
        grid.set(0, 1, Some(1)).unwrap();
        grid.set(1, 1, Some(2)).unwrap();
        grid.set(2, 1, Some(3)).unwrap();
        grid.set(1, 2, Some(5)).unwrap();

        let neighbor_tuples = grid.neighbor_tuples(1, 0).unwrap();
        assert_eq!(3, neighbor_tuples.len());
        assert!(neighbor_tuples.contains(&(0, 0)));
        assert!(neighbor_tuples.contains(&(1, 1)));
        assert!(neighbor_tuples.contains(&(2, 0)));
    }

    #[test]
    fn test_neighbors_right() {
        let mut grid: Grid<i32> = Grid::new(4, 3);
        grid.set(0, 1, Some(1)).unwrap();
        grid.set(1, 1, Some(2)).unwrap();
        grid.set(2, 1, Some(3)).unwrap();
        grid.set(1, 0, Some(4)).unwrap();

        let neighbor_tuples = grid.neighbor_tuples(1, 2).unwrap();
        assert_eq!(3, neighbor_tuples.len());
        assert!(neighbor_tuples.contains(&(0, 2)));
        assert!(neighbor_tuples.contains(&(1, 1)));
        assert!(neighbor_tuples.contains(&(2, 2)));
    }

    #[test]
    fn test_neighbor_tuples_by_tuple() {
        let mut grid: Grid<i32> = Grid::new(4, 3);
        grid.set(0, 1, Some(1)).unwrap();
        grid.set(1, 1, Some(2)).unwrap();
        grid.set(2, 1, Some(3)).unwrap();
        grid.set(1, 0, Some(4)).unwrap();
        grid.set(1, 2, Some(5)).unwrap();

        let neighbor_tuples = grid.neighbor_tuples_by_tuple((1, 1)).unwrap();
        assert_eq!(4, neighbor_tuples.len());
        assert!(neighbor_tuples.contains(&(0, 1)));
        assert!(neighbor_tuples.contains(&(1, 0)));
        assert!(neighbor_tuples.contains(&(1, 2)));
        assert!(neighbor_tuples.contains(&(2, 1)));
    }

    #[test]
    fn test_neighbor_cells() {
        let mut grid: Grid<i32> = Grid::new(4, 3);
        grid.set(0, 1, Some(1)).unwrap();
        grid.set(1, 1, Some(2)).unwrap();
        grid.set(2, 1, Some(3)).unwrap();
        grid.set(1, 0, Some(4)).unwrap();
        grid.set(1, 2, Some(5)).unwrap();

        let neighbor_cells = grid.neighbor_cells(1, 1).unwrap();
        assert_eq!(4, neighbor_cells.len());
        assert_eq!(GridCell::new(0, 1, Some(&1)), neighbor_cells[0]);
        assert_eq!(GridCell::new(1, 0, Some(&4)), neighbor_cells[1]);
        assert_eq!(GridCell::new(1, 2, Some(&5)), neighbor_cells[2]);
        assert_eq!(GridCell::new(2, 1, Some(&3)), neighbor_cells[3]);
    }

    #[test]
    fn test_neighbor_cells_by_tuple() {
        let mut grid: Grid<i32> = Grid::new(4, 3);
        grid.set(0, 1, Some(1)).unwrap();
        grid.set(1, 1, Some(2)).unwrap();
        grid.set(2, 1, Some(3)).unwrap();
        grid.set(1, 0, Some(4)).unwrap();
        grid.set(1, 2, Some(5)).unwrap();

        let neighbor_cells = grid.neighbor_cells_by_tuple((1, 1)).unwrap();
        assert_eq!(4, neighbor_cells.len());
        assert_eq!(GridCell::new(0, 1, Some(&1)), neighbor_cells[0]);
        assert_eq!(GridCell::new(1, 0, Some(&4)), neighbor_cells[1]);
        assert_eq!(GridCell::new(1, 2, Some(&5)), neighbor_cells[2]);
        assert_eq!(GridCell::new(2, 1, Some(&3)), neighbor_cells[3]);
    }

    #[test]
    fn test_all_neighbor_tuples() {
        let mut grid: Grid<i32> = Grid::new(4, 3);
        grid.set(0, 0, Some(1)).unwrap();
        grid.set(0, 1, Some(2)).unwrap();
        grid.set(0, 2, Some(3)).unwrap();
        grid.set(1, 0, Some(4)).unwrap();
        grid.set(1, 1, Some(5)).unwrap();
        grid.set(1, 2, Some(6)).unwrap();
        grid.set(2, 0, Some(7)).unwrap();
        grid.set(2, 1, Some(8)).unwrap();
        grid.set(2, 2, Some(9)).unwrap();
        grid.set(3, 0, Some(10)).unwrap();
        grid.set(3, 1, Some(11)).unwrap();
        grid.set(3, 2, Some(12)).unwrap();

        let neighbor_tuples = grid.all_neighbor_tuples(1, 1).unwrap();
        assert_eq!(8, neighbor_tuples.len());
        assert!(neighbor_tuples.contains(&(0, 0)));
        assert!(neighbor_tuples.contains(&(0, 1)));
        assert!(neighbor_tuples.contains(&(0, 2)));
        assert!(neighbor_tuples.contains(&(1, 0)));
        assert!(neighbor_tuples.contains(&(1, 2)));
        assert!(neighbor_tuples.contains(&(2, 0)));
        assert!(neighbor_tuples.contains(&(2, 1)));
        assert!(neighbor_tuples.contains(&(2, 2)));
    }

    #[test]
    fn test_all_neighbor_cells() {
        let mut grid: Grid<i32> = Grid::new(4, 3);
        grid.set(0, 0, Some(1)).unwrap();
        grid.set(0, 1, Some(2)).unwrap();
        grid.set(0, 2, Some(3)).unwrap();
        grid.set(1, 0, Some(4)).unwrap();
        grid.set(1, 1, Some(5)).unwrap();
        grid.set(1, 2, Some(6)).unwrap();
        grid.set(2, 0, Some(7)).unwrap();
        grid.set(2, 1, Some(8)).unwrap();
        grid.set(2, 2, Some(9)).unwrap();
        grid.set(3, 0, Some(10)).unwrap();
        grid.set(3, 1, Some(11)).unwrap();
        grid.set(3, 2, Some(12)).unwrap();

        let neighbor_cells = grid.all_neighbor_cells(1, 1).unwrap();
        assert_eq!(8, neighbor_cells.len());
        assert_eq!(GridCell::new(0, 0, Some(&1)), neighbor_cells[0]);
        assert_eq!(GridCell::new(0, 1, Some(&2)), neighbor_cells[1]);
        assert_eq!(GridCell::new(0, 2, Some(&3)), neighbor_cells[2]);
        assert_eq!(GridCell::new(1, 0, Some(&4)), neighbor_cells[3]);
        assert_eq!(GridCell::new(1, 2, Some(&6)), neighbor_cells[4]);
        assert_eq!(GridCell::new(2, 0, Some(&7)), neighbor_cells[5]);
        assert_eq!(GridCell::new(2, 1, Some(&8)), neighbor_cells[6]);
        assert_eq!(GridCell::new(2, 2, Some(&9)), neighbor_cells[7]);
    }
}
