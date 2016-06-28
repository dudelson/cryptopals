#[path="../cryptoutil.rs"]
mod cryptoutil;

use std::fs::File;
use std::io::{Read, BufRead, BufReader};
use std::ops::Range;
use std::cmp::Ordering::Less;
use std::iter;
use std::fmt::Write;

/// the range of key lengths to try when attempting decryption
const KEYSIZE_RANGE: Range<usize> = 2..40;

/// computes the hamming distance between two strings
/// that is, this function returns the number of differing bits between
/// the two strings
fn hamming_distance(s: &str, t: &str) -> u32 {
    assert_eq!(s.len(), t.len());
    // keep a running tally of the number of ones in s ^ t
    let mut n_ones = 0;
    let s_bytes = s.as_bytes();
    let t_bytes = t.as_bytes();
    for i in 0..s.len() {
        n_ones += (s_bytes[i] ^ t_bytes[i]).count_ones();
    }
    n_ones
}

fn main() {
    // read the base64-encoded ciphertext out of the file and into a buffer w/o newlines
    let file = File::open("src/set1/6.txt").expect("Failed to open input file");
    let filebuf = BufReader::new(&file);
    let b64_ciphertext = filebuf.lines().fold(String::new(),
                                              |s, x| s + &x.expect("Error concatenating ciphertext"));

    // convert base64 to hex
    let hex_ciphertext = cryptoutil::base64_to_hex(&b64_ciphertext);

    // get the normalized hamming distance between the first two blocks for each keysize
    let mut normalized_hds: Vec<(usize, f64)> = vec![];
    for key_size in KEYSIZE_RANGE {
        // a block is key_size bytes, or key_size*2 hex digits
        let block1: String = hex_ciphertext.clone().chars().take(key_size*2).collect();
        let block2: String = hex_ciphertext.clone().chars().skip(key_size*2).take(key_size*2).collect();
        let block_hd = hamming_distance(&block1, &block2);
        normalized_hds.push((key_size, (block_hd as f64) / (key_size as f64)));
    }

    // sort the normalized hamming distances non-decreasing
    normalized_hds.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Less));

    // let's assume that the correct keysize is in the top 3
    let mut highest_overall_score = 0;
    let mut highest_ascii_plaintext = String::new();
    let mut highest_ascii_key = String::new();
    for &(key_size, normalized_hd) in normalized_hds.iter().take(3) {
        // Break the ciphertext into blocks of key_size length using our friend
        // the gnarly iterator.
        // But first, pad the ciphertext so we get uniform blocks.
        // This makes the code a lot easier to write and does not affect the
        // scoring to the best of my knowledge.
        let size = key_size*2;
        let len = &hex_ciphertext.len();
        let hex_ciphertext_padded = hex_ciphertext.clone() +
                                    &String::from_utf8(vec![b'0'; size - (len % size)]).unwrap();
        let pad_len = hex_ciphertext_padded.len();
        let blocks = (0..).take(pad_len).filter(|&x| x % size == 0)
                          .map(|i| &hex_ciphertext_padded[i..i+size]);
        // transpose the blocks
        // there's probably some mind-blowingly elegant way to do this with iterator
        // adapters, but I have not yet attained that level of functional programming
        // nirvana :(
        let mut transposed_blocks = vec![String::new(); key_size];
        for block in blocks {
            // split into groups of 2 (one byte each) using the gnarly iterator
            let bytes = (0..size).filter(|&x| x % 2 == 0).map(|i| &block[i..i+2]);
            for (i, byte) in bytes.enumerate() {
                transposed_blocks[i].push_str(&byte);
            }
        }

        // solve each block as single-character XOR

        // this is a list of possible most-frequent plaintext characters
        let candidates = vec![' ', 'e', 't', 'a', 'o', 'i'];

        let mut hex_final_key = String::new();
        for tb in transposed_blocks {
            // this code is straight out of challenge 1-4
            let hex_mfb = cryptoutil::hex_top_freq(&tb, 1)[0]; // "most frequent byte"
            let mfb = u8::from_str_radix(&hex_mfb, 16).unwrap();

            // for each candidate char, decrypt the ciphertext under the assumption
            // that that is the most common char in the plaintext.
            let mut highest_score = 0;
            let mut hex_highest_key = String::new();
            let mut ascii_highest_plaintext = String::new();
            for c in &candidates {
                let key: u8 = mfb ^ (*c as u8);
                let mut hex_key = String::new();
                write!(&mut hex_key, "{:02x}", key).unwrap();
                let hex_key_buffer: String = hex_key.chars().cycle().take(tb.len()).collect();
                let hex_plaintext: String = cryptoutil::hex_to_hex_xor(&tb, &hex_key_buffer);
                let ascii_plaintext: String = cryptoutil::hex_to_ascii(&hex_plaintext);

                // score the resulting plaintext by frequency of common english letters
                let mut score = 0;
                for c in ascii_plaintext.chars() {
                    if candidates.contains(&c) { score += 1; }
                }

                if score > highest_score {
                    highest_score = score;
                    hex_highest_key = hex_key;
                    ascii_highest_plaintext = ascii_plaintext;
                }
            }
            // keep in mind that because these are the transposed blocks that
            // are being scored, the key will be a single char, and the plaintext
            // will most likely not make sense (but it should contain all readable
            // english stuff)
            println!("\n\nThis block: keylen={}, score={}, key={}, plaintext:\n{}",
                     key_size, highest_score, hex_highest_key, ascii_highest_plaintext);

            // append the most likely key char to the "final key"
            hex_final_key.push_str(&hex_highest_key);
        }

        // decrypt the full message with our derived key!
        // this code is more or less straight out of challenge 1-5
        let hex_key_buffer: String = hex_final_key.chars().cycle().take(hex_ciphertext.len()).collect();
        let hex_plaintext = cryptoutil::hex_to_hex_xor(&hex_key_buffer, &hex_ciphertext);

        let ascii_key = cryptoutil::hex_to_ascii(&hex_final_key);
        let ascii_plaintext = cryptoutil::hex_to_ascii(&hex_plaintext);

        // score this plaintext
        // the plaintext with the highest score among all the key lengths is printed
        // out as the final answer
        let ascii_top_six = cryptoutil::ascii_top_freq(&ascii_plaintext, 6);
        let mut score = 0;
        for c in &candidates {
            if ascii_top_six.contains(c) { score += 1; }
        }
        println!("\n\nTop six: {}", ascii_top_six.into_iter().collect::<String>());
        println!("plaintext decrypted with key of length {} was scored {}\n\n", key_size, score);
        if score > highest_overall_score {
            highest_overall_score = score;
            highest_ascii_key = ascii_key;
            highest_ascii_plaintext = ascii_plaintext;
        }
    }

    println!("Key: {}\nMessage: {}", highest_ascii_key, highest_ascii_plaintext);
}
