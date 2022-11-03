use std::{
    io::{
        self, 
        Error as ioError, 
        ErrorKind::InvalidInput
    }, 
    error::Error,
    f32::consts::PI 
};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Circles!");

    let stdin = io::stdin();

    println!("Enter the radius of the circle, or leave it blank to find it:");

    let mut radius = String::new();
    stdin.read_line(&mut radius)?;

    let radius: f32 = radius.trim().parse().unwrap_or(0.0);

    println!("Enter the circumference of the circle, or leave it blank to find it:");

    let mut circ = String::new();
    stdin.read_line(&mut circ)?;

    let circ: f32 = circ.trim().parse().unwrap_or(0.0);

    println!("Enter the area of the circle, or leave it blank to find it:");

    let mut area = String::new();
    stdin.read_line(&mut area)?;

    let area: f32 = area.trim().parse().unwrap_or(0.0);

    if [radius, area, circ].iter().filter(|x| **x != 0.0).count() == 0 {
        return Err(Box::new(
            ioError::new(InvalidInput, "Need at least one value.")
        ));
    }

    if radius == 0.0 {
        if area != 0.0 {
            println!("The radius of the circle is {:.2}", (area / PI).powf(0.5));
        } else {
            println!("The radius of the circle is {:.2}", circ / (2.0 * PI));
        }
    } 

    if area == 0.0 {
        if radius != 0.0 {
            println!("The area of the circle is {:.2}", PI * radius.powf(2.0));
        } else {
            println!("The area of the circle is {:.2}", circ.powf(2.0) / (4.0 * PI));
        }
    }

    if circ == 0.0 {
        if radius != 0.0 {
            println!("The circumference of the circle is {:.2}", PI * 2.0 * radius);
        } else {
            println!("The circumference of the circle is {:.2}", 2.0 * (PI * area).powf(0.5));
        }
    }

    Ok(())
}
