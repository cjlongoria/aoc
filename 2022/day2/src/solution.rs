pub fn part1(data: &[u64]) -> u64 {
    *(data.iter().max().unwrap())
}

pub fn part2(data: &mut [u64]) -> u64 {
    data.sort();
    data.iter().rev().take(3).sum()
}
