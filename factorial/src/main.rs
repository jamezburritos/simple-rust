use std::{io, error::Error};

fn factorial(x: u32, acc: u32) -> u32 {
    if x <= 0 { return acc }

    return factorial(x - 1, acc * x);
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Enter a number: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)?;

    let input = input.trim();

    let result = factorial(input.parse()?, 1);

    println!("The factorial of {} is {}", input, result);

    Ok(())
}
