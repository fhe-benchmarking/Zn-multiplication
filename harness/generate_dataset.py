#!/usr/bin/env python3

# Copyright (c) 2025 HomomorphicEncryption.org
# All rights reserved.
#
# This software is licensed under the terms of the Apache v2 License.
# See the LICENSE.md file for details.

"""
- Generates dataset i.e. two operands for 64 bit multiplication and saves them in lhs.txt and rhs.txt respectively. 
- Computes the product between corresponding operands for expected result.
"""

import numpy
import os
from utils import parse_submission_arguments

def main():

    __, params, seed, __, clrtxt = parse_submission_arguments('Generate dataset for FHE benchmark.')
    DATASET_LHS_PATH = params.datadir() / "lhs.txt"
    DATASET_RHS_PATH = params.datadir() / "rhs.txt"
    OUT_PATH = params.datadir() / "expected.txt"

    # 1) generate and write the inputs
    os.makedirs(params.datadir(), exist_ok=True)
    if seed is not None:
        numpy.random.seed(seed)
    lhs = numpy.random.randint(2**64, size=params.size_bound, dtype=numpy.uint64)
    numpy.savetxt(DATASET_LHS_PATH, lhs, fmt='%d')
    rhs = numpy.random.randint(2**64, size=params.size_bound, dtype=numpy.uint64)
    numpy.savetxt(DATASET_RHS_PATH, rhs, fmt='%d')

    if clrtxt:

        # 2) compute reference result
        result = lhs * rhs

        # 3) write to expected.txt (overwrites if it already exists)
        numpy.savetxt(OUT_PATH, result, fmt='%d')

if __name__ == "__main__":
    main()
