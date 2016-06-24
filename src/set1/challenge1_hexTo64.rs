#![allow(non_snake_case)]

use std::env;
use std::process;
use std::char;
use std::u32;

/// given a u8, returns a string representing the binary form of that u8
/// I'm not using this currently, but I'm keeping it around in case I need it
/// later.
#[allow(dead_code)]
fn binString(x: &u8) -> String {
    let mut s: String = String::from("");
    for b in 0..8 {
        match x & (1 << (7-b)) {
            0 => s.push('0'),
            _ => s.push('1'),
        }
    }
    s
}

/// given a string representing a hex value, returns a string representing the
/// equivalent base64 value
fn hexToBase64(hexStr: &str) -> String {
    let len = hexStr.len();
    let mut group = 0;
    let mut s = "".to_string();
    // Each character in the input (hex) string represents 4 bits, and each
    // character in the output (base64) string represents 6 bits. So I take the
    // input characters in groups of 6 (6 chars * 4 bits/char = 24 bits, which
    // fits nicely into a u32), and convert them into 4 base64 chars.
    while group*6 < len {
        let hexValue: u32 = u32::from_str_radix(&hexStr[group*6 .. (group+1)*6], 16).unwrap();
        let v = vec![
            (hexValue & (63 << 18)) >> 18,
            (hexValue & (63 << 12)) >> 12,
            (hexValue & (63 << 6)) >> 6,
            hexValue & 63
        ];
        for b64val in v {
            match b64val {
                62          => s.push('+'),
                63          => s.push('/'),
                x @ 0...25  => s.push(char::from_u32(65 + x).unwrap()),
                x @ 26...51 => s.push(char::from_u32(97 + x - 26).unwrap()),
                x @ 52...61 => s.push(char::from_u32(48 + x - 52).unwrap()),
                x @ _       => println!("Generated out-of-bounds value: {}", x),
            }
        }
        group += 1;
    }
    s
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("Please enter one or more hex strings to convert to base64");
        process::exit(1);
    }

    for hexStr in &args[1..] {
        let base64Str = hexToBase64(hexStr);
        println!("{} => {}", hexStr, base64Str);
    }
}
