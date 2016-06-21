/* TODO:
 *   - validate input strings as hex
 */
#![allow(non_snake_case)]

use std::env;
use std::process;
use std::char;
use std::u32;

/*
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
*/

fn hexToBase64(hexStr: &str) -> String {
    let len = hexStr.len();
    let mut group = 0;
    let mut s = "".to_string();
    while group*3 < len {
        let hexValue: u32 = u32::from_str_radix(&hexStr[group*3 .. (group+1)*3], 16).unwrap();
        let b64val1: u32 = (hexValue & (63 << 6)) >> 6;
        let b64val2: u32 = hexValue & 63;
        let v = vec![b64val1, b64val2];
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
