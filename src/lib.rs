use std::path::Path;

pub trait Solution {
    fn solve(&self);
}

pub fn read_lines(path: &str) -> Vec<String> {
    std::fs::read_to_string(Path::new(path))
        .expect("Unable to read a file at the path provided")
        .lines()
        .map(String::from)
        .collect()
}
