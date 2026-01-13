// Copyright (c) 2026 HomomorphicEncryption.org
// All rights reserved.
//
// This software is licensed under the terms of the Apache v2 License.
// See the LICENSE.md file for details.

use tfhe::FheUint64;

pub fn half_cipher_cipher_mul_64(a: &FheUint64, b: &FheUint64) -> FheUint64 
{
    a * b
}

pub fn half_cipher_cleartext_mul_64(a: &FheUint64, b: u64) -> FheUint64 
{
    a * b
}
