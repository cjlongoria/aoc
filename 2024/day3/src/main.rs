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
    use crate::common::basic_regex;

    pub fn part1(data: &str) -> usize {
        basic_regex(data)
    }
}

pub mod common {
    use regex::Regex;
    pub fn basic_regex(data: &str) -> usize {
        Regex::new(r"mul\(([0-9]{1,3},[0-9]{1,3})\)")
            .unwrap()
            .captures_iter(data)
            .map(|c| {
                let (_, [temp]) = c.extract();
                temp
            })
            .fold(0, |acc, x| {
                let (left, right) = x.split_once(",").unwrap();
                acc + (left.parse::<usize>().unwrap() * right.parse::<usize>().unwrap())
            })
    }
}

pub mod part2 {
    use crate::common::basic_regex;
    use regex::Regex;

    pub fn part2(data: &str) -> usize {
        // multiple do() or don't() can happen in a row
        // start with implicit do()
        // doesn't have to end with don't()
        //
        // 1) split off until don't() - count beginning muls
        // 2) split off last do() - count end muls
        // 3) regex match lines between do() - don't()
        // 4) iter over each line - count muls
        // 5) return sum of 1) 2) 4)

        let (beginning, mid) = data.split_once("don't()").unwrap();
        let (mid, end) = mid.rsplit_once("do()").unwrap();
        let beginning_val = basic_regex(beginning);
        let end_val = basic_regex(end);

        //The new line char doesn't match against the '.' without the '(?s)' flag
        Regex::new(r"do\(\)(?s)(.*?)don't\(\)")
            .unwrap()
            .captures_iter(mid)
            .map(|c| {
                let (_, [temp]) = c.extract();
                temp
            })
            .fold(0, |acc, x| acc + basic_regex(x))
            + beginning_val
            + end_val
    }
}
