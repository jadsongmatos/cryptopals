/*
Fixed XOR
Write a function that takes two equal-length buffers and produces their XOR combination.

If your function works properly, then when you feed it the string: 1c0111001f010100061a024b53535009181c
after hex decoding, and when XOR'd against: 686974207468652062756c6c277320657965
should produce: 746865206b696420646f6e277420706c6179
*/

use std::env;

fn hex_to_string(input: String) -> Result<String, std::num::ParseIntError> {
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

fn string_to_hex(input: String) -> Result<String, std::num::ParseIntError> {
    let mut result = String::new();
    let chars = input.chars().collect::<Vec<char>>();

    for i in (0..chars.len()).step_by(2) {
        let hex = format!("{}{}", chars[i], chars[i + 1]);
        let byte = u8::from_str_radix(&hex, 16)?;
        result.push(char::from(byte));
    }

    Ok(result)
}

fn cipher_xor(a: &str, b: &str) -> String {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();

    if a_bytes.len() != b_bytes.len() {
        panic!("Input strings must have equal length");
    }

    let xor_result = a_bytes.iter().zip(b_bytes).map(|(x, y)| x ^ y).collect();
    String::from_utf8(xor_result).expect("XOR result is not valid UTF-8")
}


fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() > 2 {

        let hex = match hex_to_string(args[1].clone()){
           Err(e) => {
               println!("Error: {}", e);
               std::process::exit(1);
           },
           Ok(f) => f,
        };

        let xor = match hex_to_string(args[2].clone()){
           Err(e) => {
               println!("Error: {}", e);
               std::process::exit(1);
           },
           Ok(f) => f,
        };


        println!("{:?}", xor);
        let result = cipher_xor(hex.as_str(),xor.as_str());
        println!("result: {:?}", result);

        let result_hex = result.as_bytes().iter().map(|x| format!("{:02x}", x)).collect::<String>();

        println!("{:?}", result_hex);
        std::process::exit(0);
    } else {
        std::process::exit(7);
    }
}
