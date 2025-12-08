use std::collections::HashMap;

// hand_type is an int value that correspondes to how strong the hand is
// High card = 0
// One pair = 1
// Two pair = 2
// Three of a kind = 3
// Full house = 4
// Four of a kind = 5
// Five of a kind = 6
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand<'a> {
    hand_type: u8,
    sort_score: u32,
    bid: usize,
    cards: &'a str,
}

impl<'a> Hand<'a> {
    fn new(data: &'a str, joker: bool) -> Self {
        let (cards, bid_str) = data.split_once(" ").unwrap();
        let bid = bid_str.parse().unwrap();
        let map = Hand::build_map(cards);
        let hand_type = Hand::convert_map_to_hand_type(map, joker);
        let sort_score = Hand::sort_score(cards, joker);
        Self {
            bid,
            cards,
            hand_type,
            sort_score,
        }
    }

    fn card_score(card: &char, joker: bool) -> u32 {
        if joker {
            "J23456789TQKA".chars().position(|x| x == *card).unwrap() as u32
        } else {
            "23456789TJQKA".chars().position(|x| x == *card).unwrap() as u32
        }
    }

    fn sort_score(cards: &'a str, joker: bool) -> u32 {
        //We are representing the entire hand with a single number
        //by bit shifting 4 places for each card. We shift by 4 because
        //a card's value can be anywhere between 0-12
        cards
            .chars()
            .fold(0u32, |acc, x| (acc | Hand::card_score(&x, joker)) << 4)
    }

    fn build_map(cards: &'a str) -> HashMap<char, usize> {
        let mut hm = HashMap::new();
        cards.chars().for_each(|x| {
            hm.entry(x).and_modify(|e| *e += 1usize).or_insert(1usize);
        });
        hm
    }

    fn convert_map_to_hand_type(hm: HashMap<char, usize>, joker: bool) -> u8 {
        let joker_count = hm.get(&'J').copied().unwrap_or(0);
        let counts: Vec<usize> = if joker {
            hm.iter()
                .filter(|(k, _)| **k != 'J')
                .map(|(_, v)| v)
                .copied()
                .collect()
        } else {
            hm.into_values().collect()
        };

        let mut hand_type: u8 = match counts {
            x if x.contains(&5) => 6,
            x if x.contains(&4) => 5,
            x if x.contains(&3) && x.contains(&2) => 4,
            x if x.contains(&3) => 3,
            x if x.iter().filter(|x| **x == 2usize).count() >= 2 => 2,
            x if x.contains(&2) => 1,
            _ => 0,
        };

        if joker {
            hand_type = match (hand_type, joker_count) {
                (_, 0) => hand_type,
                (x, 1) if [0, 4, 5].contains(&x) => hand_type + 1,
                (x, 1) if [1, 2, 3].contains(&x) => hand_type + 2,
                (x, 2) if [0, 3].contains(&x) => hand_type + 3,
                (x, 2) if [1].contains(&x) => hand_type + 4,
                (_, 3) => hand_type + 5,
                (_, y) if [4, 5].contains(&y) => 6,
                _ => panic!(),
            }
        }
        hand_type
    }
}

pub fn part1(data: &str) {
    let mut hands: Vec<Hand> = data.lines().map(|cards| Hand::new(cards, false)).collect();
    hands.sort();
    let ans: usize = hands.iter().enumerate().map(|(i, x)| (i + 1) * x.bid).sum();
    println!("Day 7 part 1 answer - {}", ans);
}

pub fn part2(data: &str) {
    let mut hands: Vec<Hand> = data.lines().map(|cards| Hand::new(cards, true)).collect();
    hands.sort();
    let ans: usize = hands.iter().enumerate().map(|(i, x)| (i + 1) * x.bid).sum();
    println!("Day 7 part 2 answer - {}", ans);
}

