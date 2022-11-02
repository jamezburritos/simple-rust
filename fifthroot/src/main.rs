fn main() {
    let sum: f64 = (0..100)
        .map(|x| (2.0 * (x as f64)) + 1.0)
        .map(|x| x * x)
        .sum();

    println!(
        "The fifth root of the sum of the first 100 odd numbers squared is: {:.5}", 
        f64::powf(sum, 1.0 / 5.0)
    )
}
