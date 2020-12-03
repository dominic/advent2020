use std::io::{self, Read};

fn main() {
    let input = get_input();
    let x_delta = 3;
    let y_delta = 1;

    let mut tree_count = 0;
    let mut x = 0;
    let mut y = 0;

    let lines: Vec<&str> = input.split('\n').collect();
    let line_length = lines[0].chars().count();

    while y < lines.len() {
        let current_line: Vec<char> = lines[y].chars().collect();
        if current_line[x] == '#' {
            tree_count += 1
        }

        y += y_delta;
        x = (x + x_delta) % line_length;
    }

    println!("{}", tree_count);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
