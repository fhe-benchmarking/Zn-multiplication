#!/usr/bin/env python3

# Copyright (c) 2025 HomomorphicEncryption.org
# All rights reserved.
#
# This software is licensed under the terms of the Apache v2 License.
# See the LICENSE.md file for details.

import argparse
import json
import subprocess
from datetime import datetime
from params import InstanceParams, TOY, LARGE
from pathlib import Path
from typing import Tuple

# Global variable to track the last timestamp
_last_timestamp: datetime = None
# Global variable to store measured times
_timestamps = {}
_timestampsStr = {}
# Global variable to store measured sizes
_bandwidth = {}

SUBMISSION_NAME = 'implementation_0_tfhe_rs'

def parse_submission_arguments(workload: str) -> Tuple[int, InstanceParams, int, int, int]:
    """
    Get the arguments of the submission. Populate arguments as needed for the workload.
    """
    # Parse arguments using argparse
    parser = argparse.ArgumentParser(description=workload)
    parser.add_argument('size', type=int, choices=range(TOY, LARGE+1),
                        help='Instance size (0-toy/1-small/2-medium/3-large)')
    parser.add_argument('--num_runs', type=int, default=1,
                        help='Number of times to run steps 4-9 (default: 1)')
    parser.add_argument('--seed', type=int,
                        help='Random seed for dataset generation')
    parser.add_argument('--clrtxt', type=int,
                        help='Specify with 1 if to rerun the cleartext computation')

    args = parser.parse_args()
    size = args.size
    seed = args.seed
    num_runs = args.num_runs
    clrtxt = args.clrtxt

    # Use params.py to get instance parameters
    params = InstanceParams(size)
    return size, params, seed, num_runs, clrtxt

def ensure_directories(rootdir: Path):
    """ Check that the current directory has sub-directories
    'harness', 'scripts', and SUBMISSION_NAME """
    required_dirs = ['harness', 'scripts', SUBMISSION_NAME]
    for dir_name in required_dirs:
        if not (rootdir / dir_name).exists():
            print(f"Error: Required directory '{dir_name}'",
                  f"not found in {rootdir}")
            sys.exit(1)

def build_submission(script_dir: Path):
    """
    Build the submission, including pulling dependencies as neeed
    """
    subprocess.run([str(script_dir) + "/build_task_" + SUBMISSION_NAME + ".sh"], check=True)

def log_step(step_num: int, step_name: str, start: bool = False):
    """ 
    Helper function to print timestamp after each step with second precision 
    """
    global _last_timestamp
    global _timestamps
    global _timestampsStr
    now = datetime.now()
    # Format with milliseconds precision
    timestamp = now.strftime("%H:%M:%S")

    # Calculate elapsed time if this isn't the first call
    elapsed_str = ""
    elapsed_seconds = 0
    if _last_timestamp is not None:
        elapsed_seconds = (now - _last_timestamp).total_seconds()
        elapsed_str = f" (elapsed: {round(elapsed_seconds, 4)}s)"

    # Update the last timestamp for the next call
    _last_timestamp = now

    if (not start):
        print(f"{timestamp} [harness] {step_num}: {step_name} completed{elapsed_str}")
        _timestampsStr[step_name] = f"{round(elapsed_seconds, 4)}s"
        _timestamps[step_name] = elapsed_seconds


def save_run(path: Path):
    global _timestamps
    global _timestampsStr
    global _bandwidth

    json.dump({
        "total_latency_s": round(sum(_timestamps.values()), 4),
        "per_stage": _timestampsStr,
        "bandwidth": _bandwidth,
    }, open(path,"w"), indent=2)

    print("[total latency]", f"{round(sum(_timestamps.values()), 4)}s")
