use crate::part1::part1;
use crate::part2::part2;
use clap::{Parser, ValueEnum};

#[derive(Parser)]
struct Args {
    #[arg(value_enum)]
    part: Part,

    #[arg(short, long)]
    test: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Part {
    P1,
    P2,
}

fn main() {
    let args = Args::parse();
    let data = match args.test {
        true => include_str!("../data/test"),
        false => include_str!("../data/data"),
    };

    let res = match args.part {
        Part::P1 => part1(data),
        Part::P2 => part2(data),
    };

    println!("ans: {}", res);
}

pub mod part1 {
    use crate::common::parse_data;

    pub fn part1(data: &str) -> usize {
        let (rule_map, updates) = parse_data(data);
        0
    }
}

pub mod common {
    use std::collections::HashMap;

    pub fn parse_data(
        data: &str,
    ) -> (
        HashMap<usize, Vec<usize>>,
        impl Iterator<Item = impl Iterator<Item = usize> + use<'_>>,
    ) {
        let (rules, updates) = split_data(data);
        let rule_map = gen_rule_map(rules);
        (rule_map, updates)
    }

    fn split_data(
        data: &str,
    ) -> (
        impl Iterator<Item = &str>,
        impl Iterator<Item = impl Iterator<Item = usize> + use<'_>>,
    ) {
        data.split_once("\n\n")
            .map(|(rules, updates)| {
                (
                    rules.lines(),
                    updates.lines().map(|line| {
                        line.split(",")
                            .map(|update| update.parse::<usize>().unwrap())
                    }),
                )
            })
            .unwrap()
    }

    fn gen_rule_map<'a>(rules: impl Iterator<Item = &'a str>) -> HashMap<usize, Vec<usize>> {
        let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
        for rule in rules {
            let (key, value): (usize, usize) = rule
                .split_once("|")
                .map(|(key, value)| (key.parse().unwrap(), value.parse().unwrap()))
                .unwrap();

            let values = map.entry(key).or_default();
            values.push(value);
        }
        map
    }
}

pub mod part2 {

    pub fn part2(data: &str) -> usize {
        0
    }
}
