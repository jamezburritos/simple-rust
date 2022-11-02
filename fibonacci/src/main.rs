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
    println!("Calculate the Fibonacci sequence up to...");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)?;

    let n: u32 = input.trim().parse()?;

    let mut fib = Fibonacci::new(0, 1);

    loop {
        let current = fib.next().unwrap(); // fib will never return None

        if current >= n { break; }
    
        println!("{}", current);
    }

    Ok(())
}
