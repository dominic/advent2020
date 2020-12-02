use regex::Regex;
use std::io::{self, Read};

fn main() {
    let raw_input = get_input();

    let mut valid_count = 0;
    let mut passwords: Vec<(i32, i32, char, String)> = Vec::new();
    let re = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
    for line in raw_input.split('\n') {
        for cap in re.captures_iter(line) {
            passwords.push((
                (&cap[1]).parse().expect("Not a number"),
                (&cap[2]).parse().expect("Not a number"),
                (&cap[3]).chars().next().unwrap(),
                (&cap[4]).to_string(),
            ));
        }
    }

    for password_tuple in passwords {
        let pos1 = password_tuple.0 as usize;
        let pos2 = password_tuple.1 as usize;
        let check = password_tuple.2;
        let password = password_tuple.3;

        let first = password.chars().nth(pos1 - 1).unwrap_or('-');
        let second = password.chars().nth(pos2 - 1).unwrap_or('-');

        if (first == check || second == check) && first != second {
            valid_count += 1;
        }
    }

    println!("{}", valid_count);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to open file");

    return input.trim().to_string();
}
