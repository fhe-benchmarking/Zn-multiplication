// Copyright (c) 2025 HomomorphicEncryption.org
// All rights reserved.
//
// This software is licensed under the terms of the Apache v2 License.
// See the LICENSE.md file for details.

use implementation_0_tfhe_rs::full_cipher_cleartext_mul_64;
use rand::random;
use std::time::Instant;

use tfhe::{ConfigBuilder, generate_keys, set_server_key, FheUint64};
use tfhe::prelude::*;

const NUM_RUNS: usize = 100;

fn main() {
    println!("Full 64b Cipher/Cleartext multiplication test...");
    
    let config = ConfigBuilder::default().build();

    // Client-side
    let (client_key, server_key) = generate_keys(config);
    set_server_key(server_key);

    let start = Instant::now();
    for _ in 0 .. NUM_RUNS {

        let clear_a: u64 = random();
        let clear_b: u64 = random();

        let a = FheUint64::encrypt(clear_a, &client_key);

        let result = full_cipher_cleartext_mul_64(&a, clear_b);
        let decrypted_result: u128 = (tfhe::prelude::FheDecrypt::<u64>::decrypt(&result.0, &client_key) as u128) 
            | ((tfhe::prelude::FheDecrypt::<u64>::decrypt(&result.1, &client_key) as u128) << 64);
        let clear_result = (clear_a as u128) * (clear_b as u128);

        assert_eq!(decrypted_result, clear_result);
    }
    let duration = start.elapsed();

    println!("Test passed with {NUM_RUNS} muliplications");
    println!("Runtime: {:?}", duration);
}
