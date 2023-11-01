use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number! You have 5 lives!");
    println!("Please input your guess.");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut lives = 5;

    while lives > 0 {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You must enter an integer, try again.");
                subtract_life(&mut lives);
                continue;
            },
        };
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small.");
                subtract_life(&mut lives);
            },
            Ordering::Greater => {
                println!("Too big.");
                subtract_life(&mut lives);
            },
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
    if lives < 1 { println!("Game over! You lose!") }
}

fn subtract_life(lives: &mut i32) {
    *lives -=1;
    println!("You have {} lives left!", lives);
}
