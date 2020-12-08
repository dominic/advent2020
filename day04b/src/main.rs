use regex::Regex;
use std::io::{self, Read};
use std::collections::HashMap;


fn is_valid(passport: &HashMap<&str, &str>) -> bool {
    let byr_val = passport.get("byr").cloned().unwrap_or("0").parse::<i32>().unwrap();
    let iyr_val = passport.get("iyr").cloned().unwrap_or("0").parse::<i32>().unwrap();
    let eyr_val = passport.get("eyr").cloned().unwrap_or("0").parse::<i32>().unwrap();

    let hgt_val = passport.get("hgt").cloned().unwrap_or("");
    let hgt_regex = Regex::new(r"(\d+)(in|cm)").unwrap();

    let hcl_val = passport.get("hcl").cloned().unwrap_or("");
    let hcl_regex = Regex::new(r"#[0-9a-f]{6}").unwrap();
    
    let ecl_val = passport.get("ecl").cloned().unwrap_or("");
    let ecl_vec = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    let pid_val = passport.get("pid").cloned().unwrap_or("");
    let pid_regex = Regex::new(r"^\d{9}$").unwrap();

    let byr = 1920 <= byr_val && byr_val <= 2002;
    let iyr = 2010 <= iyr_val && iyr_val <= 2020;
    let eyr = 2020 <= eyr_val && eyr_val <= 2030;
    
    let mut hgt = false;
    for cap in hgt_regex.captures_iter(hgt_val) {
        let hgt_num = (&cap[1]).parse::<i32>().unwrap();
        if &cap[2] == "in" {
            hgt = 59 <= hgt_num && hgt_num <= 76;
        } else if &cap[2] == "cm" {
            hgt = 150 <= hgt_num && hgt_num <= 193;
        }
    }
    let hcl = hcl_regex.is_match(hcl_val);
    let ecl = ecl_vec.contains(&ecl_val);
    let pid = pid_regex.is_match(pid_val);


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
