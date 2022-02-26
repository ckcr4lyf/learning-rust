use std::io;

fn wrong_password(){
    println!("Wrong password bruv");
    std::process::exit(-1);
}

fn check_pass(pass: &str) -> bool {

    if pass.len() < 4 {
        return false;
    }

    if pass.chars().nth(0).expect("fucc") != 0x41 as char {
        return false;
    }

    if pass.chars().nth(1).expect("fucc") != 0x42 as char {
        return false;
    }

    if pass.chars().nth(2).expect("fucc") != 0x43 as char {
        return false;
    }

    if pass.chars().nth(3).expect("fucc") != 0x44 as char {
        return false;
    }

    return true;

}

fn main() {

    println!("Enter the password");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to get input");
    let input = input.trim();

    if input.len() % 2 != 0 {
        wrong_password();
    }

    if check_pass(input) == false {
        wrong_password();
    }

    println!("We're in bois");
}
