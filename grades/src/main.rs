use std::io;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn calculate_avg(scores: Vec<f32>, weights: Vec<f32>) -> f32 {
    Iterator::zip(scores.iter(), weights.iter())
        .map(|(score, weight)| score * weight / 100.0)
        .sum()
}

fn main() -> Result<()> {
    println!("Weighted Grades!"); 

    let mut scores: Vec<f32> = Vec::new();
    let mut weights: Vec<f32> = Vec::new(); 
    
    loop {
        println!("Enter the score (%) and the weight (%): ");
        
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)?;

            
        let input: Vec<f32> = input.trim()
            .split_whitespace()
            .map(|val| 
                 val.parse::<f32>()
                 .expect("Malformed input. Values must be valid numbers.")
            )
            .collect();

        assert_eq!(input.len(), 2, 
                   "Malformed input. Enter two percentages, separated by a space.");

        scores.push(input[0]);
        weights.push(input[1]);

        let sum_weights: f32 = weights.iter().sum();

        if sum_weights == 100.0 { break }

        assert!(sum_weights < 100.0, 
                "Sum of weights may not exceed 100%.");
    } 

    let avg = calculate_avg(scores.clone(), weights.clone());
    println!("The weighted average of your scores is {}%", avg);

    Ok(()) 
}
