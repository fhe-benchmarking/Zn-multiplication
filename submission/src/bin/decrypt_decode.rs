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
    // Get the number of inputs from the first argument
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <size> <data size>", args[0]);
        std::process::exit(1); 
    }
    let size = args[1].clone();
    let data_size = args[2].parse::<usize>()?;
    let io_dir = "io/".to_owned() + &size;

    // Load the secret key
    let serialised_data = fs::read(io_dir.clone() + "/private_keys/sk.bin")?;
    let lwe_sk: ClientKey = bincode::deserialize(&serialised_data)?;

    // Load the output ciphers
    let ciphers_out = (0 .. data_size).map(|i|
        bincode::deserialize::<FheUint64>(&fs::read(&(io_dir.clone() + "/ciphertexts_download/cipher_out_" + &i.to_string() + ".bin"))?)
    ).collect::<Result<Vec<FheUint64>, Box<bincode::ErrorKind>>>()?;

    // Decrypt them
    let results = ciphers_out.iter().map(|c| c.decrypt(&lwe_sk)).collect::<Vec<u64>>();

    // Write the results
    let cleartext_output_dir = io_dir.clone() + "/cleartext_output";
    if !Path::new(&cleartext_output_dir).exists() {
        fs::create_dir(cleartext_output_dir.clone())?;
    }
    write_numbers_to_file(&Path::new(&(cleartext_output_dir.clone() + "/out.txt")), &results)?;

    Ok(())
}
