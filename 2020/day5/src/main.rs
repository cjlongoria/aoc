use std::fs;
use std::collections::HashSet;


fn main() {
    let contents: String = fs::read_to_string("data/data.txt")
        .expect("Should have been able to read the file");

    println!("{}", part1(&contents));

}

fn part1(input: &str) -> i32 {
    let hs: HashSet<i32> = input
    .lines()
    .map(|line| { 
        let binary_string = line
            .replace("F","0")
            .replace("B","1")
            .replace("L","0")
            .replace("R","1");

        let row_binary = i32::from_str_radix(&binary_string[..7], 2).unwrap();
        let seat_binary = i32::from_str_radix(&binary_string[7..], 2).unwrap();
        row_binary * 8 + seat_binary
    })
    // part1 didn't use the hashset and just returned after unwrap
    // .max()
    // .unwrap()
    .collect();
    
    let min_val: i32 = *hs.iter().min().unwrap();
    let max_val: i32 = *hs.iter().max().unwrap();
    let full_range: HashSet<i32> = (min_val..=max_val).into_iter().collect();
    *full_range.difference(&hs).collect::<Vec<&i32>>().pop().unwrap()


}