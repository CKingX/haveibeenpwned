use std::sync::atomic::{AtomicI32, Ordering};
use std::time::{SystemTime, Duration};
use std::{ffi::OsString, io::BufRead};
use rayon::iter::ParallelIterator;

use rayon::iter::ParallelBridge;

use crate::filter::Filter;
use crate::password::Password;

pub fn file_check(password_file: OsString, filter: OsString, print_passwords: bool) {
    println!("Loading filter...");
    let filter = if let Some(filter) = Filter::open_filter(filter) {
        filter
    } else {
        return
    };
    println!("Filter loaded");

    let file = std::fs::File::options().read(true).open(&password_file);
    
    if let Err(error) = file {
        eprintln!("Unable to open password file: {}", error.kind());
        return;
    }
    let file = file.unwrap();

    let file = std::io::BufReader::new(file);

    let total_count = AtomicI32::new(0);
    let compromised_count = AtomicI32::new(0);

    let before = SystemTime::now();

    let result = file.lines().par_bridge().try_for_each(|password| {
        if let Err(_) = password {
            eprintln!("unable to read password from password file");
            return Err(());
        }
        total_count.fetch_add(1, Ordering::Relaxed);

        let password = password.unwrap();
        if let Password::CompromisedPassword = filter.check_password(&password) {
            compromised_count.fetch_add(1, Ordering::Relaxed);
            if print_passwords {
                println!("{password}");
            }
        }

        return Ok(());
    });

    if let Err(_) = result {return;}

    let after = SystemTime::now().duration_since(before);

    match after {
        Ok(time) => {let time =
                if time < Duration::from_millis(1) {
                    format!("{} µs", time.as_micros())
                } else {
                    format!("{} ms", time.as_millis())
                };
        println!("Password checking took {time}");
        }
        Err(_) => println!("Unable to determine time"),
    }

    println!("Out of {}, there were {} compromised passwords", total_count.load(Ordering::Relaxed), compromised_count.load(Ordering::Relaxed));
    
}