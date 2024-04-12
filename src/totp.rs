// use std::intrinsics::mir::BasicBlock;

// use crate::sha1;

// pub fn hmac_sha1(key: &[u8], message: &[u8]) -> Vec<u8> {
//     const opad: u32 = 0x5C;
//     const ipad: u32 = 0x36;
//     const block_size: usize = 64;

//     let key = match key.len() {
//       block_size => key,
//        ..block_size => sha1::sha_1(key),
//       block_size.. => 2
//     }

//     vec![8u8]
// }
