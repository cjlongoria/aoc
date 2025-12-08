use std::{ops::Range, str::Lines};

use itertools::Itertools;
use rangemap::RangeMap;

struct MapOfMaps<T> {
    seed_nums: Vec<T>,
    s2s: RangeMap<i64, i64>,
    s2f: RangeMap<i64, i64>,
    f2w: RangeMap<i64, i64>,
    w2l: RangeMap<i64, i64>,
    l2t: RangeMap<i64, i64>,
    t2h: RangeMap<i64, i64>,
    h2l: RangeMap<i64, i64>,
}

impl MapOfMaps<i64> {
    fn build_i64(data: &str) -> Self {
        let mut data_iter = data.lines();

        let seed_nums = data_iter
            .next()
            .unwrap()
            .strip_prefix("seeds: ")
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        data_iter.next();

        let mut new_struct = Self {
            seed_nums,
            s2s: RangeMap::new(),
            s2f: RangeMap::new(),
            f2w: RangeMap::new(),
            w2l: RangeMap::new(),
            l2t: RangeMap::new(),
            t2h: RangeMap::new(),
            h2l: RangeMap::new(),
        };
        new_struct.build_maps(data_iter);
        new_struct
    }

    fn solve(self) {
        let ans = self
            .seed_nums
            .iter()
            .map(|seed_num| {
                vec![
                    &self.s2s, &self.s2f, &self.f2w, &self.w2l, &self.l2t, &self.t2h, &self.h2l,
                ]
                .iter()
                .fold(*seed_num, |acc, rm| get_int_from_rangemap(*rm, acc))
            })
            .min()
            .unwrap();

        println!("Day5 part 1 answer - {}", ans);
    }
}

impl MapOfMaps<Range<i64>> {
    fn build_ranges(data: &str) -> Self {
        let mut data_iter = data.lines();

        let seed_nums = data_iter
            .next()
            .unwrap()
            .strip_prefix("seeds: ")
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .tuples()
            .map(|(x, y)| x..x + y)
            .collect();

        data_iter.next();

        let mut new_struct = Self {
            seed_nums,
            s2s: RangeMap::new(),
            s2f: RangeMap::new(),
            f2w: RangeMap::new(),
            w2l: RangeMap::new(),
            l2t: RangeMap::new(),
            t2h: RangeMap::new(),
            h2l: RangeMap::new(),
        };

        new_struct.build_maps(data_iter);
        new_struct
    }

    fn solve(self) {
        let ans = self
            .seed_nums
            .iter()
            .flat_map(|seed_range| {
                vec![
                    &self.s2s, &self.s2f, &self.f2w, &self.w2l, &self.l2t, &self.t2h, &self.h2l,
                ]
                .iter()
                .fold(vec![seed_range.clone()], |acc, rm| {
                    get_ranges_from_rangemap(*rm, &acc)
                })
            })
            .min_by_key(|x| x.start)
            .unwrap()
            .start;

        println!("Day5 part 2 answer - {:?}", ans);
    }
}

impl<T> MapOfMaps<T> {
    fn build_maps(&mut self, lines: Lines) {
        let mut maps = vec![
            &mut self.s2s,
            &mut self.s2f,
            &mut self.f2w,
            &mut self.w2l,
            &mut self.l2t,
            &mut self.t2h,
            &mut self.h2l,
        ]
        .into_iter();

        let mut data_lines: Vec<&str> = Vec::new();

        for line in lines {
            if line.is_empty() {
                let current_map = maps.next().unwrap();
                for data in data_lines.clone() {
                    let mut parsed_data =
                        data.split_whitespace().map(|x| x.parse::<i64>().unwrap());
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
    }
}

fn get_ranges_from_rangemap(rm: &RangeMap<i64, i64>, input: &[Range<i64>]) -> Vec<Range<i64>> {
    input
        .iter()
        .flat_map(|x| rm.overlapping(x))
        .map(|(x, y)| x.start - *y..x.end - *y)
        .collect()
}

fn get_int_from_rangemap(rm: &RangeMap<i64, i64>, input: i64) -> i64 {
    let num = rm.get(&input);

    if let Some(offset) = num {
        input - offset
    } else {
        input.clone()
    }
}

pub fn part1(data: &str) {
    MapOfMaps::build_i64(data).solve();
}

pub fn part2(data: &str) {
    MapOfMaps::build_ranges(data).solve();
}

