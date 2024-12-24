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
    use crate::common::check;
    use regex::Regex;

    pub fn part1(data: &str) -> usize {
        let re = Regex::new(r"mul\(([0-9]{1,3},[0-9]{1,3})\)").unwrap();
        let caps: Vec<&str> = re
            .captures_iter(data)
            .map(|c| {
                let (_, [temp]) = c.extract();
                temp
            })
            .collect();

        caps.iter().fold(0, |acc, x| {
            let (left, right) = x.split_once(",").unwrap();
            acc + (left.parse::<usize>().unwrap() * right.parse::<usize>().unwrap())
        })
    }
}

pub mod common {
    pub fn check(data: &[u32]) -> bool {
        if !data.is_sorted() && !data.iter().rev().is_sorted() {
            return false;
        }

        let mut prev: Option<&u32> = None;
        for level in data {
            if let Some(prev_level) = prev {
                if !(1..=3).contains(&prev_level.abs_diff(*level)) {
                    return false;
                }
            }
            prev = Some(level)
        }
        true
    }
}

pub mod part2 {
    use crate::common::check;

    // This is a pretty lame brute force approach that short circuits on the first success.
    fn problem_dampener(data: &[u32]) -> bool {
        let end = data.len();

        for i in 0..end {
            let left = &data[..i];
            let right = &data[i + 1..];
            let new = [left, right].concat();
            if check(&new) {
                return true;
            }
        }
        false
    }

    pub fn part2(data: &str) -> usize {
        let mut count = 0;
        for report in data.lines() {
            let report_data: Vec<_> = report
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect();

            if check(&report_data) {
                count += 1;
            } else if problem_dampener(&report_data) {
                count += 1;
            }
        }
        count
    }
}
