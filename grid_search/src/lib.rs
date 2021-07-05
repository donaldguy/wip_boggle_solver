//! Generic (non-backtracking) search for sequences in adjacent cells of a
//! 2D grid.
//!
//! With `grid::Grid<char>` and the `grid::dictionary::SproutableTrie` from `dictionary_builder`
//! this is a boggle solver.

use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

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

fn find_at_cell<T: grid::cell::Value>(
    cell: grid::CellPointer<T>,
    trie: &mut dictionary::SproutedTrie<T, bool>,
    unused_cells: HashSet<grid::CellPointer<T>>,
) {
    let seeds: Vec<&T> = trie.avaliable_seeds().collect();

    if seeds.is_empty() || unused_cells.is_empty() {
        return;
    }

    let adjacent_set = cell.get_adjacent_cell_in(&unused_cells);

    let mut adjacent_map: HashMap<&T, &grid::CellPointer<T>> =
        HashMap::with_capacity(adjacent_set.len());
    for adjacent in adjacent_set.iter() {
        adjacent_map.insert(adjacent.value(), adjacent);
    }

    for seed in seeds {
        if adjacent_map.contains_key(seed) {
            let mut unused_cells = unused_cells.clone();
            unused_cells.remove(&cell);
            let cell = adjacent_map[seed];

            find_at_cell(
                cell.grid()
                    .get(cell.row() as isize, cell.col() as isize)
                    .unwrap(),
                trie.sprout(seed),
                unused_cells,
            )
        }
    }
}
