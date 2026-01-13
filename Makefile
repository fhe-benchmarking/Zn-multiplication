#!/usr/bin/env python3

# Copyright (c) 2025-2026 HomomorphicEncryption.org
# All rights reserved.
#
# This software is licensed under the terms of the Apache v2 License.
# See the LICENSE.md file for details.

.PHONY: clean tests_cleartext tests_impl_0

IMPL_0 = implementation_0_tfhe_rs

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
