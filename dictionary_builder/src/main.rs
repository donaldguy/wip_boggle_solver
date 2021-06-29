use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "boggle_dictionary_builder",
    about = "CLI to take in a wordlist, precompute tries, and output a serialized form that can be loaded by server"
)]
struct DictionaryBuilder {
    #[structopt(
        short,
        long,
        help = "The world list to build from",
        default_value = "../data/CSW19.txt",
        parse(from_os_str)
    )]
    input: PathBuf,

    #[structopt(
        short,
        long,
        help = "The binary serialized trie from the dictionary",
        default_value = "../data/dictionary.bin",
        parse(from_os_str)
    )]
    output: PathBuf,
}

type CharBoolTrie = grid_search::dictionary::SproutableTrie<char, bool>;

impl DictionaryBuilder {
    fn get_word_list(&self) -> std::io::Result<impl IntoIterator<Item = String>> {
        let f = std::fs::File::open(self.input.to_owned())?;
        let lines = BufReader::new(f).lines();
        let mut results: Vec<String> = Vec::with_capacity(lines.size_hint().0);

        for line in lines {
            let word = line?.to_uppercase();
            if word.len() < 3 || word.get(0..1).unwrap() == "Q" && word.get(1..2).unwrap() != "U" {
                continue;
            }

            results.push(word);
        }
        results.sort();
        Ok(results)
    }

    fn save(&self, dict: CharBoolTrie) -> std::io::Result<()> {
        let b: Vec<u8> = match bincode::serialize(&dict) {
            Ok(bytes) => bytes,
            Err(e) => {
                panic!("{:#?}", e)
            }
        };

        std::fs::write(&self.output, b)
    }
}

fn main() -> std::io::Result<()> {
    let this = DictionaryBuilder::from_args();

    let word_list = this.get_word_list()?.into_iter();
    let word_count = word_list.size_hint().1.unwrap();

    let mut trie = CharBoolTrie::new();
    let mut i = 1;
    for word in word_list {
        println!("Processing word {} of {}", i, word_count);
        trie.insert(word.chars(), true);
        i += 1;
    }

    this.save(trie)
}
