#[path="../cryptoutil.rs"]
mod cryptoutil;

use std::fs::File;
use std::io::{Read, BufRead, BufReader};
use std::ops::Range;
use std::cmp::Ordering::Less;
use std::iter;
use std::fmt::Write;

/// the range of key lengths to try when attempting decryption
const ASCII_KEYSIZE_RANGE: Range<usize> = 2..40;
const LETTER_FREQS: Vec<(u8, f64)> = vec![
    ('a', 8.16), ('b', 1.49), ('c', 2.78), ('d', 4.25), ('e', 12.7), ('f', 2.22),
    ('g', 2.02), ('h', 6.09), ('i', 6.97), ('j', 0.15), ('k', 0.77), ('l', 4.03),
    ('m', 2.41), ('n', 6.75), ('o', 7.51), ('p', 1.93), ('q', 0.10), ('r', 5.99),
    ('s', 6.44), ('t', 9.01), ('u', 2.76), ('v', 0.98), ('w', 2.36), ('x', 0.15),
    ('y', 1.97), ('z', 0.07)
]

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
    for ascii_key_size in ASCII_KEYSIZE_RANGE {
        // a block is key_size bytes, or key_size*2 hex digits
        let hex_key_size = ascii_key_size*2;
        let block1: String = hex_ciphertext.clone().chars().take(hex_key_size).collect();
        let block2: String = hex_ciphertext.clone().chars().skip(hex_key_size).take(hex_key_size).collect();
        let block_hd = hamming_distance(&block1, &block2);
        normalized_hds.push((ascii_key_size, (block_hd as f64) / (ascii_key_size as f64)));
    }

    // sort the normalized hamming distances non-decreasing
    normalized_hds.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Less));

    /*
    // let's assume that the correct keysize is in the top 3
    let mut highest_overall_score = 0;
    let mut highest_ascii_plaintext = String::new();
    let mut highest_ascii_key = String::new();
    for &(key_size, normalized_hd) in normalized_hds.iter().take(3) {
    */
    /* ======================================================================= */
    /* For debugging purposes, assume the top keysize result is correct.       */
    /* This removes a for loop so I can consider all the code on the same      */
    /* indentation level.                                                      */

    // Break the ciphertext into blocks of key_size length using our friend
    // the gnarly iterator.
    // But first, pad the ciphertext so we get uniform blocks.
    // This makes the code a lot easier to write and does not affect the
    // scoring to the best of my knowledge.
    let ascii_key_size = normalized_hds[0].0;
    let hex_key_size = ascii_key_size * 2;
    let len = hex_ciphertext.len();
    let hex_ciphertext_padded = hex_ciphertext.clone() +
                                &String::from_utf8(
                                    vec![b'0'; hex_key_size - (len % hex_key_size)]
                                ).unwrap();
    let pad_len = hex_ciphertext_padded.len();
    let blocks = (0..).take(pad_len).filter(|&x| x % hex_key_size == 0)
                      .map(|i| &hex_ciphertext_padded[i..i+size]);
    // transpose the blocks
    // there's probably some mind-blowingly elegant way to do this with iterator
    // adapters, but I have not yet attained that level of functional programming
    // nirvana :(
    let mut transposed_blocks = vec![String::new(); ascii_key_size];
    for block in blocks {
        // split into groups of 2 (one byte each) using the gnarly iterator
        let bytes = (0..hex_key_size).filter(|&x| x % 2 == 0).map(|i| &block[i..i+2]);
        for (i, byte) in bytes.enumerate() {
            transposed_blocks[i].push_str(&byte);
        }
    }

    // solve each block as single-character XOR

    // this is a list of possible most-frequent plaintext characters
    let candidates = vec![' ', 'e', 't', 'a', 'o', 'i'];

    let mut hex_final_key = String::new();
    for tb in transposed_blocks {
        // Try every printable ascii char as a key
        // The key that generates the plaintext with the lowest chi-square value
        // is the one we append to the final key
        for key in (32..127) {
            let hex_key = format!("{:02x}", key as char);
            let hex_key_buffer: String = hex_key.chars().cycle().take(tb.len()).collect();
            let hex_plaintext = cryptoutil::hex_to_hex_xor(&hex_key_buffer, &tb);
            let ascii_plaintext = cryptoutil::hex_to_ascii(&hex_plaintext);

            let freq_table = cryptoutil::ascii_freq_analysis(&ascii_plaintext);
            // convert the frequency table into a vector of (char, percentage)
            let percentages = freq_table.into_iter().map(|(letter, count)| {
                (letter, (count as f64) / tb.len())
            });
            // google suggests that the standard way to measure the correlation of
            // two histograms is the chi-square test
            // I have no idea if I'm implementing this correctly

            /*
             * okay actually I think this can be written in the cleanest way using
             * `fold`. See here: http://www.itl.nist.gov/div898/handbook/eda/section3/eda35f.htm
             *
            for (observed, expected) in percentages.zip(LETTER_FREQS.iter()) {
                // we should be comparing stats for the same letter
                assert_eq!(observed.0, expected.0);
                let (o_value, e_value) = observed.1, expected.1;
                let chi_square = 
            }
            */
        }
    }

    // decrypt the full message with our derived key!
    // this code is more or less straight out of challenge 1-5
    let hex_key_buffer: String = hex_final_key.chars().cycle().take(hex_ciphertext.len()).collect();
    let hex_plaintext = cryptoutil::hex_to_hex_xor(&hex_key_buffer, &hex_ciphertext);

    let ascii_key = cryptoutil::hex_to_ascii(&hex_final_key);
    let ascii_plaintext = cryptoutil::hex_to_ascii(&hex_plaintext);

    println!("key: {}\nplaintext: {}", ascii_key, ascii_plaintext);

    /* ======================================================================= */
    /*
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
    */
}
