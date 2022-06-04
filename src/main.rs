mod arguments;
mod downloader;
mod error;
mod filter;
mod filter_generator;
mod interactive_file;
mod interactive_online;
mod password;

use arguments::*;

fn main() {
    let args = handle_arguments();

    match args.command {
        Commands::InteractiveFile { file } => interactive_file::interactive_file(file),
        Commands::InteractiveOnline => interactive_online::interactive(),
        Commands::Downloader { output } => downloader::downloader(output),
        Commands::FileCheck {
            password_file,
            file,
            print_passwords,
        } => todo!(),
        Commands::CreateFilter { input, output } => {
            filter_generator::generate_filter(input, output)
        }
    };
}
