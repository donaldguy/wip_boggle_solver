use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

mod cell;
mod two_d;

pub use cell::Pointer as GridCell;
pub use two_d::Grid;

impl<'a, T: Eq + Hash + Debug> GridCell<'a, T> {
    fn get_adjacent_cells(&'a self) -> HashSet<Self> {
        self.grid().get_adjacent_to(self)
    }

    fn get_adjacent_cell_in(&'a self, allowed_cells: &HashSet<Self>) -> HashSet<Self> {
        self.grid().get_adjacent_to_in(self, allowed_cells)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

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

        let by_top_corner: HashSet<GridCell<char>> = grid.get_adjacent_to(&a).into_iter().collect();
        assert!(by_top_corner.contains(&b));
        assert!(by_top_corner.contains(&e));
        assert!(by_top_corner.contains(&f));
        assert_eq!(by_top_corner.len(), 3);

        let by_bottom_corner: HashSet<GridCell<char>> = grid.get_adjacent_to(&p);
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
