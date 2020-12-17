use std::collections::HashMap;
use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now();

    // let nb_iterations = 2020;
    // let input = "0,3,6";
    // let input = "1,3,2";
    // let input = "2,1,3";
    // let input = "1,2,3";
    // let input = "2,3,1";
    // let input = "3,2,1";
    // let input = "3,1,2";
    let nb_iterations = 30000000;
    let input = "0,1,5,10,3,12,19";

    let stack: Vec<usize> = input
        .split(",")
        .map(|it| it.parse::<usize>().unwrap())
        .collect();

    let mut history: HashMap<usize, usize> = HashMap::new();
    let mut turn = 1;

    for value in 0..stack.len() - 1 {
        history.insert(stack[value], turn);
        turn += 1;
    }

    let mut prev: usize = 0;
    let mut speak = stack.last().unwrap().clone();

    for _ in turn..nb_iterations {
        prev = speak;

        let t = history.get(&prev);
        if t.is_some() && t.unwrap() != &0 {
            speak = turn - t.unwrap()
        } else {
            speak = 0;
        }

        history.insert(prev, turn);
        turn += 1
    }

    println!("{:?}", speak);
    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
