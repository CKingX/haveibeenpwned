mod arguments;
mod config;
mod downloader;
mod error;
mod filter;
mod filter_generator;
mod interactive_file;
mod interactive_online;
mod password;
mod file_check;

use arguments::*;

fn main() {
    let args = handle_arguments();

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
    };
}
