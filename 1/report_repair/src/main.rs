use std::fs;


fn main() {
    let filename = "input.txt";

    let expenses: Vec<i32> = fs::read_to_string(&filename)
                                   .expect("Could not open file. Christmas is canceled.")
                                   .lines()
                                   .map(|line| line.trim().parse().expect("Not a number, Christmas canceled"))
                                   .collect();
                                                

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

