use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::{Arc, Weak};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Inner<T: Eq + Hash + Debug> {
    columns: usize,
    rows: usize,
    cells: Vec<super::cell::Inner<T>>,
}

pub struct Grid<T: Eq + Hash + Debug> {
    inner: Inner<T>,
    self_link: RefCell<Weak<Self>>,
}

impl<T: Eq + Hash + Debug> PartialEq for Grid<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<T: Eq + Hash + Debug> Eq for Grid<T> {}

impl<T: Eq + Hash + Debug> Hash for Grid<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state)
    }
}

impl<T: Eq + Hash + Debug> Grid<T> {
    pub fn new<I: IntoIterator<Item = T>>(rows: usize, columns: usize, input: I) -> Arc<Self> {
        let mut value_it = input.into_iter();
        let max_values = value_it.size_hint().1.unwrap();
        if max_values < rows * columns {
            panic!("Attempted to construct Grid with insufficient sized input. Need {} (rows) * {} (columns) = {}, only offered {}", rows, columns, rows * columns, max_values);
        }

        let mut inner = self::Inner {
            rows,
            columns,
            cells: Vec::with_capacity(rows * columns),
        };

        for row in 0..rows {
            for col in 0..columns {
                let value = value_it.next().unwrap();
                inner.cells.push(super::cell::Inner { row, col, value });
            }
        }

        let grid = Arc::new(Self {
            inner,
            self_link: RefCell::new(Weak::new()),
        });

        grid.self_link.replace(Arc::downgrade(&grid));
        grid
    }

    fn self_link(&self) -> Arc<Self> {
        self.self_link.borrow().upgrade().unwrap()
    }

    /// get a cell by canonical zero-indexed row and column.
    ///
    /// though cells in memory are linearized, this will return the cell only if referred to by correct index
    /// (e.g. for `n` column grid: `row: 1, col: 0` ; not `row: 0, col: n`)
    /// otherwise `None`. This simplifies the implementation of `get_adjacent_to`
    pub fn get(&self, row: isize, col: isize) -> Option<super::GridCell<T>> {
        if row < 0 || col < 0 {
            return None;
        }
        let row = row as usize;
        let col = col as usize;

        match self.inner.cells.get(row * self.inner.columns + col) {
            None => None, // looking before or after the grid
            Some(candidate) => {
                if candidate.row == row && candidate.col == col {
                    Some(super::cell::Pointer::new(self.self_link(), candidate))
                } else {
                    // referred to incoherently with a non-extant column or row
                    None
                }
            }
        }
    }

    pub fn all_cells(&self) -> HashSet<super::GridCell<T>> {
        self.inner
            .cells
            .iter()
            .map(|c| super::cell::Pointer::new(self.self_link(), c))
            .collect()
    }

    pub fn get_adjacent_to(&self, cell: &super::GridCell<T>) -> HashSet<super::GridCell<T>> {
        self.get_adjacent_to_in(cell, &self.all_cells())
    }

    /// returns iterable of any adjacent cells that exist within an allowed set (generally as-yet-unused)
    pub fn get_adjacent_to_in(
        &self,
        cell: &super::GridCell<T>,
        allowed_cells: &HashSet<super::GridCell<T>>,
    ) -> HashSet<super::GridCell<T>> {
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

#[cfg(test)]
mod tests {
    #[test]
    fn can_be_built() {
        let g = super::Grid::new(1, 2, vec!['a', 'b']);
        assert_eq!(g.inner.rows, 1);
        assert_eq!(g.inner.columns, 2);
        assert_eq!(g.get(0, 0).unwrap().value(), &'a');
        assert_eq!(g.get(0, 1).unwrap().value(), &'b');
    }

    #[test]
    fn cells_can_only_be_got_correctly_indexed() {
        let g = super::Grid::new(2, 2, vec!['a', 'b', 'c', 'd']);
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
}
