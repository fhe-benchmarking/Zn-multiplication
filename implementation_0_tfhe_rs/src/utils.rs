// Copyright (c) 2025 HomomorphicEncryption.org
// All rights reserved.
//
// This software is licensed under the terms of the Apache v2 License.
// See the LICENSE.md file for details.

use std::fs;
use std::io::{self, BufReader, BufRead, BufWriter, Write};
pub use std::path::Path;


pub fn read_numbers_from_file(filepath: &Path) -> io::Result<Vec<u64>> {
    let file = fs::File::open(filepath)?;
    let reader = BufReader::new(file);

    reader.lines()
        .filter_map(|line| line.ok())
        .map(|line| line.trim().to_owned())
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<u64>())
        .collect::<Result<Vec<u64>, _>>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}


pub fn write_numbers_to_file(filepath: &Path, numbers: &[u64]) -> io::Result<()> {
    let mut file = BufWriter::new(fs::File::create(filepath)?);
    
    for number in numbers {
        writeln!(&mut file, "{}", number)?;
    }
    
    Ok(())
}
