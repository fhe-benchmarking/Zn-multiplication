# Workload implementation—64-bits multiplication

This is a reference implementation for the 64-bits multiplication workload, written in [Rust](https://rust-lang.org/) and using the [TFHE-rs](https://docs.zama.org/tfhe-rs) library.

## Security and Parameters

The reference implementation uses the default parameter set of TFHE-rs, version 1.5.4. According to the [TFHE-rs documentation](https://docs.zama.org/tfhe-rs/1.5/get-started/security-and-cryptography#security), these parameters provide 128 bits of security in the IND-CPA-D model: 
* The lowest attack cost as estimated by the [Lattice Estimator](https://github.com/malb/lattice-estimator) is above $2^{128}$.
* The probability of decryption failure after a programmable bootstrap using the [drift reduction technique](https://eprint.iacr.org/2024/1718.pdf) is below $2^{-128}$.

## Build and run

Building the workflow requires the [Rust toolchain](https://rust-lang.org/tools/install/). 

**Build**

Run 
```
cargo build
```
for a debug build or
```
cargo build --release
```
for a release build.

