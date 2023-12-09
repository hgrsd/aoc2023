use aoc2023::read_lines;

fn parse(input: &[String]) -> Vec<Vec<i64>> {
    input
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.parse().unwrap())
                .collect()
        })
        .collect()
}

fn expand(row: &[i64]) -> Vec<Vec<i64>> {
    let mut increases: Vec<Vec<i64>> = vec![row.to_vec()];
    let mut i = 0;
    loop {
        let steps: Vec<i64> = increases[i]
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect();
        i += 1;
        increases.push(steps.clone());
        if steps.iter().all(|step| *step == 0) {
            break;
        }
    }
    increases
}

fn main() {
    let input = read_lines("inputs/day9");
    let parsed = parse(&input);
    println!(
        "day 9, part 1: {}",
        parsed.iter().fold(0i64, |acc, row| {
            acc + expand(row)
                .iter()
                .rfold(0, |acc, cur| acc + cur.last().unwrap())
        })
    );
    println!(
        "day 9, part 2: {}",
        parsed.iter().fold(0i64, |acc, row| {
            acc + expand(row)
                .iter()
                .rfold(0, |acc, cur| cur.first().unwrap() - acc)
        })
    );
}
