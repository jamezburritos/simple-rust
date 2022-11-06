use std::env;
use std::process;
use std::path::Path;

use std::fs::{
    self, File
};

use std::io::{
    Read, Write,
};

const CHUNK_SIZE: usize = 1024;

fn main() {
    // Get paths from the user as cmdline arguments.
    let args: Vec<String> = env::args().collect();
    let argc = args.len();

    if argc < 3 {
        eprintln!("ERR: Expected at least 2 arguments. Found {}.", argc - 1);
        process::exit(1);
    }   

    let src = Path::new(&args[argc - 2]);
    let dest = Path::new(&args[argc - 1]);

    let mut force = false;

    // If an option is supplied, accept '-f' to overwrite the destination.
    if argc == 4 {
        if args[1] == "-f" {
            force = true;
        } else {
            eprintln!("ERR: Unrecognised option: {}", args[1]);
            process::exit(1);
        }
    }

    // Check if the destination file already exists.
    if dest.exists() && !force {
        eprintln!("ERR: {} already exists. Use -f to overwrite.", 
                  dest.display());

        process::exit(1);
    }

    // Open the source file.
    let Ok(mut src_file) = File::open(src) else {
        eprintln!("ERR: {} does not exist.", 
                  src.display());

        process::exit(1);
    };

    // Create the new file.
    let mut dest_file = match File::create(dest)  {
        Ok(file) => file,
        Err(e) => {
            eprintln!("ERR: Failed to create file {}:\n{}", 
                      dest.display(), e.to_string());

            process::exit(1); 
        }
    };

    // Start reading chunks of the source file, write them to the destination.
    loop {
        let mut buf = [0; CHUNK_SIZE];

        // Read bytes, ensure that they have been read successfully.
        let bytes_read = match src_file.read(&mut buf) {
            Ok(n) => n,
            Err(e) => {
                eprintln!("ERR: Failed to read data from {}:\n{}",
                          src.display(), e.to_string());

                process::exit(1);
            } 
        };

        // Stop if EOF is reached.
        if bytes_read == 0 { break }

        // Write bytes, ensure that they have been written successfully.
        if let Err(e) = dest_file.write(&buf) {
            eprintln!("Failed to write data to {}:\n{}",
                      dest.display(), e.to_string());

            process::exit(1); 
        };
    }

    // Delete the source file when the data has been written.
    if let Err(e) = fs::remove_file(src) {
        eprintln!("Could not remove source file {}:\n{}",
                  src.display(), e.to_string());
    } 
}
