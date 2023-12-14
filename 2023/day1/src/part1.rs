use std::fs;

pub fn solution() {
    let data: String = fs::read_to_string("./src/data/data1")
        .unwrap()
        .parse()
        .unwrap();

    let mut num: usize = 0;
    for line in data.lines() {
        let char_list: Vec<char> = line.chars().filter(|x| x.is_numeric()).collect();
        let first_digit_string: String = char_list.first().unwrap().to_string();
        let second_digit_string: String = char_list.last().unwrap().to_string();
        let number_for_line = first_digit_string + &second_digit_string;
        num += number_for_line.parse::<usize>().unwrap();
    }
    println!("Day 1, part 1 answer - {}", num);
}
