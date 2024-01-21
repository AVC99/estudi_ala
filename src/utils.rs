use std::io;
use std::io::Write;

pub fn get_number() -> usize {
    let mut input = String::new();
    print!("Enter a number: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read a line");
    return input.trim().parse().expect("Please type a number");
}

// should put this in a loop
pub fn get_float() -> f32 {
    println!("Enter a float value");
    let mut input = String::new();
    print!("Enter a number: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read a line");
    return input.trim().parse().expect("Please type a number");
}
