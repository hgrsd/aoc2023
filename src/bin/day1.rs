use std::collections::HashMap;

use aoc2023::read_lines;

fn part_1(lines: &[String]) {
    let total = lines.iter().fold(0u32, |total, line| {
        let mut digits = vec![];
        for char in line.chars() {
            if char.is_numeric() {
                digits.push(char.to_digit(10).unwrap());
            }
        }
        total + digits.first().unwrap() * 10 + digits.last().unwrap()
    });

    println!("day 1, part 1: {}", total)
}

fn try_parse(word: &str) -> Vec<u32> {
    let map: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let mut digits: Vec<(usize, u32)> = vec![];
    for key in map.keys() {
        let matches = word.match_indices(key);
        for (idx, m) in matches {
            let translation = map.get(m).unwrap();
            digits.push((idx, *translation));
        }
    }
    digits.sort_by_key(|(i, _)| *i);
    digits.iter().map(|(_, value)| *value).collect()
}

fn part_2(lines: &[String]) {
    let mut total = 0;

    for line in lines {
        let mut current_word: String = String::new();
        let mut collected_digits: Vec<u32> = vec![];

        for char in line.chars() {
            if char.is_alphabetic() {
                current_word.push(char);
            } else {
                collected_digits.extend(try_parse(&current_word));
                current_word.clear();
                let parsed = char.to_digit(10).unwrap();
                collected_digits.push(parsed);
            }
        }
        collected_digits.extend(try_parse(&current_word));
        total += collected_digits.first().unwrap() * 10 + collected_digits.last().unwrap();
    }

    println!("day 1, part 2: {}", total)
}

fn main() {
    let lines = read_lines("inputs/day1");
    part_1(&lines);
    part_2(&lines);
}