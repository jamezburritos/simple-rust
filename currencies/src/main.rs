use std::{io, error::Error};

const USD_GBP: f32 = 0.87;
const GBP_USD: f32 = 1.14;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Convert currencies: ");
    println!("[1] Pound (GBP) -> Dollar (USD)");
    println!("[2] Dollar (USD) -> Pound (GBP)");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)?;

    let choice: u8 = input.trim().parse()?;

    println!("Enter the value to convert: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)?;

    let value: u32 = input.trim().parse()?;

    let conv = match choice {
        1 => value as f32 * GBP_USD,
        2 => value as f32 * USD_GBP,
        _ => panic!("Invalid input. Must be one of ['1', '2']")
    };

    println!("Your converted value is: {}", conv);

    Ok(())
}
