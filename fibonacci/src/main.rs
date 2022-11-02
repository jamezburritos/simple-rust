use std::{io, error::Error};

struct Fibonacci {
    current: u32,
    next: u32
}

impl Fibonacci {
    fn new(current: u32, next: u32) -> Fibonacci {
        Fibonacci { current, next }
    }
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current = self.next;
        self.next += current;

        Some(self.current)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Fibonacci Generator!");
    println!("Calculate the first _ numbers of the Fibonacci sequence:");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)?;

    let n: usize = input.trim().parse()?;

    for i in Fibonacci::new(0, 1).take(n) {
        println!("{}", i);
    }

    Ok(())
}
