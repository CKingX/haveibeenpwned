use crate::filter::Filter;
use crate::password;
use std::ffi::OsString;

pub fn interactive_file(file: OsString) {
    let filter = if let Some(filter) = Filter::open_filter(file) {
        filter
    } else {
        return;
    };

    println!("Press Ctrl + C to exit");
    loop {
        let password = password::get_password();

        let result = filter.check_password(&password);

        match result {
            password::Password::SafePassword => println!("Password not compromised"),
            password::Password::CompromisedPassword => println!("Password is compromised"),
        }
    }
}
