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

impl Read<char> for Shape {
    fn read(c: char) -> Option<Shape> {
        match c {
            'A' => Some(Shape::Rock),
            'B' => Some(Shape::Paper),
            'C' => Some(Shape::Scissors),
            _   => {
                println!("Could not parse {c} to shape.");
                None
            }
        }
    }
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

impl Read<char> for Outcome {
    fn read(c: char) -> Option<Outcome> {
        match c {
            'X' => Some(Outcome::Lose),
            'Y' => Some(Outcome::Draw),
            'Z' => Some(Outcome::Win),
            _ => {
                println!("Could not parse {c} to outcome.");
                None
            }
        }
    }
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

impl Score for (Outcome, Shape) {
    fn score(&self) -> u32 {
        let (outcome, shape) = self;
        shape.score() + outcome.score()
    }
}

#[derive(Debug,Clone)]
struct Plan(Shape,Outcome);

impl Plan {
    fn determine_hand(&self) -> Shape {
        match &self {
            Plan(Shape::Rock, Outcome::Win) => Shape::Paper,
            Plan(Shape::Rock, Outcome::Draw) => Shape::Rock,
            Plan(Shape::Rock, Outcome::Lose) => Shape::Scissors,
            Plan(Shape::Paper, Outcome::Win) => Shape::Scissors,
            Plan(Shape::Paper, Outcome::Draw) => Shape::Paper,
            Plan(Shape::Paper, Outcome::Lose) => Shape::Rock,
            Plan(Shape::Scissors, Outcome::Win) => Shape::Rock,
            Plan(Shape::Scissors, Outcome::Draw) => Shape::Scissors,
            Plan(Shape::Scissors, Outcome::Lose) => Shape::Paper
        }
    }
}

impl Read<&str> for Plan {
    fn read(string: &str) -> Option<Plan> {
        if let [shape, _, outcome] = string
            .chars()
            .collect::<Vec<char>>()[..] {
            if let (Some(shape), Some(outcome)) = (Shape::read(shape), Outcome::read(outcome)) {
                Some(Plan(shape, outcome))
            } else {
                None
            }
        } else {
            None
        }
    }
}


#[derive(Debug,Clone)]
struct Tournament(Vec<Plan>);

impl Tournament {
    fn run(&self) -> u32 {
        let Tournament(plans) = self;
        plans
            .iter()
            .map(|p| (p.clone().1, p.determine_hand()) )
            .map(|i| i.score() )
            .sum()
    }
}

impl Read<&str> for Tournament {
    fn read(string: &str) -> Option<Tournament> {
        let matches = string
            .lines()
            .map(|line| Plan::read(&line));

        if matches.clone().all(|line| line.is_some()) {
            let matches: Vec<Plan> = matches
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
