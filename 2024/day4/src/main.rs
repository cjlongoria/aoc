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

    pub fn part1(data: &str) -> usize {
        let word = [b'M', b'A', b'S'];
        let matrix: Vec<_> = data.lines().collect();

        let mut counter = 0;
        for (i, line) in matrix.iter().enumerate() {
            for (j, char) in line.bytes().enumerate() {
                if char == b'X' {
                    counter += scan(&matrix, &word, i as isize, j as isize);
                }
            }
        }
        counter
    }

    pub fn scan(matrix: &[&str], word: &[u8], i: isize, j: isize) -> usize {
        if word.is_empty() {
            return 1;
        };

        let mut result = 0;

        let max_i = matrix.len() as isize;
        let max_j = matrix[0].len() as isize;
        for next_i in (i - 1)..=(i + 1) {
            if next_i < 0 || next_i >= max_i {
                continue;
            }
            for next_j in (j - 1)..=(j + 1) {
                if next_j < 0 || next_j >= max_j {
                    continue;
                }
                let search_column = matrix[next_i as usize];
                let current_char = search_column.as_bytes()[next_j as usize];
                if current_char == word[0] {
                    let direction = (next_i - i, next_j - j);
                    result += sls(matrix, &word[1..], direction, next_i, next_j);
                }
            }
        }
        result
    }

    // Straight Line Search (sls)
    pub fn sls(
        matrix: &[&str],
        word: &[u8],
        direction: (isize, isize),
        i: isize,
        j: isize,
    ) -> usize {
        if word.is_empty() {
            return 1;
        };

        let mut result = 0;

        let max_i = matrix.len() as isize;
        let max_j = matrix[0].len() as isize;
        let next_i = i + direction.0;
        let next_j = j + direction.1;
        if next_i < 0 || next_i >= max_i {
            return 0;
        }
        if next_j < 0 || next_j >= max_j {
            return 0;
        }
        let current_char = matrix[next_i as usize].as_bytes()[next_j as usize];
        if current_char == word[0] {
            let direction = (next_i - i, next_j - j);
            result += sls(matrix, &word[1..], direction, next_i, next_j);
        }
        result
    }
}

pub mod common {}

pub mod part2 {

    pub fn part2(data: &str) -> usize {
        let matrix: Vec<_> = data.lines().collect();
        let max_i = matrix.len();
        let max_j = matrix[0].len();
        let mut counter = 0;

        // trim off the outer ring so we don't have to check bounds later
        for i in 1..(max_i - 1) {
            for j in 1..(max_j - 1) {
                if matrix[i].as_bytes()[j] == b'A' {
                    counter += check_corners(&matrix, (i, j));
                }
            }
        }
        counter
    }

    pub fn check_corners(data: &[&str], coord: (usize, usize)) -> usize {
        let corners: [(usize, usize); 4] = [
            (coord.0 - 1, coord.1 - 1),
            (coord.0 - 1, coord.1 + 1),
            (coord.0 + 1, coord.1 - 1),
            (coord.0 + 1, coord.1 + 1),
        ];

        let inverted_corners: [(usize, usize); 4] = [
            (coord.0 + 1, coord.1 + 1),
            (coord.0 + 1, coord.1 - 1),
            (coord.0 - 1, coord.1 + 1),
            (coord.0 - 1, coord.1 - 1),
        ];

        let corner_sets = std::iter::zip(corners, inverted_corners);

        let mut counter = 0;
        for corner_set in corner_sets {
            let orig = corner_set.0;
            let mirror = corner_set.1;
            if data[orig.0].as_bytes()[orig.1] == b'M'
                && data[mirror.0].as_bytes()[mirror.1] == b'S'
            {
                counter += 1;
            }
        }
        if counter == 2 {
            1
        } else {
            0
        }
    }
}
