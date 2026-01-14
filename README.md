# Zn-multiplication

Harness and example implementation for the FHE Zn-multiplication workload

## Scope

This repository contains a reference implementation of encrypted 64-bits multiplication for the Homomorphic Encryption benchmarking group.

See the [integer multiplication benchmark document](https://docs.google.com/document/d/1HPHmBfDscTtQAiRGlYV3upykSgK_GOkTg-_Sy41SSAQ/edit?usp=sharing) for more information.

## Test harness

The directory `harness` contains a prototype test harness.

### Generating cleartext input/output data

The script `generate_dataset` generates input and output data, saved in the directoty `datasets`.

**Usage:**

```
python3 ./harness/generate_dataset.py [-h] [--num_runs NUM_RUNS] [--seed SEED] [--clrtxt CLRTXT] {0,1,2,3}
```

For instance, running `python3 ./harness/generate_dataset.py -h` prints a help message detailing the different flags.

### Running the test harness for implementation 0

The test harness can be run for implementation 0 (using the [TFHE-rs](https://docs.zama.ai/tfhe-rs) library) using

```
python3 ./harness/run_submission.py {0,1,2,3}
```

## Reference homomorphic implementation

The reference implementation can be found in `implementation_0_tfhe_rs`. It uses the [tfhe-rs](https://github.com/zama-ai/tfhe-rs) library.

To build the executables, install the [Rust compiler](https://www.rust-lang.org/) and run:

```
cd implementation_0_tfhe_rs && RUSTFLAGS="-Ctarget-cpu=native" cargo build --release
```

The benchmarks can then be run using

```
cd implementation_0_tfhe_rs && ./target/release/half_cipher_cleartext_64
```

and

```
cd implementation_0_tfhe_rs && ./target/release/half_cipher_cleartext_64
```

## Reference implementation benchmark results

The following results were obtained on an Intel(R) Core(TM) i7-9700K CPU @ 3.60GHz.

|         Benchmark          | Number of multiplications | Runtime (s) |
| :------------------------: | :-----------------------: | :---------: |
| `half_cipher_cleartext_64` |            100            |     140     |
|  `half_cipher_cipher_64`   |            100            |     570     |
