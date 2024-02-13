const BASE64_REF: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

const INPUT:&[u8] = b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
const EXPECTED: &str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

fn main() {
    let bytes = hex_decode(INPUT);
    let base64_str = ascii_to_base64(bytes.unwrap().as_slice());

    println!("{}", base64_str);
    assert_eq!(base64_str, EXPECTED.to_string());
}

#[derive(Debug)]
enum HexError {
    OddLength,
    InvalidCharacter,
}

fn hex_decode(hex_bytes: &[u8]) -> Result<Vec<u8>, HexError> {
    if hex_bytes.len() % 2 != 0 {
        return Err(HexError::OddLength);
    }

    let result: Result<Vec<u8>, HexError> = hex_bytes
        .chunks(2)
        .map(|c| {
            let a = hex_to_digit(c[0])?;
            let b = hex_to_digit(c[1])?;

            Ok(a * 16 + b)
        })
        .collect();

    result
}

fn hex_to_digit(hex: u8) -> Result<u8, HexError> {
    match hex as char {
        '0'..='9' => Ok(hex - b'0'),
        'A'..='F' => Ok(hex - b'A' + 10),
        'a'..='f' => Ok(hex - b'a' + 10),
        _ => Err(HexError::InvalidCharacter),
    }
}

fn ascii_to_base64(ascii_bytes: &[u8]) -> String {
    if ascii_bytes.len() == 0 {
        return String::new();
    }

    let mut base64_str = String::new();

    ascii_bytes.chunks(3).for_each(|c| {
        let bx = *c.get(0).unwrap_or(&0) as u32;
        let by = *c.get(1).unwrap_or(&0) as u32;
        let bz = *c.get(2).unwrap_or(&0) as u32;

        let concat_blocks = (bx << 0x10) + (by << 0x08) + bz;

        let mask = 0xFC0000 as usize;
        let block_size = 6_usize;

        for p_offset in 0..=c.len() {
            let char_i =
                (concat_blocks as usize & mask >> (6 * p_offset)) >> block_size * (3 - p_offset);
            base64_str.push(BASE64_REF[char_i] as char)
        }

        // Add padding
        base64_str.extend(std::iter::repeat('=').take(3 - c.len()));
    });

    base64_str
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_cryptopal_value() {
        // Value from Cryptopals

        let input = hex_decode(INPUT).unwrap();
        let result = ascii_to_base64(input.as_slice());
        assert_eq!(result, EXPECTED);
    }

    #[test]
    fn test_hex_to_base64_empty_input() {
        let input = hex_decode(b"").unwrap();
        let result = ascii_to_base64(input.as_slice());
        assert_eq!(result, "");
    }

    #[test]
    fn test_hex_to_base64_valid_input() {
        let input = hex_decode(b"48656c6c6f2c20576f726c64").unwrap(); // "Hello, World" in hex
        let expected_output = "SGVsbG8sIFdvcmxk";
        let result = ascii_to_base64(input.as_slice());
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_hex_to_base64_odd_length_input() {
        let input = hex_decode(b"1a3b5"); // Odd-length input

        println!("RESULT {:?}", input);

        assert!(input.is_err());
    }

    #[test]
    fn test_hex_to_base64_invalid_hex_characters() {
        let input = hex_decode(b"gibberish"); // Contains non-hex characters

        assert!(input.is_err());
    }
}
