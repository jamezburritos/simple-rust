use std::io;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn get_coin(value: f32) -> Result<f32> {
    println!("How many {}c coins are there?", value * 100.0);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)?;

    match input.trim().parse::<f32>() {
        Ok(count) => Ok(count * value),
        Err(e) => Err(Box::new(e))
    }
}

fn main() -> Result<()> {
    println!("Coin Counting!");

    let total: f32 = [0.05, 0.1, 0.25, 0.5]
        .iter()
        .map(|x| get_coin(*x).unwrap())
        .sum();

    println!("The total value of the coins entered is ${}", total);

    Ok(())
}
