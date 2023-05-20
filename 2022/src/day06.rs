use std::{fs, collections::HashSet};

const FILE_PATH: &str = "./inputs/day06";


fn parse_packets(signal: &str) -> Option<usize> {
    const MARKER_LENGTH: usize = 4;

    let chars: Vec<_> = signal.chars().collect();

    chars
        .windows(MARKER_LENGTH)
        .enumerate()
        .filter(|(_, chars)| {
            chars.iter().collect::<HashSet<_>>().len() == MARKER_LENGTH
        })
        .next()
        .map(|e| e.0 + MARKER_LENGTH)
}


fn main() {
    let contents = fs::read_to_string(FILE_PATH)
        .expect("Failed to read file at {FILE_PATH}.");
    let answers = parse_packets(&contents);
    println!("{answers:?}");
}
