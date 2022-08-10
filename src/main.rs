mod guessing_game;

use std::io;// bring the io library into scope
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    rev_test();
    println!("boring guessing game"); // a macro that prints a string to the screen

    // range expression: start..=end, inclusive on the lower and upper bounds
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("{secret_number}");

    loop {
        /*
         * In Rust, variables are immutable by default, mut is for mutable
         * The :: syntax in the ::new line indicates that new is an associated function of the String type, An associated function is a function that’s implemented on a type
         * In full, created a mutable variable that is currently bound to a new, empty instance of a String
         */
        let mut guess = String::new();

        println!("input your number");
        /*
         * & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.
         * Like variables, references are immutable by default.
         */
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read");

        /*
         * Rust allows us to shadow the previous value of guess with a new one.
         * : after guess tells Rust we’ll annotate the variable’s type
         */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(msg) => {
                println!("{msg}");
                continue
            }
        };

        // placeholder {}
        println!("you guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
fn rev_test() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

