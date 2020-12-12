use std::fs;
use std::collections::{HashSet};

fn part_1 (file: &str) {
    let file_content = fs::read_to_string(file).expect("Something went wrong reading the file");
    let groups: Vec<HashSet<char>> = file_content.split("\n\n").map(| a | {
        let mut answers = HashSet::new();
        a.split("\n").for_each(|line| {
            line.chars().for_each(|char| {
                answers.insert(char);
                return;
            })
        });
        answers
    }).collect();

    let mut sum = 0;
    for group in groups {
        sum += group.len();
    }
    println!("Result: {}", sum);
}

fn part_2 (file: &str) {
    let file_content = fs::read_to_string(file).expect("Something went wrong reading the file");
    let groups: Vec<HashSet<char>> = file_content.split("\n\n").map(| a | {
        let mut answers: HashSet<char> = HashSet::new();
        let mut first = true;
        a.split("\n").for_each(|line| {
            let entries: HashSet<char> = line.chars().collect();
            if first {
                answers = entries;
                first = false;
            } else {
                answers = answers.intersection(&entries).map(|x| *x).collect()
            }
        });
        answers
    }).collect();

    let mut sum = 0;
    for group in &groups {
        sum += group.len();
    }
    println!("Result: {}", sum);
}


fn main() {
    part_1("src/source.txt");
    part_2("src/source.txt");
}
