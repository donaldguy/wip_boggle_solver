use std::fmt::Debug;
use std::hash::Hash;
use std::sync::Arc;

/// The value of a grid cell when evaluating it for matching within a sequence
pub trait Value: Eq + Hash {}
/// Anything we can check for equality and hash (and for good measure, print) should do
impl<T: Eq + Hash> Value for T {}

#[derive(PartialEq, Eq, Hash)]
pub(super) struct Inner<T: Value> {
    pub value: T,
    pub row: usize,
    pub col: usize,
}

/// A pointer to a cell within a grid. Can give underlying value, row/col info, and return (pointers to) adjacent cells.
pub struct Pointer<'grid, T: Value> {
    grid: Arc<super::Grid<T>>,
    inner: &'grid Inner<T>,
}

impl<T: Value + Debug> Debug for Pointer<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("grid::cell::Pointer")
            .field("row", &self.inner.row)
            .field("col", &self.inner.col)
            .field("value", &self.inner.value)
            .finish()
    }
}

impl<T: Value> Clone for Pointer<'_, T> {
    fn clone(&self) -> Self {
        Self {
            grid: self.grid.clone(),
            inner: self.inner,
        }
    }
}

impl<T: Value> PartialEq for Pointer<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        *self.grid == *other.grid && self.inner == other.inner
    }
}

impl<T: Value> Eq for Pointer<'_, T> {}

impl<T: Value> Hash for Pointer<'_, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.grid().hash(state);
        self.inner.hash(state);
    }
}

impl<T: Value> Pointer<'_, T> {
    pub fn row(&self) -> usize {
        self.inner.row
    }
    pub fn col(&self) -> usize {
        self.inner.col
    }

    pub fn value(&self) -> &T {
        &self.inner.value
    }
}

impl<'grid, T: Value> Pointer<'grid, T> {
    pub(super) fn new(grid: Arc<super::Grid<T>>, inner: &'grid Inner<T>) -> Self {
        Self { grid, inner }
    }

    pub fn grid(&self) -> &super::Grid<T> {
        &self.grid
    }
}
