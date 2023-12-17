use std::fs;
mod gameconfig;
mod part1;
mod part2;

fn main() {
    let data: String = fs::read_to_string("./data/data1").unwrap().parse().unwrap();

    part1::solution(data.clone());
    part2::solution(data);
}

