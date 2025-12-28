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
    #[value(alias = "1")]
    Test,
    #[value(alias = "2")]
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

    let mut total: usize = 0;

    for line in data.lines() {
        let joltage = match args.part {
            Part::One => find_joltage(line, 1),
            Part::Two => find_joltage(line, 11),
        };
        total += joltage.parse::<usize>().unwrap();
    }

    println!("Answer: {total}");
}

fn find_joltage(data: &str, num: usize) -> String {
    let first_digit = data[..data.len() - num].chars().max().unwrap();
    if num == 0 {
        format!("{first_digit}")
    } else {
        let index = data.find(first_digit).unwrap();
        let rest = find_joltage(&data[index + 1..], num - 1);
        format!("{first_digit}{rest}")
    }
}
