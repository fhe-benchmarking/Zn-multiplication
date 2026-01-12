#ifndef MUL_PLAINTEXT_H
#define MUL_PLAINTEXT_H

#include <stdint.h>
    
#if defined(__GNUC__) || defined(__clang__) // Check if using GCC or Clang
    typedef unsigned __int128 uint128_t;
#else
    #ifdef __has_feature
        #if __has_feature(c_std_bitint)
            typedef unsigned _BitInt(128) uint128_t;
        #else
            #error "Compiler does not support _BitInt or __int128" // Abort if not GCC/Clang
        #endif
    #else
        #warning "Compiler does not support __has_feature.\nCannot reliably check for _BitInt support."
        typedef unsigned _BitInt(128) uint128_t;
    #endif
#endif

uint64_t half_64b_mul(const uint64_t lhs, const uint64_t rhs);
uint128_t full_64b_mul(const uint64_t lhs, const uint64_t rhs);

#endif // ifndef MUL_PLAINTEXT_H
