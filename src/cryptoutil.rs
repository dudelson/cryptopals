use std::char;
use std::cmp;
use std::collections::HashMap;
use std::ascii::AsciiExt;
use std::fmt::Write;

/// from challenge 1-1
/// given a string representing a hex value, returns a string representing the
/// equivalent base64 value
/// because we are ultimately representing ascii characters, this function has
/// the requirement that the length of the input string must be a multiple of 2
pub fn hex_to_base64(hex_str: &str) -> String {
    let len = hex_str.len();
    assert!(len % 2 == 0);
    let mut s = String::new();
    // Each character in the input (hex) string represents 4 bits, and each
    // character in the output (base64) string represents 6 bits. So I take the
    // hex chars in groups of 6, and convert each group into 4 base64 chars.
    // This is 24 bits, which fits nicely into a u32.

    // I take groups of 6 instead of groups of 3 because we are ultimately
    // encoding ascii characters, which are encoded as pairs of hex digits, and
    // the base64 encoding scheme doesn't really specify what to do if you have
    // an odd number of hex digits in the final group

    // first pad the input string with 0s until it's a multiple of 6 in length
    let hex_str_padded = hex_str.to_string() + &String::from_utf8(vec![b'0'; 6 - (len % 6)]).unwrap();
    // here's a gnarly iterator which produces slices of the input string of size 6
    // for example, if the input were "abcdefghijkl", this iterator would produce
    // "abcdef", "ghijkl"
    let it = (0..).take(len).filter(|&x| x % 6 == 0).map(|i| &hex_str_padded[i..i+6]);
    // iterate over the slices and convert each one to a base64 slice of size 4
    for slice in it {
        let hex_value = u32::from_str_radix(&slice, 16)
                             .expect(&format!("not a hex value: {}", &slice));
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
    }
    // if the original input wasn't divisible by 6, the last group was
    // converted incorrecly. Switch out some of the "0"s for padding ("=")
    if len % 6 == 2 {
        let temp: String = s.chars().take(s.len()-2).collect();
        temp + "=="
    } else if len % 6 == 4 {
        let temp: String = s.chars().take(s.len()-1).collect();
        temp + "="
    } else {
        s
    }
}

/// From challenge 1-6
/// Given a string representing a base64 value, returns a string representing
/// the equivalent hex value.
/// Because we are representing ascii characters, this function has the requirement
/// that the length of the input string must be a multiple of 4 (because 4 base64
/// characters is converted into 6 hex characters, or 3 ascii characters).
pub fn base64_to_hex(b64_str: &str) -> String {
    let len = b64_str.len();
    assert!(len % 4 == 0);
    let mut s = String::new();
    // Like I say above, take the b64 chars in groups of 4, and convert those
    // to 6 hex chars. This is 24 bits, or 3 bytes, so I use a u32.
    // We don't need to pad the input string like in `hex_to_base64` because the
    // input string should already be padded.

    // Another gnarly iterator
    let it = (0..).take(len).filter(|&x| x % 4 == 0).map(|i| &b64_str[i..i+4]);

    for slice in it {
        let mut v: Vec<u8> = vec![];
        for b64val in slice.chars() {
            match b64val {
                '='           => v.push(0), // these will be corrected at the end
                '+'           => v.push(62),
                '/'           => v.push(63),
                x @ 'A'...'Z' => v.push((x as u8) - 65),
                x @ 'a'...'z' => v.push((x as u8) + 26 - 97),
                x @ '0'...'9' => v.push((x as u8) + 52 - 48),
                x @ _         => panic!("Generated out-of-bounds value: {}", x),
            }
        }
        let h: u32 = ((v[0] as u32 & 63) << 18) | ((v[1] as u32 & 63) << 12) |
                     ((v[2] as u32 & 63) << 6) | v[3] as u32;
        s.push_str(&format!("{:06x}", h));
    }

    // remove values generated by padding
    if &b64_str[len-2..] == "==" {
        (&s[..s.len()-4]).to_string()
    } else if &b64_str[len-1..] == "=" {
        (&s[..s.len()-2]).to_string()
    } else {
        s
    }
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
