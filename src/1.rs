/*
Convert hex to base64
The string: 49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d
Should produce: SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t
So go ahead and make that happen. You'll need to use this code for the rest of the exercises.
*/

use std::env;

static BASE64_TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn string_to_hex(input: String) -> Result<String, std::num::ParseIntError> {
    let mut result = String::new();
    let mut chars = input.chars().enumerate();

    while let Some((i, c)) = chars.next() {
        let c2 = chars.next().unwrap().1;
        let hex = format!("{}{}", c, c2);
        let byte = u8::from_str_radix(&hex, 16)?;
        result.push(char::from(byte));
    }

    Ok(result)
}

fn hex_to_base64(hex_string: String) -> String {
    let decoded_hex = hex_string.into_bytes();

    let mut base64_encoded = String::new();
    let mut current_bits = 0;
    let mut bits_left = 0;

    for byte in decoded_hex {
        current_bits = (current_bits << 8) | byte as u16;
        bits_left += 8;
        while bits_left >= 6 {
            let index = (current_bits >> (bits_left - 6)) & 0b0011_1111;
            let c = BASE64_TABLE[index as usize];
            base64_encoded.push(c as char);
            bits_left -= 6;
        }
    }
    if bits_left > 0 {
        let index = (current_bits << (6 - bits_left)) & 0b0011_1111;
        let c = BASE64_TABLE[index as usize];
        base64_encoded.push(c as char);
    }
    while base64_encoded.len() % 4 != 0 {
        base64_encoded.push('=');
    }

    base64_encoded
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() > 1 {
        let hex = match string_to_hex(args[1].clone()){
           Err(e) => {
               println!("Error: {}", e);
               std::process::exit(1);
           },
           Ok(f) => f,
        };

        println!("{:?}", hex);
        let b64 = hex_to_base64(hex);
        println!("{}", b64);
        std::process::exit(0);
    } else {
        std::process::exit(7);
    }
}