use std::env;
use std::collections::HashMap;

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

fn cipher_xor(text: &str, b: u8) -> String {
    let bytes = text.as_bytes();
    let mut result = String::new();
    for byte in bytes.iter() {
        result.push(char::from(byte ^ b));
    }
    return result;
}

fn analyze_text(text: &String) -> f32 {
    let mut score = 0.0;
    let mut letters: f32 = 0.0;
    for tex in text.chars() {
        letters = letters + 1.0;
        match tex {
            'a' => score = score + 8.167,
            'A' => score = score + 8.167,
            'b' => score = score + 1.492,
            'B' => score = score + 1.492,
            'c' => score = score + 2.782,
            'C' => score = score + 2.782,
            'd' => score = score + 4.253,
            'D' => score = score + 4.253,
            'e' => score = score + 12.702,
            'E' => score = score + 12.702,
            'f' => score = score + 2.228,
            'F' => score = score + 2.228,
            'g' => score = score + 2.015,
            'G' => score = score + 2.015,
            'h' => score = score + 6.094,
            'H' => score = score + 6.094,
            'i' => score = score + 6.966,
            'I' => score = score + 6.966,
            'j' => score = score + 0.153,
            'J' => score = score + 0.153,
            'k' => score = score + 0.772,
            'K' => score = score + 0.772,
            'l' => score = score + 4.025,
            'L' => score = score + 4.025,
            'm' => score = score + 2.406,
            'M' => score = score + 2.406,
            'n' => score = score + 6.749,
            'N' => score = score + 6.749,
            'o' => score = score + 7.507,
            'O' => score = score + 7.507,
            'p' => score = score + 1.929,
            'P' => score = score + 1.929,
            'q' => score = score + 0.095,
            'Q' => score = score + 0.095,
            'r' => score = score + 5.987,
            'R' => score = score + 5.987,
            's' => score = score + 6.327,
            'S' => score = score + 6.327,
            't' => score = score + 9.056,
            'T' => score = score + 9.056,
            'u' => score = score + 2.758,
            'U' => score = score + 2.758,
            'v' => score = score + 0.978,
            'V' => score = score + 0.978,
            'w' => score = score + 2.360,
            'W' => score = score + 2.360,
            'x' => score = score + 0.150,
            'X' => score = score + 0.150,
            'y' => score = score + 1.974,
            'Y' => score = score + 1.974,
            'z' => score = score + 0.074,
            'Z' => score = score + 0.074,
            ' ' => score = score + 1.0,
            _ => {}
        }
    }

    let chance = score / letters;
    chance
}

struct TestKeys {
    text: String,
    key: u8,
    score: f32,
}

fn test_key(input: &str) -> TestKeys {
    let mut best = TestKeys {
        text: String::from(""),
        key: 0,
        score: -1.0,
    };

    let mut old = TestKeys {
        text: String::from(""),
        key: 0,
        score: -1.0,
    };

    for n in 0..255 {
        old.text = cipher_xor(input, n);
        old.score = analyze_text(&old.text);
        println!("testkey {} {}", n, old.score);
        if old.score >= best.score {
            best.text = old.text;
            best.score = old.score;

            best.key = n;
        }
    }
    return best;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() > 1 {
        let hex = match hex_to_string(args[1].clone()) {
            Err(e) => {
                println!("Error: {}", e);
                std::process::exit(1);
            }
            Ok(f) => f,
        };

        println!("{:?}", hex);

        let result = test_key(hex.as_str());
        println!("text: {:?} score: {:?} key: {}", result.text, result.score, result.key);

        //let score_en = analyze_text(hex.as_str());
        //println!("{:?}", score_en);

        std::process::exit(0);
    } else {
        std::process::exit(7);
    }
}
