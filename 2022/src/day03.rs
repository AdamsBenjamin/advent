use std::{fs, collections::{ HashMap, HashSet}};

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

fn find_badge(group: &[Vec<usize>]) -> usize {
    let mut entries = HashMap::new();

    for mut pack in group.iter().cloned() {
        pack.sort();
        pack.dedup();
        for entry in pack {
            let count: &mut usize = entries.entry(entry).or_default();
            *count += 1;
        }
    }

    *entries
        .iter()
        .filter(|(_, &v)| v == 3)
        .collect::<Vec<(&usize, &usize)>>()
        [0].0
}

fn main() {
    let content = fs::read_to_string(FILE_PATH)
        .expect("File not found");
    let priorities = content
        .lines()
        .map(|line| {
             line
             .chars()
             .map(prioritize)
             .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>();
    let groups: Vec<_> = priorities.chunks(3).collect();

    let badges: usize = groups.iter().map(|group| find_badge(&group)).sum();

    println!("{badges:#?}");
}
