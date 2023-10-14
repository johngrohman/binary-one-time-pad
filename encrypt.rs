use std::env;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

fn main() {
    // Purpose:    Parses args, reads/writes files, calls apply_key
    // Parameters: None
    // User Input: None
    // Prints:     Nothing
    // Returns:    Nothing
    // Modifies:   Nothing
    // Calls:      std::
    // Tests:      arg_tests/
    // Status:     Do this one.
    let args: Vec<String> = env::args().collect();

    // Obtain all the arguments and store them in variables
    let key_file = &args[1];
    let key_number_string = &args[2];
    let key_number = key_number_string.parse::<u64>().unwrap();
    let input_file = &args[3];
    let output_file = &args[4];

    // Read keyfile as bytes and convert to a vector
    let mut keyfile = File::open(key_file).unwrap();
    let mut keyfile_reader = BufReader::new(keyfile);
    let mut keyfile_contents = Vec::new();
    keyfile_reader.read_to_end(&mut keyfile_contents).unwrap();
    let key_vec = keyfile_contents;

    // Select key from the key_vec
    let mut key = Vec::new();
    let mut i: usize = (key_number * 256).try_into().unwrap();
    while i < ((key_number + 1) * 256).try_into().unwrap() {
        key.push(key_vec[i]);
        i = i + 1;
    }

    // Read the input file and store as a string
    let mut inputfile = File::open(input_file).unwrap();
    let mut inputfile_reader = BufReader::new(inputfile);
    let mut inputfile_contents = String::new();
    inputfile_reader
        .read_to_string(&mut inputfile_contents)
        .unwrap();

    fs::write(output_file, apply_key(&key, &inputfile_contents));
}

pub fn apply_key(key: &Vec<u8>, in_str: &String) -> String {
    // Purpose:    Applies OTP to the in_str based on the key
    // Parameters: A vector of bytes and a unicode string of equal length by chars
    // User Input: None
    // Prints:     Nothing
    // Returns:    A std::String of the same character length as the input string
    // Modifies:   Nothing
    // Calls:      std::
    // Tests:      unit_tests/
    // Status:     Do this one.

    // Convert the input string to a vector of bytes
    let mut in_vec = Vec::new();
    for i in in_str.chars() {
        in_vec.push(i as u8);
    }

    let mut output_vec = Vec::new();

    // Apply key
    for i in 0..in_vec.len() {
        output_vec.push(in_vec[i] ^ key[i]);
    }

    // Convert the output vector to a string
    let mut output_str = String::new();
    for i in output_vec {
        output_str.push(i as char);
    }

    return output_str;
}
