var searchIndex = JSON.parse('{\
"cli_solver":{"doc":"","i":[[3,"CLISolver","cli_solver","",null,null],[12,"rows","","",0,null],[12,"columns","","",0,null],[12,"grid","","",0,null],[12,"dictionary","","",0,null],[5,"main","","",null,[[],[["result",4],["box",3]]]],[11,"from","","",0,[[]]],[11,"into","","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"fmt","","",0,[[["formatter",3]],["result",6]]],[11,"clap","","",0,[[],["app",3]]],[11,"from_clap","","",0,[[["argmatches",3]]]],[11,"augment_clap","","",0,[[["app",3]],["app",3]]],[11,"is_subcommand","","",0,[[],["bool",15]]]],"p":[[3,"CLISolver"]]},\
"dictionary_builder":{"doc":"","i":[[3,"DictionaryBuilder","dictionary_builder","",null,null],[12,"input","","",0,null],[12,"output","","",0,null],[6,"CharBoolTrie","","",null,null],[11,"get_word_list","","",0,[[],["result",6]]],[11,"save","","",0,[[["bool",15],["char",15],["sproutabletrie",3]],["result",6]]],[5,"main","","",null,[[],["result",6]]],[11,"from","","",0,[[]]],[11,"into","","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"fmt","","",0,[[["formatter",3]],["result",6]]],[11,"clap","","",0,[[],["app",3]]],[11,"from_clap","","",0,[[["argmatches",3]]]],[11,"augment_clap","","",0,[[["app",3]],["app",3]]],[11,"is_subcommand","","",0,[[],["bool",15]]]],"p":[[3,"DictionaryBuilder"]]},\
"grid_search":{"doc":"Generic (non-backtracking) search for sequences in …","i":[[0,"dictionary","grid_search","",null,null],[3,"SproutableTrie","grid_search::dictionary","crates.io contains a number of tries and various text …",null,null],[11,"new","","",0,[[]]],[11,"insert","","",0,[[]]],[3,"SproutedTrie","","",null,null],[11,"new","","",1,[[["sproutabletrie",3]]]],[11,"avaliable_seeds","","",1,[[],[["sproutabletrie",3],["keys",3]]]],[11,"sprout","","",1,[[]]],[11,"flatten","","",1,[[],["vec",3]]],[0,"grid","grid_search","",null,null],[0,"cell","grid_search::grid","",null,null],[8,"Value","grid_search::grid::cell","The value of a grid cell when evaluating it for matching …",null,null],[3,"Pointer","","A pointer to a cell within a grid. Can give underlying …",null,null],[11,"row","","",2,[[],["usize",15]]],[11,"col","","",2,[[],["usize",15]]],[11,"value","","",2,[[]]],[11,"grid","","",2,[[],["grid",3]]],[3,"Grid","grid_search::grid","",null,null],[11,"get","","get a cell by canonical zero-indexed row and column.",3,[[["isize",15]],[["option",4],["cellpointer",3]]]],[11,"get_adjacent_cells","grid_search::grid::cell","",2,[[],["hashset",3]]],[11,"get_adjacent_cell_in","","",2,[[["hashset",3]],["hashset",3]]],[5,"find_sequences_from_dict_in_grid","grid_search","",null,[[["sproutabletrie",3],["grid",3]],["vec",3]]],[11,"from","grid_search::dictionary","",0,[[]]],[11,"into","","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"from","","",1,[[]]],[11,"into","","",1,[[]]],[11,"to_owned","","",1,[[]]],[11,"clone_into","","",1,[[]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"from","grid_search::grid::cell","",2,[[]]],[11,"into","","",2,[[]]],[11,"to_owned","","",2,[[]]],[11,"clone_into","","",2,[[]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"from","grid_search::grid","",3,[[]]],[11,"into","","",3,[[]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"clone","grid_search::dictionary","",1,[[],["sproutedtrie",3]]],[11,"clone","grid_search::grid::cell","",2,[[]]],[11,"eq","","",2,[[],["bool",15]]],[11,"eq","grid_search::grid","",3,[[],["bool",15]]],[11,"fmt","grid_search::dictionary","",0,[[["formatter",3]],["result",6]]],[11,"hash","grid_search::grid::cell","",2,[[]]],[11,"hash","grid_search::grid","",3,[[]]],[11,"serialize","grid_search::dictionary","",0,[[],["result",4]]],[11,"deserialize","","",0,[[],["result",4]]],[11,"new","grid_search::grid","",3,[[["usize",15]],["arc",3]]],[11,"rows","","",3,[[],["usize",15]]],[11,"columns","","",3,[[],["usize",15]]]],"p":[[3,"SproutableTrie"],[3,"SproutedTrie"],[3,"Pointer"],[3,"Grid"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);