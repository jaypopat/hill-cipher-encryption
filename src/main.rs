use hill_cipher;
use std::io;

fn main() {
    println!("Choose an option:");
    println!("1. Generate a random key matrix");
    println!("2. Enter your own key matrix");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let choice: u8 = choice.trim().parse().expect("Not a number");

    println!("A key matrix will be in the form nxn\n Enter the value of n:");
    let mut max_key = String::new();
    io::stdin()
        .read_line(&mut max_key)
        .expect("error reading input");
    let size: usize = max_key.trim().parse().expect("not a number");

    let mut key_matrix: Vec<Vec<i8>> = Vec::new();

    match choice {
        1 => {
            key_matrix = hill_cipher::generate_key_matrix(size as u8)
                .expect("could not create randomly generated matrix");
            println!("Your key matrix is: {:?}", key_matrix);
        }
        2 => {
            println!(
                "Enter the key matrix (rows separated by '/', each row separated by a space):"
            );
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            key_matrix = input
                .split('/')
                .map(|row| {
                    row.split_whitespace()
                        .map(|num| num.parse::<i8>().expect("Failed to parse number"))
                        .collect()
                })
                .collect();
        }
        _ => println!("Invalid choice. Please enter 1 or 2."),
    }

    let mut message = String::new();
    println!(
        "Enter message of length {} to be encrypted using the key matrix",
        size
    );
    io::stdin()
        .read_line(&mut message)
        .expect("error reading input");
    message = message.trim().to_string(); // Remove newline character

    if message.len() != size {
        panic!("message length needs to be same length as key matrix");
    }
    match hill_cipher::encrypt_message(&message, &key_matrix) {
        Ok(encrypted_message) => {
            println!("Encrypted message: {}", encrypted_message);
        }
        Err(err) => {
            println!("Error encrypting message: {}", err);
        }
    }
}
