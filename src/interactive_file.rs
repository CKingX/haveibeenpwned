use crate::filter::{Filter, RB};
use crate::password;
use std::ffi::OsString;

pub fn interactive_file(file: OsString) {
    let filter = if let Ok(filter) = RB::open(file) {
        filter
    } else {
        return;
    };

    println!("Press Ctrl + C to exit");
    loop {
        let password = password::get_password();

        let result = filter.check(&password);

        if result {
            println!("Password is compromised");
        } else {
            println!("Password not compromised");
        }
    }
}
