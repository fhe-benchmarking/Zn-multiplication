#!/usr/bin/env python3

# Copyright (c) 2026 HomomorphicEncryption.org
# All rights reserved.
#
# This software is licensed under the terms of the Apache v2 License.
# See the LICENSE.md file for details.

"""
run_submission.py - run the entire submission process, from build to verify
"""

import subprocess
import sys
import numpy as np
import utils
from params import instance_name, SIZE_BOUND

def main() -> int:
    """
    Run the entire submission process, from build to verify
    """

    if not sys.platform.startswith('linux'):
        print(f"It appears you're running on {platform}; only linux is supported")
        return 1

    # 0. Prepare running
    # Get the arguments
    size, params, seed, num_runs, clrtxt = utils.parse_submission_arguments('Run the 64-bits mul FHE benchmark.')
    test = instance_name(size)
    data_size = str(SIZE_BOUND[size])
    print(f"\n[harness] Running submission for {test} dataset")

    # Ensure the required directories exist
    utils.ensure_directories(params.rootdir)

    # Build the submission if needed
    utils.build_submission(params.rootdir/"scripts")

    # The harness scripts are in the 'harness' directory,
    # the executables are in the directory submission/target/release
    harness_dir = params.rootdir/"harness"
    exec_dir = params.rootdir/"submission"/"target"/"release"

    # Remove and re-create IO directory
    io_dir = params.iodir()
    if io_dir.exists():
        subprocess.run(["rm", "-rf", str(io_dir)], check=True)
    io_dir.mkdir(parents=True)
    utils.log_step(0, "Init", True)

    # 1. Client-side: Generate the datasets
    cmd = ["python3", harness_dir/"generate_dataset.py", str(size)]
    if seed is not None:
        rng = np.random.default_rng(seed)
        gendata_seed = rng.integers(0,0x7fffffff)
        cmd.extend(["--seed", str(gendata_seed)])
    subprocess.run(cmd, check=True)
    utils.log_step(1, "Input generation")

    # 2. Client-side: Preprocess the dataset using exec_dir/client_preprocess_dataset
    subprocess.run([exec_dir/"client_preprocess_input", data_size], check=True)
    utils.log_step(2, "Input preprocessing")

    # 3. Client-side: Generate the cryptographic keys 
    # Note: this does not use the rng seed above, it lets the implementation
    #   handle its own prg needs. It means that even if called with the same
    #   seed multiple times, the keys and ciphertexts will still be different.
    subprocess.run([exec_dir/"client_key_generation", test], check=True)
    utils.log_step(3, "Key Generation")
    utils.log_size(io_dir / "public_keys", "Public and evaluation keys")
    
    # 4. Client-side: Encode and encrypt the dataset
    subprocess.run([exec_dir/"client_encode_encrypt_input", test], check=True)
    utils.log_step(4, "Input encoding and encryption")
    utils.log_size(io_dir / "ciphertexts_upload", "Encrypted input")

    # Run steps 7-9 multiple times if requested
    for run in range(num_runs):
        run_path = params.measuredir() / f"results-{run+1}.json"
        if num_runs > 1:
            print(f"\n         [harness] Run {run+1} of {num_runs}")
    
        # 5. Server side: Run the encrypted processing
        subprocess.run([exec_dir/"server_encrypted_compute", test, data_size], check=True)
        utils.log_step(5, "Encrypted computation")
        utils.log_size(io_dir / "ciphertexts_download", "Encrypted results")

        # 6. Client side: Decrypt
        subprocess.run([exec_dir/"client_decrypt_decode", test, data_size], check=True)
        utils.log_step(6, "Result decryption")
        
        # 7. Client side: Postprocess
        subprocess.run([exec_dir/"client_postprocess", test, data_size], check=True)
        utils.log_step(7, "Result postprocessing")

        # 8. Harness: Verify the results
        expected_file = params.datadir() / "expected.txt"
        result_file = io_dir / "cleartext_output/out.txt"
        subprocess.run(["python3", harness_dir/"verify_result.py",
               str(expected_file), str(result_file)], check=False)

        # 9. Store measurements
        run_path = params.measuredir() / f"results-{run+1}.json"
        run_path.parent.mkdir(parents=True, exist_ok=True)
        utils.save_run(run_path)

    print(f"\nAll steps completed for the {test} dataset!")
    return 0


if __name__ == "__main__" :
    main()
