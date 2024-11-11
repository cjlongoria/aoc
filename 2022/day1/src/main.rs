mod solution;

fn main() {
    let data = include_str!("../data/data");
    let mut max = 0;
    let mut totals = Vec::new();
    for line in data.lines() {
        if line.is_empty() {
            totals.push(max);
            max = 0;
        } else {
            max += line.parse::<u64>().unwrap();
        }
    }
    println!("Part 1: {}", solution::part1(&totals));
    println!("Part 2: {}", solution::part2(totals.as_mut_slice()));
}
