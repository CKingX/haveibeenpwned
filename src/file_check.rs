use rayon::iter::ParallelIterator;
use std::sync::atomic::{AtomicI32, Ordering};
use std::{ffi::OsString, io::BufRead};

use rayon::iter::ParallelBridge;

use crate::arguments::Print;
use crate::filter::Filter;
use crate::password::Password;

pub fn file_check(password_file: OsString, filter: OsString, print_passwords: Option<Print>) {
    println!("Loading filter...");
    let filter = if let Some(filter) = Filter::open_filter(filter) {
        filter
    } else {
        return;
    };
    println!("Filter loaded");

    let file = std::fs::File::options().read(true).open(&password_file);

    if let Err(error) = file {
        eprintln!("Unable to open password file: {}", error.kind());
        return;
    }
    let file = file.unwrap();

    let file = std::io::BufReader::new(file);

    let mut total_count = 0;
    let compromised_count = AtomicI32::new(0);

    let result = file
        .lines()
        .map(|i| {
            total_count += 1;
            i
        })
        .par_bridge()
        .try_for_each(|password| {
            if password.is_err() {
                eprintln!("unable to read password from password file");
                return Err(());
            }

            let password = password.unwrap();
            if let Password::CompromisedPassword = filter.check_password(&password) {
                compromised_count.fetch_add(1, Ordering::Relaxed);
                if let Some(Print::Compromised) = print_passwords {
                    println!("{password}");
                }
            } else if let Some(Print::Safe) = print_passwords {
                    println!("{password}");
            }

            Ok(())
        });

    if result.is_err() {
        return;
    }

    println!(
        "Out of {total_count}, there were {} compromised passwords",
        compromised_count.load(Ordering::Relaxed)
    );
}
