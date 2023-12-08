use aoc2023::read_lines;

struct Race {
    duration: i64,
    distance: i64,
}

fn parse_1(input: &[String]) -> Vec<Race> {
    let mut iter = input.iter();
    let times = iter
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap());
    let distances = iter
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap());
    times
        .zip(distances)
        .map(|(duration, distance)| Race { duration, distance })
        .collect()
}

fn parse_2(input: &[String]) -> Vec<Race> {
    let mut iter = input.iter();
    let duration = iter
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<i64>()
        .unwrap();
    let distance = iter
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<i64>()
        .unwrap();
    vec![Race { duration, distance }]
}

fn do_the_thing(races: &[Race]) {
    let n_options = races.iter().fold(1, |acc, cur| {
        acc * (0..cur.duration)
            .filter(|milliseconds| (milliseconds * (cur.duration - milliseconds)) > cur.distance)
            .collect::<Vec<_>>()
            .len()
    });
    println!("{}", n_options)
}

fn main() {
    let lines = read_lines("inputs/day6");
    let r1 = parse_1(&lines);
    let r2 = parse_2(&lines);
    do_the_thing(&r1);
    do_the_thing(&r2);
}
