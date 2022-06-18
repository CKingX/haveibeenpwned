use crate::filter::{self, FilterSize};
use crate::password;
use siphasher::sip::SipHasher13;
use std::hash::Hash;
use std::io::BufWriter;
use std::{
    ffi::OsString,
    fmt::Display,
    hash::Hasher,
    io::{BufRead, BufReader},
};

enum Size {
    GB(f64),
    MB(f64),
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match &self {
            Size::GB(num) => format!("{:.1} GiB", num),
            Size::MB(num) => format!("{:.1} MiB", num),
        };
        write!(f, "{output}")
    }
}

const SIZE: f64 = 847_223_405.0;
const SMALL_FILTER_SIZE: Size = Size::MB(SIZE * 9.1 / 8.0 / 1024.0 / 1024.0);
const MEDIUM_FILTER_SIZE: Size = Size::GB(SIZE * 18.1 / 8.0 / 1024.0 / 1024.0 / 1024.0);
const LARGE_FILTER_SIZE: Size = Size::GB(SIZE * 36.2 / 8.0 / 1024.0 / 1024.0 / 1024.0);

pub fn generate_filter(input: OsString, output: OsString) {
    println!("There are 3 sizes of filters: Small (s), Medium (m), and Large (l)");
    println!("The filter can find compromised password with 100% accuracy, but it may show passwords that are not compromised as compromised (false positives)");
    println!(
        "The small filter has false positivity rate of 0.4% (1 in 256) and needs {}",
        SMALL_FILTER_SIZE
    );
    println!(
        "The medium filter has false positivity rate of 0.0016% (1 in 65,536) and needs {}",
        MEDIUM_FILTER_SIZE
    );
    println!(
        "The large filter has false positivity rate of 1 in 4,294,967,296 and needs {}",
        LARGE_FILTER_SIZE
    );

    println!("Please choose a size (s, m, l):");
    let result;
    loop {
        let mut filter_size = String::new();
        let input = std::io::stdin().read_line(&mut filter_size);
        match input {
            Ok(_) => {
                result = match filter_size.lines().next().unwrap() {
                    "s" => FilterSize::Small,
                    "m" => FilterSize::Medium,
                    "l" => FilterSize::Large,
                    _ => {
                        println!("Wrong option. Try again:");
                        continue;
                    }
                };
                break;
            }
            Err(_) => continue,
        }
    }

    let input_file = std::fs::File::options().read(true).open(input);
    let output_file = std::fs::File::options()
        .write(true)
        .create_new(true)
        .open(output);
    let keys = SipHasher13::new().keys();

    if let Err(error) = input_file {
        eprintln!("Unable to read input file: {}", error.kind());
        return;
    }

    if let Err(error) = output_file {
        eprintln!("Unable to write to output file: {}", error.kind());
        return;
    }
    let output_file = BufWriter::new(output_file.unwrap());

    let input_file = BufReader::new(input_file.unwrap())
        .lines()
        .map(|n| n.unwrap())
        .map(|n| password::remove_usage(&n))
        .map(|mut n| {
            n.make_ascii_uppercase();
            n
        })
        .map(|n| {
            let mut hasher = SipHasher13::new_with_keys(keys.0, keys.1);
            n.hash(&mut hasher);
            hasher.finish()
        })
        .collect::<Vec<_>>();

    println!("Generating filter...");

    let filter = filter::Filter::new(&input_file, keys, result);
    if let Err(()) = filter {
        eprintln!(
            "Unable to generate filter. Please report this issue with diagnostics codes {} {}",
            keys.0, keys.1
        );
        return;
    }
    let filter = filter.unwrap();

    drop(input_file);

    println!("Filter generated. Preparing filter for output...");

    let output = bincode::serialize_into(output_file, &filter);

    if let Err(error) = output {
        eprintln!("Unable to write to output file: {}", error);
        return;
    }

    println!("Filter created with {} items", filter.len());
}
