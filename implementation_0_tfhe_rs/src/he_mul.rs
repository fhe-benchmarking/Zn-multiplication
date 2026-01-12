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

pub fn full_cipher_cipher_mul_64(a: &FheUint64, b: &FheUint64) -> (FheUint64, FheUint64)
{
    // Split `a` between low and high bits
    let a_low = a.bitand((1u64 << 32) - 1);
    let a_high = a.shr(32u8);
    
    // Split `b` between low and high bits
    let b_low = b.bitand((1u64 << 32) - 1);
    let b_high = b.shr(32u8);

    // Run the 4 multiplications
    let c_ll = a_low.clone() * b_low.clone();
    let c_lh = a_low * b_high.clone();
    let c_hl = a_high.clone() * b_low;
    let c_hh = a_high * b_high;

    // Sum `c_lh` and `c_hl`
    let (c_m, carry_0) = c_lh.overflowing_add(&c_hl);

    // Split `c_m` between low and high bits
    let c_m_low = c_m.clone().bitand((1u64 << 32) - 1).shl(32u8);
    let c_m_high = c_m.bitand(((1u64 << 32) - 1) << 32).shr(32u8);

    // Low bits
    let (res_low, carry_1) = c_ll.overflowing_add(&c_m_low);

    // High bits withour the carries
    let res_high_0 = c_hh + c_m_high;

    // Add the first carry
    let res_high_1 = carry_0.if_then_else(&(res_high_0.clone() + (1u64 << 32)), &res_high_0);
    
    // Add the second carry
    let res_high = carry_1.if_then_else(&(res_high_1.clone() + 1u64), &res_high_1);

    //(res_low, res_high)
    (res_low, res_high)
}

pub fn full_cipher_cleartext_mul_64(a: &FheUint64, b: u64) -> (FheUint64, FheUint64)
{
    // Split `a` between low and high bits
    let a_low = a.bitand((1u64 << 32) - 1);
    let a_high = a.shr(32u8);
    
    // Split `b` between low and high bits
    let b_low = b.bitand((1u64 << 32) - 1);
    let b_high = b.shr(32u8);

    // Run the 4 multiplications
    let c_ll = a_low.clone() * b_low.clone();
    let c_lh = a_low * b_high.clone();
    let c_hl = a_high.clone() * b_low;
    let c_hh = a_high * b_high;

    // Sum `c_lh` and `c_hl`
    let (c_m, carry_0) = c_lh.overflowing_add(&c_hl);

    // Split `c_m` between low and high bits
    let c_m_low = c_m.clone().bitand((1u64 << 32) - 1).shl(32u8);
    let c_m_high = c_m.bitand(((1u64 << 32) - 1) << 32).shr(32u8);

    // Low bits
    let (res_low, carry_1) = c_ll.overflowing_add(&c_m_low);

    // High bits withour the carries
    let res_high_0 = c_hh + c_m_high;

    // Add the first carry
    let res_high_1 = carry_0.if_then_else(&(res_high_0.clone() + (1u64 << 32)), &res_high_0);
    
    // Add the second carry
    let res_high = carry_1.if_then_else(&(res_high_1.clone() + 1u64), &res_high_1);

    //(res_low, res_high)
    (res_low, res_high)
}
