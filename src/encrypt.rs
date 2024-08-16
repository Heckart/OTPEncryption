use std::env;
use std::fs;

fn main() {
    // Purpose:    Parses args, reads/writes files, calls apply_key
    // Parameters: None
    // User Input: None
    // Prints:     Nothing
    // Returns:    Nothing
    // Modifies:   Nothing
    // Calls:      std::
    // Tests:      None
    // Status:     Done

    let args: Vec<String> = env::args().collect();

    let key_file_to_read: &str = &args[1];
    let key_number_str: &str = &args[2];
    let key_number: isize = key_number_str.parse().expect("NaN");
    let input_utf_8_file: &str = &args[3];
    let output_utf_8_file: &str = &args[4];

    let key_bytes = fs::read(key_file_to_read).unwrap();

    let input_string = fs::read_to_string(input_utf_8_file).unwrap();

    //find the proper key. each key should be 2048 bits aka occupy 256 locations in the vector
    let key_start: usize = (key_number * 256).try_into().unwrap();
    let key_end: usize = (key_start + 255).try_into().unwrap();

    let sliced_key_bytes = key_bytes[key_start..key_end + 1].to_vec();

    if let Err(e) = fs::write(
        output_utf_8_file,
        apply_key(&sliced_key_bytes, &input_string),
    ) {
        eprintln!("Fail! {}", e);
    }
}

pub fn apply_key(key: &Vec<u8>, in_str: &String) -> String {
    // Purpose:    Applies OTP to the in_str based on the key
    // Parameters: A vector of bytes and a unicode string of equal length by chars
    // User Input: None
    // Prints:     Nothing
    // Returns:    A std::String of the same character length as the input string
    // Modifies:   Nothing
    // Calls:      std::
    // Tests:       unit_tests/
    // Status:     Done

    let mut result = String::new();

    for (i, char) in in_str.chars().enumerate() {
        let char_byte = char as u8;

        let key_byte = key[i % key.len()];

        let cipher_byte = char_byte ^ key_byte;

        let cipher_char = cipher_byte as char;
        result.push(cipher_char);
    }

    result
}
