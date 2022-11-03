use std::{
    io,
    error::Error,
    num::ParseIntError
};

fn reverse_numerals(num: i32) -> Result<i32, ParseIntError> {
    num
        .to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse()
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Reverse Numerals!");
    println!("Enter a number:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)?;

    let input: i32 = input.trim().parse()?;
    
    println!("{}", reverse_numerals(input)?);

    Ok(())
}
