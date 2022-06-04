use crate::error;
use crate::password::{self, PasswordWithUsage};
use num_format::{Locale, ToFormattedString};

pub fn interactive() {
    let password = password::get_password();

    let result = password::check_password_online(password);

    if let Ok(password_result) = result {
        match password_result {
            PasswordWithUsage::SafePassword => println!("Password not compromised"),
            PasswordWithUsage::CompromisedPassword(num) => {
                println!(
                    "Password is compromised. It has been seen {} times",
                    num.to_formatted_string(&Locale::en)
                )
            }
        }
    } else {
        error::server_error()
    }
}
