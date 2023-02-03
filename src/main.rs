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
    for c in text.chars() {
        letters = letters + 1.0;
        score += match c {
            'a' | 'A' => 8.167,
            'b' | 'B' => 1.492,
            'c' | 'C' => 2.782,
            'd' | 'D' => 4.253,
            'e' | 'E' => 12.702,
            'f' | 'F' => 2.228,
            'g' | 'G' => 2.015,
            'h' | 'H' => 6.094,
            'i' | 'I' => 6.966,
            'j' | 'J' => 0.153,
            'k' | 'K' => 0.772,
            'l' | 'L' => 4.025,
            'm' | 'M' => 2.406,
            'n' | 'N' => 6.749,
            'o' | 'O' => 7.507,
            'p' | 'P' => 1.929,
            'q' | 'Q' => 0.095,
            'r' | 'R' => 5.987,
            's' | 'S' => 6.327,
            't' | 'T' => 9.056,
            'u' | 'U' => 2.758,
            'v' | 'V' => 0.978,
            'w' | 'W' => 2.360,
            'x' | 'X' => 0.150,
            'y' | 'Y' => 1.974,
            'z' | 'Z' => 0.074,
            _ => 0.0,
        };
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
        old.score = analyze_text(&old.text) ;
        println!("testkey {} {}", n, old.score);
        if n == 88 {
            println!("{} {:?} {}", n,old.text, old.score);
        }
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
