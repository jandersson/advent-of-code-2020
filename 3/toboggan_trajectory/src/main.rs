use std::fs;

fn main() {
    let mut map: Vec<Vec<char>> = fs::read_to_string("input.txt")
        .expect("Couldn't open the file!")
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    let mut num_trees = 0;
    for (i, line) in map.iter_mut().enumerate() {
        let line_len = line.len();
        line.rotate_left((i * 3) % line_len);
        if line[0] == '#' {
            num_trees += 1;
        }
    }
    println!("Number of trees: {}", num_trees);
}
