use std::collections::HashMap;
use std::fs;

fn main() {
    let contents: String =
        fs::read_to_string("data/data.txt").expect("Should have been able to read the file");

