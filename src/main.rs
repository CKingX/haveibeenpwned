mod arguments;
mod downloader;
mod error;
mod filter_generator;
mod interactive_online;
mod password;

use arguments::*;

fn main() {
    let args = handle_arguments();

    match args.command {
        Commands::InteractiveFile {
            password_type,
            file,
        } => todo!(),
        Commands::InteractiveOnline => interactive_online::interactive(),
        Commands::Downloader { output } => downloader::downloader(output),
        Commands::FileCheck {
            password_type,
            password_file,
            file,
            print_passwords,
        } => todo!(),
        Commands::CreateFilter { input, output } => {
            filter_generator::generate_filter(input, output)
        }
    };
}
