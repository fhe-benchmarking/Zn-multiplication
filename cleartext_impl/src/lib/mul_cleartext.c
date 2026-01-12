// Copyright (c) 2025 HomomorphicEncryption.org
// All rights reserved.
//
// This software is licensed under the terms of the Apache v2 License.
// See the LICENSE.md file for details.

#include "mul_cleartext.h"


uint64_t half_64b_mul(const uint64_t lhs, const uint64_t rhs)
{
    return lhs * rhs;
}


uint128_t full_64b_mul(const uint64_t lhs, const uint64_t rhs)
{
    return ((uint128_t) lhs) * ((uint128_t) rhs);
}
