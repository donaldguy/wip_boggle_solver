use std::fmt::Debug;
use std::hash::Hash;
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Inner<T> {
    pub value: T,
    pub row: usize,
    pub col: usize,
}

#[derive(Clone)]
pub struct Pointer<'grid, T: Eq + Hash + Debug> {
    grid: Arc<super::Grid<T>>,
    inner: &'grid Inner<T>,
}

impl<T: Eq + Hash + Debug> PartialEq for Pointer<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        *self.grid == *other.grid && self.inner == other.inner
    }
}

impl<T: Eq + Hash + Debug> Eq for Pointer<'_, T> {}

impl<T: Eq + Hash + Debug> Hash for Pointer<'_, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.grid().hash(state);
        self.inner.hash(state);
    }
}

impl<'grid, T: Eq + Hash + Debug> Pointer<'grid, T> {
    pub fn new(grid: Arc<super::Grid<T>>, inner: &'grid Inner<T>) -> Self {
        Self { grid, inner }
    }

    pub fn row(&self) -> usize {
        self.inner.row
    }
    pub fn col(&self) -> usize {
        self.inner.col
    }

    pub fn value(&self) -> &T {
        &self.inner.value
    }

    pub fn grid(&self) -> &super::Grid<T> {
        &self.grid
    }
}
