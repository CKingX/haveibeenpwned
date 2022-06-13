use crate::filter::{self};
use crate::password;
use std::io::BufWriter;
use std::{
    ffi::OsString,
    io::{BufRead, BufReader},
};

pub fn generate_filter(input: OsString, output: OsString) {
    let input_file = std::fs::File::options().read(true).open(input);
    let output_file = std::fs::File::options()
        .write(true)
        .create_new(true)
        .open(output);

    if let Err(error) = input_file {
        eprintln!("Unable to read input file: {}", error.kind());
        return;
    }

    if let Err(error) = output_file {
        eprintln!("Unable to write to output file: {}", error.kind());
        return;
    }
    let output_file = BufWriter::new(output_file.unwrap());

    let mut filter = filter::RB::new(847_223_405);

    println!("Generating filter...");

    BufReader::new(input_file.unwrap())
        .lines()
        .map(|n| n.unwrap())
        .map(|n| password::remove_usage(&n))
        .map(|mut n| {
            n.make_ascii_uppercase();
            n
        })
        .for_each(|n| {
            filter.insert(&n);
        });

    println!("Filter generated. Preparing filter for output...");

    let output = filter.serialize(output_file);

    if let Err(error) = output {
        eprintln!("Unable to write to output file: {}", error);
        return;
    }

    println!("Filter created with {} items", filter.len());
}
