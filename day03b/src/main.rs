use std::io::{self, Read};

fn main() {
    let slopes: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut results: [usize; 5] = [0; 5];

    let input = get_input();
    let lines: Vec<&str> = input.split('\n').collect();
    let line_length = lines[0].chars().count();

    for (i, slope) in slopes.iter().enumerate() {
        let mut tree_count = 0;
        let mut x = 0;
        let mut y = 0;

        while y < lines.len() {
            let current_line: Vec<char> = lines[y].chars().collect();
            if current_line[x] == '#' {
                tree_count += 1
            }
            y += slope.1;
            x = (x + slope.0) % line_length;
        }
        results[i] = tree_count;
        println!("{}", tree_count);
    }

    println!("{}", results.iter().fold(1, |a, b| a * b))
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
