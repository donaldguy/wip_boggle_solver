pub mod cell;
mod structs;

pub use cell::Pointer as CellPointer;
pub use structs::Grid;

use std::{collections::HashSet, iter::FromIterator};

impl<'a, T: cell::Value> Grid<T> {
    /// get a cell by canonical zero-indexed row and column.
    ///
    /// though cells in memory are linearized, this will return the cell only if referred to by correct index
    /// (e.g. for `n` column grid: `row: 1, col: 0` ; not `row: 0, col: n`)
    /// otherwise `None`. This simplifies the implementation of `get_adjacent_to`
    pub fn get(&'a self, row: isize, col: isize) -> Option<CellPointer<'a, T>> {
        if row < 0 || col < 0 {
            return None;
        }
        let row = row as usize;
        let col = col as usize;

        match self.inner_get(row, col) {
            None => None, // looking before or after the grid
            Some(candidate) => {
                if candidate.row == row && candidate.col == col {
                    Some(self.new_cell_pointer_for(candidate))
                } else {
                    // referred to incoherently with a non-extant column or row
                    None
                }
            }
        }
    }

    fn get_adjacent_to(&'a self, cell: &CellPointer<'a, T>) -> HashSet<CellPointer<'a, T>> {
        self.get_adjacent_to_in(cell, &HashSet::from_iter(self.into_iter()))
    }

    /// returns iterable of any adjacent cells that exist within an allowed set (generally as-yet-unused)
    fn get_adjacent_to_in(
        &'a self,
        cell: &CellPointer<'a, T>,
        allowed_cells: &HashSet<CellPointer<'a, T>>,
    ) -> HashSet<CellPointer<'a, T>> {
        let row = cell.row() as isize;
        let col = cell.col() as isize;

        //clockwise from "12"

        let up = self.get(row - 1, col);
        let up_right = self.get(row - 1, col + 1);
        let right = self.get(row, col + 1);
        let down_right = self.get(row + 1, col + 1);
        let down = self.get(row + 1, col);
        let down_left = self.get(row + 1, col - 1);
        let left = self.get(row, col - 1);
        let up_left = self.get(row - 1, col - 1);

        let mut result = HashSet::with_capacity(5); //OVER-OPTIMIZATION: a 4x4 grid averages 5 neighbors, I believe
        for maybe_neighbor in vec![
            up, up_right, right, down_right, down, down_left, left, up_left,
        ] {
            if let Some(neighbor) = maybe_neighbor {
                if allowed_cells.contains(&neighbor) {
                    result.insert(neighbor);
                }
            }
        }

        result
    }
}

impl<'a, T: cell::Value> CellPointer<'a, T> {
    pub fn get_adjacent_cells(&'a self) -> HashSet<Self> {
        self.grid().get_adjacent_to(self)
    }

    pub fn get_adjacent_cell_in(&'a self, allowed_cells: &HashSet<Self>) -> HashSet<Self> {
        self.grid().get_adjacent_to_in(self, allowed_cells)
    }
}

#[cfg(test)]
mod tests {
    use super::CellPointer;
    use super::Grid;
    use std::collections::HashSet;

    #[test]
    fn can_be_built() {
        let g = Grid::new(1, 2, vec!['a', 'b']);
        assert_eq!(g.rows(), 1);
        assert_eq!(g.columns(), 2);
        assert_eq!(g.get(0, 0).unwrap().value(), &'a');
        assert_eq!(g.get(0, 1).unwrap().value(), &'b');
    }

    #[test]
    fn cells_can_only_be_got_correctly_indexed() {
        let g = Grid::new(2, 2, vec!['a', 'b', 'c', 'd']);
        assert_eq!(g.get(0, 0).unwrap().value(), &'a');
        assert_eq!(g.get(0, 1).unwrap().value(), &'b');
        assert_eq!(g.get(1, 0).unwrap().value(), &'c');
        assert_eq!(g.get(1, 1).unwrap().value(), &'d');

        // this would be 'c' if we weren't checking carefully
        assert!(g.get(0, 2).is_none());

        // whereas these would always be none since they're off the end
        assert!(g.get(1, 2).is_none());
        assert!(g.get(2, 0).is_none());
    }

    #[test]
    fn get_adjacent() {
        let grid = Grid::new(4, 4, 'a'..='p');

        // a | b | c | d
        // e | f | g | h
        // i | j | k | l
        // m | n | o | p
        let a = grid.get(0, 0).unwrap();
        let b = grid.get(0, 1).unwrap();
        let c = grid.get(0, 2).unwrap();
        let d = grid.get(0, 3).unwrap();
        let e = grid.get(1, 0).unwrap();
        let f = grid.get(1, 1).unwrap();
        let g = grid.get(1, 2).unwrap();
        let h = grid.get(1, 3).unwrap();
        let i = grid.get(2, 0).unwrap();
        let j = grid.get(2, 1).unwrap();
        let k = grid.get(2, 2).unwrap();
        let l = grid.get(2, 3).unwrap();
        let m = grid.get(3, 0).unwrap();
        let n = grid.get(3, 1).unwrap();
        let o = grid.get(3, 2).unwrap();
        let p = grid.get(3, 3).unwrap();

        let by_top_corner: HashSet<CellPointer<char>> =
            grid.get_adjacent_to(&a).into_iter().collect();
        assert!(by_top_corner.contains(&b));
        assert!(by_top_corner.contains(&e));
        assert!(by_top_corner.contains(&f));
        assert_eq!(by_top_corner.len(), 3);

        let by_bottom_corner: HashSet<CellPointer<char>> = grid.get_adjacent_to(&p);
        assert!(by_bottom_corner.contains(&o));
        assert!(by_bottom_corner.contains(&k));
        assert!(by_bottom_corner.contains(&l));
        assert_eq!(by_bottom_corner.len(), 3);

        let by_i = grid.get_adjacent_to(&i);
        assert!(by_i.contains(&e));
        assert!(by_i.contains(&f));
        assert!(by_i.contains(&j));
        assert!(by_i.contains(&n));
        assert!(by_i.contains(&m));
        assert_eq!(by_i.len(), 5);

        let by_h = h.get_adjacent_cells();
        assert!(by_h.contains(&c));
        assert!(by_h.contains(&d));
        assert!(by_h.contains(&g));
        assert!(by_h.contains(&k));
        assert!(by_h.contains(&l));
        assert_eq!(by_h.len(), 5);

        let by_f = f.get_adjacent_cells();
        assert!(by_f.contains(&a));
        assert!(by_f.contains(&b));
        assert!(by_f.contains(&c));
        assert!(by_f.contains(&e));
        assert!(by_f.contains(&g));
        assert!(by_f.contains(&i));
        assert!(by_f.contains(&j));
        assert!(by_f.contains(&k));
        assert_eq!(by_f.len(), 8);
    }
}
