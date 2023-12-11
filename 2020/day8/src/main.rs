use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let contents: String =
        fs::read_to_string("data/data.txt").expect("Should have been able to read the file");

    let hm = parse_data(contents);
    let mut hs: HashSet<isize> = HashSet::new();
    let acc_value = run(&hm, &mut hs, 0, 0, 0, 0);
    println!("{}", acc_value);
}

fn parse_data(raw: String) -> HashMap<isize, String> {
    raw.lines()
        .enumerate()
        .map(|(num, instruction)| ((num as isize), instruction.to_string()))
        .collect()
}

fn run(
    instructions: &HashMap<isize, String>,
    previous_intstructions: &mut HashSet<isize>,
    mut pointer: isize,
    mut acc: isize,
    mut jmp_counter: usize,
    jmp_limit: usize,
) -> isize {
    previous_intstructions.insert(pointer);
    let mut iter = instructions.get(&pointer).unwrap().split_whitespace();
    let cmd = iter.next().unwrap();
    let val = iter.next().unwrap();
    match cmd {
        "nop" => pointer += 1,
        "acc" => {
            acc += val.parse::<isize>().unwrap();
            pointer += 1
        }
        "jmp" => {
            if jmp_counter == jmp_limit {
                jmp_counter += 1;
                pointer += 1
            } else {
                jmp_counter += 1;
                pointer += val.parse::<isize>().unwrap()
            }
        }
        _ => panic!(),
    };

    if instructions.get(&pointer).is_none() {
        return acc;
    };

    if previous_intstructions.contains(&pointer) {
        run(instructions, &mut HashSet::new(), 0, 0, 0, jmp_limit + 1)
    } else {
        run(
            instructions,
            previous_intstructions,
            pointer,
            acc,
            jmp_counter,
            jmp_limit,
        )
    }
}
