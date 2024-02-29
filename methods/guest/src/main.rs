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
    let data: String = env::read();
    let sha = *Impl::hash_bytes(&data.as_bytes());

    // Key line - parsing json
    let data = parse(&data).unwrap();
    //let raw_data = &data["obj_field"]["string_subfield"];
    let raw_data = &data["obj_field"]["array_subfield"][1];

    let proven_val = raw_data.as_str().unwrap().to_string();
    let out = Outputs {
        data: proven_val,
        hash: sha,
    };
    env::commit(&out);
}
