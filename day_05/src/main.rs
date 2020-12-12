use std::fs;
use std::collections::HashMap;

fn binary_space (instructions: &str, max: &u32) -> u32 {
    let mut low = 0;
    let mut current = max / 2;
    for code in instructions.chars() {
        if code == 'B' || code == 'R' {
            low += current;
        }
        current = current / 2
    }
    return low;
}

fn get_seat (code: &str) -> (u32, u32, u32) {
    let row_instruction = &code[0..7];
    let column_instruction = &code[7..10];

    let row = binary_space(row_instruction, &128);
    let column = binary_space(column_instruction, &8);
    let id = row * 8 + column;
    return (row, column, id);
}

fn part_1 (filename: &str) -> u32 {
    let file_content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines = file_content.lines();
    let mut max = 0;
    for line in lines {
        let (_, __, id) = get_seat(line);
        if id > max {
            max = id
        }
    }

    max
}

fn part_2 (filename: &str) {
    let file_content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines = file_content.lines();

    let mut seats = HashMap::new();
    for row in 1..127 {
        for column in 0..8 {
            seats.insert((row, column), row * 8 + column);
        }
    }
    for line in lines {
        let (row, column, _) = get_seat(line);
        seats.remove(&(row, column));
    }

    let mut ids: Vec<u32> = seats.into_iter().map(|(_, v)| v).collect();
    ids.sort();

    for i in 1..ids.len()-1 {
        let id = ids[i];
        let lower = ids[i-1];
        let higher = ids[i+1];
        if id - 1 != lower && id+1 != higher {
            println!("Available seat: {:?}", id);
        }
    }
}

fn main() {
    let result = part_1("src/exo5.source.txt");
    println!("Result part 1: {}", result);

    part_2("src/exo5.source.txt");

}
