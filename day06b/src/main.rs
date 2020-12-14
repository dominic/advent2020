use std::io::{self, Read};

fn main() {
    let input = get_input();
    let mut total_sum = 0;

    let lines : Vec<&str> = input.split('\n').collect();

    let mut group_set : [i32; 26] = [0; 26];
    let mut group_count = 0;

    for i in 0..lines.len() {
        if lines[i].is_empty() {
            // adsfsdf
            let group_total = group_set.iter().filter(|&n| *n == group_count).count();
            total_sum += group_total;

            group_set = [0; 26];
            group_count = 0;
            continue;
        }

        group_count += 1;
        for char in lines[i].chars() {
            let index = char.to_digit(36).unwrap() as usize - 10;
            group_set[index] = group_set[index] + 1;
        }
    }

    println!("{}", total_sum);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
