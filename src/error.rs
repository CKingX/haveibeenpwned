use std::fmt::Display;

use colored::Colorize;

const SERVER_ERROR: &str = "Server error";
const DOWNLOAD_OUTPUT_FILE_ERROR: &str = "Unable to output downloaded file: {error}";
const DOWNLOAD_ERROR: &str = "Unable to download password ranges";

fn print_error(error: impl Display) {
    let error_string = format!("{error}");
    eprintln!("{}", error_string.red().bold());
}

pub fn server_error() {
    print_error(SERVER_ERROR);
}

pub fn download_output_error(error: std::io::Error) {
    let error_message = format!("{}", error.kind().to_string());
    let message = format!(
        "{}",
        DOWNLOAD_OUTPUT_FILE_ERROR.replace("{error}", &error_message)
    );
    print_error(message);
}

pub fn download_error() {
    print_error(DOWNLOAD_ERROR);
}
