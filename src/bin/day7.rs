use std::cmp::Ordering;
use std::collections::HashMap;

use aoc2023::read_lines;

use crate::Hand::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair};

#[derive(Debug)]
enum Hand {
    FiveOfAKind(String),
    FourOfAKind(String),
    FullHouse(String),
    ThreeOfAKind(String),
    TwoPair(String),
    OnePair(String),
    HighCard(String),
}

fn compare_card_values(left: &str, right: &str) -> Ordering {
    for (l, r) in left.chars().zip(right.chars()) {
        let current_comparison = match (l, r) {
            ('A', 'A') => Ordering::Equal,
            ('A', _) => Ordering::Greater,
            (_, 'A') => Ordering::Less,
            ('K', 'K') => Ordering::Equal,
            ('K', _) => Ordering::Greater,
            (_, 'K') => Ordering::Less,
            ('Q', 'Q') => Ordering::Equal,
            ('Q', _) => Ordering::Greater,
            (_, 'Q') => Ordering::Less,
            ('J', 'J') => Ordering::Equal,
            ('J', _) => Ordering::Greater,
            (_, 'J') => Ordering::Less,
            ('T', 'T') => Ordering::Equal,
            ('T', _) => Ordering::Greater,
            (_, 'T') => Ordering::Less,
            (n1, n2) => n1.cmp(&n2),
        };
        if current_comparison != Ordering::Equal {
            return current_comparison;
        }
    }
    Ordering::Equal
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let mut counts: HashMap<char, usize> = HashMap::new();
        for char in value.chars() {
            let current_count = counts.get(&char).unwrap_or(&0);
            counts.insert(char, current_count + 1);
        }
        let owned_value = value.to_owned();
        for count in counts.values() {
            if *count == 5 {
                return FiveOfAKind(owned_value);
            }
            if *count == 4 {
                return FourOfAKind(owned_value);
            }
            if *count == 3 {
                if counts.values().any(|v| *v == 2) {
                    return FullHouse(owned_value);
                }
                return ThreeOfAKind(owned_value);
            }
            if *count == 2 {
                if counts.values().any(|v| *v == 3) {
                    return FullHouse(owned_value);
                }
                if counts
                    .values()
                    .filter(|v| **v == 2)
                    .collect::<Vec<_>>()
                    .len()
                    == 2
                {
                    return TwoPair(owned_value);
                }
                return OnePair(owned_value);
            }
        }
        HighCard(owned_value)
    }
}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FiveOfAKind(s0), FiveOfAKind(s1)) => s0 == s1,
            (FourOfAKind(s0), FourOfAKind(s1)) => s0 == s1,
            (FullHouse(s0), FullHouse(s1)) => s0 == s1,
            (ThreeOfAKind(s0), ThreeOfAKind(s1)) => s0 == s1,
            (TwoPair(s0), TwoPair(s1)) => s0 == s1,
            (OnePair(s0), OnePair(s1)) => s0 == s1,
            (HighCard(s0), HighCard(s1)) => s0 == s1,
            _ => false,
        }
    }
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (FiveOfAKind(s0), FiveOfAKind(s1)) => compare_card_values(s0.as_str(), s1.as_str()),
            (FiveOfAKind(_), _) => Ordering::Greater,
            (_, FiveOfAKind(_)) => Ordering::Less,
            (FourOfAKind(s0), FourOfAKind(s1)) => compare_card_values(s0.as_str(), s1.as_str()),
            (FourOfAKind(_), _) => Ordering::Greater,
            (_, FourOfAKind(_)) => Ordering::Less,
            (FullHouse(s0), FullHouse(s1)) => compare_card_values(s0.as_str(), s1.as_str()),
            (FullHouse(_), _) => Ordering::Greater,
            (_, FullHouse(_)) => Ordering::Less,
            (ThreeOfAKind(s0), ThreeOfAKind(s1)) => compare_card_values(s0.as_str(), s1.as_str()),
            (ThreeOfAKind(_), _) => Ordering::Greater,
            (_, ThreeOfAKind(_)) => Ordering::Less,
            (TwoPair(s0), TwoPair(s1)) => compare_card_values(s0.as_str(), s1.as_str()),
            (TwoPair(_), _) => Ordering::Greater,
            (_, TwoPair(_)) => Ordering::Less,
            (OnePair(s0), OnePair(s1)) => compare_card_values(s0.as_str(), s1.as_str()),
            (OnePair(_), _) => Ordering::Greater,
            (_, OnePair(_)) => Ordering::Less,
            (HighCard(s0), HighCard(s1)) => compare_card_values(s0.as_str(), s1.as_str()),
        }
    }
}

impl Round {
    fn do_the_joker_thing(&self) -> Self {
        let hand = match self.hand {
            FiveOfAKind(_) => FiveOfAKind(self.joker_hand.clone()),
            FourOfAKind(_) => FourOfAKind(self.joker_hand.clone()),
            FullHouse(_) => FullHouse(self.joker_hand.clone()),
            ThreeOfAKind(_) => ThreeOfAKind(self.joker_hand.clone()),
            TwoPair(_) => TwoPair(self.joker_hand.clone()),
            OnePair(_) => OnePair(self.joker_hand.clone()),
            HighCard(_) => HighCard(self.joker_hand.clone()),
        };
        Round {
            hand,
            joker_hand: self.joker_hand.clone(),
            bid: self.bid,
        }
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Round {
    hand: Hand,
    joker_hand: String,
    bid: i64,
}

impl Eq for Round {}

impl PartialEq<Self> for Round {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl PartialOrd<Self> for Round {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Round {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand.cmp(&other.hand)
    }
}

impl Round {
    fn new(value: &str, jokers: bool) -> Self {
        let (_value, _bid) = value.split_once(' ').unwrap();
        if jokers {
            let mut counts: HashMap<char, usize> = HashMap::new();
            for char in _value.chars() {
                let current_count = counts.get(&char).unwrap_or(&0);
                counts.insert(char, current_count + 1);
            }

            let most_common = counts
                .iter()
                .filter(|x| *x.0 != 'J')
                .max_by_key(|x| x.1)
                .map(|x| x.0)
                .unwrap_or(&'J');

            let __value = _value.replace('J', &most_common.to_string());
            let raw_jokered_hand = _value.replace('J', "0");
            Round {
                hand: __value.as_str().into(),
                joker_hand: raw_jokered_hand.to_owned(),
                bid: _bid.parse().unwrap(),
            }
        } else {
            Round {
                hand: _value.into(),
                joker_hand: _value.to_owned(),
                bid: _bid.parse().unwrap(),
            }
        }
    }
}

fn parse(input: &[String], jokers: bool) -> Vec<Round> {
    input
        .iter()
        .map(|x| {
            let r = Round::new(x.as_str(), jokers);
            r
        })
        .collect()
}

fn part_1(input: &[String]) {
    let rounds = parse(input, false);
    let mut sorted: Vec<&Round> = rounds.iter().collect();
    sorted.sort();
    let _total = sorted
        .iter()
        .enumerate()
        .fold(0, |acc, (rank, round)| acc + (rank + 1) as i64 * round.bid);

}

fn part_2(input: &[String]) {
    let rounds = parse(input, true);
    let mut sorted: Vec<Round> = rounds.iter().map(Round::do_the_joker_thing).collect();
    sorted.sort();
    let total = sorted
        .iter()
        .enumerate()
        .fold(0, |acc, (rank, round)| acc + (rank + 1) as i64 * round.bid);
    println!("day 7, part 2: {}", total);
}

fn main() {
    let input = read_lines("inputs/day7");
    part_1(&input);
    part_2(&input);
}
