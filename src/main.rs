use std::io;
use rand::prelude::*;

fn main() {
    println!("| Number Guessing Game |");
    let secret_number = rand::thread_rng().gen_range(1, 10);
    
    loop {
        println!("Please input your guess.");
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("Error while reading line");
        let number:u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("The input must be a number... {:?} :", err);
                continue
            },
        };

        if number == secret_number {
            println!("Congratulations, You Win!");
            break;
        }
        else
        {
            println!("Sorry, Try again!");
        }
    }
}
