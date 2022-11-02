use rand::{self, Rng};
use std::{io, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Guessing Game!");
        
    let mut rng = rand::thread_rng();
    let solution = rng.gen_range(0..=100);

    let stdin = io::stdin();
    
    loop {
        println!("Enter a number from 0-100:");

        let mut guess = String::new();
        stdin.read_line(&mut guess)?;

        match guess.trim().parse::<i32>() {
            Err(_) => println!("{} is not a valid number. Try again.", guess),
            Ok(val) => {
                if val == solution {
                    println!("Correct!");
                    break;
                }

                if val < solution { println!("Your guess is too low."); }
                if val > solution { println!("Your guess is too high."); }
            }
        }
    }

    Ok(())
}
