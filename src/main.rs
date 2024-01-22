use utils::get_number;
mod n_queens;
mod utils;

fn main() {
    loop {
        print_menu();

        let option = get_number();

        match option {
            1 => {
                fibonacci_recursive();
            }
            2 => fibonacci_iterative(),
            3 => cashier_problem(),
            4 => collatz_chain(),
            5 => n_queens::run_n_queens(),
            0 => {
                println!("Exiting ...");
                break;
            }
            _ => {
                println!("Enter a correct option")
            }
        }
    }
}

fn collatz_chain() {
    println!("The Collatz conjecture is one of the most famous unsolved problems in mathematics.");
    println!("The conjecture asks whether repeating two simple arithmetic operations will eventually transform every positive integer into 1.");
    println!("It concerns sequences of integers in which each term is obtained from the previous term as follows:");
    println!("If the previous term is even, the next term is one half of the previous term.");
    println!("If the previous term is odd, the next term is 3 times the previous term plus 1.");
    println!("The conjecture is that these sequences always reach 1, no matter which positive integer is chosen to start the sequence.");

    let number = utils::get_number();
    collatz(number);
    println!("Finished collatz chain!");
}

fn collatz(number: usize) {
    println!("Number : {}", number);
    if number == 1 || number == 0 {
        return;
    } else if number % 2 == 0 {
        collatz(number / 2);
    } else {
        collatz((number * 3) + 1)
    }
}

fn cashier_problem() {
    println!("Cashier problem!");
    let money: [f32; 13] = [
        500.0, 200.0, 100.0, 50.0, 20.0, 10.0, 5.0, 2.0, 1.0, 0.5, 0.2, 0.1, 0.01,
    ]; // IMPORTANT TO ORDER IN FROM BIGGER TO SMALLER (learned that the hard way :D)

    let costumer_money = utils::get_float();

    give_change(&money, costumer_money, 0);
}

fn give_change(money: &[f32], mut costumer_money: f32, index: usize) {
    if index < money.len() {
        if costumer_money - money[index] >= 0.0 {
            costumer_money -= money[index];
            println!("Givving change of: {}", money[index]);
            give_change(money, costumer_money, index)
        } else {
            give_change(money, costumer_money, index + 1)
        }
    }
}

fn fibonacci_iterative() {
    println!("Fibonacci Iterative");

    let number = utils::get_number();

    match number {
        0 => println!("Fibonacci sequence of {number} is 0"),
        1 => println!("Fibonacci sequence of {number} is 1"),
        _ => {
            let mut a: u64 = 0;
            let mut b: u64 = 1;
            let mut c: u64 = 0;

            for _ in 1..number {
                c = a + b;
                a = b;
                b = c;
            }
            println!("Fibonacci sequence of {number} is {c}");
        }
    }
}

fn print_menu() {
    println!("1. Fibonacci Recursive");
    println!("2. Fibonacci Iterative");
    println!("3. Cashier problem");
    println!("4. Collatz chain");
    println!("5. N Queens backtracking");
    println!("0. Exit.");
    println!("Enter an option: ");
}

fn fibonacci_recursive() {
    println!("Welcome to Fibonacci Recursive");
    let number = utils::get_number();
    println!("The fibonacci of {} is {}", number, fibonacci_rec(number));
}

fn fibonacci_rec(n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    return fibonacci_rec(n - 1) + fibonacci_rec(n - 2);
}
