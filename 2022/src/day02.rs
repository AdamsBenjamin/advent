// This solution is a bit contrived.
// Just trying to find an excuse to use fancy features.

use std::io;
use std::fs;

const FILE_PATH: &str = "./inputs/day02";

trait Read<A> {
    fn read(string: A) -> Option<Self>
        where Self: Sized;
}


trait Score {
    fn score(&self) -> u32;
}

#[derive(Debug,Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Score for Shape {
    fn score(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3
        }
    }
}



#[derive(Debug,Clone)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Score for Outcome {
    fn score(&self) -> u32 {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

impl Read<char> for Shape {
    fn read(c: char) -> Option<Shape> {
        match c {
            'A' => Some(Shape::Rock),
            'B' => Some(Shape::Paper),
            'C' => Some(Shape::Scissors),
            'X' => Some(Shape::Rock),
            'Y' => Some(Shape::Paper),
            'Z' => Some(Shape::Scissors),
            _   => {
                println!("Could not parse {c} to shape.");
                None
            }
        }
    }
}

impl Score for (Shape, Outcome) {
    fn score(&self) -> u32 {
        let (shape, outcome) = self;
        shape.score() + outcome.score()
    }
}


#[derive(Debug,Clone)]
struct Match(Shape,Shape);

impl Match {
    fn play(&self) -> (Shape, Outcome){
        (
            self.1.clone(),
            match self {
                Match(Shape::Rock, Shape::Rock) => Outcome::Draw,
                Match(Shape::Paper, Shape::Paper) => Outcome::Draw,
                Match(Shape::Scissors, Shape::Scissors) => Outcome::Draw,
                Match(Shape::Rock, Shape::Scissors) => Outcome::Lose,
                Match(Shape::Scissors, Shape::Paper) => Outcome::Lose,
                Match(Shape::Paper, Shape::Rock) => Outcome::Lose,
                _ => Outcome::Win
            }
        )
    }
}

impl Read<&str> for Match {
    // There has to be a better way to do this.
    // TODO: Revise this.
    fn read(string: &str) -> Option<Match> {
        let shapes = string
            .split_ascii_whitespace()
            .map(|item| {
                let chars = item.chars().collect::<Vec<char>>();
                if let [x] = chars[..] {
                    Shape::read(x)
                } else {
                    None
                }
            })
            .collect::<Vec<Option<Shape>>>();

        match &shapes[..] {
            [Some(a), Some(b)] => Some(Match(a.clone(), b.clone())),
            _ => {
                println!("Could not parse {string} to match.");
                None
            }
        }
    }
}


#[derive(Debug,Clone)]
struct Tournament(Vec<Match>);

impl Tournament {
    fn run(&self) -> u32 {
        self.0
            .iter()
            .map(|m| m.play() )
            .map(|i| i.score())
            .sum()
    }
}

impl Read<&str> for Tournament {
    fn read(string: &str) -> Option<Tournament> {
        let matches = string
            .lines()
            .map(|line| Match::read(&line))
            .collect::<Vec<Option<Match>>>();

        if matches.iter().all(|line| line.is_some()) {
            let matches: Vec<Match> = matches
                .iter()
                .map(|m| m.clone().unwrap())
                .collect();
            Some(Tournament(matches))
        } else {
            None
        }
    }
}


fn main() -> io::Result<()> {
    let contents = fs::read_to_string(FILE_PATH)
        .expect("File not found.");

    let tournament = Tournament::read(&contents)
        .expect("Couldn't parse tournament.");

    println!("{:?}", tournament.run());

    Ok(())
}
