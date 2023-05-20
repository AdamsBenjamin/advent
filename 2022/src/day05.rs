use std::fs;

const FILE_PATH: &str = "./inputs/day05";


struct Instruction {
    quantity: usize,
    from: usize,
    to: usize,
}


type Crates<T> = Vec<Vec<T>>;

fn parse_crates(stacks: &[&str]) -> Crates<char> {
    let mut crates: Vec<Vec<char>> = Vec::new();
    let mut stacks_iter = stacks.iter().rev();
    stacks_iter.next(); // ignore the stack numbers

    for line in stacks_iter {
        let chars_vec: Vec<_> = line.chars().collect();
        for (i, entry) in chars_vec.chunks(4).enumerate() {
            let letter: Vec<_> = entry
                .iter()
                .filter(|c| c.is_alphabetic())
                .collect();

            if let [c] = letter[..] {
                if let Some(stack) = crates.get_mut(i) {
                    stack.push(c.clone());
                } else {
                    crates.push(vec![c.clone()]);
                }
            }
        }
    }

    crates
}

fn parse_instructions(raw: &[&str]) -> Vec<Instruction> {
    let mut instructions = vec![];

    for line in raw {
        if let ["move", quantity, "from", from, "to", to] = line.split_whitespace().collect::<Vec<_>>()[..] {
            let [quantity, from, to] = [quantity, from, to]
                .map(|n| n.parse().unwrap());
            instructions.push(Instruction{
                quantity,
                from: from - 1,  // The input uses a 1-based system
                to: to - 1       // but this solution uses a 0-based system
            });
        }
    }

    instructions
}

fn parse_input<'a>(
    lines: &Vec<&'a str>
) -> (Crates<char>, Vec<Instruction>) {
    let parts: Vec<_> = lines.split(|line| line.is_empty()).collect();
    if let [crates, instructions] = parts[..] {
        (parse_crates(&crates), parse_instructions(instructions))
    } else {
        panic!("Incorrect input format.")
    }
}


fn move_crates<T>(instruction: &Instruction, crates: &mut Crates<char>) {
    let Instruction{quantity, from, to} = instruction;
    let start_height = crates[*from].len();

    let mut popped: Vec<_> = crates[*from]
        .drain(start_height - quantity..)
        .collect();

    crates[*to].append(&mut popped);
}


fn main() {
    let contents = fs::read_to_string(FILE_PATH)
        .expect("Could not read file at {FILE_PATH}.");

    let lines: Vec<_> = contents.lines().collect();
    let (mut crates, instructions) = parse_input(&lines);
    for instruction in instructions {
        move_crates::<char>(&instruction, &mut crates)
    }

    let answer: Vec<char> = crates
        .iter()
        .map(|s| s.last().unwrap().clone())
        .collect();

    println!("{answer:?}");
}
