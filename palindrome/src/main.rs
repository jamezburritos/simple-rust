use std::io;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn check_palindrome(input: String) -> bool {
    let filtered: String = input.chars()
        .filter(|ch| ch.is_alphabetic())
        .map(|ch| ch.to_ascii_lowercase())
        .collect();

    let reversed: String = filtered.chars()
        .rev()
        .collect();

    filtered == reversed
}

fn main() -> Result<()> {
    println!("Palindromes!");
    println!("Enter a string to check:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)?;

    let input = input.trim();

    let is_pal = check_palindrome(String::from(input));

    println!(
        "The string '{}' {} a palindrome.", 
        input, 
        if is_pal { "is" } else { "is not" }
    );
    
    Ok(()) 
}

#[test]
fn test_palindromes() {
    assert!(check_palindrome(String::from("racecar")));
    assert!(!check_palindrome(String::from("quick brown fox")));
}
