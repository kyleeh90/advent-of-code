// Using md5 for hashing
use md5;

// Base of key string
const KEY_BASE: &str = "yzbqklnj";

fn main(){
    // Setup necessary variables
    let mut current_number: u64 = 1;

    let mut hash: String = String::new();

    // Loop until the below criteria is met
    // The input to the MD5 hash is some secret key (your puzzle input, given below) followed by a number in decimal. 
    // To mine AdventCoins, you must find Santa the lowest positive number (no leading zeroes: 1, 2, 3, ...) that produces such a hash.

    // Part two Now find one that starts with six zeroes.
    while !hash.starts_with("000000"){
        // Compute a new hash
        hash = format!("{:x}", md5::compute(format!("{}{}", KEY_BASE, current_number)));

        // Increment number
        current_number += 1;
    }

    println!("Lowest Positive Number That Begins With 6 Zeroes: {}", current_number - 1)
}
