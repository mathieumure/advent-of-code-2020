use std::fs;

fn get_adjacent_seats (x: i32, y: i32, map: &Vec<Vec<char>>) -> Vec<char> {
    let mut seats: Vec<char> = vec![];

    for i in -1..2 {
        if i + x < 0 || x + i >= map.len() as i32 {
            continue;
        }
        let line = &map[(x + i) as usize];
        for j in -1..2 {
            if j + y < 0 || y + j >= line.len()  as i32{
                continue;
            }
            if i == 0 && j == 0 {
                continue
            }
            let char = line[(y+j) as usize];
            if char != '.' {
                seats.push(char)
            }
        }
    }

    seats
}

fn is_outside (x: i32, y: i32, map: &Vec<Vec<char>>) -> bool {
    if x < 0 || x >= map.len() as i32 {
        return true;
    }
    if y < 0 || y >= map[x as usize].len() as i32 {
        return true;
    }
    return false;
}

fn find_direction (map: &Vec<Vec<char>>, start_x: i32, start_y: i32, inc_x: i32, inc_y: i32) -> Option<char> {
    let mut try_x = start_x;
    let mut try_y = start_y;
    return loop {
        try_x = try_x + inc_x;
        try_y = try_y + inc_y;
        if is_outside(try_x, try_y, map) {
            break None;
        }
        if map[try_x as usize][try_y as usize] != '.' {
            break Some(map[try_x as usize][try_y as usize]);
        }
    };
}

fn get_visible_seats (x: i32, y: i32, map: &Vec<Vec<char>>) -> Vec<char> {
    let mut seats: Vec<char> = vec![];

    // O..n = y
    // .
    // n = x

    // TOP
    let val = find_direction(map, x, y, -1, 0);
    if val.is_some() {
        seats.push(val.unwrap())
    }
    // TOP RIGHT
    let val = find_direction(map, x, y, -1, 1);
    if val.is_some() {
        seats.push(val.unwrap());
    }
    // RIGHT
    let val = find_direction(map, x, y, 0, 1);
    if val.is_some() {
        seats.push(val.unwrap());
    }
    // BOTTOM RIGHT
    let val = find_direction(map, x, y, 1, 1);
    if val.is_some() {
        seats.push(val.unwrap());
    }
    // BOTTOM
    let val = find_direction(map, x, y, 1, 0);
    if val.is_some() {
        seats.push(val.unwrap());
    }
    // BOTTOM LEFT
    let val = find_direction(map, x, y, 1, -1);
    if val.is_some() {
        seats.push(val.unwrap());
    }
    // LEFT
    let val = find_direction(map, x, y, 0, -1);
    if val.is_some() {
        seats.push(val.unwrap());
    }
    // TOP_LEFT
    let val = find_direction(map, x, y, -1, -1);
    if val.is_some() {
        seats.push(val.unwrap());
    }

    seats
}

fn compute(map: &Vec<Vec<char>>, find_seat: fn(i32, i32, &Vec<Vec<char>>) -> Vec<char>) -> Vec<Vec<char>> {
    let mut next = map.clone();
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let item = &map[i][j];
            let adjacent_seats = find_seat(i as i32, j as i32, map);
            match item {
                'L' => {
                    let nb_free = adjacent_seats.clone().into_iter().filter(|i| i.clone() =='L').count();
                    if nb_free == adjacent_seats.len() {
                        next[i][j] = '#';
                    }
                },
                '#' => {
                    let nb_occupied = adjacent_seats.clone().into_iter().filter(|i| i.clone() =='#').count();
                    if nb_occupied >= 5 {
                        next[i][j] = 'L';
                    }
                },
                _ => ()
            }
        }
    }
    next
}

fn as_str (input: &Vec<Vec<char>>) -> String {
    input.into_iter().map(|chars| chars.iter().cloned().collect::<String>()).collect::<Vec<String>>().join("\n")
}

fn run (map: &Vec<Vec<char>>, find_seat: fn(i32, i32, &Vec<Vec<char>>) -> Vec<char>) {
    let mut base = map.clone();
    let mut _i = 0;

    loop {
        _i += 1;
        println!("\n### {} ###\n", _i);

        let next = compute(&base, find_seat);
        println!("{}", as_str(&next));

        if as_str(&next) == as_str(&base) {
            base = next;
            break;
        } else {
            base = next
        }
    }

    let mut nb_occupied_seats = 0;
    for i in 0..base.len() {
        let line = &base[i];
        for j in 0..line.len() {
            let char = line[j];
            if char == '#' {
                nb_occupied_seats += 1;
            }
        }
    }

    println!("\n\nResult: {}", nb_occupied_seats);
}

fn main() {
    let file_path = "src/input.txt";
    let lines: Vec<Vec<char>> = fs::read_to_string(file_path).expect("Cannot open file!").lines().map(|l| l.chars().collect()).collect();
    run(&lines, get_visible_seats);
    //let result = get_visible_seats(4, 3, &lines);
    //println!("{:?}", result);
}
