pub fn solve(data: &str, part2: bool) {
    let ans: i64 = data
        .lines()
        .map(|line| {
            let measurements: Vec<i64> = line
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect();

            calc(&measurements, part2)
        })
        .sum();

    if part2 {
        println!("day 9 part 2 - {}", ans);
    } else {
        println!("day 9 part 1 - {}", ans);
    }
}

fn calc(measurements: &[i64], part2: bool) -> i64 {
    if measurements.iter().all(|&x| x == 0) {
        0i64
    } else {
        let differences: Vec<i64> = measurements
            .iter()
            .scan(0i64, |state, item| {
                let tmp = *item - *state;
                *state = *item;
                Some(tmp)
            })
            .skip(1)
            .collect();

        if part2 {
            measurements.first().unwrap() - calc(&differences, part2)
        } else {
            measurements.last().unwrap() + calc(&differences, part2)
        }
    }
}

