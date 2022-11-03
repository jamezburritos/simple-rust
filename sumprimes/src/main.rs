use std::io;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn sieve_primes(n: u32) -> Vec<u32> {
    let mut sieve: Vec<bool> = vec![true; n as usize];

    // exclude non-prime numbers
    sieve[0] = false;
    sieve[1] = false;

    // find prime numbers
    for i in 2..=(n as f32).sqrt() as usize {
        if sieve[i] {
            for j in (i.pow(2)..n as usize).step_by(i) {
                sieve[j] = false;
            }
        }
    }

    // collect prime numbers
    let mut primes: Vec<u32> = Vec::new();
    for i in 0..n {
        if sieve[i as usize] { primes.push(i); }
    }
    
    primes
}

fn find_sums(target: u32, nums: Vec<u32>) -> Vec<(u32, u32)> {
    let mut sums = Vec::new();

    let mut head = 0;
    let mut tail = nums.len() - 1;

    while head < tail {
        let sum = nums[head] + nums[tail];

        if sum == target { 
            sums.push((nums[head], nums[tail]));
            tail -= 1;
            head += 1;
            continue;
        } 

        if sum > target { 
            tail -= 1 
        } else { 
            head += 1 
        }
    }

    sums 
}

fn main() -> Result<()> {
    println!("Sum of Primes!");
    println!("Enter a number:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)?;

    let input: u32 = input.trim().parse()?;
    
    let primes = sieve_primes(input);
    let sums   = find_sums(input, primes);

    println!("The prime numbers which add up to {input} are:");
    for sum in sums {
        println!("{} + {}", sum.0, sum.1);
    }

    Ok(())
}
