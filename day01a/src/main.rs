use std::io::{self, Read};

fn main() {
    let raw_input = get_input();

    let mut numbers: Vec<i32> = Vec::new();
    for line in raw_input.split_whitespace() {
        numbers.push(line.parse().unwrap_or(0));
    }

    let numbers_len = numbers.len();
    for i in 0..numbers_len {
        for j in 0..numbers_len {
            if i == j {
                continue;
            }
            let first = numbers[i];
            let second = numbers[j];
            if first + second == 2020 {
                println!("{}", first * second);
                return;
            }
        }
    }
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to open file");

    return input.trim().to_string();
}
