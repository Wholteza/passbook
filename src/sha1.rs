// Implemented followin rfc3174
// https://datatracker.ietf.org/doc/html/rfc3174#section-6.1

// hex digit = 4 bit string
// word = 32 bit string, unsigned 32 bit integer in hex formatting really
// if z = u64, z can represent a pair of x and y if x and y are both u32 respectivly
//// hex value of least significance is to the right
// block = 512 bit string, and can be represented as a sequence of 16 words

// & bitwise and
// | bitwise or
// ^ bitwise xor
// ! bitwise complement

// << left shift
// >> right shift

// We add padding if the message length, which is the number of bytes, is not n * 512.
// padding is added by a "1" followed by m amount of "0"s followed by a 64 bit integer appended at the end
// the padding produces a message length of 512 * n which can then be processed as sha-1 as part of a 512 bit block
// the 64 bit integer must represent the original message lenght.

// Functions
// Each function f(t); where 0 <= t <= 79; operates on 3 words and produces one word as output
// ( 0 <= t <= 19) f(t;B;C;D) = (B & D) | ((!B) & D)
// (20 <= t <= 39) f(t;B;C;D) = B ^ C ^ D
// (40 <= t <= 59) f(t;B;C;D) = (B & C) | (B & D) | (C & D)
// (60 <= t <= 79) f(t;B;C;D) = (B & D) | ((!B) & D)

// constant words
// for K(t); where 0 <= t <= 79;
// ( 0 <= t <= 19) K(t) = 0x5A827999
// (20 <= t <= 39) K(t) = 0x6ED9EBA1
// (40 <= t <= 59) K(t) = 0x8F1BBCDC
// (60 <= t <= 79) K(t) = 0xCA62C1D6

use std::array;

// Method 1
const PADDING_MIN_LENGTH: usize = 9;
const BLOCK_BYTE_SIZE: usize = 64;
const PADDING_END_BYTE_SIZE: usize = 8;
const PADDING_START_BYTE_SIZE: usize = 1;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn padding_is_not_needed() {
        assert_eq!(calculate_padding_length_in_bytes(BLOCK_BYTE_SIZE), 0)
    }
    #[test]
    fn less_than_block_size_padding_is_needed() {
        const LENGTH: usize = BLOCK_BYTE_SIZE - PADDING_MIN_LENGTH;
        let padding_length = calculate_padding_length_in_bytes(LENGTH);
        assert!(padding_length < BLOCK_BYTE_SIZE)
    }
    #[test]
    fn more_than_block_size_padding_is_needed() {
        const LENGTH: usize = BLOCK_BYTE_SIZE - PADDING_MIN_LENGTH + 1;
        let padding_length = calculate_padding_length_in_bytes(LENGTH);
        assert!(padding_length > BLOCK_BYTE_SIZE)
    }

    // #[test]
    // fn applied_padding_results_in_n_times_blocksize_length() {
    //     for index in 0..(BLOCK_BYTE_SIZE * 10) {
    //         const PADDING_LENGTH_IN_BYTES: usize = BLOCK_BYTE_SIZE - 1;
    //         let input = vec![0xFF; index];
    //         let padded_input = apply_padding(&input, PADDING_LENGTH_IN_BYTES);
    //         let padded_input_length = padded_input.len();
    //         assert_eq!(padded_input_length % BLOCK_BYTE_SIZE, 0);
    //     }
    // }
}

// pub fn sha_1(input: &[u8]) {
//     let mut h0: u32 = 0x67452301;
//     let mut h1: u32 = 0xEFCDAB89;
//     let mut h2: u32 = 0x98BADCFE;
//     let mut h3: u32 = 0x10325476;
//     let mut h4: u32 = 0xC3D2E1F0;

//     let padding_length = calculate_padding_length_in_bytes(input.len());
//     let needs_padding = padding_length != 0;

//     let input = match needs_padding {
//       false => input,
//       true => apply_padding(input, padding_length)
//     };

//    // start unit testing step by step

// }

pub fn calculate_padding_length_in_bytes(input_length: usize) -> usize {
    match (input_length) % BLOCK_BYTE_SIZE {
        0 => 0,
        bytes_populated_in_last_block => {
            match (BLOCK_BYTE_SIZE - bytes_populated_in_last_block) < PADDING_MIN_LENGTH {
                true => (BLOCK_BYTE_SIZE * 2) - bytes_populated_in_last_block,
                false => BLOCK_BYTE_SIZE - bytes_populated_in_last_block,
            }
        }
    }
}

pub fn apply_padding(input: &Vec<u8>, padding_length_in_bytes: usize) -> Vec<u8> {
    let mut buf = vec![];
    for word in input {
        buf.push(word.to_owned());
    }

    buf.push(0x80);

    let bytes_of_zeros_to_apply =
        padding_length_in_bytes - (PADDING_END_BYTE_SIZE + PADDING_START_BYTE_SIZE);
    for _ in 0..bytes_of_zeros_to_apply {
        buf.push(0x00)
    }

    // append "u64 length"
    let len = u64::try_from(input.len()).expect("error");
    for byte in len.to_be_bytes() {
        buf.push(byte);
    }
    buf
}

// pub fn process_m(block: &[u32], mut h0: u32, mut h1: u32, mut h2: u32, mut h3: u32, mut h4: u32) {
//   let words = block.sp
// }
