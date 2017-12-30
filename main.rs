extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // print string text
    println!("Guess the number!");

    // Generate a random number between 1-101
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // Print random number
    println!("The secret number is: {}", secret_number);

    loop { // loop the folow instruction
        println!("Please input your guess.");
        // Create new string
        let mut guess = String::new();
        // Read user input
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        // Verify if input user is a number
        /* let guess: u32 = guess.trim().parse()
            .expect("Please type a number!"); */
        // Another option   
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // pass to next instruction
            Err(_) => continue, // return to init loop
        };
        // Print user input
        println!("You guessed: {}", guess);
        // Compare user input to random number
        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"), // If user input is more small than rand
            Ordering::Greater => println!("Too big!"), // If user input is equal to rand
            Ordering::Equal   => {
                println!("You win!");
                break; // id the input number is equal to rand number break loop
                }, // If user input is big than rand
        }
    }
}
