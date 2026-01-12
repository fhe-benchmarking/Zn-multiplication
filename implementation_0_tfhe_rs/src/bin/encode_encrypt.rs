// Copyright (c) 2025 HomomorphicEncryption.org
// All rights reserved.
//
// This software is licensed under the terms of the Apache v2 License.
// See the LICENSE.md file for details.

use std::env;
use std::fs;

use tfhe::{ClientKey, FheUint64};
use tfhe::prelude::*;

use implementation_0_tfhe_rs::utils::*;


pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <size>", args[0]);
        std::process::exit(1); 
    }
    let size = args[1].clone();
    let io_dir = "io/".to_owned() + &size;

    // Load the secret key
    let serialised_data = fs::read(io_dir.clone() + "/sk.bin")?;
    let lwe_sk: ClientKey = bincode::deserialize(&serialised_data)?;

    // Load the input data (LHS and RHS)
    let lhs_cleartext: Vec<u64> = read_numbers_from_file(&Path::new(&("datasets/".to_owned() + &size + "/lhs.txt")))?;
    let rhs_cleartext: Vec<u64> = read_numbers_from_file(&Path::new(&("datasets/".to_owned() + &size + "/rhs.txt")))?;
    
    // Encode and encrypt the LHS
    let lhs_ciphers = lhs_cleartext.into_iter().map(|m| FheUint64::encrypt(m, &lwe_sk));
 
    // Write the LHS
    for (i, cipher) in lhs_ciphers.enumerate() {
        fs::write(io_dir.clone() + "/cipher_lhs_" + &i.to_string() + ".bin", &bincode::serialize(&cipher)?)?
    }
    
    // Encode and encrypt the RHS
    let rhs_ciphers = rhs_cleartext.into_iter().map(|m| FheUint64::encrypt(m, &lwe_sk));

    // Write the RHS
    for (i, cipher) in rhs_ciphers.enumerate() {
        fs::write(io_dir.clone() + "/cipher_rhs_" + &i.to_string() + ".bin", &bincode::serialize(&cipher)?)?
    }

    Ok(())
}
