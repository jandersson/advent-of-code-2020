use std::fs;
struct Password {
    policy_min: u8,
    policy_max: u8,
    policy_letter: char,
    text: String,
}
fn is_password_compliant(password: &Password) -> bool {
    let mut policy_counter = 0;
    for c in password.text.chars() {
        if c == password.policy_letter {
            policy_counter += 1;
        }
    }
    if policy_counter > password.policy_max || policy_counter < password.policy_min {
        return false;
    };
    true
}

fn is_password_still_compliant(password: &Password) -> bool {
    (password
        .text
        .chars()
        .nth((password.policy_min - 1).into())
        .unwrap()
        == password.policy_letter)
        ^ (password
            .text
            .chars()
            .nth((password.policy_max - 1).into())
            .unwrap()
            == password.policy_letter)
}

fn main() {
    let filename = "input.txt";

    let mut num_valid_passwords = 0;
    let mut num_still_valid_passwords = 0;

    fs::read_to_string(&filename)
        .expect("Could not open file.")
        .lines()
        .for_each(|line| {
            let data: Vec<&str> = line
                .split(&['-', ':', ' '][..])
                .filter(|&i| i != "")
                .collect();
            let password = Password {
                policy_min: data[0].parse().expect("Not a number"),
                policy_max: data[1].parse().expect("Not a number"),
                policy_letter: data[2].parse().expect("Not a character"),
                text: data[3].to_string(),
            };
            if is_password_compliant(&password) {
                num_valid_passwords += 1;
            };
            if is_password_still_compliant(&password) {
                num_still_valid_passwords += 1;
            }
        });

    println!("Number of valid passwords: {}", num_valid_passwords);
    println!(
        "Number of passwords that are still valid: {}",
        num_still_valid_passwords
    );
}
