use std::fs;

fn is_valid (preamble: &Vec<usize>, value: usize) -> bool {
    for x in 0..preamble.len() {
        let value_x = preamble[x];
        if value_x >= value {
            break;
        }
        for y in (x+1)..preamble.len() {
            let value_y = preamble[y];
            if value_y >= value {
                break;
            }
            // println!("{} + {} = {} // {}", value_x, value_y, value_x + value_y, value);
            if (value_x + value_y) == value {
                return true;
            }
        }
    }
    return false;
}

fn part_1 (lines: &Vec<usize>, preamble_length: usize) -> usize {
    for i in preamble_length..lines.len() {
        let value: usize = lines[i];
        let mut preamble = lines[(i - preamble_length)..i].to_vec();
        preamble.sort();
        let valid = is_valid(&preamble, value);

        if !valid {
           return value;
        }
    }

    return 0;
}

fn part_2 (lines: &Vec<usize>, target: usize) -> (usize, usize) {
    for start in 0..lines.len() {
        let value: usize = lines[start];

        let mut sum = value;
        for end in (start+1)..lines.len() {
            sum += lines[end];
            println!("{} {} {}", start, end, sum);
            if sum > target {
                break;
            }
            if sum == target {
                return (start, end);
            }
        }
    }

    return (0, 0);
}

fn main() {
    let filepath = "src/source.txt";
    const PREAMBLE_LENGTH: usize = 25;
    let file = fs::read_to_string(filepath).expect(&["Cannot open file", filepath].join(""));
    let lines: Vec<usize> = file.lines().map(|it| it.parse::<usize>().unwrap()).collect();

    let result = part_1(&lines, PREAMBLE_LENGTH);
    println!("Result part 1 {}", result);
    let (start, end) = part_2(&lines, result);
    let min = lines[start..end].iter().min().unwrap();
    let max = lines[start..end].iter().max().unwrap();
    println!("Result part 2 {}", min + max);
}
