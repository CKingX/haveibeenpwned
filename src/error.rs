use std::fmt::Display;

use colored::Colorize;

const SERVER_ERROR: &str = "Server error";
const DOWNLOAD_OUTPUT_FILE_ERROR: &str = "Unable to output downloaded file: {error}";
const DOWNLOAD_ERROR: &str = "Unable to download password range: ";

fn print_error(error: impl Display) {
    let error_string = format!("{error}");
    eprintln!("{}", error_string.red().bold());
}

pub fn server_error() {
    print_error(SERVER_ERROR);
}

pub fn download_output_error(error: std::io::Error) {
    let error_message = error.kind();
    let message = DOWNLOAD_OUTPUT_FILE_ERROR.replace("{error}", &error_message.to_string());
    print_error(message);
}

pub fn download_error(range: u64) {
    print_error(format!("{DOWNLOAD_ERROR}{:05X}", range));
}
