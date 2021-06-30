//! Generic (non-backtracking) search for sequences in adjacent cells of a
//! 2D grid.
//!
//! With `grid::Grid<char>` and the `grid::dictionary::SproutableTrie` from `dictionary_builder`
//! this is a boggle solver.
pub mod dictionary;
pub mod grid;
