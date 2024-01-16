mod solution;

use std::fs;

fn main() {
    let data: String = fs::read_to_string("./data/data").unwrap();

    solution::solve(&data, false);
    solution::solve(&data, true);
}
