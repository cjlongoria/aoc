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
    fn new(data: &'a str) -> Self {
        let (cards, bid_str) = data.split_once(" ").unwrap();
        let bid = bid_str.parse().unwrap();
        let map = Hand::build_map(cards);
        let hand_type = Hand::convert_map_to_hand_type(map);
        let sort_score = Hand::sort_score(cards);
        Self {
            bid,
            cards,
            hand_type,
            sort_score,
        }
    }

    fn card_score(card: &char) -> u32 {
        "23456789TJQKA".chars().position(|x| x == *card).unwrap() as u32
    }

    fn sort_score(cards: &'a str) -> u32 {
        //We are representing the entire hand with a single number
        //by bit shifting 4 places for each card. We shift by 4 because
        //a card's value can be anywhere between 0-12
        cards
            .chars()
            .fold(0u32, |acc, x| (acc | Hand::card_score(&x)) << 4)
    }

    fn build_map(cards: &'a str) -> HashMap<char, usize> {
        let mut hm = HashMap::new();
        cards.chars().for_each(|x| {
            hm.entry(x).and_modify(|e| *e += 1usize).or_insert(1usize);
        });
        hm
    }

    fn convert_map_to_hand_type(hm: HashMap<char, usize>) -> u8 {
        let counts: Vec<usize> = hm.into_values().collect();
        if counts.contains(&5) {
            6
        } else if counts.contains(&4) {
            5
        } else if counts.contains(&3) && counts.contains(&2) {
            4
        } else if counts.contains(&3) {
            3
        } else if counts.iter().filter(|x| **x == 2usize).count() == 2 {
            2
        } else if counts.contains(&2) {
            1
        } else {
            0
        }
    }
}

pub fn part1(data: &str) {
    let mut hands: Vec<Hand> = data.lines().map(Hand::new).collect();
    hands.sort();
    let ans: usize = hands.iter().enumerate().map(|(i, x)| (i + 1) * x.bid).sum();
    println!("Day 7 part 1 answer - {}", ans);
}

