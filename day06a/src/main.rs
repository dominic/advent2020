use std::io::{self, Read};
use std::collections::HashSet;

fn main() {
    let input = get_input();
    let mut group_sum = 0;

    let lines : Vec<&str> = input.split('\n').collect();
    let mut group_set = HashSet::new();
    for i in 0..lines.len() {
        if lines[i].is_empty() {
            group_sum += group_set.len();
            group_set = HashSet::new();
            continue;
        }
        for char in lines[i].chars() {
            group_set.insert(char);
        }
    }

    println!("{}", group_sum)
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
