mod solution;

use rangemap::RangeMap;
use std::fs;

fn main() {
    let data: String = fs::read_to_string("./data/data1").unwrap();
    // what_is_a_rangemap();

    solution::part1(&data);
    solution::part2(&data);
}

fn _what_is_a_rangemap() {
    let mut map = RangeMap::new();
    map.insert(0..5, 1);
    map.insert(8..15, 5);

    let v1 = map.get(&2).unwrap();
    let v2 = map.get(&10).unwrap();

    println!("1 - {}\n5 - {}", v1, v2);

    if map.get(&7).is_none() {
        println!("none returned right");
    }
}
