use std::{io, str::FromStr};
use crate::mat::Matrix;

pub fn menu(items: &[&'static str]) -> &'static str {
    println!("Select an operation: ");

    // Print out options as a numbered list.
    for (i, option) in items.iter().enumerate() {
        println!("{}) {option}", i + 1);
    }

    loop {
        let choice: usize = get_int();

        if choice <= 0 || choice > items.len() {
            return items[choice]
        }

        println!("Error: Input must be one of the options given.")
    } 
}

pub fn get_int<T>() -> T
where
    T: FromStr
{
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input).unwrap();

    loop {
        match input.trim().parse() {
            Ok(val) => return val,
            Err(_) => println!("Error: Input must be a number.")
        };
    }
}

pub fn get_mat() -> Matrix {
    println!("Enter the number of rows in the matrix:");
    let rows: usize = get_int();

    println!("Enter the number of columns in the matrix:");
    let cols: usize = get_int();

    let mut mat = Matrix::new(rows, cols);

    println!("Matrix data:");
    for i in 0..rows {
        for j in 0..cols {
            println!("Enter the value at ({}, {}):", i + 1, j + 1);
            mat[(i, j)] = get_int();
        }
    }

    mat
}
