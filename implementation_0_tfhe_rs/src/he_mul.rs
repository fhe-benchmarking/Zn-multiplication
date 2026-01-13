// Copyright (c) 2025 HomomorphicEncryption.org
// All rights reserved.
//
// This software is licensed under the terms of the Apache v2 License.
// See the LICENSE.md file for details.

use tfhe::prelude::*;
use tfhe::FheUint64;
use std::ops::{ Shr, Shl, BitAnd };

pub fn half_cipher_cipher_mul_64(a: &FheUint64, b: &FheUint64) -> FheUint64 
{
    a * b
}

pub fn half_cipher_cleartext_mul_64(a: &FheUint64, b: u64) -> FheUint64 
{
    a * b
}
