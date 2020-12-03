use std::fs;

fn count_tree_hits(mut map: Vec<Vec<char>>, x_rule: usize, y_rule: usize) -> usize {
    let mut num_trees = 0;
    let map_length = map[0].len();

    if y_rule > 1 {
        // This feels non-idiomatic
        for i in 0..map.len() {
            if i % y_rule != 0 {
                map.remove();
            }
        }
    }
    for (i, line) in map.iter_mut().enumerate() {
        line.rotate_left((i * x_rule) % map_length);
        if line[0] == '#' {
            num_trees += 1;
        }
    }
    num_trees
}

fn main() {
    let map: Vec<Vec<char>> = fs::read_to_string("input.txt")
        .expect("Couldn't open the file!")
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    println!("Number of trees: {}", count_tree_hits(map.clone(), 3, 1));
    let tree_product = count_tree_hits(map.clone(), 1, 1)
        * count_tree_hits(map.clone(), 3, 1)
        * count_tree_hits(map.clone(), 5, 1)
        * count_tree_hits(map.clone(), 7, 1)
        * count_tree_hits(map.clone(), 1, 2);
    println!("{}", tree_product);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pulka() {
        let test_map: Vec<Vec<char>> = fs::read_to_string(
            "/home/jonas/dev/advent-of-code-2020/3/toboggan_trajectory/src/test.txt",
        )
        .expect("Couldn't open the file!")
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();
        assert_eq!(count_tree_hits(test_map.clone(), 3, 1), 7);
        assert_eq!(count_tree_hits(test_map.clone(), 1, 1), 2);
        assert_eq!(count_tree_hits(test_map.clone(), 5, 1), 3);
        assert_eq!(count_tree_hits(test_map.clone(), 7, 1), 4);
        assert_eq!(count_tree_hits(test_map.clone(), 1, 2), 2);
    }
}
