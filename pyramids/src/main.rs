use std::{io, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Pyramid Printer!");
    println!("Enter a number: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)?;

    let input: u8 = input.trim().parse()?;

    println!("\ntop-down pyramid:");

    for i in 1..=input {
        println!("{}", "*".repeat(i as usize));
    }

    println!("\nbottom-up pyramid:");

    for i in 0..input {
        println!("{}", "*".repeat((input - i) as usize));
    }

    println!("\nonly odd numbers:");

    for i in 1..=((input / 2) as f64).ceil() as u32 {
        println!("{}", "*".repeat(((i * 2) - 1) as usize));
    }
    
    Ok(())
}
