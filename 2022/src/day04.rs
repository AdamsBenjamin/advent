use std::fs;


const FILE_PATH: &str = "./inputs/day04";


trait Between<T> {
    fn between(&self, left: &T, right: &T) -> bool;
}

trait Contains<T> {
    fn contains(&self, other: &T) -> bool;
}

trait Overlap<T> {
    fn overlaps(&self, other: &T) -> bool;
}

impl<T: Ord> Between<T> for T {
    fn between(&self, left: &T, right: &T)  -> bool {
        left <= self && self <= right
    }
}

type Range = (usize, usize);

impl Contains<Range> for Range {
    fn contains(&self, other: &Range) -> bool {
        let (left1, right1) = self;
        let (left2, right2) = other;

        left2.between(left1, right1) && right2.between(left1, right1)
    }
}

impl Overlap<Range> for Range {
    fn overlaps(&self, other: &Range) -> bool {
        let (left1, right1) = self;
        let (left2, right2) = other;

        left2.between(left1, right1) || right2.between(left1, right1)
            || left1.between(left2, right2) || right1.between(left2, right2)
    }
}

fn parse_line(line: &str) -> (Range, Range) {
    let sections = line
        .split(",")
        .map(|range| range.split("-").collect::<Vec<&str>>())
        .flatten()
        .map(|num| num.parse().unwrap())
        .collect::<Vec<usize>>();

    if let [i1, j1, i2, j2] = sections[..] {
        ((i1, j1), (i2, j2))
    } else {
        panic!("Bad input file format.")
    }
}

fn main() {
    let contents = fs::read_to_string(FILE_PATH)
        .expect("Could not file input file.");
    let lines = contents
        .lines()
        .map(parse_line);
    let overlaps = lines
        .filter(|(r1, r2)| r1.overlaps(r2))
        .count();

    println!("{:?}", overlaps);
}
