#![no_main]
// If you want to try std support, also update the guest Cargo.toml file

use risc0_zkvm::{
    guest::env,
    sha::{Impl, Sha256},
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
    associated_data_vec: &Vec<u8>,
    iv_expanded: &[u8; 12],
    key_expanded: &[u8; 16],
) {
    let cipher = Aes128Gcm::new(key_expanded.into());
    let nonce = iv_expanded;
    let success = cipher.decrypt_in_place(nonce.into(), associated_data_vec, ciphertext_vec);
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
    let message = Message::Client(hex::decode("1703030043a23f7054b62c94d0affafe8228ba55cbefacea42f914aa66bcab3f2b9819a8a5b46b395bd54a9a20441e2b62974e1f5a6292a2977014bd1e3deae63aeebb21694915e4").unwrap());
    let key_data = KeyData {
        iv_expanded: [
            0x5b, 0x78, 0x92, 0x3d, 0xee, 0x08, 0x57, 0x90, 0x33, 0xe5, 0x23, 0xd9,
        ],
        key_expanded: [
            0x17, 0x42, 0x2d, 0xda, 0x59, 0x6e, 0xd5, 0xd9, 0xac, 0xd8, 0x90, 0xe3, 0xc6, 0x3f,
            0x50, 0x51,
        ],
    };

    // Decoding message example
    let ciphertext = "a23f7054b62c94d0affafe8228ba55cbefacea42f914aa66bcab3f2b9819a8a5b46b395bd54a9a20441e2b62974e1f5a6292a2977014bd1e3deae63aeebb21694915e4";
    let associated_data = "1703030043";
    let mut ciphertext_vec = hex::decode(ciphertext).unwrap();
    let associated_data_vec = hex::decode(associated_data).unwrap();

    decode_message(
        &mut ciphertext_vec,
        &associated_data_vec,
        &key_data.iv_expanded,
        &key_data.key_expanded,
    );
    println!("RESULT:");
    println!("{:?}", ciphertext_vec);

    // IV XOR example
    let sequence_num: u64 = 1234567890;
    let xored: [u8; 12] = do_iv_xor(sequence_num, &key_data.iv_expanded);
    println!("RESULT:");
    println!("{:?}", xored);

    //Test
    // let complete_record = "17 03 03 00 43 a2 3f 70 54 b6 2c 94
    // d0 af fa fe 82 28 ba 55 cb ef ac ea 42 f9 14 aa 66 bc ab 3f 2b
    // 98 19 a8 a5 b4 6b 39 5b d5 4a 9a 20 44 1e 2b 62 97 4e 1f 5a 62
    // 92 a2 97 70 14 bd 1e 3d ea e6 3a ee bb 21 69 49 15 e4";
    // Removed white spaces with: https://www.browserling.com/tools/remove-all-whitespace
    let complete_record = "1703030043a23f7054b62c94d0affafe8228ba55cbefacea42f914aa66bcab3f2b9819a8a5b46b395bd54a9a20441e2b62974e1f5a6292a2977014bd1e3deae63aeebb21694915e4";
    println!("{:?}", complete_record);

    let record_first_three_bytes = &complete_record[0..6];
    println!("{:?}", record_first_three_bytes);

    if record_first_three_bytes != "170303" {
        println!("ERROR: Encoded hex string does not start with 170303.");
        return;
    }
    println!("Encoded hex string starts with 170303 as expected.");

    let decoded_string = hex::decode(complete_record);
    println!("{:?}", decoded_string); //

    //Input
    let data: String = env::read();
    let sha = *Impl::hash_bytes(&data.as_bytes());

    // Key line - parsing json
    let data = parse(&data).unwrap();
    //let raw_data = &data["obj_field"]["string_subfield"];
    let raw_data = &data["obj_field"]["array_subfield"][1];

    // Submit proof
    let proven_val = raw_data.as_str().unwrap().to_string();
    let out = Outputs {
        data: proven_val,
        hash: sha,
    };
    env::commit(&out);
}
