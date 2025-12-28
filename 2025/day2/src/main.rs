use clap::Parser;
use clap::ValueEnum;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    mode: Mode,

    #[arg(short, long)]
    part: Part,
}

#[derive(ValueEnum, Clone)]
enum Mode {
    Test,
    Real,
}

#[derive(ValueEnum, Clone)]
enum Part {
    #[value(name = "1")]
    One,
    #[value(name = "2")]
    Two,
}

fn main() {
    let args = Args::parse();
    let data = match args.mode {
        Mode::Test => include_str!("../data/test"),
        Mode::Real => include_str!("../data/data"),
    };

    let res = data.split(",").fold(0, |acc, range| {
        let (start, end) = range.split_once("-").unwrap();
        let start: usize = start.parse().unwrap();
        let end: usize = end.parse().unwrap();
        acc + (end - start)
    });
    println!("{res}");

    let ranges = data.split(",").map(|r| r.split_once("-").unwrap());
    let mut total: usize = 0;

    for (lower, upper) in ranges {
        let lower_int: usize = lower.parse().unwrap();
        let upper_int: usize = upper.parse().unwrap();
        for id_int in lower_int..=upper_int {
            let id = id_int.to_string();
            match args.part {
                Part::One => part1(&id, &mut total),
                Part::Two => part2(&id, &mut total),
            }
        }
    }
    println!("{total}");
}

fn part1(id: &str, total: &mut usize) {
    // Skip ids that cannot be split evenly.
    if id.len() % 2 != 0 {
        return;
    }
    let halfway_point = id.len() / 2;
    if id[..halfway_point] == id[halfway_point..] {
        *total += id.parse::<usize>().unwrap();
    }
}

fn part2(id: &str, total: &mut usize) {
    let len = id.len() / 2;
    for i in 0..=len {
        let combos = id.split(&id[0..i]).all(|x| x.is_empty());
        if combos {
            *total += id.parse::<usize>().unwrap();
            return;
        }
    }
}
