use clap::{Parser, ValueEnum};

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    test: bool,

    #[arg(value_enum)]
    part: Part,
}

#[derive(Copy, Clone, PartialEq, PartialOrd, ValueEnum)]
enum Part {
    P1,
    P2,
}

fn main() {
    let args = Args::parse();
    let data = if args.test {
        include_str!("../data/test")
    } else {
        include_str!("../data/data")
    };

    match args.part {
        Part::P1 => solution::part1(data),
        Part::P2 => solution::part2(data),
    };
}

mod solution {
    use std::collections::HashSet;

    use itertools::Itertools;

    pub fn part1(data: &str) {
        let mut all_common_items: Vec<char> = Vec::new();
        for line in data.lines() {
            let midpoint = line.len() / 2;
            let (first, second) = line.split_at(midpoint);
            let first_set: HashSet<char> = first.chars().collect();
            let second_set: HashSet<char> = second.chars().collect();
            let common_items = first_set.intersection(&second_set);

            all_common_items.extend(common_items);
        }

        let results: u64 = all_common_items.iter().map(convert_to_int).sum();

        println!("Part 1 answer is: {results}");
    }

    pub fn part2(data: &str) {
        let mut all_badges: Vec<char> = Vec::new();
        for mut chunk in &data.lines().chunks(3) {
            let bag1: HashSet<char> = chunk.next().unwrap().chars().collect();
            let bag2: HashSet<char> = chunk.next().unwrap().chars().collect();
            let bag3: HashSet<char> = chunk.next().unwrap().chars().collect();
            let common_items1: HashSet<char> = bag1.intersection(&bag2).copied().collect();
            let common_items2: HashSet<char> = bag1.intersection(&bag3).copied().collect();
            all_badges.extend(common_items1.intersection(&common_items2));
        }
        let results: u64 = all_badges.iter().map(convert_to_int).sum();
        println!("Part 2 answer is: {results}");
    }

    fn convert_to_int(item: &char) -> u64 {
        if item.is_ascii_lowercase() {
            (*item as u64) - 96
        } else {
            (*item as u64) - 38
        }
    }
}
