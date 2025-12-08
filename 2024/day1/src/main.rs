use clap::{Parser, ValueEnum};
use std::{collections::HashMap, iter::zip};

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

    let data = parse_data(data);

    let res = match args.part {
        Part::P1 => part1(data),
        Part::P2 => part2(data),
    };

    println!("ans: {}", res);
}

fn parse_data(data: &str) -> (Vec<isize>, Vec<isize>) {
    data.lines()
        .map(|l| {
            let (x, y) = l.split_once("   ").unwrap();
            let x_num: isize = x.parse().unwrap();
            let y_num: isize = y.parse().unwrap();
            (x_num, y_num)
        })
        .unzip()
}

fn part1(data: (Vec<isize>, Vec<isize>)) -> usize {
    let mut left = data.0;
    let mut right = data.1;

    left.sort();
    right.sort();

    zip(left, right).map(|(x, y)| x.abs_diff(y)).sum()
}

fn part2(data: (Vec<isize>, Vec<isize>)) -> usize {
    let left = data.0;
    let right = data.1;

    let mut hm: HashMap<isize, isize> = HashMap::new();

    for num in right {
        hm.entry(num)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    left.into_iter().fold(0, |acc, x| {
        let val = hm.get(&x).unwrap_or(&0);
        acc + (x * val)
    }) as usize
}
