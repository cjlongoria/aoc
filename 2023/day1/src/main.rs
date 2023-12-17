use std::fs;
mod part1;
mod part2;

fn main() {
    let data: String = fs::read_to_string("./src/data/data1")
        .unwrap()
        .parse()
        .unwrap();

    let data2: String = fs::read_to_string("./src/data/data2")
        .unwrap()
        .parse()
        .unwrap();

    part1::solution(data);
    part2::solution(data2);
}

