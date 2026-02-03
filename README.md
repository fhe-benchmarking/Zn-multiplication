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

python -m venv zn_mul_env
source ./zn_mul_env/bin/activate
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

***

```console
$ python3 ./harness/run_submission.py 0 --seed 3 --num_runs 2
 

[harness] Running submission for single inference
[get-openfhe] Found OpenFHE at .../ml-inference/third_party/openfhe (use --force to rebuild).
-- FOUND PACKAGE OpenFHE
-- OpenFHE Version: 1.3.1
-- OpenFHE installed as shared libraries: ON
-- OpenFHE include files location: .../ml-inference/third_party/openfhe/include/openfhe
-- OpenFHE lib files location: .../ml-inference/third_party/openfhe/lib
-- OpenFHE Native Backend size: 64
-- Configuring done (0.0s)
-- Generating done (0.0s)
-- Build files have been written to: .../ml-inference/submission/build
[ 12%] Built target client_preprocess_input
[ 25%] Built target client_postprocess
[ 37%] Built target server_preprocess_model
[ 62%] Built target client_key_generation
[ 62%] Built target mlp_encryption_utils
[ 75%] Built target client_encode_encrypt_input
[100%] Built target client_decrypt_decode
[100%] Built target server_encrypted_compute
22:50:49 [harness] 1: Harness: MNIST Test dataset generation completed (elapsed: 7.5552s)
22:50:51 [harness] 2: Client: Key Generation completed (elapsed: 2.2688s)
         [harness] Client: Public and evaluation keys size: 1.4G
22:50:51 [harness] 3: Server: (Encrypted) model preprocessing completed (elapsed: 0.1603s)

         [harness] Run 1 of 2
100.0%
100.0%
100.0%
100.0%
22:51:04 [harness] 4: Harness: Input generation for MNIST completed (elapsed: 13.1305s)
22:51:04 [harness] 5: Client: Input preprocessing completed (elapsed: 0.0489s)
22:51:04 [harness] 6: Client: Input encryption completed (elapsed: 0.0481s)
         [harness] Client: Encrypted input size: 358.8K
         [server] Loading keys
         [server] Run encrypted MNIST inference
         [server] Execution time for ciphertext 0 : 11 seconds
22:51:18 [harness] 7: Server: Encrypted ML Inference computation completed (elapsed: 13.3027s)
         [harness] Client: Encrypted results size: 69.6K
22:51:18 [harness] 8: Client: Result decryption completed (elapsed: 0.1729s)
22:51:18 [harness] 9: Client: Result postprocessing completed (elapsed: 0.0921s)
[harness] PASS  (expected=7, got=7)
[total latency] 36.7796s

         [harness] Run 2 of 2
22:51:23 [harness] 4: Harness: Input generation for MNIST completed (elapsed: 5.2028s)
22:51:23 [harness] 5: Client: Input preprocessing completed (elapsed: 0.0986s)
22:51:23 [harness] 6: Client: Input encryption completed (elapsed: 0.0998s)
         [harness] Client: Encrypted input size: 358.8K
         [server] Loading keys
         [server] Run encrypted MNIST inference
         [server] Execution time for ciphertext 0 : 12 seconds
22:51:37 [harness] 7: Server: Encrypted ML Inference computation completed (elapsed: 13.8138s)
         [harness] Client: Encrypted results size: 69.6K
22:51:37 [harness] 8: Client: Result decryption completed (elapsed: 0.1219s)
22:51:37 [harness] 9: Client: Result postprocessing completed (elapsed: 0.0827s)
[harness] PASS  (expected=7, got=7)
[total latency] 29.4041s

All steps completed for the single inference!
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
|   ├─ run_submission.py
|   ├─ verify_result.py
|   ├─ calculate_quality.py
|   └─ [...]
├─ datasets/     # The harness scripts create and populate this directory
├─ docs/         # Optional: additional documentation
├─ io/           # This directory is used for client<->server communication
├─ measurements/ # Holds logs with performance numbers
├─ scripts/      # Helper scripts for dependencies and build system
└─ submission/   # This is where the workload implementation lives
    ├─ README.md   # Submission documentation (mandatory)
    ├─ LICENSE.md  # Optional software license (if different from Apache v2)
    └─ [...]
└─ submission_remote/  # This is where the remote-backend workload implementation lives
    └─ [...]
```
Submitters must overwrite the contents of the `scripts` and `submissions`
subdirectories.

## Description of stages

A submitter can edit any of the `client_*` / `server_*` sources in `/submission`. 
Moreover, for the particular parameters related to a workload, the submitter can modify the params files.
If the current description of the files are inaccurate, the stage names in `run_submission` can be also 
modified.

The current stages are the following, targeted to a client-server scenario.
The order in which they are happening in `run_submission` assumes an initialization step which is 
database-dependent and run only once, and potentially multiple runs for multiple queries.
Each file can take as argument the test case size.


| Stage executables                | Description |
|----------------------------------|-------------|
| `server_get_params`              | (Optional) Get cryptographic context from a remote server.
| `client_key_generation`          | Generate all key material and cryptographic context at the client.           
| `server_upload_ek`               | (Optional) Upload evaluation key to a remote backend.
| `client_preprocess_dataset`      | (Optional) Any in the clear computations the client wants to apply over the dataset/model.
| `client_preprocess_input`        | (Optional) Any in the clear computations the client wants to apply over the input.
| `client_encode_encrypt_query`    | Plaintext encoding and encryption of the input at the client.
| `server_preprocess_model`        | (Optional) Any in the clear or encrypted computations the server wants to apply over the model.
| `server_encrypted_compute`       | The computation the server applies to achieve the workload solution over encrypted data.
| `client_decrypt_decode`          | Decryption and plaintext decoding of the result at the client.
| `client_postprocess`             | Any in the clear computation that the client wants to apply on the decrypted result.


The outer python script measures the runtime of each stage.
The current stage separation structure requires reading and writing to files more times than minimally necessary.
For a more granular runtime measuring, which would account for the extra overhead described above, we encourage
submitters to separate and print in a log the individual times for reads/writes and computations inside each stage. 
