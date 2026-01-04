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

type Grid = Vec<Vec<char>>;

fn main() {
    let args = Args::parse();
    let data = match args.mode {
        Mode::Test => include_str!("../data/test"),
        Mode::Real => include_str!("../data/data"),
    };

    let grid: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    let total = match args.part {
        Part::One => part1(&grid),
        Part::Two => todo!(),
    };

    println!("Answer: {total}");
}

fn part1(grid: &Grid) -> usize {
    let mut total = 0;
    for (rv, row) in grid.iter().enumerate() {
        for (cv, _column) in row.iter().enumerate() {
            if grid[rv][cv] == '@' {
                let neighbors = search(grid, rv, cv);
                if neighbors < 4 {
                    total += 1;
                }
            }
        }
    }
    total
}

fn search(grid: &Grid, row: usize, column: usize) -> usize {
    let row_max = grid.len() as isize;
    let column_max = grid[0].len() as isize;
    let mut neighbors = 0;

    for i in -1..=1 {
        for j in -1..=1 {
            // skip over the grid point that was passed in
            if i == 0 && j == 0 {
                continue;
            }
            let search_row = row as isize + i;
            let search_column = column as isize + j;
            if search_row < row_max
                && search_row >= 0
                && search_column < column_max
                && search_column >= 0
            {
                if grid[search_row as usize][search_column as usize] == '@' {
                    neighbors += 1;
                }
            }
        }
    }
    neighbors
}
