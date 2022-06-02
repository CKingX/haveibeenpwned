use crate::error;
use crate::password::{self, PasswordWithUsage};
use num_format::{Locale, ToFormattedString};

pub fn interactive() {
    loop {
        let password = rpassword::prompt_password("Password to check: ");
        if password.is_err() || password.as_ref().unwrap().is_empty() {
            println!("No password given. Try again");
            continue;
        }

        if let Ok(password) = password {
            let result = password::check_password_online(password);

            if let Ok(password_result) = result {
                match password_result {
                    PasswordWithUsage::SafePassword => println!("Password not compromised"),
                    PasswordWithUsage::CompromisedPassword(num) => {
                        println!(
                            "The password is compromised. It has been seen {} times",
                            num.to_formatted_string(&Locale::en)
                        )
                    }
                }
            } else {
                error::server_error()
            }

            break;
        }
    }
}
