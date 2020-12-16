use std::fs;

fn part_1(time_arrival: &u32, bus_ids: &Vec<u32>) -> u32 {
    let times: Vec<(&u32, u32)> = bus_ids
        .into_iter()
        .map(|bus_id| (bus_id, bus_id - (time_arrival % bus_id)))
        .collect();

    let mut min: (&u32, u32) = (&0, u32::max_value());
    for time in times {
        if time.1 < min.1 {
            min = time;
        }
    }

    min.0 * min.1
}

fn part_2(second_line: &str) -> u64 {
    let mut t: u64 = 0;
    let mut step = 1;

    let raw_ids: Vec<&str> = second_line.split(",").collect();

    let mut bus_ids = vec![];
    for i in 0..raw_ids.len() {
        let id = raw_ids[i];
        if id != "x" {
            bus_ids.push((id.parse::<u64>().unwrap(), i as u64));
        }
    }

    for (bus_id, mins) in bus_ids {
        while (t + mins) % bus_id != 0 {
            t += step;
        }
        step *= bus_id;
    }

    t
}

fn main() {
    let path = "src/input.txt";

    let file_content = fs::read_to_string(path).unwrap();
    let first_line = file_content.lines().nth(0).unwrap();
    let second_line = file_content.lines().nth(1).unwrap();
    let time_arrival = first_line.parse::<u32>().unwrap();
    let bus_ids: Vec<u32> = second_line
        .split(",")
        .filter(|it| it != &"x")
        .map(|it| it.parse::<u32>().unwrap())
        .collect();

    let result_part1 = part_1(&time_arrival, &bus_ids);
    println!("Result part 1: {}", result_part1);

    let result_part2 = part_2(&second_line);
    println!("Result part 2: {}", result_part2);
}
