use std::fs;

fn part1(contents: String) {
    let mut position = 0;
    let mut tree_counter = 0;
    for line in contents.lines() {
        position = position % line.len();
        if line.chars().nth(position).unwrap() == '#' {
            tree_counter += 1;
        }
        position += 3;
    } 
    println!("{}", tree_counter);
}

fn part2(contents: &String, left_shift: usize, down_shift: usize) -> usize {
    let mut position = 0;
    let mut tree_counter = 0;
    for line in contents.lines().step_by(down_shift) {
        position = position % line.len();
        if line.chars().nth(position).unwrap() == '#' {
            tree_counter += 1;
        }
        position += left_shift;
    } 
    println!("{}", tree_counter);
    tree_counter
}

fn main() {
    let contents: String = fs::read_to_string("data/data.txt")
        .expect("Should have been able to read the file");

    let val0 = part2(&contents, 1, 1);
    let val1 = part2(&contents, 3, 1);
    let val2 = part2(&contents, 5, 1);
    let val3 = part2(&contents, 7, 1);
    let val4 = part2(&contents, 1, 2);
    println!("final answer - {}", val0 * val1 * val2 * val3 * val4)
}
