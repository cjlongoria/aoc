use std::collections::HashMap;

pub fn solution(data: String) {
    let digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];

    let digit_map: HashMap<&str, usize> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);

    let mut num = 0;
    for line in data.lines() {
        let mut hm_first: HashMap<&str, usize> = HashMap::new();
        let mut hm_last: HashMap<&str, usize> = HashMap::new();

        for key in digits {
            let first_pos = line.find(key);
            if let Some(value) = first_pos {
                hm_first.insert(key, value);
            }

            let last_pos = line.rfind(key);
            if let Some(value) = last_pos {
                hm_last.insert(key, value);
            }
        }
        let first_val = hm_first
            .iter()
            .min_by(|(_, b), (_, y)| b.cmp(y))
            .map(|(k, _)| k)
            .unwrap();
        let last_val = hm_last
            .iter()
            .max_by(|(_, b), (_, y)| b.cmp(y))
            .map(|(k, _)| k)
            .unwrap();

        let first_digit = digit_map.get(first_val).unwrap();
        let second_digit = digit_map.get(last_val).unwrap();
        let line_ans = (first_digit * 10) + second_digit;
        num += line_ans;
    }
    println!("Day 1, part 2 answer - {}", num);
}

