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

#[derive(ValueEnum, Clone, PartialEq)]
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

    let mut grid: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    let mut total = 0;
    let removed_count = grid_scan(&mut grid, args.part);

    // TODO: if part one total = removed count. if part two, then scan again if removed_count > 0

    // Debugging final grid
    for row in grid {
        println!("{:?}", row);
    }

    println!("Answer: {total}");
}

// TODO: Need to do multiple passes. If a grid point gets removed there isn't a way to check if
// that would then remove previous grid spaces revisited. Probably just iterate over the grid until
// it returns 0.
fn grid_scan(grid: &mut Grid, part: Part) -> usize {
    let mut total = 0;
    for rv in 0..grid.len() {
        for cv in 0..grid[rv].len() {
            if grid[rv][cv] == '@' {
                let mutate = part == Part::Two;
                let neighbors = search(grid, rv, cv);
                if neighbors < 4 {
                    total += 1;
                    if mutate {
                        grid[rv][cv] = 'x';
                    }
                }
            }
        }
    }
    total
}

fn search(grid: &mut Grid, row: usize, column: usize) -> usize {
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
