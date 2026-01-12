#include "mul_plaintext.h"


uint64_t half_64b_mul(const uint64_t lhs, const uint64_t rhs)
{
    return lhs * rhs;
}


uint128_t full_64b_mul(const uint64_t lhs, const uint64_t rhs)
{
    return ((uint128_t) lhs) * ((uint128_t) rhs);
}
