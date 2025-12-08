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
    use std::ops::RangeInclusive;

    pub fn part1(data: &str) {
        let mut counter = 0;
        let range_tuples = range_tuples(data);

        for tuple in range_tuples {
            if tuple.0.contains(tuple.1.start()) && tuple.0.contains(tuple.1.end()) {
                counter += 1;
            } else if tuple.1.contains(tuple.0.start()) && tuple.1.contains(tuple.0.end()) {
                counter += 1;
            }
        }
        println!("Part 1 answer = {counter}");
    }
    pub fn part2(data: &str) {
        let mut counter = 0;
        let range_tuples = range_tuples(data);

        for tuple in range_tuples {
            if tuple.0.contains(tuple.1.start()) {
                counter += 1;
            } else if tuple.1.contains(tuple.0.start()) {
                counter += 1;
            }
        }
        println!("Part 2 answer = {counter}");
    }

    fn range_tuples(data: &str) -> Vec<(RangeInclusive<usize>, RangeInclusive<usize>)> {
        data.lines()
            .map(|line| line.split_once(",").unwrap())
            .map(convert_tuple_to_range)
            .collect()
    }

    fn convert_tuple_to_range(
        data: (&str, &str),
    ) -> (RangeInclusive<usize>, RangeInclusive<usize>) {
        let (r1, r2) = data;
        let first_range = r1
            .split_once("-")
            .map(|(lower, upper)| lower.parse().unwrap()..=upper.parse().unwrap())
            .unwrap();

        let second_range = r2
            .split_once("-")
            .map(|(lower, upper)| lower.parse().unwrap()..=upper.parse().unwrap())
            .unwrap();

        (first_range, second_range)
    }
}
