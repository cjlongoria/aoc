use std::fs;

fn part1() {
    let contents: String = fs::read_to_string("data/data.txt")
        .expect("Should have been able to read the file");
    let results: Vec<&str> = contents
        .lines()
        .filter_map(|x| {
            let split_string: Vec<&str> = x.split_whitespace().collect();
            let bounds = &mut split_string[0].split("-");
            let lower_bound: usize = bounds.next().unwrap().parse().unwrap();
            let upper_bound: usize = bounds.next().unwrap().parse().unwrap();
            let policy_char: &str = &split_string[1].trim_end_matches(":");
            let unchecked_password: &str = split_string[2];

            let char_count: usize = unchecked_password.matches(&policy_char).count();

            if (lower_bound..=upper_bound).contains(&char_count) {
                Some(unchecked_password)
            } else {
                None
            }
            }
        )
        .collect();

    println!("{:#?}", &results);
    println!("Valid passwords - {}", results.len());
}

fn part2() {
    let contents: String = fs::read_to_string("data/data.txt")
        .expect("Should have been able to read the file");

    let mut counter = 0;
    for line in contents.lines() {
        let split_string: Vec<&str> = line.split_whitespace().collect();
        let bounds = &mut split_string[0].split("-");
        let first_index: usize = bounds.next().unwrap().parse().unwrap();
        let second_index: usize = bounds.next().unwrap().parse().unwrap();
        let policy_letter: char = split_string[1].trim_end_matches(":").chars().next().unwrap();
        let unchecked_password: &str = split_string[2];

        let first_check: bool = unchecked_password.chars().nth(first_index-1).unwrap() == policy_letter;
        let second_check: bool = unchecked_password.chars().nth(second_index-1).unwrap() == policy_letter;

        if first_check ^ second_check {
            counter += 1;
        } 
    }
    println!("Valid passwords - {}", counter);
}


fn main() {
    // part1();
    part2();
}
