// Copyright (c) 2026 HomomorphicEncryption.org
// All rights reserved.
//
// This software is licensed under the terms of the Apache v2 License.
// See the LICENSE.md file for details.

use std::env;
use std::path::Path;
use std::fs;

use tfhe::{ClientKey, FheUint64};
use tfhe::prelude::*;

use zn_multiplication::utils::*;


pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <size>", args[0]);
        std::process::exit(1); 
    }
    let size = args[1].clone();
    let io_dir = "io/".to_owned() + &size;

    // Load the secret key
    let serialised_data = fs::read(io_dir.clone() + "/private_keys/sk.bin")?;
    let lwe_sk: ClientKey = bincode::deserialize(&serialised_data)?;

    // Load the input data (LHS and RHS)
    let lhs_cleartext: Vec<u64> = read_numbers_from_file(Path::new(&("datasets/".to_owned() + &size + "/lhs.txt")))?;
    let rhs_cleartext: Vec<u64> = read_numbers_from_file(Path::new(&("datasets/".to_owned() + &size + "/rhs.txt")))?;
    
    // Encode and encrypt the LHS
    let lhs_ciphers = lhs_cleartext.into_iter().map(|m| FheUint64::encrypt(m, &lwe_sk));
 
    // Write the LHS
    let ciphertexts_dir = io_dir.clone() + "/ciphertexts_upload";
    if !Path::new(&ciphertexts_dir).exists() {
        fs::create_dir(&ciphertexts_dir)?;
    }
    for (i, cipher) in lhs_ciphers.enumerate() {
        fs::write(ciphertexts_dir.clone() + "/cipher_lhs_" + &i.to_string() + ".bin", &bincode::serialize(&cipher)?)?
    }
    
    // Encode and encrypt the RHS
    let rhs_ciphers = rhs_cleartext.into_iter().map(|m| FheUint64::encrypt(m, &lwe_sk));

    // Write the RHS
    for (i, cipher) in rhs_ciphers.enumerate() {
        fs::write(ciphertexts_dir.clone() + "/cipher_rhs_" + &i.to_string() + ".bin", &bincode::serialize(&cipher)?)?
    }

    Ok(())
}
