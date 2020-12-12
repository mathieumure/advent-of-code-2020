use std::fs;
use std::time::{Instant, Duration};

fn parse_input (filename: String) -> Vec<u32> {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let lines = contents.lines();

    return lines.map(|line| line.parse::<u32>().unwrap()).rev().collect();
}

fn do_part () -> Duration {
    let start = Instant::now();
    let mut numbers = parse_input(String::from("exo1.source.txt"));

    numbers.sort();
    let size = numbers.len();

    // Objectif optimiser ce code ;)
    'myLoop: for i in 0..size-1 {
        for j in i..size-1 {
            for k in j..size-1 {
                let x = numbers[i];
                let y = numbers[j];
                let z = numbers[k];

                let sum = x + y + z;
                if sum == 2020 {
                    let product = x * y * z;
                    println!("Result: {}", product);
                    break 'myLoop;
                }
            }
        }
    }
    return start.elapsed();
}

fn main () {
    let mut times: Vec<Duration> = Vec::new();
    let mut sum: Duration = Duration::from_secs(0);

    for i in 0..20 {
        let time = do_part();
        times.push(time);
        sum += time;
    }
    let sum_converted = sum.as_micros();
    println!("Time elapsed in expensive_function() is: {:?} µs", sum_converted as f32 / times.len() as f32);
}