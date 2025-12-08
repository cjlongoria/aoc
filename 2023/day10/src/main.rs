mod solution;

use std::fs;

fn main() {
    let data: String = fs::read_to_string("./data/data_test").unwrap();

    solution::solve(&data);
}
