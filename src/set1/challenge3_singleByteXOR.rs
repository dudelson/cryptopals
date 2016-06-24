/* This code is kind of a mess, so I am providing my thought process here.
 * Initially, I thought the easiest thing to do would be to perform frequency
 * analysis on the ciphertext and derive the encryption key under the assumption
 * that the bytes that appeared most frequently in the ciphertext are the most
 * common engligh letters. This analysis is performed by the function `main1`.
 * In retrospect, I forgot to account for uppercase letters here, but what actually tripped me
 * up was that the most common character in the plaintext turned out to be a space,
 * instead of the letter 'e'. I was stumped as to why my frequency analysis
 * wasn't working, so I opted to instead try a brute force approach. I wrote `main2`,
 * which decodes the ciphertext with every printable ascii character in turn used
 * as the key. It then ranks these plaintexts by the number of common english letters
 * that appear, and prints out the top scoring plaintext. This was how I actually
 * decoded the ciphertext and recovered the key. I then went back to `main1` and
 * confirmed that my frequency analysis would have worked had I though to try out
 * a space as the most frequently-occuring character.
 */
use std::collections;

/// given a hex-encoded string of ciphertext, returns a hashmap where keys
/// are bytes and values are the frequencies of those bytes
fn freq_analysis(input: &str) -> collections::HashMap<&str, u32> {
    let mut freq_table = collections::HashMap::new();
    // so I guess the "best" way to step thru iterators (with the step_by() function)
    // is unstable right now, so in order to compile this on stable rust, I'm using
    // a while loop instead
    let mut i = 0;
    while i < input.len() {
        let byte = &input[i..i+2];
        let count = freq_table.entry(byte).or_insert(0);
        *count += 1;
        i += 2;
    }  
    freq_table
}

/// given some a ciphertext and a char that was used to encrypt the ciphertext
/// using XOR, runs XOR again to decrypt the text
/// This function was completely unnecessary; I should have created a buffer
/// of equal length to the ciphertext and filled it with the key, and then called
/// fixed_xor from challenge 2. Live and learn.
fn decrypt(ciphertext: &str, encryption_char: char) -> String {
    let mut s = "".to_string();
    let mut i = 0;
    while i < ciphertext.len() {
        let byte = u8::from_str_radix(&ciphertext[i..i+2], 16).unwrap();
        let ascii = (byte ^ (encryption_char as u8));
        //println!("0x{:02x} ^ 0x{:02x} = 0x{:02x}", byte, (encryption_char as u8), ascii);
        s.push(ascii as char);
        i += 2;
    }
    s
}

fn main1() {
    let ciphertext = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

    // associate bytes with their frequencies
    let freq_table = freq_analysis(ciphertext);

    // get the most frequently-used byte. This is probably the letter 'e'
    let mut sorted: Vec<_> = freq_table.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));

    // Since XOR undoes itself, XOR 'e' and the most-frequent byte to get the
    // character that was used to produce the ciphertext

    // Retrospective comment: you can see that I'm actually using a space; this
    // was changed after I recovered the plaintext with `main2` and realized what
    // the most common character actually was
    let freq_byte_ascii = u8::from_str_radix(sorted[0].0, 16).unwrap();
    let encryption_char: char = ((' ' as u8) ^ freq_byte_ascii) as char;

    // XOR the ciphertext against this character and print the decrypted message!
    println!("The message is: {}", decrypt(ciphertext, encryption_char));
}

fn main2() {
    let ciphertext = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

    // try every printable ascii character as a potential key
    let mut highscore = 0;
    let mut highplaintext = "".to_string();
    for encryption_key in 32..127 {
        // decode the ciphertext using this key. This gives us a candidate string
        // of plaintext
        let plaintext = decrypt(ciphertext, (encryption_key as u8 as char));
        println!("{}", plaintext);

        // associate chars in the plaintext to their freqs
        let mut freq_table = collections::HashMap::new();
        for c in plaintext.chars() {
            let mut count = freq_table.entry(c).or_insert(0);
            *count += 1;
        }

        // sort the hashmap by frequency so we can get a list of chars sorted by
        // freq non-increasing
        let mut sorted: Vec<_> = freq_table.iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(a.1));
        let (chars, freqs): (Vec<char>, Vec<u32>) = sorted.iter().cloned().unzip();

        // if the most common english letters are among the highest frequencies
        // in the plaintext, we might have something legible
        let mut score = 0;
        let common_english_chars = vec!['e', 't', 'a', 'o', 'i', 'E', 'T', 'A', 'O', 'I'];
        for common_char in chars.iter().take(5) {
            //println!("checking for {}", common_char);
            if common_english_chars.contains(common_char) { score += 1; }
        }

        // greedily take the plaintext with the highest score
        if score > highscore { highscore = score; highplaintext = plaintext.clone(); }
        //println!("With key {} (score: {}) message is: {}", encryption_key as u8 as char, score, plaintext);
    }

    println!("The message is: {}", highplaintext);
}

fn main() {
    // only one of these should run
    main1();
    //main2();
}
