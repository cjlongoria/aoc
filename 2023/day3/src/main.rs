mod solution;

use std::fs;

fn main() {
    let data: String = fs::read_to_string("./data/data1").unwrap().parse().unwrap();

    solution::part1(data.clone());
    solution::part2(data.clone());
}
