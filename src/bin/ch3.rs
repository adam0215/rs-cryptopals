use rs_cryptopals::hex::hex_decode;
use std::collections::HashMap;

const INPUT: &[u8] = b"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

fn main() {
    let decoded_hex = hex_decode(INPUT).unwrap();

    let (key, _) = (0..=127)
        .map(|ascii_char| {
            let xored: Vec<_> = decoded_hex.iter().map(|ch| ch ^ ascii_char).collect();
            let scored = score_text(xored.as_slice());

            (ascii_char, scored)
        })
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap();

    let deciphered_str: String = decoded_hex.iter().map(|ch| (*ch ^ key) as char).collect();

    println!("Cipher key: {}", key as char);
    println!("{}", deciphered_str);
}

fn score_text(text: &[u8]) -> usize {
    // Scoring table based on hint in challenge
    let scoring_ref = [
        b'e', b't', b'a', b'o', b'i', b'n', b' ', b's', b'h', b'r', b'd', b'l', b'u',
    ];

    text.iter()
        .fold(0, |acc, ch| acc + (scoring_ref.contains(ch) as usize))
}
