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
use config::Config;
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
        Commands::InteractiveFile { file } => {
            interactive_file::interactive_file(arguments::filter_file(file))
        }
        Commands::InteractiveOnline => interactive_online::interactive(),
        Commands::Downloader { output, force } => downloader::downloader(output, force, false),
        Commands::FileCheck {
            password_file,
            filter,
            print_compromised_passwords,
        } => file_check::file_check(
            password_file,
            arguments::filter_file(filter),
            print_compromised_passwords,
        ),
        Commands::CreateFilter { input, output } => {
            filter_generator::generate_filter(input, output)
        }
        Commands::ResumeDownload => {
            let config = Config::load();
            if config.resume_token.is_none() {
                eprintln!("There is no download file that can be resumed");
                return;
            }
            downloader::downloader(config.resume_token.unwrap().download_file, false, true)
        }
    };
}
