use rs_cryptopals::hex::hex_encode;
const KEY: &[u8] = b"ICE";
const INPUT: &[u8] = b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
#[allow(dead_code)]
const EXPECTED: &str = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

fn main() {
    let encrypted_str = repeating_xor_encrypt(INPUT, KEY);

    assert_eq!(encrypted_str, EXPECTED);
    println!("{}", encrypted_str)
}

fn repeating_xor_encrypt(bytes: &[u8], key: &[u8]) -> String {
    let xored_str: String = bytes
        .iter()
        .enumerate()
        .flat_map(|(i, ch)| hex_encode(&[(*ch ^ key[i % key.len()])]))
        .collect();

    xored_str
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_cryptopal_value() {
        // Value from Cryptopals
        let encrypted_str = repeating_xor_encrypt(INPUT, KEY);

        assert_eq!(encrypted_str, EXPECTED);
    }
}
