use std::{io, error::Error};

fn reverse(input: String, l: usize, r: usize) -> String {
    if l >= r { return input }

    let mut working: Vec<char> = input.chars().collect();
    working.swap(l, r);
    let working: String = working.iter().collect();

    reverse(working, l + 1, r - 1)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Reverse String!");
    println!("Enter a string to reverse:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)?;

    let input = input.trim();

    println!("{}", reverse(String::from(input), 0, input.len() - 1));

    Ok(())
}
