use std::fs;
use std::collections::HashSet;
use regex::Regex;

fn compute (instructions: &Vec<&str>, change_index: i32) -> (bool, i32) {
    let mut executed_instructions: HashSet<i32> = HashSet::new();
    let mut accumulator: i32 = 0;
    let mut pointer: i32 = 0;
    let mut is_success: bool = false;
    let re = Regex::new(r"^(\w+) ([+-])(\d+)$").unwrap();

    loop {

        if executed_instructions.contains(&pointer) {
            is_success = false;
            break;
        }
        executed_instructions.insert(pointer);

        if pointer as usize>= instructions.len() {
            is_success = true;
            break;
        }

        let instruction = instructions[pointer as usize];
        let cap = re.captures(instruction).unwrap();

        let mut code = &cap[1];
        let add = &cap[2] == "+";
        let amount = &cap[3].parse::<i32>().unwrap();

        if change_index == pointer && code == "nop" {
            code = "jmp"
        }
        if change_index == pointer && code == "jmp" {
            code = "nop"
        }

        match code {
            "nop" => {
                pointer += 1;
            }
            "acc" => {
                if add {
                    accumulator += amount;
                } else {
                    accumulator -= amount;
                }
                pointer += 1;
            }
            "jmp" => {
                if add {
                    pointer += amount;
                } else {
                    pointer -= amount;
                }
            }
            _ => {}
        }
    }

    return (is_success, accumulator);
}

fn part_1 (file: &str) {
    let file_content = fs::read_to_string(file).expect("Something went wrong reading the file");
    let lines = file_content.lines();
    let instructions: Vec<&str> = lines.into_iter().collect();
    let (_, result) = compute(&instructions, -1);
    println!("{}", result)
}

fn part_2 (file: &str) {
    let file_content = fs::read_to_string(file).expect("Something went wrong reading the file");
    let lines = file_content.lines();
    let instructions: Vec<&str> = lines.into_iter().collect();
    let mut final_result: i32 = -1;
    for i in 0..instructions.len() {
        let (is_success, result) = compute(&instructions, i as i32);
        println!("Attempting changing {}: {} - {}", i, is_success, result);
        if is_success {
            final_result = result;
            break;
        }
    }
    println!("Result: {}", final_result);
}

fn main() {
    part_1("src/source.txt");
    part_2("src/source.txt");
}
