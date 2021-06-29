use std::collections::HashSet;

use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct GridCell<T: Eq + Hash> {
    pub value: T,
    pub row: usize,
    pub col: usize,
}

#[derive(Debug)]
pub struct Grid<T: Eq + Hash> {
    columns: usize,
    rows: usize,
    cells: Vec<GridCell<T>>,
}

impl<T: Eq + Hash> Grid<T> {
    pub fn new<I: IntoIterator<Item = T>>(rows: usize, columns: usize, input: I) -> Self {
        let mut value_it = input.into_iter();
        let max_values = value_it.size_hint().1.unwrap();
        if max_values < rows * columns {
            panic!("Attempted to construct Grid with insufficient sized input. Need {} (rows) * {} (columns) = {}, only offered {}", rows, columns, rows * columns, max_values);
        }

        let mut grid: Grid<T> = Self {
            rows,
            columns,
            cells: Vec::with_capacity(rows * columns),
        };

        for row in 0..rows {
            for col in 0..columns {
                let value = value_it.next().unwrap();
                grid.cells.push(GridCell { row, col, value });
            }
        }

        grid
    }

    pub fn cells_set(&self) -> HashSet<&GridCell<T>> {
        self.cells.iter().collect()
    }

    /// get a cell by canonical zero-indexed row and column.
    ///
    /// though cells in memory are linearized, this will return the cell only if referred to by correct index
    /// (e.g. for `n` column grid: `row: 1, col: 0` ; not `row: 0, col: n`)
    /// otherwise `None`. This simplifies the implementation of `get_adjacent_to`
    pub fn get(&self, row: isize, col: isize) -> Option<&GridCell<T>> {
        if row < 0 || col < 0 {
            return None;
        }
        let row = row as usize;
        let col = col as usize;

        match self.cells.get(row * self.columns + col) {
            None => None, // looking before or after the grid
            Some(candidate) => {
                if candidate.row == row && candidate.col == col {
                    Some(candidate)
                } else {
                    // referred to incoherently with a non-extant column or row
                    None
                }
            }
        }
    }

    /// returns iterable of any adjacent cells that exist
    // XXX: I'd prefer this be `cell.get_adjacent` or at least `get_adjacent_to(&self, cell: GridCell)`
    // but that slightly better API was a rabbit hole of borrow checking fighting
    pub fn get_adjacent_to(&self, row: usize, col: usize) -> impl IntoIterator<Item = &GridCell<T>> {
        //clockwise from "12"

        let row = row as isize;
        let col = col as isize;

        let up = self.get(row - 1, col);
        let up_right = self.get(row - 1, col + 1);
        let right = self.get(row, col + 1);
        let down_right = self.get(row + 1, col + 1);
        let down = self.get(row + 1, col);
        let down_left = self.get(row + 1, col - 1);
        let left = self.get(row, col - 1);
        let up_left = self.get(row - 1, col - 1);

        let mut result = Vec::with_capacity(5); //OVER-OPTIMIZATION: a 4x4 grid averages 5 neighbors, I believe
        for maybe_neighbor in &[
            up, up_right, right, down_right, down, down_left, left, up_left,
        ] {
            if let Some(neighbor) = maybe_neighbor {
                result.push(*neighbor)
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn grid_can_be_built() {
        let g: Grid<char> = Grid::new(1, 2, vec!['a', 'b']);
        assert_eq!(g.rows, 1);
        assert_eq!(g.columns, 2);
        assert_eq!(g.get(0, 0).unwrap().value, 'a');
        assert_eq!(g.get(0, 1).unwrap().value, 'b');
    }

    #[test]
    fn cells_can_only_be_got_correctly_indexed() {
        let g = Grid::new(2, 2, vec!['a', 'b', 'c', 'd']);
        assert_eq!(g.get(0, 0).unwrap().value, 'a');
        assert_eq!(g.get(0, 1).unwrap().value, 'b');
        assert_eq!(g.get(1, 0).unwrap().value, 'c');
        assert_eq!(g.get(1, 1).unwrap().value, 'd');

        // this would be 'c' if we weren't checking carefully
        assert!(g.get(0, 2).is_none());

        // whereas these would always be none since they're off the end
        assert!(g.get(1, 2).is_none());
        assert!(g.get(2, 0).is_none());
    }

    #[test]
    fn get_adjacent() {
        // a | b | c | d
        // e | f | g | h
        // i | j | k | l
        // m | n | o | p

        let g: Grid<char> = Grid::new(4, 4, 'a'..='p');

        let by_top_corner: HashSet<&GridCell<char>> = g.get_adjacent_to(0, 0).into_iter().collect();
        assert!(by_top_corner.contains(&GridCell {
            row: 0,
            col: 1,
            value: 'b'
        }));
        assert!(by_top_corner.contains(&GridCell {
            row: 1,
            col: 0,
            value: 'e'
        }));
        assert!(by_top_corner.contains(&GridCell {
            row: 1,
            col: 1,
            value: 'f'
        }));
        assert_eq!(by_top_corner.len(), 3);

        let by_bottom_corner: HashSet<&GridCell<char>> = g.get_adjacent_to(3, 3).into_iter().collect();
        assert!(by_bottom_corner.contains(&GridCell {
            row: 3,
            col: 2,
            value: 'o'
        }));
        assert!(by_bottom_corner.contains(&GridCell {
            row: 2,
            col: 2,
            value: 'k'
        }));
        assert!(by_bottom_corner.contains(&GridCell {
            row: 2,
            col: 3,
            value: 'l'
        }));
        assert_eq!(by_bottom_corner.len(), 3);

        let by_h: HashSet<&GridCell<char>> = g.get_adjacent_to(1, 3).into_iter().collect();
        assert!(by_h.contains(&GridCell {
            row: 0,
            col: 2,
            value: 'c'
        }));
        assert!(by_h.contains(&GridCell {
            row: 0,
            col: 3,
            value: 'd'
        }));
        assert!(by_h.contains(&GridCell {
            row: 1,
            col: 2,
            value: 'g'
        }));
        assert!(by_h.contains(&GridCell {
            row: 2,
            col: 2,
            value: 'k'
        }));
        assert!(by_h.contains(&GridCell {
            row: 2,
            col: 3,
            value: 'l'
        }));
        assert_eq!(by_h.len(), 5);

        let by_f: HashSet<&GridCell<char>> = g.get_adjacent_to(1, 1).into_iter().collect();
        assert!(by_f.contains(&GridCell {
            row: 0,
            col: 0,
            value: 'a'
        }));
        assert!(by_f.contains(&GridCell {
            row: 0,
            col: 1,
            value: 'b'
        }));
        assert!(by_f.contains(&GridCell {
            row: 0,
            col: 2,
            value: 'c'
        }));
        assert!(by_f.contains(&GridCell {
            row: 1,
            col: 0,
            value: 'e'
        }));
        assert!(by_f.contains(&GridCell {
            row: 1,
            col: 2,
            value: 'g'
        }));
        assert!(by_f.contains(&GridCell {
            row: 2,
            col: 0,
            value: 'i'
        }));
        assert!(by_f.contains(&GridCell {
            row: 2,
            col: 1,
            value: 'j'
        }));
        assert!(by_f.contains(&GridCell {
            row: 2,
            col: 2,
            value: 'k'
        }));
        assert_eq!(by_f.len(), 8);
    }
}
