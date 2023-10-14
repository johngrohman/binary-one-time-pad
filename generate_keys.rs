use rand::Rng;
use std::fs;

pub fn generate_keys(num_keys: usize) -> Vec<u8> {
    // Purpose:    Gets a random vector of bytes, with each key being 2048 bits
    // Parameters: The number of keys
    // User Input: None
    // Prints:     Nothing
    // Returns:    A Vec<u8> containing the bytes of num_keys keys
    // Modifies:   Nothing
    // Calls:      std:: , rand::
    // Tests:      unit_tests/
    // Status:     Do this one.
    let mut i = 0;
    let mut vec = Vec::new();
    while i < (256 * num_keys) {
        vec.push(rand::random::<u8>());
        i = i + 1;
    }
    return vec;
}

fn main() {
    // Purpose:    Parses the args for this file, and writes the keyfile
    // Parameters: None
    // User Input: None
    // Prints:     Nothing
    // Returns:    Nothing
    // Modifies:   Nothing
    // Calls:      std::
    // Tests:      custom_test (see grade.sh)
    // Status:     Do this one.
    let file: String = std::env::args().collect();
    fs::write(file, generate_keys(50));
}
