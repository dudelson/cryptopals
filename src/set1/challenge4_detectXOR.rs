#![allow(non_snake_case)]

mod challenge2_fixedXOR;
mod challenge3_singleByteXOR;

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::iter;
use std::fmt::Write;

// I replaced all the calls to unwrap() with match statements while trying to debug
// an error, but it turns out it wasn't in this file
fn main() {
    // this is probably how you open a file in rust
    let file = match File::open("src/set1/4.txt") {
        Ok(v) => v,
        Err(e) => panic!("Failed to open file\n{}", e),
    };
    let ciphertexts = BufReader::new(&file);
    for (i, line) in ciphertexts.lines().enumerate() {
        let ciphertext = match line {
            Ok(v) => v,
            Err(e) => panic!("Invalid line\n{}", e),
        };
        // associate bytes with their frequencies and sort by frequency
        // this code is copied from challenge 3
        let freq_table = challenge3_singleByteXOR::freq_analysis(&ciphertext);
        let mut sorted: Vec<_> = freq_table.iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(a.1));

        // this is the most frequent byte in the ciphertext
        // it should be interpreted as an encrypted ascii code
        let most_freq_ascii = match u8::from_str_radix(sorted[0].0, 16) {
            Ok(v) => v,
            Err(e) => panic!("Failed to convert byte to ascii\n{:?}", e),
        };

        // this is a list of possible most-frequent plaintext characters
        let candidates = vec![' ', 'e', 't', 'a', 'o', 'i'];

        // for each candidate char, decrypt the ciphertext under the assumption
        // that that is the most common char in the plaintext.
        let mut highest_score = 0;
        let mut highest_plaintext = "".to_string();
        for c in &candidates {
            let key_ascii = most_freq_ascii ^ (*c as u8);
            let mut key_byte = "".to_string();
            write!(&mut key_byte, "{:02x}", key_ascii).unwrap();
            let key_buffer: String = iter::repeat(key_byte).take(ciphertext.len()/2).collect();
            let xor: String = challenge2_fixedXOR::fixed_xor(&ciphertext, &key_buffer);

            let mut plaintext = "".to_string();
            let mut i = 0;
            while i < xor.len() {
                let ascii = u8::from_str_radix(&xor[i..i+2], 16).unwrap();
                plaintext.push(ascii as char);
                i += 2;
            }

            // score the resulting plaintext by frequency of common english letters
            let mut score = 0;
            for p in plaintext.chars() {
                if candidates.contains(&p) { score += 1; }
            }

            if score > highest_score {
                highest_score = score;
                highest_plaintext = plaintext;
            }
        }

        // print out the highest score for this ciphertext
        println!("Highest-scoring ({}) plaintext for ciphertext #{} is: {}",
                  highest_score, i+1, highest_plaintext);
    }
}
