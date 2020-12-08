use std::io::{self, Read};
use std::collections::HashMap;

fn is_valid(passport: &HashMap<&str, &str>) -> bool {
    let byr = passport.contains_key("byr");
    let iyr = passport.contains_key("iyr");
    let eyr = passport.contains_key("eyr");
    let hgt = passport.contains_key("hgt");
    let hcl = passport.contains_key("hcl");
    let ecl = passport.contains_key("ecl");
    let pid = passport.contains_key("pid");

    return byr && iyr && eyr && hgt && hcl && ecl && pid;
}

fn main() {
    let input = get_input();
    let mut valid_count = 0;

    let mut passport = HashMap::new();
    for line in input.split('\n') {
        if line.is_empty() {
            if is_valid(&passport) {
                valid_count += 1;
            }
            passport = HashMap::new();
            continue;
        }

        for part in line.split(' ') {
            let element : Vec<&str> = part.split(':').collect();
            let key = element[0];
            let val = element[1];
            passport.insert(key, val);
        }
    }

    // Check the final passport object for validity
    if is_valid(&passport) {
        valid_count += 1;
    }

    println!("{}", valid_count);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
