use std::fs;
use regex::Regex;
use std::fmt;
use std::time::{Instant};

struct Entry {
    min: u32,
    max: u32,
    letter: char,
    password: String,
    valid: bool
}

impl fmt::Debug for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Entry")
            .field("min", &self.min)
            .field("max", &self.max)
            .field("letter", &self.letter)
            .field("password", &self.password)
            .field("valid", &self.valid)
            .finish()
    }
}

fn parse_input (filename: String) -> Vec<Entry> {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let lines = contents.lines();

    // 1-3 a: abcde
    return lines.map(|line| {
        let reg = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
        let cap = reg.captures(line).unwrap();
        let mut entry = Entry {
            max: *&cap[2].parse::<u32>().unwrap(),
            min: *&cap[1].parse::<u32>().unwrap(),
            letter: (&cap[3]).parse().unwrap(),
            password: String::from(&cap[4]),
            valid: false
        };

        let mut amount = 0;
        for char in entry.password.chars() {
            if char == entry.letter {
                amount += 1;
            }
        }
        entry.valid = amount >= entry.min && amount <= entry.max;

        return entry
    }).filter(|entry| entry.valid)
    .rev().collect();
}


fn parse_input_2 (filename: String) -> Vec<Entry> {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let lines = contents.lines();

    // 1-3 a: abcde
    return lines.map(|line| {
        let reg = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
        let cap = reg.captures(line).unwrap();
        let mut entry = Entry {
            min: *&cap[1].parse::<u32>().unwrap(),
            max: *&cap[2].parse::<u32>().unwrap(),
            letter: (&cap[3]).parse().unwrap(),
            password: String::from(&cap[4]),
            valid: false
        };

        let first = entry.password.chars().nth((entry.min - 1) as usize).unwrap() == entry.letter;
        let last = entry.password.chars().nth((entry.max - 1) as usize).unwrap() == entry.letter;
        entry.valid = first ^ last;

        return entry;
    }).filter(|entry| entry.valid)
    .rev().collect();
}

fn main() {
    let start = Instant::now();
    // let results = parse_input(String::from("src/exo2.test.txt"));
    // let results = parse_input(String::from("src/exo2.source.txt"));
    let results = parse_input_2(String::from("src/exo2.source.txt"));

    let duration = start.elapsed();
    println!("Result: {}", results.len());
    println!("Time elapsed in expensive_function() is: {:?} µs", duration);
}
