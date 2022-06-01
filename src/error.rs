use std::fmt::Display;

use colored::Colorize;

const SERVER_ERROR: &str = "Server error";

fn print_error(error: impl Display) {
    let error_string = format!("{error}");
    eprintln!("{}", error_string.red().bold());
}

pub fn server_error() {
    print_error(SERVER_ERROR);
}
