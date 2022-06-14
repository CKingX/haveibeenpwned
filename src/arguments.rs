use clap::{Parser, Subcommand};
use std::{ffi::OsString, path::Path};

use crate::config::Config;

pub static FILTER: Option<&str> = if cfg!(feature = "winfilter") {
    Some("C:\\Program Files\\haveibeenwpned\\filter.bin")
} else if cfg!(feature = "linuxfilter") {
    Some("/usr/share/haveibeenpwned/filter.bin")
} else {
    None
};

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Check if password is compromised using a filter
    InteractiveFile {
        /// Location of password file
        #[clap (required = is_filter_required())]
        file: Option<OsString>,
    },
    /// Checks if password is compromised using HIBP server online
    InteractiveOnline,
    /// Download compromised passwords from HIBP by querying all password ranges
    Downloader {
        /// output of the downloaded HIBP file
        output: OsString,
        /// Overwrite existing file
        #[clap(short)]
        force: bool,
    },
    /// Check all passwords in a file to see if they are compromised
    FileCheck {
        /// Path to the file containing passwords to check
        password_file: OsString,
        /// Path to the filter file
        #[clap (required = is_filter_required())]
        filter: Option<OsString>,
        /// Use -p if you want to print compromised passwords
        #[clap(short, long)]
        print_compromised_passwords: bool,
    },
    /// Create an efficient filter that allows you to check passwords offline
    /// However, while significantly smaller, it can result in false positives
    CreateFilter {
        /// Input downloaded compromised password file to create filter from
        input: OsString,
        /// Output location of the filter
        output: OsString,
    },
    /// Resume existing download
    #[clap(hide = hide_resume())]
    ResumeDownload,
}

pub fn handle_arguments() -> Cli {
    Cli::parse()
}

fn hide_resume() -> bool {
    let config = Config::load();
    if config.resume_token.is_none() {
        return true;
    }

    let resume_file = config.resume_token.unwrap().download_file;
    let resume_file: &Path = resume_file.as_ref();
    match resume_file.canonicalize() {
        Ok(_) => false,
        Err(_) => true,
    }
}

fn is_filter_required() -> bool {
    let config = Config::load();
    if FILTER.is_some() {
        return false;
    }
    if config.password_filter.is_none() {
        return true;
    }
    let filter = config.password_filter.unwrap();
    let filter: &Path = filter.as_ref();
    let filter_exists = filter.canonicalize();

    if filter_exists.is_ok() {
        false
    } else {
        true
    }
}

pub fn filter_file(file: Option<OsString>) -> OsString {
    if let Some(file) = file {
        file
    } else {
        if let Some(filter) = FILTER {
            OsString::from(filter)
        } else {
            let config = Config::load();
            let filter = config.password_filter.unwrap();
            filter
        }
    }
}