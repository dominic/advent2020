use regex::Regex;
use std::io::{self, Read};

fn main() {
    let input = get_input();
    let mut max_valid = -1;

    let seat_regex = Regex::new(r"([FB]{7})([RL]{3})").unwrap();
    for line in input.split('\n') {
        let captures = seat_regex.captures(line).unwrap();

        // Convert row from binary string to int
        let row_binary = &captures[1].chars().collect::<String>()
            .replace("F", "0")
            .replace("B", "1");

        let row_num = isize::from_str_radix(row_binary, 2).unwrap();

        // Convert seat from binary string to int
        let seat_binary = &captures[2].chars().collect::<String>()
            .replace("L", "0")
            .replace("R", "1");
        let seat_num = isize::from_str_radix(seat_binary, 2).unwrap();

        let seat_id = row_num * 8 + seat_num;
        if seat_id > max_valid {
            max_valid = seat_id
        }
    }

    println!("{}", max_valid);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
