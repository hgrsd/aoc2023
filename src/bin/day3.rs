use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use aoc2023::read_lines;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
struct Coordinate {
    x: i32,
    y: i32,
}

#[derive(Clone)]
enum Thing {
    Period(Vec<Coordinate>),
    Symbol(Vec<Coordinate>, char),
    Number(Vec<Coordinate>, u32),
}

fn neighbouring_coordinates(of: &Coordinate) -> Vec<Coordinate> {
    vec![
        // left neighbour
        Coordinate {
            x: of.x - 1,
            y: of.y,
        },
        // top-left neighbour
        Coordinate {
            x: of.x - 1,
            y: of.y - 1,
        },
        // top neighbour
        Coordinate {
            x: of.x,
            y: of.y - 1,
        },
        // top-right neighbour
        Coordinate {
            x: of.x + 1,
            y: of.y - 1,
        },
        // right neighbour
        Coordinate {
            x: of.x + 1,
            y: of.y,
        },
        // bottom-right neighbour
        Coordinate {
            x: of.x + 1,
            y: of.y + 1,
        },
        // bottom neighbour
        Coordinate {
            x: of.x,
            y: of.y + 1,
        },
        // bottom-left neighbour
        Coordinate {
            x: of.x - 1,
            y: of.y + 1,
        },
    ]
}

struct Grid {
    grid: HashMap<Coordinate, Thing>,
}

impl Grid {
    pub fn things(&self) -> Vec<&Thing> {
        let mut v = vec![];
        let mut seen = HashSet::new();
        for thing in self.grid.values() {
            match thing {
                Thing::Number(coords, _) => {
                    if !seen.contains(coords) {
                        v.push(thing);
                    }
                    seen.insert(coords);
                }
                _ => v.push(thing),
            }
        }
        v
    }

    pub fn neighbours(&self, coordinates: &Vec<Coordinate>) -> Vec<&Thing> {
        let mut v = vec![];
        let mut seen = HashSet::new();
        for coordinate in coordinates {
            let n = neighbouring_coordinates(coordinate);
            for coordinate in n {
                let thing = self.grid.get(&coordinate);
                match thing {
                    Some(Thing::Number(coords, _)) => {
                        if !seen.contains(coords) {
                            v.push(thing.unwrap());
                        }
                        seen.insert(coords);
                    }
                    Some(_) => v.push(thing.unwrap()),
                    _ => {}
                }
            }
        }
        v
    }
}

fn number(digits: &str, coordinates: &[Coordinate]) -> Thing {
    let parsed_number = digits.parse::<u32>().unwrap();
    Thing::Number(coordinates.to_owned(), parsed_number)
}

impl From<&[String]> for Grid {
    fn from(value: &[String]) -> Self {
        let mut map = HashMap::new();
        let mut current_coordinate = Coordinate { x: 0, y: 0 };
        for line in value {
            let mut digits = String::new();
            let mut digit_coordinates = vec![];
            for char in line.chars() {
                if char.is_numeric() {
                    digits.push(char);
                    digit_coordinates.push(current_coordinate);
                } else {
                    if !digits.is_empty() {
                        for coordinate in &digit_coordinates {
                            map.insert(*coordinate, number(&digits, &digit_coordinates));
                        }
                    }
                    digits.clear();
                    digit_coordinates.clear();
                    match char {
                        '.' => {
                            map.insert(current_coordinate, Thing::Period(vec![current_coordinate]));
                        }
                        _ => {
                            map.insert(
                                current_coordinate,
                                Thing::Symbol(vec![current_coordinate], char),
                            );
                        }
                    }
                }
                current_coordinate = Coordinate {
                    x: current_coordinate.x + 1,
                    y: current_coordinate.y,
                }
            }
            if !digits.is_empty() {
                for coordinate in &digit_coordinates {
                    map.insert(*coordinate, number(&digits, &digit_coordinates));
                }
            }
            current_coordinate = Coordinate {
                x: 0,
                y: current_coordinate.y + 1,
            }
        }
        Grid { grid: map }
    }
}

fn part_1(grid: &Grid) {
    let total = grid.things().iter().fold(0, |total, thing| match thing {
        Thing::Number(coords, value) => {
            let neighbouring_things = grid.neighbours(coords);
            if neighbouring_things
                .iter()
                .any(|thing| matches!(thing, Thing::Symbol(_, _)))
            {
                total + value
            } else {
                total
            }
        }
        _ => total,
    });

    println!("day 3, part 1: {}", total);
}

fn part_2(grid: &Grid) {
    let mut total = 0;

    for thing in grid.things() {
        if let Thing::Symbol(coords, '*') = thing {
            let neighbours = grid.neighbours(coords);
            let neighbouring_numbers = neighbours
                .iter()
                .filter(|thing| matches!(thing, Thing::Number(_, _)))
                .collect::<Vec<&&Thing>>();
            if neighbouring_numbers.len() == 2 {
                let first = if let Thing::Number(_, val) = neighbouring_numbers[0] {
                    *val
                } else {
                    0
                };
                let second = if let Thing::Number(_, val) = neighbouring_numbers[1] {
                    *val
                } else {
                    0
                };
                total += first * second;
            }
        }
    }

    println!("day 3, part 2: {}", total);
}

fn main() {
    let lines = read_lines("inputs/day3");
    let grid = Grid::from(lines.as_slice());
    part_1(&grid);
    part_2(&grid);
}
