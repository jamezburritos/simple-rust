use matrices::io::*;

fn main() {
    println!("Matrix Calculator!");

    let choices = [
        "-A",
        "A + B",
        "A - B",
        "A * B",
        "x * A"
    ];

    match menu(&choices) {
        "-A" => {
            println!("Matrix A:");
            let a = get_mat();

            println!("{}", -a)
        },

        "A + B" => {
            println!("Matrix A:");
            let a = get_mat();
            
            println!("Matrix B:");
            let b = get_mat();

            println!("{}", a + b);
        },

        "A - B" => {
            println!("Matrix A:");
            let a = get_mat();
            
            println!("Matrix B:");
            let b = get_mat();

            println!("{}", a - b);
        },

        "A * B" => {
            println!("Matrix A:");
            let a = get_mat();
            
            println!("Matrix B:");
            let b = get_mat();

            println!("{}", a * b);
        },
        "x * A" => {
            println!("Enter the value of x:");
            let x: f32 = get_int();
            
            println!("Matrix A:");
            let a = get_mat();

            println!("{}", a * x);
        },

        _ => panic!() // This will never happen.
    }
}



