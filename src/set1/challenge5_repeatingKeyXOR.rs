#[path="../cryptoutil.rs"]
mod cryptoutil;

use std::env;
use std::process;
use std::string::String;

/* mistakes:
 *  - using repeat to key the key buffer instead of cycle (assumed plaintext length
 *    would be a multiple of key length; not so!)
 */
fn main() {
    let USAGE = "usage: challenge5_repeatingKeyXOR <key> <plaintext>";
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 3 {
        println!("{}", USAGE);
        process::exit(1);
    }

    let ascii_key = &args[1];
    let ascii_plaintext = &args[2];
    let ascii_key_buffer: String = ascii_key.chars().cycle().take(ascii_plaintext.len()).collect();
    let hex_key_buffer = cryptoutil::ascii_to_hex(&ascii_key_buffer);
    let hex_plaintext = cryptoutil::ascii_to_hex(&ascii_plaintext);
    let hex_ciphertext = cryptoutil::hex_to_hex_xor(&hex_key_buffer, &hex_plaintext);
    println!("{}", hex_ciphertext);
}
