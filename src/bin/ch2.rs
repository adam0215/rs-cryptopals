use rs_cryptopals::hex::{hex_decode, hex_encode};

const INPUT: &[u8] = b"1c0111001f010100061a024b53535009181c";
const XOR_AGAINST: &[u8] = b"686974207468652062756c6c277320657965";
const EXPECTED: &str = "746865206b696420646f6e277420706c6179";

fn main() {
    let fixed_xor = hex_fixed_xor(INPUT, XOR_AGAINST);

    println!("{}", fixed_xor);
    assert_eq!(fixed_xor, EXPECTED)
}

fn hex_fixed_xor(b1: &[u8], b2: &[u8]) -> String {
    let b1 = hex_decode(b1).unwrap();
    let b2 = hex_decode(b2).unwrap();

    let xored: Vec<u8> = b1.iter().zip(b2.iter()).map(|(a, b)| a ^ b).collect();

    let fixed_xor: String = hex_encode(xored.as_slice()).iter().collect();

    fixed_xor
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_cryptopal_value() {
        // Value from Cryptopals
        let fixed_xor = hex_fixed_xor(INPUT, XOR_AGAINST);

        assert_eq!(fixed_xor, EXPECTED);
    }
}
