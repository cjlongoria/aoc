pub fn solve(data: &str) {
    let ans: i64 = data
        .lines()
        .map(|line| {
            let measurements: Vec<i64> = line
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect();

            calc(&measurements)
        })
        .sum();

    println!("day 9 part 1 - {}", ans);
}

fn calc(measurements: &[i64]) -> i64 {
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

        let ans = measurements.last().unwrap() + calc(&differences);
        ans
    }
}

