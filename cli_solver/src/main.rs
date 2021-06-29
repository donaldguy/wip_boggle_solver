use solver::dictionary::SproutedTrie;
use std::usize;
use std::{collections::HashSet, fs, io::BufReader, path::PathBuf};
use structopt::StructOpt;

use solver::grid::Cube;
use solver::grid::Grid;

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
    cubes_used: Vec<&'grid Cube<char>>,
    remaining_cubes: HashSet<&'grid Cube<char>>,
    dict: &'grid solver::dictionary::SproutedTrie<'dict, char, bool>,
}

impl<'grid, 'dict> PartialSolution<'grid, 'dict> {
    fn new(grid: &'grid Grid<char>, dict: &'grid SproutedTrie<'dict, char, bool>) -> Self {
        Self {
            grid,
            cubes_used: vec![],
            remaining_cubes: grid.cubes_set(),
            dict,
        }
    }

    fn iterate(&mut self) -> Vec<Self> {
        let last_used_cube = *self.cubes_used.last().unwrap();
        let mut cubes_to_try: HashSet<&Cube<char>> = self
            .grid
            .get_adjacent_to(last_used_cube.row, last_used_cube.col)
            .into_iter()
            .collect();
        for cube in cubes_to_try {
            if !self.remaining_cubes.contains(cube) {
                cubes_to_try.remove(&cube);
            }
        }

        let avaliable_seeds: HashSet<&char> = self.dict.avaliable_seeds().collect();

        let next_iteration: Vec<Self> =
            Vec::with_capacity(avaliable_seeds.len().min(cubes_to_try.len()));

        for cube in cubes_to_try.iter() {
            if avaliable_seeds.contains(&cube.value) {
                let mut next_gen = self.clone();
                next_gen.dict = next_gen.dict.sprout(&cube.value);
                next_gen
                    .cubes_used
                    .push(next_gen.remaining_cubes.take(cube).unwrap());
                next_iteration.push(next_gen);
            }
        }
        next_iteration
    }

    fn out_of_options(&self) -> bool {
        self.remaining_cubes.is_empty() || self.dict.avaliable_seeds().count() == 0
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let this = CLISolver::from_args();

    let grid = solver::grid::Grid::new(this.rows, this.columns, "GNESSRIPETALTSEB".chars());
    let dict: solver::dictionary::SproutableTrie<char, bool> =
        bincode::deserialize_from(BufReader::new(fs::File::open(this.dictionary)?))?;
    let sproduted_dict = solver::dictionary::SproutedTrie::new(&dict);

    let ps = PartialSolution::new(&grid, &dict);

    // let mut avaliable_cubes = grid.cubes_set();
    // let mut words:  = Vec::new();

    // let letter_trie = dict.get_node(&[cube.value]).unwrap();
    // avaliable_cubes.remove(cube);
    // let adjacent_cubes = grid.get_adjacent_to(cube.row, cube.col);
    // for ac in adjacent_cubes

    Ok(())
}
