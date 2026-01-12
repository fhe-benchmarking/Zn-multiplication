#!/bin/sh

# Copyright (c) 2025 HomomorphicEncryption.org
# All rights reserved.
#
# This software is licensed under the terms of the Apache v2 License.
# See the LICENSE.md file for details.

# Install the Rust toolchain if needed
if [ "$(command -v rustc)" ]; then
  echo "Rust toolchain already installed.";
else
  echo "Installing the Rust toolchain...";
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh;
fi

# Build he submission
cd implementation_0_tfhe_rs
cargo build --release
