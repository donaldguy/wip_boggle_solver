use grid_search::dictionary::SproutedTrie;
use std::usize;
use std::{collections::HashSet, fs, io::BufReader, path::PathBuf};
use structopt::StructOpt;

use grid_search::grid::GridCell;
use grid_search::grid::Grid;

#[derive(StructOpt, Debug)]
struct CLISolver {
    #[structopt(short, long, default_value = "4")]
    rows: usize,
    #[structopt(short, long, default_value = "4")]
    columns: usize,

    #[structopt(short = "i", long, default_value = "GNESSRIPETALTSEB")]
    grid: String,

    #[structopt(short, long, default_value = "../data/dictionary.bin")]
    dictionary: PathBuf,
}

#[derive(Clone)]
struct PartialSolution<'grid, 'dict> {
    grid: &'grid Grid<char>,
    cells_used: Vec<&'grid GridCell<char>>,
    remaining_cells: HashSet<&'grid GridCell<char>>,
    dict: &'grid grid_search::dictionary::SproutedTrie<'dict, char, bool>,
}

impl<'grid, 'dict> PartialSolution<'grid, 'dict> {
    fn new(grid: &'grid Grid<char>, dict: &'grid SproutedTrie<'dict, char, bool>) -> Self {
        Self {
            grid,
            cells_used: vec![],
            remaining_cells: grid.cells_set(),
            dict,
        }
    }

    fn iterate(&mut self) -> Vec<Self> {
        let last_used_cell = *self.cells_used.last().unwrap();
        let mut cells_to_try: HashSet<&GridCell<char>> = self
            .grid
            .get_adjacent_to(last_used_cell.row, last_used_cell.col)
            .into_iter()
            .collect();
        for cell in cells_to_try {
            if !self.remaining_cells.contains(cell) {
                cells_to_try.remove(&cell);
            }
        }

        let avaliable_seeds: HashSet<&char> = self.dict.avaliable_seeds().collect();

        let next_iteration: Vec<Self> =
            Vec::with_capacity(avaliable_seeds.len().min(cells_to_try.len()));

        for cell in cells_to_try.iter() {
            if avaliable_seeds.contains(&cell.value) {
                let mut next_gen = self.clone();
                next_gen.dict = next_gen.dict.sprout(&cell.value);
                next_gen
                    .cells_used
                    .push(next_gen.remaining_cells.take(cell).unwrap());
                next_iteration.push(next_gen);
            }
        }
        next_iteration
    }

    fn out_of_options(&self) -> bool {
        self.remaining_cells.is_empty() || self.dict.avaliable_seeds().count() == 0
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let this = CLISolver::from_args();

    let grid = grid_search::grid::Grid::new(this.rows, this.columns, "GNESSRIPETALTSEB".chars());
    let dict: grid_search::dictionary::SproutableTrie<char, bool> =
        bincode::deserialize_from(BufReader::new(fs::File::open(this.dictionary)?))?;
    let sproduted_dict = grid_search::dictionary::SproutedTrie::new(&dict);

    let ps = PartialSolution::new(&grid, &dict);

    // let mut avaliable_cells = grid.cells_set();
    // let mut words:  = Vec::new();

    // let letter_trie = dict.get_node(&[cell.value]).unwrap();
    // avaliable_cells.remove(cell);
    // let adjacent_cells = grid.get_adjacent_to(cell.row, cell.col);
    // for ac in adjacent_cells

    Ok(())
}
