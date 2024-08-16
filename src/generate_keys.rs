use rand::Rng;
use std::env;
use std::fs;

pub fn generate_keys(num_keys: usize) -> Vec<u8> {
    // Purpose:    Gets a random vector of bytes, with each key being 2048 bits
    // Parameters: The number of keys
    // User Input: t
    // Prints:     Nothing
    // Returns:    A Vec<u8> containing the bytes of num_keys keys
    // Modifies:   Nothing
    // Calls:      std:: , rand::
    // Tests:      unit_tests/
    // Status:     Done

    let mut rng = rand::thread_rng();
    let mut keys: Vec<u8> = Vec::new();
    let mut iter: usize = 0;

    while iter < num_keys {
        for _i in 0..256 {
            let new_rand: u8 = rng.gen();
            keys.push(new_rand);
        }

        iter = iter + 1;
    }

    return keys;
}

fn main() {
    // Purpose:    Parses the args for this file, and writes the keyfile
    // Parameters: None
    // User Input: None
    // Prints:     Nothing
    // Returns:    Nothing
    // Modifies:   Nothing
    // Calls:      std::
    // Tests:      None
    // Status:     Done
    let args: Vec<String> = env::args().collect();

    let path: &str = &args[1];

    let contents: Vec<u8> = generate_keys(50);

    if let Err(e) = fs::write(path, contents) {
        eprintln!("Fail! {}", e);
    }
}
