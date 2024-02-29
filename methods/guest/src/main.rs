#![no_main]
// If you want to try std support, also update the guest Cargo.toml file

use risc0_zkvm::{
    guest::env,
    sha::{Impl, Sha256},
};

risc0_zkvm::guest::entry!(main);

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm,
    Key, // Or `Aes128Gcm`
    Nonce,
};
use http::{Request, Response, Version};
use httparse::Request as ParsedRequest;
use httparse::Response as ParsedResponse;
use json::parse;
use json_core::Outputs;

fn main() {

    //Test
    // let complete_record = "17 03 03 00 43 a2 3f 70 54 b6 2c 94
    // d0 af fa fe 82 28 ba 55 cb ef ac ea 42 f9 14 aa 66 bc ab 3f 2b
    // 98 19 a8 a5 b4 6b 39 5b d5 4a 9a 20 44 1e 2b 62 97 4e 1f 5a 62
    // 92 a2 97 70 14 bd 1e 3d ea e6 3a ee bb 21 69 49 15 e4";
    // Removed white spaces with: https://www.browserling.com/tools/remove-all-whitespace
    let complete_record = "1703030043a23f7054b62c94d0affafe8228ba55cbefacea42f914aa66bcab3f2b9819a8a5b46b395bd54a9a20441e2b62974e1f5a6292a2977014bd1e3deae63aeebb21694915e4";
    println!("{:?}",complete_record);

    let record_first_three_bytes = &complete_record[0..6];
    println!("{:?}",record_first_three_bytes);

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
