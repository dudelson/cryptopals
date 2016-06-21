use std::env;
use std::process;
use std::cmp;

/// Given two equal-length, hex-encoded strings, returns a hex-encoded string
/// representing the XOR of the input strings
fn fixed_xor(buf1: &str, buf2: &str) -> String {
    assert_eq!(buf1.len(), buf2.len());
    let mut s = "".to_string();
    let mut group = 0;
    while group*8 < buf1.len() {
        let (start, end) = (group*8, cmp::min((group+1)*8, buf1.len()));
        let slice1 = &buf1[start .. end];
        let slice2 = &buf2[start .. end];
        let hex1 = u32::from_str_radix(slice1, 16).unwrap();
        let hex2 = u32::from_str_radix(slice2, 16).unwrap();
        s.push_str(&format!("{:x}", hex1 ^ hex2));
        group += 1;
    }

    s
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        println!("Enter two strings as arguments");
        process::exit(1);
    }

    println!("{}", fixed_xor(&args[1], &args[2]));
}
