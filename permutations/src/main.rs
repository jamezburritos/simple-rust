use std::{io, error::Error};

fn permute(word: String, l: usize, r: usize) {
    if l == r { 
        println!("{}", word);
        return; 
    }

    for i in l..r {
        let mut working: Vec<char> = word.chars().collect();

        let tmp = working[l];
        working[l] = working[i];
        working[i] = tmp;

        let working: String = working.iter().collect();
        
        permute(working, l+1, r);
    }
}

// TODO: figure out why this prints "()" at the end of the list

fn main() -> Result<(), Box<dyn Error>> {
    println!("Permutations!");
    println!("Enter a string:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)?;

    let input = input.trim();

    println!("{:?}", permute(String::from(input), 0, input.len()));

    Ok(())
}
