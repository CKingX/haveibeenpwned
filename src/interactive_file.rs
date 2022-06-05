use serde::Deserialize;

use crate::filter::Filter;
use crate::password;
use std::{ffi::OsString, io::Read};

pub fn interactive_file(file: OsString) {
    let input_file = std::fs::File::options().read(true).open(file);
    if let Err(error) = input_file {
        eprintln!("Unable to open the input file: {}", error.kind());
        return;
    }

    let mut mp_file = Vec::new();
    let input_file = input_file.unwrap().read_to_end(&mut mp_file);

    if let Err(error) = input_file {
        eprintln!("Unable to read the input file: {}", error.kind());
        return;
    }

    drop(input_file);

    let filter = flexbuffers::Reader::get_root(&*mp_file);

    if filter.is_err() {
        eprintln!("Input file is not a valid filter");
        return;
    }

    let filter = filter.unwrap();

    let filter = Filter::deserialize(filter);
    drop(mp_file);

    if filter.is_err() {
        eprintln!("Input file is not a valid filter");
        return;
    }

    let filter = filter.unwrap();

    let password = password::get_password();

    let result = filter.check_password(&password);

    match result {
        password::Password::SafePassword => println!("Password not compromised"),
        password::Password::CompromisedPassword => println!("Password is compromised"),
    }
}
