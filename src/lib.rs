use rand::Rng;
use std::error::Error;

pub fn generate_key_matrix(size: u8) -> Result<Vec<Vec<i8>>, Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let mut matrix = Vec::new();

    for _ in 0..size {
        let mut row = Vec::new();
        for _ in 0..size {
            let value = rng.gen_range(1..=26);
            row.push(value);
        }
        matrix.push(row);
    }

    Ok(matrix)
}


pub fn encrypt_message(
    message: &String,
    key_matrix: &Vec<Vec<i8>>,
) -> Result<String, Box<dyn Error>> {
    // Convert the message to numerical representation
    let message_chars: Vec<char> = message.chars().collect();
    let message_numerical: Vec<i8> = message_chars
        .iter()
        .map(|c| (c.to_ascii_uppercase() as i8 - 'A' as i8) % 26)
        .collect();

    // Encrypt each block of the message
    let mut encrypted = Vec::new();
    for block in message_numerical.chunks(key_matrix.len()) {
        let mut encrypted_block = vec![0; key_matrix.len()];
        for (i, &ref key_row) in key_matrix.iter().enumerate() {
            for (j, &key_val) in key_row.iter().enumerate() {
                // Perform arithmetic in i32 to avoid overflow, then reduce modulo 26
                encrypted_block[i] = (encrypted_block[i] + (key_val as i32 * block[j] as i32)) % 26;
            }
        }
        encrypted.extend(encrypted_block);
    }

    // Convert the encrypted numerical representation back to letters
    let encrypted_message: String = encrypted
        .iter()
        .map(|&num| ((num + 'A' as i32) as u8) as char)
        .collect();

    Ok(encrypted_message)
}
