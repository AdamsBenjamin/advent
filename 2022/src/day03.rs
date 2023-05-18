use std::fs;

const FILE_PATH: &str = "./inputs/day03";


fn prioritize(character: char) -> usize {
    let chars = {
        let mut chars = Vec::new();
        chars.extend('a'..='z');
        chars.extend('A'..='Z');
        chars
    };
    chars.iter().position(|&c| c == character).unwrap() + 1usize
}

fn find_misplaced_entry(entries: Vec<usize>) -> usize {
    let mut halves = entries.chunks(entries.len() / 2);
    let front = halves.next().unwrap_or_else(|| panic!("failed to chunk front"));
    let back = halves.next().unwrap_or_else(|| panic!("failed to chunk back"));

    if let Some(i) = front.iter().position(|e| back.contains(e)) {
        front[i]
    } else {
        panic!("Couldn't find matching elements.")
    }
}

fn main() {
    let content = fs::read_to_string(FILE_PATH)
        .expect("File not found");
    let lines = content.lines()
        .map(|line| {
             line
             .chars()
             .map(prioritize)
             .collect::<Vec<usize>>()
        });

    let matches: Vec<_> = lines.map(find_misplaced_entry).collect();
    let answers: usize = matches.iter().sum();

    println!("{answers:#?}");
}
