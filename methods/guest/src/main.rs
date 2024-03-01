#![no_main]
// If you want to try std support, also update the guest Cargo.toml file

use risc0_zkvm::{
    guest::env,
    sha::{Digest, Impl},
};

risc0_zkvm::guest::entry!(main);
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    aes::cipher::BlockCipher,
    AeadInPlace, Aes128Gcm, Aes256Gcm, Key, Nonce,
};
use hex;
use http::{Request, Response, Version};
use httparse::Request as ParsedRequest;
use httparse::Response as ParsedResponse;
use json::parse;
use json_core::Outputs;
// use sha2::{Digest, Sha256, Sha512};
use sha2::{Digest as _, Sha256};

enum Message {
    Client(Vec<u8>), // message like we had earlier
    Server(Vec<u8>), //same thing
}

struct KeyData {
    iv_expanded: [u8; 12],
    key_expanded: [u8; 16],
}

// Decodes IN PLACE - mutates the input ciphertext
fn decode_message(
    ciphertext_vec: &mut Vec<u8>,
    metadata_vec: &Vec<u8>,
    iv_expanded: &[u8; 12],
    key_expanded: &[u8; 16],
) {
    let cipher = Aes128Gcm::new(key_expanded.into());
    let nonce = iv_expanded;
    let success = cipher.decrypt_in_place(nonce.into(), metadata_vec, ciphertext_vec);
    assert!(success.is_ok());
}

fn do_iv_xor(sequence_num: u64, iv_expanded: &[u8; 12]) -> [u8; 12] {
    // take sequence number
    // treat it as a 64 bit number, turn it into bytes big endian - pad to the left with 0s, and then xor with iv
    let mut sequence_num_padded = [0u8; 12]; // Pad with 4 bytes
    sequence_num_padded[4..].copy_from_slice(&sequence_num.to_be_bytes()); // Copy the original bytes to the right side

    let mut result = [0u8; 12];
    for i in 0..12 {
        result[i] = sequence_num_padded[i] ^ iv_expanded[i];
    }

    result
}

fn main() {
    let message_ = Message::Client(hex::decode("1703030043a23f7054b62c94d0affafe8228ba55cbefacea42f914aa66bcab3f2b9819a8a5b46b395bd54a9a20441e2b62974e1f5a6292a2977014bd1e3deae63aeebb21694915e4").unwrap());
    let key_data_client = KeyData {
        iv_expanded: [
            0x5b, 0x78, 0x92, 0x3d, 0xee, 0x08, 0x57, 0x90, 0x33, 0xe5, 0x23, 0xd9,
        ],
        key_expanded: [
            0x17, 0x42, 0x2d, 0xda, 0x59, 0x6e, 0xd5, 0xd9, 0xac, 0xd8, 0x90, 0xe3, 0xc6, 0x3f,
            0x50, 0x51,
        ],
    };
    let key_data_server = KeyData {
        iv_expanded: [
            0x5b, 0x78, 0x92, 0x3d, 0xee, 0x08, 0x57, 0x90, 0x33, 0xe5, 0x23, 0xd9,
        ],
        key_expanded: [
            0x17, 0x42, 0x2d, 0xda, 0x59, 0x6e, 0xd5, 0xd9, 0xac, 0xd8, 0x90, 0xe3, 0xc6, 0x3f,
            0x50, 0x51,
        ],
    };

    let message_vec = vec![message_];
    let mut hasher = Sha256::new();
    // let digest = Sha256::digest(&data.as_bytes());

    for message in &message_vec {
        // let digest = Digest::try_from(digest.as_slice()).unwrap();
        //Sha256::hash_pair(a, b)

        // Need to separate actual cyphertext and metadata
        // let ciphertext = "a23f7054b62c94d0affafe8228ba55cbefacea42f914aa66bcab3f2b9819a8a5b46b395bd54a9a20441e2b62974e1f5a6292a2977014bd1e3deae63aeebb21694915e4";
        // let associated_data = "1703030043";
        let mut ciphertext_vec;
        let metadata_vec;
        let key_data;
        match message {
            Message::Client(ciphertext) => {
                ciphertext_vec = ciphertext[5..].to_vec();
                metadata_vec = ciphertext[0..5].to_vec();
                key_data = &key_data_client;
            }
            Message::Server(ciphertext) => {
                ciphertext_vec = ciphertext[5..].to_vec();
                metadata_vec = ciphertext[0..5].to_vec();
                key_data = &key_data_server;
            }
        }

        // Perform checks before decrypting:
        // First check - initial 3 bytes should always be (hex string) "170303"
        let record_first_three_bytes = &metadata_vec[0..3];
        let record_length_bytes = &metadata_vec[3..5];
        assert!(record_first_three_bytes == [23, 3, 3]);
        println!("Encoded hex string starts with 170303 as expected.");

        // Second check - length should match the length of the ciphertext
        let mut flag_len: u16 = 0;
        for byte in record_length_bytes {
            flag_len = (flag_len << 8) | *byte as u16;
        }
        assert!(flag_len as usize == ciphertext_vec.len());

        // Now that we've confirmed message is valid, we can decode
        decode_message(
            &mut ciphertext_vec,
            &metadata_vec,
            &key_data.iv_expanded,
            &key_data.key_expanded,
        );
        println!("RESULT:");
        println!("{:?}", ciphertext_vec);

        let bytes: &[u8] = ciphertext_vec.as_slice();
        hasher.update(bytes);

        // IV XOR example
        let sequence_num: u64 = 1234567890;
        let xored: [u8; 12] = do_iv_xor(sequence_num, &key_data.iv_expanded);
        println!("RESULT:");
        println!("{:?}", xored);
    }

    ////////// Tested input
    //Input
    let data: String = env::read();
    // let sha = *Impl::hash_bytes(&data.as_bytes());

    // Key line - parsing json
    let data = parse(&data).unwrap();
    //let raw_data = &data["obj_field"]["string_subfield"];
    let raw_data = &data["obj_field"]["array_subfield"][1];

    // Submit proof
    let proven_val = raw_data.as_str().unwrap().to_string();
    let out = Outputs { data: proven_val };

    // read hash digest and consume hasher
    let result = hasher.finalize();
    println!("OUR HASHED RESULT");
    println!("{:?}", result);
    // env::commit(&result);

    env::commit(&out);
}
