// Copyright (c) 2025 HomomorphicEncryption.org
// All rights reserved.
//
// This software is licensed under the terms of the Apache v2 License.
// See the LICENSE.md file for details.

use std::env;
use std::fs;
use tfhe::{ConfigBuilder, generate_keys};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <size>", args[0]);
        std::process::exit(1); 
    }
    let size = args[1].clone();
    let io_dir = "io/".to_owned() + &size;
    
    // Generate the keys
    let config = ConfigBuilder::default().build();
    let (client_key, server_key) = generate_keys(config);

    // Serialize and save the secret key
    let serialised_data = bincode::serialize(&client_key)?;
    fs::write(io_dir.clone() + "/sk.bin", &serialised_data)?;
    
    // Serialize and save the public key
    let serialised_data = bincode::serialize(&server_key)?;
    fs::write(io_dir.clone() + "/pk.bin", &serialised_data)?;

    Ok(())
}
