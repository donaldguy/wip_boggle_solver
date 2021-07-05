use std::{
    cell::RefCell,
    sync::{Arc, Weak},
};

use super::cell;

#[derive(PartialEq, Eq, Hash)]
pub(super) struct Inner<T: cell::Value> {
    columns: usize,
    rows: usize,
    pub(super) cells: Vec<cell::Inner<T>>,
}

pub struct Grid<T: cell::Value> {
    inner: Inner<T>,
    self_link: RefCell<Weak<Self>>,
}

impl<T: cell::Value> PartialEq for Grid<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<T: cell::Value> Eq for Grid<T> {}

impl<T: cell::Value> std::hash::Hash for Grid<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state)
    }
}

impl<T: cell::Value> Grid<T> {
    pub fn new<I>(rows: usize, columns: usize, input: I) -> Arc<Self>
    where
        I: IntoIterator<Item = T>,
    {
        let mut value_it = input.into_iter();
        let max_values = value_it.size_hint().1.unwrap();
        if max_values < rows * columns {
            panic!("Attempted to construct Grid with insufficient sized input. Need {} (rows) * {} (columns) = {}, only offered {}", rows, columns, rows * columns, max_values);
        }

        let mut inner = Inner {
            rows,
            columns,
            cells: Vec::with_capacity(rows * columns),
        };

        for row in 0..rows {
            for col in 0..columns {
                let value = value_it.next().unwrap();
                inner.cells.push(cell::Inner { row, col, value });
            }
        }

        let grid = Arc::new(Self {
            inner,
            self_link: RefCell::new(Weak::new()),
        });

        grid.self_link.replace(Arc::downgrade(&grid));
        grid
    }

    pub fn rows(&self) -> usize {
        self.inner.rows
    }

    pub fn columns(&self) -> usize {
        self.inner.columns
    }

    pub(super) fn inner_get(&self, row: usize, col: usize) -> Option<&cell::Inner<T>> {
        self.inner.cells.get(row * self.inner.columns + col)
    }

    pub(super) fn new_cell_pointer_for<'a>(
        &'a self,
        inner: &'a cell::Inner<T>,
    ) -> cell::Pointer<'a, T> {
        cell::Pointer::new(self.self_link.borrow().upgrade().unwrap(), inner)
    }
}

impl<'a, T: cell::Value> std::iter::IntoIterator for &'a Grid<T> {
    type Item = super::CellPointer<'a, T>;

    type IntoIter = std::vec::IntoIter<cell::Pointer<'a, T>>;

    fn into_iter(self) -> Self::IntoIter {
        let mut upgraded = Vec::with_capacity(self.inner.cells.len());

        for c in self.inner.cells.iter() {
            upgraded.push(self.new_cell_pointer_for(c))
        }

        upgraded.into_iter()
    }
}
