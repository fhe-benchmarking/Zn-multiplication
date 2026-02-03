// Copyright (c) 2026 HomomorphicEncryption.org
// All rights reserved.
//
// This software is licensed under the terms of the Apache v2 License.
// See the LICENSE.md file for details.

use std::env;
use std::path::Path;
use std::fs;

use tfhe::{ FheUint64, set_server_key, ServerKey };

use zn_multiplication::half_cipher_cipher_mul_64;

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

    // Load the server key
    let serialised_data = fs::read(io_dir.clone() + "/public_keys/pk.bin")?;
    let server_key: ServerKey = bincode::deserialize(&serialised_data)?;
    set_server_key(server_key);
 
    // Load the LHS input ciphers
    let ciphertexts_in_dir = io_dir.clone() + "/ciphertexts_upload";
    let ciphers_lhs = (0 .. data_size).map(|i|
        bincode::deserialize::<FheUint64>(&fs::read(ciphertexts_in_dir.clone() + "/cipher_lhs_" + &i.to_string() + ".bin")?)
    ).collect::<Result<Vec<FheUint64>, Box<bincode::ErrorKind>>>()?;
    
    // Load the RHS input ciphers
    let ciphers_rhs = (0 .. data_size).map(|i|
        bincode::deserialize::<FheUint64>(&fs::read(ciphertexts_in_dir.clone() + "/cipher_rhs_" + &i.to_string() + ".bin")?)
    ).collect::<Result<Vec<FheUint64>, Box<bincode::ErrorKind>>>()?;

    // Run the homomorphic multiplications
    let ciphers_out = ciphers_lhs.iter().zip(ciphers_rhs.iter())
                                 .map(|(lhs, rhs)| half_cipher_cipher_mul_64(lhs, rhs))
                                 .collect::<Vec<FheUint64>>();

    // Write the results
    let ciphertexts_out_dir = io_dir.clone() + "/ciphertexts_download";
    if !Path::new(&ciphertexts_out_dir).exists() {
        fs::create_dir(&ciphertexts_out_dir)?;
    }
    for (i, cipher) in ciphers_out.iter().enumerate() {
        fs::write(ciphertexts_out_dir.clone() + "/cipher_out_" + &i.to_string() + ".bin", &bincode::serialize(&cipher)?)?
    }

    Ok(())
}
