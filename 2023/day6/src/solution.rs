pub fn part1(data: &str) {
    let mut data_iter = data.lines();

    let times = data_iter
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap());

    let distances = data_iter
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap());

    let datapoints = times.zip(distances);
    let ans = calculate(datapoints);
    println!("Day 6 part 1 answer - {}", ans);
}

pub fn part2(data: &str) {
    let mut data_iter = data.lines();

    let times = data_iter
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .concat()
        .parse::<i64>()
        .unwrap();

    let distances = data_iter
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .concat()
        .parse::<i64>()
        .unwrap();

    let datapoints = [(times, distances)].into_iter();
    let ans = calculate(datapoints);
    println!("Day 6 part 2 answer - {}", ans);
}

fn calculate(datapoints: impl Iterator<Item = (i64, i64)>) -> i64 {
    let mut start = 0;
    let mut end = 0;
    let mut ans = 1;
    for datapoint in datapoints {
        let (time, dist) = datapoint;
        for x in 1..time {
            if x * (time - x) > dist {
                start = x;
                break;
            }
        }
        for x in (1..time).rev() {
            if x * (time - x) > dist {
                end = x;
                break;
            }
        }
        if end > 0 {
            ans *= (end - start) + 1;
        }
    }
    ans
}

