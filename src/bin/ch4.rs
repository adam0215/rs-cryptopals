use rs_cryptopals::hex::hex_decode;

fn main() {
    let input_file = include_bytes!("../../inputs/ch4.txt");
    let decoded_strings: Vec<_> = input_file
        .split(|&ch| ch == b'\n')
        .map(|ln| hex_decode(ln).unwrap())
        .collect();

    let top_candidates: Vec<_> = decoded_strings
        .iter()
        .map(|ln| {
            let (key, score) = get_top_lang_score(ln);

            let deciphered_str: String = ln.iter().map(|ch| (*ch ^ key) as char).collect();

            (key, score, deciphered_str)
        })
        .collect();

    let (key, _, top_deciphered) = top_candidates
        .iter()
        .max_by(|(_, a_score, _), (_, b_score, _)| a_score.cmp(b_score))
        .unwrap();

    println!("Cipher key: {}", *key as char);
    println!("{}", top_deciphered.as_str().trim());
}

fn get_top_lang_score(string: &Vec<u8>) -> (u8, usize) {
    (0..=127)
        .map(|ascii_char| {
            let max_lang_score = score_text(
                string
                    .iter()
                    .map(|ch| ch ^ ascii_char)
                    .collect::<Vec<_>>()
                    .as_slice(),
            );

            (ascii_char, max_lang_score)
        })
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
}

fn score_text(text: &[u8]) -> usize {
    // Ordered based on frequency in the English language
    let letter_freqs = [
        b'z', b'j', b'q', b'x', b'k', b'v', b'b', b'p', b'g', b'w', b'y', b'f', b'm', b'c', b'u',
        b'l', b'd', b'h', b'r', b's', b'n', b'i', b'o', b'a', b't', b'e',
    ];

    text.to_ascii_lowercase().iter().fold(0_usize, |acc, ch| {
        acc + (letter_freqs.iter().position(|c| c == ch).unwrap_or(0))
    })
}
