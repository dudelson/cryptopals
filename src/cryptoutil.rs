use std::char;
use std::cmp;
use std::collections::HashMap;
use std::ascii::AsciiExt;
use std::fmt::Write;

/// from challenge 1-1
/// given a string representing a hex value, returns a string representing the
/// equivalent base64 value
pub fn hex_to_base64(hex_str: &str) -> String {
    let mut group = 0;
    let mut s = "".to_string();
    // Each character in the input (hex) string represents 4 bits, and each
    // character in the output (base64) string represents 6 bits. So I take the
    // input characters in groups of 6 (6 chars * 4 bits/char = 24 bits, which
    // fits nicely into a u32), and convert them into 4 base64 chars.
    while group*6 < hex_str.len() {
        let hex_value = u32::from_str_radix(&hex_str[group*6 .. (group+1)*6], 16).unwrap();
        let v = vec![
            (hex_value & (63 << 18)) >> 18,
            (hex_value & (63 << 12)) >> 12,
            (hex_value & (63 << 6)) >> 6,
             hex_value & 63
        ];
        for b64val in v {
            match b64val {
                62          => s.push('+'),
                63          => s.push('/'),
                x @ 0...25  => s.push(char::from_u32(65 + x).unwrap()),
                x @ 26...51 => s.push(char::from_u32(97 + x - 26).unwrap()),
                x @ 52...61 => s.push(char::from_u32(48 + x - 52).unwrap()),
                x @ _       => panic!("Generated out-of-bounds value: {}", x),
            }
        }
        group += 1;
    }
    s
}

/// from challenge 1-2
/// Given two equal-length, hex-encoded strings, returns a hex-encoded string
/// representing the XOR of the input strings
pub fn hex_to_hex_xor(buf1: &str, buf2: &str) -> String {
    assert_eq!(buf1.len(), buf2.len());
    let mut s = "".to_string();
    let mut i = 0;
    while i < buf1.len() {
        let hex1 = u32::from_str_radix(&buf1[i..i+2], 16).unwrap();
        let hex2 = u32::from_str_radix(&buf2[i..i+2], 16).unwrap();
        write!(&mut s, "{:02x}", hex1 ^ hex2);
        i += 2;
    }
    s
}

/// from challenge 1-2
/// given a hex-encoded string, returns the decoded ascii string
/// (note that the string will actually be a unicode (UTF-8 encoded) string since
///  this is rust, but the characters that make up the string are all guaranteed
///  to be part of the original ascii charset).
pub fn hex_to_ascii(s: &str) -> String {
    let mut t = "".to_string();
    let mut i = 0;
    while i < s.len() {
        let ascii = u8::from_str_radix(&s[i..i+2], 16).unwrap();
        t.push(ascii as char);

        i += 2;
    }
    t
}

/// from challenge 1-2
/// given an ascii string, returns an encoded hex string
/// (note that this function takes a normal, UTF-8 encoded rust string slice as an
///  argument, but panics if any character is not part of the original ascii charset)
pub fn ascii_to_hex(s: &str) -> String {
    let mut t = "".to_string();
    for ascii_char in s.chars() {
        assert!(ascii_char.is_ascii());
        write!(&mut t, "{:02x}", ascii_char as u8);
    }
    t
}

/// from challenge 1-3
/// given a hex-encoded string, returns a hashmap where keys are strings representing
/// byte values, and values are the frequencies of those byte values in the input string
pub fn hex_freq_analysis(s: &str) -> HashMap<&str, u32> {
    let mut freq_table = HashMap::new();
    let mut i = 0;
    while i < s.len() {
        let byte = &s[i..i+2];
        let count = freq_table.entry(byte).or_insert(0);
        *count += 1;
        i += 2;
    }  
    freq_table
}

/// from challenge 1-3
/// given an ascii string, returns a hashmap where keys are ascii characters, and
/// values are the frequencies of those byte values in the input string
pub fn ascii_freq_analysis(s: &str) -> HashMap<char, u32> {
    let mut freq_table = HashMap::new();
    for c in s.chars() {
        let count = freq_table.entry(c).or_insert(0);
        *count += 1;
    }
    freq_table
}

/// from challenge 1-3
/// given a hex-encoded string, returns a vector containing the first <n> most common
/// bytes in that string
pub fn hex_top_freq(s: &str, n: usize) -> Vec<&str> {
    let freq_table = hex_freq_analysis(&s);
    let mut v: Vec<_> = freq_table.iter().collect();
    v.sort_by(|a, b| b.1.cmp(a.1));
    let (mut chars, freqs): (Vec<&str>, Vec<u32>) = v.iter().cloned().unzip();
    chars.truncate(n);
    chars
}

/// from challenge 1-3
/// given an ascii string, returns a vector containing the first <n> most common
/// chars in that string
pub fn ascii_top_freq(s: &str, n: usize) -> Vec<char> {
    let freq_table = ascii_freq_analysis(&s);
    let mut v: Vec<_> = freq_table.iter().collect();
    v.sort_by(|a, b| b.1.cmp(a.1));
    let (mut chars, freqs): (Vec<char>, Vec<u32>) = v.iter().cloned().unzip();
    chars.truncate(n);
    chars
}
