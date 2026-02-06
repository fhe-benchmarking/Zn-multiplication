#!/usr/bin/env python3

# Copyright (c) 2026 HomomorphicEncryption.org
# All rights reserved.
#
# This software is licensed under the terms of the Apache v2 License.
# See the LICENSE.md file for details.

"""
run_submission.py - run the entire submission process, from build to verify
"""

import numpy
import subprocess
import utils
from params import instance_name, SIZE_BOUND
from sys import platform

def main() -> int:
    """
    Run the entire submission process, from build to verify
    """

    if not platform.startswith('linux'):
        print(f"It appears you're running on {platform}; only linux is supported")
        return 1

    # 0. Prepare running
    # Get the arguments
    size, params, seed, num_runs, clrtxt = utils.parse_submission_arguments('Run the 64-bits mul FHE benchmark.')
    test = instance_name(size)
    print(f"\n[harness] Running submission for {test} dataset")

    # Ensure the required directories exist
    utils.ensure_directories(params.rootdir)

    # Build the submission if needed
    print("Build the submission executable...")
    utils.build_submission(params.rootdir/"scripts")
    print("Executable built")

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

    # 1. Harness: Generate the datasets
    cmd = ["python3", harness_dir/"generate_dataset.py", str(size)]
    if seed is not None:
        cmd.extend(["--seed", str(seed)])
    if clrtxt is not None:
        cmd.extend(["--clrtxt", str(clrtxt)])
    subprocess.run(cmd, check=True)
    utils.log_step(1, "Dataset generation")

    # 2 Client side: Generate the keys
    cmd = [exec_dir/"client_key_generation", test]
    subprocess.run(cmd, check=True)
    utils.log_step(2, "Client: Key Generation")
    utils.log_size(io_dir / "public_keys", "Public and evaluation keys")
    
    # 3.1 Client side: Preprocess
    cmd = [exec_dir/"client_preprocess_input", test]
    subprocess.run(cmd, check=True)
    utils.log_step(3.1, "Client: Input preprocessing")

    # 3.2 Client side: Encode and encrypt the dataset
    cmd = [exec_dir/"client_encode_encrypt_input", test]
    subprocess.run(cmd, check=True)
    utils.log_step(3.2, "Client: Encryption")
    utils.log_size(io_dir / "ciphertexts_upload", "Client: encrypted inputs")

    # Run steps 4-6 multiple times if requested
    for run in range(num_runs):
        run_path = params.measuredir() / f"results-{run+1}.json"
        if num_runs > 1:
            print(f"\n         [harness] Run {run+1} of {num_runs}")
    
        # 4. Server side: Run the encrypted processing
        cmd = [exec_dir/"server_encrypted_compute", test, str(SIZE_BOUND[size])]
        subprocess.run(cmd, check=True)
        utils.log_step(4, "Server: Homomorphic mul")
        utils.log_size(io_dir / "ciphertexts_download", "Client: encrypted results")

        # 5. Client side: Decrypt
        cmd = [exec_dir/"client_decrypt_decode", test, str(SIZE_BOUND[size])]
        subprocess.run(cmd, check=True)
        utils.log_step(5, "Client: Result decryption")
        
        # 6. Client side: Postprocess
        cmd = [exec_dir/"client_postprocess", test, str(SIZE_BOUND[size])]
        subprocess.run(cmd, check=True)
        utils.log_step(6, "Client: Result postprocessing")

        # 7. Harness: Check the results
        expected = numpy.loadtxt("datasets/" + test + "/expected.txt")
        out = numpy.loadtxt("io/" + test + "/cleartext_output/out.txt")
        assert (expected == out).all()
        utils.log_step(7, "Checking results")

        # 8. Store measurements
        run_path = params.measuredir() / f"results-{run+1}.json"
        run_path.parent.mkdir(parents=True, exist_ok=True)
        utils.save_run(run_path)

    print(f"\nAll steps completed for the {test} dataset!")
    return 0


if __name__ == "__main__" :
    main()
