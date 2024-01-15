use std::collections::BTreeMap;

pub fn part1(data: &str) {
    let mut data_iter = data.lines();

    let mut instructions = data_iter.next().unwrap().chars().cycle();
    data_iter.next();

    let mut hm: BTreeMap<&str, (&str, &str)> = BTreeMap::new();
    for line in data_iter {
        let (key, value) = line.split_once(" = ").unwrap();
        let (left, right) = value
            .strip_prefix("(")
            .unwrap()
            .strip_suffix(")")
            .unwrap()
            .split_once(", ")
            .unwrap();
        hm.insert(key, (left, right));
    }

    let mut counter = 0;
    let mut node = "AAA";
    while node != "ZZZ" {
        counter += 1;
        let dir = instructions.next().unwrap();
        let (left, right) = hm.get(&node).unwrap();
        match dir {
            'R' => node = right,
            'L' => node = left,
            _ => panic!(),
        }
    }
    dbg!(counter);
}

