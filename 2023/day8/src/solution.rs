use crate::lcm::lcm;
use std::collections::BTreeMap;
use std::iter::Cycle;
use std::str::Chars;

pub fn solve(data: &str) {
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
    part1_solve(&hm, &mut instructions);
    //We can use a &mut again because the intructions wraps around completely
    //and starts back at the first element. If it wasn't a clean iteration
    //then I would .clone() the instructions.
    part2_solve(&hm, &mut instructions);
}

fn part1_solve(hm: &BTreeMap<&str, (&str, &str)>, instructions: &mut Cycle<Chars>) {
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
    println!("Day 8 part 1 ans - {}", counter);
}

fn part2_solve(hm: &BTreeMap<&str, (&str, &str)>, instructions: &mut Cycle<Chars>) {
    let mut starting_nodes: Vec<&str> = hm
        .keys()
        .copied()
        .filter(|key| key.ends_with("A"))
        .collect();

    let mut keep_going = true;
    let mut cycle_lengths = Vec::new();
    let mut counter = 0;

    while keep_going {
        let dir = instructions.next().unwrap();
        let mut new_nodes = Vec::new();

        keep_going = false;
        for node in &starting_nodes {
            if !node.ends_with("Z") {
                keep_going = true;
                let (left, right) = hm.get(node).unwrap();
                match dir {
                    'R' => new_nodes.push(*right),
                    'L' => new_nodes.push(*left),
                    _ => panic!(),
                }
            } else {
                cycle_lengths.push(counter);
            }
        }
        counter += 1;
        starting_nodes = new_nodes;
    }
    let ans = lcm(&cycle_lengths);
    println!("Day 8 part 2 ans - {}", ans);
}

