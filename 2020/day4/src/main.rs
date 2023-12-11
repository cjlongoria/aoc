use std::fs;


fn main() {
    let contents: String = fs::read_to_string("data/data.txt")
        .expect("Should have been able to read the file");

    println!("{}", basic_filter(contents));

}

fn basic_filter(contents: String) -> usize {
    contents
    .split("\r\n\r\n")
    .filter(|x| part1_filter(x))
    .filter(|x| part2_filter(x))
    .count()
}

fn part1_filter(input: &str) -> bool {
    input.contains("byr") && input.contains("iyr") && input.contains("eyr") && input.contains("hgt") && input.contains("hcl") && input.contains("ecl") && input.contains("pid")
}

fn part2_filter(input: &str) -> bool {
    let kv_pairs = input
    .split_whitespace();

    for kv_string in kv_pairs {
        let mut iter = kv_string.split(':');
        let key = iter.next().unwrap();
        let value = iter.next().unwrap().trim_end();       
        let validated = match key {
            "byr" => (1920..=2002).contains(&value.parse::<i64>().unwrap_or_else(|_| -1)),
            "iyr" => (2010..=2020).contains(&value.parse::<i64>().unwrap_or_else(|_| -1)),
            "eyr" => (2020..=2030).contains(&value.parse::<i64>().unwrap_or_else(|_| -1)),
            "hgt" => height_check(value),
            "hcl" => color_check(value),
            "ecl" => value.contains("amb") || value.contains("blu") || value.contains("brn") || value.contains("gry") || value.contains("grn") || value.contains("hzl") || value.contains("oth"),
            "pid" => (value.len() == 9) && (0..=999999999).contains(&value.parse::<i64>().unwrap_or_else(|_| -1)),
            _ => continue
        };
        if !validated {return false};
    }
    true


}

fn height_check(value: &str) -> bool {
    if value.contains("cm"){
        let num: usize = value.strip_suffix("cm").unwrap().parse().unwrap();
        (150..=193).contains(&num)
    } else if value.contains("in"){
        let num: usize = value.strip_suffix("in").unwrap().parse().unwrap();
        (59..=76).contains(&num)
    } else {
        false
    }
}

fn color_check(value: &str) -> bool {
    if !value.starts_with("#") {return false};
    if value.len() != 7 {return false};
    for item in value.chars().skip(1) {
        if !(0..=15).contains(&item.to_digit(16).unwrap_or_else(|| 16)) {return false}
    }
    true
}