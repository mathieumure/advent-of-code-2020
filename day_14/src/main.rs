use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::str::Lines;

fn part_1(lines: Lines) {
    let mut memory = HashMap::new();
    let reg = Regex::new(r"^mem\[(\d+)\] = (\d+)").unwrap();

    let mut or_mask = 0;
    let mut and_mask: u64 = 2u64.pow(36) - 1;

    for line in lines {
        if line.starts_with("mask = ") {
            or_mask = 0;
            and_mask = 2u64.pow(36) - 1;
            let sub = &line[7..];
            for i in 0..sub.len() {
                let bit = sub.chars().nth(sub.len() - 1 - i).unwrap();
                match bit {
                    '1' => or_mask += 2u64.pow(i as u32),
                    '0' => and_mask -= 2u64.pow(i as u32),
                    _ => (),
                }
            }
        }
        if line.starts_with("mem") {
            let groups = reg.captures(line).unwrap();
            let value = groups[2].parse::<u64>().unwrap();
            let mem_index = groups[1].parse::<usize>().unwrap();
            let masked_value = value & and_mask | or_mask;
            memory.insert(mem_index, masked_value);
        }
    }

    let sum = memory.iter().fold(0, |acc, (_, mem_value)| acc + mem_value);

    println!("Part 1 : {}", sum);
}

fn part_2(lines: Lines) {
    let mut memory = HashMap::new();
    let reg = Regex::new(r"^mem\[(\d+)\] = (\d+)").unwrap();
    let mut mask: String = String::new();

    for line in lines {
        if line.starts_with("mask = ") {
            let sub = &line[7..];
            mask = String::from(sub);
        }
        if line.starts_with("mem") {
            let groups = reg.captures(line).unwrap();
            let value = groups[2].parse::<u64>().unwrap();
            let mem_index = groups[1].parse::<u64>().unwrap();
            let mem_index_as_bin = format!("{:0width$b}", mem_index, width = mask.len());
            let mem_index_with_mask: String = mask
                .clone()
                .chars()
                .enumerate()
                .map(|(i, c)| match c {
                    '0' => mem_index_as_bin.chars().nth(i).unwrap(),
                    _ => c,
                })
                .collect();

            let nb_floating = mem_index_with_mask.chars().filter(|x| x == &'X').count();
            let nb_addr = 2usize.pow(nb_floating as u32);
            for i in 0..nb_addr {
                let as_bin = format!("{:0width$b}", i, width = nb_floating);
                let mut new_addr = String::from(&mem_index_with_mask);
                for j in 0..as_bin.len() {
                    let new_bit = as_bin.chars().nth(j).unwrap().to_string();
                    new_addr = new_addr.replacen('X', new_bit.as_str(), 1);
                }
                let new_addr_dec = u64::from_str_radix(&new_addr, 2).unwrap();
                //println!("new address {} (decimal {})", new_mask, new_mask_dec);
                memory.insert(new_addr_dec, value);
            }
        }
    }

    let sum = memory.iter().fold(0, |acc, (_, mem_value)| acc + mem_value);

    println!("Part 2: {}", sum);
}

fn main() {
    let path = "src/input.txt";
    let file_content = fs::read_to_string(path).unwrap();
    let lines = file_content.lines();
    part_1(lines);
    let lines = file_content.lines();
    part_2(lines);
}
