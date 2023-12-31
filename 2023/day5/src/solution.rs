use rangemap::RangeMap;

pub fn part1(data: &str) {
    let mut s2s = RangeMap::new();
    let mut s2f = RangeMap::new();
    let mut f2w = RangeMap::new();
    let mut w2l = RangeMap::new();
    let mut l2t = RangeMap::new();
    let mut t2h = RangeMap::new();
    let mut h2l = RangeMap::new();

    let mut maps = vec![
        &mut s2s, &mut s2f, &mut f2w, &mut w2l, &mut l2t, &mut t2h, &mut h2l,
    ]
    .into_iter();

    let mut data_iter = data.lines();

    //Get seeds and skip the empty line
    let seed_num: Vec<i64> = data_iter
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let mut data_lines: Vec<&str> = Vec::new();
    data_iter.next();

    for line in data_iter {
        if line.is_empty() {
            let current_map = maps.next().unwrap();
            for data in data_lines.clone() {
                let mut parsed_data = data.split_whitespace().map(|x| x.parse::<i64>().unwrap());
                let end_point = parsed_data.next().unwrap();
                let starting_point = parsed_data.next().unwrap();
                let offset = parsed_data.next().unwrap();
                let range = starting_point..starting_point + offset;
                current_map.insert(range, starting_point - end_point);
            }
            data_lines.clear();
        } else if line.contains(":") {
            continue;
        } else {
            data_lines.push(line);
        }
    }
    let current_map = maps.next().unwrap();

    for data in data_lines {
        let mut parsed_data = data.split_whitespace().map(|x| x.parse::<i64>().unwrap());
        let end_point = parsed_data.next().unwrap();
        let starting_point = parsed_data.next().unwrap();
        let offset = parsed_data.next().unwrap();
        let range = starting_point..starting_point + offset;
        current_map.insert(range, starting_point - end_point);
    }

    let ans = seed_num
        .iter()
        .map(|x| {
            vec![&s2s, &s2f, &f2w, &w2l, &l2t, &t2h, &h2l]
                .iter()
                .fold(x.clone(), |acc, rm| get_from_rangemap(*rm, acc))
        })
        .min()
        .unwrap();

    println!("Day5 part 1 answer - {}", ans);
}

fn get_from_rangemap(rm: &RangeMap<i64, i64>, input: i64) -> i64 {
    let num = rm.get(&input);
    if let Some(offset) = num {
        input - offset
    } else {
        input.clone()
    }
}

