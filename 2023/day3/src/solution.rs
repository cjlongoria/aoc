use std::collections::HashSet;

pub fn part1(data: String) {
    let mut part_list: Vec<usize> = Vec::new();
    let data_matrix: Vec<&str> = data.lines().collect();
    for row in 0..data_matrix.len() {
        for column in 0..data_matrix[row].chars().collect::<Vec<char>>().len() {
            match data_matrix[row].chars().nth(column).unwrap() {
                '.' => continue,
                '0'..='9' => continue,
                _ => scan_neighbors(&data_matrix, row, column)
                    .iter()
                    .for_each(|x| {
                        part_list.push(*x);
                    }),
            }
        }
    }
    println!(
        "Answer for Day 3 part 1 - {}",
        part_list.iter().sum::<usize>()
    );
}

pub fn part2(data: String) {
    let mut part_list: Vec<usize> = Vec::new();
    let data_matrix: Vec<&str> = data.lines().collect();
    for row in 0..data_matrix.len() {
        for column in 0..data_matrix[row].chars().collect::<Vec<char>>().len() {
            if data_matrix[row].chars().nth(column).unwrap() == '*' {
                let neighbors = scan_neighbors(&data_matrix, row, column);
                if neighbors.len() == 2 {
                    part_list.push(neighbors.into_iter().reduce(|acc, x| acc * x).unwrap());
                }
            }
        }
    }
    println!(
        "Answer for Day 3 part 2 - {}",
        part_list.iter().sum::<usize>()
    );
}

fn scan_neighbors(matrix: &[&str], row: usize, column: usize) -> HashSet<usize> {
    // Using a HashSet here was a lucky assumption that might not work for everyone
    // The idea was that I didn't want to duplicate the same number multiple times
    // in my results. For example;
    // ..234..
    // ...*...
    // ..234..
    // This would only return a HashSet with a single value (234)
    //
    // If you wanted to get around this you could use a Vec and have some
    // sort of flag to toggle every time you came across a '.' to signify a new int
    let mut numbers: HashSet<usize> = HashSet::new();
    for i in row - 1..=row + 1 {
        for j in column - 1..=column + 1 {
            if matrix
                .get(i)
                .unwrap()
                .chars()
                .nth(j)
                .filter(|x| x.is_numeric())
                .is_some()
            {
                numbers.insert(find_num(matrix, i, j));
            }
        }
    }
    numbers
}

fn find_num(matrix: &[&str], row: usize, column: usize) -> usize {
    let mut int_buff: Vec<char> = Vec::new();
    let data_line = matrix.get(row).unwrap();
    let mut counter = 0;

    for c in data_line.chars() {
        if c.is_numeric() {
            int_buff.push(c);
        } else {
            if counter >= column {
                break;
            } else {
                int_buff.clear();
            }
        }
        counter += 1;
    }

    int_buff.iter().collect::<String>().parse().unwrap()
}

