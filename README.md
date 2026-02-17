# FHE Benchmarking Suite - 64-bits multiplication
This repository contains the harness for the 64-bits multiplication workload of the FHE benchmarking suite of [HomomorphicEncryption.org](https://www.HomomorphicEncryption.org).
The harness currently supports ‘half’ multiplication with a 64-bits output.
The `main` branch contains a reference implementation of this workload, under the `submission` subdirectory.

Submitters need to clone this repository, replace the content of the `submission` subdirectory by their own implementation.
They also may need to changes or replace the script `scripts/build_task.sh` to account for dependencies and build environment for their submission.

## Execution Modes

The 64-bits workload currently only support local execution mode:

All steps are executed on a single machine:
- Cryptographic context setup
- Key generation
- Input preprocessing and encryption
- Homomorphic multiplication
- Decryption and postprocessing

## Running the 64-bits multiplication workload

#### Dependencies
- Python 3.12+
- The build environment for local execution depends on the Rust toolchain being installed. See https://rust-lang.org/tools/install/ .

#### Execution
To run the workload, clone and install dependencies:
```console
git clone https://github.com/fhe-benchmarking/Zn-multiplication.git
cd Zn-multiplication

python -m venv virtualenv
source ./virtualenv/bin/activate
pip install -r requirements.txt

python3 harness/run_submission.py -h  # Information about command-line options
```

The harness script `harness/run_submission.py` will attempt to build the submission itself, downloading required Rust crates, if it is not already built. If already built, it will use the same project without re-building it (unless the code has changed). An example run is provided below.


```console
$ python3 harness/run_submission.py -h
usage: run_submission.py [-h] [--num_runs NUM_RUNS] [--seed SEED] [--clrtxt CLRTXT] {0,1,2,3}

Run the 64-bits mul FHE benchmark.

positional arguments:
  {0,1,2,3}            Instance size (0-single/1-small/2-medium/3-large)

options:
  -h, --help           show this help message and exit
  --num_runs NUM_RUNS  Number of times to run steps 4-9 (default: 1)
  --seed SEED          Random seed for dataset generation
  --clrtxt CLRTXT      Specify with 1 if to rerun the cleartext computation
```

The single instance runs the multiplication for a single pair of inputs and verifies the correctness of the result.

```console
$ python3 ./harness/run_submission.py 0 --seed 3 --num_runs 2

[harness] Running submission for single dataset
Build the submission executable...
Rust toolchain already installed.
   Compiling zn_multiplication v0.1.0 (/media/florent/Optalysys/Zn-multiplication/submission)
    Finished `release` profile [optimized] target(s) in 0.61s
Executable built
08:58:21 [harness] 1: Dataset generation completed (elapsed: 0.1762s)
08:58:22 [harness] 2: Key generation completed (elapsed: 0.8709s)
         [harness] Public and evaluation keys size: 125.0M
08:58:22 [harness] 3: Encryption completed (elapsed: 0.0094s)
         [harness] Client: encrypted inputs size: 1.0M

         [harness] Run 1 of 2
08:58:28 [harness] 4: Homomorphic mul completed (elapsed: 6.7748s)
         [harness] Client: encrypted results size: 514.4K
08:58:28 [harness] 5: Decryption completed (elapsed: 0.0087s)
08:58:28 [harness] 6: Checking results completed (elapsed: 0.0038s)
[total latency] 7.8438s

         [harness] Run 2 of 2
08:58:35 [harness] 4: Homomorphic mul completed (elapsed: 6.8825s)
         [harness] Client: encrypted results size: 514.4K
08:58:35 [harness] 5: Decryption completed (elapsed: 0.0075s)
08:58:35 [harness] 6: Checking results completed (elapsed: 0.0006s)
[total latency] 7.9471s

All steps completed for the single dataset!
```

After finishing the run, deactivate the virtual environment.
```console
deactivate
```

## Directory structure

The directory structure of this reposiroty is as follows:
```
├─ README.md     # This file
├─ LICENSE.md    # Harness software license (Apache v2)
├─ harness/      # Scripts to drive the workload implementation
|   ├─ generate_dataset.py
|   ├─ params.py
|   ├─ run_submission.py
|   ├─ utils.py
|   └─ [,,,]
├─ datasets/     # The harness scripts create and populate this directory
├─ docs/         # Optional: additional documentation
├─ io/           # This directory is used for client<->server communication
├─ measurements/ # Holds logs with performance numbers
├─ scripts/      # Helper scripts for dependencies and build system
└─ submission/   # This is where the workload implementation lives
    ├─ README.md   # Submission documentation (mandatory)
    ├─ LICENSE.md  # Optional software license (if different from Apache v2)
    └─ [...]
```
Submitters must overwrite the contents of the `scripts` and `submissions`
subdirectories.

## Description of stages

A submitter can edit any of the files in `/submission`. 
Moreover, for the particular parameters related to a workload, the submitter can modify the `harness/params.py` files.
If the current description of the files are inaccurate, the stage names in `harness/run_submission.py` can be also 
modified.

The order in which they are happening in `run_submission` assumes an initialization step which run only once, and potentially multiple runs for the multiplication.
Each file can take as argument the test case size.

***


| Stage executables                | Description |
|----------------------------------|-------------|
| `client_key_generation`          | Generate all key material and cryptographic context.           
| `client_preprocess_input`        | (Optional) Any in the clear computations the client wants to apply over the input.
| `client_encode_encrypt_input`    | Plaintext encoding and encryption of the input.
| `server_encrypted_compute`       | The computation the server applies to achieve the workload solution over encrypted data.
| `client_decrypt_decode`          | Decryption and plaintext decoding of the result at the client.
| `client_postprocess`             | Any in the clear computation that the client wants to apply on the decrypted result.


The outer python script measures the runtime of each stage.
The current stage separation structure requires reading and writing to files more times than minimally necessary.
For a more granular runtime measuring, which would account for the extra overhead described above, we encourage
submitters to separate and print in a log the individual times for reads/writes and computations inside each stage. 
