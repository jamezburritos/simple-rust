use std::{
    io::{
        self,
        Error as IoError,
        ErrorKind::InvalidInput
    }, 
    error::Error
};

fn calculator(operation: String) -> Result<f32, Box<dyn Error>> {
    match operation.as_str().find(|ch: char| "+-*/".contains(ch)) {
        Some(index) => {
            let operand: char = operation.chars().nth(index).unwrap(); // can never be None
            let lhs: f32 = operation[0..index].trim().parse()?;
            let rhs: f32 = operation[index + 1..].trim().parse()?;

            match operand {
                '+' => Ok(lhs + rhs),
                '-' => Ok(lhs - rhs),
                '*' => Ok(lhs * rhs),
                '/' => Ok(lhs / rhs),

                _ => panic!() // operands will never not be "+-*/"
            }

        },

        None => Err(Box::new(
            IoError::new(InvalidInput, "Operations must have an operand.")
        ))
    }

}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Calculator!");
    println!("Enter a simple operation: (eg. 25*2)");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)?;
    
    println!("{}", calculator(input)?);

    Ok(())
}

#[test]
fn test_calculator() -> Result<(), Box<dyn Error>> {
    let case0 = calculator(String::from("12 + 10"))?;
    let case1 = calculator(String::from("100 - 50"))?;
    let case2 = calculator(String::from("50 * 1.5"))?;
    let case3 = calculator(String::from("1 / 2"))?;

    assert_eq!(case0, 22.0);
    assert_eq!(case1, 50.0);
    assert_eq!(case2, 75.0);
    assert_eq!(case3, 0.5);

    Ok(())
}
