use std::usize;
use std::{fs, io::BufReader, path::PathBuf};
use structopt::StructOpt;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let this = CLISolver::from_args();

    let grid = grid_search::grid::Grid::new(this.rows, this.columns, "GNESSRIPETALTSEB".chars());
    let dict: grid_search::dictionary::SproutableTrie<char, bool> =
        bincode::deserialize_from(BufReader::new(fs::File::open(this.dictionary)?))?;

    let words = grid_search::find_sequences_from_dict_in_grid(&dict, &grid);

    let mut i = 0;
    for (word, _) in words {
        for char in word {
            i += 1;
            print!("{}", char);
        }
        print!("\n");
    }
    println!("{}", i);

    // let mut avaliable_cells = grid.cells_set();
    // let mut words:  = Vec::new();

    // let letter_trie = dict.get_node(&[cell.value]).unwrap();
    // avaliable_cells.remove(cell);
    // let adjacent_cells = grid.get_adjacent_to(cell.row, cell.col);
    // for ac in adjacent_cells

    Ok(())
}
