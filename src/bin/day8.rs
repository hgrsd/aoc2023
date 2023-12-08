use aoc2023::read_lines;
use num::Integer;
use std::collections::HashMap;

struct Node {
    left: String,
    right: String,
}

struct Graph {
    edges: HashMap<String, Node>,
}

impl From<&[String]> for Graph {
    fn from(value: &[String]) -> Self {
        let mut edges = HashMap::new();
        for line in value {
            let (node_name, current_edges) = line.split_once(" = ").unwrap();
            let left = current_edges.chars().skip(1).take(3).collect::<String>();
            let right = current_edges.chars().skip(6).take(3).collect::<String>();
            let node = Node { left, right };
            edges.insert(node_name.to_owned(), node);
        }
        Graph { edges }
    }
}

fn part_1(instructions: &str, graph: &Graph) {
    let mut iterations = 0;
    let mut current_node = "AAA";
    loop {
        for char in instructions.chars() {
            iterations += 1;
            let node = graph.edges.get(current_node).unwrap();
            let next_node = match char {
                'L' => &node.left,
                'R' => &node.right,
                _ => unreachable!(),
            };
            if next_node.as_str() == "ZZZ" {
                println!("day 8, part 1: {}", iterations);
                return;
            }
            current_node = next_node.as_str();
        }
    }
}

fn part_2(instructions: &str, graph: &Graph) {
    let current_nodes: Vec<&str> = graph
        .edges
        .keys()
        .filter_map(|k| {
            if k.ends_with('A') {
                Some(k.as_str())
            } else {
                None
            }
        })
        .collect();
    let mut multipliers: Vec<i64> = vec![];
    for node in current_nodes {
        let mut cur = node;
        let mut iterations = 0;
        'outer: loop {
            for char in instructions.chars() {
                iterations += 1;
                let node = graph.edges.get(cur).unwrap();
                cur = match char {
                    'L' => &node.left,
                    'R' => &node.right,
                    _ => unreachable!(),
                };
                if cur.ends_with('Z') {
                    multipliers.push(iterations);
                    break 'outer;
                }
            }
        }
    }

    let total = multipliers
        .iter()
        .fold(1i64, |left, right| left.lcm(right));
    println!("day 8, part 2: {}", total);
}

fn parse(input: &[String]) -> (String, Graph) {
    let mut iter = input.iter();
    let instructions = iter.next().unwrap().to_owned(); // first line is inputs
    iter.next().unwrap(); // discard the newline before the graph starts
    let remainder = iter.cloned().collect::<Vec<String>>();
    let graph = Graph::from(remainder.as_slice());
    (instructions, graph)
}

fn main() {
    let input = read_lines("inputs/day8");
    let (instructions, graph) = parse(&input);
    part_1(&instructions, &graph);
    part_2(&instructions, &graph);
}
