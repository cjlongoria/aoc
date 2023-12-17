use std::fs;
mod part1;

fn main() {
    let data: String = fs::read_to_string("./data/data1").unwrap().parse().unwrap();

    part1::solution(data);
}

