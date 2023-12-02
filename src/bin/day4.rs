use std::collections::{HashMap, HashSet};

use aoc2023::read_lines;

struct ScratchCard {
    id: usize,
    numbers: Vec<u8>,
    winning_numbers: HashSet<u8>,
}

impl From<&str> for ScratchCard {
    fn from(value: &str) -> Self {
        let (id_part, rest) = value.split_once(':').unwrap();
        let id = id_part
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let (winning_nums, nums) = rest.split_once('|').unwrap();
        let winning_numbers = HashSet::from_iter(
            winning_nums
                .split_whitespace()
                .map(|n| n.parse::<u8>().unwrap()),
        );
        let numbers = nums
            .split_whitespace()
            .map(|n| n.parse::<u8>().unwrap())
            .collect();

        ScratchCard {
            id,
            numbers,
            winning_numbers,
        }
    }
}

fn winning_numbers(scratch_card: &ScratchCard) -> usize {
    scratch_card
        .numbers
        .iter()
        .filter(|num| scratch_card.winning_numbers.contains(*num))
        .collect::<Vec<_>>()
        .len()
}

fn part_1(scratch_cards: &[ScratchCard]) {
    let total = scratch_cards.iter().fold(0, |total, card| {
        let winning = winning_numbers(card);
        if winning > 0 {
            let exponent: u32 = (winning - 1) as u32;
            let score = 2_u32.pow(exponent);
            total + score
        } else {
            total
        }
    });
    println!("day 4, part 1: {}", total);
}

fn part_2(scratch_cards: &[ScratchCard]) {
    let mut copies = HashMap::new();

    for card in scratch_cards {
        let copies_of_current_card = copies.get(&card.id).unwrap_or(&0) + 1;
        copies.insert(card.id, copies_of_current_card);

        let winning = winning_numbers(card);
        for i in 1..=winning {
            let key = card.id + i;
            let existing_copies = copies.get(&key).unwrap_or(&0);
            copies.insert(key, existing_copies + copies_of_current_card);
        }
    }

    let total_copies: i32 = copies.values().sum();
    println!("day 4, part 2: {}", total_copies);
}

fn main() {
    let scratch_cards = read_lines("inputs/day4")
        .iter()
        .map(|line| ScratchCard::from(line.as_str()))
        .collect::<Vec<_>>();
    part_1(&scratch_cards);
    part_2(&scratch_cards);
}
