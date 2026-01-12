#!/usr/bin/env python3

# Copyright (c) 2025 HomomorphicEncryption.org
# All rights reserved.
#
# This software is licensed under the terms of the Apache v2 License.
# See the LICENSE.md file for details.

.PHONY: clean tests_cleartext tests_impl_0

CC := gcc
CLEARTEXT_IMPL = cleartext_impl
IMPL_0 = implementation_0_tfhe_rs

build/half_64b_mul_cleartext_1:\
	${CLEARTEXT_IMPL}/src/include/mul_cleartext.h\
	${CLEARTEXT_IMPL}/src/lib/mul_cleartext.c\
	${CLEARTEXT_IMPL}/tests/half_64b_mul_cleartext_1.c
	mkdir -p build
	$(CC) ${CLEARTEXT_IMPL}/src/lib/mul_cleartext.c\
	      ${CLEARTEXT_IMPL}/tests/half_64b_mul_cleartext_1.c\
		  -o $@\
		  -I${CLEARTEXT_IMPL}/src/include

build/full_64b_mul_cleartext_1:\
	${CLEARTEXT_IMPL}/src/include/mul_cleartext.h\
	${CLEARTEXT_IMPL}/src/lib/mul_cleartext.c\
	${CLEARTEXT_IMPL}/tests/full_64b_mul_cleartext_1.c
	mkdir -p build
	$(CC) ${CLEARTEXT_IMPL}/src/lib/mul_cleartext.c\
	      ${CLEARTEXT_IMPL}/tests/full_64b_mul_cleartext_1.c\
		  -o $@\
		  -I${CLEARTEXT_IMPL}/src/include

tests_cleartext: build/half_64b_mul_cleartext_1 build/full_64b_mul_cleartext_1
	cp -ru ${CLEARTEXT_IMPL}/tests/data ./build/
	cd build && ./half_64b_mul_cleartext_1
	cd build && ./full_64b_mul_cleartext_1

tests_impl_0:
	cd ${IMPL_0} \
	  && cargo build --release \
	  && ./target/release/half_cipher_cleartext_64 \
	  && ./target/release/full_cipher_cleartext_64 \
	  && ./target/release/half_cipher_cipher_64 \
	  && ./target/release/full_cipher_cipher_64

clean: 
	rm -rf build
	rm -rf io
	rm -rf measurements
	cd ${IMPL_0} && cargo clean
	rm -f ${IMPL_0}/Cargo.lock
	rm -rf datasets
	rm -rf harness/__pycache__
