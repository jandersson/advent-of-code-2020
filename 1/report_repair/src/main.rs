use std::fs;


fn main() {
    let filename = "input.txt";

    let mut expenses: Vec<i32> = Vec::new();

    let contents = fs::read_to_string(&filename).expect("Couldn't read the file, Christmas is canceled");

    for expense in contents.lines() {
        let expense: i32 = expense.trim().parse().expect("Bad input, not a number. Christmas canceled.");
        expenses.push(expense);
    }

    for expense in expenses.iter() {
        let remainder = 2020 - expense;
        if expenses.iter().rfind( |&&x| x == remainder) != None { 
            println!("two element product: {}", remainder * expense);
            break;
        };
    }

    for x in expenses.iter() {
        for y in expenses.iter() {
            for z in expenses.iter() {
                if x + y + z == 2020 {
                    println!("{} * {} * {} = {}", x, y, z, x*y*z);
                    return;
                }
            }
        }
    }

}

