use std::fs;
use regex::Regex;
use std::collections::HashMap;

fn valid_part_1 (passport: &HashMap<String,String>) -> bool {
        if !passport.contains_key("byr") {
            return false;
        }
        if !passport.contains_key("iyr") {
            return false;
        }
        if !passport.contains_key("eyr") {
            return false;
        }
        if !passport.contains_key("hgt") {
            return false;
        }
        if !passport.contains_key("hcl") {
            return false;
        }
        if !passport.contains_key("ecl") {
            return false;
        }
        if !passport.contains_key("pid"){
            return false;
        }
    return true;
}

fn is_valid_date (value: Option<&String>, min: u32, max: u32) -> bool {
    let date = value.unwrap().parse::<u32>().unwrap();
    return date >= min && date <= max;
}

fn valid_part_2 (passport: &HashMap<String,String>) -> bool {
    if !passport.contains_key("byr") || !is_valid_date(passport.get("byr"), 1920, 2002) {
        return false;
    }

    if !passport.contains_key("iyr") || !is_valid_date(passport.get("iyr"), 2010, 2020) {
        return false;
    }

    if !passport.contains_key("eyr") || !is_valid_date(passport.get("eyr"), 2020, 2030) {
        return false;
    }

    if !passport.contains_key("hgt") {
        return false;
    } else {
        let hgt = passport.get("hgt").unwrap();
        let re = Regex::new(r"(\d+)(in|cm)").unwrap();
        if !re.is_match(hgt) {
            return false;
        }
        let res = re.captures(hgt).unwrap();
        let amount = res[1].parse::<u32>().unwrap();
        if &res[2] == "cm" && (amount < 150 || amount > 193) {
            return false;
        }
        if &res[2] == "in" && (amount < 59 || amount > 76) {
            return false;
        }
    }

    if !passport.contains_key("hcl") {
        return false;
    } else {
        let hcl = passport.get("hcl").unwrap();
        let re = Regex::new(r"^#[0-9A-Fa-f]{6}$").unwrap();
        if !re.is_match(hcl) {
            return false;
        }
    }

    if !passport.contains_key("ecl") {
        return false;
    } else {
        let ecl = passport.get("ecl").unwrap();
        let re = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
        if !re.is_match(ecl) {
            return false;
        }
    }
    if !passport.contains_key("pid"){
        return false;
    } else {
        let pid = passport.get("pid").unwrap();
        let re = Regex::new(r"^\d{9}$").unwrap();
        if !re.is_match(pid) {
            return false;
        }
    }

    return true;
}

fn run (filename: String, validate: fn(&HashMap<String,String>) -> bool) -> i32 {
    let file_content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines = file_content.lines();
    let re = Regex::new(r"(\w+):([#\w]+)").unwrap();

    let mut pass = HashMap::new();
    let mut nb_valid = 0;
    for line in lines {
        if line.is_empty() {
            if validate(&pass) {
                nb_valid += 1;
            }
            pass.clear();
            continue;
        }

        for cap in re.captures_iter(line) {
            pass.insert(String::from(&cap[1]), String::from(&cap[2]));
        }
    }

    return nb_valid;
}


fn main() {
    // let result = run(String::from("src/exo4.source.txt"), valid_part_1);
    let result = run(String::from("src/exo4.source.txt"), valid_part_2);
    println!("{}", result);
}
