use std::fs;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Dir {
    TopLeft,
    Top,
    TopRight,
    Left,
    Right,
    BotLeft,
    Bot,
    BotRight,
}

enum SeatStatus {
    Empty,
    Occupied,
    None,
}

fn main() {
    let contents: String =
        fs::read_to_string("data/data.txt").expect("Should have been able to read the file");

    let mut data_vec: Vec<Vec<char>> = contents.lines().map(|x| x.chars().collect()).collect();
    let mut tmp_state: Vec<Vec<char>> = vec![vec!['.'; data_vec[0].len()]; data_vec.len()];
    let mut state_change = true;
    while state_change {
        state_change = false;
        for i in 0..data_vec.len() {
            for j in 0..data_vec[0].len() {
                let vals = empty_occupied(&data_vec, i, j);
                match data_vec[i][j] {
                    'L' => {
                        if vals.1 == 0 {
                            state_change = true;
                            tmp_state[i][j] = '#'
                        }
                    }
                    '#' => {
                        if vals.1 >= 5 {
                            state_change = true;
                            tmp_state[i][j] = 'L'
                        }
                    }
                    _ => continue,
                }
            }
        }

        data_vec = tmp_state.clone();
    }
    let mut seat_counter = 0;
    for i in 0..data_vec.len() {
        for j in 0..data_vec[0].len() {
            if let '#' = data_vec[i][j] {
                seat_counter += 1;
            }
        }
    }
    println!("{seat_counter}");
}

fn empty_occupied(data: &Vec<Vec<char>>, row_index: usize, col_index: usize) -> (usize, usize) {
    let mut empty: usize = 0;
    let mut occupied: usize = 0;

    let mut dirs: Vec<Dir> = vec![
        Dir::TopLeft,
        Dir::Top,
        Dir::TopRight,
        Dir::Left,
        Dir::Right,
        Dir::BotLeft,
        Dir::Bot,
        Dir::BotRight,
    ];
    let mut dir_pointer: usize = 0;

    let min_row: usize = if col_index == 0 {
        dirs.retain(|dir| ![Dir::TopLeft, Dir::Left, Dir::BotLeft].contains(dir));
        0
    } else {
        col_index - 1
    };
    let max_row: usize = if col_index == data[0].len() - 1 {
        dirs.retain(|dir| ![Dir::TopRight, Dir::Right, Dir::BotRight].contains(dir));
        data[0].len()
    } else {
        col_index + 2
    };
    let min_col: usize = if row_index == 0 {
        dirs.retain(|dir| ![Dir::TopLeft, Dir::Top, Dir::TopRight].contains(dir));
        0
    } else {
        row_index - 1
    };
    let max_col: usize = if row_index == data.len() - 1 {
        dirs.retain(|dir| ![Dir::BotLeft, Dir::Bot, Dir::BotRight].contains(dir));
        data.len()
    } else {
        row_index + 2
    };

    for i in min_col..max_col {
        for j in min_row..max_row {
            if i == row_index && j == col_index {
                continue;
            }
            match data[i][j] {
                '.' => match keep_going(data, i as isize, j as isize, &dirs[dir_pointer]) {
                    SeatStatus::Empty => empty += 1,
                    SeatStatus::Occupied => occupied += 1,
                    SeatStatus::None => (),
                },
                'L' => empty += 1,
                '#' => occupied += 1,
                _ => panic!(),
            };
            dir_pointer += 1
        }
    }
    (empty, occupied)
}

fn keep_going(data: &Vec<Vec<char>>, i: isize, j: isize, dir: &Dir) -> SeatStatus {
    let (next_col, next_row) = match dir {
        Dir::TopLeft => (i - 1, j - 1),
        Dir::Top => (i - 1, j),
        Dir::TopRight => (i - 1, j + 1),
        Dir::Left => (i, j - 1),
        Dir::Right => (i, j + 1),
        Dir::BotLeft => (i + 1, j - 1),
        Dir::Bot => (i + 1, j),
        Dir::BotRight => (i + 1, j + 1),
    };

    if (next_col < 0 || next_col >= data.len().try_into().unwrap())
        || (next_row < 0 || next_row >= data[0].len().try_into().unwrap())
    {
        return SeatStatus::None;
    };

    match data[next_col as usize][next_row as usize] {
        '.' => keep_going(data, next_col, next_row, dir),
        'L' => SeatStatus::Empty,
        '#' => SeatStatus::Occupied,
        _ => panic!(),
    }
}
