use std::fs;

fn main() {
    let filename = "input.txt";

    let mut expenses: Vec<i32> = fs::read_to_string(&filename)
        .expect("Could not open file. Christmas is canceled.")
        .lines()
        .map(|line| {
            line.trim()
                .parse()
                .expect("Not a number, Christmas canceled")
        })
        .collect();

    for expense in expenses.iter() {
        let remainder = 2020 - expense;
        if expenses.iter().rfind(|&&x| x == remainder) != None {
            println!("two element product: {}", remainder * expense);
            break;
        };
    }

    // Its a subset sum problem!
    // Check wikipedia for 3SUM
    expenses.sort();
    for i in 0..expenses.len() - 2 {
        let a = expenses[i];
        let mut start = i + 1;
        let mut end = expenses.len() - 1;
        while start < end {
            let b = expenses[start];
            let c = expenses[end];
            if a + b + c == 2020 {
                println!("{} + {} + {} = {}", a, b, c, a + b + c);
                println!("{} * {} * {} = {}", a, b, c, a * b * c);
                return;
            } else if a + b + c > 2020 {
                end -= 1;
            } else {
                start += 1;
            }
        }
    }
}
