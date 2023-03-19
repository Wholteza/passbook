// Implemented following rfc3174
// https://datatracker.ietf.org/doc/html/rfc3174#section-6.1

use std::{array, io::Read, string};

// Method 1
const PADDING_MIN_LENGTH_IN_BYTES: usize = 9;
const BLOCK_SIZE_IN_BYTES: usize = 64;
const PADDING_END_SIZE_IN_BYTES: usize = 8;
const PADDING_START_SIZE_IN_BYTES: usize = 1;
const MESSAGE_MAX_LENGTH_IN_BYTES: usize = u64::MAX as usize / 8;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn padded_message_is_always_a_multiple_of_64() {
        for index in 1..(BLOCK_SIZE_IN_BYTES * 100) {
            let input = vec![0xFF; index];
            let padded_input = apply_padding(&input);
            let padded_input_length = padded_input.len();
            assert_eq!(padded_input_length % BLOCK_SIZE_IN_BYTES, 0);
        }
    }

    #[test]
    fn padded_message_can_be_parsed_into_blocks_of_words() {
        let message = apply_padding(&vec![0xFF; 10]);
        let blocks = into_blocks(&message);
        assert_eq!(blocks.len(), message.len() / BLOCK_SIZE_IN_BYTES);
        for words in blocks {
            assert_eq!(words.len(), 16)
        }
    }

    #[test]
    fn decimals_are_converted_to_hex() {
        let decimal: u32 = 255;
        let hexadecimal = to_hex(decimal);
        assert_eq!(hexadecimal, "FF");

        let decimal: u32 = 16;
        let hexadecimal = to_hex(decimal);
        assert_eq!(hexadecimal, "10");

        let decimal: u32 = 128;
        let hexadecimal = to_hex(decimal);
        assert_eq!(hexadecimal, "80");
    }

    #[test]
    fn hexadecimals_less_than_16_are_padded_with_zero() {
        let decimal: u32 = 0;
        let hexadecimal = to_hex(decimal);
        assert_eq!(hexadecimal, "00");

        let decimal: u32 = 8;
        let hexadecimal = to_hex(decimal);
        assert_eq!(hexadecimal, "08");

        let decimal: u32 = 15;
        let hexadecimal = to_hex(decimal);
        assert_eq!(hexadecimal, "0F");
    }
}

pub fn sha_1(message: &Vec<u8>) {
    let mut h0: u32 = 0x67452301;
    let mut h1: u32 = 0xEFCDAB89;
    let mut h2: u32 = 0x98BADCFE;
    let mut h3: u32 = 0x10325476;
    let mut h4: u32 = 0xC3D2E1F0;

    let message = apply_padding(message);
    let blocks = into_blocks(&message);

    for mut words in blocks {
        for t in 16..80 {
            words[t] = (words[t - 3] ^ words[t - 8] ^ words[t - 14] ^ words[t - 16]).rotate_left(1);
        }

        let mut a: u32 = h0.to_owned();
        let mut b: u32 = h2.to_owned();
        let mut c: u32 = h2.to_owned();
        let mut d: u32 = h3.to_owned();
        let mut e: u32 = h4.to_owned();

        for t in 0..80 {
            let temp = a
                .rotate_left(5)
                .wrapping_add(operation(t, &b, &c, &d))
                .wrapping_add(e)
                .wrapping_add(words[t])
                .wrapping_add(constant(t));
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;
        }

        h0 = h0.wrapping_add(a);
        h1 = h1.wrapping_add(b);
        h2 = h2.wrapping_add(c);
        h3 = h3.wrapping_add(d);
        h4 = h4.wrapping_add(e);
    }
}

fn to_hex(int: u32) -> String {
    let hex_chars = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
    ];
    let mut input = int;
    let mut hex_string = String::new();
    if input < 16 {
        let mut string = String::from(hex_chars[(input) as usize]);
        string.insert(0, '0');
        return string;
    }
    loop {
        let rest = (input % 16) as usize;
        input = input / 16;
        hex_string.insert(0, hex_chars[rest]);
        if input < 16 {
            hex_string.insert(0, hex_chars[(input) as usize]);
            break;
        }
    }
    if hex_string.len() == 1 {
        hex_string.insert(0, '0');
    }

    hex_string
}

pub fn apply_padding(message: &Vec<u8>) -> Vec<u8> {
    let original_length: u64 = message.len().try_into().expect("error");
    let mut buf = vec![];

    for byte in message {
        buf.push(byte.to_owned());
    }

    // Push start of padding
    buf.push(0x80);

    // Push the padding itself
    while (buf.len() + PADDING_END_SIZE_IN_BYTES) % BLOCK_SIZE_IN_BYTES != 0 {
        buf.push(0x00);
    }

    // Push the end of the padding
    let (low, high) = split_u64_to_u32(original_length);
    for byte in [low.to_be_bytes(), high.to_be_bytes()].concat() {
        buf.push(byte);
    }

    buf
}

pub fn split_u64_to_u32(int: u64) -> (u32, u32) {
    (int as u32, (int >> 32) as u32)
}

pub fn u8_array_to_u32(bytes: &[u8]) -> u32 {
    assert_eq!(bytes.len(), 4);
    u32::from_be_bytes(bytes.try_into().expect("nope"))
}

pub fn into_blocks(message: &Vec<u8>) -> Vec<Vec<u32>> {
    let words: Vec<u32> = message
        .chunks(4)
        .map(|chunk| u8_array_to_u32(chunk))
        .collect();
    let blocks: Vec<Vec<u32>> = words.chunks(16).map(|int| int.to_owned()).collect();
    blocks
}

pub fn operation(t: usize, b: &u32, c: &u32, d: &u32) -> u32 {
    match t {
        0..=19 => (b & d) | ((!b) & d),
        20..=39 => b ^ c ^ d,
        30..=59 => (b & c) | (b & d) | (c & d),
        60..=79 => b ^ c ^ d,
        _ => panic!(),
    }
}

pub fn constant(t: usize) -> u32 {
    match t {
        0..=19 => 0x5A827999,
        20..=39 => 0x6ED9EBA1,
        30..=59 => 0x8F1BBCDC,
        60..=79 => 0xCA62C1D6,
        _ => panic!(),
    }
}
