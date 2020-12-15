use std::fs;

const DIRECTIONS: [char; 4] = ['E', 'S', 'W', 'N'];

fn part_1(commands: &Vec<(char, i32)>) -> i32 {
    let mut direction_idx: i32 = 0;
    let mut north_coord = 0;
    let mut east_coord = 0;

    let mut update_indices = |action: char, coord: i32| {
        match action {
            'N' => north_coord += coord, //means to move north by the given value.
            'S' => north_coord -= coord, //means to move south by the given value.
            'E' => east_coord += coord,  //means to move east by the given value.
            'W' => east_coord -= coord,  //means to move west by the given value.
            _ => (),
        }
    };

    for command in commands {
        let direction = DIRECTIONS[direction_idx as usize % DIRECTIONS.len()];
        match command.0 {
            'N' | 'S' | 'E' | 'W' => update_indices(command.0, command.1),
            'F' => update_indices(direction, command.1), //means to move forward by the given value in the direction the ship is currently facing.
            'R' => {
                let offset = command.1 / 90;
                direction_idx += offset
            } //means to turn right the given number of degrees.
            'L' => {
                let offset = command.1 / 90;
                direction_idx -= offset
            } //means to turn left the given number of degrees.
            _ => (),
        }
    }

    // println!("{:?} {:?}", north_coord, east_coord)
    return north_coord.abs() + east_coord.abs();
}

fn rotate_clockwise(x: i32, y: i32) -> (i32, i32) {
    (y, -x)
}

fn rotate_anti_clockwise(x: i32, y: i32) -> (i32, i32) {
    (-y, x)
}

fn part_2(commands: &Vec<(char, i32)>) -> i32 {
    let mut north_coord = 0;
    let mut east_coord = 0;
    let mut north_waypoint = 1;
    let mut east_waypoint = 10;

    for command in commands {
        match command.0 {
            'N' => north_waypoint += command.1, //means to move north by the given value.
            'S' => north_waypoint -= command.1, //means to move south by the given value.
            'E' => east_waypoint += command.1,  //means to move east by the given value.
            'W' => east_waypoint -= command.1,  //means to move west by the given value.
            'R' => {
                let offset = command.1 / 90;
                for _ in 0..offset {
                    let rotated = rotate_clockwise(east_waypoint, north_waypoint);
                    east_waypoint = rotated.0;
                    north_waypoint = rotated.1;
                }
            } //means to turn right the given number of degrees.
            'L' => {
                let offset = command.1 / 90;
                for _ in 0..offset {
                    let rotated = rotate_anti_clockwise(east_waypoint, north_waypoint);
                    east_waypoint = rotated.0;
                    north_waypoint = rotated.1;
                }
            } //means to turn left the given number of degrees.
            'F' => {
                north_coord += north_waypoint * command.1;
                east_coord += east_waypoint * command.1;
            } //means to move forward by the given value in the direction the ship is currently facing.
            _ => (),
        }
    }

    // println!("{:?} {:?}", north_coord, east_coord)
    return north_coord.abs() + east_coord.abs();
}

fn main() {
    let path = "src/input.txt";
    let lines: Vec<(char, i32)> = fs::read_to_string(path)
        .expect("Cannot open file!")
        .lines()
        .map(|it| {
            let as_str = String::from(it);
            return (
                as_str.chars().nth(0).unwrap(),
                as_str.get(1..).unwrap().parse::<i32>().unwrap(),
            );
        })
        .collect();

    // let result = part_1(&lines);
    let result = part_2(&lines);

    println!("Result: {:?}", result);
}
