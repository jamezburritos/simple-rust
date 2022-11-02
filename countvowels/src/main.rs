use std::{io, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Counting Vowels!");
    println!("Enter a string:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)?;

    let mut vowels = 0;
    let mut cons   = 0;

    for ch in input.chars() {
        if !ch.is_alphabetic() { break; }

        if "aeiou".contains(ch) { 
            vowels += 1; 
        } else { 
            cons += 1; 
        }
    }

    println!("There are {} vowels and {} consonants in the string '{}'.", vowels, cons, input.trim());

    Ok(())
}
