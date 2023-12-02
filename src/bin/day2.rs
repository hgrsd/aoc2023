use aoc2023::read_lines;

struct Turn {
    reds: u32,
    greens: u32,
    blues: u32,
}

struct Game {
    id: u32,
    turns: Vec<Turn>,
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let (identifier, round) = value.split_once(": ").unwrap();
        let id = identifier
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();

        let turns = round
            .split("; ")
            .map(|turn| {
                let mut reds = 0;
                let mut greens = 0;
                let mut blues = 0;
                for cubes in turn.split(", ") {
                    let (n, colour) = cubes.split_once(' ').unwrap();
                    let amount = n.parse().unwrap();
                    match colour {
                        "red" => reds = amount,
                        "green" => greens = amount,
                        "blue" => blues = amount,
                        _ => {}
                    }
                }
                Turn {
                    reds,
                    greens,
                    blues,
                }
            })
            .collect();

        Game { id, turns }
    }
}

fn part_1(lines: &[String]) {
    let games = lines.iter().map(|x| x.as_str().into());
    let possible_games = games.filter(|game: &Game| {
        game.turns
            .iter()
            .all(|turn| turn.reds <= 12 && turn.greens <= 13 && turn.blues <= 14)
    });
    let summed_ids = possible_games.fold(0, |sum, game| sum + game.id);
    println!("day 2, part 1: {}", summed_ids);
}

fn part_2(lines: &[String]) {
    let games = lines.iter().map(|x| x.as_str().into());
    let summed_powers = games.fold(0, |acc, game: Game| {
        let (r, g, b) = game.turns.iter().fold((0, 0, 0), |(r, g, b), turn| {
            (r.max(turn.reds), g.max(turn.greens), b.max(turn.blues))
        });
        acc + r * g * b
    });
    println!("day 2, part 2: {}", summed_powers);
}

fn main() {
    let lines = read_lines("inputs/day1");
    part_1(&lines);
    part_2(&lines);
}
