use crate::filter;
use crate::password;
use std::ffi::OsString;
use std::io::BufReader;

pub fn interactive_file(file: OsString) {
    let input_file = std::fs::File::options().read(true).open(file);
    if let Err(error) = input_file {
        eprintln!("Unable to open the input file: {}", error.kind());
        return;
    }

    let input_file = BufReader::new(input_file.unwrap());

    let filter: Result<filter::Filter, _> = bincode::deserialize_from(input_file);

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
