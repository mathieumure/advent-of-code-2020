use std::fs;

fn part1(nums: &Vec<u64>) -> u64 {
    let mut diff1_count = 0;
    let mut diff3_count = 0;

    for window in nums.windows(2) {
        match window[1] - window[0] {
            1 => diff1_count += 1,
            3 => diff3_count += 1,
            _ => (),
        }
    }

    diff1_count * diff3_count
}

fn part2(nums: &Vec<u64>) -> u64 {
    let mut slices = vec![];
    let mut current_slice = vec![];

    for window in nums.windows(2) {
        match window[1] - window[0] {
            1 => current_slice.push(window[0]),
            3 => {
                current_slice.push(window[0]);
                slices.push(current_slice);
                current_slice = vec![];
            }
            _ => (),
        }
    }

    slices
        .iter()
        .map(|slice| match slice.len() {
            1 => 1,
            2 => 1,
            3 => 2,
            4 => 4,
            5 => 7,
            _ => panic!("unexpected slice of size N > 5 consecutive 1-diff elements"),
        })
        .product()
}

fn main() {
    const FILE_PATH: &str = "src/source.txt";
    let file_content = fs::read_to_string(FILE_PATH).expect(&["Cannot open ", FILE_PATH].join(""));
    let mut lines: Vec<u64> = file_content.lines().map(|it| it.parse::<u64>().unwrap()).collect();
    lines.sort();

    lines.insert(0, 0);
    lines.push(lines.last().unwrap() + 3);

    let part1 = part1(&lines);
    println!("Part 1: {}", part1);

    let part2 = part2(&lines);
    println!("Part 2: {}", part2);
}