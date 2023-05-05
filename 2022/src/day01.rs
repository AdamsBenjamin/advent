use std::fs;
use std::io;

const FILE_PATH: &str = "./inputs/day01";

fn main() -> io::Result<()> {

    let content = fs::read_to_string(FILE_PATH).expect("Couldn't read file.'");
    let packs = parse_packs(Vec::from_iter(content.split("\n").into_iter()));

    println!("The elf is carrying {:?} calories.", packs.iter().max().unwrap());

    Ok(())
}

fn parse_packs(lines: Vec<&str>) -> Vec<u32> {
    lines
        .split(|line| line.is_empty())
        .map(|pack| {
            pack.iter()
                .map(|item| item.parse::<u32>())
                .filter(|item| item.is_ok())
                .map(|item| item.unwrap())
                .sum::<u32>()
        })
        .collect()
}
