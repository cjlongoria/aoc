use crate::part1::part1;
use crate::part2::part2;
use clap::{Parser, ValueEnum};

#[derive(Parser)]
struct Args {
    #[arg(value_enum)]
    part: Part,

    #[arg(short, long)]
    test: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Part {
    P1,
    P2,
}

fn main() {
    let args = Args::parse();
    let data = match args.test {
        true => include_str!("../data/test"),
        false => include_str!("../data/data"),
    };

    let res = match args.part {
        Part::P1 => part1(data),
        Part::P2 => part2(data),
    };

    println!("ans: {}", res);
}

pub mod part1 {
    use crate::common::parse_data;

    pub fn part1(data: &str) -> usize {
        let (rule_map, updates) = parse_data(data);

        let mut mid_points: Vec<usize> = Vec::new();
        let mut pages: Vec<usize> = Vec::new();
        'outer: for update in updates {
            for page_num in update {
                if let Some(must_come_before) = rule_map.get(&page_num) {
                    for prev_page in &pages {
                        if must_come_before.contains(prev_page) {
                            pages.clear();
                            continue 'outer;
                        }
                    }
                }
                pages.push(page_num);
            }
            let mid_point = pages[pages.len() / 2];
            mid_points.push(mid_point);
            pages.clear();
        }
        mid_points.iter().sum()
    }
}

pub mod common {
    use std::collections::{HashMap, HashSet};

    pub fn parse_data(
        data: &str,
    ) -> (
        HashMap<usize, HashSet<usize>>,
        Vec<Vec<usize>>,
        // impl Iterator<Item = impl Iterator<Item = usize> + use<'_>>,
    ) {
        let (rules, updates) = split_data(data);
        let rule_map = gen_rule_map(rules);
        (rule_map, updates)
    }

    fn split_data(
        data: &str,
    ) -> (
        Vec<&str>,
        Vec<Vec<usize>>,
        // impl Iterator<Item = &str>,
        // impl Iterator<Item = impl Iterator<Item = usize> + use<'_>>,
    ) {
        data.split_once("\n\n")
            .map(|(rules, updates)| {
                (
                    rules.lines().collect(),
                    updates
                        .lines()
                        .map(|line| {
                            line.split(",")
                                .map(|update| update.parse::<usize>().unwrap())
                                .collect()
                        })
                        .collect(),
                )
            })
            .unwrap()
    }

    // fn gen_rule_map<'a>(rules: impl Iterator<Item = &'a str>) -> HashMap<usize, HashSet<usize>> {
    fn gen_rule_map(rules: Vec<&str>) -> HashMap<usize, HashSet<usize>> {
        let mut map: HashMap<usize, HashSet<usize>> = HashMap::new();
        for rule in rules {
            let (key, value): (usize, usize) = rule
                .split_once("|")
                .map(|(key, value)| (key.parse().unwrap(), value.parse().unwrap()))
                .unwrap();

            let values = map.entry(key).or_default();
            values.insert(value);
        }
        map
    }
}

pub mod part2 {
    use std::{
        cmp::Ordering,
        collections::{HashMap, HashSet},
    };

    use crate::common::parse_data;

    pub fn part2(data: &str) -> usize {
        let (rule_map, updates) = parse_data(data);

        let mut bad_orders: Vec<Vec<usize>> = Vec::new();
        let mut pages: Vec<usize> = Vec::new();
        for update in updates {
            let mut bad_order = false;
            for page_num in update {
                if let Some(must_come_before) = rule_map.get(&page_num) {
                    for prev_page in &pages {
                        if must_come_before.contains(prev_page) {
                            bad_order = true;
                        }
                    }
                }
                pages.push(page_num);
            }
            if bad_order {
                bad_orders.push(pages.clone());
            }
            pages.clear();
        }
        for order in bad_orders.iter_mut() {
            order.sort_by(|a, b| funky_sort(a, b, &rule_map))
        }
        bad_orders
            .into_iter()
            .map(|order| order[order.len() / 2])
            .sum()
    }

    fn funky_sort(a: &usize, b: &usize, map: &HashMap<usize, HashSet<usize>>) -> Ordering {
        if let Some(must_come_before) = map.get(b) {
            if must_come_before.contains(a) {
                return Ordering::Greater;
            }
        };

        if let Some(must_come_before) = map.get(a) {
            if must_come_before.contains(b) {
                return Ordering::Less;
            }
        };

        Ordering::Equal
    }
}
