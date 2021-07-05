//! Generic (non-backtracking) search for sequences in adjacent cells of a
//! 2D grid.
//!
//! With `grid::Grid<char>` and the `grid::dictionary::SproutableTrie` from `dictionary_builder`
//! this is a boggle solver.

use std::{collections::HashSet, iter::FromIterator};

use dictionary::SproutedTrie;
pub mod dictionary;
pub mod grid;

// type Result<'a, T> = Option<Vec<Vec<&'a grid::CellPointer<'a, T>>>>;

pub fn find_sequences_from_dict_in_grid<'a, T: grid::cell::Value>(
    dict: &'a dictionary::SproutableTrie<T, bool>,
    grid: &'a grid::Grid<T>,
) -> Vec<(Vec<&'a T>, &'a bool)> {
    let mut trie = SproutedTrie::new(dict);
    for cell in grid {
        let cell_seed = trie
            .avaliable_seeds()
            .find(|s| *s == cell.value())
            .expect("Initial cell value not in seeds");

        find_at_cell(
            cell,
            trie.sprout(cell_seed),
            HashSet::from_iter(grid.into_iter()),
        );
    }

    trie.flatten()
}

fn find_at_cell<'a, T: grid::cell::Value>(
    cell: grid::CellPointer<'a, T>,
    trie: &mut dictionary::SproutedTrie<T, bool>,
    mut unused_cells: HashSet<grid::CellPointer<'a, T>>,
) {
    let seeds: HashSet<&T> = trie.avaliable_seeds().collect();

    if seeds.is_empty() || unused_cells.is_empty() {
        return;
    }
    unused_cells.remove(&cell);

    let adjacent_cells = cell.get_adjacent_cell_in(&unused_cells);

    for next_cell in adjacent_cells {
        let next_cell_value = next_cell.value();
        if seeds.contains(next_cell_value) {
            let trie = trie.sprout(seeds.get(next_cell.value()).unwrap());

            find_at_cell(next_cell, trie, unused_cells.clone());
        }
    }
}
