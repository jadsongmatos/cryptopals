# Crypto Challenge Set 1

## Convert hex to base64
The string: `49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d`

Should produce: `SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t`

So go ahead and make that happen. You'll need to use this code for the rest of the exercises.

### 1.rs

This is a Rust program that converts a hexadecimal string to a base64 string. The program takes the hexadecimal string as a command line argument. The hexadecimal string is first converted to a decoded string using the `string_to_hex` function and then the `hex_to_base64` function is used to encode the decoded string in base64. If the input is not provided as a command line argument or if there is an error in parsing the hexadecimal string, the program exits with an error code.



## Fixed XOR
Write a function that takes two equal-length buffers and produces their XOR combination.

If your function works properly, then when you feed it the string: `1c0111001f010100061a024b53535009181c`

after hex decoding, and when XOR'd against: `686974207468652062756c6c277320657965`

should produce: `746865206b696420646f6e277420706c6179`

### 2.rs

This code takes two equal-length hexadecimal strings as command line arguments and performs XOR combination on them. The input hexadecimal strings are first decoded to their respective string representations and then XORed using the `cipher_xor` function. The XOR result is then encoded back to hexadecimal representation and printed on the console. If there are not exactly two arguments, the code exits with the exit code 7.

## Single-byte XOR cipher
The hex encoded string: `1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736`

has been XOR'd against a single character. Find the key, decrypt the message.

You can do this by hand. But don't: write code to do it for you.

How? Devise some method for "scoring" a piece of English plaintext. Character frequency is a good metric. Evaluate each output and choose the one with the best score.

### 3.rs

This is an implementation of a single-byte XOR cipher in Rust programming language. The code takes a hex-encoded string as input and finds the XOR key used to encrypt the string by evaluating the English character frequency of decrypted outputs. The score is calculated by comparing the decrypted output's frequency of letters to the frequency of letters in the English language. The decrypted string with the highest score is considered the correct message. The code consists of several helper functions to handle conversions between hexadecimal and string, XOR encryption/decryption, and scoring the decrypted string.

## Detect single-character XOR
One of the 60-character strings in this file has been encrypted by single-character XOR.

### 4.rs

This code decrypts a single-character XOR encrypted string from a file with multiple strings. It does so by using a hex to string conversion function and then XORing the string with a range of key values from 0 to 255. It calculates a score for each XORed string using an "analyze_text" function, which evaluates the frequency of each character in the XORed string compared to the expected frequency of letters in the English language. The XORed string with the highest score is assumed to be the decrypted string. The code then prints the decrypted string and the key used.
