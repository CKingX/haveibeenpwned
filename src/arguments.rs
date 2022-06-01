use clap::{IntoApp, Parser, Subcommand};
use std::ffi::OsString;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Check if password is compromised using either a filter created by HIBP or  downloaded passwords file
    InteractiveFile {
        #[clap(arg_enum)]
        /// Choose a password file type
        password_type: FileType,
        /// Location of password file
        file: OsString,
    },
    /// Checks if password is compromised using HIBP server online
    InteractiveOnline,
    /// Download compromised passwords from HIBP by querying all password ranges
    Downloader {
        /// output of the downloaded HIBP file
        output: OsString,
    },
    /// Check all passwords in a file to see if they are compromised
    FileCheck {
        #[clap(arg_enum)]
        password_type: FileType,
        password_file: OsString,
        file: OsString,
        #[clap(short, long)]
        print_passwords: bool,
    },
    /// Create an efficient filter that allows you to check passwords offline
    /// However, while significantly smaller, it can result in false positives
    CreateFilter {
        /// Input downloaded compromised password file to create filter from
        input: OsString,
        /// Output location of the filter
        output: OsString,
    },
}

#[derive(clap::ArgEnum, Clone)]
pub enum FileType {
    /// Downloaded HIBP file
    DownloadedPasswords,
    /// Filter created by hibp
    Filter,
}

pub fn handle_arguments() -> Cli {
    Cli::parse()
}

#[allow(deprecated)]
pub fn _handle_command() -> clap::App<'static> {
    Cli::command()
}
