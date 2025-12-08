use std::collections::{HashMap, HashSet};

pub fn part1(data: &str) {
    let mut total_sum: usize = 0;
    for line in data.lines() {
        let mut parsed_data = line.split(":").last().unwrap().trim().split("|");
        let winning_nums: HashSet<&str> = parsed_data.next().unwrap().split_whitespace().collect();
        let my_nums: HashSet<&str> = parsed_data.next().unwrap().split_whitespace().collect();
        let winning_num_count = winning_nums.intersection(&my_nums).count();
        if winning_num_count != 0 {
            total_sum += 2usize.pow((winning_num_count as u32) - 1);
        }
    }
    println!("Day 4 part 1 answer - {}", total_sum);
}

pub fn part2(data: &str) {
    let mut number_counter: HashMap<usize, usize> = HashMap::new();

    for (game_id, line) in data.lines().enumerate() {
        let (_, numbers) = line.split_once(':').unwrap();
        let (win_num_string, my_num_string) = numbers.trim().split_once("|").unwrap();
        let winning_nums: HashSet<&str> = win_num_string.split_whitespace().collect();
        let my_nums: HashSet<&str> = my_num_string.split_whitespace().collect();
        let winning_num_count = winning_nums.intersection(&my_nums).count();
        let count = number_counter
            .entry(game_id)
            .and_modify(|counter| *counter += 1)
            .or_insert(1)
            .clone();

        for i in game_id + 1..=game_id + winning_num_count {
            number_counter
                .entry(i)
                .and_modify(|counter| *counter += count)
                .or_insert(count);
        }
    }
    let total_sum: usize = number_counter.values().sum();

    println!("Day 4 part 2 answer - {}", total_sum);
}

