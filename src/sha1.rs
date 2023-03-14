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

// Method 1
pub fn sha_1(input: &[u32]) {
    let mut h0: u32 = 0x67452301;
    let mut h1: u32 = 0xEFCDAB89;
    let mut h2: u32 = 0x98BADCFE;
    let mut h3: u32 = 0x10325476;
    let mut h4: u32 = 0xC3D2E1F0;


    if need_padding(input) {
      apply_padding(input);
    } else{

    }
}

pub fn need_padding(input: &[u32]) -> bool {
  input.len() % 512 == 0
}

pub fn calculate_padding_length(input_length: u32) -> u32 {
  512 - (input_length % 512) 
}
 
pub fn apply_padding(input: &[u32]) -> [u32]{

}

pub fn process_m(block: &[u32], mut h0: u32, mut h1: u32, mut h2: u32, mut h3: u32, mut h4: u32) {
  let words = block.sp
}
