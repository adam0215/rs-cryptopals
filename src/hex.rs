#[derive(Debug)]
pub enum HexError {
    OddLength,
    InvalidCharacter,
}

pub fn hex_decode(hex_bytes: &[u8]) -> Result<Vec<u8>, HexError> {
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

pub fn hex_encode(hex_bytes: &[u8]) -> Vec<char> {
    hex_bytes
        .iter()
        .flat_map(|ch| [(*ch / 16), (*ch % 16)])
        .map(|hex| match hex {
            0..=9 => (hex + b'0') as char,
            10..=16 => (hex + b'a' - 10) as char,
            _ => unreachable!(),
        })
        .collect()
}

fn hex_to_digit(hex: u8) -> Result<u8, HexError> {
    match hex as char {
        '0'..='9' => Ok(hex - b'0'),
        'A'..='F' => Ok(hex - b'A' + 10),
        'a'..='f' => Ok(hex - b'a' + 10),
        _ => Err(HexError::InvalidCharacter),
    }
}
