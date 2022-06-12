mod arguments;
mod config;
mod downloader;
mod error;
mod file_check;
mod filter;
mod filter_generator;
mod interactive_file;
mod interactive_online;
mod password;

use arguments::*;
use update_informer::{registry, Check};

fn main() {
    let args = handle_arguments();

    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let informer = update_informer::new(registry::Crates, name, version);
    if let Ok(Some(version)) = informer.check_version() {
        println!(
            "New version is available: {} at https://github.com/CKingX/haveibeenpwned",
            version
        );
    }

    match args.command {
        Commands::InteractiveFile { file } => interactive_file::interactive_file(file),
        Commands::InteractiveOnline => interactive_online::interactive(),
        Commands::Downloader { output } => downloader::downloader(output),
        Commands::FileCheck {
            password_file,
            filter,
            print_compromised_passwords,
        } => file_check::file_check(password_file, filter, print_compromised_passwords),
        Commands::CreateFilter { input, output } => {
            filter_generator::generate_filter(input, output)
        }
        Commands::ResumeDownload => todo!(),
    };
}
