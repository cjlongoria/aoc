use clap::Parser;
use clap::ValueEnum;

use std::collections::VecDeque;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    mode: Mode,
}

#[derive(ValueEnum, Clone)]
enum Mode {
    Test,
    Real,
}

#[derive(Debug)]
enum Instr {
    R(usize),
    L(usize),
}

impl FromStr for Instr {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let first_char = chars.next().unwrap();
        let num: usize = chars.as_str().parse()?;
        match first_char {
            'L' => Ok(Instr::L(num)),
            'R' => Ok(Instr::R(num)),
            _ => panic!("unexpected value to parse: {}", first_char),
        }
    }
}

impl Instr {
    fn get_inner(&self) -> usize {
        match self {
            Self::R(num) | Self::L(num) => *num,
        }
    }
}

struct Solution {
    buf: VecDeque<u8>,
    zero_counter: usize,
}

impl Solution {
    fn new() -> Self {
        let mut buf: VecDeque<u8> = (0..100).collect();
        buf.rotate_left(50);
        let zero_counter = 0;
        Self { buf, zero_counter }
    }

    fn spin(&mut self, instr: Instr) {
        let full_spin = instr.get_inner() / 100;
        let mut zero = full_spin;
        let pos = self.buf[0];

        match instr {
            Instr::L(num) => {
                self.buf.rotate_right(num % 100);
                if (self.buf[0] >= pos || self.buf[0] == 0) && pos != 0 {
                    zero += 1
                };
            }
            Instr::R(num) => {
                self.buf.rotate_left(num % 100);
                if self.buf[0] <= pos || self.buf[0] == 0 {
                    zero += 1
                };
            }
        }

        self.zero_counter += zero;
    }

    fn answer(&self) -> usize {
        self.zero_counter
    }
}

fn main() {
    let args = Args::parse();
    let data = match args.mode {
        Mode::Test => include_str!("../data/test"),
        Mode::Real => include_str!("../data/data"),
    };

    let mut solution = Solution::new();

    for line in data.lines() {
        let instr: Instr = line.parse().unwrap();
        solution.spin(instr);
    }
    println!("Answer: {}", solution.answer());
}
