#![allow(non_snake_case)]

// I couldn't figure out any other way to import a file in a parent directory...
#[path="../cryptoutil.rs"]
mod cryptoutil;

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::iter;
use std::fmt::Write;

// I replaced all the calls to unwrap() with match statements while trying to debug
// an error, but it turns out it wasn't in this file
fn main() {
    // we're going to be looking for the ciphertext with the highest overall score
    let mut overall_highest_score = 0;
    let mut answer = "".to_string();
    let mut answer_index = 0;
    // this is probably how you open a file in rust
    let file = match File::open("src/set1/4.txt") {
        Ok(v) => v,
        Err(e) => panic!("Failed to open file\n{}", e),
    };
    let ciphertexts = BufReader::new(&file);
    for (i, line) in ciphertexts.lines().enumerate() {
        let hex_ciphertext = match line {
            Ok(v) => v,
            Err(e) => panic!("Invalid line\n{}", e),
        };
        // get the most frequency-occuring byte value in the ciphertext
        let hex_mfb = cryptoutil::hex_top_freq(&hex_ciphertext, 1)[0]; // "most frequent byte"
        let mfb = u8::from_str_radix(&hex_mfb, 16).unwrap();

        // this is a list of possible most-frequent plaintext characters
        let candidates = vec![' ', 'e', 't', 'a', 'o', 'i'];

        // for each candidate char, decrypt the ciphertext under the assumption
        // that that is the most common char in the plaintext.
        let mut highest_score = 0;
        let mut ascii_highest_plaintext = "".to_string();
        for c in &candidates {
            let key: u8 = mfb ^ (*c as u8);
            let mut hex_key = "".to_string();
            write!(&mut hex_key, "{:02x}", key).unwrap();
            let hex_key_buffer: String = iter::repeat(hex_key).take(hex_ciphertext.len()/2).collect();
            let hex_plaintext: String = cryptoutil::hex_to_hex_xor(&hex_ciphertext, &hex_key_buffer);
            let ascii_plaintext: String = cryptoutil::hex_to_ascii(&hex_plaintext);

            // score the resulting plaintext by frequency of common english letters
            let mut score = 0;
            for c in ascii_plaintext.chars() {
                if candidates.contains(&c) { score += 1; }
            }

            if score > highest_score {
                highest_score = score;
                ascii_highest_plaintext = ascii_plaintext;
            }
        }

        // print out the highest score for this ciphertext
        println!("Highest-scoring ({}) plaintext for ciphertext #{} is: {}",
                  highest_score, i+1, ascii_highest_plaintext);

        if highest_score > overall_highest_score {
            overall_highest_score = highest_score;
            answer = ascii_highest_plaintext.clone();
            answer_index = i;
        }
    }

    println!("The message is: {}\nCiphertext #{}, score {}",
             answer, answer_index, overall_highest_score);
}
