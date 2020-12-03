use std::fs;

fn run (filename: String, right_incr: usize, down_incr: usize) -> u32 {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split('\n').collect();
    let max_height = lines.len();
    let max_size = lines[0].len();

    let mut count = 0;
    let mut right = right_incr;

    let mut top = down_incr;
    loop {
        let char = lines[top].chars().nth(right).unwrap();

        if char == '#' {
            count += 1;
        }
        right = (right + right_incr) % max_size;
        top = top + down_incr;
        if top >= max_height { break; }
    }

    count
}

fn main () {
    let a = run(String::from("src/exo3.source.txt"), 1, 1);
    let b = run(String::from("src/exo3.source.txt"), 3, 1);
    let c = run(String::from("src/exo3.source.txt"), 5, 1);
    let d = run(String::from("src/exo3.source.txt"), 7, 1);
    let e = run(String::from("src/exo3.source.txt"), 1, 2);
    println!("count: {:?}", a * b * c * d * e);

}