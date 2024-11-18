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
    use std::collections::{HashMap, VecDeque};

    pub fn part1(data: &str) {
        let mut temp: Vec<VecDeque<char>> = Vec::new();
        for line in data.lines() {
            for (i, ch) in line.chars().enumerate() {
                match temp.get_mut(i) {
                    Some(queue) => queue.push_front(ch),
                    None => temp.push(VecDeque::from([ch])),
                }
            }
            if line.is_empty() {
                break;
            }
        }
        let stacks: HashMap<char, VecDeque<char>> = temp
            .into_iter()
            .filter(|x| x[0].is_numeric())
            .map(|data| {
                data.into_iter()
                    .filter(|x| x.is_ascii_alphanumeric())
                    .collect()
            })
            .map(|mut data: VecDeque<char>| {
                let values = data.split_off(1);
                (data[0], values)
            })
            .collect();
        println!("{stacks:?}");
    }
    pub fn part2(data: &str) {}
}
