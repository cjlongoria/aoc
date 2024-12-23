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
        //TODO: Change back to `part2()`
        Part::P2 => part1(data),
    };

    println!("ans: {}", res);
}

fn part1(data: &str) -> usize {
    let mut count = 0;
    for report in data.lines() {
        let report_data: Vec<_> = report
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        let mut prev: Option<u32> = None;
        let mut safe = true;

        if !report_data.is_sorted() && !report_data.iter().rev().is_sorted() {
            continue;
        }

        for level in report_data {
            if let Some(prev_level) = prev {
                if !(1..=3).contains(&prev_level.abs_diff(level)) {
                    safe = false;
                    break;
                }
            }
            prev = Some(level)
        }

        if safe {
            count += 1;
        }
    }
    count
}

// fn part2(data: (Vec<isize>, Vec<isize>)) -> usize {}
