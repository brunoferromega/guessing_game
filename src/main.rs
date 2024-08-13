use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Join and enjoy the game!");

    loop {
        println!("Please input a number");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("Your guess was {guess}");

        let guess_as_number = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };

        let result = match guess_as_number.cmp(&secret_number) {
            Ordering::Less => "Too small",
            Ordering::Greater => "Too big",
            Ordering::Equal => "Too you win",
        };

        println!("{result}");

        if secret_number == guess_as_number {
            break;
        }
    }
}
